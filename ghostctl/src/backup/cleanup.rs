pub fn cleanup_old_backups() {
    println!("🧹 Cleanup Old Backups");
    let config_path = dirs::config_dir().unwrap().join("ghostctl/restic.env");
    if config_path.exists() {
        let _ = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!(
                "source {} && restic forget --prune --keep-daily 7 --keep-weekly 4",
                config_path.display()
            ))
            .status();
    } else {
        println!("❌ No backup configuration found");
    }
}

pub fn run() {
    println!("Cleaning up restic backups...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("restic forget --prune --keep-last 7")
        .status();
    match status {
        Ok(s) if s.success() => println!("Cleanup completed successfully."),
        _ => println!("Cleanup failed."),
    }
}
