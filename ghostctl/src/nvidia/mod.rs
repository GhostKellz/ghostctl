pub mod dkms;
pub mod optimize;

pub fn clean() {
    println!("ghostctl :: NVIDIA Clean DKMS/Modules");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo dkms remove nvidia/$(pacman -Q nvidia | awk '{print $2}') --all || true")
        .status();
    match status {
        Ok(s) if s.success() => println!("Old NVIDIA DKMS modules cleaned."),
        _ => println!("Failed to clean DKMS modules (may not be critical)."),
    }
}

pub fn fix() {
    println!("ghostctl :: NVIDIA DKMS Rebuild + mkinitcpio");
    dkms::rebuild();
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo mkinitcpio -P")
        .status();
    match status {
        Ok(s) if s.success() => println!("mkinitcpio completed."),
        _ => println!("mkinitcpio failed."),
    }
}

pub fn diagnostics() {
    println!("ghostctl :: NVIDIA Diagnostics");
    let status = std::process::Command::new("nvidia-smi").status();
    match status {
        Ok(s) if s.success() => println!("nvidia-smi ran successfully."),
        _ => println!("nvidia-smi failed. Driver may not be installed."),
    }
}

pub fn install_proprietary() {
    println!("Installing NVIDIA proprietary driver...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -S --noconfirm nvidia nvidia-utils")
        .status();
    match status {
        Ok(s) if s.success() => println!("NVIDIA proprietary driver installed."),
        _ => println!("Failed to install NVIDIA proprietary driver."),
    }
}

pub fn install_open() {
    println!("Installing NVIDIA open driver...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -S --noconfirm nvidia-open")
        .status();
    match status {
        Ok(s) if s.success() => println!("NVIDIA open driver installed."),
        _ => println!("Failed to install NVIDIA open driver."),
    }
}

pub fn install_open_beta() {
    println!("Installing NVIDIA open beta driver from AUR...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("yay -S --noconfirm nvidia-open-beta || ghostbrew -S --noconfirm nvidia-open-beta || pacu -S --noconfirm nvidia-open-beta")
        .status();
    match status {
        Ok(s) if s.success() => println!("NVIDIA open beta driver installed."),
        _ => println!("Failed to install NVIDIA open beta driver (tried yay, ghostbrew, pacu)."),
    }
}

pub fn wayland_check() {
    println!("ghostctl :: NVIDIA Wayland Compatibility Check");
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
            }
        },
        Err(e) => println!("Failed to check kernel params: {}", e),
    }
    // Check for GBM/EGL
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("echo $GBM_BACKEND $EGL_PLATFORM")
        .output();
    match output {
        Ok(out) => println!("GBM/EGL: {}", String::from_utf8_lossy(&out.stdout)),
        Err(e) => println!("Failed to check GBM/EGL: {}", e),
    }
}

pub fn wayland_config() {
    println!("ghostctl :: NVIDIA Wayland Config Helper");
    println!("- To enable Wayland, add 'options nvidia-drm modeset=1' to /etc/modprobe.d/nvidia.conf");
    println!("- Add 'nvidia-drm.modeset=1' to your kernel command line (GRUB/loader)");
    println!("- Use GBM backend for best compatibility");
}

pub fn perf_mode() {
    println!("ghostctl :: NVIDIA Performance Mode");
    let status = std::process::Command::new("nvidia-smi")
        .args(["-pm", "1"])
        .status();
    match status {
        Ok(s) if s.success() => println!("Performance mode enabled."),
        _ => println!("Failed to enable performance mode."),
    }
}

pub fn troubleshoot() {
    println!("ghostctl :: NVIDIA Troubleshooting");
    println!("- Check dmesg, journalctl, and /var/log/Xorg.0.log for errors");
    println!("- Try reinstalling drivers or switching between open/proprietary");
    println!("- See the NVIDIA Arch Wiki for more tips");
}

pub fn info() {
    println!("ghostctl :: NVIDIA Info");
    let status = std::process::Command::new("nvidia-smi").status();
    match status {
        Ok(s) if s.success() => println!("nvidia-smi output above."),
        _ => println!("nvidia-smi failed. Driver may not be installed."),
    }
    // Show driver version
    let output = std::process::Command::new("modinfo")
        .arg("nvidia")
        .output();
    match output {
        Ok(out) => {
            let info = String::from_utf8_lossy(&out.stdout);
            for line in info.lines() {
                if line.starts_with("version:") {
                    println!("Driver version: {}", line.replace("version:", "").trim());
                }
            }
        },
        Err(_) => println!("Could not get driver version from modinfo."),
    }
}

pub fn status() {
    println!("ghostctl :: NVIDIA Status");
    // Check installed packages
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("pacman -Qs nvidia")
        .output();
    match output {
        Ok(out) => println!("Installed NVIDIA packages:\n{}", String::from_utf8_lossy(&out.stdout)),
        Err(_) => println!("Could not query pacman for NVIDIA packages."),
    }
    // DKMS status
    let output = std::process::Command::new("dkms")
        .arg("status")
        .output();
    match output {
        Ok(out) => println!("DKMS status:\n{}", String::from_utf8_lossy(&out.stdout)),
        Err(_) => println!("Could not get DKMS status."),
    }
    // Kernel module
    let output = std::process::Command::new("lsmod")
        .output();
    match output {
        Ok(out) => {
            let lsmod = String::from_utf8_lossy(&out.stdout);
            if lsmod.contains("nvidia") {
                println!("nvidia kernel module is loaded.");
            } else {
                println!("nvidia kernel module is NOT loaded.");
            }
        },
        Err(_) => println!("Could not check kernel modules."),
    }
    // Xorg/Wayland status (basic)
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("loginctl show-session $(loginctl | grep $(whoami) | awk '{print $1}') -p Type")
        .output();
    match output {
        Ok(out) => println!("Session type: {}", String::from_utf8_lossy(&out.stdout)),
        Err(_) => println!("Could not determine session type (Xorg/Wayland)."),
    }
}

pub fn optimize() {
    optimize::optimize();
}

pub fn write_nvidia_conf() {
    use std::io::Write;
    let conf_path = "/etc/modprobe.d/nvidia.conf";
    let content = "options nvidia-drm modeset=1\n";
    println!("About to write to {}:\n{}", conf_path, content);
    println!("Prompting for confirmation...");
    use dialoguer::Confirm;
    if Confirm::new().with_prompt("Overwrite /etc/modprobe.d/nvidia.conf?").default(false).interact().unwrap() {
        match std::fs::File::create(conf_path) {
            Ok(mut file) => {
                if file.write_all(content.as_bytes()).is_ok() {
                    println!("nvidia.conf written successfully.");
                } else {
                    println!("Failed to write nvidia.conf.");
                }
            },
            Err(e) => println!("Failed to open nvidia.conf for writing: {}", e),
        }
    } else {
        println!("Aborted writing nvidia.conf.");
    }
}

