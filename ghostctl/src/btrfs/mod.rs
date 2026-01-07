pub mod recovery;
pub mod snapshot;

use crate::tui;
use crate::utils::{is_headless, sudo_run};
// TODO: Remove these imports once all functions are converted to use tui helpers
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::process::Command;

// We'll use the BtrfsAction enum from main.rs

pub fn handle_btrfs_action(action: crate::BtrfsAction) {
    match action {
        crate::BtrfsAction::List => list_snapshots(),
        crate::BtrfsAction::Create { name, subvolume } => {
            snapshot::create_snapshot(&subvolume, &name)
        }
        crate::BtrfsAction::Delete { name } => snapshot::delete_snapshot(&name),
        crate::BtrfsAction::Restore { name, target } => snapshot::restore_snapshot(&name, &target),
        crate::BtrfsAction::SnapperSetup => snapshot::snapper_setup(),
        crate::BtrfsAction::SnapperEdit { config } => snapshot::snapper_edit(&config),
        crate::BtrfsAction::SnapperList => snapshot::snapper_list(),
        crate::BtrfsAction::SnapperCleanup => snapshot::bulk_cleanup_snapshots(),
        crate::BtrfsAction::Status => show_filesystem_status(),
        crate::BtrfsAction::Scrub { mountpoint } => snapshot::scrub(&mountpoint),
        crate::BtrfsAction::Balance { mountpoint } => snapshot::balance(&mountpoint),
        crate::BtrfsAction::Usage { mountpoint } => show_filesystem_usage(&mountpoint),
        crate::BtrfsAction::Quota { mountpoint } => show_quota_info(&mountpoint),
        crate::BtrfsAction::EmergencyCleanup => snapshot::emergency_cleanup_all_snapshots(),
        crate::BtrfsAction::CleanupByAge { days } => {
            snapshot::cleanup_snapshots_by_age(&days);
        }
        crate::BtrfsAction::CleanupByRange { range } => {
            snapshot::cleanup_snapshots_by_range(&range);
        }
        crate::BtrfsAction::DiskSpace => snapshot::check_disk_space(),
    }
}

pub fn btrfs_menu() {
    if is_headless() {
        tui::warn("Btrfs menu cannot be displayed in headless mode. Use CLI subcommands instead.");
        tui::info("Example: ghostctl btrfs list");
        return;
    }

    let options = [
        "📊 Filesystem Overview",
        "📸 Snapshot Management",
        "💾 Backup Integration",
        "🛟 Disaster Recovery",
    ];

    while let Some(choice) = tui::select_with_back("Btrfs Management", &options, 0) {
        match choice {
            0 => btrfs_filesystem_overview(),
            1 => snapshot_management(),
            2 => backup_integration(),
            3 => recovery::disaster_recovery_menu(),
            _ => {}
        }
    }
}

pub fn btrfs_filesystem_overview() {
    tui::header("Btrfs Filesystem Overview");

    // Check if btrfs tools are installed
    if !check_btrfs_tools() {
        tui::error("Btrfs tools not found. Please install btrfs-progs.");
        return;
    }

    tui::subheader("Btrfs Filesystems");
    let _ = sudo_run("btrfs", &["filesystem", "show"]);

    tui::subheader("Disk Usage");
    let _ = sudo_run("btrfs", &["filesystem", "usage", "/"]);

    tui::subheader("Subvolumes");
    let _ = sudo_run("btrfs", &["subvolume", "list", "/"]);
}

pub fn snapshot_management() {
    tui::header("Snapshot Management with Snapper");

    if !check_snapper_installed() {
        offer_snapper_installation();
        return;
    }

    let options = [
        "📋 List all snapshots",
        "📸 Create manual snapshot",
        "🗑️  Delete snapshots",
        "🔄 Rollback to snapshot",
        "⚙️  Configure snapper",
    ];

    if let Some(choice) = tui::select_with_back("Snapshot Management", &options, 0) {
        match choice {
            0 => list_snapshots(),
            1 => create_manual_snapshot(),
            2 => delete_snapshots_interactive(),
            3 => rollback_to_snapshot(),
            4 => snapshot::snapper_menu(),
            _ => {}
        }
    }
}

