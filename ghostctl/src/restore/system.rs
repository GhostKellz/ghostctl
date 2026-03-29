use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::path::Path;
use std::process::Command;

pub fn restore_from_restic() {
    println!("💾 Restore from Restic Backup");
    println!("=============================");

    let config_path = match dirs::config_dir() {
        Some(dir) => dir.join("ghostctl/restic.env"),
        None => {
            println!("❌ Could not determine config directory");
            return;
        }
    };
    if !config_path.exists() {
        println!("❌ No restic configuration found. Run backup setup first.");
        return;
    }

    // List available snapshots
    println!("📋 Available snapshots:");
    let _ = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "source {} && restic snapshots",
            config_path.display()
        ))
        .status();

    let snapshot_id: String = match Input::new()
        .with_prompt("Snapshot ID to restore")
        .interact_text()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    let restore_path: String = match Input::new()
        .with_prompt("Restore to path")
        .default("/tmp/restic-restore".into())
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let confirm = match Confirm::new()
        .with_prompt(format!(
            "Restore snapshot {} to {}?",
            snapshot_id, restore_path
        ))
        .default(false)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    if confirm {
        println!("🔄 Restoring snapshot...");
        let status = Command::new("bash")
            .arg("-c")
            .arg(format!(
                "source {} && restic restore {} --target {}",
                config_path.display(),
                snapshot_id,
                restore_path
            ))
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Restore completed successfully"),
            _ => println!("❌ Restore failed"),
        }
    }
}

pub fn rollback_btrfs_snapshot() {
    println!("📸 Rollback Btrfs Snapshot");
    println!("==========================");

    // Check if btrfs tools are available
    if Command::new("which").arg("btrfs").status().is_err() {
        println!("❌ Btrfs tools not found. Please install btrfs-progs.");
        return;
    }

    // List available snapshots
    println!("📋 Available Btrfs snapshots:");
    let _ = Command::new("sudo")
        .args(&["btrfs", "subvolume", "list", "/"])
        .status();

    let snapshot_name: String = match Input::new().with_prompt("Snapshot name").interact_text() {
        Ok(s) => s,
        Err(_) => return,
    };

    let target: String = match Input::new()
        .with_prompt("Target subvolume (e.g., /)")
        .default("/".into())
        .interact_text()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    let confirm = match Confirm::new()
        .with_prompt(format!(
            "⚠️  This will replace {} with snapshot {}. Continue?",
            target, snapshot_name
        ))
        .default(false)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    if confirm {
        println!("🔄 Rolling back to snapshot...");

        // This is a simplified example - real rollback is more complex
        let source = format!("/@snapshots/{}", snapshot_name);
        let status = Command::new("sudo")
            .args(&["btrfs", "subvolume", "snapshot", &source, &target])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Snapshot rollback completed");
                println!("⚠️  You may need to reboot for changes to take effect");
            }
            _ => println!("❌ Rollback failed"),
        }
    }
}

pub fn enter_recovery_chroot() {
    println!("🛠️  Enter Recovery Chroot");
    println!("========================");

    let mountpoint: String = match Input::new()
        .with_prompt("Root filesystem mountpoint")
        .default("/mnt".into())
        .interact_text()
    {
        Ok(m) => m,
        Err(_) => return,
    };

    if !Path::new(&mountpoint).exists() {
        println!("❌ Mountpoint {} does not exist", mountpoint);
        return;
    }

    let confirm = match Confirm::new()
        .with_prompt(format!("Enter chroot environment at {}?", mountpoint))
        .default(false)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    if confirm {
        setup_chroot_environment(&mountpoint);
    }
}

