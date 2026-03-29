use dialoguer::Input;

/// Restore a Btrfs snapshot to a target subvolume (e.g. / or /home)
pub fn restore_snapshot() {
    let snapshot: String = match Input::new()
        .with_prompt("Snapshot name (e.g. 2024-06-01-@)")
        .interact_text()
    {
        Ok(s) => s,
        Err(_) => {
            println!("Failed to read snapshot name.");
            return;
        }
    };
    let target: String = match Input::new()
        .with_prompt("Restore target (e.g. / or /home)")
        .interact_text()
    {
        Ok(t) => t,
        Err(_) => {
            println!("Failed to read restore target.");
            return;
        }
    };
    let source = format!("/@snapshots/{}", snapshot);
    println!("Restoring snapshot '{}' to '{}'...", snapshot, target);
    let status = std::process::Command::new("sudo")
        .args(["btrfs", "subvolume", "snapshot", &source, &target])
        .status();
    match status {
        Ok(s) if s.success() => println!("Snapshot '{}' restored to '{}'.", snapshot, target),
        _ => println!("Failed to restore snapshot."),
    }
}
