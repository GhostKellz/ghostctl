pub fn fix_pacman() {
    println!("ghostctl :: Arch Pacman Fix");

    // Later: run commands using `duct` or `std::process::Command`
    println!("- sudo pacman -Syyu");
    println!("- sudo pacman -S archlinux-keyring");
    println!("- Optionally refresh mirrorlist with reflector");
}

pub fn fix() {
    println!("ghostctl :: Arch System Fix");
    // Remove pacman lock if present
    let _ = std::process::Command::new("sudo")
        .args(["rm", "/var/lib/pacman/db.lck"])
        .status();
    // Full sync/upgrade
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -Syyu --noconfirm")
        .status();
    match status {
        Ok(s) if s.success() => println!("System fully upgraded."),
        _ => println!("Failed to upgrade system."),
    }
    // Refresh keyring
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -S --noconfirm archlinux-keyring")
        .status();
    match status {
        Ok(s) if s.success() => println!("Keyring refreshed."),
        _ => println!("Failed to refresh keyring."),
    }
    // Optionally refresh mirrorlist
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("command -v reflector && sudo reflector --latest 20 --sort rate --save /etc/pacman.d/mirrorlist || echo 'reflector not installed'")
        .status();
    match status {
        Ok(s) if s.success() => println!("Mirrorlist refreshed (if reflector installed)."),
        _ => println!("Could not refresh mirrorlist (reflector missing?)."),
    }
}

pub fn orphans() {
    println!("ghostctl :: Arch Orphan Cleanup");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -Rns $(pacman -Qtdq) --noconfirm")
        .status();
    match status {
        Ok(s) if s.success() => println!("Orphaned packages removed."),
        _ => println!("No orphaned packages to remove or failed to clean up."),
    }
}

pub fn optimize() {
    println!("ghostctl :: Arch System Optimization");
    // Enable zram
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo systemctl enable --now systemd-zram-setup@zram0.service")
        .status();
    match status {
        Ok(s) if s.success() => println!("zram enabled."),
        _ => println!("Failed to enable zram."),
    }
    // Enable zswap (if kernel supports)
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("echo 1 | sudo tee /sys/module/zswap/parameters/enabled")
        .status();
    match status {
        Ok(s) if s.success() => println!("zswap enabled (if supported)."),
        _ => println!("Failed to enable zswap (may not be supported)."),
    }
}
