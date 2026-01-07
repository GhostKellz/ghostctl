//! Btrfs Disaster Recovery Module
//!
//! Provides TUI for disaster recovery operations including:
//! - Snapshot browsing and comparison
//! - File-level restore from snapshots
//! - Full system rollback
//! - Snapshot verification and health checks

use crate::progress::Spinner;
use crate::tui::{
    confirm, confirm_dangerous, error, header, icons, info, input, select_with_back, success, warn,
};
use crate::utils::{is_dry_run, sudo_run, sudo_shell};
use std::path::Path;
use std::process::Command;

/// Snapshot information for recovery
#[derive(Debug, Clone)]
pub struct RecoverySnapshot {
    pub number: u32,
    pub date: String,
    pub description: String,
    pub snapshot_type: String,
    pub cleanup: String,
    pub path: String,
}

/// Disaster recovery menu
pub fn disaster_recovery_menu() {
    loop {
        header("Btrfs Disaster Recovery");

        let options = [
            "Browse Snapshots (File Recovery)",
            "Compare Snapshots",
            "Rollback System",
            "Verify Snapshot Health",
            "Emergency Recovery Tools",
            "View Recovery History",
            "Back",
        ];

        match select_with_back("Choose recovery option", &options, 0) {
            Some(0) => browse_snapshots_for_recovery(),
            Some(1) => compare_snapshots(),
            Some(2) => system_rollback_wizard(),
            Some(3) => verify_snapshot_health(),
            Some(4) => emergency_recovery_tools(),
            Some(5) => view_recovery_history(),
            _ => break,
        }
    }
}

/// Browse snapshots for file-level recovery
fn browse_snapshots_for_recovery() {
    let configs = get_snapper_configs();
    if configs.is_empty() {
        warn("No snapper configurations found");
        return;
    }

    let config_strs: Vec<&str> = configs.iter().map(|s| s.as_str()).collect();
    let config_idx = match select_with_back("Select configuration", &config_strs, 0) {
        Some(idx) => idx,
        None => return,
    };
    let config = &configs[config_idx];

    let snapshots = get_snapshots(config);
    if snapshots.is_empty() {
        warn("No snapshots found for this configuration");
        return;
    }

    // Display snapshots
    println!("\n{} Available Snapshots for '{}':", icons::disk(), config);
    println!("{}", "=".repeat(60));

    for snap in &snapshots {
        println!(
            "  #{:4} | {} | {} | {}",
            snap.number, snap.date, snap.snapshot_type, snap.description
        );
    }

    // Let user select snapshot
    let snapshot_nums: Vec<String> = snapshots.iter().map(|s| format!("#{}", s.number)).collect();
    let snap_strs: Vec<&str> = snapshot_nums.iter().map(|s| s.as_str()).collect();

    let snap_idx = match select_with_back("Select snapshot to browse", &snap_strs, 0) {
        Some(idx) => idx,
        None => return,
    };

    let selected = &snapshots[snap_idx];
    browse_snapshot_contents(config, selected);
}

/// Browse contents of a specific snapshot
fn browse_snapshot_contents(config: &str, snapshot: &RecoverySnapshot) {
    let snapshot_path = format!("/.snapshots/{}/snapshot", snapshot.number);

    if !Path::new(&snapshot_path).exists() {
        error(&format!("Snapshot path not found: {}", snapshot_path));
        return;
    }

    println!(
        "\n{} Browsing snapshot #{}",
        icons::folder(),
        snapshot.number
    );
    println!("  Path: {}", snapshot_path);
    println!("  Date: {}", snapshot.date);

    loop {
        let options = [
            "List files in snapshot root",
            "Compare file with current system",
            "Restore specific file/directory",
            "Open shell in snapshot (read-only)",
            "Back",
        ];

        match select_with_back("Choose action", &options, 0) {
            Some(0) => {
                let _ = Command::new("ls").args(["-la", &snapshot_path]).status();
            }
            Some(1) => compare_file_with_current(&snapshot_path),
            Some(2) => restore_from_snapshot(&snapshot_path),
            Some(3) => {
                info(&format!("Opening shell in {}", snapshot_path));
                info("This is read-only. Type 'exit' to return.");
                let _ = Command::new("bash").current_dir(&snapshot_path).status();
            }
            _ => break,
        }
    }
}

/// Compare a file from snapshot with current system
fn compare_file_with_current(snapshot_path: &str) {
    let relative_path = match input("Enter file path (e.g., /etc/fstab)", None) {
        Some(p) if !p.is_empty() => p,
        _ => return,
    };

    let snapshot_file = format!("{}{}", snapshot_path, relative_path);
    let current_file = &relative_path;

    if !Path::new(&snapshot_file).exists() {
        error(&format!("File not found in snapshot: {}", relative_path));
        return;
    }

    if !Path::new(current_file).exists() {
        warn(&format!(
            "File does not exist in current system: {}",
            relative_path
        ));
        info("The file may have been deleted since the snapshot.");
        return;
    }

    info("Comparing files (snapshot vs current):");

    // Use diff to compare
    let _ = Command::new("diff")
        .args(["--color=auto", "-u", &snapshot_file, current_file])
        .status();
}

