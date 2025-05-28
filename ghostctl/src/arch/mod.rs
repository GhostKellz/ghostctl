pub mod archfix;
pub mod perf;
pub mod pkgfix;

pub fn fix(target: String) {
    match target.as_str() {
        "pacman" => archfix::fix_pacman(),
        "pkgbuild" => pkgfix::fix_pkgbuild(),
        "optimize" => perf::tune(),
        _ => println!("Unknown arch fix target"),
    }
}

pub fn optimize_mirrors() {
    println!("Optimizing Arch mirrorlist using reflector...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -S --noconfirm reflector && sudo reflector --latest 20 --sort rate --save /etc/pacman.d/mirrorlist")
        .status();
    match status {
        Ok(s) if s.success() => println!("Mirrorlist optimized."),
        _ => println!("Failed to optimize mirrorlist."),
    }
}

pub fn cleanup_orphans() {
    println!("Cleaning up orphaned packages...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -Rns $(pacman -Qtdq)")
        .status();
    match status {
        Ok(s) if s.success() => println!("Orphaned packages removed."),
        _ => println!("No orphaned packages to remove or failed to clean up."),
    }
}
