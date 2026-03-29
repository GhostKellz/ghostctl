#[allow(dead_code)]
pub fn rollback(snapshot: &str, mountpoint: &str) {
    use dialoguer::Confirm;
    println!(
        "Restoring Btrfs snapshot '{}' to '{}' (rollback)...",
        snapshot, mountpoint
    );
    let confirmed = match Confirm::new()
        .with_prompt(format!(
            "This will DELETE the current subvolume at '{}' and replace it with snapshot '{}'. Continue?",
            mountpoint, snapshot
        ))
        .default(false)
        .interact()
    {
        Ok(c) => c,
        Err(_) => {
            println!("Aborted rollback.");
            return;
        }
    };
    if !confirmed {
        println!("Aborted rollback.");
        return;
    }
    // Delete current subvolume
    let status = std::process::Command::new("sudo")
        .args(["btrfs", "subvolume", "delete", mountpoint])
        .status();
    match status {
        Ok(s) if s.success() => println!("Deleted subvolume: {}", mountpoint),
        _ => {
            println!(
                "Failed to delete subvolume: {}. Aborting restore.",
                mountpoint
            );
            return;
        }
    }
    // Restore snapshot to mountpoint
    let source = format!("/@snapshots/{}", snapshot);
    let status = std::process::Command::new("sudo")
        .args(["btrfs", "subvolume", "snapshot", &source, mountpoint])
        .status();
    match status {
        Ok(s) if s.success() => println!("Snapshot '{}' restored to '{}'.", snapshot, mountpoint),
        _ => println!("Failed to restore snapshot."),
    }
}
