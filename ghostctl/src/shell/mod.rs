use std::process::Command;

pub mod zsh;
pub mod terminals;

pub fn setup() {
    println!("ghostctl :: Shell Setup");
}

pub fn install_zsh() {
    println!("Installing ZSH...");
    let is_installed = Command::new("which").arg("zsh").output().map(|o| o.status.success()).unwrap_or(false);
    if is_installed {
        println!("ZSH is already installed.");
    } else {
        let status = Command::new("sh")
            .arg("-c")
            .arg("sudo pacman -S --noconfirm zsh || yay -S --noconfirm zsh")
            .status()
            .expect("failed to execute install command");
        if status.success() {
            println!("ZSH installed successfully.");
        } else {
            println!("Failed to install ZSH. Please install it manually.");
            return;
        }
    }
}

pub fn install_ohmyzsh() {
    println!("Installing Oh My Zsh...");
    let home = dirs::home_dir().unwrap();
    let ohmyzsh_dir = home.join(".oh-my-zsh");
    if ohmyzsh_dir.exists() {
        println!("Oh My Zsh is already installed.");
    } else {
        let status = Command::new("sh")
            .arg("-c")
            .arg("RUNZSH=no CHSH=no sh -c '$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)'")
            .status()
            .expect("failed to execute install command");
        if status.success() {
            println!("Oh My Zsh installed successfully.");
        } else {
            println!("Failed to install Oh My Zsh. Please install it manually.");
            return;
        }
    }
}

pub fn install_powerlevel10k() {
    println!("Installing Powerlevel10k...");
    let home = dirs::home_dir().unwrap();
    let p10k_dir = home.join(".oh-my-zsh/custom/themes/powerlevel10k");
    if p10k_dir.exists() {
        println!("Powerlevel10k is already installed.");
    } else {
        let status = Command::new("git")
            .args(["clone", "--depth=1", "https://github.com/romkatv/powerlevel10k.git", p10k_dir.to_str().unwrap()])
            .status()
            .expect("failed to clone Powerlevel10k");
        if status.success() {
            println!("Powerlevel10k installed successfully.");
        } else {
            println!("Failed to install Powerlevel10k. Please install it manually.");
            return;
        }
    }
}

pub fn set_default_zsh() {
    println!("Setting ZSH as default shell...");
    let status = Command::new("chsh")
        .arg("-s")
        .arg("$(which zsh)")
        .status()
        .expect("failed to set default shell");
    if status.success() {
        println!("ZSH set as default shell.");
    } else {
        println!("Failed to set ZSH as default shell. Please do it manually.");
    }
}

pub fn install_tmux() {
    println!("Installing tmux...");
    let is_installed = Command::new("which").arg("tmux").output().map(|o| o.status.success()).unwrap_or(false);
    if is_installed {
        println!("tmux is already installed.");
    } else {
        let status = Command::new("sh")
            .arg("-c")
            .arg("sudo pacman -S --noconfirm tmux || yay -S --noconfirm tmux")
            .status()
            .expect("failed to execute install command");
        if status.success() {
            println!("tmux installed successfully.");
        } else {
            println!("Failed to install tmux. Please install it manually.");
        }
    }
}
