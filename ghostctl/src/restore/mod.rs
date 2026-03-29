pub mod system;

use dialoguer::{Select, theme::ColorfulTheme};

pub fn restore_menu() {
    println!("🚨 System Recovery & Restore");
    println!("============================");

    let options = [
        "💾 Restore from Restic Backup",
        "📸 Rollback Btrfs Snapshot",
        "🛠️  Enter Recovery Chroot",
        "🔄 Full System Recovery",
        "📋 List Available Backups",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Recovery Options")
        .items(&options)
        .default(0)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    match choice {
        0 => system::restore_from_restic(),
        1 => system::rollback_btrfs_snapshot(),
        2 => system::enter_recovery_chroot(),
        3 => system::full_system_recovery(),
        4 => system::list_available_backups(),
        _ => return,
    }
}
