use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use std::fs;
use std::process::Command;

pub fn driver_menu() {
    println!("🎮 NVIDIA Driver Management");
    println!("===========================");

    let options = [
        "📊 Check current driver status",
        "🏢 Install proprietary drivers",
        "🔓 Install open-source drivers",
        "🧪 Install open-source beta (AUR)",
        "🔄 Switch between driver types",
        "🗑️  Remove all NVIDIA drivers",
        "🔧 Fix driver issues",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Driver Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => check_driver_status(),
        1 => install_proprietary_drivers(),
        2 => install_open_drivers(),
        3 => install_open_beta_drivers(),
        4 => switch_drivers(),
        5 => remove_all_drivers(),
        6 => fix_driver_issues(),
        _ => return,
    }
}

pub fn check_driver_status() {
    println!("📊 Checking NVIDIA driver status...\n");

    // Check installed NVIDIA packages
    println!("=== INSTALLED PACKAGES ===");
    let _ = Command::new("pacman").args(&["-Qs", "nvidia"]).status();

    // Check kernel module
    println!("\n=== KERNEL MODULE ===");
    let output = Command::new("lsmod").output();
    if let Ok(output) = output {
        let lsmod = String::from_utf8_lossy(&output.stdout);
        if lsmod.contains("nvidia") {
            println!("✅ NVIDIA kernel module is loaded");

            // Show module details
            for line in lsmod.lines() {
                if line.contains("nvidia") {
                    println!("  {}", line);
                }
            }
        } else {
            println!("❌ NVIDIA kernel module is NOT loaded");
        }
    }

    // Check driver version
    println!("\n=== DRIVER VERSION ===");
    let output = Command::new("nvidia-smi")
        .args(&["--query-gpu=driver_version", "--format=csv,noheader"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("✅ Driver version: {}", version);
        }
        _ => {
            println!("❌ Could not get driver version (nvidia-smi failed)");

            // Try alternative method
            let output = Command::new("modinfo").args(&["nvidia"]).output();

            if let Ok(output) = output {
                let info = String::from_utf8_lossy(&output.stdout);
                for line in info.lines() {
                    if line.starts_with("version:") {
                        println!("  Module version: {}", line.replace("version:", "").trim());
                        break;
                    }
                }
            }
        }
    }

    // Check GPU info
    println!("\n=== GPU INFORMATION ===");
    let _ = Command::new("nvidia-smi").args(&["-L"]).status();

    // Check session type (X11/Wayland)
    println!("\n=== SESSION TYPE ===");
    if let Ok(session_type) = std::env::var("XDG_SESSION_TYPE") {
        println!("Session type: {}", session_type);

        if session_type == "wayland" {
            check_wayland_support();
        }
    } else {
        println!("Could not determine session type");
    }

    // Check for common issues
    println!("\n=== HEALTH CHECK ===");
    check_common_issues();
}

fn check_wayland_support() {
    println!("\n=== WAYLAND SUPPORT ===");

    // Check kernel parameters
    if let Ok(cmdline) = fs::read_to_string("/proc/cmdline") {
        if cmdline.contains("nvidia-drm.modeset=1") {
            println!("✅ nvidia-drm.modeset=1 found in kernel parameters");
        } else {
            println!("⚠️  nvidia-drm.modeset=1 NOT found in kernel parameters");
            println!("   Add to GRUB_CMDLINE_LINUX in /etc/default/grub");
        }
    }

    // Check modprobe config
    if let Ok(content) = fs::read_to_string("/etc/modprobe.d/nvidia.conf") {
        if content.contains("modeset=1") {
            println!("✅ modeset=1 found in /etc/modprobe.d/nvidia.conf");
        } else {
            println!("⚠️  modeset=1 NOT found in /etc/modprobe.d/nvidia.conf");
        }
    } else {
        println!("⚠️  /etc/modprobe.d/nvidia.conf not found");
    }

    // Check GBM backend
    if let Ok(gbm_backend) = std::env::var("GBM_BACKEND") {
        println!("GBM_BACKEND: {}", gbm_backend);
    } else {
        println!("💡 Consider setting GBM_BACKEND=nvidia-drm");
    }
}