fn delete_snapshots_interactive() {
    tui::header("Delete Snapshots");

    let configs = get_snapper_configs();
    if configs.is_empty() {
        tui::error("No snapper configurations found. Run snapper setup first.");
        return;
    }

    let config_choice = match tui::select("Select configuration", &configs, 0) {
        Some(c) => c,
        None => return,
    };

    let config = &configs[config_choice];

    // List available snapshots
    tui::subheader(&format!("Available snapshots for '{}'", config));
    let output = Command::new("sudo")
        .args(&["snapper", "-c", config, "list"])
        .output();

    let snapshot_nums: Vec<String> = match output {
        Ok(out) if out.status.success() => {
            let output_str = String::from_utf8_lossy(&out.stdout);
            println!("{}", output_str);

            // Parse snapshot numbers from output
            output_str
                .lines()
                .skip(2) // Skip header lines
                .filter_map(|line| {
                    line.split_whitespace()
                        .next()
                        .and_then(|s| s.parse::<u32>().ok())
                        .map(|n| n.to_string())
                })
                .collect()
        }
        _ => {
            tui::error("Failed to list snapshots");
            return;
        }
    };

    if snapshot_nums.is_empty() {
        tui::success("No snapshots to delete");
        return;
    }

    let delete_options = [
        "🎯 Delete specific snapshot(s)",
        "📅 Delete by age (older than X days)",
        "🔢 Delete by range (e.g., 1-50)",
    ];

    let delete_choice = match tui::select_with_back("Delete method", &delete_options, 0) {
        Some(c) => c,
        None => return,
    };

    match delete_choice {
        0 => {
            let snapshots =
                match tui::input("Enter snapshot number(s) to delete (space-separated)", None) {
                    Some(s) if !s.trim().is_empty() => s,
                    _ => {
                        tui::error("No snapshots specified");
                        return;
                    }
                };

            let nums: Vec<&str> = snapshots.split_whitespace().collect();

            if !tui::confirm_dangerous(&format!("Delete {} snapshot(s)?", nums.len())) {
                tui::info("Deletion cancelled");
                return;
            }

            tui::status("🗑️", "Deleting snapshots...");
            let status = Command::new("sudo")
                .args(&["snapper", "-c", config, "delete"])
                .args(&nums)
                .status();

            match status {
                Ok(s) if s.success() => tui::success("Snapshots deleted successfully"),
                _ => tui::error("Failed to delete some snapshots"),
            }
        }
        1 => {
            let days = tui::input_required("Delete snapshots older than how many days?", "30");

            if !tui::confirm_dangerous(&format!("Delete all snapshots older than {} days?", days)) {
                tui::info("Deletion cancelled");
                return;
            }

            tui::status("🗑️", "Deleting old snapshots...");
            // Note: snapper doesn't have direct age deletion, using cleanup algorithm
            let status = Command::new("sudo")
                .args(&["snapper", "-c", config, "cleanup", "number"])
                .status();

            match status {
                Ok(s) if s.success() => tui::success("Old snapshots cleaned up"),
                _ => tui::error("Cleanup failed"),
            }
        }
        2 => {
            let range = match tui::input("Enter snapshot range (e.g., 1-50)", None) {
                Some(r) if !r.trim().is_empty() => r,
                _ => return,
            };

            if !tui::confirm_dangerous(&format!("Delete snapshot range {}?", range)) {
                tui::info("Deletion cancelled");
                return;
            }

            tui::status("🗑️", "Deleting snapshot range...");
            let status = Command::new("sudo")
                .args(&["snapper", "-c", config, "delete", &range])
                .status();

            match status {
                Ok(s) if s.success() => tui::success("Snapshot range deleted"),
                _ => tui::error("Failed to delete range"),
            }
        }
        _ => {}
    }
}

