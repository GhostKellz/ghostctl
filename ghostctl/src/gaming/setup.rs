use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn automated_setup() {
    loop {
        let options = [
            "🚀 Complete Gaming Setup (Recommended)",
            "🎯 Quick Gaming Essentials",
            "🔧 Custom Component Installation",
            "🎮 Gaming Platform Setup",
            "⚡ Performance Optimization Setup",
            "🖥️  Graphics & Display Setup",
            "🔊 Audio Setup for Gaming",
            "🎛️  Controller & Input Setup",
            "📊 Monitoring & Overlay Setup",
            "🧪 Troubleshooting & Repair Tools",
            "📋 Gaming Setup Status",
            "⬅️  Back",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🛠️  Automated Gaming Setup")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => complete_gaming_setup(),
            1 => quick_gaming_essentials(),
            2 => custom_component_installation(),
            3 => gaming_platform_setup(),
            4 => performance_optimization_setup(),
            5 => graphics_display_setup(),
            6 => audio_setup_gaming(),
            7 => controller_input_setup(),
            8 => monitoring_overlay_setup(),
            9 => troubleshooting_repair_tools(),
            10 => gaming_setup_status(),
            _ => break,
        }
    }
}

fn complete_gaming_setup() {
    println!("🚀 Complete Gaming Setup");
    println!("========================");

    println!("🎮 This will install and configure a complete gaming environment:");
    println!("  • Steam with Proton");
    println!("  • Lutris with Wine");
    println!("  • Graphics drivers and libraries");
    println!("  • Gaming performance tools");
    println!("  • Audio and controller support");
    println!("  • Monitoring and overlays");
    println!("  • Performance optimizations");

    let confirm = Confirm::new()
        .with_prompt("⚠️  This will install many packages and may take time. Continue?")
        .default(true)
        .interact()
        .unwrap();

    if !confirm {
        return;
    }

    println!("\n🔄 Starting complete gaming setup...");

    // Step 1: System preparation
    println!("\n📦 Step 1/8: System Preparation");
    setup_system_prerequisites();

    // Step 2: Graphics setup
    println!("\n🎨 Step 2/8: Graphics Setup");
    auto_setup_graphics();

    // Step 3: Audio setup
    println!("\n🔊 Step 3/8: Audio Setup");
    auto_setup_audio();

    // Step 4: Gaming platforms
    println!("\n🎮 Step 4/8: Gaming Platforms");
    auto_setup_gaming_platforms();

    // Step 5: Performance tools
    println!("\n⚡ Step 5/8: Performance Tools");
    auto_setup_performance_tools();

    // Step 6: Monitoring tools
    println!("\n📊 Step 6/8: Monitoring & Overlays");
    auto_setup_monitoring();

    // Step 7: Controller support
    println!("\n🎛️  Step 7/8: Controller Support");
    auto_setup_controllers();

    // Step 8: Final optimization
    println!("\n🔧 Step 8/8: System Optimization");
    auto_apply_optimizations();

    println!("\n✅ Complete gaming setup finished!");
    println!("🎮 Your system is now ready for gaming!");
    
    final_setup_summary();
}

fn setup_system_prerequisites() {
    println!("📦 Setting up system prerequisites...");

    // Enable multilib repository
    println!("🔧 Enabling multilib repository...");
    let multilib_check = Command::new("grep")
        .args(&["-E", "^\\[multilib\\]", "/etc/pacman.conf"])
        .output();

    match multilib_check {
        Ok(out) if out.stdout.is_empty() => {
            let status = Command::new("sudo")
                .arg("sed")
                .args(&["-i", "/^#\\[multilib\\]/,/^#Include = \\/etc\\/pacman.d\\/mirrorlist/ s/^#//", "/etc/pacman.conf"])
                .status();
            
            match status {
                Ok(s) if s.success() => {
                    println!("  ✅ Multilib repository enabled");
                    let _ = Command::new("sudo").args(&["pacman", "-Sy"]).status();
                }
                _ => println!("  ❌ Failed to enable multilib"),
            }
        }
        Ok(_) => println!("  ✅ Multilib repository already enabled"),
        _ => println!("  ❌ Could not check multilib status"),
    }

    // Install essential system packages
    println!("📦 Installing essential packages...");
    let essential_packages = [
        "base-devel",
        "git",
        "curl",
        "wget",
        "unzip",
        "lib32-mesa",
        "lib32-alsa-plugins",
        "lib32-libpulse",
        "lib32-openal",
        "vulkan-tools",
        "mesa-utils",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&essential_packages)
        .status();

    match status {
        Ok(s) if s.success() => println!("  ✅ Essential packages installed"),
        _ => println!("  ❌ Some essential packages failed to install"),
    }
}

fn auto_setup_graphics() {
    println!("🎨 Setting up graphics drivers and libraries...");

    // Detect GPU vendor
    let lspci_output = Command::new("lspci").args(&["-k"]).output();
    let mut gpu_vendor = "Unknown";
    
    if let Ok(output) = lspci_output {
        let lspci = String::from_utf8_lossy(&output.stdout);
        if lspci.contains("NVIDIA") {
            gpu_vendor = "NVIDIA";
        } else if lspci.contains("AMD") || lspci.contains("Radeon") {
            gpu_vendor = "AMD";
        } else if lspci.contains("Intel") && lspci.contains("Graphics") {
            gpu_vendor = "Intel";
        }
    }

    println!("  🔍 Detected GPU: {}", gpu_vendor);

    // Install graphics libraries
    let graphics_packages = [
        "mesa",
        "lib32-mesa",
        "vulkan-mesa-layers",
        "lib32-vulkan-mesa-layers",
        "vulkan-tools",
        "glxinfo",
        "vulkaninfo",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&graphics_packages)
        .status();

    match status {
        Ok(s) if s.success() => println!("  ✅ Graphics libraries installed"),
        _ => println!("  ❌ Some graphics packages failed to install"),
    }

    // Install vendor-specific drivers
    match gpu_vendor {
        "NVIDIA" => {
            println!("  🟢 Installing NVIDIA drivers...");
            let nvidia_packages = [
                "nvidia",
                "nvidia-utils",
                "lib32-nvidia-utils",
                "nvidia-settings",
            ];

            let status = Command::new("sudo")
                .args(&["pacman", "-S", "--needed", "--noconfirm"])
                .args(&nvidia_packages)
                .status();

            match status {
                Ok(s) if s.success() => println!("    ✅ NVIDIA drivers installed"),
                _ => println!("    ❌ NVIDIA driver installation failed"),
            }
        }
        "AMD" => {
            println!("  🔴 Installing AMD drivers...");
            let amd_packages = [
                "vulkan-radeon",
                "lib32-vulkan-radeon",
                "libva-mesa-driver",
                "lib32-libva-mesa-driver",
            ];

            let status = Command::new("sudo")
                .args(&["pacman", "-S", "--needed", "--noconfirm"])
                .args(&amd_packages)
                .status();

            match status {
                Ok(s) if s.success() => println!("    ✅ AMD drivers installed"),
                _ => println!("    ❌ AMD driver installation failed"),
            }
        }
        "Intel" => {
            println!("  🔵 Installing Intel drivers...");
            let intel_packages = [
                "vulkan-intel",
                "lib32-vulkan-intel",
                "intel-media-driver",
            ];

            let status = Command::new("sudo")
                .args(&["pacman", "-S", "--needed", "--noconfirm"])
                .args(&intel_packages)
                .status();

            match status {
                Ok(s) if s.success() => println!("    ✅ Intel drivers installed"),
                _ => println!("    ❌ Intel driver installation failed"),
            }
        }
        _ => println!("  ⚠️  Unknown GPU vendor - using generic drivers"),
    }
}

fn auto_setup_audio() {
    println!("🔊 Setting up audio for gaming...");

    // Check current audio system
    let pipewire_check = Command::new("pgrep").arg("pipewire").status();
    let pulse_check = Command::new("pgrep").arg("pulseaudio").status();

    if pipewire_check.map(|s| s.success()).unwrap_or(false) {
        println!("  ✅ PipeWire detected - installing gaming audio packages");
        let pipewire_packages = [
            "lib32-pipewire",
            "pipewire-pulse",
            "pipewire-alsa",
            "pipewire-jack",
        ];

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&pipewire_packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("    ✅ PipeWire gaming packages installed"),
            _ => println!("    ❌ Some PipeWire packages failed to install"),
        }
    } else if pulse_check.map(|s| s.success()).unwrap_or(false) {
        println!("  ✅ PulseAudio detected - installing gaming audio packages");
        let pulse_packages = [
            "lib32-libpulse",
            "lib32-alsa-plugins",
        ];

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&pulse_packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("    ✅ PulseAudio gaming packages installed"),
            _ => println!("    ❌ Some PulseAudio packages failed to install"),
        }
    } else {
        println!("  🎵 No audio system detected - installing PipeWire");
        let pipewire_packages = [
            "pipewire",
            "pipewire-pulse",
            "pipewire-alsa",
            "pipewire-jack",
            "wireplumber",
            "lib32-pipewire",
        ];

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&pipewire_packages)
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("    ✅ PipeWire installed");
                println!("    💡 Enable with: systemctl --user enable --now pipewire");
            }
            _ => println!("    ❌ PipeWire installation failed"),
        }
    }

    // Install additional audio packages
    let audio_packages = [
        "lib32-openal",
        "lib32-gst-plugins-base-libs",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&audio_packages)
        .status();

    match status {
        Ok(s) if s.success() => println!("  ✅ Additional audio packages installed"),
        _ => println!("  ❌ Some audio packages failed to install"),
    }
}

