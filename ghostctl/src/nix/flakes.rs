use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn flakes_management() {
    println!("🔄 NixOS Flakes Management");
    println!("==========================");

    let options = [
        "🆕 Initialize flake",
        "🔄 Update flake inputs",
        "📋 Show flake info",
        "🔒 Lock flake",
        "🧹 Clean flake cache",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Flakes Management")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => initialize_flake(),
        1 => update_flake_inputs(),
        2 => show_flake_info(),
        3 => lock_flake(),
        4 => clean_flake_cache(),
        _ => return,
    }
}

fn initialize_flake() {
    println!("🆕 Initialize Flake");
    println!("==================");

    let Ok(path) = Input::<String>::new()
        .with_prompt("Flake directory")
        .default(".".into())
        .interact_text()
    else {
        return;
    };

    let Ok(confirm) = Confirm::new()
        .with_prompt(format!("Initialize flake in {}?", path))
        .default(true)
        .interact()
    else {
        return;
    };

    if confirm {
        let _ = Command::new("nix")
            .args(&["flake", "init"])
            .current_dir(&path)
            .status();
    }
}

fn update_flake_inputs() {
    println!("🔄 Update Flake Inputs");
    println!("======================");

    let _ = Command::new("nix").args(&["flake", "update"]).status();
}

fn show_flake_info() {
    println!("📋 Flake Information");
    println!("====================");

    let _ = Command::new("nix").args(&["flake", "show"]).status();
}

fn lock_flake() {
    println!("🔒 Lock Flake");
    println!("=============");

    let _ = Command::new("nix").args(&["flake", "lock"]).status();
}

fn clean_flake_cache() {
    println!("🧹 Clean Flake Cache");
    println!("====================");

    let Ok(confirm) = Confirm::new()
        .with_prompt("Clean flake registry and cache?")
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        let _ = Command::new("nix")
            .args(&["registry", "remove", "--all"])
            .status();
    }
}
