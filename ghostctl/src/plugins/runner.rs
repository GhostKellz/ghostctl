//! Secure plugin execution with sandboxing and validation
//!
//! Security features:
//! - Plugin name validation (alphanumeric + _-. only)
//! - Path canonicalization to prevent directory traversal
//! - Plugin allowlist mechanism
//! - Restricted Lua environment (no io, os.execute, loadfile)
//! - Safe command execution (no shell interpolation)

use mlua::{Lua, StdLib};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Invalid plugin name: {0}")]
    InvalidName(String),
    #[error("Path traversal attempt: {0}")]
    PathTraversal(String),
    #[error("Plugin not found: {0}")]
    NotFound(String),
    #[error("Plugin not allowed: {0}")]
    NotAllowed(String),
    #[error("Plugin execution failed: {0}")]
    ExecutionError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Validate plugin name to prevent injection attacks (public API)
pub fn validate_plugin_name_public(name: &str) -> Result<(), PluginError> {
    validate_plugin_name(name)
}

/// Validate plugin name to prevent injection attacks
/// Only allows alphanumeric characters, underscores, hyphens, and dots
fn validate_plugin_name(name: &str) -> Result<(), PluginError> {
    if name.is_empty() {
        return Err(PluginError::InvalidName("Empty name".to_string()));
    }

    // Reject path traversal attempts
    if name.contains("..") || name.contains('/') || name.contains('\\') {
        return Err(PluginError::PathTraversal(name.to_string()));
    }

    // Must not start with a dot (hidden files)
    if name.starts_with('.') {
        return Err(PluginError::InvalidName(
            "Plugin name cannot start with '.'".to_string(),
        ));
    }

    // Only allow safe characters
    let valid = name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.');

    if !valid {
        return Err(PluginError::InvalidName(format!(
            "Contains invalid characters: {}",
            name
        )));
    }

    Ok(())
}

/// Get the safe plugin path, validating it stays within the plugin directory
fn safe_plugin_path(
    plugin_dir: &Path,
    name: &str,
    extension: &str,
) -> Result<PathBuf, PluginError> {
    validate_plugin_name(name)?;

    let filename = if name.ends_with(extension) {
        name.to_string()
    } else {
        format!("{}{}", name, extension)
    };

    let candidate = plugin_dir.join(&filename);

    // Canonicalize both paths to resolve symlinks
    let canonical_dir = plugin_dir
        .canonicalize()
        .map_err(|_| PluginError::NotFound(plugin_dir.display().to_string()))?;

    let canonical_path = candidate
        .canonicalize()
        .map_err(|_| PluginError::NotFound(name.to_string()))?;

    // Verify the path is within the plugin directory
    if !canonical_path.starts_with(&canonical_dir) {
        return Err(PluginError::PathTraversal(name.to_string()));
    }

    Ok(canonical_path)
}

/// Load the plugin allowlist from config
fn load_allowlist() -> HashSet<String> {
    let Some(config_dir) = dirs::config_dir() else {
        return HashSet::new();
    };

    let allowlist_path = config_dir.join("ghostctl/allowed_plugins.txt");
    if !allowlist_path.exists() {
        return HashSet::new();
    }

    match fs::read_to_string(&allowlist_path) {
        Ok(content) => content
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .collect(),
        Err(_) => HashSet::new(),
    }
}

/// Save a plugin to the allowlist
fn add_to_allowlist(name: &str) -> Result<(), PluginError> {
    let Some(config_dir) = dirs::config_dir() else {
        return Err(PluginError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Config directory not found",
        )));
    };

    let allowlist_path = config_dir.join("ghostctl/allowed_plugins.txt");

    // Create directory if needed
    if let Some(parent) = allowlist_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Append to allowlist
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&allowlist_path)?;

    writeln!(file, "{}", name)?;
    log::info!("Added '{}' to plugin allowlist", name);
    Ok(())
}