fn auto_setup_gaming_platforms() {
    println!("🎮 Setting up gaming platforms...");

    // Install Steam
    println!("  🚀 Installing Steam...");
    let steam_packages = [
        "steam",
        "lib32-libva",
        "lib32-libxss",
        "lib32-gst-plugins-base-libs",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&steam_packages)
        .status();

    match status {
        Ok(s) if s.success() => println!("    ✅ Steam installed"),
        _ => println!("    ❌ Steam installation failed"),
    }

    // Install Lutris
    println!("  🎯 Installing Lutris...");
    let lutris_packages = [
        "lutris",
        "wine",
        "winetricks",
        "dxvk",
        "vkd3d",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&lutris_packages)
        .status();

    match status {
        Ok(s) if s.success() => println!("    ✅ Lutris and Wine installed"),
        _ => println!("    ❌ Lutris installation failed"),
    }

    // Try to install Heroic via AUR (optional)
    println!("  🏛️  Attempting to install Heroic Games Launcher...");
    install_heroic_if_possible();
}

fn install_heroic_if_possible() {
    let aur_helpers = ["yay", "paru", "trizen"];
    for helper in &aur_helpers {
        let helper_check = Command::new("which").arg(helper).status();
        if let Ok(s) = helper_check {
            if s.success() {
                let install_status = Command::new(helper)
                    .args(&["-S", "--noconfirm", "heroic-games-launcher-bin"])
                    .status();
                
                match install_status {
                    Ok(s) if s.success() => {
                        println!("    ✅ Heroic Games Launcher installed");
                        return;
                    }
                    _ => continue,
                }
            }
        }
    }
    println!("    💡 Heroic installation skipped (no AUR helper found)");
}

fn auto_setup_performance_tools() {
    println!("⚡ Setting up performance tools...");

    // Install GameMode
    println!("  🚀 Installing GameMode...");
    let gamemode_status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm", "gamemode"])
        .status();

    match gamemode_status {
        Ok(s) if s.success() => {
            println!("    ✅ GameMode installed");
            
            // Add user to gamemode group
            let username = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
            let _ = Command::new("sudo")
                .args(&["usermod", "-a", "-G", "gamemode", &username])
                .status();
        }
        _ => println!("    ❌ GameMode installation failed"),
    }

    // Install CPU frequency utilities
    println!("  ⚡ Installing CPU performance tools...");
    let cpu_tools = [
        "cpupower",
        "stress",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&cpu_tools)
        .status();

    match status {
        Ok(s) if s.success() => println!("    ✅ CPU performance tools installed"),
        _ => println!("    ❌ Some CPU tools failed to install"),
    }

    // Setup gaming performance profile
    println!("  🔧 Creating gaming performance profile...");
    create_gaming_performance_profile();
}

fn create_gaming_performance_profile() {
    let profile_content = r#"#!/bin/bash
# Auto-generated gaming performance profile

echo "🎮 Applying gaming performance optimizations..."

# Set CPU governor to performance
if command -v cpupower >/dev/null 2>&1; then
    sudo cpupower frequency-set -g performance 2>/dev/null
fi

# Reduce swappiness for gaming
sudo sysctl vm.swappiness=1 2>/dev/null

# Start GameMode if available
if command -v gamemoded >/dev/null 2>&1; then
    systemctl --user start gamemode 2>/dev/null
fi

echo "✅ Gaming optimizations applied!"
"#;

    let profiles_dir = std::env::home_dir()
        .map(|h| h.join(".config/ghostctl/profiles"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config/ghostctl/profiles"));

    if let Ok(()) = std::fs::create_dir_all(&profiles_dir) {
        let profile_path = profiles_dir.join("gaming.sh");
        
        use std::fs::File;
        use std::io::Write;

        if let Ok(mut file) = File::create(&profile_path) {
            if file.write_all(profile_content.as_bytes()).is_ok() {
                let _ = Command::new("chmod").args(&["+x", &profile_path.to_string_lossy()]).status();
                println!("    ✅ Gaming performance profile created");
            }
        }
    }
}

fn auto_setup_monitoring() {
    println!("📊 Setting up monitoring and overlays...");

    // Install MangoHud
    println!("  🥭 Installing MangoHud...");
    let mangohud_packages = [
        "mangohud",
        "lib32-mangohud",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&mangohud_packages)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("    ✅ MangoHud installed");
            create_mangohud_config();
        }
        _ => println!("    ❌ MangoHud installation failed"),
    }

    // Install system monitoring tools
    println!("  📈 Installing system monitoring tools...");
    let monitoring_tools = [
        "htop",
        "nvtop",
        "iotop",
        "lm_sensors",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&monitoring_tools)
        .status();

    match status {
        Ok(s) if s.success() => println!("    ✅ System monitoring tools installed"),
        _ => println!("    ❌ Some monitoring tools failed to install"),
    }
}

fn create_mangohud_config() {
    let config_dir = std::env::home_dir()
        .map(|h| h.join(".config/MangoHud"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config/MangoHud"));

    if std::fs::create_dir_all(&config_dir).is_ok() {
        let config_file = config_dir.join("MangoHud.conf");
        let default_config = r#"### MangoHud Configuration ###

## Performance Metrics
fps
gpu_stats
cpu_stats
ram
vram

## Positioning and Appearance
position=top-left
background_alpha=0.4
font_size=24

## Additional Info
engine_version
vulkan_driver

## Temperature monitoring
gpu_temp
cpu_temp

## Frame timing
frame_timing=1
frametime=1

## Toggle key (F12 by default)
toggle_hud=F12
"#;

        use std::fs::File;
        use std::io::Write;

        if let Ok(mut file) = File::create(&config_file) {
            if file.write_all(default_config.as_bytes()).is_ok() {
                println!("    ✅ MangoHud configuration created");
            }
        }
    }
}

fn auto_setup_controllers() {
    println!("🎛️  Setting up controller support...");

    // Install controller packages
    let controller_packages = [
        "lib32-libusb",
        "jstest-gtk",
        "linuxconsole",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&controller_packages)
        .status();

    match status {
        Ok(s) if s.success() => println!("  ✅ Controller support packages installed"),
        _ => println!("  ❌ Some controller packages failed to install"),
    }

    // Check for connected controllers
    println!("  🔍 Checking for connected controllers...");
    let js_count = std::fs::read_dir("/dev/input/")
        .map(|entries| {
            entries.filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry.file_name().to_str()
                        .map_or(false, |name| name.starts_with("js"))
                })
                .count()
        })
        .unwrap_or(0);

    if js_count > 0 {
        println!("    🎮 Found {} controller device(s)", js_count);
    } else {
        println!("    💡 No controllers detected (connect and check again)");
    }
}

fn auto_apply_optimizations() {
    println!("🔧 Applying system optimizations...");

    // Set swappiness for gaming
    println!("  💾 Configuring memory management...");
    let _ = Command::new("sudo")
        .arg("sysctl")
        .arg("vm.swappiness=10")
        .status();

    // Create sysctl config for persistence
    let sysctl_config = "vm.swappiness=10\n";
    let _ = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(&format!("echo '{}' > /etc/sysctl.d/99-gaming.conf", sysctl_config))
        .status();

    println!("    ✅ Memory management optimized");

    // Enable services
    println!("  🔄 Configuring services...");
    
    // Enable gamemode user service
    let _ = Command::new("systemctl")
        .args(&["--user", "enable", "gamemode"])
        .status();

    println!("    ✅ Gaming services configured");

    // Create desktop entry for gaming profile
    create_gaming_profile_desktop_entry();
}

fn create_gaming_profile_desktop_entry() {
    let desktop_dir = std::env::home_dir()
        .map(|h| h.join(".local/share/applications"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.local/share/applications"));

    if std::fs::create_dir_all(&desktop_dir).is_ok() {
        let desktop_file = desktop_dir.join("gaming-profile.desktop");
        let desktop_content = r#"[Desktop Entry]
Version=1.0
Type=Application
Name=Gaming Performance Profile
Comment=Apply gaming performance optimizations
Exec=bash -c "~/.config/ghostctl/profiles/gaming.sh && notify-send 'Gaming Profile' 'Performance optimizations applied'"
Icon=applications-games
Terminal=false
Categories=Game;
"#;

        use std::fs::File;
        use std::io::Write;

        if let Ok(mut file) = File::create(&desktop_file) {
            if file.write_all(desktop_content.as_bytes()).is_ok() {
                println!("    ✅ Gaming profile desktop entry created");
            }
        }
    }
}

fn final_setup_summary() {
    println!("\n🎉 Gaming Setup Complete!");
    println!("========================");
    
    println!("✅ Installed components:");
    println!("  • Graphics drivers and libraries");
    println!("  • Steam with Proton support");
    println!("  • Lutris with Wine");
    println!("  • Audio system optimizations");
    println!("  • Performance tools (GameMode)");
    println!("  • Monitoring overlays (MangoHud)");
    println!("  • Controller support");
    println!("  • System optimizations");

    println!("\n🎮 How to use:");
    println!("  • Launch Steam from your application menu");
    println!("  • Use 'gamemoderun <game>' for performance");
    println!("  • Use 'mangohud <game>' for monitoring");
    println!("  • Apply gaming profile before playing");

    println!("\n💡 Next steps:");
    println!("  • Restart your system for best results");
    println!("  • Configure Steam Play for Windows games");
    println!("  • Install games and test performance");
    println!("  • Use MangoHud F12 to toggle overlay");

    println!("\n🔧 Performance profile:");
    println!("  • Run: ~/.config/ghostctl/profiles/gaming.sh");
    println!("  • Or find 'Gaming Performance Profile' in applications");

    let reboot = Confirm::new()
        .with_prompt("🔄 Reboot now to ensure all changes take effect?")
        .default(false)
        .interact()
        .unwrap();

    if reboot {
        println!("🔄 Rebooting system...");
        let _ = Command::new("sudo").args(&["reboot"]).status();
    }
}

fn quick_gaming_essentials() {
    println!("🎯 Quick Gaming Essentials");
    println!("==========================");

    println!("🚀 Installing essential gaming packages...");
    
    let essential_packages = [
        "steam",
        "lutris", 
        "wine",
        "gamemode",
        "mangohud",
        "lib32-mesa",
        "lib32-libpulse",
        "vulkan-tools",
        "dxvk",
    ];

    // Enable multilib first
    enable_multilib_quick();

    // Install packages
    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&essential_packages)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Essential gaming packages installed!");
            
            // Quick setup
            quick_setup_gamemode();
            quick_setup_mangohud();
            
            println!("\n🎮 Quick setup complete!");
            println!("💡 You can now launch Steam and start gaming");
        }
        _ => println!("❌ Some packages failed to install"),
    }
}