pub fn full_system_recovery() {
    println!("🔄 Full System Recovery");
    println!("======================");

    let recovery_options = [
        "🔄 Restore system from latest backup",
        "📸 Rollback to last known good snapshot",
        "🛠️  Manual recovery (chroot + repair)",
        "📋 Recovery diagnostics",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Recovery Type")
        .items(&recovery_options)
        .default(0)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    match choice {
        0 => restore_system_from_backup(),
        1 => rollback_to_last_snapshot(),
        2 => manual_system_recovery(),
        3 => recovery_diagnostics(),
        _ => return,
    }
}

pub fn list_available_backups() {
    println!("📋 Available Backups");
    println!("====================");

    // List Restic snapshots
    println!("💾 Restic Snapshots:");
    let config_path = match dirs::config_dir() {
        Some(dir) => dir.join("ghostctl/restic.env"),
        None => {
            println!("  ❌ Could not determine config directory");
            // Continue to show other backup types
            std::path::PathBuf::new()
        }
    };
    if config_path.exists() {
        let _ = Command::new("bash")
            .arg("-c")
            .arg(format!(
                "source {} && restic snapshots --compact",
                config_path.display()
            ))
            .status();
    } else {
        println!("  ❌ No restic configuration found");
    }

    // List Btrfs snapshots
    println!("\n📸 Btrfs Snapshots:");
    if Command::new("which").arg("btrfs").status().is_ok() {
        let _ = Command::new("sudo")
            .args(&["btrfs", "subvolume", "list", "/"])
            .status();
    } else {
        println!("  ❌ Btrfs tools not available");
    }

    // List Snapper snapshots
    println!("\n📷 Snapper Snapshots:");
    if Command::new("which").arg("snapper").status().is_ok() {
        let _ = Command::new("sudo").args(&["snapper", "list"]).status();
    } else {
        println!("  ❌ Snapper not available");
    }
}

// Helper functions
fn setup_chroot_environment(mountpoint: &str) {
    println!("🔧 Setting up chroot environment...");

    // Bind mount essential directories
    let mount_points = ["/dev", "/proc", "/sys", "/run"];

    for mount_point in &mount_points {
        let target = format!("{}{}", mountpoint, mount_point);
        let _ = Command::new("sudo")
            .args(&["mount", "--bind", mount_point, &target])
            .status();
    }

    // Mount /boot and /efi if they exist
    for boot_dir in ["/boot", "/efi"] {
        let target = format!("{}{}", mountpoint, boot_dir);
        if Path::new(&target).exists() {
            let _ = Command::new("sudo")
                .args(&["mount", "--bind", boot_dir, &target])
                .status();
        }
    }

    println!("🚀 Entering chroot environment...");
    println!("Type 'exit' to leave the chroot environment");

    let _ = Command::new("sudo")
        .args(&["arch-chroot", mountpoint])
        .status();

    // Cleanup after chroot exit
    cleanup_chroot_environment(mountpoint);
}

fn cleanup_chroot_environment(mountpoint: &str) {
    println!("🧹 Cleaning up chroot environment...");

    // Unmount in reverse order
    let mount_points = ["/efi", "/boot", "/run", "/sys", "/proc", "/dev"];

    for mount_point in &mount_points {
        let target = format!("{}{}", mountpoint, mount_point);
        let _ = Command::new("sudo")
            .args(&["umount", "-l", &target])
            .status();
    }

    println!("✅ Chroot environment cleaned up");
}

fn restore_system_from_backup() {
    println!("🔄 Restore System from Backup");
    println!("=============================");

    let backup_type = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Backup source")
        .items(&[
            "💾 Latest Restic backup",
            "📸 Latest Btrfs snapshot",
            "📷 Latest Snapper snapshot",
        ])
        .default(0)
        .interact()
    {
        Ok(b) => b,
        Err(_) => return,
    };

    match backup_type {
        0 => {
            println!("🔄 Restoring from latest Restic backup...");
            restore_from_restic();
        }
        1 => {
            println!("🔄 Rolling back to latest Btrfs snapshot...");
            rollback_btrfs_snapshot();
        }
        2 => {
            println!("🔄 Rolling back to latest Snapper snapshot...");
            rollback_snapper_snapshot();
        }
        _ => return,
    }
}

fn rollback_to_last_snapshot() {
    println!("📸 Rollback to Last Known Good Snapshot");
    println!("=======================================");

    // Find the most recent snapshot
    if Command::new("which").arg("snapper").status().is_ok() {
        println!("🔍 Finding latest snapshot...");
        let _ = Command::new("sudo").args(&["snapper", "list"]).status();

        let snapshot_id: String = match Input::new()
            .with_prompt("Snapshot number to rollback to")
            .interact_text()
        {
            Ok(s) => s,
            Err(_) => return,
        };

        let confirm = match Confirm::new()
            .with_prompt(format!("Rollback to snapshot {}?", snapshot_id))
            .default(false)
            .interact()
        {
            Ok(c) => c,
            Err(_) => return,
        };

        if confirm {
            let _ = Command::new("sudo")
                .args(&["snapper", "undochange", &format!("{}..0", snapshot_id)])
                .status();
        }
    } else {
        println!("❌ Snapper not available. Using Btrfs snapshots...");
        rollback_btrfs_snapshot();
    }
}

fn manual_system_recovery() {
    println!("🛠️  Manual System Recovery");
    println!("=========================");

    let recovery_steps = [
        "🔍 Check filesystem integrity",
        "🔧 Repair filesystem errors",
        "🛠️  Enter chroot for manual fixes",
        "🔄 Rebuild initramfs",
        "⚙️  Fix bootloader",
    ];

    println!("Recovery steps available:");
    for (i, step) in recovery_steps.iter().enumerate() {
        println!("  {}. {}", i + 1, step);
    }

    let step = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select recovery step")
        .items(&recovery_steps)
        .default(0)
        .interact()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    match step {
        0 => check_filesystem_integrity(),
        1 => repair_filesystem(),
        2 => enter_recovery_chroot(),
        3 => rebuild_initramfs(),
        4 => fix_bootloader(),
        _ => return,
    }
}

fn recovery_diagnostics() {
    println!("📋 Recovery Diagnostics");
    println!("=======================");

    println!("🔍 System Status:");
    println!("  Boot Status: Checking...");
    let _ = Command::new("systemctl")
        .args(&["is-system-running"])
        .status();

    println!("\n💾 Filesystem Status:");
    let _ = Command::new("df").args(&["-h"]).status();

    println!("\n🔧 Kernel Modules:");
    let _ = Command::new("lsmod").status();

    println!("\n📝 Recent Boot Logs:");
    let _ = Command::new("journalctl")
        .args(&["-b", "-p", "err", "--no-pager", "-n", "10"])
        .status();
}

fn rollback_snapper_snapshot() {
    println!("📷 Rollback Snapper Snapshot");
    if Command::new("which").arg("snapper").status().is_ok() {
        let _ = Command::new("sudo").args(&["snapper", "list"]).status();
    } else {
        println!("❌ Snapper not available");
    }
}

fn check_filesystem_integrity() {
    println!("🔍 Checking filesystem integrity...");
    let _ = Command::new("sudo").args(&["fsck", "-f", "/"]).status();
}

fn repair_filesystem() {
    println!("🔧 Repairing filesystem...");
    let _ = Command::new("sudo").args(&["fsck", "-y", "/"]).status();
}

fn rebuild_initramfs() {
    println!("🔄 Rebuilding initramfs...");
    let _ = Command::new("sudo").args(&["mkinitcpio", "-P"]).status();
}

fn fix_bootloader() {
    println!("⚙️  Fixing bootloader...");
    let bootloader = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Bootloader type")
        .items(&["GRUB", "systemd-boot", "rEFInd"])
        .default(0)
        .interact()
    {
        Ok(b) => b,
        Err(_) => return,
    };

    match bootloader {
        0 => {
            let _ = Command::new("sudo")
                .args(&["grub-install", "/dev/sda"])
                .status();
            let _ = Command::new("sudo")
                .args(&["grub-mkconfig", "-o", "/boot/grub/grub.cfg"])
                .status();
        }
        1 => {
            let _ = Command::new("sudo").args(&["bootctl", "install"]).status();
        }
        _ => println!("Manual bootloader repair needed"),
    }
}
