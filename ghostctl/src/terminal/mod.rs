use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use std::fs;
use std::process::Command;

pub mod alacritty;

pub fn terminal_menu() {
    println!("💻 Terminal Setup & Configuration");
    println!("=================================");

    let options = [
        "👻 Setup Ghostty",
        "🔷 Setup WezTerm",
        "⚡ Setup Alacritty",
        "🔧 Configure terminal preferences",
        "📊 Terminal information",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Terminal Management")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => setup_ghostty(),
        1 => setup_wezterm(),
        2 => alacritty::alacritty_menu(),
        3 => configure_preferences(),
        4 => show_terminal_info(),
        _ => return,
    }
}

pub fn setup_ghostty() {
    println!("👻 Setting up Ghostty terminal emulator");
    println!("=======================================");

    // Check if Ghostty is already installed
    let is_installed = Command::new("which")
        .arg("ghostty")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if is_installed {
        println!("✅ Ghostty is already installed");
    } else {
        println!("📦 Installing Ghostty...");

        // Try different installation methods
        let install_methods = [
            ("Arch Linux (AUR)", "yay -S --noconfirm ghostty"),
            ("Build from source", ""),
            ("AppImage", ""),
        ];

        let Ok(method) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Installation method")
            .items(
                &install_methods
                    .iter()
                    .map(|(name, _)| *name)
                    .collect::<Vec<_>>(),
            )
            .default(0)
            .interact()
        else {
            return;
        };

        match method {
            0 => install_ghostty_aur(),
            1 => install_ghostty_source(),
            2 => install_ghostty_appimage(),
            _ => return,
        }
    }

    // Configure Ghostty
    configure_ghostty();
}

fn install_ghostty_aur() {
    println!("Installing Ghostty from AUR...");

    // Check if yay is available
    let yay_available = Command::new("which")
        .arg("yay")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if yay_available {
        let status = Command::new("yay")
            .args(["-S", "--noconfirm", "ghostty"])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("Ghostty installed successfully via AUR");
                return;
            }
            _ => {
                println!("Failed to install via yay, trying paru...");
            }
        }
    }

    // Try paru as fallback
    let paru_available = Command::new("which")
        .arg("paru")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if paru_available {
        let status = Command::new("paru")
            .args(["-S", "--noconfirm", "ghostty"])
            .status();

        match status {
            Ok(s) if s.success() => println!("Ghostty installed successfully via paru"),
            Ok(s) => eprintln!(
                "Failed to install Ghostty via paru (exit code: {})",
                s.code().unwrap_or(-1)
            ),
            Err(e) => eprintln!("Failed to run paru: {}", e),
        }
    } else if !yay_available {
        eprintln!("AUR helper not found. Please install yay or paru first");
    }
}