fn check_common_issues() {
    let mut issues = Vec::new();

    // Check for conflicting drivers
    let output = Command::new("lsmod").output();
    if let Ok(output) = output {
        let lsmod = String::from_utf8_lossy(&output.stdout);
        if lsmod.contains("nouveau") {
            issues.push("❌ Nouveau driver is loaded (conflicts with NVIDIA)".to_string());
        }
    }

    // Check DKMS status
    let output = Command::new("dkms").args(&["status"]).output();
    if let Ok(output) = output {
        let status = String::from_utf8_lossy(&output.stdout);
        if status.contains("ERROR") || status.contains("broken") {
            issues.push("❌ DKMS errors detected".to_string());
        }
    }

    // Check for missing packages
    let required_packages = ["nvidia-utils", "nvidia-settings"];
    for package in &required_packages {
        let output = Command::new("pacman").args(&["-Qi", package]).output();

        if let Ok(output) = output
            && !output.status.success()
        {
            let issue = format!("⚠️  {} not installed", package);
            issues.push(issue);
        }
    }

    if issues.is_empty() {
        println!("✅ No common issues detected");
    } else {
        for issue in &issues {
            println!("{}", issue);
        }
    }
}

#[allow(dead_code)]
pub fn install_proprietary_drivers() {
    println!("🏢 Installing NVIDIA proprietary drivers...");

    // Check current status
    let output = Command::new("pacman").args(&["-Qi", "nvidia"]).output();

    if let Ok(output) = output
        && output.status.success()
    {
        println!("⚠️  NVIDIA proprietary drivers already installed");
        let reinstall = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Reinstall anyway?")
            .interact()
            .unwrap();

        if !reinstall {
            return;
        }
    }

    // Detect kernel
    let kernel_type = detect_kernel();
    println!("💡 Detected kernel: {}", kernel_type);

    let packages = match kernel_type.as_str() {
        "linux-lts" => vec!["nvidia-lts", "nvidia-utils", "nvidia-settings"],
        "linux-hardened" => vec!["nvidia-dkms", "nvidia-utils", "nvidia-settings"],
        "linux-zen" => vec!["nvidia-dkms", "nvidia-utils", "nvidia-settings"],
        _ => vec!["nvidia", "nvidia-utils", "nvidia-settings"],
    };

    println!("📦 Installing packages: {}", packages.join(", "));

    // Remove conflicting packages first
    remove_conflicting_packages();

    // Install packages
    let mut cmd = Command::new("sudo");
    cmd.args(&["pacman", "-S", "--noconfirm"]);
    cmd.args(&packages);

    let status = cmd.status();
    match status {
        Ok(status) if status.success() => {
            println!("✅ NVIDIA proprietary drivers installed successfully");
            post_install_setup();
        }
        _ => println!("❌ Failed to install NVIDIA proprietary drivers"),
    }
}

#[allow(dead_code)]
pub fn install_open_drivers() {
    println!("🔓 Installing NVIDIA open-source drivers...");

    // Check GPU compatibility
    if !check_open_driver_compatibility() {
        println!("❌ Your GPU may not be compatible with open-source drivers");
        let continue_anyway = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Continue anyway?")
            .interact()
            .unwrap();

        if !continue_anyway {
            return;
        }
    }

    let kernel_type = detect_kernel();
    let packages = match kernel_type.as_str() {
        "linux-lts" => vec!["nvidia-open-lts", "nvidia-utils", "nvidia-settings"],
        _ => vec!["nvidia-open", "nvidia-utils", "nvidia-settings"],
    };

    println!("📦 Installing packages: {}", packages.join(", "));

    // Remove conflicting packages
    remove_conflicting_packages();

    let mut cmd = Command::new("sudo");
    cmd.args(&["pacman", "-S", "--noconfirm"]);
    cmd.args(&packages);

    let status = cmd.status();
    match status {
        Ok(status) if status.success() => {
            println!("✅ NVIDIA open-source drivers installed successfully");
            post_install_setup();
        }
        _ => println!("❌ Failed to install NVIDIA open-source drivers"),
    }
}

#[allow(dead_code)]
pub fn install_open_beta_drivers() {
    println!("🧪 Installing NVIDIA open-source beta drivers from AUR...");

    // Check for AUR helper
    let aur_helper = detect_aur_helper();
    if aur_helper.is_none() {
        println!("❌ No AUR helper found. Install yay, paru, or similar first.");
        return;
    }

    let helper = aur_helper.unwrap();
    let packages = vec![
        "nvidia-open-beta-dkms",
        "nvidia-utils-beta",
        "nvidia-settings-beta",
    ];

    println!("📦 Installing AUR packages: {}", packages.join(", "));
    println!("🔧 Using AUR helper: {}", helper);

    // Remove conflicting packages
    remove_conflicting_packages();

    let mut cmd = Command::new(&helper);
    cmd.args(&["-S", "--noconfirm"]);
    cmd.args(&packages);

    let status = cmd.status();
    match status {
        Ok(status) if status.success() => {
            println!("✅ NVIDIA open-source beta drivers installed successfully");
            post_install_setup();
        }
        _ => println!("❌ Failed to install NVIDIA open-source beta drivers"),
    }
}

