pub fn run() {
    println!("Restoring restic backup...");
    // Example: restore latest snapshot to /tmp/restore
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("restic restore latest --target /tmp/restore")
        .status();
    match status {
        Ok(s) if s.success() => println!("Restore completed successfully."),
        _ => println!("Restore failed."),
    }
}