fn install_ghostty_source() {
    println!("Building Ghostty from source...");
    println!("Prerequisites: Zig compiler");

    // Check if zig is installed
    let zig_available = Command::new("which")
        .arg("zig")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !zig_available {
        eprintln!("Zig compiler not found");
        let Ok(install_zig) = Confirm::new()
            .with_prompt("Install Zig compiler?")
            .default(true)
            .interact()
        else {
            return;
        };

        if install_zig {
            println!("Installing Zig...");
            let status = Command::new("sudo")
                .args(["pacman", "-S", "--noconfirm", "zig"])
                .status();

            match status {
                Ok(s) if s.success() => println!("Zig installed successfully"),
                Ok(s) => {
                    eprintln!(
                        "Failed to install Zig (exit code: {})",
                        s.code().unwrap_or(-1)
                    );
                    return;
                }
                Err(e) => {
                    eprintln!("Failed to run pacman: {}", e);
                    return;
                }
            }
        } else {
            return;
        }
    }

    let Some(home_dir) = dirs::home_dir() else {
        eprintln!("Could not determine home directory");
        return;
    };
    let build_dir = home_dir.join("src/ghostty");

    println!("Cloning Ghostty repository...");
    let Some(build_dir_str) = build_dir.to_str() else {
        eprintln!("Invalid build directory path");
        return;
    };

    let clone_status = Command::new("git")
        .args([
            "clone",
            "https://github.com/mitchellh/ghostty",
            build_dir_str,
        ])
        .status();

    match clone_status {
        Ok(s) if s.success() => println!("Repository cloned successfully"),
        Ok(s) if s.code() == Some(128) => {
            // Directory may already exist
            println!("Repository may already exist, continuing...");
        }
        Ok(s) => {
            eprintln!(
                "Failed to clone repository (exit code: {})",
                s.code().unwrap_or(-1)
            );
            return;
        }
        Err(e) => {
            eprintln!("Failed to run git: {}", e);
            return;
        }
    }

    println!("Building Ghostty (this may take a while)...");
    let status = Command::new("zig")
        .args(["build", "-Doptimize=ReleaseFast"])
        .current_dir(&build_dir)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("Ghostty built successfully");

            // Install binary
            let Ok(install) = Confirm::new()
                .with_prompt("Install to /usr/local/bin?")
                .default(true)
                .interact()
            else {
                return;
            };

            if install {
                let binary_path = build_dir.join("zig-out/bin/ghostty");
                let Some(binary_path_str) = binary_path.to_str() else {
                    eprintln!("Invalid binary path");
                    return;
                };

                let install_status = Command::new("sudo")
                    .args([
                        "install",
                        "-Dm755",
                        binary_path_str,
                        "/usr/local/bin/ghostty",
                    ])
                    .status();

                match install_status {
                    Ok(s) if s.success() => {
                        println!("Ghostty installed to /usr/local/bin/ghostty")
                    }
                    Ok(s) => eprintln!(
                        "Failed to install binary (exit code: {})",
                        s.code().unwrap_or(-1)
                    ),
                    Err(e) => eprintln!("Failed to run install: {}", e),
                }
            }
        }
        Ok(s) => eprintln!("Build failed (exit code: {})", s.code().unwrap_or(-1)),
        Err(e) => eprintln!("Failed to run zig build: {}", e),
    }
}

fn install_ghostty_appimage() {
    println!("📦 Installing Ghostty AppImage...");
    println!("⚠️  AppImage not yet available for Ghostty");
    println!("💡 Use source build or wait for official releases");
}

fn configure_ghostty() {
    println!("⚙️  Configuring Ghostty...");

    let Some(home_dir) = dirs::home_dir() else {
        println!("❌ Could not determine home directory");
        return;
    };
    let config_dir = home_dir.join(".config/ghostty");
    if let Err(e) = fs::create_dir_all(&config_dir) {
        println!("❌ Failed to create config directory: {}", e);
        return;
    }

    // Define config constants
    const BACKGROUND: &str = "#1e1e2e";
    const FOREGROUND: &str = "#cdd6f4";
    const FONT_FAMILY: &str = "FiraCode Nerd Font";

    let config_content = format!(
        r#"# Theme
theme = "dark"
background = "{}"
foreground = "{}"

# Font
font-family = "{}"
font-size = 12

# Window
window-decoration = false
window-padding-x = 10
window-padding-y = 10

# Cursor
cursor-style = "block"
cursor-style-blink = false

# Shell integration
shell-integration = "fish"

# Key bindings
keybind = "ctrl+shift+c=copy_to_clipboard"
keybind = "ctrl+shift+v=paste_from_clipboard"
keybind = "ctrl+shift+n=new_window"
keybind = "ctrl+shift+t=new_tab"

# Mouse
mouse-hide-while-typing = true
"#,
        BACKGROUND, FOREGROUND, FONT_FAMILY
    );

    let config_file = config_dir.join("config");
    if let Err(e) = fs::write(&config_file, config_content) {
        println!("❌ Failed to write config file: {}", e);
        return;
    }

    println!("✅ Ghostty configuration saved to: {:?}", config_file);
}

pub fn setup_wezterm() {
    println!("Setting up WezTerm");
    println!("=======================================");

    let is_installed = Command::new("which")
        .arg("wezterm")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if is_installed {
        println!("WezTerm is available");
    } else {
        println!("Installing WezTerm...");

        // Try pacman first (direct command, no shell)
        let pacman_status = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "wezterm"])
            .status();

        let success = match pacman_status {
            Ok(s) if s.success() => true,
            _ => {
                // Try yay as fallback
                Command::new("yay")
                    .args(["-S", "--noconfirm", "wezterm-git"])
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false)
            }
        };

        if success {
            println!("WezTerm setup complete");
        } else {
            eprintln!("Failed to install WezTerm via package manager");
            println!("Try: https://wezfurlong.org/wezterm/installation.html");
            return;
        }
    }

    // Configure WezTerm
    configure_wezterm();
}

