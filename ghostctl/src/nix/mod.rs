pub mod configuration;
pub mod flakes;
pub mod packages;

use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::process::Command;

// Define NixosAction enum
#[allow(dead_code)]
#[derive(Debug)]
pub enum NixosAction {
    Rebuild,
    Update,
    Rollback,
    GarbageCollect,
    Generations,
}

pub fn nixos_menu() {
    println!("❄️  NixOS Management");
    println!("==================");

    let options = [
        "📦 Package Management",
        "⚙️  System Configuration",
        "🔄 Flakes Management",
        "🔧 System Rebuild",
        "📋 System Status",
        "🗑️  Garbage Collection",
        "📊 Generation Management",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("NixOS Management")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => packages::package_management(),
        1 => configuration::edit_configuration(),
        2 => flakes::flakes_management(),
        3 => rebuild_system(),
        4 => system_status(),
        5 => garbage_collection(),
        6 => generation_management(),
        _ => return,
    }
}

fn rebuild_system() {
    println!("🔧 NixOS System Rebuild");
    println!("=======================");

    let rebuild_options = [
        "🔄 Switch (rebuild and switch)",
        "🧪 Test (rebuild and test)",
        "🚀 Boot (rebuild for next boot)",
        "🔍 Dry-run (check what would change)",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Rebuild type")
        .items(&rebuild_options)
        .default(0)
        .interact()
    else {
        return;
    };

    let rebuild_cmd = match choice {
        0 => "switch",
        1 => "test",
        2 => "boot",
        3 => "dry-activate",
        _ => return,
    };

    let Ok(confirm) = Confirm::new()
        .with_prompt(format!("Rebuild system with '{}'?", rebuild_cmd))
        .default(true)
        .interact()
    else {
        return;
    };

    if confirm {
        println!("🚀 Running: sudo nixos-rebuild {}", rebuild_cmd);
        let _ = Command::new("sudo")
            .args(&["nixos-rebuild", rebuild_cmd])
            .status();
    }
}

fn system_status() {
    println!("📋 NixOS System Status");
    println!("======================");

    // Show current generation
    println!("🔢 Current Generation:");
    let _ = Command::new("nixos-version").status();

    // Show system configuration
    println!("\n⚙️  System Configuration:");
    let _ = Command::new("nix-env")
        .args(&[
            "--list-generations",
            "--profile",
            "/nix/var/nix/profiles/system",
        ])
        .status();

    // Show installed packages
    println!("\n📦 User Packages:");
    let _ = Command::new("nix-env").args(&["-q"]).status();

    // Show disk usage
    println!("\n💾 Nix Store Usage:");
    let _ = Command::new("nix")
        .args(&["path-info", "-S", "/nix/store"])
        .status();
}

fn garbage_collection() {
    println!("🗑️  Nix Garbage Collection");
    println!("==========================");

    let gc_options = [
        "🧹 Delete unreachable paths",
        "🗓️  Delete older than 30 days",
        "🗓️  Delete older than 7 days",
        "💾 Show what would be deleted",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Garbage collection type")
        .items(&gc_options)
        .default(0)
        .interact()
    else {
        return;
    };

    let gc_cmd = match choice {
        0 => vec!["nix-collect-garbage", "-d"],
        1 => vec!["nix-collect-garbage", "--delete-older-than", "30d"],
        2 => vec!["nix-collect-garbage", "--delete-older-than", "7d"],
        3 => vec!["nix-collect-garbage", "--dry-run"],
        _ => return,
    };

    if choice != 3 {
        let Ok(confirm) = Confirm::new()
            .with_prompt("Run garbage collection?")
            .default(false)
            .interact()
        else {
            return;
        };

        if !confirm {
            return;
        }
    }

    println!("🚀 Running garbage collection...");
    let _ = Command::new("sudo").args(&gc_cmd).status();
}

fn generation_management() {
    println!("📊 NixOS Generation Management");
    println!("==============================");

    // List generations
    println!("📋 Available generations:");
    let _ = Command::new("nixos-rebuild")
        .args(&["list-generations"])
        .status();

    let gen_options = [
        "🔄 Rollback to previous generation",
        "⏮️  Switch to specific generation",
        "🗑️  Delete old generations",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Generation management")
        .items(&gen_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => rollback_generation(),
        1 => switch_to_generation(),
        2 => delete_old_generations(),
        _ => return,
    }
}

fn rollback_generation() {
    let Ok(confirm) = Confirm::new()
        .with_prompt("Rollback to previous generation?")
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        let _ = Command::new("sudo")
            .args(&["nixos-rebuild", "switch", "--rollback"])
            .status();
    }
}

fn switch_to_generation() {
    let Ok(generation) = Input::<String>::new()
        .with_prompt("Generation number")
        .interact_text()
    else {
        return;
    };

    let Ok(confirm) = Confirm::new()
        .with_prompt(format!("Switch to generation {}?", generation))
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        let _ = Command::new("sudo")
            .args(&[
                "/nix/var/nix/profiles/system-{}-link/bin/switch-to-configuration",
                "switch",
            ])
            .status();
    }
}

fn delete_old_generations() {
    let Ok(keep) = Input::<String>::new()
        .with_prompt("Keep how many generations")
        .default("5".into())
        .interact_text()
    else {
        return;
    };

    let Ok(confirm) = Confirm::new()
        .with_prompt(format!("Delete generations older than {}?", keep))
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        let _ = Command::new("sudo")
            .args(&[
                "nix-env",
                "--delete-generations",
                &format!("+{}", keep),
                "--profile",
                "/nix/var/nix/profiles/system",
            ])
            .status();
    }
}

// Handle CLI commands
#[allow(dead_code)]
pub fn handle_nixos_action(action: crate::NixosAction) {
    match action {
        crate::NixosAction::Rebuild => rebuild_system(),
        crate::NixosAction::Update => {
            println!("🔄 Updating NixOS channels...");
            let _ = Command::new("sudo")
                .args(&["nix-channel", "--update"])
                .status();
            rebuild_system();
        }
        crate::NixosAction::Rollback => rollback_generation(),
        crate::NixosAction::GarbageCollect => garbage_collection(),
        crate::NixosAction::Generations => generation_management(),
    }
}
