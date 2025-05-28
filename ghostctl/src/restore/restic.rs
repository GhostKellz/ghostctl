pub fn start() {
    println!("Restoring from Restic repository...");

    println!("- Listing snapshots...");
    println!("restic snapshots");

    println!("- Choose snapshot ID...");
    println!("restic restore <id> --target /mnt");

    // Later: prompt user, run commands
}
