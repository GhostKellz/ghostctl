pub fn list_snapshots() {
    println!("- Listing Btrfs snapshots using `btrfs subvolume list`...");
    // Later: use `Command` to parse + print output
}

pub fn create_snapshot(subvolume: &str, name: &str) {
    println!("- Creating snapshot: {}", name);
    println!("sudo btrfs subvolume snapshot {} /@snapshots/{}", subvolume, name);
}
