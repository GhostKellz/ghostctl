pub mod chroot;
pub mod cleanup;
pub mod restore;
pub mod schedule;
pub mod setup;
pub mod verify;

use crate::btrfs;
use dialoguer::{Select, theme::ColorfulTheme};

pub fn menu() {
    println!(
        "Backup menu: Please select a backup subcommand (run, schedule, verify, cleanup, restore) from the CLI or TUI."
    );
}

#[allow(dead_code)]
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
        .unwrap()
    {
        0 => setup::restic_restore(),
        1 => {
            use dialoguer::Input;
            let name: String = Input::new()
                .with_prompt("Snapshot name to restore")
                .interact_text()
                .unwrap();
            let target: String = Input::new()
                .with_prompt("Restore target (mountpoint or subvolume)")
                .interact_text()
                .unwrap();
            btrfs::restore_snapshot(&name, &target)
        }
        2 => chroot::enter_chroot(),
        _ => (),
    }
}