fn rollback_to_snapshot() {
    tui::header("Rollback to Snapshot");
    tui::warn("WARNING: Rollback will restore your system to a previous state.");
    tui::warn("Any changes made after the snapshot will be lost!");
    println!();

    let configs = get_snapper_configs();
    if configs.is_empty() {
        tui::error("No snapper configurations found.");
        return;
    }

    // For rollback, typically only root config is relevant
    let config = if configs.contains(&"root".to_string()) {
        "root".to_string()
    } else {
        match tui::select("Select configuration", &configs, 0) {
            Some(c) => configs[c].clone(),
            None => return,
        }
    };

    // List available snapshots
    tui::subheader("Available snapshots for rollback");
    let output = Command::new("sudo")
        .args(&["snapper", "-c", &config, "list"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            println!("{}", String::from_utf8_lossy(&out.stdout));
        }
        _ => {
            tui::error("Failed to list snapshots");
            return;
        }
    }

    let snapshot_num = match tui::input("Enter snapshot number to rollback to", None) {
        Some(s) if !s.trim().is_empty() => s,
        _ => {
            tui::error("No snapshot specified");
            return;
        }
    };

    // Show what will change
    tui::subheader("Preview: Changes that will be reverted");
    let _ = Command::new("sudo")
        .args(&["snapper", "-c", &config, "status", &snapshot_num, "0"])
        .status();

    println!();
    tui::warn("IMPORTANT ROLLBACK INFORMATION:");
    tui::info("1. This will create a snapshot of the current state first");
    tui::info("2. Your current state can be restored if needed");
    tui::info("3. A reboot is required after rollback");
    println!();

    if !tui::confirm_dangerous(&format!(
        "Rollback to snapshot {}? (requires reboot)",
        snapshot_num
    )) {
        tui::info("Rollback cancelled");
        return;
    }

    // Create a pre-rollback snapshot
    tui::status("📸", "Creating pre-rollback snapshot...");
    let _ = Command::new("sudo")
        .args(&[
            "snapper",
            "-c",
            &config,
            "create",
            "--description",
            "Pre-rollback snapshot",
        ])
        .status();

    // Perform rollback
    tui::status("🔄", "Performing rollback...");
    let status = Command::new("sudo")
        .args(&["snapper", "-c", &config, "undochange", &snapshot_num, "0"])
        .status();

    match status {
        Ok(s) if s.success() => {
            tui::success("Rollback completed successfully");
            println!();
            tui::info("Next steps:");
            println!("   1. Review the changes that were applied");
            println!("   2. Reboot your system: sudo reboot");
            println!("   3. If issues occur, you can rollback to the pre-rollback snapshot");

            if tui::confirm("Reboot now?", false) {
                tui::status("🔄", "Rebooting...");
                let _ = Command::new("sudo").args(&["reboot"]).status();
            }
        }
        _ => {
            tui::error("Rollback failed");
            tui::info("You can try manual rollback with: snapper rollback <number>");
        }
    }
}

pub fn backup_integration() {
    println!("💾 Backup Integration (Restic + Btrfs)");
    println!("======================================");

    if !check_restic_installed() {
        offer_restic_installation();
        return;
    }

    let options = [
        "🔄 Backup snapshots to restic",
        "⚙️  Setup automated workflows",
        "📊 Backup status",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Backup Integration")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => backup_snapshots_to_restic(),
        1 => automated_backup_workflows(),
        2 => println!("Backup status not yet implemented"),
        _ => return,
    }
}

pub fn list_snapshots() {
    println!("📋 Listing All Snapshots");
    println!("========================");

    // Try snapper first
    let output = Command::new("sudo").args(&["snapper", "list"]).output();

    match output {
        Ok(out) if out.status.success() => {
            println!("{}", String::from_utf8_lossy(&out.stdout));
        }
        _ => {
            // Fall back to btrfs subvolume list
            println!("📸 Listing Btrfs subvolumes:");
            snapshot::list_snapshots();
        }
    }
}

pub fn create_manual_snapshot() {
    println!("📸 Create Manual Snapshot");
    println!("=========================");

    let configs = get_snapper_configs();

    if configs.is_empty() {
        println!("❌ No snapper configurations found. Run snapper setup first.");
        return;
    }

    let config_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select configuration")
        .items(&configs)
        .default(0)
        .interact()
        .unwrap();

    let config = &configs[config_choice];

    let description: String = Input::new()
        .with_prompt("Snapshot description")
        .default("Manual snapshot".into())
        .interact_text()
        .unwrap();

    let status = Command::new("sudo")
        .args(&[
            "snapper",
            "-c",
            config,
            "create",
            "--description",
            &description,
        ])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Snapshot created successfully"),
        _ => println!("❌ Failed to create snapshot"),
    }
}

