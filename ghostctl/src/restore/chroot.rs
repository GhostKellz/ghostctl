pub fn enter(mountpoint: &str) {
    use std::process::Command;
    use dialoguer::Confirm;
    println!("Setting up chroot environment at '{}'...", mountpoint);
    if !Confirm::new().with_prompt(format!("Proceed to mount and chroot into '{}'?", mountpoint)).default(false).interact().unwrap() {
        println!("Aborted chroot setup.");
        return;
    }
    // Mount root if not already mounted
    let _ = Command::new("sudo").args(["mount", mountpoint, mountpoint]).status();
    // Bind-mount system dirs
    for dir in ["/dev", "/proc", "/sys", "/run"] {
        let target = format!("{}/{}", mountpoint, dir.trim_start_matches('/'));
        let _ = Command::new("sudo").args(["mount", "--bind", dir, &target]).status();
    }
    // Optionally mount /boot and /efi if present
    for dir in ["/boot", "/efi"] {
        let target = format!("{}/{}", mountpoint, dir.trim_start_matches('/'));
        if std::path::Path::new(&target).exists() {
            let _ = Command::new("sudo").args(["mount", "--bind", dir, &target]).status();
        }
    }
    println!("Launching arch-chroot...");
    let _ = Command::new("sudo").args(["arch-chroot", mountpoint]).status();
    println!("Chroot session ended. Unmounting...");
    // Unmount in reverse order
    for dir in ["/efi", "/boot", "/run", "/sys", "/proc", "/dev"] {
        let target = format!("{}/{}", mountpoint, dir.trim_start_matches('/'));
        let _ = Command::new("sudo").args(["umount", "-l", &target]).status();
    }
    let _ = Command::new("sudo").args(["umount", "-l", mountpoint]).status();
    println!("Chroot environment cleaned up.");
}
