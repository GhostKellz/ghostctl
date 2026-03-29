use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn monitoring_menu() {
    loop {
        let options = [
            "📊 Install Gaming Overlays (MangoHud)",
            "📈 Performance Monitoring Tools",
            "🔍 System Resource Monitoring",
            "🎮 Game-specific Monitoring",
            "⚡ Real-time Performance Analysis",
            "📋 Monitoring Configuration",
            "📊 Monitoring Status",
            "⬅️  Back",
        ];

        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("📊 Gaming Monitoring & Overlays")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match choice {
            0 => install_gaming_overlays(),
            1 => performance_monitoring_tools(),
            2 => system_resource_monitoring(),
            3 => game_specific_monitoring(),
            4 => realtime_performance_analysis(),
            5 => monitoring_configuration(),
            6 => monitoring_status(),
            _ => break,
        }
    }
}

fn install_gaming_overlays() {
    println!("📊 Install Gaming Overlays");
    println!("==========================");

    let overlay_options = [
        "🥭 MangoHud (Universal Overlay)",
        "🎯 DXVK HUD (DirectX Games)",
        "🌋 Vulkan Overlay",
        "🐧 Linux Performance Overlay",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Gaming Overlays")
        .items(&overlay_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => install_mangohud(),
        1 => setup_dxvk_hud(),
        2 => setup_vulkan_overlay(),
        3 => setup_linux_performance_overlay(),
        _ => return,
    }
}

fn install_mangohud() {
    println!("🥭 Installing MangoHud");
    println!("======================");

    let Ok(confirm) = Confirm::new()
        .with_prompt("Install MangoHud and dependencies?")
        .default(true)
        .interact()
    else {
        return;
    };

    if !confirm {
        return;
    }

    // Install MangoHud packages
    let packages = [
        "mangohud",
        "lib32-mangohud",
        "python-mako", // Required for MangoHud
    ];

    println!("📦 Installing MangoHud packages...");
    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&packages)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ MangoHud installed successfully!");

            // Create default config
            create_mangohud_config();

            println!("\n💡 Usage examples:");
            println!("  mangohud <game_command>         # Run game with overlay");
            println!("  MANGOHUD=1 <game_command>       # Alternative method");
            println!("  mangohud steam                  # Steam with overlay");

            let Ok(test_mangohud) = Confirm::new()
                .with_prompt("Test MangoHud with glxgears?")
                .default(false)
                .interact()
            else {
                return;
            };

            if test_mangohud {
                let _ = Command::new("mangohud").arg("glxgears").spawn();
            }
        }
        _ => println!("❌ Failed to install MangoHud"),
    }
}

