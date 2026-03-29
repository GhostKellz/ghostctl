pub mod container;
pub mod dkms;
pub mod dlss;
pub mod drivers;
pub mod errors;
pub mod optimize;
pub mod passthrough;
pub mod source_build;
pub mod wayland;

pub fn clean() {
    println!("ghostctl :: NVIDIA Clean DKMS/Modules");

    // Get NVIDIA package version from pacman without using awk
    let version = std::process::Command::new("pacman")
        .args(["-Q", "nvidia"])
        .output()
        .ok()
        .and_then(|out| {
            let output = String::from_utf8_lossy(&out.stdout);
            // Format: "nvidia <version>", extract version (second field)
            output.split_whitespace().nth(1).map(|v| v.to_string())
        });

    let status = match version {
        Some(ver) => {
            let module = format!("nvidia/{}", ver);
            std::process::Command::new("sudo")
                .args(["dkms", "remove", &module, "--all"])
                .status()
        }
        None => {
            // Fallback: try to remove any nvidia module
            std::process::Command::new("sudo")
                .args(["dkms", "remove", "-m", "nvidia", "--all"])
                .status()
        }
    };

    match status {
        Ok(s) if s.success() => println!("Old NVIDIA DKMS modules cleaned."),
        _ => println!("Failed to clean DKMS modules (may not be critical)."),
    }
}

pub fn fix() {
    println!("ghostctl :: NVIDIA DKMS Rebuild + mkinitcpio");
    dkms::rebuild();
    let status = std::process::Command::new("sudo")
        .args(["mkinitcpio", "-P"])
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
    let status = std::process::Command::new("sudo")
        .args(["pacman", "-S", "--noconfirm", "nvidia", "nvidia-utils"])
        .status();
    match status {
        Ok(s) if s.success() => println!("NVIDIA proprietary driver installed."),
        _ => println!("Failed to install NVIDIA proprietary driver."),
    }
}

pub fn install_open() {
    println!("Installing NVIDIA open driver...");
    let status = std::process::Command::new("sudo")
        .args(["pacman", "-S", "--noconfirm", "nvidia-open"])
        .status();
    match status {
        Ok(s) if s.success() => println!("NVIDIA open driver installed."),
        _ => println!("Failed to install NVIDIA open driver."),
    }
}

pub fn install_open_beta() {
    println!("Installing NVIDIA open beta driver from AUR...");
    // Try each AUR helper in sequence
    let helpers = [
        ("yay", vec!["-S", "--noconfirm", "nvidia-open-beta"]),
        ("ghostbrew", vec!["-S", "--noconfirm", "nvidia-open-beta"]),
        ("pacu", vec!["-S", "--noconfirm", "nvidia-open-beta"]),
    ];

    for (helper, args) in &helpers {
        if let Ok(status) = std::process::Command::new(helper).args(args).status()
            && status.success()
        {
            println!("NVIDIA open beta driver installed via {}.", helper);
            return;
        }
    }
    println!("Failed to install NVIDIA open beta driver (tried yay, ghostbrew, pacu).");
}

#[allow(dead_code)]
pub fn wayland_check() {
    println!("ghostctl :: NVIDIA Wayland Compatibility Check");
    // Check for nvidia-drm.modeset=1 in kernel params
    match std::fs::read_to_string("/proc/cmdline") {
        Ok(cmdline) => {
            if cmdline.contains("nvidia-drm.modeset=1") {
                println!("[OK] nvidia-drm.modeset=1 is set in kernel params");
            } else {
                println!("[WARN] nvidia-drm.modeset=1 is NOT set in kernel params");
            }
        }
        Err(e) => println!("Failed to check kernel params: {}", e),
    }
    // Check for GBM/EGL from environment variables
    let gbm_backend = std::env::var("GBM_BACKEND").unwrap_or_default();
    let egl_platform = std::env::var("EGL_PLATFORM").unwrap_or_default();
    println!("GBM/EGL: {} {}", gbm_backend, egl_platform);
}

#[allow(dead_code)]
pub fn wayland_config() {
    println!("ghostctl :: NVIDIA Wayland Config Helper");
    println!(
        "- To enable Wayland, add 'options nvidia-drm modeset=1' to /etc/modprobe.d/nvidia.conf"
    );
    println!("- Add 'nvidia-drm.modeset=1' to your kernel command line (GRUB/loader)");
    println!("- Use GBM backend for best compatibility");
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
    let output = std::process::Command::new("modinfo").arg("nvidia").output();
    match output {
        Ok(out) => {
            let info = String::from_utf8_lossy(&out.stdout);
            for line in info.lines() {
                if line.starts_with("version:") {
                    println!("Driver version: {}", line.replace("version:", "").trim());
                }
            }
        }
        Err(_) => println!("Could not get driver version from modinfo."),
    }
}

