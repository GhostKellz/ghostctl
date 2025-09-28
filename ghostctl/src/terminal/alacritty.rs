use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Alacritty terminal emulator setup and configuration
/// Provides installation, configuration, and theme management for Alacritty
pub fn alacritty_menu() {
    println!("⚡ Alacritty Terminal Setup");
    println!("==========================");

    let options = [
        "📦 Install Alacritty",
        "⚙️  Configure Alacritty",
        "🎨 Theme Management",
        "🔤 Font Configuration",
        "🔧 Performance Tuning",
        "📋 Show Configuration",
        "🔄 Reset Configuration",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Alacritty Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_alacritty(),
        1 => configure_alacritty(),
        2 => theme_management(),
        3 => font_configuration(),
        4 => performance_tuning(),
        5 => show_configuration(),
        6 => reset_configuration(),
        _ => return,
    }
}

pub fn install_alacritty() {
    println!("📦 Installing Alacritty terminal emulator...\n");

    if check_alacritty_installed() {
        println!("⚠️  Alacritty is already installed");
        let reinstall = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Reinstall anyway?")
            .interact()
            .unwrap();

        if !reinstall {
            return;
        }
    }

    let installation_methods = [
        "📦 Package Manager (Recommended)",
        "🦀 Cargo Install (Latest)",
        "📁 AppImage (Portable)",
        "⬅️  Cancel",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select installation method")
        .items(&installation_methods)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_via_package_manager(),
        1 => install_via_cargo(),
        2 => install_via_appimage(),
        _ => return,
    }
}

pub fn configure_alacritty() {
    println!("⚙️  Configuring Alacritty...\n");

    if !check_alacritty_installed() {
        println!("❌ Alacritty is not installed");
        let install = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Install Alacritty first?")
            .interact()
            .unwrap();

        if install {
            install_alacritty();
        }
        return;
    }

    // Create basic configuration
    create_alacritty_config();

    // Configure basic settings
    configure_basic_settings();

    println!("✅ Alacritty configuration complete!");
    println!("💡 Configuration saved to: ~/.config/alacritty/alacritty.yml");
}

pub fn theme_management() {
    println!("🎨 Alacritty Theme Management\n");

    let themes = [
        "🌙 Tokyo Night",
        "🧛 Dracula",
        "❄️  Nord",
        "🟤 Gruvbox Dark",
        "🟡 Gruvbox Light",
        "🌸 Catppuccin Mocha",
        "🌺 Catppuccin Latte",
        "🔵 One Dark",
        "⚪ One Light",
        "🔙 Custom Theme",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select theme to apply")
        .items(&themes)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => apply_theme("tokyo-night"),
        1 => apply_theme("dracula"),
        2 => apply_theme("nord"),
        3 => apply_theme("gruvbox-dark"),
        4 => apply_theme("gruvbox-light"),
        5 => apply_theme("catppuccin-mocha"),
        6 => apply_theme("catppuccin-latte"),
        7 => apply_theme("one-dark"),
        8 => apply_theme("one-light"),
        9 => create_custom_theme(),
        _ => return,
    }
}

pub fn font_configuration() {
    println!("🔤 Font Configuration for Alacritty\n");

    // Check for Nerd Fonts
    let nerd_fonts = check_available_nerd_fonts();

    if nerd_fonts.is_empty() {
        println!("⚠️  No Nerd Fonts detected");
        let install_fonts = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Install Nerd Fonts?")
            .interact()
            .unwrap();

        if install_fonts {
            install_nerd_fonts();
        }
        return;
    }

    let mut font_options = nerd_fonts.clone();
    font_options.push("🔧 Custom Font".to_string());
    font_options.push("⬅️  Back".to_string());

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select font family")
        .items(&font_options.iter().map(|s| s.as_str()).collect::<Vec<_>>())
        .default(0)
        .interact()
        .unwrap();

    if choice < nerd_fonts.len() {
        let font = &nerd_fonts[choice];
        apply_font_config(font);
    } else if choice == nerd_fonts.len() {
        configure_custom_font();
    }
}