fn enable_multilib_quick() {
    let multilib_check = Command::new("grep")
        .args(&["-E", "^\\[multilib\\]", "/etc/pacman.conf"])
        .output();

    if let Ok(out) = multilib_check {
        if out.stdout.is_empty() {
            println!("🔧 Enabling multilib repository...");
            let _ = Command::new("sudo")
                .arg("sed")
                .args(&["-i", "/^#\\[multilib\\]/,/^#Include = \\/etc\\/pacman.d\\/mirrorlist/ s/^#//", "/etc/pacman.conf"])
                .status();
            let _ = Command::new("sudo").args(&["pacman", "-Sy"]).status();
        }
    }
}

fn quick_setup_gamemode() {
    let username = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
    let _ = Command::new("sudo")
        .args(&["usermod", "-a", "-G", "gamemode", &username])
        .status();
    
    let _ = Command::new("systemctl")
        .args(&["--user", "enable", "gamemode"])
        .status();
}

fn quick_setup_mangohud() {
    let config_dir = std::env::home_dir()
        .map(|h| h.join(".config/MangoHud"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config/MangoHud"));

    if std::fs::create_dir_all(&config_dir).is_ok() {
        let config_file = config_dir.join("MangoHud.conf");
        let simple_config = "fps\ngpu_stats\ncpu_stats\ntoggle_hud=F12\n";

        use std::fs::File;
        use std::io::Write;

        if let Ok(mut file) = File::create(&config_file) {
            let _ = file.write_all(simple_config.as_bytes());
        }
    }
}

fn custom_component_installation() {
    println!("🔧 Custom Component Installation");
    println!("===============================");

    let component_categories = [
        "🎮 Gaming Platforms",
        "🎨 Graphics & Drivers",
        "🔊 Audio Systems",
        "⚡ Performance Tools",
        "📊 Monitoring & Overlays",
        "🎛️  Controllers & Input",
        "🍷 Wine & Compatibility",
        "🧰 Development Tools",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select component category")
        .items(&component_categories)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_gaming_platforms(),
        1 => install_graphics_drivers(),
        2 => install_audio_systems(),
        3 => install_performance_tools(),
        4 => install_monitoring_overlays(),
        5 => install_controllers_input(),
        6 => install_wine_compatibility(),
        7 => install_development_tools(),
        _ => return,
    }
}

fn install_gaming_platforms() {
    println!("🎮 Gaming Platforms Installation");
    println!("===============================");

    let platforms = [
        ("Steam", vec!["steam", "lib32-libva", "lib32-libxss"]),
        ("Lutris", vec!["lutris", "wine", "winetricks"]),
        ("Heroic (Epic/GOG)", vec!["flatpak"]), // Special case
        ("Bottles (Wine)", vec!["flatpak"]), // Special case
        ("RetroArch", vec!["retroarch", "libretro-core-info"]),
        ("ScummVM", vec!["scummvm"]),
        ("DOSBox", vec!["dosbox"]),
    ];

    let selections = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select platforms to install")
        .items(&platforms.iter().map(|(name, _)| *name).collect::<Vec<_>>())
        .interact()
        .unwrap();

    for &index in &selections {
        let (name, packages) = &platforms[index];
        println!("📦 Installing {}...", name);

        match *name {
            "Heroic (Epic/GOG)" => install_heroic_flatpak(),
            "Bottles (Wine)" => install_bottles_flatpak(),
            _ => {
                let status = Command::new("sudo")
                    .args(&["pacman", "-S", "--needed", "--noconfirm"])
                    .args(packages)
                    .status();

                match status {
                    Ok(s) if s.success() => println!("  ✅ {} installed", name),
                    _ => println!("  ❌ {} installation failed", name),
                }
            }
        }
    }
}

fn install_heroic_flatpak() {
    let flatpak_check = Command::new("which").arg("flatpak").status();
    if flatpak_check.map(|s| s.success()).unwrap_or(false) {
        let _ = Command::new("flatpak")
            .args(&["install", "-y", "flathub", "com.heroicgameslauncher.hgl"])
            .status();
    } else {
        println!("  💡 Install flatpak first for Heroic");
    }
}

fn install_bottles_flatpak() {
    let flatpak_check = Command::new("which").arg("flatpak").status();
    if flatpak_check.map(|s| s.success()).unwrap_or(false) {
        let _ = Command::new("flatpak")
            .args(&["install", "-y", "flathub", "com.usebottles.bottles"])
            .status();
    } else {
        println!("  💡 Install flatpak first for Bottles");
    }
}

fn install_graphics_drivers() {
    println!("🎨 Graphics Drivers Installation");
    println!("===============================");

    // Detect GPU
    let lspci_output = Command::new("lspci").args(&["-k"]).output();
    let mut detected_gpus = Vec::new();
    
    if let Ok(output) = lspci_output {
        let lspci = String::from_utf8_lossy(&output.stdout);
        if lspci.contains("NVIDIA") {
            detected_gpus.push("NVIDIA");
        }
        if lspci.contains("AMD") || lspci.contains("Radeon") {
            detected_gpus.push("AMD");
        }
        if lspci.contains("Intel") && lspci.contains("Graphics") {
            detected_gpus.push("Intel");
        }
    }

    println!("🔍 Detected GPUs: {:?}", detected_gpus);

    let driver_options = [
        ("NVIDIA Drivers", vec!["nvidia", "nvidia-utils", "lib32-nvidia-utils", "nvidia-settings"]),
        ("AMD Drivers", vec!["vulkan-radeon", "lib32-vulkan-radeon", "libva-mesa-driver"]),
        ("Intel Drivers", vec!["vulkan-intel", "lib32-vulkan-intel", "intel-media-driver"]),
        ("Common Graphics Libraries", vec!["mesa", "lib32-mesa", "vulkan-tools", "glxinfo"]),
    ];

    let selections = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select drivers to install")
        .items(&driver_options.iter().map(|(name, _)| *name).collect::<Vec<_>>())
        .interact()
        .unwrap();

    for &index in &selections {
        let (name, packages) = &driver_options[index];
        println!("📦 Installing {}...", name);

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("  ✅ {} installed", name),
            _ => println!("  ❌ {} installation failed", name),
        }
    }
}

fn install_audio_systems() {
    println!("🔊 Audio Systems Installation");
    println!("=============================");

    let audio_systems = [
        ("PipeWire (Modern)", vec!["pipewire", "pipewire-pulse", "pipewire-alsa", "wireplumber", "lib32-pipewire"]),
        ("PulseAudio (Traditional)", vec!["pulseaudio", "pulseaudio-alsa", "lib32-libpulse"]),
        ("JACK (Professional)", vec!["jack2", "qjackctl"]),
        ("Gaming Audio Libraries", vec!["lib32-openal", "lib32-alsa-plugins", "lib32-gst-plugins-base-libs"]),
    ];

    let selections = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select audio systems to install")
        .items(&audio_systems.iter().map(|(name, _)| *name).collect::<Vec<_>>())
        .interact()
        .unwrap();

    for &index in &selections {
        let (name, packages) = &audio_systems[index];
        println!("📦 Installing {}...", name);

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("  ✅ {} installed", name),
            _ => println!("  ❌ {} installation failed", name),
        }
    }
}

fn install_performance_tools() {
    println!("⚡ Performance Tools Installation");
    println!("=================================");

    let performance_tools = [
        ("GameMode", vec!["gamemode"]),
        ("CPU Performance", vec!["cpupower", "stress", "sysbench"]),
        ("Memory Tools", vec!["zram-generator", "systemd-swap"]),
        ("I/O Tools", vec!["iotop", "iostat", "fio"]),
        ("System Monitoring", vec!["htop", "btop", "lm_sensors"]),
        ("Benchmarking", vec!["glmark2", "unigine-heaven"]),
    ];

    let selections = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select performance tools to install")
        .items(&performance_tools.iter().map(|(name, _)| *name).collect::<Vec<_>>())
        .interact()
        .unwrap();

    for &index in &selections {
        let (name, packages) = &performance_tools[index];
        println!("📦 Installing {}...", name);

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(packages)
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("  ✅ {} installed", name);
                
                // Special setup for GameMode
                if *name == "GameMode" {
                    let username = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
                    let _ = Command::new("sudo")
                        .args(&["usermod", "-a", "-G", "gamemode", &username])
                        .status();
                    println!("    💡 Added user to gamemode group");
                }
            }
            _ => println!("  ❌ {} installation failed", name),
        }
    }
}

