use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn update_mirror_list() {
    println!("🌐 Updating Arch Mirror List");
    println!("============================");

    let options = [
        "🔄 Auto-update with reflector (recommended)",
        "🌍 Select country for mirrors",
        "⚡ Rank mirrors by speed",
        "📋 View current mirror list",
        "🔧 Manual edit mirrorlist",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Mirror Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => auto_update_mirrors(),
        1 => update_mirrors_by_country(),
        2 => rank_mirrors_by_speed(),
        3 => view_current_mirrors(),
        4 => manual_edit_mirrors(),
        _ => return,
    }
}

fn auto_update_mirrors() {
    println!("🔄 Auto-updating mirrors with reflector...");

    // Check if reflector is installed
    if Command::new("which").arg("reflector").output().is_ok() {
        println!("📡 Fetching fastest mirrors...");
        let status = Command::new("sudo")
            .args([
                "reflector",
                "--latest",
                "20",
                "--protocol",
                "https",
                "--sort",
                "rate",
                "--save",
                "/etc/pacman.d/mirrorlist",
            ])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Mirror list updated successfully!");
                println!("📋 New mirrors have been saved to /etc/pacman.d/mirrorlist");
            }
            _ => println!("❌ Failed to update mirrors"),
        }
    } else {
        println!("⚠️  Reflector is not installed.");
        println!("Would you like to install it? (y/n)");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() == "y" {
            let _ = Command::new("sudo")
                .args(["pacman", "-S", "--noconfirm", "reflector"])
                .status();
            auto_update_mirrors(); // Retry after installation
        }
    }
}

fn update_mirrors_by_country() {
    let country = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter country code (e.g., US, GB, DE)")
        .default("US".to_string())
        .interact()
        .unwrap();

    println!("🌍 Updating mirrors for country: {}", country);

    let status = Command::new("sudo")
        .args([
            "reflector",
            "--country",
            &country,
            "--latest",
            "10",
            "--protocol",
            "https",
            "--sort",
            "rate",
            "--save",
            "/etc/pacman.d/mirrorlist",
        ])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Mirrors updated for {}", country),
        _ => println!("❌ Failed to update mirrors for {}", country),
    }
}

fn rank_mirrors_by_speed() {
    println!("⚡ Ranking mirrors by download speed...");
    println!("This may take a few minutes...");

    // Backup current mirrorlist
    let _ = Command::new("sudo")
        .args(&[
            "cp",
            "/etc/pacman.d/mirrorlist",
            "/etc/pacman.d/mirrorlist.backup",
        ])
        .status();

    let status = Command::new("sudo")
        .args(&[
            "reflector",
            "--verbose",
            "--latest",
            "50",
            "--protocol",
            "https",
            "--sort",
            "rate",
            "--threads",
            "5",
            "--save",
            "/etc/pacman.d/mirrorlist",
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Mirrors ranked by speed!");
            println!("📋 Backup saved to /etc/pacman.d/mirrorlist.backup");
        }
        _ => println!("❌ Failed to rank mirrors"),
    }
}

fn view_current_mirrors() {
    println!("📋 Current Mirror List:");
    println!("======================");

    let _ = Command::new("grep")
        .args(["-E", "^Server", "/etc/pacman.d/mirrorlist"])
        .status();
}

fn manual_edit_mirrors() {
    println!("🔧 Opening mirrorlist in editor...");

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    let _ = Command::new("sudo")
        .args([&editor, "/etc/pacman.d/mirrorlist"])
        .status();
}
