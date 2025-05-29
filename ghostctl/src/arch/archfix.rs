use dialoguer::{Select, theme::ColorfulTheme};

#[allow(dead_code)]
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

pub fn mirrors() {
    println!("ghostctl :: Arch Mirror Optimization");
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

pub fn pkgfix() {
    println!("ghostctl :: Arch PKGBUILD/Build Environment Fix");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo rm -rf /tmp/yaourt-tmp-* /tmp/pamac-build-* /tmp/makepkg-* && echo 'Build environment cleaned.'")
        .status();
    match status {
        Ok(s) if s.success() => println!("Build environment cleaned."),
        _ => println!("Failed to clean build environment."),
    }
}

#[allow(dead_code)]
pub fn keyring() {
    println!("ghostctl :: Arch Keyring Refresh");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -S --noconfirm archlinux-keyring")
        .status();
    match status {
        Ok(s) if s.success() => println!("Keyring refreshed."),
        _ => println!("Failed to refresh keyring."),
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

#[allow(dead_code)]
pub fn full() {
    println!("ghostctl :: Full Arch Maintenance");
    fix();
    keyring();
    mirrors();
    orphans();
    optimize();
}

#[allow(dead_code)]
pub fn tui_menu() {
    let opts = [
        "System Fix (Upgrade, Keyring, Mirrors)",
        "Keyring Refresh",
        "Mirror Optimization",
        "Orphan Cleanup",
        "PKGBUILD/Build Env Fix",
        "Performance Optimize (zram/zswap)",
        "Full Maintenance",
        "Back",
    ];
    match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Arch Maintenance")
        .items(&opts)
        .default(0)
        .interact()
        .unwrap()
    {
        0 => fix(),
        1 => keyring(),
        2 => mirrors(),
        3 => orphans(),
        4 => pkgfix(),
        5 => optimize(),
        6 => full(),
        _ => (),
    }
}
