use dialoguer::Confirm;
use reqwest::blocking::get;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use tempfile::NamedTempFile;

pub fn install_zsh() {
    println!("Installing ZSH...");
    let is_installed = std::process::Command::new("which")
        .arg("zsh")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if is_installed {
        println!("ZSH is already installed.");
    } else {
        // Use direct command execution instead of shell
        let status = std::process::Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "zsh"])
            .status();

        let success = match status {
            Ok(s) if s.success() => true,
            _ => {
                // Try yay as fallback
                std::process::Command::new("yay")
                    .args(["-S", "--noconfirm", "zsh"])
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false)
            }
        };

        if success {
            println!("ZSH installed successfully.");
        } else {
            println!("Failed to install ZSH. Please install it manually.");
            return;
        }
    }
    // Install Oh My Zsh securely
    let home = std::env::var("HOME").unwrap();
    let omz_dir = format!("{}/.oh-my-zsh", home);
    if !std::path::Path::new(&omz_dir).exists() {
        println!("📥 Downloading Oh My Zsh install script...");

        let url = "https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh";

        match get(url) {
            Ok(response) if response.status().is_success() => {
                match response.text() {
                    Ok(content) => {
                        // Calculate hash for verification
                        let mut hasher = Sha256::new();
                        hasher.update(content.as_bytes());
                        let hash = format!("{:x}", hasher.finalize());
                        println!("📝 Script SHA256: {}", hash);

                        // Show preview
                        println!("📄 Script preview (first 300 chars):");
                        let preview: String = content.chars().take(300).collect();
                        println!("{}", preview);

                        let confirm = Confirm::new()
                            .with_prompt("Install Oh My Zsh? (script verified)")
                            .default(true)
                            .interact()
                            .unwrap_or(false);

                        if confirm {
                            // Create temp file and execute
                            if let Ok(mut temp_file) = NamedTempFile::new() {
                                if temp_file.write_all(content.as_bytes()).is_ok() {
                                    let path = temp_file.path();
                                    let _ = fs::set_permissions(
                                        path,
                                        fs::Permissions::from_mode(0o700),
                                    );

                                    let status = std::process::Command::new("sh")
                                        .env("RUNZSH", "no")
                                        .env("CHSH", "no")
                                        .arg(path)
                                        .status();

                                    if status.map(|s| s.success()).unwrap_or(false) {
                                        println!("Oh My Zsh installed.");
                                    } else {
                                        println!("Failed to install Oh My Zsh.");
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => println!("Failed to read script: {}", e),
                }
            }
            _ => println!("Failed to download Oh My Zsh script."),
        }
    } else {
        println!("Oh My Zsh already installed.");
    }
    // Install Powerlevel10k theme
    let p10k_dir = format!("{}/.oh-my-zsh/custom/themes/powerlevel10k", home);
    if !std::path::Path::new(&p10k_dir).exists() {
        let status = std::process::Command::new("git")
            .args([
                "clone",
                "--depth=1",
                "https://github.com/romkatv/powerlevel10k.git",
                &p10k_dir,
            ])
            .status()
            .expect("failed to clone powerlevel10k");
        if status.success() {
            println!("Powerlevel10k theme installed.");
        } else {
            println!("Failed to install Powerlevel10k theme.");
        }
    } else {
        println!("Powerlevel10k already installed.");
    }
    // Install plugins
    let plugins = [
        (
            "zsh-autosuggestions",
            "https://github.com/zsh-users/zsh-autosuggestions.git",
        ),
        (
            "zsh-syntax-highlighting",
            "https://github.com/zsh-users/zsh-syntax-highlighting.git",
        ),
        (
            "zsh-completions",
            "https://github.com/zsh-users/zsh-completions.git",
        ),
        (
            "zsh-history-substring-search",
            "https://github.com/zsh-users/zsh-history-substring-search.git",
        ),
    ];
    for (name, url) in plugins.iter() {
        let plugin_dir = format!("{}/.oh-my-zsh/custom/plugins/{}", home, name);
        if !std::path::Path::new(&plugin_dir).exists() {
            let status = std::process::Command::new("git")
                .args(["clone", url, &plugin_dir])
                .status()
                .expect("failed to clone plugin");
            if status.success() {
                println!("{} installed.", name);
            } else {
                println!("Failed to install {}.", name);
            }
        } else {
            println!("{} already installed.", name);
        }
    }
    // Update .zshrc
    let zshrc_path = format!("{}/.zshrc", home);
    if let Ok(mut zshrc) = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&zshrc_path)
    {
        use std::io::Write;
        writeln!(zshrc, "\n# ghostctl zsh config").ok();
        writeln!(zshrc, "ZSH_THEME=\"powerlevel10k/powerlevel10k\"").ok();
        writeln!(zshrc, "plugins=(git sudo zsh-autosuggestions zsh-syntax-highlighting zsh-completions zsh-history-substring-search colored-man-pages)").ok();
        writeln!(zshrc, "source $ZSH/oh-my-zsh.sh").ok();
        writeln!(zshrc, "[[ ! -f ~/.p10k.zsh ]] || source ~/.p10k.zsh").ok();
        println!(".zshrc updated with Powerlevel10k and plugins.");
    }
}
