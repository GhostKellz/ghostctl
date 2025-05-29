use std::process::Command;
use std::fs;
use dirs;

pub fn setup_ghostty() {
    println!("ghostctl :: Setup Ghostty terminal emulator");
    let is_installed = Command::new("which").arg("ghostty").output().map(|o| o.status.success()).unwrap_or(false);
    if is_installed {
        println!("Ghostty is already installed.");
    } else {
        println!("Ghostty not found. Installing via pacman/yay...");
        let status = Command::new("sh")
            .arg("-c")
            .arg("sudo pacman -S --noconfirm ghostty || yay -S --noconfirm ghostty")
            .status()
            .expect("failed to execute install command");
        if status.success() {
            println!("Ghostty installed successfully.");
        } else {
            println!("Failed to install Ghostty. Please install it manually.");
        }
    }
    let config_dir = dirs::home_dir().unwrap().join(".config/ghostty");
    fs::create_dir_all(&config_dir).unwrap();
    let config_file = config_dir.join("ghostty.toml");
    if !config_file.exists() {
        fs::write(&config_file, "# Default Ghostty config\n").unwrap();
        println!("Created default Ghostty config at {:?}", config_file);
    } else {
        println!("Ghostty config already exists at {:?}", config_file);
    }
}

pub fn setup_wezterm() {
    println!("ghostctl :: Setup WezTerm terminal emulator");
    let is_installed = Command::new("which").arg("wezterm").output().map(|o| o.status.success()).unwrap_or(false);
    if is_installed {
        println!("WezTerm is already installed.");
    } else {
        println!("WezTerm not found. Installing via pacman/yay...");
        let status = Command::new("sh")
            .arg("-c")
            .arg("sudo pacman -S --noconfirm wezterm || yay -S --noconfirm wezterm")
            .status()
            .expect("failed to execute install command");
        if status.success() {
            println!("WezTerm installed successfully.");
        } else {
            println!("Failed to install WezTerm. Please install it manually.");
        }
    }
    let config_dir = dirs::home_dir().unwrap().join(".config/wezterm");
    fs::create_dir_all(&config_dir).unwrap();
    let config_file = config_dir.join("wezterm.lua");
    if !config_file.exists() {
        fs::write(&config_file, "-- Default WezTerm config\n").unwrap();
        println!("Created default WezTerm config at {:?}", config_file);
    } else {
        println!("WezTerm config already exists at {:?}", config_file);
    }
}