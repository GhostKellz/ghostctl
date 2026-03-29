use dialoguer::{Select, theme::ColorfulTheme};
use std::fs;
use std::process::Command;

// NOTE: Add this to your Cargo.toml:
// chrono = "0.4"

pub mod diagnostics;
pub mod mason;
pub mod plugins;
pub mod setup;

pub fn nvim_menu() {
    println!("🚀 Neovim Management");
    println!("===================");

    let options = [
        "🔽 Install Neovim Distribution",
        "🔨 Mason - LSP & Tool Management",
        "🏥 Health Check & Diagnostics",
        "🔌 Plugin Management",
        "🔧 Configuration Tools",
        "📊 Show Neovim Information",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Neovim Management")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => install(),
        1 => mason::mason_menu(),
        2 => diagnostics::health_check_menu(),
        3 => plugins::plugin_management(),
        4 => setup::configuration_menu(),
        5 => show_nvim_info(),
        _ => return,
    }
}

pub fn install() {
    println!("ghostctl :: Neovim Setup");
    println!("Choose a Neovim distro: LazyVim or Kickstart");
    let distros = ["LazyVim", "Kickstart", "Back"];
    let Ok(selection) = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Select Neovim Distro")
        .items(&distros)
        .default(0)
        .interact()
    else {
        return;
    };
    let Some(home) = dirs::home_dir() else {
        println!("Could not determine home directory");
        return;
    };
    let nvim_config = home.join(".config/nvim");
    if nvim_config.exists() {
        println!("Backing up existing Neovim config...");
        let backup = home.join(format!(
            ".config/nvim.backup-{}",
            chrono::Utc::now().timestamp()
        ));
        if let Err(e) = fs::rename(&nvim_config, &backup) {
            println!("Failed to backup config: {}", e);
            return;
        }
        println!("Backed up to {:?}", backup);
    }
    let Some(nvim_config_str) = nvim_config.to_str() else {
        println!("Invalid config path");
        return;
    };
    match selection {
        0 => {
            // LazyVim
            println!("Cloning LazyVim...");
            let _ = Command::new("git")
                .args([
                    "clone",
                    "https://github.com/LazyVim/starter",
                    nvim_config_str,
                ])
                .status();
        }
        1 => {
            // Kickstart
            println!("Cloning Kickstart...");
            let _ = Command::new("git")
                .args([
                    "clone",
                    "https://github.com/nvim-lua/kickstart.nvim",
                    nvim_config_str,
                ])
                .status();
        }
        _ => {
            println!("Aborted Neovim setup.");
            return;
        }
    }
    println!("Running Neovim to install plugins...");
    let _ = Command::new("nvim")
        .args(["--headless", "+Lazy! sync", "+qa"])
        .status();
    println!("Neovim setup complete!");
}

#[allow(dead_code)]
pub fn diagnostics() {
    println!("ghostctl :: Neovim Diagnostics");
    let tools = [("nvim", "Neovim"), ("git", "Git"), ("curl", "curl")];
    for (bin, name) in tools.iter() {
        let found = Command::new("which")
            .arg(bin)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        if found {
            println!("[OK]   {} is installed", name);
        } else {
            println!("[MISS] {} is NOT installed", name);
        }
    }
}

#[allow(dead_code)]
pub fn list_plugins() {
    println!("ghostctl :: List Neovim Plugins");
    let Some(home) = dirs::home_dir() else {
        println!("Could not determine home directory");
        return;
    };
    let nvim_dir = home.join(".config/nvim");
    let plugin_dir = nvim_dir.join(".local/share/nvim");
    if plugin_dir.exists() {
        println!("Plugins directory: {:?}", plugin_dir);
        // Optionally, list plugin folders/files
    } else {
        println!("No plugins found (directory missing)");
    }
}

#[allow(dead_code)]
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

fn show_nvim_info() {
    println!("Neovim Information");
    println!("====================");

    // Check Neovim version
    let _ = Command::new("nvim").args(&["--version"]).status();

    // Show config location
    let Some(home) = dirs::home_dir() else {
        println!("Could not determine home directory");
        return;
    };
    let nvim_config = home.join(".config/nvim");
    println!("Config location: {:?}", nvim_config);

    if nvim_config.exists() {
        println!("Neovim config found");
    } else {
        println!("No Neovim config found");
    }
}

pub fn install_neovim() {
    println!("🚀 Installing Neovim");
    install();
}

pub fn install_lazyvim() {
    println!("⚡ Installing LazyVim");
    setup::install_lazyvim();
}
