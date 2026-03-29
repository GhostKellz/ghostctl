use dialoguer::{Select, theme::ColorfulTheme};
use std::fs;
use std::process::Command;

pub fn health_check_menu() {
    println!("🏥 Neovim Health Check & Diagnostics");
    println!("====================================");

    let options = [
        "🔍 Run Full Health Check",
        "📊 System Requirements Check",
        "🔌 Plugin Health Check",
        "🛠️  LSP Status Check",
        "🌳 Treesitter Status",
        "📋 Configuration Analysis",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Health Check Options")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => run_full_health_check(),
        1 => system_requirements_check(),
        2 => plugin_health_check(),
        3 => lsp_status_check(),
        4 => treesitter_status(),
        5 => configuration_analysis(),
        _ => return,
    }
}

fn run_full_health_check() {
    println!("🔍 Running Full Neovim Health Check");
    println!("===================================");

    let _ = Command::new("nvim")
        .args(&["--headless", "+checkhealth", "+qall"])
        .status();
}

fn system_requirements_check() {
    println!("📊 System Requirements Check");
    println!("===========================");

    let tools = [
        ("nvim", "Neovim"),
        ("git", "Git"),
        ("curl", "curl"),
        ("node", "Node.js"),
        ("npm", "npm"),
        ("python3", "Python 3"),
        ("pip", "pip"),
        ("make", "make"),
        ("gcc", "GCC"),
        ("unzip", "unzip"),
    ];

    for (bin, name) in &tools {
        match Command::new("which").arg(bin).output() {
            Ok(output) if output.status.success() => {
                println!("✅ {} is installed", name);
            }
            _ => {
                println!("❌ {} is NOT installed", name);
            }
        }
    }
}

fn plugin_health_check() {
    println!("🔌 Plugin Health Check");
    println!("======================");

    let Some(home) = dirs::home_dir() else {
        println!("❌ Could not determine home directory");
        return;
    };
    let nvim_data = home.join(".local/share/nvim");

    if nvim_data.exists() {
        println!("✅ Neovim data directory found: {:?}", nvim_data);

        let lazy_dir = nvim_data.join("lazy");
        if lazy_dir.exists() {
            println!("✅ Lazy.nvim plugin manager found");

            // Count plugins
            if let Ok(entries) = fs::read_dir(&lazy_dir) {
                let plugin_count = entries.count();
                println!("📦 {} plugins installed", plugin_count);
            }
        } else {
            println!("❌ No plugin manager found");
        }
    } else {
        println!("❌ Neovim data directory not found");
    }
}

fn lsp_status_check() {
    println!("🛠️  LSP Status Check");
    println!("==================");

    let _ = Command::new("nvim")
        .args(&["--headless", "+checkhealth lsp", "+qall"])
        .status();
}

fn treesitter_status() {
    println!("🌳 Treesitter Status");
    println!("===================");

    let _ = Command::new("nvim")
        .args(&["--headless", "+checkhealth nvim-treesitter", "+qall"])
        .status();
}

fn configuration_analysis() {
    println!("📋 Configuration Analysis");
    println!("========================");

    let Some(home) = dirs::home_dir() else {
        println!("❌ Could not determine home directory");
        return;
    };
    let nvim_config = home.join(".config/nvim");

    if nvim_config.exists() {
        println!("✅ Neovim config found: {:?}", nvim_config);

        let init_file = nvim_config.join("init.lua");
        if init_file.exists() {
            println!("✅ init.lua found");
        } else {
            let init_vim = nvim_config.join("init.vim");
            if init_vim.exists() {
                println!("✅ init.vim found");
            } else {
                println!("❌ No init file found");
            }
        }
    } else {
        println!("❌ No Neovim config found");
    }
}

#[allow(dead_code)]
pub fn run_checks() {
    println!("- Checking Neovim install: `nvim --version`");
    println!("- Checking Lua plugin health: `:checkhealth`");
}
