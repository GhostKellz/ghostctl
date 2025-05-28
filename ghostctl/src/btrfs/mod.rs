pub mod snapshot;
pub mod restore;

pub fn run() {
    println!("ghostctl :: Btrfs Manager");

    snapshot::list_snapshots();
}
