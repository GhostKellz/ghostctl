use chrono::Utc;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::fs;
use std::process::Command;

pub fn configuration_menu() {
    println!("🔧 Neovim Configuration Tools");
    println!("=============================");

    let options = [
        "🚀 Install Neovim Distributions",
        "📝 Edit Configuration",
        "💾 Backup Current Config",
        "📋 Configuration Templates",
        "🔄 Reset Configuration",
        "🗂️  Manage Config Files",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Configuration Tools")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_distributions(),
        1 => edit_configuration(),
        2 => backup_configuration(),
        3 => configuration_templates(),
        4 => reset_configuration(),
        5 => manage_config_files(),
        _ => (),
    }
}

fn install_distributions() {
    println!("🚀 Install Neovim Distributions");
    println!("===============================");

    let distributions = [
        ("LazyVim", "Modern Neovim setup with lazy loading"),
        ("Kickstart", "Minimal, well-documented starting config"),
        ("AstroNvim", "Feature-rich, batteries-included setup"),
        ("NvChad", "Blazing fast and beautiful config"),
        ("LunarVim", "IDE-like configuration framework"),
    ];

    let display_options: Vec<String> = distributions
        .iter()
        .map(|(name, desc)| format!("{} - {}", name, desc))
        .collect();
    let mut all_options = display_options;
    all_options.push("⬅️  Back".to_string());

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select distribution to install")
        .items(&all_options)
        .default(0)
        .interact()
        .unwrap();

    if choice < distributions.len() {
        let (name, _) = distributions[choice];
        install_specific_distribution(name);
    }
}

fn install_specific_distribution(name: &str) {
    println!("🚀 Installing {}", name);

    let home = dirs::home_dir().unwrap();
    let nvim_config = home.join(".config/nvim");

    // Backup existing config
    if nvim_config.exists() {
        let backup_name = format!("nvim.backup.{}", Utc::now().timestamp());
        let backup_path = home.join(format!(".config/{}", backup_name));

        println!("💾 Backing up existing config to: {:?}", backup_path);
        let _ = Command::new("mv")
            .args([nvim_config.to_str().unwrap(), backup_path.to_str().unwrap()])
            .status();
    }

    // Install distribution
    let repo_url = match name {
        "LazyVim" => "https://github.com/LazyVim/starter",
        "AstroNvim" => "https://github.com/AstroNvim/AstroNvim",
        "Kickstart" => "https://github.com/nvim-lua/kickstart.nvim",
        "NvChad" => "https://github.com/NvChad/NvChad",
        "LunarVim" => {
            // Special handling for LunarVim
            println!("🌙 Installing LunarVim...");
            let _ = Command::new("bash")
                .arg("-c")
                .arg("LV_BRANCH='release-1.3/neovim-0.9' bash <(curl -s https://raw.githubusercontent.com/lunarvim/lunarvim/release-1.3/neovim-0.9/utils/installer/install.sh)")
                .status();
            return;
        }
        _ => {
            println!("❌ Unknown distribution: {}", name);
            return;
        }
    };

    println!("📥 Cloning {} from {}", name, repo_url);
    let status = Command::new("git")
        .args(["clone", repo_url, nvim_config.to_str().unwrap()])
        .status();

    match status {
        Ok(status) if status.success() => {
            println!("✅ {} cloned successfully", name);

            // Special setup for LazyVim
            if name == "LazyVim" {
                setup_lazyvim_starter();
            }

            // Install plugins
            println!("📦 Installing plugins...");
            let _ = Command::new("nvim")
                .args(["--headless", "+Lazy! sync", "+qa"])
                .status();

            // Install Starship prompt
            install_starship_prompt();

            println!("🎉 {} installation complete!", name);
            println!("💡 Run 'nvim' to start using your new setup");
        }
        _ => {
            println!("❌ Failed to clone {}", name);
        }
    }
}

fn setup_lazyvim_starter() {
    println!("⚙️  Setting up LazyVim starter configuration...");

    let home = dirs::home_dir().unwrap();
    let nvim_config = home.join(".config/nvim");

    // Remove .git directory to make it your own
    let git_dir = nvim_config.join(".git");
    if git_dir.exists() {
        let _ = std::fs::remove_dir_all(&git_dir);
        println!("🗑️  Removed .git directory");
    }

    // Create initial git repository
    let _ = Command::new("git")
        .args(["init"])
        .current_dir(&nvim_config)
        .status();

    let _ = Command::new("git")
        .args(["add", "."])
        .current_dir(&nvim_config)
        .status();

    let _ = Command::new("git")
        .args(["commit", "-m", "Initial LazyVim setup"])
        .current_dir(&nvim_config)
        .status();

    println!("✅ LazyVim starter setup complete");
}