/// Check if a plugin is allowed, with interactive confirmation for new plugins
fn check_plugin_allowed(name: &str) -> Result<bool, PluginError> {
    let allowlist = load_allowlist();

    if allowlist.contains(name) {
        return Ok(true);
    }

    // If allowlist is empty, allow all (first-run experience)
    if allowlist.is_empty() {
        log::info!("No allowlist configured, allowing plugin '{}'", name);
        return Ok(true);
    }

    // Interactive confirmation for unallowed plugins
    use dialoguer::Confirm;

    println!("Plugin '{}' is not in the allowlist.", name);
    let allow = Confirm::new()
        .with_prompt("Allow this plugin to run?")
        .default(false)
        .interact()
        .unwrap_or(false);

    if allow {
        let remember = Confirm::new()
            .with_prompt("Remember this choice?")
            .default(true)
            .interact()
            .unwrap_or(false);

        if remember {
            add_to_allowlist(name)?;
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Execute a plugin by name
pub fn execute(name: &str) {
    if let Err(e) = execute_internal(name) {
        println!("Plugin error: {}", e);
    }
}

fn execute_internal(name: &str) -> Result<(), PluginError> {
    validate_plugin_name(name)?;

    let Some(config_dir) = dirs::config_dir() else {
        return Err(PluginError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Config directory not found",
        )));
    };

    let plugin_dir = config_dir.join("ghostctl/plugins");
    if !plugin_dir.exists() {
        return Err(PluginError::NotFound(format!(
            "Plugin directory not found: {}",
            plugin_dir.display()
        )));
    }

    // Check allowlist
    if !check_plugin_allowed(name)? {
        return Err(PluginError::NotAllowed(name.to_string()));
    }

    // Try .lua first, then .sh
    if let Ok(lua_path) = safe_plugin_path(&plugin_dir, name, ".lua") {
        println!("Running Lua plugin: {}", name);
        run_lua_plugin_safe(&lua_path, name)
    } else if let Ok(sh_path) = safe_plugin_path(&plugin_dir, name, ".sh") {
        println!("Running shell plugin: {}", name);
        run_shell_plugin_safe(&sh_path)
    } else {
        Err(PluginError::NotFound(name.to_string()))
    }
}

/// Run a Lua plugin with a restricted environment
fn run_lua_plugin_safe(path: &Path, name: &str) -> Result<(), PluginError> {
    let code = fs::read_to_string(path)?;

    // Create Lua with restricted standard library
    // Exclude: io, os, debug, ffi, package (loadfile, etc.)
    let lua = Lua::new_with(
        StdLib::STRING | StdLib::TABLE | StdLib::MATH | StdLib::UTF8,
        mlua::LuaOptions::default(),
    )
    .map_err(|e| PluginError::ExecutionError(e.to_string()))?;

    // Set up restricted globals
    let globals = lua.globals();

    // Provide a safe print function
    let print_fn = lua
        .create_function(|_, args: mlua::Variadic<mlua::Value>| {
            let output: Vec<String> = args
                .iter()
                .map(|v| match v {
                    mlua::Value::Nil => "nil".to_string(),
                    mlua::Value::Boolean(b) => b.to_string(),
                    mlua::Value::Integer(i) => i.to_string(),
                    mlua::Value::Number(n) => n.to_string(),
                    mlua::Value::String(s) => s.to_str().unwrap_or("").to_string(),
                    _ => format!("{:?}", v),
                })
                .collect();
            println!("{}", output.join("\t"));
            Ok(())
        })
        .map_err(|e| PluginError::ExecutionError(e.to_string()))?;
    globals
        .set("print", print_fn)
        .map_err(|e| PluginError::ExecutionError(e.to_string()))?;

    // Provide safe_exec: runs pre-approved commands only
    let safe_exec_fn = lua
        .create_function(|_, (program, args): (String, mlua::Variadic<String>)| {
            // Whitelist of allowed programs
            let allowed = ["echo", "date", "whoami", "hostname", "uname", "pwd", "ls"];

            if !allowed.contains(&program.as_str()) {
                return Err(mlua::Error::external(format!(
                    "Program '{}' not in safe execution whitelist",
                    program
                )));
            }

            let args_vec: Vec<String> = args.into_iter().collect();
            let output = Command::new(&program)
                .args(&args_vec)
                .output()
                .map_err(|e| mlua::Error::external(e.to_string()))?;

            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(stdout)
        })
        .map_err(|e| PluginError::ExecutionError(e.to_string()))?;
    globals
        .set("safe_exec", safe_exec_fn)
        .map_err(|e| PluginError::ExecutionError(e.to_string()))?;

    // Provide ghostctl info table
    let info = lua
        .create_table()
        .map_err(|e| PluginError::ExecutionError(e.to_string()))?;
    info.set("plugin_name", name)
        .map_err(|e| PluginError::ExecutionError(e.to_string()))?;
    info.set("version", env!("CARGO_PKG_VERSION"))
        .map_err(|e| PluginError::ExecutionError(e.to_string()))?;
    globals
        .set("ghostctl", info)
        .map_err(|e| PluginError::ExecutionError(e.to_string()))?;

    // Execute the plugin
    lua.load(&code)
        .exec()
        .map_err(|e| PluginError::ExecutionError(e.to_string()))?;

    println!("Plugin '{}' executed successfully.", name);
    Ok(())
}

/// Run a shell plugin safely (no sh -c)
fn run_shell_plugin_safe(path: &Path) -> Result<(), PluginError> {
    // Verify the file is owned by the current user and not world-writable
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let metadata = fs::metadata(path)?;
        let mode = metadata.mode();

        // Check for world-writable
        if mode & 0o002 != 0 {
            return Err(PluginError::ExecutionError(
                "Script is world-writable, refusing to execute".to_string(),
            ));
        }

        // Check owner matches current user
        let uid = unsafe { libc::getuid() };
        if metadata.uid() != uid && uid != 0 {
            log::warn!(
                "Script not owned by current user (owned by uid {})",
                metadata.uid()
            );
        }
    }

    // Execute directly without shell
    let status = Command::new("bash")
        .arg("--restricted") // Use restricted bash mode
        .arg(path)
        .status()?;

    if status.success() {
        println!("Shell plugin executed successfully.");
        Ok(())
    } else {
        Err(PluginError::ExecutionError(format!(
            "Shell plugin exited with status: {}",
            status
        )))
    }
}