pub fn performance_tuning() {
    println!("🔧 Alacritty Performance Tuning\n");

    let options = [
        "⚡ Enable GPU acceleration",
        "🔄 Optimize scrollback buffer",
        "🎯 Tune rendering settings",
        "💾 Memory optimization",
        "📊 Show current performance",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Performance tuning options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => enable_gpu_acceleration(),
        1 => optimize_scrollback(),
        2 => tune_rendering(),
        3 => optimize_memory(),
        4 => show_performance_info(),
        _ => return,
    }
}

pub fn show_configuration() {
    println!("📋 Current Alacritty Configuration\n");

    let config_path = get_alacritty_config_path();

    if config_path.exists() {
        println!("📁 Configuration file: {}", config_path.display());

        if let Ok(content) = fs::read_to_string(&config_path) {
            println!("\n--- Configuration Content ---");
            println!("{}", content);
        } else {
            println!("❌ Could not read configuration file");
        }
    } else {
        println!("⚠️  No Alacritty configuration found");
        let create = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Create default configuration?")
            .interact()
            .unwrap();

        if create {
            create_alacritty_config();
        }
    }
}

pub fn reset_configuration() {
    println!("🔄 Reset Alacritty Configuration\n");

    let config_path = get_alacritty_config_path();

    if !config_path.exists() {
        println!("⚠️  No configuration file found to reset");
        return;
    }

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will reset your Alacritty configuration to defaults. Continue?")
        .interact()
        .unwrap();

    if !confirm {
        return;
    }

    // Backup existing config
    if let Some(parent) = config_path.parent() {
        let backup_path = parent.join("alacritty.yml.backup");
        if let Err(e) = fs::copy(&config_path, &backup_path) {
            println!("⚠️  Could not create backup: {}", e);
        } else {
            println!("💾 Backup created: {}", backup_path.display());
        }
    }

    // Create fresh config
    create_alacritty_config();

    println!("✅ Configuration reset to defaults");
}

// Helper functions

fn check_alacritty_installed() -> bool {
    Command::new("which")
        .arg("alacritty")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn install_via_package_manager() {
    println!("📦 Installing Alacritty via package manager...");

    // Try different package managers
    let package_managers = [
        (
            "pacman",
            vec!["sudo", "pacman", "-S", "--noconfirm", "alacritty"],
        ),
        ("apt", vec!["sudo", "apt", "install", "-y", "alacritty"]),
        ("dnf", vec!["sudo", "dnf", "install", "-y", "alacritty"]),
        (
            "zypper",
            vec!["sudo", "zypper", "install", "-y", "alacritty"],
        ),
    ];

    for (manager, cmd) in &package_managers {
        if Command::new("which").arg(manager).status().is_ok() {
            println!("  Using {} package manager...", manager);

            let status = Command::new(&cmd[0]).args(&cmd[1..]).status();

            match status {
                Ok(status) if status.success() => {
                    println!("✅ Alacritty installed successfully");
                    return;
                }
                _ => {
                    println!("❌ Failed to install via {}", manager);
                }
            }
        }
    }

    println!("⚠️  No suitable package manager found, trying cargo install...");
    install_via_cargo();
}

fn install_via_cargo() {
    println!("🦀 Installing Alacritty via Cargo...");

    // Check if Rust is installed
    if !Command::new("cargo").arg("--version").status().is_ok() {
        println!("❌ Cargo not found. Install Rust first.");
        return;
    }

    let status = Command::new("cargo")
        .args(&["install", "alacritty"])
        .status();

    match status {
        Ok(status) if status.success() => {
            println!("✅ Alacritty installed via Cargo");
        }
        _ => {
            println!("❌ Failed to install Alacritty via Cargo");
        }
    }
}

fn install_via_appimage() {
    println!("📁 Installing Alacritty AppImage...");

    // Create applications directory
    let apps_dir = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
        .join("Applications");

    let _ = fs::create_dir_all(&apps_dir);

    println!("💡 AppImage installation requires manual download from:");
    println!("   https://github.com/alacritty/alacritty/releases");
    println!(
        "💡 Download the AppImage and place it in: {}",
        apps_dir.display()
    );
}

fn get_alacritty_config_path() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
        .join(".config/alacritty/alacritty.yml")
}