fn install_starship_prompt() {
    println!("🌟 Installing Starship prompt...");

    // Check if starship is already installed
    if Command::new("which").arg("starship").status().is_ok() {
        println!("✅ Starship already installed");
        configure_starship();
        return;
    }

    // Install starship
    let install_status = Command::new("bash")
        .arg("-c")
        .arg("curl -sS https://starship.rs/install.sh | sh -s -- -y")
        .status();

    match install_status {
        Ok(status) if status.success() => {
            println!("✅ Starship installed successfully");
            configure_starship();
        }
        _ => {
            println!("❌ Failed to install Starship");
            // Try package manager fallback
            println!("🔄 Trying package manager...");

            // Try different package managers
            let pkg_managers = [
                (
                    "pacman",
                    vec!["sudo", "pacman", "-S", "--noconfirm", "starship"],
                ),
                ("apt", vec!["sudo", "apt", "install", "-y", "starship"]),
                ("dnf", vec!["sudo", "dnf", "install", "-y", "starship"]),
                ("cargo", vec!["cargo", "install", "starship", "--locked"]),
            ];

            for (name, cmd) in &pkg_managers {
                if Command::new("which").arg(name).status().is_ok() {
                    println!("📦 Installing via {}...", name);
                    let status = Command::new(cmd[0]).args(&cmd[1..]).status();
                    if status.is_ok() && status.unwrap().success() {
                        println!("✅ Starship installed via {}", name);
                        configure_starship();
                        break;
                    }
                }
            }
        }
    }
}

fn configure_starship() {
    println!("⚙️  Configuring Starship...");

    let home = dirs::home_dir().unwrap();
    let config_dir = home.join(".config");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create starship config
    let starship_config = r#"# Starship Configuration for Neovim Development
format = """
[┌─────────────────────────────────────────────────────────────────────────────────────────────────────────────]
[│ $username$hostname$directory$git_branch$git_status$git_commit$python$rust$nodejs$golang$cmd_duration $time
[└─[$character](bold green)]"""

[username]
show_always = true
format = "[$user]($style)@"

[hostname]
format = "[$hostname]($style) "
ssh_only = false

[directory]
truncation_length = 3
truncate_to_repo = false
format = "[$path]($style)[$read_only]($read_only_style) "

[git_branch]
format = "[$symbol$branch]($style) "
symbol = "🌿 "

[git_status]
format = '([\[$all_status$ahead_behind\]]($style) )'

[time]
disabled = false
format = "🕐 [$time]($style)"

[character]
success_symbol = "[❯](bold green)"
error_symbol = "[❯](bold red)"

[cmd_duration]
min_time = 500
format = "⏱️  [$duration]($style) "

[python]
format = '[${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'

[rust]
format = "[⚡ $version](red bold)"

[nodejs]
format = "[⬢ $version](bold green) "
"#;

    let starship_config_path = config_dir.join("starship.toml");
    if let Err(e) = std::fs::write(&starship_config_path, starship_config) {
        println!("⚠️  Warning: Could not write starship config: {}", e);
    } else {
        println!("✅ Starship configuration created");
    }

    // Add to shell configs
    configure_shell_starship();
}