fn detect_kernel() -> String {
    let output = Command::new("uname").args(&["-r"]).output();
    if let Ok(output) = output {
        let kernel = String::from_utf8_lossy(&output.stdout);
        if kernel.contains("-lts") {
            return "linux-lts".to_string();
        } else if kernel.contains("-hardened") {
            return "linux-hardened".to_string();
        } else if kernel.contains("-zen") {
            return "linux-zen".to_string();
        }
    }
    "linux".to_string()
}

fn detect_aur_helper() -> Option<String> {
    let helpers = ["yay", "paru", "trizen", "pikaur"];
    for helper in &helpers {
        if Command::new("which").arg(helper).status().is_ok() {
            return Some(helper.to_string());
        }
    }
    None
}

fn check_open_driver_compatibility() -> bool {
    // Open drivers require RTX 20 series or newer (Turing+)
    let output = Command::new("nvidia-smi")
        .args(&["--query-gpu=name", "--format=csv,noheader"])
        .output();

    if let Ok(output) = output {
        let gpu_name = String::from_utf8_lossy(&output.stdout).to_lowercase();

        // RTX series
        if gpu_name.contains("rtx") {
            return true;
        }

        // GTX 16 series (Turing)
        if gpu_name.contains("gtx 16") {
            return true;
        }

        // Quadro RTX
        if gpu_name.contains("quadro rtx") {
            return true;
        }
    }

    false
}

fn remove_conflicting_packages() {
    println!("🗑️  Removing conflicting packages...");

    let conflicting = ["xf86-video-nouveau", "nouveau-dri"];

    for package in &conflicting {
        let _ = Command::new("sudo")
            .args(&["pacman", "-Rns", "--noconfirm", package])
            .status();
    }

    // Blacklist nouveau
    let blacklist_content = "blacklist nouveau\n";
    let _ = fs::write("/tmp/blacklist-nouveau.conf", blacklist_content);
    let _ = Command::new("sudo")
        .args(&["mv", "/tmp/blacklist-nouveau.conf", "/etc/modprobe.d/"])
        .status();
}

fn post_install_setup() {
    println!("🔧 Performing post-installation setup...");

    // Create nvidia.conf for modeset
    let nvidia_conf = "options nvidia-drm modeset=1\n";
    let _ = fs::write("/tmp/nvidia.conf", nvidia_conf);
    let _ = Command::new("sudo")
        .args(&["mv", "/tmp/nvidia.conf", "/etc/modprobe.d/"])
        .status();

    // Rebuild initramfs
    println!("🔄 Rebuilding initramfs...");
    let _ = Command::new("sudo").args(&["mkinitcpio", "-P"]).status();

    println!("✅ Post-installation setup complete");
    println!("🔄 Reboot required to load new drivers");
}

