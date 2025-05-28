pub mod snapshot;
pub mod restore;

pub fn run() {
    println!("ghostctl :: Btrfs Snapshot Manager");
    println!("- Listing available Btrfs snapshots...");
    // TODO: Integrate with btrfs or snapper CLI to list snapshots
    println!("- To create a new snapshot: sudo snapper create --description 'ghostctl snapshot'");
    println!("- To restore a snapshot: sudo snapper rollback <snapshot>");
    println!("- To configure snapper: sudo snapper -c <config> create-config <mountpoint>");
}

pub fn setup_snapper() {
    println!("ghostctl :: Snapper Setup");
    println!("- Installing snapper if not present");
    println!("- Creating snapper config for / and /home");
    println!("- Setting up default snapshot schedule");
}

pub fn create_snapshot() {
    println!("Creating a new Btrfs snapshot...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo btrfs subvolume snapshot / /@_ghostctl_$(date +%s)")
        .status();
    match status {
        Ok(s) if s.success() => println!("Snapshot created."),
        _ => println!("Failed to create snapshot."),
    }
}

pub fn delete_snapshot() {
    println!("Deleting a Btrfs snapshot (stub, prompt for name in future)");
}

pub fn restore_snapshot() {
    println!("ghostctl :: Btrfs Snapshot Restore (stub)");
}

pub fn scrub() {
    println!("Running Btrfs scrub...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo btrfs scrub start /")
        .status();
    match status {
        Ok(s) if s.success() => println!("Scrub started."),
        _ => println!("Failed to start scrub."),
    }
}

pub fn balance() {
    println!("Running Btrfs balance...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo btrfs balance start /")
        .status();
    match status {
        Ok(s) if s.success() => println!("Balance started."),
        _ => println!("Failed to start balance."),
    }
}
