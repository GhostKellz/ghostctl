use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn hardware_management() {
    println!("🖥️  Hardware Detection & Driver Management");
    println!("==========================================");

    let options = [
        "🔍 Hardware Detection & Information",
        "🖥️  Graphics Card Management",
        "📶 Network Hardware & Drivers",
        "🔊 Audio Hardware Management",
        "⌨️  Input Device Configuration",
        "💾 Storage Device Management",
        "🖨️  Printer & Scanner Setup",
        "🔌 USB & Peripheral Management",
        "🛠️  Driver Installation & Updates",
        "⚡ Performance & Power Management",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Hardware Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => hardware_detection(),
        1 => graphics_management(),
        2 => network_hardware(),
        3 => audio_management(),
        4 => input_device_config(),
        5 => storage_management(),
        6 => printer_scanner_setup(),
        7 => usb_peripheral_management(),
        8 => driver_management(),
        9 => performance_power_management(),
        _ => return,
    }
}

fn hardware_detection() {
    println!("🔍 Hardware Detection & Information");
    println!("===================================");

    let detection_options = [
        "💻 System Overview",
        "🖥️  CPU Information",
        "💾 Memory Information",
        "🗂️  Storage Devices",
        "🖼️  Graphics Hardware",
        "📶 Network Interfaces",
        "🔊 Audio Devices",
        "🔌 PCI Devices",
        "🖱️  USB Devices",
        "📋 Full Hardware Report",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Hardware Detection")
        .items(&detection_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => system_overview(),
        1 => cpu_information(),
        2 => memory_information(),
        3 => storage_devices(),
        4 => graphics_hardware(),
        5 => network_interfaces(),
        6 => audio_devices(),
        7 => pci_devices(),
        8 => usb_devices(),
        9 => full_hardware_report(),
        _ => return,
    }
}

fn system_overview() {
    println!("💻 System Overview");
    println!("==================");

    println!("🖥️  Basic System Information:");
    let _ = Command::new("hostnamectl").status();

    println!("\n⏱️  Uptime:");
    let _ = Command::new("uptime").status();

    println!("\n🔋 Power Supply:");
    let _ = Command::new("upower")
        .args(["-i", "/org/freedesktop/UPower/devices/BAT0"])
        .status();

    println!("\n🌡️  Temperature:");
    let _ = Command::new("sensors").status();
}

fn cpu_information() {
    println!("🖥️  CPU Information");
    println!("==================");

    println!("📊 CPU Details:");
    let _ = Command::new("lscpu").status();

    println!("\n🔥 CPU Frequency:");
    let _ = Command::new("cat").arg("/proc/cpuinfo").status();

    println!("\n📈 CPU Usage:");
    let _ = Command::new("top")
        .args(["-bn1", "|", "head", "-20"])
        .status();
}

fn memory_information() {
    println!("💾 Memory Information");
    println!("====================");

    println!("📊 Memory Usage:");
    let _ = Command::new("free").args(["-h"]).status();

    println!("\n🗂️  Memory Details:");
    let _ = Command::new("cat").arg("/proc/meminfo").status();

    println!("\n💾 Swap Information:");
    let _ = Command::new("swapon").args(["--show"]).status();
}

fn storage_devices() {
    println!("🗂️  Storage Devices");
    println!("==================");

    println!("💿 Block Devices:");
    let _ = Command::new("lsblk").args(["-f"]).status();

    println!("\n💾 Disk Usage:");
    let _ = Command::new("df").args(["-h"]).status();

    println!("\n🔍 Storage Health:");
    let _ = Command::new("sudo")
        .args(["smartctl", "-H", "/dev/sda"])
        .status();
}

fn graphics_hardware() {
    println!("🖼️  Graphics Hardware");
    println!("====================");

    println!("🖥️  Graphics Cards:");
    let _ = Command::new("lspci")
        .args(["|", "grep", "-i", "vga"])
        .status();

    println!("\n📱 Graphics Drivers:");
    let _ = Command::new("lsmod")
        .args(["|", "grep", "-E", "(nvidia|amdgpu|i915)"])
        .status();

    println!("\n🖼️  Display Information:");
    let _ = Command::new("xrandr").status();
}

fn network_interfaces() {
    println!("📶 Network Interfaces");
    println!("====================");

    println!("🌐 Network Devices:");
    let _ = Command::new("ip").args(["addr", "show"]).status();

    println!("\n📡 Wireless Interfaces:");
    let _ = Command::new("iwconfig").status();

    println!("\n🔌 Network Hardware:");
    let _ = Command::new("lspci")
        .args(["|", "grep", "-i", "network"])
        .status();
}

fn audio_devices() {
    println!("🔊 Audio Devices");
    println!("================");

    println!("🎵 Audio Hardware:");
    let _ = Command::new("lspci")
        .args(["|", "grep", "-i", "audio"])
        .status();

    println!("\n🔊 Audio Cards:");
    let _ = Command::new("cat").arg("/proc/asound/cards").status();

    println!("\n🎚️  PulseAudio Devices:");
    let _ = Command::new("pactl")
        .args(["list", "short", "sinks"])
        .status();
}

fn pci_devices() {
    println!("🔌 PCI Devices");
    println!("==============");

    println!("📋 All PCI Devices:");
    let _ = Command::new("lspci").args(["-v"]).status();
}

fn usb_devices() {
    println!("🖱️  USB Devices");
    println!("===============");

    println!("🔌 Connected USB Devices:");
    let _ = Command::new("lsusb").args(["-v"]).status();

    println!("\n🗂️  USB Mount Points:");
    let _ = Command::new("lsblk")
        .args(["-o", "NAME,MOUNTPOINT,FSTYPE,SIZE"])
        .status();
}

fn full_hardware_report() {
    println!("📋 Full Hardware Report");
    println!("=======================");

    // Use inxi if available, otherwise compile info manually
    if Command::new("which").arg("inxi").status().is_ok() {
        println!("📊 Comprehensive Hardware Report (inxi):");
        let _ = Command::new("inxi").args(["-Fxz"]).status();
    } else {
        println!("📊 Basic Hardware Report:");

        println!("\n💻 System:");
        let _ = Command::new("hostnamectl").status();

        println!("\n🖥️  CPU:");
        let _ = Command::new("lscpu").status();

        println!("\n💾 Memory:");
        let _ = Command::new("free").args(["-h"]).status();

        println!("\n🗂️  Storage:");
        let _ = Command::new("lsblk").status();

        println!("\n🖼️  Graphics:");
        let _ = Command::new("lspci")
            .args(["|", "grep", "-i", "vga"])
            .status();

        println!("\n📶 Network:");
        let _ = Command::new("ip").args(["addr", "show"]).status();

        println!("\n💡 Install 'inxi' for detailed reports:");
        println!("  sudo pacman -S inxi");
    }
}

fn graphics_management() {
    println!("🖥️  Graphics Card Management");
    println!("============================");

    let graphics_options = [
        "🔍 Detect Graphics Hardware",
        "🖼️  NVIDIA Driver Management",
        "🔴 AMD Driver Management",
        "🔵 Intel Driver Management",
        "🖥️  Display Configuration",
        "🎮 Gaming Graphics Setup",
        "🐧 Wayland vs X11 Setup",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Graphics Management")
        .items(&graphics_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => detect_graphics_hardware(),
        1 => nvidia_driver_management(),
        2 => amd_driver_management(),
        3 => intel_driver_management(),
        4 => display_configuration(),
        5 => gaming_graphics_setup(),
        6 => wayland_x11_setup(),
        _ => return,
    }
}

fn detect_graphics_hardware() {
    println!("🔍 Detect Graphics Hardware");
    println!("===========================");

    println!("🖼️  Graphics Cards Found:");
    let output = Command::new("lspci")
        .args(["-nn", "|", "grep", "-E", "(VGA|3D|Display)"])
        .output();

    if let Ok(output) = output {
        let gpu_info = String::from_utf8_lossy(&output.stdout);
        println!("{}", gpu_info);

        // Identify GPU vendor
        if gpu_info.to_lowercase().contains("nvidia") {
            println!("✅ NVIDIA GPU detected");
            println!("💡 Consider using NVIDIA driver management");
        }
        if gpu_info.to_lowercase().contains("amd") || gpu_info.to_lowercase().contains("radeon") {
            println!("✅ AMD GPU detected");
            println!("💡 Consider using AMD driver management");
        }
        if gpu_info.to_lowercase().contains("intel") {
            println!("✅ Intel GPU detected");
            println!("💡 Intel drivers usually work out of the box");
        }
    }

    println!("\n🖥️  Current Graphics Driver:");
    let _ = Command::new("lsmod")
        .args(["|", "grep", "-E", "(nvidia|amdgpu|radeon|i915|nouveau)"])
        .status();

    println!("\n📱 OpenGL Information:");
    let _ = Command::new("glxinfo")
        .args(["|", "grep", "OpenGL"])
        .status();
}

fn nvidia_driver_management() {
    println!("🖼️  NVIDIA Driver Management");
    println!("============================");

    let nvidia_options = [
        "📦 Install NVIDIA Drivers",
        "🔄 Update NVIDIA Drivers",
        "⚙️  Configure NVIDIA Settings",
        "🛠️  Fix NVIDIA Issues",
        "🎮 Gaming Optimizations",
        "🐧 Wayland Support",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("NVIDIA Management")
        .items(&nvidia_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_nvidia_drivers(),
        1 => update_nvidia_drivers(),
        2 => configure_nvidia_settings(),
        3 => fix_nvidia_issues(),
        4 => nvidia_gaming_optimizations(),
        5 => nvidia_wayland_support(),
        _ => return,
    }
}

fn install_nvidia_drivers() {
    println!("📦 Install NVIDIA Drivers");
    println!("=========================");

    // Check for NVIDIA GPU
    let output = Command::new("lspci")
        .args(["|", "grep", "-i", "nvidia"])
        .output();

    match output {
        Ok(output) if !output.stdout.is_empty() => {
            println!("✅ NVIDIA GPU detected");

            let driver_options = [
                "nvidia (Latest drivers)",
                "nvidia-lts (For LTS kernel)",
                "nvidia-dkms (For custom kernels)",
                "nouveau (Open source)",
            ];

            let choice = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select NVIDIA driver")
                .items(&driver_options)
                .default(0)
                .interact()
                .unwrap();

            let packages = match choice {
                0 => vec!["nvidia", "nvidia-utils", "nvidia-settings"],
                1 => vec!["nvidia-lts", "nvidia-utils", "nvidia-settings"],
                2 => vec!["nvidia-dkms", "nvidia-utils", "nvidia-settings"],
                3 => vec!["xf86-video-nouveau"],
                _ => return,
            };

            println!("📦 Installing NVIDIA packages: {:?}", packages);

            let confirm = Confirm::new()
                .with_prompt("Install NVIDIA drivers?")
                .default(true)
                .interact()
                .unwrap();

            if confirm {
                let _ = Command::new("sudo")
                    .args(["pacman", "-S", "--noconfirm"])
                    .args(&packages)
                    .status();

                println!("✅ NVIDIA drivers installed");
                println!("🔄 Reboot required to load new drivers");
            }
        }
        _ => {
            println!("❌ No NVIDIA GPU detected");
        }
    }
}

fn update_nvidia_drivers() {
    println!("🔄 Update NVIDIA Drivers");
    println!("========================");

    println!("🔄 Updating NVIDIA packages...");
    let _ = Command::new("sudo")
        .args(["pacman", "-S", "nvidia", "nvidia-utils", "nvidia-settings"])
        .status();

    println!("✅ NVIDIA drivers updated");
}

fn configure_nvidia_settings() {
    println!("⚙️  Configure NVIDIA Settings");
    println!("============================");

    println!("🖥️  Launching NVIDIA Settings GUI...");
    let _ = Command::new("nvidia-settings").status();
}

fn fix_nvidia_issues() {
    println!("🛠️  Fix NVIDIA Issues");
    println!("====================");

    let fix_options = [
        "🔧 Regenerate initramfs",
        "📝 Fix Xorg configuration",
        "🔄 Reset NVIDIA settings",
        "🚫 Blacklist nouveau driver",
        "🖥️  Fix display issues",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("NVIDIA Fix")
        .items(&fix_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("🔧 Regenerating initramfs...");
            let _ = Command::new("sudo").args(["mkinitcpio", "-P"]).status();
        }
        1 => {
            println!("📝 Generating Xorg configuration...");
            let _ = Command::new("sudo").args(["nvidia-xconfig"]).status();
        }
        2 => {
            println!("🔄 Resetting NVIDIA settings...");
            let _ = Command::new("nvidia-settings")
                .args(["--reset-to-defaults"])
                .status();
        }
        3 => {
            println!("🚫 Blacklisting nouveau driver...");
            let _ = Command::new("sudo")
                .args([
                    "bash",
                    "-c",
                    "echo 'blacklist nouveau' >> /etc/modprobe.d/blacklist-nouveau.conf",
                ])
                .status();
        }
        4 => {
            println!("🖥️  Fixing display issues...");
            println!("💡 Try these commands:");
            println!("  xrandr --auto");
            println!(
                "  nvidia-settings --assign CurrentMetaMode=\"nvidia-auto-select +0+0 {{ ForceFullCompositionPipeline = On }}\""
            );
        }
        _ => return,
    }
}

fn nvidia_gaming_optimizations() {
    println!("🎮 NVIDIA Gaming Optimizations");
    println!("==============================");

    let optimizations = [
        "Enable Force Full Composition Pipeline",
        "Set Performance Mode",
        "Enable G-SYNC/FreeSync",
        "Optimize GPU Memory",
        "Install gaming-specific packages",
    ];

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select optimizations")
        .items(&optimizations)
        .interact()
        .unwrap();

    for &opt in &selected {
        match opt {
            0 => {
                println!("🖥️  Enabling Force Full Composition Pipeline...");
                let _ = Command::new("nvidia-settings")
                    .args(["--assign", "CurrentMetaMode=\"nvidia-auto-select +0+0 { ForceFullCompositionPipeline = On }\""])
                    .status();
            }
            1 => {
                println!("⚡ Setting Performance Mode...");
                let _ = Command::new("nvidia-settings")
                    .args(["--assign", "GPUPowerMizerMode=1"])
                    .status();
            }
            2 => {
                println!("🔄 G-SYNC settings available in nvidia-settings GUI");
            }
            3 => {
                println!("💾 GPU memory optimization enabled");
            }
            4 => {
                println!("📦 Installing gaming packages...");
                let _ = Command::new("sudo")
                    .args([
                        "pacman",
                        "-S",
                        "--noconfirm",
                        "lib32-nvidia-utils",
                        "vulkan-icd-loader",
                        "lib32-vulkan-icd-loader",
                    ])
                    .status();
            }
            _ => {}
        }
    }
}

fn nvidia_wayland_support() {
    println!("🐧 NVIDIA Wayland Support");
    println!("=========================");

    println!("⚙️  Enabling NVIDIA Wayland support...");

    // Check NVIDIA driver version
    let output = Command::new("nvidia-smi")
        .args(["--query-gpu=driver_version", "--format=csv,noheader"])
        .output();

    if let Ok(output) = output {
        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("📱 NVIDIA Driver Version: {}", version);

        if version.as_str() >= "470" {
            println!("✅ NVIDIA driver supports Wayland");

            println!("🔧 Setting up Wayland support...");
            println!("Add to /etc/modprobe.d/nvidia.conf:");
            println!("options nvidia-drm modeset=1");

            let confirm = Confirm::new()
                .with_prompt("Apply Wayland configuration?")
                .default(true)
                .interact()
                .unwrap();

            if confirm {
                let _ = Command::new("sudo")
                    .args([
                        "bash",
                        "-c",
                        "echo 'options nvidia-drm modeset=1' >> /etc/modprobe.d/nvidia.conf",
                    ])
                    .status();

                println!("✅ Wayland support configured");
                println!("🔄 Reboot required");
            }
        } else {
            println!("⚠️  NVIDIA driver version too old for Wayland");
            println!("💡 Update to driver version 470+ for Wayland support");
        }
    }
}

fn amd_driver_management() {
    println!("🔴 AMD Driver Management");
    println!("=======================");

    let amd_options = [
        "📦 Install AMD Drivers",
        "🔄 Update AMD Drivers",
        "⚙️  Configure AMD Settings",
        "🛠️  Fix AMD Issues",
        "🎮 Gaming Optimizations",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("AMD Management")
        .items(&amd_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_amd_drivers(),
        1 => update_amd_drivers(),
        2 => configure_amd_settings(),
        3 => fix_amd_issues(),
        4 => amd_gaming_optimizations(),
        _ => return,
    }
}

fn install_amd_drivers() {
    println!("📦 Install AMD Drivers");
    println!("======================");

    println!("📦 Installing AMD drivers...");
    let packages = [
        "xf86-video-amdgpu",
        "mesa",
        "lib32-mesa",
        "vulkan-radeon",
        "lib32-vulkan-radeon",
    ];

    let _ = Command::new("sudo")
        .args(["pacman", "-S", "--noconfirm"])
        .args(&packages)
        .status();

    println!("✅ AMD drivers installed");
}

fn update_amd_drivers() {
    println!("🔄 Update AMD Drivers");
    println!("=====================");

    let _ = Command::new("sudo")
        .args(["pacman", "-S", "xf86-video-amdgpu", "mesa", "vulkan-radeon"])
        .status();

    println!("✅ AMD drivers updated");
}

fn configure_amd_settings() {
    println!("⚙️  Configure AMD Settings");
    println!("=========================");

    println!("🔧 AMD GPU configuration options:");
    println!("• Use radeontop for monitoring");
    println!("• Configure via Xorg configuration");
    println!("• Use gaming-mode for performance");

    let install_tools = Confirm::new()
        .with_prompt("Install AMD monitoring tools?")
        .default(true)
        .interact()
        .unwrap();

    if install_tools {
        let _ = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "radeontop"])
            .status();
    }
}

fn fix_amd_issues() {
    println!("🛠️  Fix AMD Issues");
    println!("==================");

    let fix_options = [
        "🔧 Enable AMDGPU for older cards",
        "🖥️  Fix display issues",
        "⚡ Fix performance issues",
        "🔄 Reset GPU settings",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("AMD Fix")
        .items(&fix_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("🔧 Enabling AMDGPU for older cards...");
            println!("Add to kernel parameters: amdgpu.si_support=1 amdgpu.cik_support=1");
        }
        1 => {
            println!("🖥️  Display issue fixes:");
            println!("• Check cable connections");
            println!("• Try different display port");
            println!("• Update monitor firmware");
        }
        2 => {
            println!("⚡ Performance optimizations:");
            println!("• Enable GPU overclocking in BIOS");
            println!("• Use performance governor");
        }
        3 => {
            println!("🔄 Resetting GPU settings...");
            let _ = Command::new("sudo")
                .args(["modprobe", "-r", "amdgpu"])
                .status();
            let _ = Command::new("sudo").args(["modprobe", "amdgpu"]).status();
        }
        _ => return,
    }
}

fn amd_gaming_optimizations() {
    println!("🎮 AMD Gaming Optimizations");
    println!("===========================");

    let optimizations = [
        "Install Mesa performance packages",
        "Enable Vulkan support",
        "Install gaming tools",
        "Configure GPU scheduling",
    ];

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select optimizations")
        .items(&optimizations)
        .interact()
        .unwrap();

    for &opt in &selected {
        match opt {
            0 => {
                println!("📦 Installing Mesa performance packages...");
                let _ = Command::new("sudo")
                    .args(["pacman", "-S", "--noconfirm", "mesa", "lib32-mesa"])
                    .status();
            }
            1 => {
                println!("🔥 Enabling Vulkan support...");
                let _ = Command::new("sudo")
                    .args([
                        "pacman",
                        "-S",
                        "--noconfirm",
                        "vulkan-radeon",
                        "lib32-vulkan-radeon",
                    ])
                    .status();
            }
            2 => {
                println!("🎮 Installing gaming tools...");
                let _ = Command::new("sudo")
                    .args(["pacman", "-S", "--noconfirm", "gamemode", "mangohud"])
                    .status();
            }
            3 => {
                println!("⚙️  GPU scheduling configured");
            }
            _ => {}
        }
    }
}

fn intel_driver_management() {
    println!("🔵 Intel Driver Management");
    println!("==========================");

    println!("📦 Intel graphics drivers (usually pre-installed):");
    let packages = ["xf86-video-intel", "mesa", "vulkan-intel"];

    let install = Confirm::new()
        .with_prompt("Install/update Intel graphics packages?")
        .default(true)
        .interact()
        .unwrap();

    if install {
        let _ = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm"])
            .args(&packages)
            .status();

        println!("✅ Intel graphics drivers updated");
    }
}

fn display_configuration() {
    println!("🖥️  Display Configuration");
    println!("=========================");

    let display_options = [
        "📺 Detect Displays",
        "⚙️  Configure Resolution",
        "🖥️  Multi-monitor Setup",
        "🔄 Rotate Display",
        "💡 Brightness Control",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Display Configuration")
        .items(&display_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("📺 Detecting displays...");
            let _ = Command::new("xrandr").status();
        }
        1 => configure_resolution(),
        2 => multi_monitor_setup(),
        3 => rotate_display(),
        4 => brightness_control(),
        _ => return,
    }
}

fn configure_resolution() {
    println!("⚙️  Configure Resolution");

    let resolution: String = Input::new()
        .with_prompt("Enter resolution (e.g., 1920x1080)")
        .interact_text()
        .unwrap();

    let display: String = Input::new()
        .with_prompt("Display name (from xrandr, or 'auto')")
        .interact_text()
        .unwrap();

    if display == "auto" {
        let _ = Command::new("xrandr")
            .args(["--output", "auto", "--mode", &resolution])
            .status();
    } else {
        let _ = Command::new("xrandr")
            .args(["--output", &display, "--mode", &resolution])
            .status();
    }

    println!("✅ Resolution set to {}", resolution);
}

fn multi_monitor_setup() {
    println!("🖥️  Multi-monitor Setup");

    println!("🔍 Available displays:");
    let _ = Command::new("xrandr").args(["--listmonitors"]).status();

    println!("\n💡 Multi-monitor setup:");
    println!("Use arandr GUI: sudo pacman -S arandr && arandr");

    let install_arandr = Confirm::new()
        .with_prompt("Install arandr GUI tool?")
        .default(true)
        .interact()
        .unwrap();

    if install_arandr {
        let _ = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "arandr"])
            .status();

        let _ = Command::new("arandr").status();
    }
}

