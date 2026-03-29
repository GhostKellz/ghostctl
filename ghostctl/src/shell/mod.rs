use std::process::Command;

pub mod terminals;
pub mod zsh;

pub fn setup() {
    println!("ghostctl :: Shell setup (ZSH, Oh My Zsh, Powerlevel10k, plugins)");
    crate::shell::zsh::install_zsh();
}

#[allow(dead_code)]
pub fn install_zsh() {
    println!("Installing ZSH...");
    let is_installed = Command::new("which")
        .arg("zsh")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if is_installed {
        println!("ZSH is already installed.");
    } else {
        // Try pacman first (direct command, no shell)
        let pacman_status = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "zsh"])
            .status();

        let success = match pacman_status {
            Ok(s) if s.success() => true,
            _ => {
                // Try yay as fallback
                Command::new("yay")
                    .args(["-S", "--noconfirm", "zsh"])
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false)
            }
        };

        if success {
            println!("ZSH installed successfully.");
        } else {
            eprintln!("Failed to install ZSH. Please install it manually.");
        }
    }
}

#[allow(dead_code)]
pub fn install_ohmyzsh() {
    println!("Installing Oh My Zsh...");
    let Some(home) = dirs::home_dir() else {
        eprintln!("Failed to determine home directory.");
        return;
    };
    let ohmyzsh_dir = home.join(".oh-my-zsh");
    if ohmyzsh_dir.exists() {
        println!("Oh My Zsh is already installed.");
    } else {
        // Use the secure installation method from zsh.rs
        // which downloads the script first, verifies it, and then runs it
        println!("For secure Oh My Zsh installation, use 'ghostctl shell' instead.");
        println!("That downloads and verifies the install script before execution.");
    }
}

#[allow(dead_code)]
pub fn install_powerlevel10k() {
    println!("Installing Powerlevel10k...");
    let Some(home) = dirs::home_dir() else {
        eprintln!("Failed to determine home directory.");
        return;
    };
    let p10k_dir = home.join(".oh-my-zsh/custom/themes/powerlevel10k");
    if p10k_dir.exists() {
        println!("Powerlevel10k is already installed.");
    } else {
        let Some(p10k_path) = p10k_dir.to_str() else {
            eprintln!("Failed to convert path to string.");
            return;
        };
        let status = Command::new("git")
            .args([
                "clone",
                "--depth=1",
                "https://github.com/romkatv/powerlevel10k.git",
                p10k_path,
            ])
            .status();

        match status {
            Ok(s) if s.success() => println!("Powerlevel10k installed successfully."),
            Ok(s) => eprintln!(
                "Failed to install Powerlevel10k (exit code: {})",
                s.code().unwrap_or(-1)
            ),
            Err(e) => eprintln!("Failed to run git clone: {}", e),
        }
    }
}

#[allow(dead_code)]
pub fn set_default_zsh() {
    println!("Setting ZSH as default shell...");

    // Find the actual zsh path instead of using shell expansion
    let zsh_path = Command::new("which").arg("zsh").output();

    let zsh_path = match zsh_path {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => {
            // Common fallback paths
            if std::path::Path::new("/usr/bin/zsh").exists() {
                "/usr/bin/zsh".to_string()
            } else if std::path::Path::new("/bin/zsh").exists() {
                "/bin/zsh".to_string()
            } else {
                eprintln!("Could not find zsh path. Please install zsh first.");
                return;
            }
        }
    };

    let status = Command::new("chsh").args(["-s", &zsh_path]).status();

    match status {
        Ok(s) if s.success() => println!("ZSH set as default shell."),
        Ok(s) => eprintln!(
            "Failed to set ZSH as default shell (exit code: {}). Try: chsh -s {}",
            s.code().unwrap_or(-1),
            zsh_path
        ),
        Err(e) => eprintln!("Failed to run chsh: {}", e),
    }
}

#[allow(dead_code)]
pub fn install_tmux() {
    println!("Installing tmux...");
    let is_installed = Command::new("which")
        .arg("tmux")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if is_installed {
        println!("tmux is already installed.");
    } else {
        // Try pacman first (direct command, no shell)
        let pacman_status = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "tmux"])
            .status();

        let success = match pacman_status {
            Ok(s) if s.success() => true,
            _ => {
                // Try yay as fallback
                Command::new("yay")
                    .args(["-S", "--noconfirm", "tmux"])
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false)
            }
        };

        if success {
            println!("tmux installed successfully.");
        } else {
            eprintln!("Failed to install tmux. Please install it manually.");
        }
    }
}