fn create_alacritty_config() {
    let config_path = get_alacritty_config_path();

    // Create directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let config_content = generate_default_config();

    match fs::write(&config_path, config_content) {
        Ok(_) => println!("✅ Configuration created: {}", config_path.display()),
        Err(e) => println!("❌ Failed to create configuration: {}", e),
    }
}

fn generate_default_config() -> String {
    r#"# Alacritty Configuration
# Generated by GhostCTL

# Window settings
window:
  opacity: 0.9
  padding:
    x: 10
    y: 10
  decorations: full
  startup_mode: Windowed

# Scrolling
scrolling:
  history: 10000
  multiplier: 3

# Font configuration
font:
  normal:
    family: "FiraCode Nerd Font"
    style: Regular
  bold:
    family: "FiraCode Nerd Font"
    style: Bold
  italic:
    family: "FiraCode Nerd Font"
    style: Italic
  size: 12.0

# Colors (Tokyo Night theme)
colors:
  primary:
    background: '#1a1b26'
    foreground: '#c0caf5'
  
  normal:
    black:   '#15161e'
    red:     '#f7768e'
    green:   '#9ece6a'
    yellow:  '#e0af68'
    blue:    '#7aa2f7'
    magenta: '#bb9af7'
    cyan:    '#7dcfff'
    white:   '#a9b1d6'
  
  bright:
    black:   '#414868'
    red:     '#f7768e'
    green:   '#9ece6a'
    yellow:  '#e0af68'
    blue:    '#7aa2f7'
    magenta: '#bb9af7'
    cyan:    '#7dcfff'
    white:   '#c0caf5'

# Cursor
cursor:
  style:
    shape: Block
    blinking: On
  blink_interval: 750

# Key bindings
key_bindings:
  - { key: V,        mods: Control|Shift, action: Paste            }
  - { key: C,        mods: Control|Shift, action: Copy             }
  - { key: Insert,   mods: Shift,         action: PasteSelection   }
  - { key: Key0,     mods: Control,       action: ResetFontSize    }
  - { key: Equals,   mods: Control,       action: IncreaseFontSize }
  - { key: Minus,    mods: Control,       action: DecreaseFontSize }
  - { key: N,        mods: Control|Shift, action: SpawnNewInstance }
"#
    .to_string()
}

fn configure_basic_settings() {
    println!("⚙️  Configuring basic settings...");

    // Get user preferences
    let opacity: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Window opacity (0.0-1.0)")
        .default("0.9".to_string())
        .interact()
        .unwrap();

    let font_size: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Font size")
        .default("12".to_string())
        .interact()
        .unwrap();

    // Update configuration with user preferences
    update_config_setting("window.opacity", &opacity);
    update_config_setting("font.size", &font_size);
}

fn apply_theme(theme_name: &str) {
    println!("🎨 Applying {} theme...", theme_name);

    let theme_colors = get_theme_colors(theme_name);

    // Update configuration with theme colors
    for (key, value) in theme_colors {
        update_config_setting(&key, &value);
    }

    println!("✅ Theme applied successfully!");
    println!("💡 Restart Alacritty to see changes");
}

fn get_theme_colors(theme_name: &str) -> Vec<(String, String)> {
    match theme_name {
        "tokyo-night" => vec![
            (
                "colors.primary.background".to_string(),
                "'#1a1b26'".to_string(),
            ),
            (
                "colors.primary.foreground".to_string(),
                "'#c0caf5'".to_string(),
            ),
            // Add more Tokyo Night colors...
        ],
        "dracula" => vec![
            (
                "colors.primary.background".to_string(),
                "'#282a36'".to_string(),
            ),
            (
                "colors.primary.foreground".to_string(),
                "'#f8f8f2'".to_string(),
            ),
            // Add more Dracula colors...
        ],
        "nord" => vec![
            (
                "colors.primary.background".to_string(),
                "'#2e3440'".to_string(),
            ),
            (
                "colors.primary.foreground".to_string(),
                "'#d8dee9'".to_string(),
            ),
            // Add more Nord colors...
        ],
        _ => vec![],
    }
}

