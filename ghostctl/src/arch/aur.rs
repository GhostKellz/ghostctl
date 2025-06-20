use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use std::process::Command;

#[allow(dead_code)]
pub fn aur_helper_management() {
    println!("📦 AUR Helper Management");
    println!("========================");

    let options = [
        "🔍 Check installed AUR helpers",
        "📥 Install AUR helper",
        "🔄 Update AUR packages",
        "🧹 Clean AUR cache",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("AUR Helper Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => check_aur_helpers(),
        1 => install_aur_helper(),
        2 => update_aur_packages(),
        3 => clean_aur_cache(),
        _ => (),
    }
}

#[allow(dead_code)]
fn check_aur_helpers() {
    println!("🔍 Checking AUR Helpers");
    println!("=======================");

    let helpers = [
        ("reaper", "reap", "GhostKellz's modern AUR helper"),
        ("paru", "paru", "Feature packed AUR helper"),
        ("yay", "yay", "Yet another Yogurt AUR helper"),
        ("trizen", "trizen", "Lightweight AUR helper"),
        ("pikaur", "pikaur", "AUR helper with minimal dependencies"),
    ];

    let mut found_helpers = Vec::new();

    for (name, cmd, description) in &helpers {
        if Command::new("which").arg(cmd).status().is_ok() {
            println!("  ✅ {} - {}", name, description);
            found_helpers.push(*name);
        } else {
            println!("  ❌ {} - {} (not installed)", name, description);
        }
    }

    if found_helpers.is_empty() {
        println!("\n💡 No AUR helpers found. Consider installing one!");
    } else {
        println!("\n📊 Found {} AUR helper(s)", found_helpers.len());
    }
}

#[allow(dead_code)]
fn install_aur_helper() {
    println!("📥 Install AUR Helper");
    println!("====================");

    let helpers = [
        "reaper (Recommended - GhostKellz)",
        "paru (Feature rich)",
        "yay (Popular choice)",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select AUR helper to install")
        .items(&helpers)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_reaper(),
        1 => install_paru(),
        2 => install_yay(),
        _ => (),
    }
}

#[allow(dead_code)]
fn install_reaper() {
    println!("🔥 Installing Reaper AUR Helper");
    println!("===============================");

    let confirm = Confirm::new()
        .with_prompt("Install Reaper via official installer?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("📥 Downloading and installing Reaper...");
        let status = Command::new("bash")
            .arg("-c")
            .arg("curl -sSL https://raw.githubusercontent.com/face-hh/reaper/main/release/install.sh | bash")
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Reaper installed successfully!");
                println!("💡 Use 'reap -S package' to install AUR packages");
            }
            _ => println!("❌ Failed to install Reaper"),
        }
    }
}

#[allow(dead_code)]
fn install_paru() {
    println!("🦀 Installing Paru AUR Helper");
    println!("=============================");

    // Check if rust is installed
    if Command::new("which").arg("cargo").status().is_err() {
        println!("📦 Installing Rust toolchain...");
        let _ = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "rust"])
            .status();
    }

    let confirm = Confirm::new()
        .with_prompt("Build and install Paru from AUR?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("🔨 Building Paru from source...");
        let build_dir = "/tmp/paru-build";

        let _ = std::fs::remove_dir_all(build_dir);

        let status = Command::new("git")
            .args(["clone", "https://aur.archlinux.org/paru.git", build_dir])
            .status();

        if status.is_ok() && status.unwrap().success() {
            let build_status = Command::new("makepkg")
                .args(["-si", "--noconfirm"])
                .current_dir(build_dir)
                .status();

            match build_status {
                Ok(s) if s.success() => {
                    println!("✅ Paru installed successfully!");
                    println!("💡 Use 'paru -S package' to install AUR packages");
                }
                _ => println!("❌ Failed to build Paru"),
            }
        }

        let _ = std::fs::remove_dir_all(build_dir);
    }
}

#[allow(dead_code)]
fn install_yay() {
    println!("🚀 Installing Yay AUR Helper");
    println!("============================");

    let confirm = Confirm::new()
        .with_prompt("Build and install Yay from AUR?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("🔨 Building Yay from source...");
        let build_dir = "/tmp/yay-build";

        let _ = std::fs::remove_dir_all(build_dir);

        let status = Command::new("git")
            .args(["clone", "https://aur.archlinux.org/yay.git", build_dir])
            .status();

        if status.is_ok() && status.unwrap().success() {
            let build_status = Command::new("makepkg")
                .args(["-si", "--noconfirm"])
                .current_dir(build_dir)
                .status();

            match build_status {
                Ok(s) if s.success() => {
                    println!("✅ Yay installed successfully!");
                    println!("💡 Use 'yay -S package' to install AUR packages");
                }
                _ => println!("❌ Failed to build Yay"),
            }
        }

        let _ = std::fs::remove_dir_all(build_dir);
    }
}

#[allow(dead_code)]
fn update_aur_packages() {
    println!("🔄 Update AUR Packages");
    println!("======================");

    // Try available AUR helpers
    let helpers = [("reap", "-Syu"), ("paru", "-Syu"), ("yay", "-Syu")];

    for (helper, args) in &helpers {
        if Command::new("which").arg(helper).status().is_ok() {
            println!("🔄 Updating with {}...", helper);
            let _ = Command::new(helper).arg(args).status();
            return;
        }
    }

    println!("❌ No AUR helper found for updates");
}

#[allow(dead_code)]
fn clean_aur_cache() {
    println!("🧹 Clean AUR Cache");
    println!("==================");

    // Try available AUR helpers
    let helpers = [("reap", "-Sc"), ("paru", "-Sc"), ("yay", "-Sc")];

    for (helper, args) in &helpers {
        if Command::new("which").arg(helper).status().is_ok() {
            let confirm = Confirm::new()
                .with_prompt(format!("Clean cache with {}?", helper))
                .default(true)
                .interact()
                .unwrap();

            if confirm {
                let _ = Command::new(helper).arg(args).status();
            }
            return;
        }
    }

    println!("❌ No AUR helper found for cache cleaning");
}

#[allow(dead_code)]
pub fn get_preferred_aur_helper() -> Option<String> {
    let helpers = ["reap", "paru", "yay", "trizen", "pikaur"];

    for helper in &helpers {
        if Command::new("which").arg(helper).status().is_ok() {
            return Some(helper.to_string());
        }
    }

    None
}
