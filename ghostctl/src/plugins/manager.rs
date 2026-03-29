//! Plugin manager for listing, installing, and verifying plugins
//!
//! Security features:
//! - SHA256 checksum verification for downloaded plugins
//! - HTTPS enforcement for plugin downloads
//! - Plugin name validation

use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use super::runner::PluginError;

/// List all installed plugins
pub fn list_plugins() {
    let Some(config_dir) = dirs::config_dir() else {
        println!("Could not determine config directory");
        return;
    };

    let plugins_dir = config_dir.join("ghostctl/plugins");

    println!("Installed Plugins");
    println!("=================");

    if !plugins_dir.exists() {
        println!("No plugins directory found at {}", plugins_dir.display());
        println!("Create it with: mkdir -p {}", plugins_dir.display());
        return;
    }

    match fs::read_dir(&plugins_dir) {
        Ok(entries) => {
            let mut count = 0;
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

                    let plugin_type = match extension {
                        "lua" => "Lua",
                        "sh" => "Shell",
                        _ => continue, // Skip non-plugin files
                    };

                    let name = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown");

                    // Calculate checksum
                    let checksum = match calculate_file_checksum(&path) {
                        Ok(hash) => format!("sha256:{}", &hash[..16]),
                        Err(_) => "error".to_string(),
                    };

                    println!("  {} [{}] ({})", name, plugin_type, checksum);
                    count += 1;
                }
            }

            if count == 0 {
                println!("  No plugins installed");
            } else {
                println!("\nTotal: {} plugin(s)", count);
            }
        }
        Err(e) => println!("Failed to read plugins directory: {}", e),
    }
}

/// Calculate SHA256 checksum of a file
fn calculate_file_checksum(path: &PathBuf) -> Result<String, std::io::Error> {
    let content = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

/// Install a plugin from URL with checksum verification
pub fn install_from_url(url: &str) {
    if let Err(e) = install_from_url_internal(url, None) {
        println!("Installation failed: {}", e);
    }
}

/// Install a plugin from URL with optional expected checksum
pub fn install_from_url_with_checksum(url: &str, expected_checksum: Option<&str>) {
    if let Err(e) = install_from_url_internal(url, expected_checksum) {
        println!("Installation failed: {}", e);
    }
}

fn install_from_url_internal(
    url: &str,
    expected_checksum: Option<&str>,
) -> Result<(), PluginError> {
    // Validate URL
    if !url.starts_with("https://") {
        return Err(PluginError::ExecutionError(
            "Only HTTPS URLs are allowed for security".to_string(),
        ));
    }

    // Extract filename from URL
    let filename = url.rsplit('/').next().ok_or_else(|| {
        PluginError::InvalidName("Could not extract filename from URL".to_string())
    })?;

    // Validate filename
    super::runner::validate_plugin_name_public(filename)?;

    // Ensure it's a supported plugin type
    if !filename.ends_with(".lua") && !filename.ends_with(".sh") {
        return Err(PluginError::InvalidName(
            "Only .lua and .sh plugins are supported".to_string(),
        ));
    }

    // Get plugins directory
    let Some(config_dir) = dirs::config_dir() else {
        return Err(PluginError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Config directory not found",
        )));
    };

    let plugins_dir = config_dir.join("ghostctl/plugins");
    fs::create_dir_all(&plugins_dir)?;

    let dest_path = plugins_dir.join(filename);

    println!("Downloading {} ...", url);

    // Download using curl with security flags
    let output = Command::new("curl")
        .args([
            "--fail",       // Fail on HTTP errors
            "--silent",     // Silent mode
            "--show-error", // Show errors
            "--location",   // Follow redirects
            "--max-redirs",
            "5", // Limit redirects
            "--proto",
            "=https", // HTTPS only
            url,
        ])
        .output()
        .map_err(|e| PluginError::ExecutionError(format!("curl failed: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(PluginError::ExecutionError(format!(
            "Download failed: {}",
            stderr
        )));
    }

    let content = output.stdout;

    // Calculate checksum
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let actual_checksum = hex::encode(hasher.finalize());

    // Verify checksum if provided
    if let Some(expected) = expected_checksum {
        if actual_checksum != expected {
            return Err(PluginError::ExecutionError(format!(
                "Checksum mismatch! Expected: {}, Got: {}",
                expected, actual_checksum
            )));
        }
        println!("Checksum verified: {}", &actual_checksum[..16]);
    } else {
        println!("Checksum (save this!): sha256:{}", actual_checksum);
        println!("WARNING: No checksum provided - cannot verify integrity");
    }

    // Write file
    let mut file = fs::File::create(&dest_path)?;
    file.write_all(&content)?;

    // Set permissions (non-executable by default for safety)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&dest_path)?.permissions();
        perms.set_mode(0o644); // Read/write for owner, read for others
        fs::set_permissions(&dest_path, perms)?;
    }

    println!("Plugin installed: {}", dest_path.display());
    println!(
        "Run with: ghostctl plugin run {}",
        filename.trim_end_matches(".lua").trim_end_matches(".sh")
    );

    Ok(())
}

