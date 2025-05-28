use std::process::Command;
use std::fs;
use dirs;

// NOTE: Add this to your Cargo.toml:
// chrono = "0.4"

pub mod setup;
pub mod diagnostics;
pub mod plugins;

pub fn install() {
    println!("ghostctl :: Neovim Setup");
    println!("Choose a Neovim distro: LazyVim or Kickstart");
    let distros = ["LazyVim", "Kickstart", "Back"];
    let selection = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Select Neovim Distro")
        .items(&distros)
        .default(0)
        .interact()
        .unwrap();
    let home = dirs::home_dir().unwrap();
    let nvim_config = home.join(".config/nvim");
    if nvim_config.exists() {
        println!("Backing up existing Neovim config...");
        let backup = home.join(format!(".config/nvim.backup-{}", chrono::Utc::now().timestamp()));
        fs::rename(&nvim_config, &backup).unwrap();
        println!("Backed up to {:?}", backup);
    }
    match selection {
        0 => { // LazyVim
            println!("Cloning LazyVim...");
            let _ = Command::new("git")
                .args(["clone", "https://github.com/LazyVim/starter", nvim_config.to_str().unwrap()])
                .status();
        },
        1 => { // Kickstart
            println!("Cloning Kickstart...");
            let _ = Command::new("git")
                .args(["clone", "https://github.com/nvim-lua/kickstart.nvim", nvim_config.to_str().unwrap()])
                .status();
        },
        _ => {
            println!("Aborted Neovim setup.");
            return;
        }
    }
    println!("Running Neovim to install plugins...");
    let _ = Command::new("nvim").args(["--headless", "+Lazy! sync", "+qa"]).status();
    println!("Neovim setup complete!");
}

pub fn diagnostics() {
    println!("ghostctl :: Neovim Diagnostics");
    let tools = [
        ("nvim", "Neovim"),
        ("git", "Git"),
        ("curl", "curl"),
    ];
    for (bin, name) in tools.iter() {
        let found = Command::new("which").arg(bin).output().map(|o| o.status.success()).unwrap_or(false);
        if found {
            println!("[OK]   {} is installed", name);
        } else {
            println!("[MISS] {} is NOT installed", name);
        }
    }
}

pub fn list_plugins() {
    println!("ghostctl :: List Neovim Plugins");
    let home = dirs::home_dir().unwrap();
    let nvim_dir = home.join(".config/nvim");
    let plugin_dir = nvim_dir.join(".local/share/nvim");
    if plugin_dir.exists() {
        println!("Plugins directory: {:?}", plugin_dir);
        // Optionally, list plugin folders/files
    } else {
        println!("No plugins found (directory missing)");
    }
}

pub fn update_plugins() {
    println!("Updating Neovim plugins...");
    let status = Command::new("nvim")
        .args(["--headless", "+Lazy! sync", "+qa"])
        .status();
    match status {
        Ok(s) if s.success() => println!("Plugins updated successfully."),
        _ => println!("Failed to update plugins. Is Neovim installed?"),
    }
}