/// Restore a file or directory from snapshot
fn restore_from_snapshot(snapshot_path: &str) {
    let relative_path = match input("Enter path to restore (e.g., /etc/nginx/nginx.conf)", None) {
        Some(p) if !p.is_empty() => p,
        _ => return,
    };

    let source = format!("{}{}", snapshot_path, relative_path);
    let dest = &relative_path;

    if !Path::new(&source).exists() {
        error(&format!("Path not found in snapshot: {}", relative_path));
        return;
    }

    // Check if destination exists
    let dest_exists = Path::new(dest).exists();
    if dest_exists {
        warn(&format!("Current file exists: {}", dest));
        if !confirm("Create backup of current file before restoring?", true) {
            if !confirm_dangerous("Proceed without backup?") {
                return;
            }
        } else {
            // Create backup
            let backup = format!(
                "{}.bak.{}",
                dest,
                chrono::Local::now().format("%Y%m%d_%H%M%S")
            );
            if is_dry_run() {
                info(&format!("[DRY RUN] Would backup {} to {}", dest, backup));
            } else {
                if let Err(e) = sudo_run("cp", &["-a", dest, &backup]) {
                    error(&format!("Failed to create backup: {:?}", e));
                    return;
                }
                success(&format!("Backup created: {}", backup));
            }
        }
    }

    // Perform restore
    if is_dry_run() {
        info(&format!("[DRY RUN] Would restore {} to {}", source, dest));
        return;
    }

    if !confirm(&format!("Restore {} from snapshot?", relative_path), false) {
        return;
    }

    let spinner = Spinner::new(&format!("Restoring {}...", relative_path));

    match sudo_run("cp", &["-a", &source, dest]) {
        Ok(result) if result.success => {
            spinner.finish_with_success(&format!("Restored: {}", relative_path));
        }
        Ok(result) => {
            spinner.finish_with_error(&format!("Failed: {}", result.stderr));
        }
        Err(e) => {
            spinner.finish_with_error(&format!("Error: {}", e));
        }
    }
}

/// Compare two snapshots
fn compare_snapshots() {
    let configs = get_snapper_configs();
    if configs.is_empty() {
        warn("No snapper configurations found");
        return;
    }

    let config_strs: Vec<&str> = configs.iter().map(|s| s.as_str()).collect();
    let config_idx = match select_with_back("Select configuration", &config_strs, 0) {
        Some(idx) => idx,
        None => return,
    };
    let config = &configs[config_idx];

    let snapshots = get_snapshots(config);
    if snapshots.len() < 2 {
        warn("Need at least 2 snapshots to compare");
        return;
    }

    let snapshot_nums: Vec<String> = snapshots
        .iter()
        .map(|s| format!("#{} ({})", s.number, s.date))
        .collect();
    let snap_strs: Vec<&str> = snapshot_nums.iter().map(|s| s.as_str()).collect();

    let first_idx = match select_with_back("Select first (older) snapshot", &snap_strs, 0) {
        Some(idx) => idx,
        None => return,
    };

    let second_idx = match select_with_back(
        "Select second (newer) snapshot",
        &snap_strs,
        first_idx.min(snapshots.len() - 1) + 1,
    ) {
        Some(idx) => idx,
        None => return,
    };

    let first = &snapshots[first_idx];
    let second = &snapshots[second_idx];

    info(&format!(
        "Comparing snapshot #{} vs #{}",
        first.number, second.number
    ));

    // Use snapper diff
    let spinner = Spinner::new("Computing differences...");

    let output = Command::new("snapper")
        .args([
            "-c",
            config,
            "diff",
            &first.number.to_string(),
            &second.number.to_string(),
        ])
        .output();

    spinner.finish();

    match output {
        Ok(o) if o.status.success() => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            if stdout.is_empty() {
                info("No differences found between snapshots");
            } else {
                println!(
                    "\n{} Changes between #{} and #{}:",
                    icons::info(),
                    first.number,
                    second.number
                );
                println!("{}", stdout);
            }
        }
        Ok(o) => {
            error(&format!(
                "Comparison failed: {}",
                String::from_utf8_lossy(&o.stderr)
            ));
        }
        Err(e) => {
            error(&format!("Failed to run snapper: {}", e));
        }
    }
}