pub fn backup_snapshots_to_restic() {
    println!("🔄 Backup Btrfs Snapshots to Restic");
    println!("====================================");

    // Check if restic is installed
    if !check_restic_installed() {
        offer_restic_installation();
        return;
    }

    // Check if restic is configured
    let config_path = dirs::config_dir().unwrap().join("ghostctl/restic.env");
    if !config_path.exists() {
        println!("❌ Restic not configured. Please run backup setup first.");
        println!("💡 Go to: Backup Menu > Setup > Initialize New Repository");
        return;
    }

    // Get available snapshot configurations
    let configs = get_snapper_configs();
    if configs.is_empty() {
        println!("❌ No snapper configurations found.");
        println!("💡 Set up snapper first: Btrfs Menu > Snapshot Management > Configure snapper");
        return;
    }

    let backup_options = [
        "📸 Backup latest snapshot",
        "📋 Backup specific snapshot",
        "🔄 Backup all recent snapshots",
        "⚙️  Configure snapshot backup",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Snapshot Backup Options")
        .items(&backup_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => backup_latest_snapshot(&config_path, &configs),
        1 => backup_specific_snapshot(&config_path, &configs),
        2 => backup_recent_snapshots(&config_path, &configs),
        3 => configure_snapshot_backup(),
        _ => return,
    }
}

fn backup_latest_snapshot(config_path: &std::path::Path, configs: &[String]) {
    println!("\n📸 Backing up Latest Snapshot");
    println!("==============================");

    let config = if configs.len() == 1 {
        configs[0].clone()
    } else {
        let config_choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select snapper configuration")
            .items(configs)
            .default(0)
            .interact()
            .unwrap();
        configs[config_choice].clone()
    };

    // Get latest snapshot
    let output = Command::new("sudo")
        .args(&["snapper", "-c", &config, "list", "--columns", "number"])
        .output();

    let latest_num = match output {
        Ok(out) if out.status.success() => {
            let output_str = String::from_utf8_lossy(&out.stdout);
            output_str
                .lines()
                .skip(2)
                .filter_map(|line| line.trim().parse::<u32>().ok())
                .max()
        }
        _ => {
            println!("❌ Failed to get snapshot list");
            return;
        }
    };

    let snapshot_num = match latest_num {
        Some(n) => n,
        None => {
            println!("❌ No snapshots found");
            return;
        }
    };

    println!("📸 Latest snapshot: #{}", snapshot_num);

    // Get snapshot path
    let snapshot_path = format!("/.snapshots/{}/snapshot", snapshot_num);

    if !std::path::Path::new(&snapshot_path).exists() {
        println!("❌ Snapshot path not found: {}", snapshot_path);
        return;
    }

    println!("📂 Snapshot path: {}", snapshot_path);

    let confirm = Confirm::new()
        .with_prompt("Backup this snapshot to restic?")
        .default(true)
        .interact()
        .unwrap();

    if !confirm {
        println!("❌ Backup cancelled");
        return;
    }

    println!("🚀 Starting backup to restic...");
    println!("   This may take a while depending on snapshot size.");

    let status = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "source {} && restic backup --tag btrfs-snapshot --tag snapshot-{} {}",
            config_path.display(),
            snapshot_num,
            snapshot_path
        ))
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Snapshot #{} backed up successfully", snapshot_num);

            // Show backup info
            println!("\n📊 Recent restic snapshots:");
            let _ = Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "source {} && restic snapshots --last 5",
                    config_path.display()
                ))
                .status();
        }
        _ => {
            println!("❌ Backup failed");
            println!("💡 Check restic configuration and repository access");
        }
    }
}

fn backup_specific_snapshot(config_path: &std::path::Path, configs: &[String]) {
    println!("\n📋 Backup Specific Snapshot");
    println!("===========================");

    let config = if configs.len() == 1 {
        configs[0].clone()
    } else {
        let config_choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select snapper configuration")
            .items(configs)
            .default(0)
            .interact()
            .unwrap();
        configs[config_choice].clone()
    };

    // List snapshots
    println!("\n📋 Available snapshots:");
    let _ = Command::new("sudo")
        .args(&["snapper", "-c", &config, "list"])
        .status();

    let snapshot_num: String = Input::new()
        .with_prompt("Enter snapshot number to backup")
        .interact_text()
        .unwrap();

    let snapshot_path = format!("/.snapshots/{}/snapshot", snapshot_num);

    if !std::path::Path::new(&snapshot_path).exists() {
        println!("❌ Snapshot path not found: {}", snapshot_path);
        return;
    }

    let confirm = Confirm::new()
        .with_prompt(format!("Backup snapshot #{} to restic?", snapshot_num))
        .default(true)
        .interact()
        .unwrap();

    if !confirm {
        println!("❌ Backup cancelled");
        return;
    }

    println!("🚀 Starting backup...");

    let status = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "source {} && restic backup --tag btrfs-snapshot --tag snapshot-{} {}",
            config_path.display(),
            snapshot_num,
            snapshot_path
        ))
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Snapshot #{} backed up successfully", snapshot_num),
        _ => println!("❌ Backup failed"),
    }
}

