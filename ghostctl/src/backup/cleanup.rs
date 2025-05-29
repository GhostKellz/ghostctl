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
