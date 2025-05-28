pub fn configure() {
    println!("ghostctl :: NVIDIA Wayland Configuration Helper");
    // Check for nvidia-drm.modeset=1 in kernel params
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("cat /proc/cmdline")
        .output();
    match output {
        Ok(out) => {
            let cmdline = String::from_utf8_lossy(&out.stdout);
            if cmdline.contains("nvidia-drm.modeset=1") {
                println!("[OK] nvidia-drm.modeset=1 is set in kernel params");
            } else {
                println!("[WARN] nvidia-drm.modeset=1 is NOT set in kernel params");
                println!("To enable, add 'nvidia-drm.modeset=1' to your kernel command line (GRUB/loader).\nExample: GRUB_CMDLINE_LINUX=\"... nvidia-drm.modeset=1\"");
            }
        },
        Err(e) => println!("Failed to check kernel params: {}", e),
    }
    // Suggest modprobe.d config
    println!("- To enable Wayland, add 'options nvidia-drm modeset=1' to /etc/modprobe.d/nvidia.conf");
    // Detect driver type
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("pacman -Qs nvidia-open")
        .output();
    match output {
        Ok(out) => {
            let pkgs = String::from_utf8_lossy(&out.stdout);
            if pkgs.contains("nvidia-open") {
                println!("Detected: NVIDIA open driver installed.");
            } else {
                println!("Detected: NVIDIA proprietary driver (or no open driver).");
            }
        },
        Err(_) => println!("Could not detect driver type."),
    }
    println!("- Use GBM backend for best compatibility (export GBM_BACKEND=\"nvidia-drm\")");
}

