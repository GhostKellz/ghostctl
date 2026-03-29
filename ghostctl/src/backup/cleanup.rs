pub fn cleanup_old_backups() {
    println!("🧹 Cleanup Old Backups");
    println!("====================");

    // Try to use configured backup first
    let config_path = match dirs::config_dir() {
        Some(dir) => dir.join("ghostctl/restic.env"),
        None => {
            println!("❌ Could not determine config directory. Using environment variables...");
            // Fallback to environment variables
            let status = std::process::Command::new("restic")
                .arg("forget")
                .arg("--prune")
                .arg("--keep-daily")
                .arg("7")
                .arg("--keep-weekly")
                .arg("4")
                .arg("--keep-monthly")
                .arg("12")
                .status();

            match status {
                Ok(s) if s.success() => println!("✅ Cleanup completed successfully"),
                _ => {
                    println!("❌ No backup configuration found. Run 'ghostctl backup setup' first")
                }
            }
            return;
        }
    };
    if config_path.exists() {
        println!("📋 Using configured backup settings...");
        let status = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!(
                "source {} && restic forget --prune --keep-daily 7 --keep-weekly 4 --keep-monthly 12",
                config_path.display()
            ))
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Cleanup completed successfully"),
            _ => println!("❌ Cleanup failed - check your restic configuration"),
        }
    } else {
        // Fallback to environment variables
        println!("📋 Using environment variables for restic...");
        let status = std::process::Command::new("restic")
            .arg("forget")
            .arg("--prune")
            .arg("--keep-daily")
            .arg("7")
            .arg("--keep-weekly")
            .arg("4")
            .arg("--keep-monthly")
            .arg("12")
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Cleanup completed successfully"),
            _ => println!("❌ No backup configuration found. Run 'ghostctl backup setup' first"),
        }
    }
}
