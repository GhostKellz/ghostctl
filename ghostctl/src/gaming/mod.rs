pub mod environment;
pub mod graphics;
pub mod monitoring;
pub mod performance;
pub mod platforms;
pub mod setup;
pub mod steam;

use dialoguer::{Select, theme::ColorfulTheme};

pub fn gaming_menu() {
    loop {
        let options = [
            "🚀 Steam & Proton Management",
            "🎯 Gaming Platforms (Lutris/Heroic/Bottles)",
            "⚡ Performance Optimization", 
            "📊 Gaming Monitoring & Overlays",
            "🎨 Graphics & Compatibility",
            "🔧 Gaming Environment Setup",
            "🛠️  Automated Gaming Setup",
            "📋 Gaming System Status",
            "⬅️  Back",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🎮 Gaming & Performance Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => steam::steam_menu(),
            1 => platforms::platforms_menu(),
            2 => performance::performance_menu(),
            3 => monitoring::monitoring_menu(),
            4 => graphics::graphics_menu(),
            5 => environment::environment_menu(),
            6 => setup::automated_setup(),
            7 => gaming_status(),
            _ => break,
        }
    }
}

pub fn gaming_status() {
    println!("🎮 Gaming System Status");
    println!("======================");
    
    // Check multilib
    println!("📦 Checking multilib repository...");
    let output = std::process::Command::new("grep")
        .args(&["-E", "^\\[multilib\\]", "/etc/pacman.conf"])
        .output();
    match output {
        Ok(out) if !out.stdout.is_empty() => println!("  ✅ Multilib enabled"),
        _ => println!("  ❌ Multilib disabled (required for 32-bit games)"),
    }

    // Check Steam
    println!("\n🚀 Steam Status:");
    let status = std::process::Command::new("which").arg("steam").status();
    match status {
        Ok(s) if s.success() => {
            println!("  ✅ Steam installed");
            // Check if Steam is running
            let running = std::process::Command::new("pgrep").arg("steam").status();
            match running {
                Ok(s) if s.success() => println!("  🟢 Steam is running"),
                _ => println!("  ⭕ Steam is not running"),
            }
        }
        _ => println!("  ❌ Steam not installed"),
    }

    // Check Lutris
    println!("\n🎯 Lutris Status:");
    let status = std::process::Command::new("which").arg("lutris").status();
    match status {
        Ok(s) if s.success() => println!("  ✅ Lutris installed"),
        _ => println!("  ❌ Lutris not installed"),
    }

    // Check Wine
    println!("\n🍷 Wine Status:");
    let status = std::process::Command::new("which").arg("wine").status();
    match status {
        Ok(s) if s.success() => {
            println!("  ✅ Wine installed");
            let output = std::process::Command::new("wine").arg("--version").output();
            match output {
                Ok(out) => println!("  📋 Version: {}", String::from_utf8_lossy(&out.stdout).trim()),
                _ => {},
            }
        }
        _ => println!("  ❌ Wine not installed"),
    }

    // Check GameMode
    println!("\n⚡ GameMode Status:");
    let status = std::process::Command::new("which").arg("gamemoderun").status();
    match status {
        Ok(s) if s.success() => {
            println!("  ✅ GameMode installed");
            let running = std::process::Command::new("pgrep").arg("gamemode").status();
            match running {
                Ok(s) if s.success() => println!("  🟢 GameMode daemon running"),
                _ => println!("  ⭕ GameMode daemon not running"),
            }
        }
        _ => println!("  ❌ GameMode not installed"),
    }

    // Check MangoHud
    println!("\n📊 MangoHud Status:");
    let status = std::process::Command::new("which").arg("mangohud").status();
    match status {
        Ok(s) if s.success() => println!("  ✅ MangoHud installed"),
        _ => println!("  ❌ MangoHud not installed"),
    }

    // Check Graphics Drivers
    println!("\n🎨 Graphics Status:");
    let output = std::process::Command::new("lspci").args(&["-k"]).output();
    match output {
        Ok(out) => {
            let lspci = String::from_utf8_lossy(&out.stdout);
            if lspci.contains("NVIDIA") {
                println!("  🟢 NVIDIA GPU detected");
                let nvidia_status = std::process::Command::new("nvidia-smi").status();
                match nvidia_status {
                    Ok(s) if s.success() => println!("  ✅ NVIDIA drivers working"),
                    _ => println!("  ❌ NVIDIA drivers not working"),
                }
            }
            if lspci.contains("AMD") || lspci.contains("Radeon") {
                println!("  🔴 AMD GPU detected");
                let amd_status = std::process::Command::new("glxinfo").args(&["|", "grep", "Radeon"]).status();
                match amd_status {
                    Ok(s) if s.success() => println!("  ✅ AMD drivers working"),
                    _ => println!("  ⚠️  AMD driver status unclear"),
                }
            }
            if lspci.contains("Intel") && lspci.contains("Graphics") {
                println!("  🔵 Intel GPU detected");
            }
        }
        _ => println!("  ❌ Could not detect graphics hardware"),
    }

    // Check Vulkan
    println!("\n🌋 Vulkan Status:");
    let status = std::process::Command::new("vulkaninfo").args(&["--summary"]).status();
    match status {
        Ok(s) if s.success() => println!("  ✅ Vulkan working"),
        _ => println!("  ❌ Vulkan not working or not installed"),
    }

    println!("\n📖 For detailed gaming setup, use the automated setup option!");
}