pub fn switch_drivers() {
    println!("🔄 Switching between NVIDIA driver types...");

    // Check current installation
    let proprietary_installed = Command::new("pacman")
        .args(&["-Qi", "nvidia"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    let open_installed = Command::new("pacman")
        .args(&["-Qi", "nvidia-open"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !proprietary_installed && !open_installed {
        println!("❌ No NVIDIA drivers currently installed");
        return;
    }

    let options = if proprietary_installed {
        vec!["Switch to open-source drivers", "Cancel"]
    } else {
        vec!["Switch to proprietary drivers", "Cancel"]
    };

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Driver switch options")
        .items(&options)
        .default(1)
        .interact()
        .unwrap();

    match choice {
        0 => {
            if proprietary_installed {
                println!("🔄 Switching to open-source drivers...");
                remove_all_drivers();
                install_open_drivers();
            } else {
                println!("🔄 Switching to proprietary drivers...");
                remove_all_drivers();
                install_proprietary_drivers();
            }
        }
        _ => return,
    }
}

pub fn remove_all_drivers() {
    println!("🗑️  Removing all NVIDIA drivers...");

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will remove all NVIDIA drivers. Continue?")
        .interact()
        .unwrap();

    if !confirm {
        return;
    }

    // List of possible NVIDIA packages
    let nvidia_packages = [
        "nvidia",
        "nvidia-lts",
        "nvidia-dkms",
        "nvidia-open",
        "nvidia-open-lts",
        "nvidia-utils",
        "nvidia-settings",
        "nvidia-open-beta-dkms",
        "nvidia-utils-beta",
        "nvidia-settings-beta",
    ];

    for package in &nvidia_packages {
        let _ = Command::new("sudo")
            .args(&["pacman", "-Rns", "--noconfirm", package])
            .status();
    }

    // Clean up configuration files
    let _ = fs::remove_file("/etc/modprobe.d/nvidia.conf");
    let _ = fs::remove_file("/etc/modprobe.d/blacklist-nouveau.conf");

    // Rebuild initramfs
    let _ = Command::new("sudo").args(&["mkinitcpio", "-P"]).status();

    println!("✅ All NVIDIA drivers removed");
    println!("🔄 Reboot recommended");
}

pub fn fix_driver_issues() {
    println!("🔧 Fixing common NVIDIA driver issues...");

    let options = [
        "Rebuild DKMS modules",
        "Fix missing kernel modules",
        "Reset NVIDIA configuration",
        "Fix Wayland issues",
        "Fix X11 configuration",
        "Clean and reinstall drivers",
        "Cancel",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select fix option")
        .items(&options)
        .default(6)
        .interact()
        .unwrap();

    match choice {
        0 => rebuild_dkms(),
        1 => fix_kernel_modules(),
        2 => reset_nvidia_config(),
        3 => fix_wayland_issues(),
        4 => fix_x11_config(),
        5 => clean_reinstall(),
        _ => return,
    }
}

fn rebuild_dkms() {
    println!("🔨 Rebuilding DKMS modules...");

    // Remove existing modules
    let _ = Command::new("sudo")
        .args(&["dkms", "remove", "-m", "nvidia", "--all"])
        .status();

    // Add and build
    let _ = Command::new("sudo").args(&["dkms", "autoinstall"]).status();

    // Rebuild initramfs
    let _ = Command::new("sudo").args(&["mkinitcpio", "-P"]).status();

    println!("✅ DKMS rebuild complete");
}

fn fix_kernel_modules() {
    println!("🔧 Fixing kernel modules...");

    // Unload modules
    let modules = ["nvidia_drm", "nvidia_modeset", "nvidia_uvm", "nvidia"];
    for module in &modules {
        let _ = Command::new("sudo")
            .args(&["modprobe", "-r", module])
            .status();
    }

    // Load modules
    for module in modules.iter().rev() {
        let _ = Command::new("sudo").args(&["modprobe", module]).status();
    }

    println!("✅ Kernel modules reloaded");
}

fn reset_nvidia_config() {
    println!("♻️  Resetting NVIDIA configuration...");

    // Backup and remove configs
    let config_files = [
        "/etc/X11/xorg.conf",
        "/etc/X11/xorg.conf.d/20-nvidia.conf",
        "/etc/modprobe.d/nvidia.conf",
    ];

    for config in &config_files {
        if std::path::Path::new(config).exists() {
            let backup = format!("{}.backup", config);
            let _ = Command::new("sudo").args(&["cp", config, &backup]).status();
            let _ = Command::new("sudo").args(&["rm", config]).status();
            println!("  Backed up and removed: {}", config);
        }
    }

    // Regenerate basic config
    post_install_setup();

    println!("✅ NVIDIA configuration reset");
}

fn fix_wayland_issues() {
    println!("🖥️  Fixing Wayland compatibility issues...");

    // Enable DRM modeset
    let nvidia_conf =
        "options nvidia-drm modeset=1\noptions nvidia NVreg_PreserveVideoMemoryAllocations=1\n";
    let _ = fs::write("/tmp/nvidia.conf", nvidia_conf);
    let _ = Command::new("sudo")
        .args(&["mv", "/tmp/nvidia.conf", "/etc/modprobe.d/"])
        .status();

    // Set environment variables
    println!("💡 Add these environment variables:");
    println!("  GBM_BACKEND=nvidia-drm");
    println!("  __GLX_VENDOR_LIBRARY_NAME=nvidia");
    println!("  WLR_NO_HARDWARE_CURSORS=1");

    println!("✅ Wayland fixes applied");
}

fn fix_x11_config() {
    println!("🖥️  Fixing X11 configuration...");

    // Generate basic xorg.conf
    let _ = Command::new("sudo").args(&["nvidia-xconfig"]).status();

    println!("✅ X11 configuration regenerated");
}

fn clean_reinstall() {
    println!("🧹 Performing clean driver reinstall...");

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will remove and reinstall all NVIDIA drivers. Continue?")
        .interact()
        .unwrap();

    if !confirm {
        return;
    }

    remove_all_drivers();

    let driver_options = ["Proprietary", "Open-source", "Open-source beta (AUR)"];
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select driver type to install")
        .items(&driver_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_proprietary_drivers(),
        1 => install_open_drivers(),
        2 => install_open_beta_drivers(),
        _ => {}
    }
}
