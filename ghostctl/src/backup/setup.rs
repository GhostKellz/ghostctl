pub fn generate() {
    println!("- Creating ~/.config/ghostctl/restic.env");
    println!("- Creating systemd services: restic-backup.service / .timer");
    println!("- Creating Snapper config for / and /home");

    // future: write to ~/.config/ghostctl/restic.env
    // and ~/.config/systemd/user/restic-backup.{service,timer}
}

pub fn setup() {
    println!("ghostctl :: Setup Backup (stub)");
}

pub fn restic_restore() {
    println!("ghostctl :: Restic Restore (stub)");
}