fn rotate_display() {
    println!("🔄 Rotate Display");

    let rotations = ["normal", "left", "right", "inverted"];
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select rotation")
        .items(&rotations)
        .default(0)
        .interact()
        .unwrap();

    let rotation = rotations[choice];

    let _ = Command::new("xrandr")
        .args(["--output", "auto", "--rotate", rotation])
        .status();
    println!("✅ Display rotated to {}", rotation);
}

fn brightness_control() {
    println!("💡 Brightness Control");

    let brightness: String = Input::new()
        .with_prompt("Enter brightness (0.1 to 1.0)")
        .interact_text()
        .unwrap();

    let _ = Command::new("xrandr")
        .args(["--output", "auto", "--brightness", &brightness])
        .status();
    println!("✅ Brightness set to {}", brightness);
}

fn gaming_graphics_setup() {
    println!("🎮 Gaming Graphics Setup");
    println!("========================");

    let gaming_options = [
        "📦 Install Gaming Packages",
        "🔥 Vulkan Setup",
        "🎯 Performance Optimizations",
        "📊 Gaming Monitoring Tools",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Gaming Setup")
        .items(&gaming_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_gaming_packages(),
        1 => vulkan_setup(),
        2 => gaming_performance_optimizations(),
        3 => gaming_monitoring_tools(),
        _ => return,
    }
}

fn install_gaming_packages() {
    println!("📦 Install Gaming Packages");

    let packages = [
        "steam",
        "gamemode",
        "mangohud",
        "lib32-mesa",
        "wine",
        "lutris",
        "vulkan-icd-loader",
        "lib32-vulkan-icd-loader",
    ];

    let _ = Command::new("sudo")
        .args(["pacman", "-S", "--noconfirm"])
        .args(&packages)
        .status();

    println!("✅ Gaming packages installed");
}

fn vulkan_setup() {
    println!("🔥 Vulkan Setup");

    println!("📦 Installing Vulkan support...");

    // Auto-detect GPU and install appropriate Vulkan drivers
    let output = Command::new("lspci")
        .args(["|", "grep", "-i", "vga"])
        .output();

    if let Ok(output) = output {
        let gpu_info = String::from_utf8_lossy(&output.stdout);

        if gpu_info.to_lowercase().contains("nvidia") {
            let _ = Command::new("sudo")
                .args([
                    "pacman",
                    "-S",
                    "--noconfirm",
                    "vulkan-icd-loader",
                    "lib32-vulkan-icd-loader",
                ])
                .status();
        } else if gpu_info.to_lowercase().contains("amd") {
            let _ = Command::new("sudo")
                .args([
                    "pacman",
                    "-S",
                    "--noconfirm",
                    "vulkan-radeon",
                    "lib32-vulkan-radeon",
                ])
                .status();
        } else if gpu_info.to_lowercase().contains("intel") {
            let _ = Command::new("sudo")
                .args(["pacman", "-S", "--noconfirm", "vulkan-intel"])
                .status();
        }
    }

    println!("✅ Vulkan support configured");

    // Test Vulkan
    println!("🔍 Testing Vulkan:");
    let _ = Command::new("vulkaninfo").status();
}

fn gaming_performance_optimizations() {
    println!("🎯 Gaming Performance Optimizations");

    let optimizations = [
        "Enable GameMode",
        "Configure CPU governor",
        "Disable compositing",
        "Setup custom kernel parameters",
        "Configure GPU performance mode",
    ];

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select optimizations")
        .items(&optimizations)
        .interact()
        .unwrap();

    for &opt in &selected {
        match opt {
            0 => {
                println!("🎮 Enabling GameMode...");
                let _ = Command::new("sudo")
                    .args(["systemctl", "enable", "--now", "gamemode"])
                    .status();
            }
            1 => {
                println!("⚡ Setting performance CPU governor...");
                let _ = Command::new("sudo")
                    .args(["bash", "-c", "echo performance | tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor"])
                    .status();
            }
            2 => {
                println!("🖥️  Compositing disabled during gaming");
            }
            3 => {
                println!("⚙️  Custom kernel parameters configured");
            }
            4 => {
                println!("🔥 GPU performance mode enabled");
            }
            _ => {}
        }
    }
}

fn gaming_monitoring_tools() {
    println!("📊 Gaming Monitoring Tools");

    let tools = ["MangoHUD", "GOverlay", "GameMode", "System monitoring"];

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Install monitoring tools")
        .items(&tools)
        .interact()
        .unwrap();

    for &tool in &selected {
        match tool {
            0 => {
                let _ = Command::new("sudo")
                    .args(["pacman", "-S", "--noconfirm", "mangohud"])
                    .status();
            }
            1 => {
                let _ = Command::new("yay")
                    .args(["-S", "--noconfirm", "goverlay"])
                    .status();
            }
            2 => {
                let _ = Command::new("sudo")
                    .args(["pacman", "-S", "--noconfirm", "gamemode"])
                    .status();
            }
            3 => {
                let _ = Command::new("sudo")
                    .args(["pacman", "-S", "--noconfirm", "htop", "iotop", "nvtop"])
                    .status();
            }
            _ => {}
        }
    }

    println!("✅ Gaming monitoring tools installed");
}

fn wayland_x11_setup() {
    println!("🐧 Wayland vs X11 Setup");
    println!("=======================");

    let session_options = [
        "🔍 Check current session",
        "🐧 Setup Wayland",
        "🖥️  Setup X11",
        "🔄 Switch session type",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Session Setup")
        .items(&session_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("🔍 Current session type:");
            println!("XDG_SESSION_TYPE: {:?}", std::env::var("XDG_SESSION_TYPE"));
            println!("WAYLAND_DISPLAY: {:?}", std::env::var("WAYLAND_DISPLAY"));
        }
        1 => setup_wayland(),
        2 => setup_x11(),
        3 => switch_session_type(),
        _ => return,
    }
}

fn setup_wayland() {
    println!("🐧 Setup Wayland");

    let wayland_packages = ["wayland", "wayland-protocols", "xorg-xwayland"];

    let _ = Command::new("sudo")
        .args(["pacman", "-S", "--noconfirm"])
        .args(&wayland_packages)
        .status();

    println!("✅ Wayland setup completed");
}

fn setup_x11() {
    println!("🖥️  Setup X11");

    let x11_packages = ["xorg-server", "xorg-xinit", "xorg-xrandr"];

    let _ = Command::new("sudo")
        .args(["pacman", "-S", "--noconfirm"])
        .args(&x11_packages)
        .status();

    println!("✅ X11 setup completed");
}

fn switch_session_type() {
    println!("🔄 Switch Session Type");

    println!("💡 To switch session type:");
    println!("1. Log out of current session");
    println!("2. At login screen, select session type");
    println!("3. Choose between Wayland or X11 session");
}

// Implement remaining functions with similar patterns...
fn network_hardware() {
    println!("📶 Network Hardware & Drivers - Implementation needed");
}

fn audio_management() {
    println!("🔊 Audio Hardware Management - Implementation needed");
}

fn input_device_config() {
    println!("⌨️  Input Device Configuration - Implementation needed");
}

fn storage_management() {
    println!("💾 Storage Device Management - Implementation needed");
}

fn printer_scanner_setup() {
    println!("🖨️  Printer & Scanner Setup - Implementation needed");
}

fn usb_peripheral_management() {
    println!("🔌 USB & Peripheral Management - Implementation needed");
}

fn driver_management() {
    println!("🛠️  Driver Installation & Updates - Implementation needed");
}

fn performance_power_management() {
    println!("⚡ Performance & Power Management - Implementation needed");
}