fn create_mangohud_config() {
    let config_dir = std::env::home_dir()
        .map(|h| h.join(".config/MangoHud"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config/MangoHud"));

    if std::fs::create_dir_all(&config_dir).is_err() {
        println!("❌ Failed to create MangoHud config directory");
        return;
    }

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
wine

## Logging (uncomment to enable)
# output_folder=~/Documents/MangoHud_Logs
# log_duration=30
# autostart_log=1

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

    match File::create(&config_file) {
        Ok(mut file) => {
            if file.write_all(default_config.as_bytes()).is_err() {
                println!("❌ Failed to write MangoHud config");
            } else {
                println!("✅ Created MangoHud config: {}", config_file.display());
                println!("📝 Edit config at: ~/.config/MangoHud/MangoHud.conf");
            }
        }
        Err(_) => println!("❌ Failed to create MangoHud config file"),
    }
}

fn setup_dxvk_hud() {
    println!("🎯 DXVK HUD Setup");
    println!("=================");

    println!("💡 DXVK HUD displays performance info for DirectX games running through DXVK");
    println!("\n🔧 DXVK HUD Environment Variables:");
    println!("  export DXVK_HUD=fps                    # Show FPS only");
    println!("  export DXVK_HUD=full                   # Show all metrics");
    println!("  export DXVK_HUD=fps,memory,gpuload     # Custom metrics");
    println!("  export DXVK_HUD=1                      # Basic overlay");

    println!("\n📊 Available DXVK HUD metrics:");
    println!("  • fps - Frame rate");
    println!("  • frametimes - Frame timing graph");
    println!("  • submissions - GPU submissions");
    println!("  • drawcalls - Draw calls per frame");
    println!("  • pipelines - Pipeline stats");
    println!("  • memory - VRAM usage");
    println!("  • gpuload - GPU utilization");
    println!("  • version - DXVK version");
    println!("  • api - Graphics API");

    let Ok(setup_env) = Confirm::new()
        .with_prompt("Add DXVK HUD environment variables to ~/.profile?")
        .default(false)
        .interact()
    else {
        return;
    };

    if setup_env {
        setup_dxvk_environment();
    }
}

fn setup_dxvk_environment() {
    let dxvk_env = r#"
# DXVK HUD Configuration
export DXVK_HUD=fps,memory,gpuload
export DXVK_LOG_LEVEL=info
"#;

    let profile_path = std::env::home_dir()
        .map(|h| h.join(".profile"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.profile"));

    use std::fs::OpenOptions;
    use std::io::Write;

    if let Ok(mut file) = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&profile_path)
    {
        if writeln!(file, "{}", dxvk_env).is_err() {
            println!("❌ Failed to write to profile");
        } else {
            println!("✅ DXVK HUD environment added to ~/.profile");
            println!("💡 Source ~/.profile or restart shell to apply changes");
        }
    }
}

fn setup_vulkan_overlay() {
    println!("🌋 Vulkan Overlay Setup");
    println!("=======================");

    println!("💡 Vulkan has built-in overlay layers for performance monitoring");
    println!("\n🔧 Vulkan Overlay Environment Variables:");
    println!("  export VK_INSTANCE_LAYERS=VK_LAYER_MESA_overlay");
    println!("  export VK_LAYER_MESA_OVERLAY_CONFIG=fps");
    println!("  export VK_LAYER_MESA_OVERLAY_CONFIG=fps,cpu,gpu");

    println!("\n📊 Available Mesa overlay options:");
    println!("  • fps - Frame rate");
    println!("  • cpu - CPU usage");
    println!("  • gpu - GPU usage");
    println!("  • memory - Memory usage");
    println!("  • io - I/O statistics");

    let Ok(install_vulkan_layers) = Confirm::new()
        .with_prompt("Install Vulkan overlay layers?")
        .default(true)
        .interact()
    else {
        return;
    };

    if install_vulkan_layers {
        let packages = ["vulkan-mesa-layers", "lib32-vulkan-mesa-layers"];

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Vulkan overlay layers installed"),
            _ => println!("❌ Failed to install Vulkan layers"),
        }
    }
}

fn setup_linux_performance_overlay() {
    println!("🐧 Linux Performance Overlay");
    println!("============================");

    println!("💡 Alternative performance overlays for Linux gaming:");

    let overlay_tools = [
        "📊 goverlay - GUI for MangoHud",
        "🎮 CoreCtrl - AMD GPU control with overlay",
        "⚡ gamemode - Performance optimization",
        "📈 htop/btop - System monitoring",
    ];

    println!("\nAvailable tools:");
    for (i, tool) in overlay_tools.iter().enumerate() {
        println!("{}. {}", i + 1, tool);
    }

    let Ok(install_goverlay) = Confirm::new()
        .with_prompt("Install GOverlay (GUI for MangoHud configuration)?")
        .default(false)
        .interact()
    else {
        return;
    };

    if install_goverlay {
        install_goverlay_tool();
    }
}

fn install_goverlay_tool() {
    println!("📊 Installing GOverlay");

    let aur_helpers = ["yay", "paru", "trizen"];
    for helper in &aur_helpers {
        let helper_check = Command::new("which").arg(helper).status();
        if let Ok(s) = helper_check
            && s.success()
        {
            let install_status = Command::new(helper)
                .args(&["-S", "--noconfirm", "goverlay"])
                .status();

            match install_status {
                Ok(s) if s.success() => {
                    println!("✅ GOverlay installed");
                    println!("🎮 Launch with: goverlay");
                    return;
                }
                _ => continue,
            }
        }
    }

    println!("❌ No AUR helper found. Install yay first:");
    println!("   sudo pacman -S --needed base-devel git");
    println!("   git clone https://aur.archlinux.org/yay.git && cd yay && makepkg -si");
}

fn performance_monitoring_tools() {
    println!("📈 Performance Monitoring Tools");
    println!("==============================");

    let monitoring_options = [
        "🖥️  System Monitors (htop, btop, nvtop)",
        "🎮 GPU Monitoring (radeontop, nvtop)",
        "📊 Network Monitoring",
        "💾 Storage I/O Monitoring",
        "🔍 Process Monitoring",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Performance Monitoring Tools")
        .items(&monitoring_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => install_system_monitors(),
        1 => install_gpu_monitoring(),
        2 => install_network_monitoring(),
        3 => install_storage_monitoring(),
        4 => install_process_monitoring(),
        _ => return,
    }
}

fn install_system_monitors() {
    println!("🖥️  Installing System Monitors");
    println!("==============================");

    let system_monitors = [
        ("htop", "Interactive process viewer"),
        ("btop", "Modern system monitor"),
        ("iotop", "I/O monitoring"),
        ("nethogs", "Network usage per process"),
    ];

    println!("Available system monitors:");
    for (i, (tool, desc)) in system_monitors.iter().enumerate() {
        println!("{}. {} - {}", i + 1, tool, desc);
    }

    let Ok(selections) = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select monitors to install")
        .items(
            &system_monitors
                .iter()
                .map(|(tool, desc)| format!("{} - {}", tool, desc))
                .collect::<Vec<_>>(),
        )
        .interact()
    else {
        return;
    };

    if !selections.is_empty() {
        let packages: Vec<&str> = selections.iter().map(|&i| system_monitors[i].0).collect();

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ System monitors installed"),
            _ => println!("❌ Failed to install some monitors"),
        }
    }
}

fn install_gpu_monitoring() {
    println!("🎮 Installing GPU Monitoring Tools");
    println!("==================================");

    let gpu_monitors = [
        ("nvtop", "NVIDIA/AMD GPU monitor"),
        ("radeontop", "AMD GPU monitor"),
        ("intel-gpu-tools", "Intel GPU utilities"),
    ];

    println!("Available GPU monitors:");
    for (i, (tool, desc)) in gpu_monitors.iter().enumerate() {
        println!("{}. {} - {}", i + 1, tool, desc);
    }

    let Ok(selections) = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select GPU monitors to install")
        .items(
            &gpu_monitors
                .iter()
                .map(|(tool, desc)| format!("{} - {}", tool, desc))
                .collect::<Vec<_>>(),
        )
        .interact()
    else {
        return;
    };

    for &index in &selections {
        let (package, _) = gpu_monitors[index];
        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm", package])
            .status();

        match status {
            Ok(s) if s.success() => println!("  ✅ {} installed", package),
            _ => println!("  💡 {} not available in repos", package),
        }
    }
}

fn install_network_monitoring() {
    println!("📊 Installing Network Monitoring Tools");
    println!("======================================");

    let network_tools = ["nethogs", "iftop", "bandwhich", "nload"];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&network_tools)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Network monitoring tools installed");
            println!("💡 Usage:");
            println!("  nethogs - Per-process bandwidth usage");
            println!("  iftop - Network interface monitoring");
            println!("  nload - Network traffic visualization");
        }
        _ => println!("❌ Failed to install network monitoring tools"),
    }
}

