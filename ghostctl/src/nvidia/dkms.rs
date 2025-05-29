pub fn rebuild() {
    println!("ghostctl :: NVIDIA DKMS Rebuild");
    // Check if nvidia-dkms is installed
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("pacman -Qs nvidia-dkms")
        .output();
    match status {
        Ok(out) => {
            let pkgs = String::from_utf8_lossy(&out.stdout);
            if pkgs.contains("nvidia-dkms") {
                println!("[OK] nvidia-dkms is installed.");
            } else {
                println!("[WARN] nvidia-dkms is NOT installed. Rebuild will not work.");
                return;
            }
        }
        Err(_) => println!("Could not check for nvidia-dkms."),
    }
    // Rebuild DKMS modules
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo dkms install -m nvidia -k $(uname -r)")
        .status();
    match status {
        Ok(s) if s.success() => println!("DKMS modules rebuilt for current kernel."),
        _ => println!("Failed to rebuild DKMS modules."),
    }
    // Print DKMS status
    let status = std::process::Command::new("dkms").arg("status").output();
    match status {
        Ok(out) => println!("DKMS status:\n{}", String::from_utf8_lossy(&out.stdout)),
        Err(_) => println!("Could not get DKMS status."),
    }
    // Suggest mkinitcpio
    println!("- You may need to run 'sudo mkinitcpio -P' after rebuilding DKMS modules.");
}
