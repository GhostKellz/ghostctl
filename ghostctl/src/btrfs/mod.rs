pub mod snapshot;

use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
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
    loop {
        let options = [
            "📊 Filesystem Overview",
            "📸 Snapshot Management",
            "💾 Backup Integration",
            "⬅️  Back",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🗂️  Btrfs Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => btrfs_filesystem_overview(),
            1 => snapshot_management(),
            2 => backup_integration(),
            _ => break,
        }
    }
}

pub fn btrfs_filesystem_overview() {
    println!("📊 Btrfs Filesystem Overview");
    println!("============================");

    // Check if btrfs tools are installed
    if !check_btrfs_tools() {
        println!("❌ Btrfs tools not found. Please install btrfs-progs.");
        return;
    }

    println!("🗂️  Btrfs Filesystems:");
    let _ = Command::new("sudo")
        .args(&["btrfs", "filesystem", "show"])
        .status();

    println!("\n💾 Disk Usage:");
    let _ = Command::new("sudo")
        .args(&["btrfs", "filesystem", "usage", "/"])
        .status();

    println!("\n📸 Subvolumes:");
    let _ = Command::new("sudo")
        .args(&["btrfs", "subvolume", "list", "/"])
        .status();
}

pub fn snapshot_management() {
    println!("📸 Snapshot Management with Snapper");
    println!("===================================");

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
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Snapshot Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => list_snapshots(),
        1 => create_manual_snapshot(),
        2 => println!("Delete snapshots feature not yet implemented"),
        3 => println!("Rollback feature not yet implemented"),
        4 => snapshot::snapper_menu(),
        _ => return,
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
    println!("This feature is not yet implemented");
}

pub fn automated_backup_workflows() {
    println!("🔄 Automated Backup Workflows");
    println!("=============================");
    println!("This feature is not yet implemented");
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