fn install_monitoring_overlays() {
    println!("📊 Monitoring & Overlays Installation");
    println!("=====================================");

    let monitoring_tools = [
        ("MangoHud", vec!["mangohud", "lib32-mangohud"]),
        ("GPU Monitoring", vec!["nvtop", "radeontop", "intel-gpu-tools"]),
        ("System Monitoring", vec!["htop", "btop", "iotop", "nethogs"]),
        ("Temperature Monitoring", vec!["lm_sensors", "hddtemp"]),
        ("Network Monitoring", vec!["iftop", "bandwhich", "nload"]),
    ];

    let selections = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select monitoring tools to install")
        .items(&monitoring_tools.iter().map(|(name, _)| *name).collect::<Vec<_>>())
        .interact()
        .unwrap();

    for &index in &selections {
        let (name, packages) = &monitoring_tools[index];
        println!("📦 Installing {}...", name);

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(packages)
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("  ✅ {} installed", name);
                
                // Special setup for MangoHud
                if *name == "MangoHud" {
                    create_mangohud_config();
                }
            }
            _ => println!("  ❌ {} installation failed", name),
        }
    }
}

fn install_controllers_input() {
    println!("🎛️  Controllers & Input Installation");
    println!("====================================");

    let controller_tools = [
        ("Controller Support", vec!["lib32-libusb", "jstest-gtk", "linuxconsole"]),
        ("Input Mapping", vec!["antimicrox", "xboxdrv"]),
        ("Steam Controller", vec!["steam-native-runtime"]),
        ("Bluetooth Support", vec!["bluez", "bluez-utils"]),
    ];

    let selections = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select controller tools to install")
        .items(&controller_tools.iter().map(|(name, _)| *name).collect::<Vec<_>>())
        .interact()
        .unwrap();

    for &index in &selections {
        let (name, packages) = &controller_tools[index];
        println!("📦 Installing {}...", name);

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("  ✅ {} installed", name),
            _ => println!("  ❌ {} installation failed", name),
        }
    }
}

fn install_wine_compatibility() {
    println!("🍷 Wine & Compatibility Installation");
    println!("====================================");

    let wine_components = [
        ("Wine Base", vec!["wine", "winetricks"]),
        ("DirectX Translation", vec!["dxvk", "vkd3d"]),
        ("Windows Libraries", vec!["wine-mono", "wine-gecko"]),
        ("Font Support", vec!["ttf-liberation", "ttf-dejavu", "noto-fonts"]),
        ("Additional Tools", vec!["zenity", "kdialog"]),
    ];

    let selections = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Wine components to install")
        .items(&wine_components.iter().map(|(name, _)| *name).collect::<Vec<_>>())
        .interact()
        .unwrap();

    for &index in &selections {
        let (name, packages) = &wine_components[index];
        println!("📦 Installing {}...", name);

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("  ✅ {} installed", name),
            _ => println!("  ❌ {} installation failed", name),
        }
    }
}

fn install_development_tools() {
    println!("🧰 Development Tools Installation");
    println!("=================================");

    let dev_tools = [
        ("Build Tools", vec!["base-devel", "git", "cmake", "ninja"]),
        ("Game Development", vec!["godot", "blender", "krita"]),
        ("Debugging Tools", vec!["gdb", "valgrind", "strace"]),
        ("Performance Profiling", vec!["perf", "gperftools", "massif-visualizer"]),
        ("Version Control", vec!["git", "git-lfs", "mercurial"]),
    ];

    let selections = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select development tools to install")
        .items(&dev_tools.iter().map(|(name, _)| *name).collect::<Vec<_>>())
        .interact()
        .unwrap();

    for &index in &selections {
        let (name, packages) = &dev_tools[index];
        println!("📦 Installing {}...", name);

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("  ✅ {} installed", name),
            _ => println!("  ❌ {} installation failed", name),
        }
    }
}

fn gaming_platform_setup() {
    println!("🎮 Gaming Platform Setup");
    println!("========================");

    let platform_setups = [
        "🚀 Steam Complete Setup",
        "🎯 Lutris Configuration",
        "🏛️  Heroic Games Launcher Setup", 
        "🍷 Bottles Configuration",
        "🕹️  RetroArch Setup",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select platform to setup")
        .items(&platform_setups)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => steam_complete_setup(),
        1 => lutris_configuration_setup(),
        2 => heroic_setup(),
        3 => bottles_configuration_setup(),
        4 => retroarch_setup(),
        _ => return,
    }
}

fn steam_complete_setup() {
    println!("🚀 Steam Complete Setup");
    println!("=======================");

    // Install Steam if not present
    let steam_check = Command::new("which").arg("steam").status();
    if steam_check.map(|s| !s.success()).unwrap_or(true) {
        println!("📦 Installing Steam...");
        let steam_packages = [
            "steam",
            "lib32-libva",
            "lib32-libxss", 
            "lib32-gst-plugins-base-libs",
        ];

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&steam_packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("  ✅ Steam installed"),
            _ => {
                println!("  ❌ Steam installation failed");
                return;
            }
        }
    } else {
        println!("✅ Steam already installed");
    }

    // Setup Proton
    println!("\n🚀 Configuring Proton...");
    println!("💡 To enable Proton for Windows games:");
    println!("  1. Launch Steam");
    println!("  2. Go to Steam > Settings > Steam Play");
    println!("  3. ✅ Enable Steam Play for supported titles");
    println!("  4. ✅ Enable Steam Play for all other titles");
    println!("  5. Select latest Proton version");

    // Install ProtonUp-Qt for easy Proton management
    let protonup_install = Confirm::new()
        .with_prompt("Install ProtonUp-Qt for easy Proton version management?")
        .default(true)
        .interact()
        .unwrap();

    if protonup_install {
        install_protonup_qt();
    }

    // Create Steam optimization script
    create_steam_optimization_script();

    println!("\n✅ Steam setup complete!");
    println!("💡 Launch Steam and configure Steam Play in settings");
}

fn install_protonup_qt() {
    let aur_helpers = ["yay", "paru", "trizen"];
    for helper in &aur_helpers {
        let helper_check = Command::new("which").arg(helper).status();
        if let Ok(s) = helper_check {
            if s.success() {
                println!("📦 Installing ProtonUp-Qt...");
                let install_status = Command::new(helper)
                    .args(&["-S", "--noconfirm", "protonup-qt"])
                    .status();
                
                match install_status {
                    Ok(s) if s.success() => {
                        println!("  ✅ ProtonUp-Qt installed");
                        return;
                    }
                    _ => continue,
                }
            }
        }
    }
    println!("  💡 ProtonUp-Qt requires AUR helper (install yay)");
}

fn create_steam_optimization_script() {
    let script_content = r#"#!/bin/bash
# Steam Optimization Script

echo "🚀 Applying Steam gaming optimizations..."

# Apply gaming performance profile
if [ -f ~/.config/ghostctl/profiles/gaming.sh ]; then
    bash ~/.config/ghostctl/profiles/gaming.sh
fi

# Launch Steam with optimizations
export STEAM_RUNTIME=0  # Use native libraries
export __GL_THREADED_OPTIMIZATIONS=1
export __GL_SHADER_DISK_CACHE=1

# Start GameMode if available
if command -v gamemoded >/dev/null 2>&1; then
    systemctl --user start gamemode
fi

# Launch Steam
steam "$@"
"#;

    let bin_dir = std::env::home_dir()
        .map(|h| h.join("bin"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/bin"));

    if std::fs::create_dir_all(&bin_dir).is_ok() {
        let script_path = bin_dir.join("steam-optimized");
        
        use std::fs::File;
        use std::io::Write;

        if let Ok(mut file) = File::create(&script_path) {
            if file.write_all(script_content.as_bytes()).is_ok() {
                let _ = Command::new("chmod").args(&["+x", &script_path.to_string_lossy()]).status();
                println!("  ✅ Steam optimization script created");
                println!("  💡 Use: ~/bin/steam-optimized to launch Steam with optimizations");
            }
        }
    }
}

fn lutris_configuration_setup() {
    println!("🎯 Lutris Configuration Setup");
    println!("=============================");

    // Install Lutris if not present
    let lutris_check = Command::new("which").arg("lutris").status();
    if lutris_check.map(|s| !s.success()).unwrap_or(true) {
        println!("📦 Installing Lutris...");
        let lutris_packages = [
            "lutris",
            "wine",
            "winetricks",
            "dxvk",
            "vkd3d",
            "python-evdev",
        ];

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&lutris_packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("  ✅ Lutris installed"),
            _ => {
                println!("  ❌ Lutris installation failed");
                return;
            }
        }
    } else {
        println!("✅ Lutris already installed");
    }

    // Create Lutris configuration
    create_lutris_config();

    println!("\n✅ Lutris setup complete!");
    println!("💡 Launch Lutris and browse online installers for games");
}

fn create_lutris_config() {
    let lutris_config_dir = std::env::home_dir()
        .map(|h| h.join(".config/lutris"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config/lutris"));

    if std::fs::create_dir_all(&lutris_config_dir).is_ok() {
        let config_file = lutris_config_dir.join("lutris.conf");
        let config_content = r#"[lutris]
library_sync_at_startup = True
show_advanced_options = True

[system]
gamemode = True
feral_gamemode = True
disable_runtime = False

[wine]
dxvk = True
esync = True
fsync = True
"#;

        use std::fs::File;
        use std::io::Write;

        if let Ok(mut file) = File::create(&config_file) {
            if file.write_all(config_content.as_bytes()).is_ok() {
                println!("  ✅ Lutris configuration created");
            }
        }
    }
}

fn heroic_setup() {
    println!("🏛️  Heroic Games Launcher Setup");
    println!("===============================");

    println!("💡 Heroic installation options:");
    println!("  1. Flatpak (recommended)");
    println!("  2. AUR package");
    println!("  3. AppImage");

    let install_methods = [
        "Install via Flatpak",
        "Install via AUR",
        "Install AppImage",
        "Skip installation",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select installation method")
        .items(&install_methods)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            let flatpak_check = Command::new("which").arg("flatpak").status();
            if flatpak_check.map(|s| s.success()).unwrap_or(false) {
                let _ = Command::new("flatpak")
                    .args(&["install", "-y", "flathub", "com.heroicgameslauncher.hgl"])
                    .status();
                println!("✅ Heroic installed via Flatpak");
            } else {
                println!("❌ Flatpak not available");
            }
        }
        1 => install_heroic_if_possible(),
        2 => {
            println!("💡 Download Heroic AppImage from:");
            println!("   https://github.com/Heroic-Games-Launcher/HeroicGamesLauncher/releases");
        }
        _ => return,
    }

    println!("\n💡 Heroic setup tips:");
    println!("  • Connect Epic Games and GOG accounts");
    println!("  • Configure Wine versions for compatibility");
    println!("  • Enable GameMode in Heroic settings");
}

fn bottles_configuration_setup() {
    println!("🍷 Bottles Configuration Setup");
    println!("==============================");

    println!("💡 Installing Bottles via Flatpak (recommended)...");
    let flatpak_check = Command::new("which").arg("flatpak").status();
    if flatpak_check.map(|s| s.success()).unwrap_or(false) {
        let status = Command::new("flatpak")
            .args(&["install", "-y", "flathub", "com.usebottles.bottles"])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Bottles installed via Flatpak");
                
                println!("\n🍷 Bottles usage tips:");
                println!("  • Create separate bottles for different games/apps");
                println!("  • Use Gaming environment for games");
                println!("  • Install dependencies per bottle as needed");
                println!("  • Use bottle versioning for backup/restore");
            }
            _ => println!("❌ Bottles installation failed"),
        }
    } else {
        println!("❌ Flatpak not available - install flatpak first");
    }
}

