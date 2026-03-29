use dialoguer::{Confirm, Input, Select, MultiSelect, theme::ColorfulTheme};
use std::collections::HashMap;
use std::fs;
use std::process::Command;

pub fn emergency_cleanup_all_snapshots() {
    println!("🚨 EMERGENCY: Removing ALL BTRFS snapshots to free disk space");
    println!("⚠️  This is irreversible and will delete all system snapshots!");

    let confirmed = match Confirm::new()
        .with_prompt("Are you absolutely sure? This cannot be undone!")
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => false,
    };
    if !confirmed {
        println!("❌ Emergency cleanup aborted");
        return;
    }

    // Get current disk usage
    check_disk_space();

    println!("🧹 Removing all BTRFS snapshots...");

    // Method 1: Snapper cleanup (safer)
    println!("📋 Attempting snapper cleanup first...");
    if let Ok(output) = Command::new("snapper")
        .args(["-c", "root", "list", "--columns", "number"])
        .output()
    {
        let snapshots: Vec<&str> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .skip(2) // Skip headers
            .filter(|line| !line.trim().is_empty())
            .collect();

        if !snapshots.is_empty() {
            let snapshot_numbers: String = snapshots.join(" ");
            let status = Command::new("sudo")
                .args(["snapper", "-c", "root", "delete", &snapshot_numbers])
                .status();
            match status {
                Ok(s) if s.success() => println!("✅ Snapper cleanup completed"),
                _ => println!("⚠️  Snapper cleanup failed, trying direct BTRFS method..."),
            }
        }
    }

    // Method 2: Direct BTRFS cleanup (more aggressive)
    println!("🔥 Performing direct BTRFS subvolume cleanup...");
    let status = Command::new("sudo")
        .args(["bash", "-c", "find /.snapshots -maxdepth 2 -name 'snapshot' -type d | while read snap; do btrfs subvolume delete \"$snap\" 2>/dev/null || true; done"])
        .status();

    match status {
        Ok(s) if s.success() => println!("🗑️  Direct cleanup completed"),
        _ => println!("⚠️  Some snapshots may require manual cleanup"),
    }

    // Method 3: Remove snapshot directories
    println!("📂 Cleaning up snapshot directories...");
    let _ = Command::new("sudo")
        .args(["rm", "-rf", "/.snapshots/*"])
        .status();

    println!("🔄 Checking remaining disk space...");
    check_disk_space();

    println!("✅ Emergency cleanup completed");
    println!("📝 Recommend running 'btrfs filesystem usage /' to verify space recovery");
}

pub fn bulk_cleanup_snapshots() {
    println!("🧹 Bulk Snapshot Cleanup");
    println!("========================");

    let cleanup_options = [
        "📅 Delete by age (older than X days)",
        "📊 Delete by number range (e.g., 1-100)",
        "🎯 Delete specific snapshots",
        "🔥 Emergency cleanup (ALL snapshots)",
        "💾 Show disk usage first",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Cleanup Method")
        .items(&cleanup_options)
        .default(4) // Default to show disk usage
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => cleanup_by_age_interactive(),
        1 => cleanup_by_range_interactive(),
        2 => cleanup_specific_snapshots(),
        3 => emergency_cleanup_all_snapshots(),
        4 => {
            check_disk_space();
            bulk_cleanup_snapshots(); // Return to menu
        }
        _ => return,
    }
}

pub fn cleanup_snapshots_by_age(days: &str) {
    println!("🗓️  Deleting snapshots older than {} days...", days);

    let status = Command::new("sudo")
        .args(["snapper", "-c", "root", "delete", "--sync"])
        .arg(format!("--older-than={}", days))
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Age-based cleanup completed"),
        _ => println!("❌ Age-based cleanup failed"),
    }
}

pub fn cleanup_snapshots_by_range(range: &str) {
    println!("🔢 Deleting snapshot range {}...", range);

    let confirmed = match Confirm::new()
        .with_prompt(format!("Delete snapshots {}?", range))
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => false,
    };
    if !confirmed {
        println!("❌ Range cleanup aborted");
        return;
    }

    let status = Command::new("sudo")
        .args(["snapper", "-c", "root", "delete", range])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Range cleanup completed"),
        _ => println!("❌ Range cleanup failed"),
    }
}

fn cleanup_by_age_interactive() {
    let days: String = match Input::new()
        .with_prompt("Delete snapshots older than how many days?")
        .default("30".to_string())
        .interact_text()
    {
        Ok(d) => d,
        Err(_) => return,
    };

    cleanup_snapshots_by_age(&days);
}

fn cleanup_by_range_interactive() {
    let range: String = match Input::new()
        .with_prompt("Enter snapshot range (e.g., 1-100)")
        .interact_text()
    {
        Ok(r) => r,
        Err(_) => return,
    };

    cleanup_snapshots_by_range(&range);
}

fn cleanup_specific_snapshots() {
    // Get available snapshots
    if let Ok(output) = Command::new("snapper")
        .args(["-c", "root", "list"])
        .output()
    {
        let snapshot_list = String::from_utf8_lossy(&output.stdout);
        println!("📋 Available snapshots:");
        println!("{}", snapshot_list);
    }

    let snapshots: String = match Input::new()
        .with_prompt("Enter snapshot numbers to delete (space-separated, e.g., '184 187 188')")
        .interact_text()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    if !snapshots.trim().is_empty() {
        let confirmed = match Confirm::new()
            .with_prompt(format!("Delete snapshots: {}?", snapshots))
            .default(false)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            _ => false,
        };
        if confirmed {
            println!("🎯 Deleting specific snapshots...");

            let status = Command::new("sudo")
                .args(["snapper", "-c", "root", "delete"])
                .args(snapshots.split_whitespace())
                .status();

            match status {
                Ok(s) if s.success() => println!("✅ Specific snapshot cleanup completed"),
                _ => println!("❌ Specific snapshot cleanup failed"),
            }
        }
    }
}

