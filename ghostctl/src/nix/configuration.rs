use dialoguer::{theme::ColorfulTheme, Select};
use std::fs;
use std::process::Command;

pub fn edit_configuration() {
    println!("⚙️  NixOS Configuration Editor");
    println!("=============================");

    let config_options = [
        "📝 Edit configuration.nix",
        "📋 Show current configuration",
        "🔄 Rebuild and switch",
        "🧪 Test configuration",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Configuration management")
        .items(&config_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => edit_config_file(),
        1 => show_current_config(),
        2 => rebuild_and_switch(),
        3 => test_configuration(),
        _ => return,
    }
}

fn edit_config_file() {
    let config_path = "/etc/nixos/configuration.nix";

    if !std::path::Path::new(config_path).exists() {
        println!("❌ Configuration file not found: {}", config_path);
        return;
    }

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    println!("📝 Opening {} with {}", config_path, editor);

    let _ = Command::new("sudo").args(&[&editor, config_path]).status();
}

fn show_current_config() {
    println!("📋 Current NixOS Configuration");
    println!("==============================");

    let config_path = "/etc/nixos/configuration.nix";

    if let Ok(content) = fs::read_to_string(config_path) {
        println!("{}", content);
    } else {
        println!("❌ Could not read configuration file");
    }
}

fn rebuild_and_switch() {
    println!("🔄 Rebuilding and switching configuration...");

    let _ = Command::new("sudo")
        .args(&["nixos-rebuild", "switch"])
        .status();
}

fn test_configuration() {
    println!("🧪 Testing configuration...");

    let _ = Command::new("sudo")
        .args(&["nixos-rebuild", "test"])
        .status();
}
