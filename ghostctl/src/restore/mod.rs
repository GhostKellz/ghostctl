pub mod chroot;
pub mod btrfs;

pub fn run() {
    println!("ghostctl :: Restore Utility");

    println!("Choose a restore mode:");
    println!("1. Restic");
    println!("2. Btrfs Snapshot");
    println!("3. Enter Recovery Chroot");

    // Later: Use dialoguer or args
    restic::start(); // placeholder
}