fn configure_wezterm() {
    println!("⚙️  Configuring WezTerm...");

    let Some(home_dir) = dirs::home_dir() else {
        println!("❌ Could not determine home directory");
        return;
    };
    let config_dir = home_dir.join(".config/wezterm");
    if let Err(e) = fs::create_dir_all(&config_dir) {
        println!("❌ Failed to create config directory: {}", e);
        return;
    }

    let config_file = config_dir.join("wezterm.lua");

    if !config_file.exists() {
        let config_content = r#"-- WezTerm Configuration
local wezterm = require 'wezterm'

local config = {}

-- Use config builder if available
if wezterm.config_builder then
  config = wezterm.config_builder()
end

-- Color scheme
config.color_scheme = 'Catppuccin Mocha'

-- Font
config.font = wezterm.font 'FiraCode Nerd Font'
config.font_size = 12.0

-- Window
config.window_decorations = "RESIZE"
config.window_padding = {
  left = 10,
  right = 10,
  top = 10,
  bottom = 10,
}

-- Tab bar
config.hide_tab_bar_if_only_one_tab = true
config.use_fancy_tab_bar = false

-- Cursor
config.default_cursor_style = 'BlinkingBlock'
config.cursor_blink_rate = 800

-- Key bindings
config.keys = {
  -- Copy/Paste
  { key = 'c', mods = 'CTRL|SHIFT', action = wezterm.action.CopyTo 'Clipboard' },
  { key = 'v', mods = 'CTRL|SHIFT', action = wezterm.action.PasteFrom 'Clipboard' },
  
  -- Tabs
  { key = 't', mods = 'CTRL|SHIFT', action = wezterm.action.SpawnTab 'CurrentPaneDomain' },
  { key = 'w', mods = 'CTRL|SHIFT', action = wezterm.action.CloseCurrentTab{ confirm = true } },
  
  -- Panes
  { key = 'd', mods = 'CTRL|SHIFT', action = wezterm.action.SplitHorizontal{ domain = 'CurrentPaneDomain' } },
  { key = 'D', mods = 'CTRL|SHIFT', action = wezterm.action.SplitVertical{ domain = 'CurrentPaneDomain' } },
}

-- Performance
config.front_end = "WebGpu"
config.webgpu_power_preference = "HighPerformance"

return config
"#;

        if let Err(e) = fs::write(&config_file, config_content) {
            println!("❌ Failed to write config file: {}", e);
            return;
        }
        println!("✅ Created WezTerm config at {:?}", config_file);
    } else {
        println!("⚠️  WezTerm config already exists at {:?}", config_file);
    }

    println!("🎨 WezTerm features enabled:");
    println!("  • GPU acceleration");
    println!("  • Catppuccin theme");
    println!("  • Nerd Font support");
    println!("  • Custom key bindings");
}

fn configure_preferences() {
    println!("🔧 Terminal Preferences");
    println!("=======================");

    let preferences = [
        "🎨 Install Nerd Fonts",
        "🌈 Setup terminal themes",
        "⌨️  Configure shell integration",
        "🔧 Set default terminal",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Preferences")
        .items(&preferences)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => install_nerd_fonts(),
        1 => setup_terminal_themes(),
        2 => configure_shell_integration(),
        3 => set_default_terminal(),
        _ => return,
    }
}

fn install_nerd_fonts() {
    println!("Installing Nerd Fonts...");

    let fonts = [
        "FiraCode Nerd Font",
        "JetBrains Mono Nerd Font",
        "Hack Nerd Font",
        "Source Code Pro Nerd Font",
        "Ubuntu Mono Nerd Font",
    ];

    let Ok(selected) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select font to install")
        .items(&fonts)
        .default(0)
        .interact()
    else {
        return;
    };

    let font_package = match selected {
        0 => "ttf-firacode-nerd",
        1 => "ttf-jetbrains-mono-nerd",
        2 => "ttf-hack-nerd",
        3 => "ttf-sourcecodepro-nerd",
        4 => "ttf-ubuntu-mono-nerd",
        _ => return,
    };

    println!("Installing {}...", fonts[selected]);
    let status = Command::new("sudo")
        .args(["pacman", "-S", "--noconfirm", font_package])
        .status();

    match status {
        Ok(s) if s.success() => println!("{} installed successfully", fonts[selected]),
        Ok(s) => eprintln!(
            "Failed to install {} (exit code: {})",
            fonts[selected],
            s.code().unwrap_or(-1)
        ),
        Err(e) => eprintln!("Failed to run pacman: {}", e),
    }
}