fn configure_shell_starship() {
    let home = dirs::home_dir().unwrap();

    // Configure bash
    let bashrc = home.join(".bashrc");
    if bashrc.exists() {
        add_starship_to_shell(&bashrc, r#"eval "$(starship init bash)""#);
    }

    // Configure zsh
    let zshrc = home.join(".zshrc");
    if zshrc.exists() {
        add_starship_to_shell(&zshrc, r#"eval "$(starship init zsh)""#);
    }

    // Configure fish
    let fish_config = home.join(".config/fish/config.fish");
    if fish_config.exists() {
        add_starship_to_shell(&fish_config, "starship init fish | source");
    }

    println!("🐚 Shell configurations updated for Starship");
    println!("💡 Restart your shell or run 'source ~/.bashrc' to enable Starship");
}

fn add_starship_to_shell(shell_config: &std::path::Path, init_command: &str) {
    if let Ok(content) = std::fs::read_to_string(shell_config) {
        if !content.contains("starship init") {
            let updated_content = format!("{}\n\n# Starship prompt\n{}\n", content, init_command);
            let _ = std::fs::write(shell_config, updated_content);
            println!("✅ Updated {:?}", shell_config.file_name().unwrap());
        } else {
            println!(
                "⏭️  Starship already configured in {:?}",
                shell_config.file_name().unwrap()
            );
        }
    }
}

fn edit_configuration() {
    println!("📝 Edit Neovim Configuration");
    println!("============================");

    let home = dirs::home_dir().unwrap();
    let nvim_config = home.join(".config/nvim");

    if !nvim_config.exists() {
        println!("❌ No Neovim configuration found");
        return;
    }

    let config_files = [
        "init.lua",
        "init.vim",
        "lua/config/options.lua",
        "lua/config/keymaps.lua",
        "lua/plugins/init.lua",
    ];

    let mut available_files = Vec::new();
    for file in &config_files {
        let file_path = nvim_config.join(file);
        if file_path.exists() {
            available_files.push(*file);
        }
    }

    if available_files.is_empty() {
        println!("❌ No standard config files found");
        return;
    }

    available_files.push("📁 Browse config directory");
    available_files.push("⬅️  Back");

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select file to edit")
        .items(&available_files)
        .default(0)
        .interact()
        .unwrap();

    if choice < available_files.len() - 2 {
        let file = available_files[choice];
        let file_path = nvim_config.join(file);
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string());

        let _ = Command::new(&editor).arg(file_path).status();
    } else if choice == available_files.len() - 2 {
        let _ = Command::new("ls")
            .args(["-la", nvim_config.to_str().unwrap()])
            .status();
    }
}

fn backup_configuration() {
    println!("💾 Backup Neovim Configuration");
    println!("==============================");

    let home = dirs::home_dir().unwrap();
    let nvim_config = home.join(".config/nvim");

    if !nvim_config.exists() {
        println!("❌ No Neovim configuration to backup");
        return;
    }

    let backup_name: String = Input::new()
        .with_prompt("Backup name")
        .default(format!("nvim-backup-{}", chrono::Utc::now().timestamp()))
        .interact_text()
        .unwrap();

    let backup_path = home.join(format!(".config/{}", backup_name));

    println!("💾 Creating backup at {:?}", backup_path);
    let status = Command::new("cp")
        .args([
            "-r",
            nvim_config.to_str().unwrap(),
            backup_path.to_str().unwrap(),
        ])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Backup created successfully"),
        _ => println!("❌ Failed to create backup"),
    }
}

fn configuration_templates() {
    println!("📋 Configuration Templates");
    println!("==========================");

    let templates = [
        "Basic init.lua template",
        "LSP configuration template",
        "Plugin management template",
        "Keybinding template",
        "Colorscheme template",
    ];

    println!("Available templates:");
    for (i, template) in templates.iter().enumerate() {
        println!("{}. {}", i + 1, template);
    }

    println!("\nTemplates help you get started with specific configurations.");
}

fn reset_configuration() {
    println!("🔄 Reset Neovim Configuration");
    println!("=============================");

    let home = dirs::home_dir().unwrap();
    let nvim_config = home.join(".config/nvim");

    if !nvim_config.exists() {
        println!("❌ No configuration to reset");
        return;
    }

    let confirm = Confirm::new()
        .with_prompt("Are you sure you want to reset your Neovim configuration? This will delete everything!")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let backup = Confirm::new()
            .with_prompt("Create backup before reset?")
            .default(true)
            .interact()
            .unwrap();

        if backup {
            backup_configuration();
        }

        let _ = fs::remove_dir_all(&nvim_config);
        println!("✅ Configuration reset complete");
    }
}

fn manage_config_files() {
    println!("🗂️  Manage Configuration Files");
    println!("=============================");

    let home = dirs::home_dir().unwrap();
    let nvim_config = home.join(".config/nvim");

    if nvim_config.exists() {
        println!("📁 Configuration directory: {:?}", nvim_config);
        let _ = Command::new("find")
            .args([
                nvim_config.to_str().unwrap(),
                "-type",
                "f",
                "-name",
                "*.lua",
                "-o",
                "-name",
                "*.vim",
            ])
            .status();
    } else {
        println!("❌ No configuration directory found");
    }
}

// AUR Helper management with reaper priority
#[allow(dead_code)]
fn get_aur_helper() -> Option<String> {
    let aur_helpers = ["reap", "paru", "yay"];

    for helper in &aur_helpers {
        if Command::new("which").arg(helper).status().is_ok() {
            return Some(helper.to_string());
        }
    }

    None
}

#[allow(dead_code)]
fn install_aur_package(package: &str) -> bool {
    if let Some(helper) = get_aur_helper() {
        println!("📦 Installing {} with {}...", package, helper);

        match helper.as_str() {
            "reap" => Command::new("reap")
                .arg(package)
                .status()
                .map(|s| s.success())
                .unwrap_or(false),
            "paru" => Command::new("paru")
                .args(["-S", "--noconfirm", package])
                .status()
                .map(|s| s.success())
                .unwrap_or(false),
            "yay" => Command::new("yay")
                .args(["-S", "--noconfirm", package])
                .status()
                .map(|s| s.success())
                .unwrap_or(false),
            _ => false,
        }
    } else {
        println!("❌ No AUR helper found. Install reaper, paru, or yay first");
        false
    }
}

