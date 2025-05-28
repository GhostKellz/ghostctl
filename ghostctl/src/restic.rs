pub fn setup() {
    println!("ghostctl :: Restic Setup");
    println!("- Initializing restic repository if needed");
    println!("- Prompt for backup source/destination");
    println!("- Generate systemd service and timer for scheduled backups");
    println!("- Enable/disable and check status of backup timer");
}
