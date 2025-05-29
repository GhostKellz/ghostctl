pub mod chroot;
pub mod restorebtrfs;

use dialoguer::{Input, Select, theme::ColorfulTheme};

#[allow(dead_code)]
pub fn run() {
    println!("ghostctl :: Restore Utility");

    println!("Choose a restore mode:");
    println!("1. Restic");
    println!("2. Btrfs Snapshot");
    println!("3. Enter Recovery Chroot");

    // Later: Use dialoguer or args
    crate::restic::setup(); // call the root-level restic module
}

#[allow(dead_code)]
pub fn menu() {
    let opts = ["Rollback Btrfs Snapshot", "Enter Recovery Chroot", "Back"];
    match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Restore Menu")
        .items(&opts)
        .default(0)
        .interact()
        .unwrap()
    {
        0 => {
            let name: String = Input::new()
                .with_prompt("Snapshot name to rollback")
                .interact_text()
                .unwrap();
            let mountpoint: String = Input::new()
                .with_prompt("Mountpoint to restore to")
                .default("/".into())
                .interact_text()
                .unwrap();
            restorebtrfs::rollback(&name, &mountpoint)
        }
        1 => {
            let mountpoint: String = Input::new()
                .with_prompt("Chroot mountpoint (e.g. /mnt)")
                .default("/mnt".into())
                .interact_text()
                .unwrap();
            chroot::enter(&mountpoint)
        }
        _ => (),
    }
}
