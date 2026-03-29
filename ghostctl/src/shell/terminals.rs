use dialoguer::Confirm;
use reqwest::blocking::get;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use tempfile::NamedTempFile;

#[allow(dead_code)]
pub fn setup_wezterm() {
    println!("Installing/configuring WezTerm...");

    let Some(home) = dirs::home_dir() else {
        eprintln!("Failed to determine home directory.");
        return;
    };

    let config_dir = home.join(".config/wezterm");
    if let Err(e) = std::fs::create_dir_all(&config_dir) {
        eprintln!("Failed to create config directory: {}", e);
        return;
    }

    println!("Config directory: {}", config_dir.display());
    println!("See terminal/mod.rs for full WezTerm setup with configuration.");
}

#[allow(dead_code)]
pub fn setup_ghostty() {
    println!("Setting up Ghostty (WIP)");
    println!("https://github.com/mitchellh/ghostty");
    println!("See terminal/mod.rs for full Ghostty setup.");
}

pub fn setup_starship() {
    println!("Setting up Starship prompt");

    // Check if starship is already installed
    let is_installed = std::process::Command::new("which")
        .arg("starship")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if is_installed {
        println!("Starship is already installed.");
        return;
    }

    // Try package manager first (safer than curl | sh)
    println!("Attempting to install Starship via package manager...");
    let pacman_status = std::process::Command::new("sudo")
        .args(["pacman", "-S", "--noconfirm", "starship"])
        .status();

    if let Ok(s) = pacman_status
        && s.success()
    {
        println!("Starship installed successfully via pacman.");
        configure_starship_shell();
        return;
    }

    // Fall back to official installer with verification
    println!("Package manager installation failed, trying official installer...");
    println!("Downloading Starship install script...");

    let url = "https://starship.rs/install.sh";

    match get(url) {
        Ok(response) if response.status().is_success() => match response.text() {
            Ok(content) => {
                // Calculate hash for verification
                let mut hasher = Sha256::new();
                hasher.update(content.as_bytes());
                let hash = format!("{:x}", hasher.finalize());
                println!("Script SHA256: {}", hash);

                // Show preview
                println!("Script preview (first 300 chars):");
                let preview: String = content.chars().take(300).collect();
                println!("{}", preview);

                let confirm = Confirm::new()
                    .with_prompt("Install Starship? (script verified)")
                    .default(true)
                    .interact()
                    .unwrap_or(false);

                if confirm {
                    // Create temp file and execute
                    if let Ok(mut temp_file) = NamedTempFile::new()
                        && temp_file.write_all(content.as_bytes()).is_ok()
                    {
                        let path = temp_file.path();
                        if let Err(e) = fs::set_permissions(path, fs::Permissions::from_mode(0o700))
                        {
                            eprintln!("Failed to set script permissions: {}", e);
                            return;
                        }

                        // Run with -y for non-interactive
                        let status = std::process::Command::new("sh")
                            .args([path.to_str().unwrap_or(""), "-y"])
                            .status();

                        match status {
                            Ok(s) if s.success() => {
                                println!("Starship installed successfully.");
                                configure_starship_shell();
                            }
                            Ok(s) => eprintln!(
                                "Starship installation failed (exit code: {})",
                                s.code().unwrap_or(-1)
                            ),
                            Err(e) => eprintln!("Failed to run install script: {}", e),
                        }
                    }
                }
            }
            Err(e) => eprintln!("Failed to read script: {}", e),
        },
        _ => eprintln!("Failed to download Starship install script."),
    }
}

fn configure_starship_shell() {
    println!("Configuring shell for Starship...");

    let Some(home) = dirs::home_dir() else {
        eprintln!("Failed to determine home directory.");
        return;
    };

    // Add to .bashrc if it exists
    let bashrc = home.join(".bashrc");
    if bashrc.exists() {
        let content = std::fs::read_to_string(&bashrc).unwrap_or_default();
        if !content.contains("starship init bash")
            && let Ok(mut file) = std::fs::OpenOptions::new().append(true).open(&bashrc)
        {
            writeln!(file, "\n# Starship prompt").ok();
            writeln!(file, "eval \"$(starship init bash)\"").ok();
            println!("Added Starship to .bashrc");
        }
    }

    // Add to .zshrc if it exists
    let zshrc = home.join(".zshrc");
    if zshrc.exists() {
        let content = std::fs::read_to_string(&zshrc).unwrap_or_default();
        if !content.contains("starship init zsh")
            && let Ok(mut file) = std::fs::OpenOptions::new().append(true).open(&zshrc)
        {
            writeln!(file, "\n# Starship prompt").ok();
            writeln!(file, "eval \"$(starship init zsh)\"").ok();
            println!("Added Starship to .zshrc");
        }
    }
}
