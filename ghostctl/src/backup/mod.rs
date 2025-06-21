pub mod cleanup;
pub mod schedule;
pub mod setup;
pub mod verify;

use dialoguer::{Select, theme::ColorfulTheme};

pub fn backup_menu() {
    println!("💾 Backup Management");
    println!("===================");

    let options = [
        "🔧 Setup Backup Repository",
        "▶️  Run Manual Backup",
        "📅 Schedule Automated Backups",
        "✅ Verify Backup Integrity",
        "🧹 Cleanup Old Backups",
        "📊 Backup Status",
        "🔧 Restic CLI Tools",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Backup Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => setup::setup(),
        1 => setup::run_backup(),
        2 => schedule::setup_schedule(),
        3 => verify::verify_backups(),
        4 => cleanup::cleanup_old_backups(),
        5 => backup_status(),
        6 => crate::restic::setup(),
        _ => return,
    }
}

fn backup_status() {
    println!("📊 Backup Status");
    println!("================");

    let config_path = dirs::config_dir().unwrap().join("ghostctl/restic.env");
    if config_path.exists() {
        println!("✅ Backup configuration found");

        // Show recent snapshots
        println!("\n📋 Recent snapshots:");
        let _ = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!(
                "source {} && restic snapshots --last --compact",
                config_path.display()
            ))
            .status();

        // Show repository stats
        println!("\n📊 Repository statistics:");
        let _ = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!("source {} && restic stats", config_path.display()))
            .status();
    } else {
        println!("❌ No backup configuration found");
        println!("Run 'Setup Backup Repository' first");
    }
}
