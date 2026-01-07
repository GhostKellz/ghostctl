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
    use dialoguer::Confirm;
    let target = format!("/@snapshots/{}", name);
    if Confirm::new()
        .with_prompt(format!("Delete snapshot '{}'?", name))
        .default(false)
        .interact()
        .unwrap()
    {
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
    use dialoguer::Confirm;
    println!("Restoring snapshot '{}' to '{}'...", name, target);
    if Confirm::new()
        .with_prompt(format!("This will overwrite '{}'. Continue?", target))
        .default(false)
        .interact()
        .unwrap()
    {
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

pub fn snapper_setup() {
    println!("Deploying Snapper base configs for root and home...");
    // Install snapper if not present
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -S --noconfirm snapper")
        .status();
    match status {
        Ok(s) if s.success() => println!("Snapper installed or already present."),
        _ => println!("Failed to install snapper."),
    }
    // Create root config
    let status = std::process::Command::new("sudo")
        .args(["snapper", "-c", "root", "create-config", "/"])
        .status();
    match status {
        Ok(s) if s.success() => println!("Snapper config 'root' created for '/'."),
        _ => println!("Failed to create Snapper config for root."),
    }
    // Create home config
    let status = std::process::Command::new("sudo")
        .args(["snapper", "-c", "home", "create-config", "/home"])
        .status();
    match status {
        Ok(s) if s.success() => println!("Snapper config 'home' created for '/home'."),
        _ => println!("Failed to create Snapper config for home."),
    }
    println!(
        "You may want to edit /etc/snapper/configs/root and /etc/snapper/configs/home for retention and cleanup settings."
    );
}

pub fn snapper_edit(config: &str) {
    use std::process::Command;
    let config_path = format!("/etc/snapper/configs/{}", config);
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    let status = Command::new(&editor).arg(&config_path).status();
    match status {
        Ok(s) if s.success() => println!("Edited Snapper config: {}", config_path),
        _ => println!("Failed to edit Snapper config: {}", config_path),
    }
}

pub fn snapper_list() {
    use std::fs;
    let configs_dir = "/etc/snapper/configs";
    match fs::read_dir(configs_dir) {
        Ok(entries) => {
            println!("Available Snapper configs:");
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    println!("- {}", name);
                }
            }
        }
        Err(_) => println!("No Snapper configs found in {}", configs_dir),
    }
}

pub fn scrub(mountpoint: &str) {
    println!("Starting btrfs scrub on {}...", mountpoint);
    let status = std::process::Command::new("sudo")
        .args(["btrfs", "scrub", "start", mountpoint])
        .status();
    match status {
        Ok(s) if s.success() => println!("Scrub started on {}.", mountpoint),
        _ => println!("Failed to start scrub on {}.", mountpoint),
    }
}

pub fn balance(mountpoint: &str) {
    println!("Starting btrfs balance on {}...", mountpoint);
    let status = std::process::Command::new("sudo")
        .args(["btrfs", "balance", "start", mountpoint])
        .status();
    match status {
        Ok(s) if s.success() => println!("Balance started on {}.", mountpoint),
        _ => println!("Failed to start balance on {}.", mountpoint),
    }
}

pub fn snapper_menu() {
    use dialoguer::{Input, Select, theme::ColorfulTheme};
    let opts = ["Deploy Base Config", "Edit Config", "List Configs", "Back"];
    match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Snapper Menu")
        .items(&opts)
        .default(0)
        .interact()
        .unwrap()
    {
        0 => snapper_setup(),
        1 => {
            let config: String = Input::new()
                .with_prompt("Config to edit")
                .default("root".into())
                .interact_text()
                .unwrap();
            snapper_edit(&config)
        }
        2 => snapper_list(),
        _ => return,
    }
}