pub fn nvidia_menu() {
    use dialoguer::{Select, theme::ColorfulTheme};

    println!("🎮 NVIDIA Management");
    println!("===================");

    let options = [
        "📊 Check status and driver info",
        "🚀 Driver management",
        "🎯 DLSS Management",
        "🔨 Build from source",
        "🐳 Container & virtualization",
        "🖥️  GPU passthrough",
        "🔧 DKMS rebuild and fixes",
        "⚡ Performance optimization",
        "🧹 Clean DKMS modules",
        "📋 Diagnostics",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("NVIDIA Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => status(),
        1 => drivers::driver_menu(),
        2 => dlss::dlss_menu(),
        3 => source_build::source_build_menu(),
        4 => container::container_menu(),
        5 => passthrough::passthrough_menu(),
        6 => fix(),
        7 => optimize(),
        8 => clean(),
        9 => diagnostics(),
        _ => return,
    }
}

pub fn status() {
    println!("ghostctl :: NVIDIA Status");
    // Check installed packages
    let output = std::process::Command::new("pacman")
        .args(["-Qs", "nvidia"])
        .output();
    match output {
        Ok(out) => println!(
            "Installed NVIDIA packages:\n{}",
            String::from_utf8_lossy(&out.stdout)
        ),
        Err(_) => println!("Could not query pacman for NVIDIA packages."),
    }
    // DKMS status
    let output = std::process::Command::new("dkms").arg("status").output();
    match output {
        Ok(out) => println!("DKMS status:\n{}", String::from_utf8_lossy(&out.stdout)),
        Err(_) => println!("Could not get DKMS status."),
    }
    // Kernel module
    let output = std::process::Command::new("lsmod").output();
    match output {
        Ok(out) => {
            let lsmod = String::from_utf8_lossy(&out.stdout);
            if lsmod.contains("nvidia") {
                println!("nvidia kernel module is loaded.");
            } else {
                println!("nvidia kernel module is NOT loaded.");
            }
        }
        Err(_) => println!("Could not check kernel modules."),
    }
    // Xorg/Wayland status (basic) - use XDG_SESSION_TYPE or loginctl
    // First try the environment variable (most reliable)
    if let Ok(session_type) = std::env::var("XDG_SESSION_TYPE") {
        println!("Session type: {}", session_type);
    } else {
        // Fallback: get session ID and query loginctl
        let session_id = std::env::var("XDG_SESSION_ID").ok().or_else(|| {
            // Try to get session ID from loginctl output
            std::process::Command::new("loginctl")
                .args(["list-sessions", "--no-legend"])
                .output()
                .ok()
                .and_then(|out| {
                    let output = String::from_utf8_lossy(&out.stdout);
                    // Format: "SESSION UID USER SEAT TTY" - first field is session ID
                    output
                        .lines()
                        .next()
                        .and_then(|line| line.split_whitespace().next().map(|s| s.to_string()))
                })
        });

        if let Some(sid) = session_id {
            let output = std::process::Command::new("loginctl")
                .args(["show-session", &sid, "-p", "Type"])
                .output();
            match output {
                Ok(out) => {
                    let type_output = String::from_utf8_lossy(&out.stdout);
                    println!("Session type: {}", type_output.trim());
                }
                Err(_) => println!("Could not determine session type (Xorg/Wayland)."),
            }
        } else {
            println!("Could not determine session type (Xorg/Wayland).");
        }
    }
}

pub fn optimize() {
    optimize::optimize();
}

#[allow(dead_code)]
pub fn write_nvidia_conf() {
    use std::io::Write;
    let conf_path = "/etc/modprobe.d/nvidia.conf";
    let content = "options nvidia-drm modeset=1\n";
    println!("About to write to {}:\n{}", conf_path, content);
    println!("Prompting for confirmation...");
    use dialoguer::Confirm;
    let confirmed = Confirm::new()
        .with_prompt("Overwrite /etc/modprobe.d/nvidia.conf?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);
    if confirmed {
        match std::fs::File::create(conf_path) {
            Ok(mut file) => {
                if file.write_all(content.as_bytes()).is_ok() {
                    println!("nvidia.conf written successfully.");
                } else {
                    println!("Failed to write nvidia.conf.");
                }
            }
            Err(e) => println!("Failed to open nvidia.conf for writing: {}", e),
        }
    } else {
        println!("Aborted writing nvidia.conf.");
    }
}