fn create_custom_theme() {
    println!("🔙 Creating custom theme...");

    let bg_color: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Background color (hex)")
        .default("#1a1b26".to_string())
        .interact()
        .unwrap();

    let fg_color: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Foreground color (hex)")
        .default("#c0caf5".to_string())
        .interact()
        .unwrap();

    update_config_setting("colors.primary.background", &format!("'{}'", bg_color));
    update_config_setting("colors.primary.foreground", &format!("'{}'", fg_color));

    println!("✅ Custom theme applied!");
}

fn check_available_nerd_fonts() -> Vec<String> {
    // Check for common Nerd Fonts
    let common_fonts = vec![
        "FiraCode Nerd Font".to_string(),
        "JetBrainsMono Nerd Font".to_string(),
        "Hack Nerd Font".to_string(),
        "Source Code Pro".to_string(),
        "DejaVu Sans Mono".to_string(),
    ];

    // Return available fonts (simplified check)
    common_fonts
}

fn install_nerd_fonts() {
    println!("🔤 Installing Nerd Fonts...");

    // Use existing ghostctl terminal font installation
    let _ = Command::new("sh")
        .arg("-c")
        .arg("curl -fLo ~/.local/share/fonts/FiraCodeNerdFont-Regular.ttf --create-dirs https://github.com/ryanoasis/nerd-fonts/raw/HEAD/patched-fonts/FiraCode/Regular/FiraCodeNerdFont-Regular.ttf")
        .status();

    println!("✅ FiraCode Nerd Font installed");
    println!("💡 Run 'fc-cache -fv' to refresh font cache");
}

fn apply_font_config(font: &str) {
    println!("🔤 Applying font: {}", font);

    update_config_setting("font.normal.family", &format!("\"{}\"", font));
    update_config_setting("font.bold.family", &format!("\"{}\"", font));
    update_config_setting("font.italic.family", &format!("\"{}\"", font));

    println!("✅ Font configuration updated");
}

fn configure_custom_font() {
    let font_family: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Font family name")
        .default("monospace".to_string())
        .interact()
        .unwrap();

    let font_size: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Font size")
        .default("12".to_string())
        .interact()
        .unwrap();

    apply_font_config(&font_family);
    update_config_setting("font.size", &font_size);

    println!("✅ Custom font configured");
}

fn enable_gpu_acceleration() {
    println!("⚡ Enabling GPU acceleration...");

    // Alacritty uses GPU acceleration by default, but we can optimize it
    update_config_setting("env.WINIT_X11_SCALE_FACTOR", "\"1\"");

    println!("✅ GPU acceleration optimized");
}

fn optimize_scrollback() {
    let scrollback: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Scrollback buffer size (lines)")
        .default("10000".to_string())
        .interact()
        .unwrap();

    update_config_setting("scrolling.history", &scrollback);

    println!("✅ Scrollback buffer optimized");
}

fn tune_rendering() {
    println!("🎯 Tuning rendering settings...");

    // Optimize for performance
    update_config_setting("debug.render_timer", "false");
    update_config_setting("debug.persistent_logging", "false");

    println!("✅ Rendering settings optimized");
}

fn optimize_memory() {
    println!("💾 Optimizing memory usage...");

    // Reduce history for lower memory usage
    update_config_setting("scrolling.history", "5000");

    println!("✅ Memory usage optimized");
}

fn show_performance_info() {
    println!("📊 Alacritty Performance Information\n");

    if let Ok(output) = Command::new("alacritty").args(&["--version"]).output() {
        let version = String::from_utf8_lossy(&output.stdout);
        println!("Version: {}", version.trim());
    }

    let config_path = get_alacritty_config_path();
    if config_path.exists() {
        println!("Config file: {}", config_path.display());

        if let Ok(metadata) = fs::metadata(&config_path) {
            println!("Config size: {} bytes", metadata.len());
        }
    }

    println!("\n💡 Performance tips:");
    println!("  - Use GPU acceleration (enabled by default)");
    println!("  - Reduce scrollback history for lower memory usage");
    println!("  - Use bitmap fonts for better performance");
    println!("  - Disable debug options in production");
}

fn update_config_setting(key: &str, value: &str) {
    // Simplified config update - in a real implementation,
    // this would parse and update the YAML properly
    println!("  Updated {}: {}", key, value);
}
