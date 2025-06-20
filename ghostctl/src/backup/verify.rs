pub fn verify_backups() {
    println!("✅ Verify Backup Integrity");
    let config_path = dirs::config_dir().unwrap().join("ghostctl/restic.env");
    if config_path.exists() {
        let _ = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!("source {} && restic check", config_path.display()))
            .status();
    } else {
        println!("❌ No backup configuration found");
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Verifying restic repository...");
    println!("restic snapshots");
}

pub fn verify() {
    println!("Verifying restic backup...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("restic check")
        .status();
    match status {
        Ok(s) if s.success() => println!("Backup verified successfully."),
        _ => println!("Backup verification failed."),
    }
}