fn backup_recent_snapshots(config_path: &std::path::Path, configs: &[String]) {
    println!("\n🔄 Backup Recent Snapshots");
    println!("==========================");

    let config = if configs.len() == 1 {
        configs[0].clone()
    } else {
        let config_choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select snapper configuration")
            .items(configs)
            .default(0)
            .interact()
            .unwrap();
        configs[config_choice].clone()
    };

    let count: String = Input::new()
        .with_prompt("How many recent snapshots to backup?")
        .default("5".to_string())
        .interact_text()
        .unwrap();

    let count_num: usize = count.parse().unwrap_or(5);

    // Get recent snapshots
    let output = Command::new("sudo")
        .args(&["snapper", "-c", &config, "list", "--columns", "number"])
        .output();

    let snapshot_nums: Vec<u32> = match output {
        Ok(out) if out.status.success() => {
            let output_str = String::from_utf8_lossy(&out.stdout);
            let mut nums: Vec<u32> = output_str
                .lines()
                .skip(2)
                .filter_map(|line| line.trim().parse::<u32>().ok())
                .collect();
            nums.sort_by(|a, b| b.cmp(a)); // Sort descending
            nums.into_iter().take(count_num).collect()
        }
        _ => {
            println!("❌ Failed to get snapshot list");
            return;
        }
    };

    if snapshot_nums.is_empty() {
        println!("❌ No snapshots found");
        return;
    }

    println!(
        "📸 Will backup {} snapshot(s): {:?}",
        snapshot_nums.len(),
        snapshot_nums
    );

    let confirm = Confirm::new()
        .with_prompt("Start backup of these snapshots?")
        .default(true)
        .interact()
        .unwrap();

    if !confirm {
        println!("❌ Backup cancelled");
        return;
    }

    let mut success_count = 0;
    for num in &snapshot_nums {
        let snapshot_path = format!("/.snapshots/{}/snapshot", num);

        if !std::path::Path::new(&snapshot_path).exists() {
            println!("  ⚠️  Snapshot #{} path not found, skipping", num);
            continue;
        }

        println!("  📸 Backing up snapshot #{}...", num);

        let status = Command::new("bash")
            .arg("-c")
            .arg(format!(
                "source {} && restic backup --tag btrfs-snapshot --tag snapshot-{} {} 2>&1",
                config_path.display(),
                num,
                snapshot_path
            ))
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("    ✅ Done");
                success_count += 1;
            }
            _ => println!("    ❌ Failed"),
        }
    }

    println!(
        "\n📊 Backup Summary: {}/{} snapshots backed up",
        success_count,
        snapshot_nums.len()
    );
}

fn configure_snapshot_backup() {
    println!("\n⚙️  Configure Snapshot Backup");
    println!("=============================");
    println!("💡 Configure automated snapshot-to-restic backups");
    println!();
    println!("Options:");
    println!("  1. Set up systemd timer for automatic backups");
    println!("  2. Configure retention policy");
    println!("  3. Set backup tags and metadata");
    println!();
    println!("Use 'Automated Backup Workflows' for timer setup.");
}

pub fn automated_backup_workflows() {
    println!("🔄 Automated Backup Workflows");
    println!("=============================");

    let options = [
        "⏰ Create systemd timer for btrfs-to-restic backup",
        "📋 View existing backup timers",
        "🔧 Edit backup schedule",
        "🗑️  Remove backup timer",
        "📊 View backup status",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Automated Workflows")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => create_btrfs_backup_timer(),
        1 => view_backup_timers(),
        2 => edit_backup_schedule(),
        3 => remove_backup_timer(),
        4 => view_backup_status(),
        _ => return,
    }
}

