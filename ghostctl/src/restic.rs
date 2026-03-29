use dialoguer::{Input, Select, theme::ColorfulTheme};
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

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Restic CLI Tools")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

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
    let Ok(repo) = Input::<String>::new()
        .with_prompt("Repository path/URL")
        .interact_text()
    else {
        return;
    };

    println!("🏗️  Initializing repository: {}", repo);
    match init_repository(&repo) {
        Ok(_) => println!("✅ Repository initialized successfully"),
        Err(e) => println!("❌ Failed to initialize repository: {}", e),
    }
}

fn restic_backup_interactive() {
    let Ok(repo) = Input::<String>::new()
        .with_prompt("Repository path/URL")
        .interact_text()
    else {
        return;
    };

    let Ok(paths) = Input::<String>::new()
        .with_prompt("Paths to backup (space-separated)")
        .interact_text()
    else {
        return;
    };

    let path_list: Vec<&str> = paths.split_whitespace().collect();

    println!("💾 Creating backup...");
    match backup(&path_list, &repo) {
        Ok(_) => println!("✅ Backup completed successfully"),
        Err(e) => println!("❌ Backup failed: {}", e),
    }
}

fn restic_list_interactive() {
    let Ok(repo) = Input::<String>::new()
        .with_prompt("Repository path/URL")
        .interact_text()
    else {
        return;
    };

    println!("📋 Listing snapshots...");
    match list_snapshots(&repo) {
        Ok(_) => {}
        Err(e) => println!("❌ Failed to list snapshots: {}", e),
    }
}

fn restic_restore_interactive() {
    let Ok(repo) = Input::<String>::new()
        .with_prompt("Repository path/URL")
        .interact_text()
    else {
        return;
    };

    let Ok(snapshot_id) = Input::<String>::new()
        .with_prompt("Snapshot ID")
        .interact_text()
    else {
        return;
    };

    let Ok(target) = Input::<String>::new()
        .with_prompt("Restore target directory")
        .interact_text()
    else {
        return;
    };

    println!("🔄 Restoring snapshot {} to {}...", snapshot_id, target);
    match restore(&snapshot_id, &target, &repo) {
        Ok(_) => println!("✅ Restore completed successfully"),
        Err(e) => println!("❌ Restore failed: {}", e),
    }
}

fn restic_forget_interactive() {
    let Ok(repo) = Input::<String>::new()
        .with_prompt("Repository path/URL")
        .interact_text()
    else {
        return;
    };

    let Ok(keep_daily) = Input::<u32>::new()
        .with_prompt("Keep daily snapshots for N days")
        .default(7)
        .interact()
    else {
        return;
    };

    let Ok(keep_weekly) = Input::<u32>::new()
        .with_prompt("Keep weekly snapshots for N weeks")
        .default(4)
        .interact()
    else {
        return;
    };

    let Ok(keep_monthly) = Input::<u32>::new()
        .with_prompt("Keep monthly snapshots for N months")
        .default(6)
        .interact()
    else {
        return;
    };

    println!("🧹 Forgetting old snapshots...");
    match forget_snapshots(&repo, keep_daily, keep_weekly, keep_monthly) {
        Ok(_) => println!("✅ Snapshot cleanup completed"),
        Err(e) => println!("❌ Cleanup failed: {}", e),
    }
}

fn restic_check_interactive() {
    let Ok(repo) = Input::<String>::new()
        .with_prompt("Repository path/URL")
        .interact_text()
    else {
        return;
    };

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