#[allow(dead_code)]
pub fn install_lazyvim() {
    println!("- Cloning LazyVim...");
    println!("git clone https://github.com/LazyVim/starter ~/.config/nvim");
    // In future: shell out to run this automatically
}

#[allow(dead_code)]
pub fn install_kickstart() {
    println!("🚀 Installing Kickstart.nvim");
    println!("============================");

    let home = dirs::home_dir().unwrap();
    let nvim_config = home.join(".config/nvim");

    // Backup existing config
    if nvim_config.exists() {
        let backup_path = home.join(format!(".config/nvim.backup.{}", Utc::now().timestamp()));
        println!("📦 Backing up existing config to: {:?}", backup_path);

        let confirm = Confirm::new()
            .with_prompt("Backup existing Neovim config?")
            .default(true)
            .interact()
            .unwrap();

        if confirm {
            let _ = std::process::Command::new("mv")
                .args([nvim_config.to_str().unwrap(), backup_path.to_str().unwrap()])
                .status();
        } else {
            let _ = fs::remove_dir_all(&nvim_config);
        }
    }

    // Clone Kickstart
    println!("📥 Cloning Kickstart.nvim...");
    let status = std::process::Command::new("git")
        .args([
            "clone",
            "https://github.com/nvim-lua/kickstart.nvim.git",
            nvim_config.to_str().unwrap(),
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Kickstart.nvim installed successfully");

            // Remove .git directory
            let git_dir = nvim_config.join(".git");
            if git_dir.exists() {
                let _ = fs::remove_dir_all(&git_dir);
                println!("🔧 Removed .git directory - config is now yours");
            }

            println!("🎉 Kickstart.nvim installation complete!");
            println!("💡 Run 'nvim' to complete the setup");
        }
        _ => println!("❌ Failed to install Kickstart.nvim"),
    }
}

#[allow(dead_code)]
pub fn install_astronvim() {
    println!("🚀 Installing AstroNvim");
    println!("=======================");

    let home = dirs::home_dir().unwrap();
    let nvim_config = home.join(".config/nvim");

    // Backup existing config
    if nvim_config.exists() {
        let backup_path = home.join(format!(".config/nvim.backup.{}", Utc::now().timestamp()));
        println!("📦 Backing up existing config to: {:?}", backup_path);

        let confirm = Confirm::new()
            .with_prompt("Backup existing Neovim config?")
            .default(true)
            .interact()
            .unwrap();

        if confirm {
            let _ = std::process::Command::new("mv")
                .args([nvim_config.to_str().unwrap(), backup_path.to_str().unwrap()])
                .status();
        } else {
            let _ = fs::remove_dir_all(&nvim_config);
        }
    }

    // Clone AstroNvim
    println!("📥 Cloning AstroNvim...");
    let status = std::process::Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "https://github.com/AstroNvim/template",
            nvim_config.to_str().unwrap(),
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ AstroNvim installed successfully");

            // Remove .git directory
            let git_dir = nvim_config.join(".git");
            if git_dir.exists() {
                let _ = fs::remove_dir_all(&git_dir);
                println!("🔧 Removed .git directory - config is now yours");
            }

            println!("🎉 AstroNvim installation complete!");
            println!("💡 Run 'nvim' to complete the setup");
        }
        _ => println!("❌ Failed to install AstroNvim"),
    }
}

#[allow(dead_code)]
pub fn install_nvchad() {
    println!("🚀 Installing NvChad");
    println!("====================");

    let home = dirs::home_dir().unwrap();
    let nvim_config = home.join(".config/nvim");

    // Backup existing config
    if nvim_config.exists() {
        let backup_path = home.join(format!(".config/nvim.backup.{}", Utc::now().timestamp()));
        println!("📦 Backing up existing config to: {:?}", backup_path);

        let confirm = Confirm::new()
            .with_prompt("Backup existing Neovim config?")
            .default(true)
            .interact()
            .unwrap();

        if confirm {
            let _ = std::process::Command::new("mv")
                .args([nvim_config.to_str().unwrap(), backup_path.to_str().unwrap()])
                .status();
        } else {
            let _ = fs::remove_dir_all(&nvim_config);
        }
    }

    // Clone NvChad
    println!("📥 Cloning NvChad starter...");
    let status = std::process::Command::new("git")
        .args([
            "clone",
            "https://github.com/NvChad/starter",
            nvim_config.to_str().unwrap(),
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ NvChad installed successfully");

            // Remove .git directory
            let git_dir = nvim_config.join(".git");
            if git_dir.exists() {
                let _ = fs::remove_dir_all(&git_dir);
                println!("🔧 Removed .git directory - config is now yours");
            }

            println!("🎉 NvChad installation complete!");
            println!("💡 Run 'nvim' to complete the setup");
        }
        _ => println!("❌ Failed to install NvChad"),
    }
}
