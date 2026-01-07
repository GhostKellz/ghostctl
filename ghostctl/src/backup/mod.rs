pub mod cleanup;
pub mod schedule;
pub mod setup;
pub mod verify;

use crate::tui;
use crate::utils::is_headless;

pub fn backup_menu() {
    // Check for headless mode
    if is_headless() {
        tui::warn("Backup menu cannot be displayed in headless mode. Use CLI subcommands instead.");
        tui::info("Example: ghostctl backup run --paths /home");
        return;
    }

    let options = [
        "🔧 Setup Backup Repository",
        "▶️  Run Manual Backup",
        "📅 Schedule Automated Backups",
        "✅ Verify Backup Integrity",
        "🧹 Cleanup Old Backups",
        "📊 Backup Status",
        "🔧 Restic CLI Tools",
    ];

    loop {
        tui::header("Backup Management");

        if let Some(choice) = tui::select_with_back("Backup Management", &options, 0) {
            match choice {
                0 => setup::setup(),
                1 => setup::run_backup(),
                2 => schedule::setup_schedule(),
                3 => verify::verify_backups(),
                4 => cleanup::cleanup_old_backups(),
                5 => backup_status(),
                6 => crate::restic::setup(),
                _ => {}
            }
        } else {
            break;
        }
    }
}

fn backup_status() {
    tui::header("Backup Status");

    let config_path = match dirs::config_dir() {
        Some(dir) => dir.join("ghostctl/restic.env"),
        None => {
            tui::error("Could not determine config directory");
            return;
        }
    };

    if config_path.exists() {
        tui::success("Backup configuration found");

        // Show recent snapshots
        tui::subheader("Recent Snapshots");
        let _ = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!(
                "source {} && restic snapshots --last --compact",
                config_path.display()
            ))
            .status();

        // Show repository stats
        tui::subheader("Repository Statistics");
        let _ = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!("source {} && restic stats", config_path.display()))
            .status();
    } else {
        tui::error("No backup configuration found");
        tui::info("Run 'Setup Backup Repository' first");
    }
}