fn setup_terminal_themes() {
    println!("🌈 Terminal Theme Setup");
    println!("=======================");
    println!("Available themes:");
    println!("  • Catppuccin (Mocha, Macchiato, Frappé, Latte)");
    println!("  • Tokyo Night");
    println!("  • Dracula");
    println!("  • Nord");
    println!("  • Gruvbox");
    println!("\nThemes are configured per terminal in their config files");
}

fn configure_shell_integration() {
    println!("⌨️  Shell Integration");
    println!("====================");

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
    println!("Current shell: {}", shell);

    if shell.contains("fish") {
        println!("🐟 Fish shell integration available in terminal configs");
    } else if shell.contains("zsh") {
        println!("🚀 Zsh shell integration available");
    } else {
        println!("🐚 Bash shell integration available");
    }
}

fn set_default_terminal() {
    println!("Set Default Terminal");
    println!("======================");

    let terminals = ["ghostty", "wezterm", "alacritty", "kitty", "gnome-terminal"];
    let mut available_terminals = Vec::new();

    for terminal in &terminals {
        let available = Command::new("which")
            .arg(terminal)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        if available {
            available_terminals.push(*terminal);
        }
    }

    if available_terminals.is_empty() {
        eprintln!("No supported terminals found");
        return;
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select default terminal")
        .items(&available_terminals)
        .default(0)
        .interact()
    else {
        return;
    };

    let selected_terminal = available_terminals[choice];
    let terminal_path = format!("/usr/bin/{}", selected_terminal);

    // Verify the terminal binary exists at the expected path
    if !std::path::Path::new(&terminal_path).exists() {
        // Try to find the actual path
        let which_output = Command::new("which").arg(selected_terminal).output();
        match which_output {
            Ok(output) if output.status.success() => {
                let actual_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                println!(
                    "Note: {} is at {}, not {}",
                    selected_terminal, actual_path, terminal_path
                );
            }
            _ => {
                eprintln!(
                    "Warning: {} not found at expected path {}",
                    selected_terminal, terminal_path
                );
            }
        }
    }

    // Set default terminal (this varies by desktop environment)
    println!("Setting {} as default terminal...", selected_terminal);

    // For Debian-based systems
    let status = Command::new("sudo")
        .args([
            "update-alternatives",
            "--install",
            "/usr/bin/x-terminal-emulator",
            "x-terminal-emulator",
            &terminal_path,
            "50",
        ])
        .status();

    match status {
        Ok(s) if s.success() => println!("Default terminal set to {}", selected_terminal),
        Ok(_) => {
            // update-alternatives may not exist on Arch
            println!("update-alternatives not available (common on Arch Linux)");
            println!("To set default terminal on Arch:");
            println!(
                "  - For GNOME: gsettings set org.gnome.desktop.default-applications.terminal exec '{}'",
                selected_terminal
            );
            println!(
                "  - For KDE: Configure in System Settings > Applications > Default Applications"
            );
            println!("  - For i3/sway: Set $term variable in config");
        }
        Err(e) => eprintln!("Failed to run update-alternatives: {}", e),
    }
}

fn show_terminal_info() {
    println!("📊 Terminal Information");
    println!("======================");

    // Show installed terminals
    println!("📦 Installed terminals:");
    let terminals = [
        "ghostty",
        "wezterm",
        "alacritty",
        "kitty",
        "gnome-terminal",
        "konsole",
    ];

    for terminal in &terminals {
        if Command::new("which").arg(terminal).status().is_ok() {
            if let Ok(output) = Command::new(terminal).arg("--version").output() {
                let version = String::from_utf8_lossy(&output.stdout);
                println!(
                    "  ✅ {} - {}",
                    terminal,
                    version.lines().next().unwrap_or("")
                );
            } else {
                println!("  ✅ {}", terminal);
            }
        }
    }

    // Show current terminal
    if let Ok(term) = std::env::var("TERM") {
        println!("\n🖥️  Current TERM: {}", term);
    }

    if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
        println!("📱 Terminal program: {}", term_program);
    }
}