fn create_btrfs_backup_timer() {
    println!("\n⏰ Create Automated Backup Timer");
    println!("=================================");

    let config_path = dirs::config_dir().unwrap().join("ghostctl/restic.env");
    if !config_path.exists() {
        println!("❌ Restic not configured. Please run backup setup first.");
        return;
    }

    let frequency_options = [
        "Daily (2:00 AM)",
        "Twice daily (2:00 AM, 2:00 PM)",
        "Weekly (Sunday 3:00 AM)",
        "Custom schedule",
    ];

    let freq_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Backup frequency")
        .items(&frequency_options)
        .default(0)
        .interact()
        .unwrap();

    let timer_spec = match freq_choice {
        0 => "*-*-* 02:00:00".to_string(),
        1 => "*-*-* 02,14:00:00".to_string(),
        2 => "Sun *-*-* 03:00:00".to_string(),
        3 => {
            let custom: String = Input::new()
                .with_prompt("Enter systemd calendar spec (e.g., '*-*-* 04:00:00')")
                .interact_text()
                .unwrap();
            custom
        }
        _ => return,
    };

    // Get snapper config to backup
    let configs = get_snapper_configs();
    let snapper_config = if configs.is_empty() {
        println!("⚠️  No snapper configs found, will backup /home and /etc");
        "".to_string()
    } else if configs.len() == 1 {
        configs[0].clone()
    } else {
        let config_choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Snapper config to backup (or select for paths)")
            .items(&configs)
            .default(0)
            .interact()
            .unwrap();
        configs[config_choice].clone()
    };

    // Create the backup script
    let script_content = if snapper_config.is_empty() {
        format!(
            r#"#!/bin/bash
# GhostCTL Automated Backup Script
set -e

# Load restic configuration
source {}

# Backup standard paths
echo "Starting backup of /home and /etc..."
restic backup --tag ghostctl-auto --tag daily /home /etc

# Cleanup old backups
echo "Cleaning up old backups..."
restic forget --prune --keep-daily 7 --keep-weekly 4 --keep-monthly 12

echo "Backup completed successfully"
"#,
            config_path.display()
        )
    } else {
        format!(
            r#"#!/bin/bash
# GhostCTL Automated Btrfs Snapshot Backup Script
set -e

# Load restic configuration
source {}

# Get latest snapshot number
LATEST=$(sudo snapper -c {} list --columns number | tail -n1 | tr -d ' ')

if [ -z "$LATEST" ]; then
    echo "No snapshots found"
    exit 1
fi

SNAPSHOT_PATH="/.snapshots/$LATEST/snapshot"

if [ ! -d "$SNAPSHOT_PATH" ]; then
    echo "Snapshot path not found: $SNAPSHOT_PATH"
    exit 1
fi

echo "Backing up snapshot #$LATEST from $SNAPSHOT_PATH..."
restic backup --tag ghostctl-auto --tag btrfs-snapshot --tag snapshot-$LATEST "$SNAPSHOT_PATH"

# Cleanup old backups
echo "Cleaning up old backups..."
restic forget --prune --keep-daily 7 --keep-weekly 4 --keep-monthly 12

echo "Backup completed successfully"
"#,
            config_path.display(),
            snapper_config
        )
    };

    let script_dir = dirs::config_dir().unwrap().join("ghostctl/scripts");
    std::fs::create_dir_all(&script_dir).unwrap();
    let script_path = script_dir.join("btrfs-backup.sh");
    std::fs::write(&script_path, script_content).unwrap();

    // Make script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&script_path).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&script_path, perms).unwrap();
    }

    // Create systemd service
    let service_content = format!(
        r#"[Unit]
Description=GhostCTL Btrfs Snapshot Backup
After=network-online.target
Wants=network-online.target

[Service]
Type=oneshot
ExecStart={}
StandardOutput=journal
StandardError=journal
"#,
        script_path.display()
    );

    // Create systemd timer
    let timer_content = format!(
        r#"[Unit]
Description=GhostCTL Btrfs Backup Timer
Documentation=https://github.com/ghostkellz/ghostctl

[Timer]
OnCalendar={}
Persistent=true
RandomizedDelaySec=300

[Install]
WantedBy=timers.target
"#,
        timer_spec
    );

    let systemd_dir = dirs::config_dir().unwrap().join("systemd/user");
    std::fs::create_dir_all(&systemd_dir).unwrap();

    let service_path = systemd_dir.join("ghostctl-backup.service");
    let timer_path = systemd_dir.join("ghostctl-backup.timer");

    std::fs::write(&service_path, service_content).unwrap();
    std::fs::write(&timer_path, timer_content).unwrap();

    println!("✅ Created systemd files:");
    println!("   📄 Service: {}", service_path.display());
    println!("   ⏰ Timer: {}", timer_path.display());
    println!("   📜 Script: {}", script_path.display());
    println!();
    println!("📋 To enable the timer:");
    println!("   systemctl --user daemon-reload");
    println!("   systemctl --user enable --now ghostctl-backup.timer");
    println!();
    println!("📊 To check timer status:");
    println!("   systemctl --user status ghostctl-backup.timer");
    println!("   systemctl --user list-timers");

    let enable_now = Confirm::new()
        .with_prompt("Enable and start the timer now?")
        .default(true)
        .interact()
        .unwrap();

    if enable_now {
        let _ = Command::new("systemctl")
            .args(&["--user", "daemon-reload"])
            .status();

        let status = Command::new("systemctl")
            .args(&["--user", "enable", "--now", "ghostctl-backup.timer"])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Timer enabled and started");
                let _ = Command::new("systemctl")
                    .args(&["--user", "status", "ghostctl-backup.timer"])
                    .status();
            }
            _ => {
                println!("❌ Failed to enable timer");
                println!("💡 Try manually: systemctl --user enable --now ghostctl-backup.timer");
            }
        }
    }
}