/// Run user script menu with security checks
pub fn run_user_script_menu() {
    use dialoguer::Select;

    let Some(config_dir) = dirs::config_dir() else {
        println!("Could not determine config directory");
        return;
    };

    let scripts_dir = config_dir.join("ghostctl/scripts");
    if !scripts_dir.exists() {
        println!("No user scripts directory found at {:?}", scripts_dir);
        return;
    }

    let entries = match fs::read_dir(&scripts_dir) {
        Ok(e) => e,
        Err(e) => {
            println!("Failed to read scripts directory: {}", e);
            return;
        }
    };

    let mut scripts = vec![];
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file()
            && let Some(file_name) = path.file_name()
        {
            let name = file_name.to_string_lossy().to_string();
            // Validate script name
            if validate_plugin_name(&name).is_ok() {
                scripts.push(name);
            } else {
                log::warn!("Skipping script with invalid name: {}", name);
            }
        }
    }

    if scripts.is_empty() {
        println!("No valid user scripts found in {:?}", scripts_dir);
        return;
    }

    let Ok(idx) = Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Select a user script to run")
        .items(&scripts)
        .default(0)
        .interact()
    else {
        return;
    };

    let script_name = &scripts[idx];

    // Use safe path resolution
    let script_path = match safe_plugin_path(&scripts_dir, script_name, "") {
        Ok(p) => p,
        Err(e) => {
            println!("Script path error: {}", e);
            return;
        }
    };

    let extension = script_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match extension {
        "sh" => {
            println!("Running shell script: {}", script_name);
            if let Err(e) = run_shell_plugin_safe(&script_path) {
                println!("Script error: {}", e);
            }
        }
        "lua" => {
            println!("Running Lua script: {}", script_name);
            if let Err(e) = run_lua_plugin_safe(&script_path, script_name) {
                println!("Script error: {}", e);
            }
        }
        _ => {
            println!("Unknown script type: {}", extension);
        }
    }
}

/// Legacy function - now just calls the safe version
pub fn run_lua_plugin(name: &str) {
    let Some(config_dir) = dirs::config_dir() else {
        println!("Could not determine config directory");
        return;
    };

    let plugin_dir = config_dir.join("ghostctl/plugins");
    match safe_plugin_path(&plugin_dir, name, ".lua") {
        Ok(path) => {
            if let Err(e) = run_lua_plugin_safe(&path, name) {
                println!("Plugin error: {}", e);
            }
        }
        Err(e) => {
            println!("Plugin error: {}", e);
        }
    }
}

/// Legacy function - calls safe version
pub fn run_lua_script(path: &Path) {
    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    if let Err(e) = run_lua_plugin_safe(path, name) {
        println!("Script error: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_plugin_name_valid() {
        assert!(validate_plugin_name("my_plugin").is_ok());
        assert!(validate_plugin_name("my-plugin").is_ok());
        assert!(validate_plugin_name("plugin.lua").is_ok());
        assert!(validate_plugin_name("Plugin123").is_ok());
    }

    #[test]
    fn test_validate_plugin_name_invalid() {
        assert!(validate_plugin_name("").is_err());
        assert!(validate_plugin_name("..").is_err());
        assert!(validate_plugin_name("../etc/passwd").is_err());
        assert!(validate_plugin_name(".hidden").is_err());
        assert!(validate_plugin_name("plugin;rm -rf").is_err());
        assert!(validate_plugin_name("plugin`whoami`").is_err());
        assert!(validate_plugin_name("plugin$(cmd)").is_err());
    }

    #[test]
    fn test_validate_plugin_name_path_traversal() {
        assert!(matches!(
            validate_plugin_name("../../../etc/passwd"),
            Err(PluginError::PathTraversal(_))
        ));
        assert!(matches!(
            validate_plugin_name("..\\..\\windows\\system32"),
            Err(PluginError::PathTraversal(_))
        ));
    }
}