/// System rollback wizard
fn system_rollback_wizard() {
    header("System Rollback Wizard");

    warn("WARNING: System rollback will revert your root filesystem to a previous state.");
    warn("This is a destructive operation that cannot be easily undone.");

    if !confirm_dangerous("Do you understand the risks and want to proceed?") {
        return;
    }

    let configs = get_snapper_configs();
    let root_config = configs
        .iter()
        .find(|c| *c == "root")
        .or_else(|| configs.first());

    let config = match root_config {
        Some(c) => c,
        None => {
            error("No root configuration found");
            return;
        }
    };

    let snapshots = get_snapshots(config);
    if snapshots.is_empty() {
        warn("No snapshots available for rollback");
        return;
    }

    // Show snapshots
    println!("\n{} Available snapshots:", icons::disk());
    for snap in &snapshots {
        println!(
            "  #{:4} | {} | {} | {}",
            snap.number, snap.date, snap.snapshot_type, snap.description
        );
    }

    // Select snapshot
    let snapshot_nums: Vec<String> = snapshots
        .iter()
        .map(|s| format!("#{} - {}", s.number, s.description))
        .collect();
    let snap_strs: Vec<&str> = snapshot_nums.iter().map(|s| s.as_str()).collect();

    let snap_idx = match select_with_back("Select snapshot to rollback to", &snap_strs, 0) {
        Some(idx) => idx,
        None => return,
    };

    let selected = &snapshots[snap_idx];

    // Final confirmation
    println!("\n{} Rollback Summary:", icons::warn());
    println!("  Target snapshot: #{}", selected.number);
    println!("  Snapshot date: {}", selected.date);
    println!("  Description: {}", selected.description);
    println!();

    if is_dry_run() {
        info("[DRY RUN] Would execute rollback commands:");
        info(&format!(
            "  snapper -c {} rollback {}",
            config, selected.number
        ));
        return;
    }

    if !confirm_dangerous(&format!(
        "FINAL CONFIRMATION: Rollback to snapshot #{}?",
        selected.number
    )) {
        info("Rollback cancelled");
        return;
    }

    // Perform rollback
    let spinner = Spinner::new("Performing rollback...");

    match sudo_run(
        "snapper",
        &["-c", config, "rollback", &selected.number.to_string()],
    ) {
        Ok(result) if result.success => {
            spinner.finish_with_success("Rollback complete!");
            warn("A reboot is required to complete the rollback.");
            if confirm("Reboot now?", false) {
                let _ = sudo_run("systemctl", &["reboot"]);
            }
        }
        Ok(result) => {
            spinner.finish_with_error(&format!("Rollback failed: {}", result.stderr));
        }
        Err(e) => {
            spinner.finish_with_error(&format!("Error: {}", e));
        }
    }
}

/// Verify snapshot health
fn verify_snapshot_health() {
    info("Checking snapshot health...");

    let spinner = Spinner::new("Scanning snapshots...");

    let configs = get_snapper_configs();
    let mut total_snapshots = 0;
    let mut issues = Vec::new();

    for config in &configs {
        let snapshots = get_snapshots(config);
        total_snapshots += snapshots.len();

        for snap in &snapshots {
            let snap_path = format!("/.snapshots/{}/snapshot", snap.number);
            if !Path::new(&snap_path).exists() {
                issues.push(format!(
                    "{}: Snapshot #{} path missing",
                    config, snap.number
                ));
            }
        }
    }

    spinner.finish();

    println!("\n{} Snapshot Health Report:", icons::status());
    println!("  Configurations: {}", configs.len());
    println!("  Total snapshots: {}", total_snapshots);

    if issues.is_empty() {
        success("All snapshots appear healthy");
    } else {
        warn(&format!("Found {} issues:", issues.len()));
        for issue in &issues {
            println!("  {} {}", icons::warn(), issue);
        }
    }

    // Check btrfs filesystem health
    println!("\n{} Filesystem Health:", icons::disk());
    let _ = Command::new("btrfs")
        .args(["device", "stats", "/"])
        .status();
}

/// Emergency recovery tools
fn emergency_recovery_tools() {
    loop {
        header("Emergency Recovery Tools");

        let options = [
            "Check/Repair Btrfs Filesystem",
            "Mount Snapshot Read-Write (Temporary)",
            "Export Snapshot to Archive",
            "Recovery Shell",
            "View Btrfs Device Stats",
            "Clear Snapshot Lock Files",
            "Back",
        ];

        match select_with_back("Choose tool", &options, 0) {
            Some(0) => check_repair_filesystem(),
            Some(1) => mount_snapshot_rw(),
            Some(2) => export_snapshot(),
            Some(3) => {
                info("Starting recovery shell with root privileges...");
                let _ = sudo_run("bash", &[]);
            }
            Some(4) => {
                let _ = sudo_run("btrfs", &["device", "stats", "/"]);
            }
            Some(5) => clear_lock_files(),
            _ => break,
        }
    }
}

