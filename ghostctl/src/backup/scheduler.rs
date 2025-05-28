pub fn enable_timer() {
    println!("Enabling user timer for restic...");
    println!("systemctl --user enable --now restic-backup.timer");
}