pub fn check_disk_space() {
    println!("💾 Current Disk Usage:");
    println!("=====================");

    // Show overall filesystem usage
    let _ = Command::new("df")
        .args(["-h", "/"])
        .status();

    // Show BTRFS specific usage
    println!("\n🗂️  BTRFS Filesystem Usage:");
    let _ = Command::new("sudo")
        .args(["btrfs", "filesystem", "usage", "/"])
        .status();

    // Show snapshot directory size (with proper error handling)
    println!("\n📸 Snapshot Directory Usage:");
    let output = Command::new("sudo")
        .args(["du", "-sh", "/.snapshots", "2>/dev/null", "||true"])
        .output();

    match output {
        Ok(out) => {
            let usage = String::from_utf8_lossy(&out.stdout);
            if !usage.trim().is_empty() {
                println!("{}", usage);
            } else {
                println!("Unable to calculate snapshot usage (permission issues)");
            }
        }
        _ => println!("Unable to access snapshot directory"),
    }

    // Count snapshots if possible
    if let Ok(output) = Command::new("sudo")
        .args(["find", "/.snapshots", "-maxdepth", "1", "-type", "d", "-name", "[0-9]*"])
        .output()
    {
        let count = String::from_utf8_lossy(&output.stdout).lines().count();
        println!("📊 Total snapshots: {}", count);
    }
}

pub fn comprehensive_snapshot_management() {
    let main_options = [
        "📸 Create Snapshot",
        "📋 List Snapshots",
        "🗑️  Delete Single Snapshot",
        "🧹 Bulk Cleanup Snapshots",
        "💾 Check Disk Space",
        "🔧 Snapper Configuration",
        "🛠️  BTRFS Maintenance",
        "⬅️  Back",
    ];

    loop {
        let choice = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("BTRFS Snapshot Management")
            .items(&main_options)
            .default(4) // Default to disk space check
            .interact_opt()
        {
            Ok(Some(c)) => c,
            _ => break,
        };

        match choice {
            0 => {
                let name: String = match Input::new()
                    .with_prompt("Snapshot name")
                    .interact_text()
                {
                    Ok(n) => n,
                    Err(_) => continue,
                };
                create_snapshot("/@", &name);
            }
            1 => list_snapshots(),
            2 => {
                let name: String = match Input::new()
                    .with_prompt("Snapshot name to delete")
                    .interact_text()
                {
                    Ok(n) => n,
                    Err(_) => continue,
                };
                delete_snapshot(&name);
            }
            3 => bulk_cleanup_snapshots(),
            4 => check_disk_space(),
            5 => snapper_menu(),
            6 => btrfs_maintenance_menu(),
            _ => break,
        }
    }
}

pub fn create_snapshot(subvolume: &str, name: &str) {
    println!("Creating snapshot: {}", name);
    let target = format!("/@snapshots/{}", name);
    let status = std::process::Command::new("sudo")
        .args(["btrfs", "subvolume", "snapshot", subvolume, &target])
        .status();
    match status {
        Ok(s) if s.success() => println!("Snapshot '{}' created.", name),
        _ => println!("Failed to create snapshot."),
    }
}

pub fn list_snapshots() {
    println!("Listing Btrfs snapshots:");
    let output = std::process::Command::new("sudo")
        .args(["btrfs", "subvolume", "list", "/@snapshots"])
        .output();
    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout)),
        Err(_) => println!("Failed to list snapshots."),
    }
}

pub fn delete_snapshot(name: &str) {
    let target = format!("/@snapshots/{}", name);
    let confirmed = match Confirm::new()
        .with_prompt(format!("Delete snapshot '{}'?", name))
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => false,
    };
    if confirmed {
        let status = std::process::Command::new("sudo")
            .args(["btrfs", "subvolume", "delete", &target])
            .status();
        match status {
            Ok(s) if s.success() => println!("Snapshot '{}' deleted.", name),
            _ => println!("Failed to delete snapshot."),
        }
    } else {
        println!("Aborted deletion.");
    }
}

pub fn restore_snapshot(name: &str, target: &str) {
    println!("Restoring snapshot '{}' to '{}'...", name, target);
    let confirmed = match Confirm::new()
        .with_prompt(format!("This will overwrite '{}'. Continue?", target))
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => false,
    };
    if confirmed {
        let source = format!("/@snapshots/{}", name);
        let status = std::process::Command::new("sudo")
            .args(["btrfs", "subvolume", "snapshot", &source, target])
            .status();
        match status {
            Ok(s) if s.success() => println!("Snapshot '{}' restored to '{}'.", name, target),
            _ => println!("Failed to restore snapshot."),
        }
    } else {
        println!("Aborted restore.");
    }
}

// Placeholder functions - implement as needed
fn snapper_menu() {
    println!("Snapper menu - implement as needed");
}

fn btrfs_maintenance_menu() {
    println!("BTRFS maintenance menu - implement as needed");
}