/// Remove an installed plugin
pub fn remove_plugin(name: &str) -> Result<(), PluginError> {
    super::runner::validate_plugin_name_public(name)?;

    let Some(config_dir) = dirs::config_dir() else {
        return Err(PluginError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Config directory not found",
        )));
    };

    let plugins_dir = config_dir.join("ghostctl/plugins");

    // Try both extensions
    let lua_path = plugins_dir.join(format!("{}.lua", name));
    let sh_path = plugins_dir.join(format!("{}.sh", name));

    let mut removed = false;

    if lua_path.exists() {
        fs::remove_file(&lua_path)?;
        println!("Removed: {}", lua_path.display());
        removed = true;
    }

    if sh_path.exists() {
        fs::remove_file(&sh_path)?;
        println!("Removed: {}", sh_path.display());
        removed = true;
    }

    if !removed {
        return Err(PluginError::NotFound(name.to_string()));
    }

    Ok(())
}

/// Verify integrity of installed plugins
pub fn verify_plugins() {
    let Some(config_dir) = dirs::config_dir() else {
        println!("Could not determine config directory");
        return;
    };

    let plugins_dir = config_dir.join("ghostctl/plugins");
    let checksums_path = config_dir.join("ghostctl/plugin_checksums.txt");

    if !checksums_path.exists() {
        println!("No checksum file found at {}", checksums_path.display());
        println!("Generate one with: ghostctl plugin generate-checksums");
        return;
    }

    let checksums_content = match fs::read_to_string(&checksums_path) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to read checksum file: {}", e);
            return;
        }
    };

    let mut verified = 0;
    let mut failed = 0;

    for line in checksums_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            continue;
        }

        let expected_hash = parts[0];
        let filename = parts[1];

        let file_path = plugins_dir.join(filename);
        if !file_path.exists() {
            println!("MISSING: {}", filename);
            failed += 1;
            continue;
        }

        match calculate_file_checksum(&file_path) {
            Ok(actual_hash) => {
                if actual_hash == expected_hash {
                    println!("OK: {}", filename);
                    verified += 1;
                } else {
                    println!("MODIFIED: {} (checksum mismatch)", filename);
                    failed += 1;
                }
            }
            Err(e) => {
                println!("ERROR: {} - {}", filename, e);
                failed += 1;
            }
        }
    }

    println!();
    println!("Verified: {}, Failed: {}", verified, failed);
}

/// Generate checksums for all installed plugins
pub fn generate_checksums() {
    let Some(config_dir) = dirs::config_dir() else {
        println!("Could not determine config directory");
        return;
    };

    let plugins_dir = config_dir.join("ghostctl/plugins");
    let checksums_path = config_dir.join("ghostctl/plugin_checksums.txt");

    if !plugins_dir.exists() {
        println!("No plugins directory found");
        return;
    }

    let mut checksums = String::new();
    checksums.push_str("# Plugin checksums (SHA256)\n");
    checksums.push_str("# Generated by ghostctl\n\n");

    match fs::read_dir(&plugins_dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    if ext == "lua" || ext == "sh" {
                        if let (Some(filename), Ok(hash)) = (
                            path.file_name().and_then(|n| n.to_str()),
                            calculate_file_checksum(&path),
                        ) {
                            checksums.push_str(&format!("{}  {}\n", hash, filename));
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to read plugins directory: {}", e);
            return;
        }
    }

    match fs::write(&checksums_path, &checksums) {
        Ok(_) => println!("Checksums written to: {}", checksums_path.display()),
        Err(e) => println!("Failed to write checksums: {}", e),
    }
}

// Re-export for external use
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Hex encoding module (minimal implementation)
mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes
            .as_ref()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}
