#[allow(dead_code)]
pub fn run() {
    println!("Running restic backup...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("restic backup /etc /home")
        .status();
    match status {
        Ok(s) if s.success() => println!("Backup completed successfully."),
        _ => println!("Backup failed."),
    }
}

#[allow(dead_code)]
pub fn schedule() {
    println!("Scheduling restic backup (stub, implement systemd timer or cron)");
}
