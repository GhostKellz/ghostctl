pub mod setup;
pub mod schedule;
pub mod verify;
pub mod cleanup;

pub fn menu() {
    println!("ghostctl :: Backup Manager");

    setup::generate();
}