// Enhanced snapshot cleanup functions
pub fn emergency_cleanup_all_snapshots() {
    println!("🚨 EMERGENCY: Removing ALL BTRFS snapshots to free disk space");
    println!("⚠️  This is irreversible and will delete all system snapshots!");

    if !dialoguer::Confirm::new()
        .with_prompt("Are you absolutely sure? This cannot be undone!")
        .default(false)
        .interact()
        .unwrap()
    {
        println!("❌ Emergency cleanup aborted");
        return;
    }

    // Get current disk usage
    check_disk_space();

    println!("🧹 Removing all BTRFS snapshots...");

    // Method 1: Snapper cleanup (safer)
    println!("📋 Attempting snapper cleanup first...");
    if let Ok(output) = std::process::Command::new("snapper")
        .args(["-c", "root", "list", "--columns", "number"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let snapshots: Vec<&str> = output_str
            .lines()
            .skip(2) // Skip headers
            .filter(|line| !line.trim().is_empty())
            .collect();

        if !snapshots.is_empty() {
            let snapshot_numbers: String = snapshots.join(" ");
            let status = std::process::Command::new("sudo")
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
    let status = std::process::Command::new("sudo")
        .args(["bash", "-c", "find /.snapshots -maxdepth 2 -name 'snapshot' -type d | while read snap; do btrfs subvolume delete \"$snap\" 2>/dev/null || true; done"])
        .status();

    match status {
        Ok(s) if s.success() => println!("🗑️  Direct cleanup completed"),
        _ => println!("⚠️  Some snapshots may require manual cleanup"),
    }

    // Method 3: Remove snapshot directories
    println!("📂 Cleaning up snapshot directories...");
    let _ = std::process::Command::new("sudo")
        .args(["rm", "-rf", "/.snapshots/*"])
        .status();

    println!("🔄 Checking remaining disk space...");
    check_disk_space();

    println!("✅ Emergency cleanup completed");
    println!("📝 Recommend running 'btrfs filesystem usage /' to verify space recovery");
}

pub fn bulk_cleanup_snapshots() {
    use dialoguer::{Select, theme::ColorfulTheme};

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Cleanup Method")
        .items(&cleanup_options)
        .default(4) // Default to show disk usage
        .interact()
        .unwrap();

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

    let status = std::process::Command::new("sudo")
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

    if !dialoguer::Confirm::new()
        .with_prompt(format!("Delete snapshots {}?", range))
        .default(false)
        .interact()
        .unwrap()
    {
        println!("❌ Range cleanup aborted");
        return;
    }

    let status = std::process::Command::new("sudo")
        .args(["snapper", "-c", "root", "delete", range])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Range cleanup completed"),
        _ => println!("❌ Range cleanup failed"),
    }
}

fn cleanup_by_age_interactive() {
    use dialoguer::Input;
    let days: String = Input::new()
        .with_prompt("Delete snapshots older than how many days?")
        .default("30".to_string())
        .interact_text()
        .unwrap();

    cleanup_snapshots_by_age(&days);
}

fn cleanup_by_range_interactive() {
    use dialoguer::Input;
    let range: String = Input::new()
        .with_prompt("Enter snapshot range (e.g., 1-100)")
        .interact_text()
        .unwrap();

    cleanup_snapshots_by_range(&range);
}

fn cleanup_specific_snapshots() {
    use dialoguer::{Confirm, Input};
    // Get available snapshots
    if let Ok(output) = std::process::Command::new("snapper")
        .args(["-c", "root", "list"])
        .output()
    {
        let snapshot_list = String::from_utf8_lossy(&output.stdout);
        println!("📋 Available snapshots:");
        println!("{}", snapshot_list);
    }

    let snapshots: String = Input::new()
        .with_prompt("Enter snapshot numbers to delete (space-separated, e.g., '184 187 188')")
        .interact_text()
        .unwrap();

    if !snapshots.trim().is_empty()
        && Confirm::new()
            .with_prompt(format!("Delete snapshots: {}?", snapshots))
            .default(false)
            .interact()
            .unwrap()
    {
        println!("🎯 Deleting specific snapshots...");

        let status = std::process::Command::new("sudo")
            .args(["snapper", "-c", "root", "delete"])
            .args(snapshots.split_whitespace())
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Specific snapshot cleanup completed"),
            _ => println!("❌ Specific snapshot cleanup failed"),
        }
    }
}

pub fn check_disk_space() {
    println!("💾 Current Disk Usage:");
    println!("=====================");

    // Show overall filesystem usage
    let _ = std::process::Command::new("df").args(["-h", "/"]).status();

    // Show BTRFS specific usage
    println!("\n🗂️  BTRFS Filesystem Usage:");
    let _ = std::process::Command::new("sudo")
        .args(["btrfs", "filesystem", "usage", "/"])
        .status();

    // Show snapshot directory size (with proper error handling)
    println!("\n📸 Snapshot Directory Usage:");
    let output = std::process::Command::new("sudo")
        .args(["du", "-sh", "/.snapshots"])
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
    if let Ok(output) = std::process::Command::new("sudo")
        .args([
            "find",
            "/.snapshots",
            "-maxdepth",
            "1",
            "-type",
            "d",
            "-name",
            "[0-9]*",
        ])
        .output()
    {
        let count = String::from_utf8_lossy(&output.stdout).lines().count();
        println!("📊 Total snapshots: {}", count);
    }

    // Warn if disk usage is high
    if let Ok(output) = std::process::Command::new("df")
        .args(["/", "--output=pcent"])
        .output()
    {
        let usage_str = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = usage_str.lines().nth(1)
            && let Ok(usage) = line.trim().trim_end_matches('%').parse::<i32>()
        {
            if usage > 90 {
                println!(
                    "\n⚠️  WARNING: Disk usage is {}% - consider emergency cleanup!",
                    usage
                );
            } else if usage > 80 {
                println!("\n⚠️  CAUTION: Disk usage is {}% - monitor closely", usage);
            }
        }
    }
}