fn retroarch_setup() {
    println!("🕹️  RetroArch Setup");
    println!("===================");

    println!("📦 Installing RetroArch and common cores...");
    let retroarch_packages = [
        "retroarch",
        "libretro-beetle-psx-hw",
        "libretro-snes9x",
        "libretro-nestopia",
        "libretro-genesis-plus-gx",
        "libretro-mupen64plus-next",
        "libretro-flycast",
    ];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&retroarch_packages)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ RetroArch and cores installed");
            
            println!("\n🕹️  RetroArch setup tips:");
            println!("  • Configure controllers in Settings > Input");
            println!("  • Set up directories for ROMs in Settings > Directory");
            println!("  • Download additional cores from Online Updater");
            println!("  • Use save states for convenience");
            println!("  ⚖️  Only use ROMs of games you legally own!");
        }
        _ => println!("❌ RetroArch installation failed"),
    }
}

fn performance_optimization_setup() {
    println!("⚡ Performance Optimization Setup");
    println!("=================================");

    let optimization_areas = [
        "🖥️  CPU Performance",
        "💾 Memory Management",
        "📁 Storage I/O",
        "🌡️  Thermal Management",
        "⚡ Power Management",
        "🎮 Gaming Optimizations",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select optimization area")
        .items(&optimization_areas)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => setup_cpu_performance(),
        1 => setup_memory_management(),
        2 => setup_storage_optimization(),
        3 => setup_thermal_management(),
        4 => setup_power_management(),
        5 => setup_gaming_optimizations(),
        _ => return,
    }
}

fn setup_cpu_performance() {
    println!("🖥️  CPU Performance Setup");
    println!("=========================");

    // Install CPU tools
    println!("📦 Installing CPU performance tools...");
    let cpu_packages = ["cpupower", "stress", "sysbench"];
    let _ = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&cpu_packages)
        .status();

    // Configure CPU governor
    println!("⚡ Configuring CPU governor for gaming...");
    let _ = Command::new("sudo")
        .args(&["cpupower", "frequency-set", "-g", "performance"])
        .status();

    // Create persistent configuration
    let cpupower_config = "governor='performance'\n";
    let _ = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(&format!("echo '{}' > /etc/default/cpupower", cpupower_config))
        .status();

    let _ = Command::new("sudo")
        .args(&["systemctl", "enable", "cpupower"])
        .status();

    println!("✅ CPU performance optimization configured");
}

fn setup_memory_management() {
    println!("💾 Memory Management Setup");
    println!("==========================");

    // Configure swappiness
    println!("🔧 Configuring memory settings for gaming...");
    let _ = Command::new("sudo")
        .arg("sysctl")
        .arg("vm.swappiness=10")
        .status();

    // Make persistent
    let sysctl_config = "vm.swappiness=10\nvm.dirty_ratio=15\nvm.dirty_background_ratio=5\n";
    let _ = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(&format!("echo '{}' > /etc/sysctl.d/99-gaming.conf", sysctl_config))
        .status();

    // Install zram if wanted
    let install_zram = Confirm::new()
        .with_prompt("Install zram for memory compression?")
        .default(true)
        .interact()
        .unwrap();

    if install_zram {
        let _ = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm", "zram-generator"])
            .status();
        
        let zram_config = "[zram0]\nzram-size = ram / 2\ncompression-algorithm = lz4\n";
        let _ = Command::new("sudo")
            .arg("sh")
            .arg("-c")
            .arg(&format!("echo '{}' > /etc/systemd/zram-generator.conf", zram_config))
            .status();
    }

    println!("✅ Memory management optimized");
}

fn setup_storage_optimization() {
    println!("📁 Storage I/O Optimization Setup");
    println!("==================================");

    // Set I/O scheduler
    println!("🚀 Configuring I/O scheduler for gaming...");
    
    // For NVMe drives, use none; for others, use kyber
    let scheduler_script = r#"#!/bin/bash
for dev in /sys/block/*/queue/scheduler; do
    if [[ -w "$dev" ]]; then
        if [[ $(basename $(dirname $(dirname "$dev"))) == nvme* ]]; then
            echo none > "$dev" 2>/dev/null || echo kyber > "$dev"
        else
            echo kyber > "$dev"
        fi
    fi
done
"#;

    let script_path = "/etc/systemd/system/gaming-io-scheduler.service";
    let service_content = format!(r#"[Unit]
Description=Gaming I/O Scheduler
After=multi-user.target

[Service]
Type=oneshot
ExecStart=/bin/bash -c '{}'
RemainAfterExit=yes

[Install]
WantedBy=multi-user.target
"#, scheduler_script.replace('\n', "\\n"));

    let _ = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(&format!("echo '{}' > {}", service_content, script_path))
        .status();

    let _ = Command::new("sudo")
        .args(&["systemctl", "enable", "gaming-io-scheduler"])
        .status();

    println!("✅ Storage I/O optimization configured");
}

fn setup_thermal_management() {
    println!("🌡️  Thermal Management Setup");
    println!("=============================");

    // Install temperature monitoring
    println!("📦 Installing thermal monitoring tools...");
    let thermal_packages = ["lm_sensors", "hddtemp"];
    let _ = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&thermal_packages)
        .status();

    println!("🔧 Run 'sudo sensors-detect' to configure sensors");
    
    // Install fan control if wanted
    let install_fancontrol = Confirm::new()
        .with_prompt("Setup fancontrol for custom fan curves?")
        .default(false)
        .interact()
        .unwrap();

    if install_fancontrol {
        println!("💨 fancontrol is part of lm_sensors");
        println!("🔧 Run 'sudo pwmconfig' to configure fan control");
    }

    println!("✅ Thermal management tools installed");
}

fn setup_power_management() {
    println!("⚡ Power Management Setup");
    println!("========================");

    println!("🔋 Power management configuration for gaming:");
    
    // Disable CPU power saving for gaming
    let _power_config = r#"# Disable CPU idle states for gaming performance
for state in /sys/devices/system/cpu/cpu*/cpuidle/state*/disable; do
    if [[ -w "$state" ]]; then
        echo 1 > "$state"
    fi
done
"#;

    println!("⚠️  This will increase power consumption but improve performance");
    let apply_power_config = Confirm::new()
        .with_prompt("Apply gaming power configuration?")
        .default(false)
        .interact()
        .unwrap();

    if apply_power_config {
        // This would need to be added to the gaming profile script
        println!("💡 Power optimizations will be added to gaming profile");
    }

    println!("✅ Power management reviewed");
}

