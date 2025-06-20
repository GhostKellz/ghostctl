use std::process::Command;

/// Pure restic CLI wrapper functions
/// For automated workflows, use src/backup/
/// For emergency recovery, use src/restore/

pub fn setup() {
    println!("🔧 Restic CLI Setup");
    println!("===================");
    println!("For full backup setup, use the backup management menu");
    println!("This is just the raw restic CLI wrapper");
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