fn view_backup_timers() {
    println!("\n📋 Backup Timers");
    println!("================");

    println!("User timers:");
    let _ = Command::new("systemctl")
        .args(&["--user", "list-timers", "--all"])
        .status();

    println!("\nGhostCTL backup timer status:");
    let _ = Command::new("systemctl")
        .args(&["--user", "status", "ghostctl-backup.timer"])
        .status();
}

fn edit_backup_schedule() {
    println!("\n🔧 Edit Backup Schedule");
    println!("=======================");

    let timer_path = dirs::config_dir()
        .unwrap()
        .join("systemd/user/ghostctl-backup.timer");

    if !timer_path.exists() {
        println!("❌ No backup timer found. Create one first.");
        return;
    }

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    let _ = Command::new(&editor).arg(&timer_path).status();

    println!("💡 Reload timer with: systemctl --user daemon-reload");
    let reload = Confirm::new()
        .with_prompt("Reload systemd now?")
        .default(true)
        .interact()
        .unwrap();

    if reload {
        let _ = Command::new("systemctl")
            .args(&["--user", "daemon-reload"])
            .status();
        println!("✅ Systemd reloaded");
    }
}

fn remove_backup_timer() {
    println!("\n🗑️  Remove Backup Timer");
    println!("=======================");

    let confirm = Confirm::new()
        .with_prompt("Remove GhostCTL backup timer?")
        .default(false)
        .interact()
        .unwrap();

    if !confirm {
        println!("❌ Removal cancelled");
        return;
    }

    // Stop and disable timer
    let _ = Command::new("systemctl")
        .args(&["--user", "disable", "--now", "ghostctl-backup.timer"])
        .status();

    // Remove files
    let systemd_dir = dirs::config_dir().unwrap().join("systemd/user");
    let _ = std::fs::remove_file(systemd_dir.join("ghostctl-backup.service"));
    let _ = std::fs::remove_file(systemd_dir.join("ghostctl-backup.timer"));

    let _ = Command::new("systemctl")
        .args(&["--user", "daemon-reload"])
        .status();

    println!("✅ Backup timer removed");
}

fn view_backup_status() {
    println!("\n📊 Backup Status");
    println!("================");

    let config_path = dirs::config_dir().unwrap().join("ghostctl/restic.env");

    if !config_path.exists() {
        println!("❌ Restic not configured");
        return;
    }

    println!("📋 Recent restic snapshots:");
    let _ = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "source {} && restic snapshots --last 10",
            config_path.display()
        ))
        .status();

    println!("\n📊 Repository statistics:");
    let _ = Command::new("bash")
        .arg("-c")
        .arg(format!("source {} && restic stats", config_path.display()))
        .status();

    println!("\n⏰ Timer status:");
    let _ = Command::new("systemctl")
        .args(&["--user", "status", "ghostctl-backup.timer"])
        .status();

    println!("\n📜 Last backup log:");
    let _ = Command::new("journalctl")
        .args(&["--user", "-u", "ghostctl-backup.service", "-n", "20"])
        .status();
}

