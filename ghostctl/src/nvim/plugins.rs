use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::fs;
use std::process::Command;

pub fn plugin_management() {
    println!("🔌 Neovim Plugin Management");
    println!("===========================");

    let options = [
        "📋 List Installed Plugins",
        "🔄 Update All Plugins",
        "🧹 Clean Unused Plugins",
        "📦 Plugin Manager Status",
        "🔍 Search Popular Plugins",
        "⚙️  Plugin Configuration",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Plugin Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => list_installed_plugins(),
        1 => update_all_plugins(),
        2 => clean_unused_plugins(),
        3 => plugin_manager_status(),
        4 => search_popular_plugins(),
        5 => plugin_configuration(),
        _ => return,
    }
}

fn list_installed_plugins() {
    println!("📋 Installed Plugins");
    println!("===================");

    let home = dirs::home_dir().unwrap();
    let lazy_dir = home.join(".local/share/nvim/lazy");

    if lazy_dir.exists() {
        println!("📂 Lazy.nvim plugins:");
        if let Ok(entries) = fs::read_dir(&lazy_dir) {
            for entry in entries.flatten() {
                if entry.file_type().unwrap().is_dir() {
                    println!("  📦 {}", entry.file_name().to_string_lossy());
                }
            }
        }
    } else {
        println!("❌ No Lazy.nvim plugins found");
    }

    // Check for Packer
    let packer_dir = home.join(".local/share/nvim/site/pack/packer");
    if packer_dir.exists() {
        println!("\n📂 Packer plugins found");
    }
}

fn update_all_plugins() {
    println!("🔄 Updating All Plugins");
    println!("=======================");

    let confirm = Confirm::new()
        .with_prompt("Update all Neovim plugins?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("🔄 Running plugin update...");
        let _ = Command::new("nvim")
            .args(&["--headless", "+Lazy! sync", "+qall"])
            .status();
        println!("✅ Plugin update complete");
    }
}

fn clean_unused_plugins() {
    println!("🧹 Cleaning Unused Plugins");
    println!("==========================");

    let confirm = Confirm::new()
        .with_prompt("Clean unused plugins?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("🧹 Cleaning unused plugins...");
        let _ = Command::new("nvim")
            .args(&["--headless", "+Lazy! clean", "+qall"])
            .status();
        println!("✅ Cleanup complete");
    }
}

fn plugin_manager_status() {
    println!("📦 Plugin Manager Status");
    println!("========================");

    let home = dirs::home_dir().unwrap();

    // Check Lazy.nvim
    let lazy_dir = home.join(".local/share/nvim/lazy");
    if lazy_dir.exists() {
        println!("✅ Lazy.nvim detected");
        if let Ok(entries) = fs::read_dir(&lazy_dir) {
            let count = entries.count();
            println!("   📦 {} plugins installed", count);
        }
    }

    // Check Packer
    let packer_dir = home.join(".local/share/nvim/site/pack/packer");
    if packer_dir.exists() {
        println!("✅ Packer detected");
    }

    // Check vim-plug
    let plug_dir = home.join(".local/share/nvim/plugged");
    if plug_dir.exists() {
        println!("✅ vim-plug detected");
    }

    if !lazy_dir.exists() && !packer_dir.exists() && !plug_dir.exists() {
        println!("❌ No plugin manager detected");
    }
}

fn search_popular_plugins() {
    println!("🔍 Popular Neovim Plugins");
    println!("=========================");

    let categories = [
        (
            "🔧 Essential",
            vec![
                "nvim-treesitter - Syntax highlighting",
                "telescope.nvim - Fuzzy finder",
                "nvim-lspconfig - LSP configuration",
                "nvim-cmp - Completion engine",
            ],
        ),
        (
            "🎨 UI/UX",
            vec![
                "lualine.nvim - Status line",
                "nvim-tree.lua - File explorer",
                "toggleterm.nvim - Terminal integration",
                "which-key.nvim - Key binding helper",
            ],
        ),
        (
            "🛠️  Development",
            vec![
                "mason.nvim - LSP/DAP/Linter installer",
                "null-ls.nvim - Diagnostics/formatting",
                "gitsigns.nvim - Git integration",
                "nvim-dap - Debugging",
            ],
        ),
    ];

    for (category, plugins) in &categories {
        println!("\n{}", category);
        println!("{}", "=".repeat(category.len() - 2));
        for plugin in plugins {
            println!("  • {}", plugin);
        }
    }
}

fn plugin_configuration() {
    println!("⚙️  Plugin Configuration");
    println!("=======================");

    let home = dirs::home_dir().unwrap();
    let config_dir = home.join(".config/nvim");

    if config_dir.exists() {
        println!("📁 Config directory: {:?}", config_dir);

        let lua_dir = config_dir.join("lua");
        if lua_dir.exists() {
            println!("✅ Lua configuration found");
        }

        let plugin_configs = config_dir.join("lua/plugins");
        if plugin_configs.exists() {
            println!("✅ Plugin configurations found");
        }
    } else {
        println!("❌ No Neovim configuration found");
    }
}

#[allow(dead_code)]
pub fn list() {
    println!("- Listing Neovim plugins...");
    println!("- TODO: Parse lazy-lock.json or plugin dir");
}
