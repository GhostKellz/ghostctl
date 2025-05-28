pub fn rollback(snapshot: &str, mountpoint: &str) {
    println!("Restoring Btrfs snapshot '{}'", snapshot);
    println!("- Deleting current subvolume: {}", mountpoint);
    println!("- Restoring snapshot with: btrfs subvolume snapshot");

    // placeholder
}