// Helper functions
fn check_btrfs_tools() -> bool {
    Command::new("which").arg("btrfs").status().is_ok()
}

fn check_snapper_installed() -> bool {
    Command::new("which").arg("snapper").status().is_ok()
}

fn check_restic_installed() -> bool {
    Command::new("which").arg("restic").status().is_ok()
}

fn offer_snapper_installation() {
    println!("❌ Snapper is not installed.");
    let install = Confirm::new()
        .with_prompt("Install snapper?")
        .default(true)
        .interact()
        .unwrap();

    if install {
        let _ = Command::new("sudo")
            .args(&["pacman", "-S", "--noconfirm", "snapper"])
            .status();
        println!("✅ Snapper installed. Configure it with 'sudo snapper -c root create-config /'");
    }
}

fn offer_restic_installation() {
    println!("❌ Restic is not installed.");
    let install = Confirm::new()
        .with_prompt("Install restic?")
        .default(true)
        .interact()
        .unwrap();

    if install {
        let _ = Command::new("sudo")
            .args(&["pacman", "-S", "--noconfirm", "restic"])
            .status();
        println!("✅ Restic installed");
    }
}

fn get_snapper_configs() -> Vec<String> {
    let mut configs = Vec::new();

    if let Ok(output) = Command::new("sudo")
        .args(&["snapper", "list-configs"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines().skip(2) {
            // Skip header lines
            if let Some(config_name) = line.split_whitespace().next() {
                configs.push(config_name.to_string());
            }
        }
    }

    configs
}

pub fn show_filesystem_status() {
    println!("📊 Btrfs Filesystem Status");
    println!("==========================");

    // Check if btrfs tools are installed
    if !check_btrfs_tools() {
        println!("❌ Btrfs tools not found. Please install btrfs-progs.");
        return;
    }

    println!("🗂️  Filesystem Show:");
    let _ = Command::new("sudo")
        .args(&["btrfs", "filesystem", "show"])
        .status();

    println!("\n📸 Subvolumes:");
    let _ = Command::new("sudo")
        .args(&["btrfs", "subvolume", "list", "/"])
        .status();

    println!("\n🔍 Device Statistics:");
    let _ = Command::new("sudo")
        .args(&["btrfs", "device", "stats", "/"])
        .status();
}

pub fn show_filesystem_usage(mountpoint: &str) {
    println!("💾 Btrfs Filesystem Usage: {}", mountpoint);
    println!("==========================================");

    // Check if btrfs tools are installed
    if !check_btrfs_tools() {
        println!("❌ Btrfs tools not found. Please install btrfs-progs.");
        return;
    }

    println!("📊 Filesystem Usage:");
    let _ = Command::new("sudo")
        .args(&["btrfs", "filesystem", "usage", mountpoint])
        .status();

    println!("\n📈 Space Info:");
    let _ = Command::new("sudo")
        .args(&["btrfs", "filesystem", "df", mountpoint])
        .status();
}

pub fn show_quota_info(mountpoint: &str) {
    println!("📏 Btrfs Quota Information: {}", mountpoint);
    println!("==========================================");

    // Check if btrfs tools are installed
    if !check_btrfs_tools() {
        println!("❌ Btrfs tools not found. Please install btrfs-progs.");
        return;
    }

    // Check if quotas are enabled
    let status = Command::new("sudo")
        .args(&["btrfs", "qgroup", "show", mountpoint])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Quotas are enabled");
            println!("\n📊 Quota Groups:");
            let _ = Command::new("sudo")
                .args(&["btrfs", "qgroup", "show", "-p", mountpoint])
                .status();
        }
        _ => {
            println!("❌ Quotas are not enabled on this filesystem");
            println!(
                "💡 To enable quotas, run: sudo btrfs quota enable {}",
                mountpoint
            );
            println!("⚠️  Note: Enabling quotas can impact performance on large filesystems");
        }
    }
}
