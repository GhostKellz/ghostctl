pub fn restore_snapshot(name: &str, mountpoint: &str) {
    use dialoguer::Confirm;
    println!("Restoring snapshot '{}' to '{}'...", name, mountpoint);
    if Confirm::new().with_prompt(format!("This will overwrite '{}'. Continue?", mountpoint)).default(false).interact().unwrap() {
        let source = format!("/@snapshots/{}", name);
        let status = std::process::Command::new("sudo")
            .args(["btrfs", "subvolume", "snapshot", &source, mountpoint])
            .status();
        match status {
            Ok(s) if s.success() => println!("Snapshot '{}' restored to '{}'.", name, mountpoint),
            _ => println!("Failed to restore snapshot."),
        }
    } else {
        println!("Aborted restore.");
    }
}