fn install_storage_monitoring() {
    println!("💾 Installing Storage I/O Monitoring Tools");
    println!("===========================================");

    let storage_tools = [
        ("iotop", "I/O usage by process"),
        ("iostat", "I/O statistics"),
        ("dstat", "System resource statistics"),
    ];

    let packages: Vec<&str> = storage_tools.iter().map(|(pkg, _)| *pkg).collect();

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&packages)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Storage monitoring tools installed");
            for (tool, desc) in &storage_tools {
                println!("  {} - {}", tool, desc);
            }
        }
        _ => println!("❌ Failed to install storage monitoring tools"),
    }
}

fn install_process_monitoring() {
    println!("🔍 Installing Process Monitoring Tools");
    println!("======================================");

    let process_tools = [
        ("lsof", "List open files"),
        ("strace", "System call tracer"),
        ("perf", "Performance analysis"),
        ("sysstat", "System activity reporter"),
    ];

    let packages: Vec<&str> = process_tools.iter().map(|(pkg, _)| *pkg).collect();

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&packages)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Process monitoring tools installed");
            for (tool, desc) in &process_tools {
                println!("  {} - {}", tool, desc);
            }
        }
        _ => println!("❌ Failed to install process monitoring tools"),
    }
}

fn system_resource_monitoring() {
    println!("🔍 System Resource Monitoring");
    println!("=============================");

    let resource_options = [
        "📊 Real-time System Overview",
        "🖥️  CPU Monitoring",
        "💾 Memory Monitoring",
        "💿 Disk I/O Monitoring",
        "🌐 Network Monitoring",
        "🌡️  Temperature Monitoring",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("System Resource Monitoring")
        .items(&resource_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => realtime_system_overview(),
        1 => cpu_monitoring(),
        2 => memory_monitoring(),
        3 => disk_io_monitoring(),
        4 => network_resource_monitoring(),
        5 => temperature_monitoring(),
        _ => return,
    }
}

fn realtime_system_overview() {
    println!("📊 Real-time System Overview");
    println!("============================");

    let overview_tools = [
        "🖥️  Launch htop",
        "📊 Launch btop",
        "⚡ Launch nvtop (GPU)",
        "🌐 Launch nethogs (Network)",
        "💾 Launch iotop (I/O)",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("System Overview Tools")
        .items(&overview_tools)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => {
            let _ = Command::new("htop").status();
        }
        1 => {
            let _ = Command::new("btop").status();
        }
        2 => {
            let _ = Command::new("nvtop").status();
        }
        3 => {
            let _ = Command::new("sudo").arg("nethogs").status();
        }
        4 => {
            let _ = Command::new("sudo").arg("iotop").status();
        }
        _ => return,
    }
}

fn cpu_monitoring() {
    println!("🖥️  CPU Monitoring");
    println!("==================");

    println!("📊 CPU Information:");
    let _ = Command::new("lscpu").status();

    println!("\n⚡ CPU Usage:");
    let _ = Command::new("top").args(&["-n", "1", "-b"]).status();

    println!("\n🌡️  CPU Temperature:");
    let _ = Command::new("sensors").status();
}

fn memory_monitoring() {
    println!("💾 Memory Monitoring");
    println!("===================");

    println!("📊 Memory Usage:");
    let _ = Command::new("free").arg("-h").status();

    println!("\n💿 Virtual Memory Statistics:");
    let _ = Command::new("vmstat").status();

    println!("\n📈 Memory by Process:");
    let _ = Command::new("ps")
        .args(&["aux", "--sort=-%mem"])
        .args(&["|", "head", "-20"])
        .status();
}

fn disk_io_monitoring() {
    println!("💿 Disk I/O Monitoring");
    println!("======================");

    println!("📊 Disk Usage:");
    let _ = Command::new("df").arg("-h").status();

    println!("\n⚡ I/O Statistics:");
    let _ = Command::new("iostat").args(&["-x", "1", "1"]).status();

    println!("\n🔍 I/O by Process:");
    let _ = Command::new("sudo")
        .arg("iotop")
        .args(&["-o", "-n", "1"])
        .status();
}

fn network_resource_monitoring() {
    println!("🌐 Network Resource Monitoring");
    println!("==============================");

    println!("📊 Network Interfaces:");
    let _ = Command::new("ip").arg("addr").status();

    println!("\n📈 Network Statistics:");
    let _ = Command::new("ss").args(&["-tuln"]).status();

    println!("\n⚡ Network Usage by Process:");
    let _ = Command::new("sudo")
        .arg("nethogs")
        .args(&["-d", "1"])
        .status();
}

fn temperature_monitoring() {
    println!("🌡️  Temperature Monitoring");
    println!("===========================");

    // Check if lm_sensors is installed
    let sensors_check = Command::new("which").arg("sensors").status();
    match sensors_check {
        Ok(s) if s.success() => {
            println!("📊 System Temperatures:");
            let _ = Command::new("sensors").status();
        }
        _ => {
            println!("❌ lm_sensors not installed");
            let Ok(install) = Confirm::new()
                .with_prompt("Install lm_sensors for temperature monitoring?")
                .default(true)
                .interact()
            else {
                return;
            };

            if install {
                let status = Command::new("sudo")
                    .args(&["pacman", "-S", "--needed", "--noconfirm", "lm_sensors"])
                    .status();

                match status {
                    Ok(s) if s.success() => {
                        println!("✅ lm_sensors installed");
                        println!("🔧 Run 'sudo sensors-detect' to configure");
                    }
                    _ => println!("❌ Failed to install lm_sensors"),
                }
            }
        }
    }
}

fn game_specific_monitoring() {
    println!("🎮 Game-specific Monitoring");
    println!("===========================");

    let game_monitoring_options = [
        "🎯 Steam Game Monitoring",
        "🍷 Wine/Proton Game Monitoring",
        "📱 Native Linux Game Monitoring",
        "🔍 Game Process Analysis",
        "📊 Game Performance Logging",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Game-specific Monitoring")
        .items(&game_monitoring_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => steam_game_monitoring(),
        1 => wine_proton_monitoring(),
        2 => native_linux_monitoring(),
        3 => game_process_analysis(),
        4 => game_performance_logging(),
        _ => return,
    }
}

fn steam_game_monitoring() {
    println!("🎯 Steam Game Monitoring");
    println!("========================");

    println!("💡 Steam game monitoring methods:");
    println!("  • Steam built-in FPS counter (Settings > In-Game)");
    println!("  • MangoHud overlay for detailed metrics");
    println!("  • Proton logging for compatibility issues");

    println!("\n🔧 Steam launch options for monitoring:");
    println!("  mangohud %command%                    # MangoHud overlay");
    println!("  DXVK_HUD=fps %command%                # DXVK FPS counter");
    println!("  PROTON_LOG=1 %command%                # Proton logging");
    println!("  gamemoderun mangohud %command%        # GameMode + MangoHud");

    println!("\n📊 Steam game performance data locations:");
    println!("  ~/.steam/steam/logs/                  # Steam logs");
    println!("  ~/.steam/steam/steamapps/compatdata/  # Proton prefixes");
}

fn wine_proton_monitoring() {
    println!("🍷 Wine/Proton Game Monitoring");
    println!("==============================");

    println!("💡 Wine/Proton monitoring tools:");
    println!("  • WINEDEBUG environment variables");
    println!("  • Proton logging");
    println!("  • DXVK/VKD3D performance overlays");

    println!("\n🔧 Useful environment variables:");
    println!("  export WINEDEBUG=+fps,+d3d           # Wine FPS and D3D debugging");
    println!("  export PROTON_LOG=1                  # Enable Proton logging");
    println!("  export DXVK_LOG_LEVEL=info           # DXVK debugging");
    println!("  export VKD3D_DEBUG=warn              # VKD3D debugging");

    let Ok(setup_wine_monitoring) = Confirm::new()
        .with_prompt("Setup Wine/Proton monitoring environment?")
        .default(false)
        .interact()
    else {
        return;
    };

    if setup_wine_monitoring {
        setup_wine_monitoring_env();
    }
}

fn setup_wine_monitoring_env() {
    let wine_monitoring_env = r#"
# Wine/Proton Monitoring Environment
export PROTON_LOG=1
export DXVK_LOG_LEVEL=info
export VKD3D_DEBUG=warn
export WINEDEBUG=+fps
"#;

    let profile_path = std::env::home_dir()
        .map(|h| h.join(".profile"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.profile"));

    use std::fs::OpenOptions;
    use std::io::Write;

    if let Ok(mut file) = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&profile_path)
    {
        if writeln!(file, "{}", wine_monitoring_env).is_err() {
            println!("❌ Failed to write to profile");
        } else {
            println!("✅ Wine monitoring environment added to ~/.profile");
        }
    }
}

fn native_linux_monitoring() {
    println!("📱 Native Linux Game Monitoring");
    println!("===============================");

    println!("💡 Native Linux game monitoring:");
    println!("  • MangoHud works with OpenGL/Vulkan games");
    println!("  • System monitoring tools for overall performance");
    println!("  • Game-specific profiling tools");

    println!("\n🎮 Popular native Linux games and monitoring:");
    println!("  • CS2: mangohud steam steam://rungameid/730");
    println!("  • Rocket League: mangohud <game_executable>");
    println!("  • Vulkan games: VK_INSTANCE_LAYERS=VK_LAYER_MESA_overlay");
}

fn game_process_analysis() {
    println!("🔍 Game Process Analysis");
    println!("========================");

    println!("💡 Game process analysis tools:");
    println!("  • htop/btop - Real-time process monitoring");
    println!("  • strace - System call tracing");
    println!("  • perf - Performance profiling");
    println!("  • gdb - Game debugging");

    println!("\n🔧 Useful commands:");
    println!("  htop -p $(pgrep <game_name>)          # Monitor specific game");
    println!("  strace -p $(pgrep <game_name>)        # Trace system calls");
    println!("  perf record -p $(pgrep <game_name>)   # Performance profiling");

    let Ok(analyze_running_games) = Confirm::new()
        .with_prompt("Show currently running games?")
        .default(true)
        .interact()
    else {
        return;
    };

    if analyze_running_games {
        show_running_games();
    }
}

fn show_running_games() {
    println!("🎮 Currently Running Games:");
    println!("===========================");

    let game_processes = [
        "steam",
        "steamwebhelper",
        "csgo",
        "cs2",
        "dota2",
        "tf2",
        "wine",
        "lutris",
        "heroic",
        "minecraft",
        "java",
    ];

    for game in &game_processes {
        let pgrep_output = Command::new("pgrep").args(&["-l", game]).output();

        if let Ok(output) = pgrep_output {
            let processes = String::from_utf8_lossy(&output.stdout);
            if !processes.trim().is_empty() {
                println!("🎮 {}: {}", game, processes.trim());
            }
        }
    }
}

fn game_performance_logging() {
    println!("📊 Game Performance Logging");
    println!("===========================");

    let logging_options = [
        "📝 Setup MangoHud Logging",
        "📊 Create Performance Log Scripts",
        "📈 Analyze Existing Logs",
        "🗑️  Clean Old Log Files",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Performance Logging")
        .items(&logging_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => setup_mangohud_logging(),
        1 => create_performance_scripts(),
        2 => analyze_performance_logs(),
        3 => clean_log_files(),
        _ => return,
    }
}

fn setup_mangohud_logging() {
    println!("📝 Setting up MangoHud Logging");
    println!("==============================");

    let logs_dir = std::env::home_dir()
        .map(|h| h.join("Documents/MangoHud_Logs"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/Documents/MangoHud_Logs"));

    if std::fs::create_dir_all(&logs_dir).is_err() {
        println!("❌ Failed to create logs directory");
        return;
    }

    println!("✅ Created logs directory: {}", logs_dir.display());

    let config_addition = format!(
        r#"
# Performance Logging Configuration
output_folder={}
log_duration=30
autostart_log=1
toggle_logging=F10
"#,
        logs_dir.display()
    );

    println!("📝 Add to MangoHud config (~/.config/MangoHud/MangoHud.conf):");
    println!("{}", config_addition);

    let Ok(update_config) = Confirm::new()
        .with_prompt("Update MangoHud config with logging settings?")
        .default(true)
        .interact()
    else {
        return;
    };

    if update_config {
        update_mangohud_config_with_logging(&config_addition);
    }
}

fn update_mangohud_config_with_logging(config_addition: &str) {
    let config_file = std::env::home_dir()
        .map(|h| h.join(".config/MangoHud/MangoHud.conf"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config/MangoHud/MangoHud.conf"));

    use std::fs::OpenOptions;
    use std::io::Write;

    if let Ok(mut file) = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&config_file)
    {
        if writeln!(file, "{}", config_addition).is_err() {
            println!("❌ Failed to update MangoHud config");
        } else {
            println!("✅ MangoHud config updated with logging settings");
        }
    }
}

fn create_performance_scripts() {
    println!("📊 Creating Performance Log Scripts");
    println!("===================================");

    let scripts_dir = std::env::home_dir()
        .map(|h| h.join("bin"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/bin"));

    if std::fs::create_dir_all(&scripts_dir).is_err() {
        println!("❌ Failed to create scripts directory");
        return;
    }

    let perf_script = scripts_dir.join("game-performance-monitor.sh");
    let script_content = r#"#!/bin/bash
# Game Performance Monitoring Script

GAME_NAME="$1"
LOG_DIR="$HOME/Documents/GameLogs"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

if [ -z "$GAME_NAME" ]; then
    echo "Usage: $0 <game_name>"
    exit 1
fi

mkdir -p "$LOG_DIR"

echo "Starting performance monitoring for: $GAME_NAME"
echo "Logs will be saved to: $LOG_DIR"

# Start monitoring in background
{
    echo "=== Performance Monitor Started at $(date) ===" 
    echo "Game: $GAME_NAME"
    echo "PID: $$"
    echo
    
    while true; do
        echo "=== $(date) ==="
        echo "CPU Usage:"
        top -bn1 | grep "$GAME_NAME" | head -5
        echo
        echo "Memory Usage:"
        ps aux | grep "$GAME_NAME" | grep -v grep
        echo
        echo "GPU Usage:"
        nvidia-smi --query-gpu=utilization.gpu,memory.used,temperature.gpu --format=csv,noheader,nounits 2>/dev/null || echo "NVIDIA GPU not available"
        echo "---"
        sleep 5
    done
} > "$LOG_DIR/${GAME_NAME}_${TIMESTAMP}.log" &

MONITOR_PID=$!
echo "Performance monitor started with PID: $MONITOR_PID"
echo "To stop monitoring: kill $MONITOR_PID"
echo "Log file: $LOG_DIR/${GAME_NAME}_${TIMESTAMP}.log"
"#;

    use std::fs::File;
    use std::io::Write;

    match File::create(&perf_script) {
        Ok(mut file) => {
            if file.write_all(script_content.as_bytes()).is_err() {
                println!("❌ Failed to write performance script");
            } else {
                // Make script executable
                let _ = Command::new("chmod")
                    .args(&["+x", &perf_script.to_string_lossy()])
                    .status();
                println!(
                    "✅ Created performance monitoring script: {}",
                    perf_script.display()
                );
                println!("💡 Usage: {} <game_name>", perf_script.display());
            }
        }
        Err(_) => println!("❌ Failed to create performance script"),
    }
}

fn analyze_performance_logs() {
    println!("📈 Analyzing Performance Logs");
    println!("=============================");

    let logs_dirs = [
        "~/Documents/MangoHud_Logs",
        "~/Documents/GameLogs",
        "~/.local/share/lutris/logs",
        "~/.steam/steam/logs",
    ];

    println!("🔍 Checking for log files in:");
    for log_dir in &logs_dirs {
        let expanded_path = if log_dir.starts_with("~/") {
            std::env::home_dir()
                .map(|h| h.join(&log_dir[2..]))
                .unwrap_or_else(|| std::path::PathBuf::from(log_dir))
        } else {
            std::path::PathBuf::from(log_dir)
        };

        if expanded_path.exists() {
            println!("  ✅ {}", expanded_path.display());
            let _ = Command::new("ls")
                .args(&["-la", &expanded_path.to_string_lossy()])
                .status();
        } else {
            println!("  ❌ {} (not found)", expanded_path.display());
        }
    }
}

fn clean_log_files() {
    println!("🗑️  Cleaning Old Log Files");
    println!("==========================");

    let logs_dirs = ["~/Documents/MangoHud_Logs", "~/Documents/GameLogs"];

    for log_dir in &logs_dirs {
        let expanded_path = if log_dir.starts_with("~/") {
            std::env::home_dir()
                .map(|h| h.join(&log_dir[2..]))
                .unwrap_or_else(|| std::path::PathBuf::from(log_dir))
        } else {
            std::path::PathBuf::from(log_dir)
        };

        if expanded_path.exists() {
            println!("📁 Checking: {}", expanded_path.display());

            let Ok(confirm) = Confirm::new()
                .with_prompt(&format!(
                    "Clean log files older than 7 days in {}?",
                    expanded_path.display()
                ))
                .default(false)
                .interact()
            else {
                continue;
            };

            if confirm {
                let status = Command::new("find")
                    .args(&[
                        &expanded_path.to_string_lossy(),
                        "-name",
                        "*.log",
                        "-mtime",
                        "+7",
                        "-delete",
                    ])
                    .status();

                match status {
                    Ok(s) if s.success() => {
                        println!("✅ Cleaned old logs from {}", expanded_path.display())
                    }
                    _ => println!("❌ Failed to clean logs from {}", expanded_path.display()),
                }
            }
        }
    }
}

fn realtime_performance_analysis() {
    println!("⚡ Real-time Performance Analysis");
    println!("=================================");

    let analysis_options = [
        "🎮 Launch Game with Full Monitoring",
        "📊 Custom Performance Dashboard",
        "🔍 Performance Bottleneck Analysis",
        "📈 Real-time Metrics Viewer",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Real-time Performance Analysis")
        .items(&analysis_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => launch_game_with_monitoring(),
        1 => custom_performance_dashboard(),
        2 => performance_bottleneck_analysis(),
        3 => realtime_metrics_viewer(),
        _ => return,
    }
}

fn launch_game_with_monitoring() {
    println!("🎮 Launch Game with Full Monitoring");
    println!("===================================");

    println!("💡 This will launch a game with comprehensive monitoring:");
    println!("  • MangoHud overlay for in-game metrics");
    println!("  • GameMode for performance optimization");
    println!("  • System monitoring in terminal");
    println!("  • Performance logging enabled");

    use dialoguer::Input;
    let Ok(game_command) = Input::<String>::new()
        .with_prompt("Enter game command (e.g., 'steam', 'lutris', or game executable)")
        .interact_text()
    else {
        return;
    };

    if !game_command.trim().is_empty() {
        println!("🚀 Launching {} with full monitoring...", game_command);

        // Launch with monitoring
        let monitoring_command = format!("gamemoderun mangohud {}", game_command);
        let _ = Command::new("sh")
            .arg("-c")
            .arg(&monitoring_command)
            .spawn();

        println!("✅ Game launched with monitoring");
        println!("💡 Check MangoHud overlay in-game (F12 to toggle)");
    }
}

fn custom_performance_dashboard() {
    println!("📊 Custom Performance Dashboard");
    println!("==============================");

    println!("💡 Creating a custom performance monitoring dashboard...");
    println!("This feature would show real-time system metrics in a dashboard format");
    println!("🚧 Feature under development");
}

fn performance_bottleneck_analysis() {
    println!("🔍 Performance Bottleneck Analysis");
    println!("==================================");

    println!("💡 Analyzing potential performance bottlenecks:");

    println!("\n📊 CPU Analysis:");
    let _ = Command::new("lscpu").status();

    println!("\n💾 Memory Analysis:");
    let _ = Command::new("free").arg("-h").status();

    println!("\n🎮 GPU Analysis:");
    let _ = Command::new("lspci")
        .args(&["|", "grep", "-i", "vga"])
        .status();

    println!("\n💿 Storage Analysis:");
    let _ = Command::new("df").arg("-h").status();
}

fn realtime_metrics_viewer() {
    println!("📈 Real-time Metrics Viewer");
    println!("===========================");

    let viewer_options = [
        "🖥️  System Overview (htop)",
        "🎮 GPU Monitoring (nvtop)",
        "🌐 Network Usage (nethogs)",
        "💿 Disk I/O (iotop)",
        "📊 All Metrics (tmux session)",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Real-time Metrics Viewer")
        .items(&viewer_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => {
            let _ = Command::new("htop").status();
        }
        1 => {
            let _ = Command::new("nvtop").status();
        }
        2 => {
            let _ = Command::new("sudo").arg("nethogs").status();
        }
        3 => {
            let _ = Command::new("sudo").arg("iotop").status();
        }
        4 => launch_monitoring_session(),
        _ => return,
    }
}

fn launch_monitoring_session() {
    println!("📊 Launching Multi-Panel Monitoring Session");
    println!("===========================================");

    let tmux_check = Command::new("which").arg("tmux").status();
    match tmux_check {
        Ok(s) if s.success() => {
            println!("🚀 Starting tmux monitoring session...");
            let _ = Command::new("tmux")
                .args(&[
                    "new-session",
                    "-d",
                    "-s",
                    "gaming-monitor",
                    "htop",
                    ";",
                    "split-window",
                    "-v",
                    "nvtop",
                    ";",
                    "split-window",
                    "-h",
                    "sudo",
                    "nethogs",
                    ";",
                    "attach-session",
                    "-t",
                    "gaming-monitor",
                ])
                .status();
        }
        _ => {
            println!("❌ tmux not found");
            let Ok(install) = Confirm::new()
                .with_prompt("Install tmux for multi-panel monitoring?")
                .default(true)
                .interact()
            else {
                return;
            };

            if install {
                let status = Command::new("sudo")
                    .args(&["pacman", "-S", "--needed", "--noconfirm", "tmux"])
                    .status();

                match status {
                    Ok(s) if s.success() => {
                        println!("✅ tmux installed");
                        launch_monitoring_session();
                    }
                    _ => println!("❌ Failed to install tmux"),
                }
            }
        }
    }
}

fn monitoring_configuration() {
    println!("📋 Monitoring Configuration");
    println!("===========================");

    let config_options = [
        "🥭 MangoHud Configuration",
        "🎯 DXVK HUD Configuration",
        "🌋 Vulkan Overlay Configuration",
        "📊 System Monitoring Configuration",
        "📝 Logging Configuration",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Monitoring Configuration")
        .items(&config_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => mangohud_configuration(),
        1 => dxvk_hud_configuration(),
        2 => vulkan_overlay_configuration(),
        3 => system_monitoring_configuration(),
        4 => logging_configuration(),
        _ => return,
    }
}

fn mangohud_configuration() {
    println!("🥭 MangoHud Configuration");
    println!("=========================");

    let config_file = std::env::home_dir()
        .map(|h| h.join(".config/MangoHud/MangoHud.conf"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config/MangoHud/MangoHud.conf"));

    if config_file.exists() {
        println!("✅ MangoHud config found: {}", config_file.display());

        let Ok(edit_config) = Confirm::new()
            .with_prompt("Edit MangoHud configuration?")
            .default(false)
            .interact()
        else {
            return;
        };

        if edit_config {
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
            let _ = Command::new(&editor).arg(&config_file).status();
        }
    } else {
        println!("❌ MangoHud config not found");
        let Ok(create_config) = Confirm::new()
            .with_prompt("Create default MangoHud configuration?")
            .default(true)
            .interact()
        else {
            return;
        };

        if create_config {
            create_mangohud_config();
        }
    }

    println!("\n💡 MangoHud configuration tips:");
    println!("  • fps - Show frame rate");
    println!("  • gpu_stats - GPU utilization");
    println!("  • cpu_stats - CPU utilization");
    println!("  • position=top-left - Overlay position");
    println!("  • toggle_hud=F12 - Toggle key");
}

fn dxvk_hud_configuration() {
    println!("🎯 DXVK HUD Configuration");
    println!("=========================");

    println!("💡 DXVK HUD environment variables:");
    println!("  export DXVK_HUD=fps                    # FPS only");
    println!("  export DXVK_HUD=full                   # All metrics");
    println!("  export DXVK_HUD=fps,memory,gpuload     # Custom selection");

    println!("\n📊 Available DXVK HUD options:");
    println!("  • fps - Frame rate");
    println!("  • frametimes - Frame timing");
    println!("  • submissions - GPU submissions");
    println!("  • drawcalls - Draw calls");
    println!("  • pipelines - Pipeline stats");
    println!("  • memory - VRAM usage");
    println!("  • gpuload - GPU utilization");
    println!("  • version - DXVK version");
}

fn vulkan_overlay_configuration() {
    println!("🌋 Vulkan Overlay Configuration");
    println!("===============================");

    println!("💡 Vulkan overlay environment variables:");
    println!("  export VK_INSTANCE_LAYERS=VK_LAYER_MESA_overlay");
    println!("  export VK_LAYER_MESA_OVERLAY_CONFIG=fps,cpu,gpu");

    println!("\n📊 Mesa overlay configuration options:");
    println!("  • fps - Frame rate");
    println!("  • cpu - CPU usage");
    println!("  • gpu - GPU usage");
    println!("  • memory - Memory usage");
    println!("  • io - I/O statistics");
}

fn system_monitoring_configuration() {
    println!("📊 System Monitoring Configuration");
    println!("==================================");

    println!("💡 System monitoring tool configurations:");
    println!("  • htop: ~/.config/htop/htoprc");
    println!("  • btop: ~/.config/btop/btop.conf");
    println!("  • sensors: /etc/sensors3.conf");

    let Ok(configure_htop) = Confirm::new()
        .with_prompt("Launch htop configuration?")
        .default(false)
        .interact()
    else {
        return;
    };

    if configure_htop {
        let _ = Command::new("htop").status();
        println!("💡 Press F2 in htop to access configuration");
    }
}

fn logging_configuration() {
    println!("📝 Logging Configuration");
    println!("========================");

    println!("💡 Gaming log locations:");
    println!("  • MangoHud: ~/Documents/MangoHud_Logs/");
    println!("  • Steam: ~/.steam/steam/logs/");
    println!("  • Lutris: ~/.local/share/lutris/logs/");
    println!("  • Wine: ~/.wine/");

    let log_dirs = ["~/Documents/MangoHud_Logs", "~/Documents/GameLogs"];

    for log_dir in &log_dirs {
        let expanded_path = if log_dir.starts_with("~/") {
            std::env::home_dir()
                .map(|h| h.join(&log_dir[2..]))
                .unwrap_or_else(|| std::path::PathBuf::from(log_dir))
        } else {
            std::path::PathBuf::from(log_dir)
        };

        if !expanded_path.exists() {
            println!("📁 Creating log directory: {}", expanded_path.display());
            let _ = std::fs::create_dir_all(&expanded_path);
        }
    }
}

fn monitoring_status() {
    println!("📊 Gaming Monitoring Status");
    println!("===========================");

    println!("🔧 Installed Monitoring Tools:");
    let monitoring_tools = [
        ("MangoHud", "mangohud"),
        ("nvtop", "nvtop"),
        ("htop", "htop"),
        ("btop", "btop"),
        ("radeontop", "radeontop"),
        ("iotop", "iotop"),
        ("nethogs", "nethogs"),
    ];

    for (name, cmd) in &monitoring_tools {
        let status = Command::new("which").arg(cmd).status();
        match status {
            Ok(s) if s.success() => println!("  ✅ {} available", name),
            _ => println!("  ❌ {} not found", name),
        }
    }

    println!("\n📁 Configuration Files:");
    let config_files = [
        ("MangoHud", "~/.config/MangoHud/MangoHud.conf"),
        ("htop", "~/.config/htop/htoprc"),
        ("btop", "~/.config/btop/btop.conf"),
    ];

    for (tool, config_path) in &config_files {
        let expanded_path = if config_path.starts_with("~/") {
            std::env::home_dir()
                .map(|h| h.join(&config_path[2..]))
                .unwrap_or_else(|| std::path::PathBuf::from(config_path))
        } else {
            std::path::PathBuf::from(config_path)
        };

        if expanded_path.exists() {
            println!("  ✅ {} config: {}", tool, expanded_path.display());
        } else {
            println!("  ❌ {} config not found", tool);
        }
    }

    println!("\n📊 Active Monitoring Processes:");
    let monitoring_processes = ["mangohud", "nvtop", "htop", "btop", "radeontop"];
    for process in &monitoring_processes {
        let pgrep_output = Command::new("pgrep").args(&["-l", process]).output();
        if let Ok(output) = pgrep_output {
            let processes = String::from_utf8_lossy(&output.stdout);
            if !processes.trim().is_empty() {
                println!("  🟢 {} running: {}", process, processes.trim());
            }
        }
    }
}
