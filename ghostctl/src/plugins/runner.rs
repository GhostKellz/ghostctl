use dialoguer::{Select};
use mlua::{Lua};
use std::fs;
use std::process::Command;

pub fn execute(name: &str) {
    let plugin_path = dirs::config_dir()
        .unwrap()
        .join("ghostctl/plugins")
        .join(name);

    if plugin_path.extension().unwrap_or_default() == "sh" {
        println!("Running shell plugin: {}", name);
        let _ = Command::new("bash").arg(plugin_path).status();
    } else if plugin_path.extension().unwrap_or_default() == "lua" {
        println!("Running Lua plugin: {}", name);
        run_lua_plugin(name);
    } else {
        println!("Unknown plugin type.");
    }
}

pub fn run_lua_plugin(name: &str) {
    let plugin_dir = dirs::config_dir().unwrap().join("ghostctl/plugins");
    let plugin_path = plugin_dir.join(format!("{}.lua", name));
    if !plugin_path.exists() {
        println!("Plugin not found: {:?}", plugin_path);
        return;
    }
    let code = match fs::read_to_string(&plugin_path) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to read plugin: {}", e);
            return;
        }
    };
    let lua = Lua::new();
    // Expose a minimal Rust API to Lua
    let globals = lua.globals();
    globals
        .set(
            "run_command",
            lua.create_function(|_, cmd: String| {
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&cmd)
                    .output();
                match output {
                    Ok(out) => {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        println!("{}", stdout);
                    }
                    Err(e) => println!("Failed to run command: {}", e),
                }
                Ok(())
            })
            .unwrap(),
        )
        .unwrap();
    match lua.load(&code).exec() {
        Ok(_) => println!("Plugin '{}' executed successfully.", name),
        Err(e) => println!("Error running plugin '{}': {}", name, e),
    }
}

pub fn run_user_script_menu() {
    use dialoguer::Select;
    use std::fs;
    let scripts_dir = dirs::config_dir().unwrap().join("ghostctl/scripts");
    if let Ok(entries) = fs::read_dir(&scripts_dir) {
        let mut scripts = vec![];
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                scripts.push(path.file_name().unwrap().to_string_lossy().to_string());
            }
        }
        if scripts.is_empty() {
            println!("No user scripts found in {:?}", scripts_dir);
            return;
        }
        let idx = Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Select a user script to run")
            .items(&scripts)
            .default(0)
            .interact()
            .unwrap();
        let script_path = scripts_dir.join(&scripts[idx]);
        if script_path.extension().unwrap_or_default() == "sh" {
            println!("Running shell script: {}", scripts[idx]);
            let _ = std::process::Command::new("bash")
                .arg(&script_path)
                .status();
        } else if script_path.extension().unwrap_or_default() == "lua" {
            println!("Running Lua script: {}", scripts[idx]);
            run_lua_script(&script_path);
        } else {
            println!("Unknown script type: {:?}", script_path);
        }
    } else {
        println!("No user scripts directory found at {:?}", scripts_dir);
    }
}

pub fn run_lua_script(path: &std::path::Path) {
    println!(
        "Running Lua script at {} (integration not implemented, requires mlua)",
        path.display()
    );
    // TODO: Integrate with mlua if desired
}
