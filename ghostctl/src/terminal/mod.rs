use dialoguer::{theme::ColorfulTheme, Confirm, Select};
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Terminal Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

        let method = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Installation method")
            .items(
                &install_methods
                    .iter()
                    .map(|(name, _)| *name)
                    .collect::<Vec<_>>(),
            )
            .default(0)
            .interact()
            .unwrap();

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
    println!("📦 Installing Ghostty from AUR...");

    // Check if yay is available
    if Command::new("which").arg("yay").status().is_ok() {
        let status = Command::new("yay")
            .args(&["-S", "--noconfirm", "ghostty"])
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Ghostty installed successfully via AUR"),
            _ => {
                println!("❌ Failed to install via yay, trying paru...");
                let _ = Command::new("paru")
                    .args(&["-S", "--noconfirm", "ghostty"])
                    .status();
            }
        }
    } else {
        println!("❌ AUR helper not found. Please install yay or paru first");
    }
}

fn install_ghostty_source() {
    println!("🔨 Building Ghostty from source...");
    println!("📋 Prerequisites: Zig compiler");

    // Check if zig is installed
    if !Command::new("which").arg("zig").status().is_ok() {
        println!("❌ Zig compiler not found");
        let install_zig = Confirm::new()
            .with_prompt("Install Zig compiler?")
            .default(true)
            .interact()
            .unwrap();

        if install_zig {
            println!("📦 Installing Zig...");
            let _ = Command::new("sudo")
                .args(&["pacman", "-S", "--noconfirm", "zig"])
                .status();
        } else {
            return;
        }
    }

    let build_dir = dirs::home_dir().unwrap().join("src/ghostty");

    println!("📥 Cloning Ghostty repository...");
    let _ = Command::new("git")
        .args(&[
            "clone",
            "https://github.com/mitchellh/ghostty",
            build_dir.to_str().unwrap(),
        ])
        .status();

    println!("🔨 Building Ghostty (this may take a while)...");
    let status = Command::new("zig")
        .args(&["build", "-Doptimize=ReleaseFast"])
        .current_dir(&build_dir)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Ghostty built successfully");

            // Install binary
            let install = Confirm::new()
                .with_prompt("Install to /usr/local/bin?")
                .default(true)
                .interact()
                .unwrap();

            if install {
                let binary_path = build_dir.join("zig-out/bin/ghostty");
                let _ = Command::new("sudo")
                    .args(&[
                        "install",
                        "-Dm755",
                        binary_path.to_str().unwrap(),
                        "/usr/local/bin/ghostty",
                    ])
                    .status();
                println!("✅ Ghostty installed to /usr/local/bin/ghostty");
            }
        }
        _ => println!("❌ Build failed"),
    }
}

fn install_ghostty_appimage() {
    println!("📦 Installing Ghostty AppImage...");
    println!("⚠️  AppImage not yet available for Ghostty");
    println!("💡 Use source build or wait for official releases");
}

fn configure_ghostty() {
    println!("⚙️  Configuring Ghostty...");

    let config_dir = dirs::home_dir().unwrap().join(".config/ghostty");
    fs::create_dir_all(&config_dir).unwrap();

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
    fs::write(&config_file, config_content).unwrap();

    println!("✅ Ghostty configuration saved to: {:?}", config_file);
}

pub fn setup_wezterm() {
    println!("🔷 Setting up WezTerm ");
    println!("=======================================");

    let is_installed = Command::new("which")
        .arg("wezterm")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if is_installed {
        println!("✅ WezTerm is available ");
    } else {
        println!("📦 Installing WezTerm...");
        let status = Command::new("sh")
            .arg("-c")
            .arg("sudo pacman -S --noconfirm wezterm || yay -S wezterm-git ")
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ WezTerm setup complete "),
            _ => {
                println!("❌ Failed to install WezTerm via package manager ");
                println!("💡 Try: https://wezfurlong.org/wezterm/installation.html");
                return;
            }
        }
    }

    // Configure WezTerm
    configure_wezterm();
}

fn configure_wezterm() {
    println!("⚙️  Configuring WezTerm...");

    let config_dir = dirs::home_dir().unwrap().join(".config/wezterm");
    fs::create_dir_all(&config_dir).unwrap();

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

        fs::write(&config_file, config_content).unwrap();
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Preferences")
        .items(&preferences)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_nerd_fonts(),
        1 => setup_terminal_themes(),
        2 => configure_shell_integration(),
        3 => set_default_terminal(),
        _ => return,
    }
}

fn install_nerd_fonts() {
    println!("🎨 Installing Nerd Fonts...");

    let fonts = [
        "FiraCode Nerd Font",
        "JetBrains Mono Nerd Font",
        "Hack Nerd Font",
        "Source Code Pro Nerd Font",
        "Ubuntu Mono Nerd Font",
    ];

    let selected = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select font to install")
        .items(&fonts)
        .default(0)
        .interact()
        .unwrap();

    let font_package = match selected {
        0 => "ttf-firacode-nerd",
        1 => "ttf-jetbrains-mono-nerd",
        2 => "ttf-hack-nerd",
        3 => "ttf-sourcecodepro-nerd",
        4 => "ttf-ubuntu-mono-nerd",
        _ => return,
    };

    println!("📦 Installing {}...", fonts[selected]);
    let _ = Command::new("sudo")
        .args(&["pacman", "-S", "--noconfirm", font_package])
        .status();
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
    println!("🔧 Set Default Terminal");
    println!("======================");

    let terminals = ["ghostty", "wezterm", "alacritty", "kitty", "gnome-terminal"];
    let mut available_terminals = Vec::new();

    for terminal in &terminals {
        if Command::new("which").arg(terminal).status().is_ok() {
            available_terminals.push(*terminal);
        }
    }

    if available_terminals.is_empty() {
        println!("❌ No supported terminals found");
        return;
    }

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select default terminal")
        .items(&available_terminals)
        .default(0)
        .interact()
        .unwrap();

    let selected_terminal = available_terminals[choice];

    // Set default terminal (this varies by desktop environment)
    println!("🔧 Setting {} as default terminal...", selected_terminal);

    // For most environments
    let _ = Command::new("sudo")
        .args(&[
            "update-alternatives",
            "--install",
            "/usr/bin/x-terminal-emulator",
            "x-terminal-emulator",
            &format!("/usr/bin/{}", selected_terminal),
            "50",
        ])
        .status();

    println!("✅ Default terminal set to {}", selected_terminal);
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
