use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::process::Command;

/// Pure restic CLI wrapper functions
/// For automated workflows, use src/backup/
/// For emergency recovery, use src/restore/
pub fn setup() {
    println!("🔧 Restic CLI Tools");
    println!("===================");

    let options = [
        "🏗️  Initialize repository",
        "💾 Create backup",
        "📋 List snapshots",
        "🔄 Restore from snapshot",
        "🧹 Forget snapshots",
        "🔍 Check repository",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Restic CLI Tools")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => restic_init_interactive(),
        1 => restic_backup_interactive(),
        2 => restic_list_interactive(),
        3 => restic_restore_interactive(),
        4 => restic_forget_interactive(),
        5 => restic_check_interactive(),
        _ => return,
    }
}

fn restic_init_interactive() {
    let repo: String = Input::new()
        .with_prompt("Repository path/URL")
        .interact_text()
        .unwrap();

    println!("🏗️  Initializing repository: {}", repo);
    match init_repository(&repo) {
        Ok(_) => println!("✅ Repository initialized successfully"),
        Err(e) => println!("❌ Failed to initialize repository: {}", e),
    }
}

fn restic_backup_interactive() {
    let repo: String = Input::new()
        .with_prompt("Repository path/URL")
        .interact_text()
        .unwrap();

    let paths: String = Input::new()
        .with_prompt("Paths to backup (space-separated)")
        .interact_text()
        .unwrap();

    let path_list: Vec<&str> = paths.split_whitespace().collect();

    println!("💾 Creating backup...");
    match backup(&path_list, &repo) {
        Ok(_) => println!("✅ Backup completed successfully"),
        Err(e) => println!("❌ Backup failed: {}", e),
    }
}

fn restic_list_interactive() {
    let repo: String = Input::new()
        .with_prompt("Repository path/URL")
        .interact_text()
        .unwrap();

    println!("📋 Listing snapshots...");
    match list_snapshots(&repo) {
        Ok(_) => {}
        Err(e) => println!("❌ Failed to list snapshots: {}", e),
    }
}

fn restic_restore_interactive() {
    let repo: String = Input::new()
        .with_prompt("Repository path/URL")
        .interact_text()
        .unwrap();

    let snapshot_id: String = Input::new()
        .with_prompt("Snapshot ID")
        .interact_text()
        .unwrap();

    let target: String = Input::new()
        .with_prompt("Restore target directory")
        .interact_text()
        .unwrap();

    println!("🔄 Restoring snapshot {} to {}...", snapshot_id, target);
    match restore(&snapshot_id, &target, &repo) {
        Ok(_) => println!("✅ Restore completed successfully"),
        Err(e) => println!("❌ Restore failed: {}", e),
    }
}

fn restic_forget_interactive() {
    let repo: String = Input::new()
        .with_prompt("Repository path/URL")
        .interact_text()
        .unwrap();

    let keep_daily: u32 = Input::new()
        .with_prompt("Keep daily snapshots for N days")
        .default(7)
        .interact()
        .unwrap();

    let keep_weekly: u32 = Input::new()
        .with_prompt("Keep weekly snapshots for N weeks")
        .default(4)
        .interact()
        .unwrap();

    let keep_monthly: u32 = Input::new()
        .with_prompt("Keep monthly snapshots for N months")
        .default(6)
        .interact()
        .unwrap();

    println!("🧹 Forgetting old snapshots...");
    match forget_snapshots(&repo, keep_daily, keep_weekly, keep_monthly) {
        Ok(_) => println!("✅ Snapshot cleanup completed"),
        Err(e) => println!("❌ Cleanup failed: {}", e),
    }
}

fn restic_check_interactive() {
    let repo: String = Input::new()
        .with_prompt("Repository path/URL")
        .interact_text()
        .unwrap();

    println!("🔍 Checking repository integrity...");
    match check_repository(&repo) {
        Ok(_) => println!("✅ Repository check completed"),
        Err(e) => println!("❌ Repository check failed: {}", e),
    }
}

pub fn backup(paths: &[&str], repo: &str) -> Result<(), String> {
    let status = Command::new("restic")
        .arg("backup")
        .args(paths)
        .env("RESTIC_REPOSITORY", repo)
        .status()
        .map_err(|e| format!("Failed to run restic backup: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err("Restic backup failed".to_string())
    }
}

pub fn restore(snapshot_id: &str, target: &str, repo: &str) -> Result<(), String> {
    let status = Command::new("restic")
        .arg("restore")
        .arg(snapshot_id)
        .arg("--target")
        .arg(target)
        .env("RESTIC_REPOSITORY", repo)
        .status()
        .map_err(|e| format!("Failed to run restic restore: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err("Restic restore failed".to_string())
    }
}

pub fn list_snapshots(repo: &str) -> Result<(), String> {
    let status = Command::new("restic")
        .arg("snapshots")
        .env("RESTIC_REPOSITORY", repo)
        .status()
        .map_err(|e| format!("Failed to list snapshots: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err("Failed to list snapshots".to_string())
    }
}

pub fn check_repository(repo: &str) -> Result<(), String> {
    let status = Command::new("restic")
        .arg("check")
        .env("RESTIC_REPOSITORY", repo)
        .status()
        .map_err(|e| format!("Failed to check repository: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err("Repository check failed".to_string())
    }
}

pub fn init_repository(repo: &str) -> Result<(), String> {
    let status = Command::new("restic")
        .arg("init")
        .env("RESTIC_REPOSITORY", repo)
        .status()
        .map_err(|e| format!("Failed to initialize repository: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err("Repository initialization failed".to_string())
    }
}

pub fn forget_snapshots(
    repo: &str,
    keep_daily: u32,
    keep_weekly: u32,
    keep_monthly: u32,
) -> Result<(), String> {
    let status = Command::new("restic")
        .arg("forget")
        .arg("--prune")
        .arg("--keep-daily")
        .arg(&keep_daily.to_string())
        .arg("--keep-weekly")
        .arg(&keep_weekly.to_string())
        .arg("--keep-monthly")
        .arg(&keep_monthly.to_string())
        .env("RESTIC_REPOSITORY", repo)
        .status()
        .map_err(|e| format!("Failed to forget snapshots: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err("Failed to forget snapshots".to_string())
    }
}
