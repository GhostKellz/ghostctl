pub fn tune() {
    println!("ghostctl :: Arch System Performance Tuning");
    // Apply sysctl tweaks
    let sysctl_conf = "/etc/sysctl.d/99-ghostctl.conf";
    let tweaks = "vm.swappiness=10\nvm.vfs_cache_pressure=50\n";
    println!("Applying sysctl tweaks to {}...", sysctl_conf);
    if let Ok(mut file) = std::fs::File::create(sysctl_conf) {
        use std::io::Write;
        if file.write_all(tweaks.as_bytes()).is_ok() {
            println!("Sysctl tweaks applied.");
        } else {
            println!("Failed to write sysctl tweaks.");
        }
    } else {
        println!("Failed to open sysctl conf for writing.");
    }
    // Clean pacman cache
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo paccache -r")
        .status();
    match status {
        Ok(s) if s.success() => println!("Pacman cache cleaned."),
        _ => println!("Failed to clean pacman cache."),
    }
    // Remove orphaned packages
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -Rns $(pacman -Qtdq) --noconfirm")
        .status();
    match status {
        Ok(s) if s.success() => println!("Orphaned packages removed."),
        _ => println!("No orphaned packages to remove or failed to clean up."),
    }
    // Show boot performance
    let status = std::process::Command::new("systemd-analyze").status();
    match status {
        Ok(s) if s.success() => println!("systemd-analyze output above."),
        _ => println!("Failed to run systemd-analyze."),
    }
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