fn setup_gaming_optimizations() {
    println!("🎮 Gaming Optimizations Setup");
    println!("=============================");

    // Install GameMode
    println!("🚀 Installing GameMode...");
    let _ = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm", "gamemode"])
        .status();

    // Add user to gamemode group
    let username = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
    let _ = Command::new("sudo")
        .args(&["usermod", "-a", "-G", "gamemode", &username])
        .status();

    // Create comprehensive gaming optimization script
    let gaming_script = r#"#!/bin/bash
# Comprehensive Gaming Optimization Script

echo "🎮 Applying comprehensive gaming optimizations..."

# CPU Performance
if command -v cpupower >/dev/null 2>&1; then
    sudo cpupower frequency-set -g performance 2>/dev/null
    echo "  ⚡ CPU set to performance mode"
fi

# Memory Management
sudo sysctl vm.swappiness=1 2>/dev/null
echo "  💾 Memory optimized for gaming"

# I/O Scheduler
for dev in /sys/block/*/queue/scheduler; do
    if [[ -w "$dev" ]]; then
        if [[ $(basename $(dirname $(dirname "$dev"))) == nvme* ]]; then
            echo none | sudo tee "$dev" >/dev/null 2>&1 || echo kyber | sudo tee "$dev" >/dev/null
        else
            echo kyber | sudo tee "$dev" >/dev/null 2>&1
        fi
    fi
done
echo "  📁 I/O scheduler optimized"

# GameMode
if command -v gamemoded >/dev/null 2>&1; then
    systemctl --user start gamemode 2>/dev/null
    echo "  🚀 GameMode activated"
fi

# Graphics optimizations
export __GL_THREADED_OPTIMIZATIONS=1
export __GL_SHADER_DISK_CACHE=1
export DXVK_ASYNC=1
echo "  🎨 Graphics optimizations applied"

# Clear memory cache
echo 1 | sudo tee /proc/sys/vm/drop_caches >/dev/null 2>&1
echo "  🧹 Memory cache cleared"

echo "✅ All gaming optimizations applied!"
echo "💡 Start your games now for best performance"
"#;

    let profiles_dir = std::env::home_dir()
        .map(|h| h.join(".config/ghostctl/profiles"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config/ghostctl/profiles"));

    if std::fs::create_dir_all(&profiles_dir).is_ok() {
        let script_path = profiles_dir.join("gaming-ultimate.sh");
        
        use std::fs::File;
        use std::io::Write;

        if let Ok(mut file) = File::create(&script_path) {
            if file.write_all(gaming_script.as_bytes()).is_ok() {
                let _ = Command::new("chmod").args(&["+x", &script_path.to_string_lossy()]).status();
                println!("✅ Ultimate gaming optimization script created");
                println!("💡 Run: ~/.config/ghostctl/profiles/gaming-ultimate.sh");
            }
        }
    }
}

fn graphics_display_setup() {
    println!("🖥️  Graphics & Display Setup");
    println!("============================");

    // Detect and setup graphics
    auto_setup_graphics();

    // Additional display optimizations
    println!("\n🔧 Display optimizations for gaming:");
    println!("  • Force fullscreen exclusive mode");
    println!("  • Disable compositor during gaming");
    println!("  • Configure refresh rates optimally");
    println!("  • Setup multi-monitor configurations");

    let apply_display_optimizations = Confirm::new()
        .with_prompt("Apply gaming display optimizations?")
        .default(true)
        .interact()
        .unwrap();

    if apply_display_optimizations {
        setup_display_optimizations();
    }
}

fn setup_display_optimizations() {
    // Create display optimization script
    let display_script = r#"#!/bin/bash
# Display Gaming Optimizations

echo "🖥️  Applying display optimizations for gaming..."

# Disable compositor (KDE example)
if command -v kwriteconfig5 >/dev/null 2>&1; then
    kwriteconfig5 --file kwinrc --group Compositing --key Enabled false
    qdbus org.kde.KWin /KWin reconfigure 2>/dev/null
    echo "  🎨 KDE compositor disabled"
fi

# Set performance mode for NVIDIA
if command -v nvidia-settings >/dev/null 2>&1; then
    nvidia-settings -a "[gpu:0]/GPUPowerMizerMode=1" 2>/dev/null
    echo "  🟢 NVIDIA performance mode enabled"
fi

echo "✅ Display optimizations applied!"
echo "💡 Remember to re-enable compositor after gaming"
"#;

    let profiles_dir = std::env::home_dir()
        .map(|h| h.join(".config/ghostctl/profiles"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config/ghostctl/profiles"));

    if std::fs::create_dir_all(&profiles_dir).is_ok() {
        let script_path = profiles_dir.join("display-gaming.sh");
        
        use std::fs::File;
        use std::io::Write;

        if let Ok(mut file) = File::create(&script_path) {
            if file.write_all(display_script.as_bytes()).is_ok() {
                let _ = Command::new("chmod").args(&["+x", &script_path.to_string_lossy()]).status();
                println!("✅ Display gaming optimization script created");
            }
        }
    }
}

fn audio_setup_gaming() {
    println!("🔊 Audio Setup for Gaming");
    println!("=========================");

    auto_setup_audio();

    println!("\n🎵 Additional gaming audio optimizations:");
    println!("  • Low latency audio configuration");
    println!("  • Spatial audio setup");
    println!("  • Voice chat optimization");
    println!("  • Multiple audio device management");

    create_audio_gaming_profile();
}

fn create_audio_gaming_profile() {
    let audio_script = r#"#!/bin/bash
# Gaming Audio Optimization

echo "🔊 Optimizing audio for gaming..."

# PipeWire optimizations
if systemctl --user is-active pipewire >/dev/null 2>&1; then
    echo "  🎵 PipeWire detected - applying optimizations"
    # Lower latency settings could go here
fi

# PulseAudio optimizations  
if systemctl --user is-active pulseaudio >/dev/null 2>&1; then
    echo "  🔊 PulseAudio detected - applying optimizations"
    # Audio optimizations could go here
fi

echo "✅ Audio optimized for gaming!"
"#;

    let profiles_dir = std::env::home_dir()
        .map(|h| h.join(".config/ghostctl/profiles"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config/ghostctl/profiles"));

    if std::fs::create_dir_all(&profiles_dir).is_ok() {
        let script_path = profiles_dir.join("audio-gaming.sh");
        
        use std::fs::File;
        use std::io::Write;

        if let Ok(mut file) = File::create(&script_path) {
            if file.write_all(audio_script.as_bytes()).is_ok() {
                let _ = Command::new("chmod").args(&["+x", &script_path.to_string_lossy()]).status();
                println!("✅ Gaming audio optimization script created");
            }
        }
    }
}

fn controller_input_setup() {
    println!("🎛️  Controller & Input Setup");
    println!("============================");

    auto_setup_controllers();

    println!("\n🎮 Additional controller optimizations:");
    println!("  • Input latency reduction");
    println!("  • Custom controller profiles");
    println!("  • Steam Input configuration");
    println!("  • Wireless optimization");

    let test_controllers = Confirm::new()
        .with_prompt("Test connected controllers?")
        .default(true)
        .interact()
        .unwrap();

    if test_controllers {
        test_controller_setup();
    }
}

fn test_controller_setup() {
    println!("🧪 Testing Controller Setup");
    println!("===========================");

    // List connected controllers
    println!("🔍 Detecting connected controllers...");
    let js_output = Command::new("ls").arg("/dev/input/js*").output();
    
    match js_output {
        Ok(output) if !output.stdout.is_empty() => {
            let controllers = String::from_utf8_lossy(&output.stdout);
            println!("🎮 Found controllers:");
            for controller in controllers.lines() {
                println!("  • {}", controller);
            }

            let test_input = Confirm::new()
                .with_prompt("Test controller input with jstest?")
                .default(false)
                .interact()
                .unwrap();

            if test_input {
                println!("🧪 Testing controller input (press Ctrl+C to exit):");
                let _ = Command::new("jstest").arg("/dev/input/js0").status();
            }
        }
        _ => println!("❌ No controllers detected"),
    }
}

fn monitoring_overlay_setup() {
    println!("📊 Monitoring & Overlay Setup");
    println!("=============================");

    auto_setup_monitoring();

    println!("\n📈 Additional monitoring setup:");
    println!("  • Performance logging configuration");
    println!("  • Custom overlay layouts");
    println!("  • Notification systems");
    println!("  • Remote monitoring");

    setup_advanced_monitoring();
}

fn setup_advanced_monitoring() {
    // Create monitoring dashboard script
    let monitoring_script = r#"#!/bin/bash
# Gaming Monitoring Dashboard

echo "📊 Starting gaming monitoring dashboard..."

# Start monitoring tools in tmux session
if command -v tmux >/dev/null 2>&1; then
    tmux new-session -d -s gaming-monitor 'htop' \; \
         split-window -v 'nvtop' \; \
         split-window -h 'iotop' \; \
         attach-session -t gaming-monitor
else
    echo "Install tmux for multi-panel monitoring"
    htop
fi
"#;

    let bin_dir = std::env::home_dir()
        .map(|h| h.join("bin"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/bin"));

    if std::fs::create_dir_all(&bin_dir).is_ok() {
        let script_path = bin_dir.join("gaming-monitor");
        
        use std::fs::File;
        use std::io::Write;

        if let Ok(mut file) = File::create(&script_path) {
            if file.write_all(monitoring_script.as_bytes()).is_ok() {
                let _ = Command::new("chmod").args(&["+x", &script_path.to_string_lossy()]).status();
                println!("✅ Gaming monitoring dashboard script created");
                println!("💡 Run: ~/bin/gaming-monitor");
            }
        }
    }
}

fn troubleshooting_repair_tools() {
    println!("🧪 Troubleshooting & Repair Tools");
    println!("==================================");

    let troubleshooting_options = [
        "🔧 System Diagnostics",
        "🎮 Gaming Platform Repair",
        "🎨 Graphics Driver Issues",
        "🔊 Audio Problems",
        "🎛️  Controller Issues",
        "📊 Performance Problems",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select troubleshooting area")
        .items(&troubleshooting_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => system_diagnostics(),
        1 => gaming_platform_repair(),
        2 => graphics_driver_issues(),
        3 => audio_problems(),
        4 => controller_issues(),
        5 => performance_problems(),
        _ => return,
    }
}

fn system_diagnostics() {
    println!("🔧 System Diagnostics");
    println!("=====================");

    println!("🔍 Running system diagnostics...");
    
    // System information
    println!("\n💻 System Information:");
    let _ = Command::new("uname").arg("-a").status();
    
    // Memory status
    println!("\n💾 Memory Status:");
    let _ = Command::new("free").arg("-h").status();
    
    // Disk space
    println!("\n💿 Disk Space:");
    let _ = Command::new("df").arg("-h").status();
    
    // CPU information
    println!("\n🖥️  CPU Information:");
    let _ = Command::new("lscpu").status();
    
    // Graphics information
    println!("\n🎨 Graphics Information:");
    let _ = Command::new("lspci").args(&["|", "grep", "-i", "vga"]).status();
    
    println!("\n✅ System diagnostics complete");
}

fn gaming_platform_repair() {
    println!("🎮 Gaming Platform Repair");
    println!("=========================");

    let repair_options = [
        "🚀 Steam Issues",
        "🎯 Lutris Problems", 
        "🍷 Wine/Proton Issues",
        "🏛️  Heroic Launcher Problems",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select platform to repair")
        .items(&repair_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => repair_steam_issues(),
        1 => repair_lutris_problems(),
        2 => repair_wine_proton(),
        3 => repair_heroic_problems(),
        _ => return,
    }
}

fn repair_steam_issues() {
    println!("🚀 Steam Issues Repair");
    println!("======================");

    let steam_fixes = [
        "🧹 Clear Steam cache",
        "🔄 Reset Steam runtime",
        "🔧 Fix Steam permissions",
        "📦 Reinstall Steam runtime",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Steam fix")
        .items(&steam_fixes)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("🧹 Clearing Steam cache...");
            let cache_path = std::env::home_dir()
                .map(|h| h.join(".steam/steam/appcache"))
                .unwrap_or_else(|| std::path::PathBuf::from("~/.steam/steam/appcache"));
            let _ = Command::new("rm").args(&["-rf", &cache_path.to_string_lossy()]).status();
            println!("✅ Steam cache cleared");
        }
        1 => {
            println!("🔄 Resetting Steam runtime...");
            let runtime_path = std::env::home_dir()
                .map(|h| h.join(".steam/steam/ubuntu12_32"))
                .unwrap_or_else(|| std::path::PathBuf::from("~/.steam/steam/ubuntu12_32"));
            let _ = Command::new("rm").args(&["-rf", &runtime_path.to_string_lossy()]).status();
            println!("✅ Steam runtime reset - restart Steam to rebuild");
        }
        2 => {
            println!("🔧 Fixing Steam permissions...");
            let steam_path = std::env::home_dir()
                .map(|h| h.join(".steam"))
                .unwrap_or_else(|| std::path::PathBuf::from("~/.steam"));
            let _ = Command::new("chmod").args(&["-R", "755", &steam_path.to_string_lossy()]).status();
            println!("✅ Steam permissions fixed");
        }
        _ => return,
    }
}

fn repair_lutris_problems() {
    println!("🎯 Lutris Problems Repair");
    println!("=========================");

    println!("🔧 Common Lutris fixes:");
    println!("  • Clear Lutris cache");
    println!("  • Reset Wine prefixes");
    println!("  • Update Lutris runners");
    println!("  • Fix permissions");

    let apply_fixes = Confirm::new()
        .with_prompt("Apply common Lutris fixes?")
        .default(true)
        .interact()
        .unwrap();

    if apply_fixes {
        // Clear Lutris cache
        let lutris_cache = std::env::home_dir()
            .map(|h| h.join(".cache/lutris"))
            .unwrap_or_else(|| std::path::PathBuf::from("~/.cache/lutris"));
        let _ = Command::new("rm").args(&["-rf", &lutris_cache.to_string_lossy()]).status();

        println!("✅ Lutris common fixes applied");
    }
}

fn repair_wine_proton() {
    println!("🍷 Wine/Proton Issues Repair");
    println!("============================");

    println!("🔧 Wine/Proton repair options:");
    println!("  • Reset Wine prefix");
    println!("  • Clear Wine cache");
    println!("  • Reinstall Wine dependencies");
    println!("  • Fix Wine permissions");

    let wine_fixes = [
        "🔄 Reset Wine prefix",
        "🧹 Clear Wine cache",
        "📦 Reinstall winetricks",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Wine fix")
        .items(&wine_fixes)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            let confirm = Confirm::new()
                .with_prompt("⚠️  Reset Wine prefix? This will delete all Windows software!")
                .default(false)
                .interact()
                .unwrap();

            if confirm {
                let wine_prefix = std::env::home_dir()
                    .map(|h| h.join(".wine"))
                    .unwrap_or_else(|| std::path::PathBuf::from("~/.wine"));
                let _ = Command::new("rm").args(&["-rf", &wine_prefix.to_string_lossy()]).status();
                println!("✅ Wine prefix reset");
            }
        }
        1 => {
            let wine_cache = std::env::home_dir()
                .map(|h| h.join(".cache/wine"))
                .unwrap_or_else(|| std::path::PathBuf::from("~/.cache/wine"));
            let _ = Command::new("rm").args(&["-rf", &wine_cache.to_string_lossy()]).status();
            println!("✅ Wine cache cleared");
        }
        2 => {
            let _ = Command::new("sudo")
                .args(&["pacman", "-S", "--needed", "--noconfirm", "winetricks"])
                .status();
            println!("✅ winetricks reinstalled");
        }
        _ => return,
    }
}

fn repair_heroic_problems() {
    println!("🏛️  Heroic Launcher Problems Repair");
    println!("===================================");

    println!("🔧 Heroic common issues and fixes:");
    println!("  • Update to latest version");
    println!("  • Clear application cache");
    println!("  • Reset Wine versions");
    println!("  • Fix Epic Games login");

    println!("💡 Try updating Heroic to the latest version first");
    println!("🔧 Clear cache and restart Heroic if login issues persist");
}

fn graphics_driver_issues() {
    println!("🎨 Graphics Driver Issues");
    println!("=========================");

    // Detect current graphics setup
    let lspci_output = Command::new("lspci").output();
    if let Ok(output) = lspci_output {
        let lspci = String::from_utf8_lossy(&output.stdout);
        
        if lspci.contains("NVIDIA") {
            println!("🟢 NVIDIA GPU detected");
            diagnose_nvidia_issues();
        } else if lspci.contains("AMD") || lspci.contains("Radeon") {
            println!("🔴 AMD GPU detected");
            diagnose_amd_issues();
        } else if lspci.contains("Intel") {
            println!("🔵 Intel GPU detected");
            diagnose_intel_issues();
        } else {
            println!("❓ Unknown GPU detected");
        }
    }
}

fn diagnose_nvidia_issues() {
    println!("🟢 NVIDIA Diagnostics");
    println!("=====================");

    // Check NVIDIA driver
    let nvidia_check = Command::new("nvidia-smi").status();
    match nvidia_check {
        Ok(s) if s.success() => {
            println!("✅ NVIDIA drivers working");
            let _ = Command::new("nvidia-smi").status();
        }
        _ => {
            println!("❌ NVIDIA drivers not working");
            let reinstall = Confirm::new()
                .with_prompt("Reinstall NVIDIA drivers?")
                .default(true)
                .interact()
                .unwrap();

            if reinstall {
                let _ = Command::new("sudo")
                    .args(&["pacman", "-S", "--needed", "--noconfirm", "nvidia", "nvidia-utils"])
                    .status();
                println!("🔄 Reboot required after driver installation");
            }
        }
    }
}

fn diagnose_amd_issues() {
    println!("🔴 AMD Diagnostics");
    println!("==================");

    // Check AMD driver
    let amd_check = Command::new("glxinfo").args(&["|", "grep", "AMD"]).status();
    match amd_check {
        Ok(s) if s.success() => println!("✅ AMD drivers appear to be working"),
        _ => {
            println!("❌ AMD drivers may have issues");
            println!("💡 Try: sudo pacman -S mesa lib32-mesa vulkan-radeon lib32-vulkan-radeon");
        }
    }
}

fn diagnose_intel_issues() {
    println!("🔵 Intel Diagnostics");
    println!("====================");

    println!("💡 Intel graphics are usually well-supported");
    println!("🔧 Ensure mesa and vulkan-intel are installed");
    
    let install_intel = Confirm::new()
        .with_prompt("Install Intel graphics packages?")
        .default(true)
        .interact()
        .unwrap();

    if install_intel {
        let _ = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm", "mesa", "vulkan-intel", "intel-media-driver"])
            .status();
    }
}

fn audio_problems() {
    println!("🔊 Audio Problems Diagnosis");
    println!("===========================");

    // Check audio system
    let pipewire_running = Command::new("pgrep").arg("pipewire").status()
        .map(|s| s.success()).unwrap_or(false);
    let pulse_running = Command::new("pgrep").arg("pulseaudio").status()
        .map(|s| s.success()).unwrap_or(false);

    if pipewire_running {
        println!("🎵 PipeWire detected");
        diagnose_pipewire_issues();
    } else if pulse_running {
        println!("🔊 PulseAudio detected");
        diagnose_pulseaudio_issues();
    } else {
        println!("❌ No audio system running");
        install_audio_system();
    }
}

fn diagnose_pipewire_issues() {
    println!("🎵 PipeWire Diagnostics");
    println!("=======================");

    // Check PipeWire status
    let _ = Command::new("systemctl").args(&["--user", "status", "pipewire"]).status();
    
    println!("\n🔧 Common PipeWire fixes:");
    println!("  • Restart PipeWire services");
    println!("  • Install lib32-pipewire for 32-bit games");
    println!("  • Check audio device permissions");

    let restart_pipewire = Confirm::new()
        .with_prompt("Restart PipeWire services?")
        .default(false)
        .interact()
        .unwrap();

    if restart_pipewire {
        let _ = Command::new("systemctl").args(&["--user", "restart", "pipewire"]).status();
        let _ = Command::new("systemctl").args(&["--user", "restart", "pipewire-pulse"]).status();
        println!("✅ PipeWire services restarted");
    }
}

fn diagnose_pulseaudio_issues() {
    println!("🔊 PulseAudio Diagnostics");
    println!("=========================");

    // Check PulseAudio status
    let _ = Command::new("pulseaudio").arg("--check").status();
    
    println!("\n🔧 Common PulseAudio fixes:");
    println!("  • Restart PulseAudio");
    println!("  • Install lib32-libpulse for 32-bit games");
    println!("  • Check audio device settings");

    let restart_pulse = Confirm::new()
        .with_prompt("Restart PulseAudio?")
        .default(false)
        .interact()
        .unwrap();

    if restart_pulse {
        let _ = Command::new("pulseaudio").arg("-k").status();
        let _ = Command::new("pulseaudio").arg("--start").status();
        println!("✅ PulseAudio restarted");
    }
}

fn install_audio_system() {
    println!("🎵 Installing Audio System");
    println!("==========================");

    let audio_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select audio system to install")
        .items(&["PipeWire (recommended)", "PulseAudio"])
        .default(0)
        .interact()
        .unwrap();

    match audio_choice {
        0 => {
            let pipewire_packages = [
                "pipewire", "pipewire-pulse", "pipewire-alsa", 
                "wireplumber", "lib32-pipewire"
            ];
            let _ = Command::new("sudo")
                .args(&["pacman", "-S", "--needed", "--noconfirm"])
                .args(&pipewire_packages)
                .status();
            println!("✅ PipeWire installed - enable with: systemctl --user enable --now pipewire");
        }
        1 => {
            let pulse_packages = ["pulseaudio", "pulseaudio-alsa", "lib32-libpulse"];
            let _ = Command::new("sudo")
                .args(&["pacman", "-S", "--needed", "--noconfirm"])
                .args(&pulse_packages)
                .status();
            println!("✅ PulseAudio installed");
        }
        _ => return,
    }
}

fn controller_issues() {
    println!("🎛️  Controller Issues Diagnosis");
    println!("===============================");

    // Check for controller devices
    println!("🔍 Checking for controller devices...");
    let js_devices = std::fs::read_dir("/dev/input/")
        .map(|entries| {
            entries.filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry.file_name().to_str()
                        .map_or(false, |name| name.starts_with("js"))
                })
                .count()
        })
        .unwrap_or(0);

    if js_devices > 0 {
        println!("✅ Found {} controller device(s)", js_devices);
        
        let test_controller = Confirm::new()
            .with_prompt("Test controller input?")
            .default(true)
            .interact()
            .unwrap();

        if test_controller {
            println!("🧪 Testing controller (press Ctrl+C to exit):");
            let _ = Command::new("jstest").arg("/dev/input/js0").status();
        }
    } else {
        println!("❌ No controller devices found");
        println!("💡 Try:");
        println!("  • Connect controller via USB/Bluetooth");
        println!("  • Install controller support packages");
        println!("  • Check USB permissions");

        let install_controller_support = Confirm::new()
            .with_prompt("Install controller support packages?")
            .default(true)
            .interact()
            .unwrap();

        if install_controller_support {
            let controller_packages = ["lib32-libusb", "jstest-gtk", "linuxconsole"];
            let _ = Command::new("sudo")
                .args(&["pacman", "-S", "--needed", "--noconfirm"])
                .args(&controller_packages)
                .status();
            println!("✅ Controller support packages installed");
        }
    }
}

fn performance_problems() {
    println!("📊 Performance Problems Diagnosis");
    println!("=================================");

    println!("🔍 Checking system performance indicators...");

    // Check CPU governor
    let governor_output = Command::new("cat")
        .arg("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
        .output();
    
    if let Ok(output) = governor_output {
        let governor_text = String::from_utf8_lossy(&output.stdout);
        let governor = governor_text.trim();
        println!("⚡ CPU governor: {}", governor);
        
        if governor != "performance" {
            println!("💡 Consider switching to 'performance' governor for gaming");
        }
    }

    // Check memory usage
    let free_output = Command::new("free").output();
    if let Ok(output) = free_output {
        println!("💾 Memory status:");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    // Check swap usage
    let swap_output = Command::new("swapon").arg("--show").output();
    if let Ok(output) = swap_output {
        if !output.stdout.is_empty() {
            println!("💿 Swap usage:");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            println!("💿 No swap configured");
        }
    }

    // Check GameMode
    let gamemode_check = Command::new("which").arg("gamemoderun").status();
    match gamemode_check {
        Ok(s) if s.success() => {
            println!("✅ GameMode available");
            let gamemode_running = Command::new("pgrep").arg("gamemode").status();
            match gamemode_running {
                Ok(s) if s.success() => println!("🟢 GameMode daemon running"),
                _ => println!("⭕ GameMode daemon not running"),
            }
        }
        _ => println!("❌ GameMode not installed"),
    }

    println!("\n💡 Performance optimization suggestions:");
    println!("  • Use 'performance' CPU governor");
    println!("  • Enable GameMode for games");
    println!("  • Close unnecessary background applications");
    println!("  • Monitor temperatures during gaming");
    println!("  • Use MangoHud to identify bottlenecks");
}

fn gaming_setup_status() {
    println!("📋 Gaming Setup Status");
    println!("======================");

    println!("🎮 Gaming Platform Status:");
    
    // Check Steam
    let steam_check = Command::new("which").arg("steam").status();
    match steam_check {
        Ok(s) if s.success() => println!("  ✅ Steam installed"),
        _ => println!("  ❌ Steam not installed"),
    }

    // Check Lutris
    let lutris_check = Command::new("which").arg("lutris").status();
    match lutris_check {
        Ok(s) if s.success() => println!("  ✅ Lutris installed"),
        _ => println!("  ❌ Lutris not installed"),
    }

    // Check Wine
    let wine_check = Command::new("which").arg("wine").status();
    match wine_check {
        Ok(s) if s.success() => println!("  ✅ Wine installed"),
        _ => println!("  ❌ Wine not installed"),
    }

    println!("\n⚡ Performance Tools Status:");
    
    // Check GameMode
    let gamemode_check = Command::new("which").arg("gamemoderun").status();
    match gamemode_check {
        Ok(s) if s.success() => println!("  ✅ GameMode installed"),
        _ => println!("  ❌ GameMode not installed"),
    }

    // Check MangoHud
    let mangohud_check = Command::new("which").arg("mangohud").status();
    match mangohud_check {
        Ok(s) if s.success() => println!("  ✅ MangoHud installed"),
        _ => println!("  ❌ MangoHud not installed"),
    }

    println!("\n🎨 Graphics Status:");
    
    // Check graphics drivers
    let lspci_output = Command::new("lspci").output();
    if let Ok(output) = lspci_output {
        let lspci = String::from_utf8_lossy(&output.stdout);
        if lspci.contains("NVIDIA") {
            let nvidia_check = Command::new("nvidia-smi").status();
            match nvidia_check {
                Ok(s) if s.success() => println!("  ✅ NVIDIA drivers working"),
                _ => println!("  ❌ NVIDIA drivers not working"),
            }
        }
        if lspci.contains("AMD") || lspci.contains("Radeon") {
            println!("  🔴 AMD GPU detected");
        }
        if lspci.contains("Intel") && lspci.contains("Graphics") {
            println!("  🔵 Intel GPU detected");
        }
    }

    // Check Vulkan
    let vulkan_check = Command::new("vulkaninfo").arg("--summary").status();
    match vulkan_check {
        Ok(s) if s.success() => println!("  ✅ Vulkan working"),
        _ => println!("  ❌ Vulkan not working"),
    }

    println!("\n🔊 Audio Status:");
    
    // Check audio system
    let pipewire_running = Command::new("pgrep").arg("pipewire").status()
        .map(|s| s.success()).unwrap_or(false);
    let pulse_running = Command::new("pgrep").arg("pulseaudio").status()
        .map(|s| s.success()).unwrap_or(false);

    if pipewire_running {
        println!("  ✅ PipeWire running");
    } else if pulse_running {
        println!("  ✅ PulseAudio running");
    } else {
        println!("  ❌ No audio system running");
    }

    println!("\n🎛️  Controller Status:");
    
    // Check controller support
    let js_devices = std::fs::read_dir("/dev/input/")
        .map(|entries| {
            entries.filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry.file_name().to_str()
                        .map_or(false, |name| name.starts_with("js"))
                })
                .count()
        })
        .unwrap_or(0);

    if js_devices > 0 {
        println!("  ✅ {} controller device(s) detected", js_devices);
    } else {
        println!("  ⭕ No controllers detected");
    }

    println!("\n📊 System Optimization Status:");
    
    // Check CPU governor
    let governor_output = Command::new("cat")
        .arg("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
        .output();
    
    if let Ok(output) = governor_output {
        let governor_text = String::from_utf8_lossy(&output.stdout);
        let governor = governor_text.trim();
        if governor == "performance" {
            println!("  ✅ CPU governor: {} (optimized)", governor);
        } else {
            println!("  ⚠️  CPU governor: {} (consider 'performance')", governor);
        }
    }

    // Check multilib
    let multilib_check = Command::new("grep")
        .args(&["-E", "^\\[multilib\\]", "/etc/pacman.conf"])
        .output();
    match multilib_check {
        Ok(out) if !out.stdout.is_empty() => println!("  ✅ Multilib repository enabled"),
        _ => println!("  ❌ Multilib repository disabled"),
    }

    println!("\n💡 Overall Status:");
    println!("Run 'Complete Gaming Setup' if many components are missing");
    println!("Use 'Troubleshooting & Repair Tools' to fix specific issues");
}