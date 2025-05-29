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
