pub mod setup;
pub mod schedule;
pub mod verify;
pub mod cleanup;
pub mod chroot;

use crate::btrfs;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn menu() {
    println!("ghostctl :: Backup Manager");
    let opts = [
        "Setup Backup",
        "Verify Backups",
        "Schedule Backups",
        "Cleanup Old Backups",
        "Back",
    ];
    match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Backup Menu")
        .items(&opts)
        .default(0)
        .interact()
        .unwrap() {
        0 => setup::setup(),
        1 => verify::verify(),
        2 => schedule::schedule(),
        3 => cleanup::cleanup(),
        _ => (),
    }
}

pub fn restore_menu() {
    println!("ghostctl :: Restore Utility");
    let opts = [
        "Restic Restore",
        "Btrfs Snapshot Restore",
        "Enter Recovery Chroot",
        "Back",
    ];
    match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Restore Menu")
        .items(&opts)
        .default(0)
        .interact()
        .unwrap() {
        0 => setup::restic_restore(),
        1 => btrfs::restore_snapshot(),
        2 => chroot::enter_chroot(),
        _ => (),
    }
}