fn check_repair_filesystem() {
    warn("Filesystem check should be done on unmounted filesystem or in rescue mode.");
    warn("Running scrub instead for mounted filesystem...");

    if !confirm("Run btrfs scrub on /?", true) {
        return;
    }

    if is_dry_run() {
        info("[DRY RUN] Would run: btrfs scrub start /");
        return;
    }

    let spinner = Spinner::new("Running btrfs scrub...");
    let _ = sudo_run("btrfs", &["scrub", "start", "-B", "/"]);
    spinner.finish();

    info("Scrub status:");
    let _ = sudo_run("btrfs", &["scrub", "status", "/"]);
}

fn mount_snapshot_rw() {
    warn("Mounting snapshots read-write can corrupt data if not careful!");

    let number = match input("Enter snapshot number", None) {
        Some(n) if !n.is_empty() => n,
        _ => return,
    };

    let mount_point = match input("Enter mount point", Some("/mnt/snapshot_rw")) {
        Some(m) => m,
        _ => return,
    };

    let snap_path = format!("/.snapshots/{}/snapshot", number);
    if !Path::new(&snap_path).exists() {
        error("Snapshot not found");
        return;
    }

    if is_dry_run() {
        info(&format!(
            "[DRY RUN] Would mount {} at {}",
            snap_path, mount_point
        ));
        return;
    }

    if !confirm_dangerous("This will make the snapshot writable. Proceed?") {
        return;
    }

    let _ = sudo_run("mkdir", &["-p", &mount_point]);
    match sudo_run("mount", &["--bind", &snap_path, &mount_point]) {
        Ok(r) if r.success => {
            success(&format!("Snapshot mounted at {}", mount_point));
            warn("Remember to unmount when done!");
        }
        _ => error("Failed to mount snapshot"),
    }
}

fn export_snapshot() {
    let number = match input("Enter snapshot number", None) {
        Some(n) if !n.is_empty() => n,
        _ => return,
    };

    let output_file = match input(
        "Enter output file path",
        Some(&format!("/tmp/snapshot_{}.tar.gz", number)),
    ) {
        Some(f) => f,
        _ => return,
    };

    let snap_path = format!("/.snapshots/{}/snapshot", number);
    if !Path::new(&snap_path).exists() {
        error("Snapshot not found");
        return;
    }

    if is_dry_run() {
        info(&format!(
            "[DRY RUN] Would export {} to {}",
            snap_path, output_file
        ));
        return;
    }

    if !confirm(
        &format!("Export snapshot #{} to {}?", number, output_file),
        true,
    ) {
        return;
    }

    let spinner = Spinner::new("Exporting snapshot (this may take a while)...");

    let result = sudo_shell(&format!("tar -czf {} -C {} .", output_file, snap_path));

    match result {
        Ok(r) if r.success => {
            spinner.finish_with_success(&format!("Exported to {}", output_file));
        }
        _ => {
            spinner.finish_with_error("Export failed");
        }
    }
}

fn clear_lock_files() {
    warn("This will clear snapper lock files. Only do this if snapshots are stuck.");

    if !confirm_dangerous("Clear all snapper lock files?") {
        return;
    }

    if is_dry_run() {
        info("[DRY RUN] Would remove /run/snapper*.lock");
        return;
    }

    let _ = sudo_shell("rm -f /run/snapper*.lock");
    success("Lock files cleared");
}

fn view_recovery_history() {
    info("Recent recovery operations (from journal):");
    let _ = Command::new("journalctl")
        .args(["-u", "snapper*", "-n", "50", "--no-pager"])
        .status();
}

/// Get snapper configurations
fn get_snapper_configs() -> Vec<String> {
    let output = Command::new("snapper").args(["list-configs"]).output();

    match output {
        Ok(o) if o.status.success() => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout
                .lines()
                .skip(2) // Skip header
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split('|').collect();
                    if !parts.is_empty() {
                        Some(parts[0].trim().to_string())
                    } else {
                        None
                    }
                })
                .filter(|s| !s.is_empty())
                .collect()
        }
        _ => Vec::new(),
    }
}

/// Get snapshots for a configuration
fn get_snapshots(config: &str) -> Vec<RecoverySnapshot> {
    let output = Command::new("snapper")
        .args(["-c", config, "list"])
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout
                .lines()
                .skip(3) // Skip headers
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
                    if parts.len() >= 6 {
                        let number = parts[0].trim().parse().ok()?;
                        Some(RecoverySnapshot {
                            number,
                            date: parts[2].to_string(),
                            description: parts[5].to_string(),
                            snapshot_type: parts[1].to_string(),
                            cleanup: parts[4].to_string(),
                            path: format!("/.snapshots/{}/snapshot", number),
                        })
                    } else {
                        None
                    }
                })
                .collect()
        }
        _ => Vec::new(),
    }
}
