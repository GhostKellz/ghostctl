use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn performance_menu() {
    loop {
        let options = [
            "⚡ System Performance Optimization",
            "🎮 Gaming-specific Performance Tuning",
            "🖥️  GPU Performance & Overclocking",
            "💾 Memory & Storage Optimization",
            "🌡️  Thermal Management",
            "📊 Performance Monitoring & Benchmarking",
            "🔧 Custom Performance Profiles",
            "🚀 Automatic Game Optimization",
            "📋 Performance Status Report",
            "⬅️  Back",
        ];

        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("⚡ Performance Optimization")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match choice {
            0 => system_performance_optimization(),
            1 => gaming_performance_tuning(),
            2 => gpu_performance_overclocking(),
            3 => memory_storage_optimization_menu(),
            4 => thermal_management(),
            5 => performance_monitoring_benchmarking(),
            6 => custom_performance_profiles(),
            7 => automatic_game_optimization(),
            8 => performance_status_report(),
            _ => break,
        }
    }
}

fn system_performance_optimization() {
    println!("⚡ System Performance Optimization");
    println!("==================================");

    let optimizations = [
        "🚀 Enable GameMode for gaming",
        "⚡ Configure CPU governor",
        "💾 Optimize memory management",
        "🔧 Kernel parameter tuning",
        "🖥️  Desktop environment optimizations",
        "📁 File system optimizations",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("System Optimizations")
        .items(&optimizations)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => enable_gamemode(),
        1 => configure_cpu_governor(),
        2 => optimize_memory_management(),
        3 => kernel_parameter_tuning(),
        4 => desktop_environment_optimizations(),
        5 => filesystem_optimizations(),
        _ => return,
    }
}

fn enable_gamemode() {
    println!("🚀 Enable GameMode for Gaming");
    println!("=============================");

    let gamemode_check = Command::new("which").arg("gamemoderun").status();
    match gamemode_check {
        Ok(s) if s.success() => {
            println!("✅ GameMode is already installed");

            // Check if gamemode daemon is running
            let daemon_check = Command::new("pgrep").arg("gamemode").status();
            match daemon_check {
                Ok(s) if s.success() => println!("🟢 GameMode daemon is running"),
                _ => {
                    println!("⚠️  GameMode daemon not running");
                    let Ok(start_daemon) = Confirm::new()
                        .with_prompt("Start GameMode daemon?")
                        .default(true)
                        .interact()
                    else {
                        return;
                    };

                    if start_daemon {
                        if let Err(e) = Command::new("systemctl")
                            .args(["--user", "start", "gamemode"])
                            .status()
                        {
                            eprintln!("Failed to start gamemode daemon: {}", e);
                        }
                    }
                }
            }
        }
        _ => {
            println!("❌ GameMode not installed");
            let Ok(install) = Confirm::new()
                .with_prompt("Install GameMode?")
                .default(true)
                .interact()
            else {
                return;
            };

            if install {
                let status = Command::new("sudo")
                    .args(["pacman", "-S", "--needed", "--noconfirm", "gamemode"])
                    .status();

                match status {
                    Ok(s) if s.success() => {
                        println!("✅ GameMode installed successfully!");

                        // Add user to gamemode group
                        let username = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
                        if let Err(e) = Command::new("sudo")
                            .args(["usermod", "-a", "-G", "gamemode", &username])
                            .status()
                        {
                            eprintln!("Failed to add user to gamemode group: {}", e);
                        }

                        println!(
                            "💡 You may need to log out and back in for group membership to take effect"
                        );
                        println!("🎮 Use: gamemoderun <game_command> to run games with GameMode");
                    }
                    _ => println!("❌ Failed to install GameMode"),
                }
            }
        }
    }

    println!("\n💡 GameMode usage examples:");
    println!("  gamemoderun steam");
    println!("  gamemoderun lutris");
    println!("  gamemoderun <game_executable>");
    println!("  GAMEMODERUNEXEC=gamemoderun <launch_command>");
}

fn configure_cpu_governor() {
    println!("⚡ Configure CPU Governor");
    println!("=========================");

    println!("📊 Current CPU governor:");
    if let Err(e) = Command::new("cat")
        .arg("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
        .status()
    {
        eprintln!("Failed to read CPU governor: {}", e);
    }

    println!("\n📋 Available governors:");
    if let Err(e) = Command::new("cat")
        .arg("/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors")
        .status()
    {
        eprintln!("Failed to read available governors: {}", e);
    }

    let governors = [
        ("performance", "Maximum performance (highest frequency)"),
        ("powersave", "Power saving (lowest frequency)"),
        ("ondemand", "Dynamic scaling based on load"),
        ("conservative", "Gradual frequency scaling"),
        ("schedutil", "Scheduler-guided scaling (recommended)"),
    ];

    println!("\n🔧 Governor descriptions:");
    for (gov, desc) in &governors {
        println!("  {} - {}", gov, desc);
    }

    let Ok(governor_choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select CPU governor")
        .items(
            &governors
                .iter()
                .map(|(name, desc)| format!("{} - {}", name, desc))
                .collect::<Vec<_>>(),
        )
        .default(0)
        .interact()
    else {
        return;
    };

    let selected_governor = governors[governor_choice].0;

    let Ok(confirm) = Confirm::new()
        .with_prompt(&format!("Set CPU governor to '{}'?", selected_governor))
        .default(true)
        .interact()
    else {
        return;
    };

    if confirm {
        // Temporary change
        let status = Command::new("sudo")
            .arg("sh")
            .arg("-c")
            .arg(&format!(
                "echo {} | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor",
                selected_governor
            ))
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ CPU governor set to '{}'", selected_governor);

                let Ok(permanent) = Confirm::new()
                    .with_prompt("Make this change permanent? (adds to /etc/default/cpupower)")
                    .default(false)
                    .interact()
                else {
                    return;
                };

                if permanent {
                    make_cpu_governor_permanent(selected_governor);
                }
            }
            _ => println!("❌ Failed to set CPU governor"),
        }
    }
}

fn make_cpu_governor_permanent(governor: &str) {
    // Install cpupower if not available
    let cpupower_check = Command::new("which").arg("cpupower").status();
    if cpupower_check.is_err() {
        println!("📦 Installing cpupower...");
        if let Err(e) = Command::new("sudo")
            .args(["pacman", "-S", "--needed", "--noconfirm", "cpupower"])
            .status()
        {
            eprintln!("Failed to install cpupower: {}", e);
        }
    }

    // Create/update cpupower configuration
    let config_content = format!("# CPU governor configuration\ngovernor='{}'\n", governor);
    let status = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(&format!(
            "echo '{}' >> /etc/default/cpupower",
            config_content
        ))
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ CPU governor configuration saved");

            // Enable cpupower service
            if let Err(e) = Command::new("sudo")
                .args(["systemctl", "enable", "cpupower"])
                .status()
            {
                eprintln!("Failed to enable cpupower service: {}", e);
            }
        }
        _ => println!("❌ Failed to save CPU governor configuration"),
    }
}

fn optimize_memory_management() {
    println!("💾 Optimize Memory Management");
    println!("=============================");

    let memory_optimizations = [
        "🔧 Configure swappiness",
        "💾 Enable zram compression",
        "🧹 Clear memory caches",
        "📊 Memory usage analysis",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Memory Optimizations")
        .items(&memory_optimizations)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => configure_swappiness(),
        1 => enable_zram(),
        2 => clear_memory_caches(),
        3 => memory_usage_analysis(),
        _ => return,
    }
}

fn configure_swappiness() {
    println!("🔧 Configure Swappiness");
    println!("=======================");

    println!("📊 Current swappiness value:");
    if let Err(e) = Command::new("cat").arg("/proc/sys/vm/swappiness").status() {
        eprintln!("Failed to read swappiness: {}", e);
    }

    println!("\n💡 Swappiness values:");
    println!("  0   - Disable swap (not recommended)");
    println!("  1   - Minimum swapping");
    println!("  10  - Gaming optimized (recommended for gaming)");
    println!("  60  - Default value");
    println!("  100 - Aggressive swapping");

    let Ok(swappiness): Result<String, _> = Input::new()
        .with_prompt("Enter desired swappiness value (1-100)")
        .default("10".to_string())
        .interact_text()
    else {
        return;
    };

    if let Ok(value) = swappiness.parse::<u32>() {
        if value <= 100 {
            let Ok(confirm) = Confirm::new()
                .with_prompt(&format!("Set swappiness to {}?", value))
                .default(true)
                .interact()
            else {
                return;
            };

            if confirm {
                // Temporary change
                let status = Command::new("sudo")
                    .arg("sysctl")
                    .arg(&format!("vm.swappiness={}", value))
                    .status();

                match status {
                    Ok(s) if s.success() => {
                        println!("✅ Swappiness set to {}", value);

                        let Ok(permanent) = Confirm::new()
                            .with_prompt("Make this change permanent?")
                            .default(true)
                            .interact()
                        else {
                            return;
                        };

                        if permanent {
                            let config_line = format!("vm.swappiness={}\n", value);
                            if let Err(e) = Command::new("sudo")
                                .arg("sh")
                                .arg("-c")
                                .arg(&format!(
                                    "echo '{}' >> /etc/sysctl.d/99-swappiness.conf",
                                    config_line
                                ))
                                .status()
                            {
                                eprintln!("Failed to save swappiness config: {}", e);
                            } else {
                                println!("✅ Swappiness configuration saved");
                            }
                        }
                    }
                    _ => println!("❌ Failed to set swappiness"),
                }
            }
        } else {
            println!("❌ Invalid swappiness value (must be 0-100)");
        }
    }
}

fn enable_zram() {
    println!("💾 Enable Zram Compression");
    println!("==========================");

    let zram_check = Command::new("lsmod")
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).contains("zram"))
        .unwrap_or(false);

    if zram_check {
        println!("✅ Zram is already enabled");
        if let Err(e) = Command::new("zramctl").status() {
            eprintln!("Failed to show zram status: {}", e);
        }
    } else {
        println!("❌ Zram not enabled");
        let Ok(enable) = Confirm::new()
            .with_prompt("Enable zram compression? (reduces memory usage)")
            .default(true)
            .interact()
        else {
            return;
        };

        if enable {
            // Install zram-generator if available
            let status = Command::new("sudo")
                .args(&["pacman", "-S", "--needed", "--noconfirm", "zram-generator"])
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("✅ zram-generator installed");

                    // Create basic zram configuration
                    let zram_config = r#"[zram0]
zram-size = ram / 2
compression-algorithm = lz4
"#;

                    let config_status = Command::new("sudo")
                        .arg("sh")
                        .arg("-c")
                        .arg(&format!(
                            "echo '{}' > /etc/systemd/zram-generator.conf",
                            zram_config
                        ))
                        .status();

                    match config_status {
                        Ok(s) if s.success() => {
                            println!("✅ Zram configuration created");
                            println!("🔄 Reboot required to enable zram");
                        }
                        _ => println!("❌ Failed to create zram configuration"),
                    }
                }
                _ => {
                    println!("⚠️  zram-generator not available in repos");
                    println!("💡 Manual zram setup:");
                    println!("  sudo modprobe zram");
                    println!("  echo lz4 | sudo tee /sys/block/zram0/comp_algorithm");
                    println!("  echo 4G | sudo tee /sys/block/zram0/disksize");
                    println!("  sudo mkswap /dev/zram0 && sudo swapon /dev/zram0");
                }
            }
        }
    }
}

fn clear_memory_caches() {
    println!("🧹 Clear Memory Caches");
    println!("======================");

    println!("📊 Current memory usage:");
    if let Err(e) = Command::new("free").arg("-h").status() {
        eprintln!("Failed to get memory usage: {}", e);
    }

    let cache_options = [
        "🧹 Clear page cache",
        "🗑️  Clear dentries and inodes",
        "💾 Clear all caches",
        "📊 Show cache usage",
    ];

    let Ok(selections) = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cache clearing options")
        .items(&cache_options)
        .interact()
    else {
        return;
    };

    for &index in &selections {
        match index {
            0 => {
                println!("🧹 Clearing page cache...");
                if let Err(e) = Command::new("sudo")
                    .arg("sh")
                    .arg("-c")
                    .arg("echo 1 > /proc/sys/vm/drop_caches")
                    .status()
                {
                    eprintln!("Failed to clear page cache: {}", e);
                }
            }
            1 => {
                println!("🗑️  Clearing dentries and inodes...");
                if let Err(e) = Command::new("sudo")
                    .arg("sh")
                    .arg("-c")
                    .arg("echo 2 > /proc/sys/vm/drop_caches")
                    .status()
                {
                    eprintln!("Failed to clear dentries/inodes: {}", e);
                }
            }
            2 => {
                println!("💾 Clearing all caches...");
                if let Err(e) = Command::new("sudo")
                    .arg("sh")
                    .arg("-c")
                    .arg("echo 3 > /proc/sys/vm/drop_caches")
                    .status()
                {
                    eprintln!("Failed to clear all caches: {}", e);
                }
            }
            3 => {
                println!("📊 Cache usage:");
                if let Err(e) = Command::new("cat").arg("/proc/meminfo").status() {
                    eprintln!("Failed to read meminfo: {}", e);
                }
            }
            _ => {}
        }
    }

    if !selections.is_empty() {
        println!("\n📊 Memory usage after clearing:");
        if let Err(e) = Command::new("free").arg("-h").status() {
            eprintln!("Failed to get memory usage: {}", e);
        }
    }
}

fn memory_usage_analysis() {
    println!("📊 Memory Usage Analysis");
    println!("=======================");

    println!("💾 Overall memory usage:");
    if let Err(e) = Command::new("free").arg("-h").status() {
        eprintln!("Failed to get memory usage: {}", e);
    }

    println!("\n📈 Memory usage by process (top 10):");
    match Command::new("ps").args(["aux", "--sort=-%mem"]).output() {
        Ok(output) => {
            let stdout_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout_str.lines().collect();
            for line in lines.iter().take(11) {
                // header + 10 lines
                println!("{}", line);
            }
        }
        Err(e) => eprintln!("Failed to get process list: {}", e),
    }

    println!("\n🔍 Detailed memory info:");
    if let Err(e) = Command::new("cat").arg("/proc/meminfo").status() {
        eprintln!("Failed to read meminfo: {}", e);
    }
}

fn kernel_parameter_tuning() {
    println!("🔧 Kernel Parameter Tuning");
    println!("===========================");

    let kernel_tweaks = [
        ("vm.dirty_ratio", "20", "Dirty memory threshold (%)"),
        (
            "vm.dirty_background_ratio",
            "5",
            "Background write threshold (%)",
        ),
        ("vm.vfs_cache_pressure", "50", "VFS cache pressure"),
        (
            "kernel.sched_autogroup_enabled",
            "0",
            "Disable automatic process grouping",
        ),
        (
            "net.core.rmem_max",
            "134217728",
            "Max socket receive buffer",
        ),
        ("net.core.wmem_max", "134217728", "Max socket send buffer"),
    ];

    println!("🔧 Recommended gaming kernel parameters:");
    for (param, value, desc) in &kernel_tweaks {
        println!("  {} = {} # {}", param, value, desc);
    }

    let Ok(apply_tweaks) = Confirm::new()
        .with_prompt("Apply recommended kernel parameters?")
        .default(false)
        .interact()
    else {
        return;
    };

    if apply_tweaks {
        let mut config_content = String::new();
        config_content.push_str("# Gaming optimizations\n");

        for (param, value, desc) in &kernel_tweaks {
            config_content.push_str(&format!("{} = {} # {}\n", param, value, desc));

            // Apply temporarily
            if let Err(e) = Command::new("sudo")
                .arg("sysctl")
                .arg(&format!("{}={}", param, value))
                .status()
            {
                eprintln!("Failed to set {}: {}", param, e);
            }
        }

        // Save permanently
        let status = Command::new("sudo")
            .arg("sh")
            .arg("-c")
            .arg(&format!(
                "echo '{}' > /etc/sysctl.d/99-gaming.conf",
                config_content
            ))
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Kernel parameters applied and saved"),
            _ => println!("❌ Failed to save kernel parameters"),
        }
    }
}

fn desktop_environment_optimizations() {
    println!("🖥️  Desktop Environment Optimizations");
    println!("======================================");

    let de_check = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "unknown".to_string());
    println!("🖥️  Detected desktop environment: {}", de_check);

    let optimizations = [
        "🎨 Disable composition during gaming",
        "⚡ Reduce visual effects",
        "🖼️  Configure window manager settings",
        "🔧 Display server optimizations",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Desktop Environment Optimizations")
        .items(&optimizations)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => disable_composition(),
        1 => reduce_visual_effects(),
        2 => configure_window_manager(),
        3 => display_server_optimizations(),
        _ => return,
    }
}

fn disable_composition() {
    println!("🎨 Disable Composition During Gaming");
    println!("====================================");

    let de = std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_else(|_| "unknown".to_string())
        .to_lowercase();

    match de.as_str() {
        "kde" | "plasma" => {
            println!("🔧 KDE Plasma detected");
            println!("💡 To disable compositor:");
            println!("  1. System Settings > Display and Monitor > Compositor");
            println!("  2. Uncheck 'Enable compositor on startup'");
            println!("  3. Or use: qdbus org.kde.KWin /Compositor suspend");
        }
        "gnome" => {
            println!("🔧 GNOME detected");
            println!("💡 GNOME compositor cannot be easily disabled");
            println!("💡 Consider using a lighter window manager for gaming");
        }
        "xfce" => {
            println!("🔧 XFCE detected");
            println!("💡 To disable compositor:");
            println!("  1. Settings > Window Manager Tweaks > Compositor");
            println!("  2. Uncheck 'Enable display compositing'");
        }
        _ => {
            println!("🔧 Unknown desktop environment");
            println!("💡 General compositor disable methods:");
            println!("  • Check your DE's settings for compositor options");
            println!("  • Look for 'composition', 'effects', or 'window manager' settings");
            println!("  • Consider using a minimal window manager for gaming");
        }
    }

    println!("\n💡 Gaming-optimized window managers:");
    println!("  • i3 - Tiling window manager");
    println!("  • Openbox - Lightweight floating WM");
    println!("  • bspwm - Binary space partitioning WM");
}

fn reduce_visual_effects() {
    println!("⚡ Reduce Visual Effects");
    println!("=======================");

    println!("🎨 Visual effect optimizations:");
    println!("  • Disable animations and transitions");
    println!("  • Reduce transparency effects");
    println!("  • Disable window shadows");
    println!("  • Turn off desktop effects");
    println!("  • Use solid wallpapers instead of animated ones");

    println!("\n💡 Performance impact:");
    println!("  • Reduced GPU usage");
    println!("  • Lower memory consumption");
    println!("  • Better frame rates in games");
    println!("  • More consistent performance");
}

fn configure_window_manager() {
    println!("🖼️  Configure Window Manager Settings");
    println!("====================================");

    println!("🔧 Window manager optimizations:");
    println!("  • Force fullscreen games to exclusive mode");
    println!("  • Disable window decorations for games");
    println!("  • Configure focus policies");
    println!("  • Set up gaming workspaces");

    println!("\n🎮 Gaming-specific settings:");
    println!("  • Disable window manager key bindings during fullscreen");
    println!("  • Configure multi-monitor setups");
    println!("  • Set proper game window handling");
}

fn display_server_optimizations() {
    println!("🔧 Display Server Optimizations");
    println!("===============================");

    let display_server = if std::env::var("WAYLAND_DISPLAY").is_ok() {
        "Wayland"
    } else if std::env::var("DISPLAY").is_ok() {
        "X11"
    } else {
        "Unknown"
    };

    println!("🖥️  Display server: {}", display_server);

    match display_server {
        "X11" => {
            println!("\n🔧 X11 optimizations:");
            println!("  • TearFree settings for AMD/Intel");
            println!("  • Force composition pipeline for NVIDIA");
            println!("  • Configure refresh rates");
            println!("  • Disable vsync for better performance");
        }
        "Wayland" => {
            println!("\n🌊 Wayland optimizations:");
            println!("  • Configure compositor settings");
            println!("  • Enable/disable VRR (Variable Refresh Rate)");
            println!("  • Optimize for gaming workloads");
            println!("  • Consider X11 for better game compatibility");
        }
        _ => {
            println!("\n❓ Unknown display server");
            println!("  Check environment variables DISPLAY and WAYLAND_DISPLAY");
        }
    }
}

fn filesystem_optimizations() {
    println!("📁 File System Optimizations");
    println!("============================");

    let fs_optimizations = [
        "🚀 Configure I/O scheduler",
        "💾 Enable file system optimizations",
        "🗂️  Optimize game directories",
        "📊 Analyze disk performance",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("File System Optimizations")
        .items(&fs_optimizations)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => configure_io_scheduler(),
        1 => enable_filesystem_optimizations(),
        2 => optimize_game_directories(),
        3 => analyze_disk_performance(),
        _ => return,
    }
}

fn configure_io_scheduler() {
    println!("🚀 Configure I/O Scheduler");
    println!("==========================");

    println!("📊 Current I/O schedulers:");
    if let Err(e) = Command::new("find")
        .args([
            "/sys/block/",
            "-name",
            "scheduler",
            "-exec",
            "grep",
            "-H",
            ".",
            "{}",
            ";",
        ])
        .status()
    {
        eprintln!("Failed to find schedulers: {}", e);
    }

    println!("\n🔧 Available schedulers:");
    println!("  • mq-deadline - Good general purpose");
    println!("  • kyber - Low latency, good for gaming");
    println!("  • bfq - Better for slow storage/desktop use");
    println!("  • none - No scheduling (for NVMe SSDs)");

    let schedulers = ["mq-deadline", "kyber", "bfq", "none"];
    let Ok(scheduler_choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select I/O scheduler for gaming")
        .items(&schedulers)
        .default(1) // kyber
        .interact()
    else {
        return;
    };

    let selected_scheduler = schedulers[scheduler_choice];

    let Ok(confirm) = Confirm::new()
        .with_prompt(&format!("Set I/O scheduler to '{}'?", selected_scheduler))
        .default(true)
        .interact()
    else {
        return;
    };

    if confirm {
        println!("🔧 Setting I/O scheduler to '{}'...", selected_scheduler);

        // This is a simplified example - in practice you'd want to detect block devices
        let status = Command::new("sudo")
            .arg("sh")
            .arg("-c")
            .arg(&format!(
                "echo {} | sudo tee /sys/block/*/queue/scheduler",
                selected_scheduler
            ))
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ I/O scheduler set to '{}'", selected_scheduler),
            _ => println!("❌ Failed to set I/O scheduler"),
        }
    }
}

fn enable_filesystem_optimizations() {
    println!("💾 Enable File System Optimizations");
    println!("===================================");

    println!("🔧 File system optimization options:");
    println!("  • Enable noatime mount option (reduces write operations)");
    println!("  • Configure read-ahead settings");
    println!("  • Optimize directory indexing");
    println!("  • Configure journal settings");

    println!("\n💡 To enable noatime for better gaming performance:");
    println!("  1. Edit /etc/fstab");
    println!("  2. Add 'noatime' to mount options");
    println!("  3. Example: UUID=... / ext4 defaults,noatime 0 1");
    println!("  4. Remount or reboot to apply");

    let Ok(show_fstab) = Confirm::new()
        .with_prompt("Show current /etc/fstab?")
        .default(false)
        .interact()
    else {
        return;
    };

    if show_fstab {
        if let Err(e) = Command::new("cat").arg("/etc/fstab").status() {
            eprintln!("Failed to read fstab: {}", e);
        }
    }
}

fn optimize_game_directories() {
    println!("🗂️  Optimize Game Directories");
    println!("=============================");

    println!("📁 Game directory optimizations:");
    println!("  • Place games on fastest storage (NVMe SSD)");
    println!("  • Separate game installs from save data");
    println!("  • Use symbolic links for large games");
    println!("  • Configure proper permissions");

    let game_dirs = [
        "~/.steam/steam/steamapps",
        "~/Games",
        "~/.local/share/lutris",
        "~/.wine",
    ];

    println!("\n📂 Common game directories:");
    for dir in &game_dirs {
        let expanded_path = if dir.starts_with("~/") {
            std::env::home_dir()
                .map(|h| h.join(&dir[2..]))
                .unwrap_or_else(|| std::path::PathBuf::from(dir))
        } else {
            std::path::PathBuf::from(dir)
        };

        if expanded_path.exists() {
            if let Err(e) = Command::new("du")
                .args(["-sh", &expanded_path.to_string_lossy()])
                .status()
            {
                eprintln!("Failed to get size for {}: {}", expanded_path.display(), e);
            }
        } else {
            println!("  {} (not found)", expanded_path.display());
        }
    }
}

fn analyze_disk_performance() {
    println!("📊 Analyze Disk Performance");
    println!("===========================");

    println!("💾 Disk usage:");
    if let Err(e) = Command::new("df").arg("-h").status() {
        eprintln!("Failed to get disk usage: {}", e);
    }

    println!("\n⚡ I/O statistics:");
    if let Err(e) = Command::new("iostat").args(["-x", "1", "1"]).status() {
        eprintln!("Failed to get I/O stats: {}", e);
    }

    println!("\n🔍 Block device info:");
    if let Err(e) = Command::new("lsblk").args(["-f"]).status() {
        eprintln!("Failed to list block devices: {}", e);
    }

    let Ok(benchmark) = Confirm::new()
        .with_prompt("Run disk benchmark? (requires fio)")
        .default(false)
        .interact()
    else {
        return;
    };

    if benchmark {
        run_disk_benchmark();
    }
}

fn run_disk_benchmark() {
    let fio_check = Command::new("which").arg("fio").status();
    match fio_check {
        Ok(s) if s.success() => {
            println!("🚀 Running disk benchmark...");
            if let Err(e) = Command::new("fio")
                .args([
                    "--name=gaming-test",
                    "--ioengine=libaio",
                    "--rw=randread",
                    "--bs=4k",
                    "--numjobs=1",
                    "--size=1G",
                    "--runtime=30",
                    "--direct=1",
                ])
                .status()
            {
                eprintln!("Failed to run fio benchmark: {}", e);
            }
        }
        _ => {
            println!("❌ fio not found. Install with: sudo pacman -S fio");
        }
    }
}

fn gaming_performance_tuning() {
    println!("🎮 Gaming-specific Performance Tuning");
    println!("=====================================");

    let gaming_options = [
        "🎯 Game-specific optimizations",
        "🔧 Wine/Proton performance tuning",
        "📊 Steam performance settings",
        "⚡ GPU-specific gaming tweaks",
        "🎮 Controller optimization",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Gaming Performance Tuning")
        .items(&gaming_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => game_specific_optimizations(),
        1 => wine_proton_performance(),
        2 => steam_performance_settings(),
        3 => gpu_gaming_tweaks(),
        4 => controller_optimization(),
        _ => return,
    }
}

fn game_specific_optimizations() {
    println!("🎯 Game-specific Optimizations");
    println!("==============================");

    println!("🎮 Popular game optimizations:");

    let games = [
        (
            "CS2 (Counter-Strike 2)",
            vec![
                "-high -threads 4 +fps_max 300",
                "DXVK_ASYNC=1",
                "Launch with: gamemoderun mangohud %command%",
            ],
        ),
        (
            "Cyberpunk 2077",
            vec![
                "PROTON_NO_ESYNC=1",
                "DXVK_ASYNC=1",
                "Lower crowd density in settings",
            ],
        ),
        (
            "GTA V",
            vec!["WINEDEBUG=-all", "DXVK_HUD=fps", "Use DirectX 11 mode"],
        ),
        (
            "Elden Ring",
            vec![
                "PROTON_USE_WINED3D=1 (if DXVK issues)",
                "gamemoderun %command%",
                "Cap at 60 FPS in-game",
            ],
        ),
    ];

    for (game, optimizations) in &games {
        println!("\n🎮 {}:", game);
        for opt in optimizations {
            println!("  • {}", opt);
        }
    }

    println!("\n💡 General game optimization tips:");
    println!("  • Use GameMode for CPU priority");
    println!("  • Enable MangoHud for monitoring");
    println!("  • Set process priority to high");
    println!("  • Close unnecessary background applications");
    println!("  • Use dedicated GPU if available");
}

fn wine_proton_performance() {
    println!("🔧 Wine/Proton Performance Tuning");
    println!("==================================");

    println!("🍷 Wine performance optimizations:");
    println!("  • Enable DXVK for DirectX games");
    println!("  • Enable Esync/Fsync for threading");
    println!("  • Use Wine-GE or TkG builds");
    println!("  • Configure Windows version appropriately");

    println!("\n🚀 Proton performance settings:");
    println!("  • PROTON_NO_ESYNC=1 (if issues)");
    println!("  • PROTON_NO_FSYNC=1 (if issues)");
    println!("  • DXVK_ASYNC=1 (async shader compilation)");
    println!("  • PROTON_LOG=1 (for debugging)");

    println!("\n⚡ Environment variables for performance:");
    println!("  export WINE_LARGE_ADDRESS_AWARE=1");
    println!("  export DXVK_HUD=fps");
    println!("  export __GL_THREADED_OPTIMIZATIONS=1");

    let Ok(setup_env) = Confirm::new()
        .with_prompt("Add Wine/Proton performance environment to ~/.profile?")
        .default(false)
        .interact()
    else {
        return;
    };

    if setup_env {
        let wine_env = r#"
# Wine/Proton Performance Environment
export WINE_LARGE_ADDRESS_AWARE=1
export __GL_THREADED_OPTIMIZATIONS=1
export __GL_SHADER_DISK_CACHE=1
export DXVK_ASYNC=1
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
            if writeln!(file, "{}", wine_env).is_err() {
                println!("❌ Failed to write to profile");
            } else {
                println!("✅ Wine performance environment added to ~/.profile");
            }
        }
    }
}

fn steam_performance_settings() {
    println!("📊 Steam Performance Settings");
    println!("=============================");

    println!("🚀 Steam client optimizations:");
    println!("  • Disable Steam overlay (if causing issues)");
    println!("  • Limit Steam downloads during gaming");
    println!("  • Disable auto-updates during gameplay");
    println!("  • Use Steam's built-in FPS counter");

    println!("\n🎮 Steam launch options for performance:");
    println!("  gamemoderun mangohud %command%     # GameMode + MangoHud");
    println!("  -high %command%                    # High CPU priority");
    println!("  -threads 4 %command%               # Limit CPU threads");
    println!("  DXVK_HUD=fps %command%             # DXVK FPS counter");

    println!("\n⚙️  Steam settings to check:");
    println!("  • Steam > Settings > In-Game > Enable Steam Overlay");
    println!("  • Steam > Settings > Downloads > Throttle during gameplay");
    println!("  • Steam > Settings > Updates > Automatic updates");

    println!("\n🔧 Proton-specific settings:");
    println!("  • Steam > Settings > Steam Play");
    println!("  • Enable Steam Play for all titles");
    println!("  • Select latest Proton version");
}

fn gpu_gaming_tweaks() {
    println!("⚡ GPU-specific Gaming Tweaks");
    println!("=============================");

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

    println!("🎮 Detected GPU vendor: {}", gpu_vendor);

    match gpu_vendor {
        "NVIDIA" => {
            println!("\n🟢 NVIDIA Gaming Optimizations:");
            println!("  • Use nvidia-settings for overclocking");
            println!("  • Enable G-Sync if supported");
            println!("  • Set Power Management Mode to 'Prefer Maximum Performance'");
            println!("  • Enable Threaded Optimization");
            println!("  • Configure shader cache location");

            let Ok(nvidia_tweaks) = Confirm::new()
                .with_prompt("Apply NVIDIA gaming environment variables?")
                .default(false)
                .interact()
            else {
                return;
            };

            if nvidia_tweaks {
                apply_nvidia_gaming_env();
            }
        }
        "AMD" => {
            println!("\n🔴 AMD Gaming Optimizations:");
            println!("  • Use corectrl for fan curves and overclocking");
            println!("  • Enable FreeSync if supported");
            println!("  • Set GPU power profile to 'performance'");
            println!("  • Configure RADV driver settings");

            let Ok(amd_tweaks) = Confirm::new()
                .with_prompt("Apply AMD gaming environment variables?")
                .default(false)
                .interact()
            else {
                return;
            };

            if amd_tweaks {
                apply_amd_gaming_env();
            }
        }
        "Intel" => {
            println!("\n🔵 Intel Gaming Optimizations:");
            println!("  • Limited gaming performance compared to discrete GPUs");
            println!("  • Enable Intel GPU monitoring with intel-gpu-tools");
            println!("  • Lower game settings for better performance");
        }
        _ => {
            println!("\n❓ Unknown GPU - General optimizations:");
            println!("  • Check GPU driver installation");
            println!("  • Monitor GPU temperatures during gaming");
            println!("  • Configure appropriate graphics settings in games");
        }
    }
}

fn apply_nvidia_gaming_env() {
    let nvidia_env = r#"
# NVIDIA Gaming Environment
export __GL_THREADED_OPTIMIZATIONS=1
export __GL_SHADER_DISK_CACHE=1
export __GL_SHADER_DISK_CACHE_PATH=~/.cache/nvidia_shader
export __GL_SYNC_TO_VBLANK=0
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
        if writeln!(file, "{}", nvidia_env).is_err() {
            println!("❌ Failed to write to profile");
        } else {
            println!("✅ NVIDIA gaming environment added to ~/.profile");

            // Create shader cache directory
            let shader_cache = std::env::home_dir()
                .map(|h| h.join(".cache/nvidia_shader"))
                .unwrap_or_else(|| std::path::PathBuf::from("~/.cache/nvidia_shader"));
            let _ = std::fs::create_dir_all(&shader_cache);
        }
    }
}

fn apply_amd_gaming_env() {
    let amd_env = r#"
# AMD Gaming Environment
export RADV_PERFTEST=aco
export MESA_VK_WSI_PRESENT_MODE=fifo
export AMD_VULKAN_ICD=RADV
export RADV_DEBUG=checkir,llvm
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
        if writeln!(file, "{}", amd_env).is_err() {
            println!("❌ Failed to write to profile");
        } else {
            println!("✅ AMD gaming environment added to ~/.profile");
        }
    }
}

fn controller_optimization() {
    println!("🎮 Controller Optimization");
    println!("==========================");

    println!("🕹️  Controller performance optimizations:");
    println!("  • Reduce input latency");
    println!("  • Configure polling rates");
    println!("  • Optimize wireless connections");
    println!("  • Set up custom profiles");

    let controller_tools = [
        "🔧 Install controller utilities",
        "📊 Check controller input latency",
        "⚙️  Configure controller settings",
        "🔋 Optimize wireless performance",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Controller Optimization")
        .items(&controller_tools)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => install_controller_utilities(),
        1 => check_controller_latency(),
        2 => configure_controller_settings(),
        3 => optimize_wireless_performance(),
        _ => return,
    }
}

fn install_controller_utilities() {
    println!("🔧 Installing Controller Utilities");
    println!("==================================");

    let controller_packages = ["jstest-gtk", "linuxconsole", "antimicrox", "lib32-libusb"];

    let Ok(install) = Confirm::new()
        .with_prompt("Install controller utilities?")
        .default(true)
        .interact()
    else {
        return;
    };

    if install {
        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&controller_packages)
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Controller utilities installed");
                println!("🎮 Available tools:");
                println!("  • jstest-gtk - Controller testing GUI");
                println!("  • jstest - Command line controller testing");
                println!("  • antimicrox - Keyboard/mouse mapping");
            }
            _ => println!("❌ Failed to install controller utilities"),
        }
    }
}

fn check_controller_latency() {
    println!("📊 Check Controller Input Latency");
    println!("=================================");

    println!("🕹️  Connected controllers:");
    if let Err(e) = Command::new("ls").arg("/dev/input/js*").status() {
        eprintln!("Failed to list controllers: {}", e);
    }

    println!("\n🔍 Controller device info:");
    if let Err(e) = Command::new("lsusb")
        .args(["|", "grep", "-i", "gamepad\\|controller\\|joystick"])
        .status()
    {
        eprintln!("Failed to get controller info: {}", e);
    }

    let Ok(test_controller) = Confirm::new()
        .with_prompt("Test controller input? (requires jstest)")
        .default(false)
        .interact()
    else {
        return;
    };

    if test_controller {
        println!("🧪 Testing controller input...");
        println!("💡 Press Ctrl+C to exit test");
        if let Err(e) = Command::new("jstest").arg("/dev/input/js0").status() {
            eprintln!("Failed to run jstest: {}", e);
        }
    }
}

fn configure_controller_settings() {
    println!("⚙️  Configure Controller Settings");
    println!("=================================");

    println!("🎮 Controller configuration options:");
    println!("  • Dead zone adjustment");
    println!("  • Button mapping");
    println!("  • Sensitivity curves");
    println!("  • Polling rate configuration");

    println!("\n🔧 Steam Input configuration:");
    println!("  • Steam > Settings > Controller");
    println!("  • Enable Steam Input for your controller type");
    println!("  • Configure per-game controller settings");

    println!("\n🛠️  System-level configuration:");
    println!("  • Use antimicrox for custom key mappings");
    println!("  • Configure evdev for advanced settings");
    println!("  • Set up udev rules for consistent device naming");
}

fn optimize_wireless_performance() {
    println!("🔋 Optimize Wireless Performance");
    println!("================================");

    println!("📡 Wireless controller optimizations:");
    println!("  • Use 2.4GHz for lower latency (avoid 5GHz interference)");
    println!("  • Keep controllers close to receiver");
    println!("  • Use wired connection for competitive gaming");
    println!("  • Disable power saving on wireless adapters");

    println!("\n🔋 Battery optimizations:");
    println!("  • Use fresh batteries or full charge");
    println!("  • Disable controller vibration to save power");
    println!("  • Adjust controller sleep timers");

    println!("\n📊 Check wireless interference:");
    if let Err(e) = Command::new("iwlist")
        .args(["scan", "|", "grep", "Frequency"])
        .status()
    {
        eprintln!("Failed to scan wireless: {}", e);
    }
}

fn gpu_performance_overclocking() {
    println!("🖥️  GPU Performance & Overclocking");
    println!("==================================");

    println!("⚠️  WARNING: Overclocking can damage hardware!");
    println!("Only proceed if you understand the risks.");

    let Ok(proceed) = Confirm::new()
        .with_prompt("Continue with GPU performance tuning?")
        .default(false)
        .interact()
    else {
        return;
    };

    if !proceed {
        return;
    }

    let gpu_options = [
        "📊 GPU monitoring and information",
        "🌡️  Temperature monitoring setup",
        "⚡ Basic performance tweaks",
        "🔧 Advanced overclocking (DANGEROUS)",
        "🧪 GPU stress testing",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("GPU Performance Options")
        .items(&gpu_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => gpu_monitoring_info(),
        1 => temperature_monitoring_setup(),
        2 => basic_performance_tweaks(),
        3 => advanced_overclocking(),
        4 => gpu_stress_testing(),
        _ => return,
    }
}

fn gpu_monitoring_info() {
    println!("📊 GPU Monitoring and Information");
    println!("================================");

    println!("🖥️  GPU Hardware Information:");
    if let Err(e) = Command::new("lspci")
        .args(["-v", "|", "grep", "-A", "10", "-i", "vga"])
        .status()
    {
        eprintln!("Failed to get GPU info: {}", e);
    }

    println!("\n🔍 GPU Monitoring Tools:");
    let monitoring_tools = [
        ("nvidia-smi", "NVIDIA GPU monitoring"),
        ("radeontop", "AMD GPU monitoring"),
        ("nvtop", "Universal GPU monitoring"),
        ("intel-gpu-tools", "Intel GPU utilities"),
    ];

    for (tool, description) in &monitoring_tools {
        let status = Command::new("which").arg(tool).status();
        match status {
            Ok(s) if s.success() => println!("  ✅ {} - {}", tool, description),
            _ => println!("  ❌ {} - {} (not installed)", tool, description),
        }
    }

    let Ok(install_tools) = Confirm::new()
        .with_prompt("Install missing GPU monitoring tools?")
        .default(true)
        .interact()
    else {
        return;
    };

    if install_tools {
        let tools_to_install = ["nvtop", "radeontop"];
        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&tools_to_install)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ GPU monitoring tools installed"),
            _ => println!("❌ Some tools may not be available in repositories"),
        }
    }
}

fn temperature_monitoring_setup() {
    println!("🌡️  Temperature Monitoring Setup");
    println!("=================================");

    println!("🔥 GPU temperature monitoring is crucial for safe overclocking");

    let lm_sensors_check = Command::new("which").arg("sensors").status();
    match lm_sensors_check {
        Ok(s) if s.success() => {
            println!("✅ lm_sensors installed");
            println!("\n🌡️  Current temperatures:");
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
                        println!("🔧 Run 'sudo sensors-detect' to configure sensors");
                    }
                    _ => println!("❌ Failed to install lm_sensors"),
                }
            }
        }
    }

    println!("\n🎯 Temperature monitoring guidelines:");
    println!("  • GPU: Keep below 80°C under load");
    println!("  • CPU: Keep below 85°C under load");
    println!("  • Monitor during stress testing");
    println!("  • Set up temperature alerts if possible");
}

fn basic_performance_tweaks() {
    println!("⚡ Basic Performance Tweaks");
    println!("===========================");

    println!("🔧 Safe GPU performance optimizations:");
    println!("  • Update GPU drivers to latest version");
    println!("  • Enable GPU performance mode");
    println!("  • Optimize power management settings");
    println!("  • Configure fan curves for better cooling");

    // Detect GPU vendor for specific instructions
    let lspci_output = Command::new("lspci").args(&["-k"]).output();
    let mut gpu_vendor = "Unknown";

    if let Ok(output) = lspci_output {
        let lspci = String::from_utf8_lossy(&output.stdout);
        if lspci.contains("NVIDIA") {
            gpu_vendor = "NVIDIA";
        } else if lspci.contains("AMD") || lspci.contains("Radeon") {
            gpu_vendor = "AMD";
        }
    }

    match gpu_vendor {
        "NVIDIA" => {
            println!("\n🟢 NVIDIA Basic Tweaks:");
            println!("  • Open nvidia-settings");
            println!("  • Set PowerMizer to 'Prefer Maximum Performance'");
            println!("  • Increase memory and core clock by +50MHz");
            println!("  • Test stability with games");

            let Ok(apply_nvidia_tweaks) = Confirm::new()
                .with_prompt("Apply basic NVIDIA performance tweaks?")
                .default(false)
                .interact()
            else {
                return;
            };

            if apply_nvidia_tweaks {
                apply_nvidia_basic_tweaks();
            }
        }
        "AMD" => {
            println!("\n🔴 AMD Basic Tweaks:");
            println!("  • Install corectrl for GUI overclocking");
            println!("  • Enable performance mode in GPU settings");
            println!("  • Adjust power limit to +20%");
            println!("  • Increase fan curve for better cooling");

            let Ok(apply_amd_tweaks) = Confirm::new()
                .with_prompt("Install CoreCtrl for AMD GPU management?")
                .default(false)
                .interact()
            else {
                return;
            };

            if apply_amd_tweaks {
                install_corectrl();
            }
        }
        _ => {
            println!("\n❓ GPU vendor not detected or unsupported");
            println!("  Check your GPU drivers and installation");
        }
    }
}

fn apply_nvidia_basic_tweaks() {
    println!("🟢 Applying NVIDIA Basic Tweaks");
    println!("===============================");

    // Check if nvidia-settings is available
    let nvidia_settings_check = Command::new("which").arg("nvidia-settings").status();
    match nvidia_settings_check {
        Ok(s) if s.success() => {
            println!("✅ nvidia-settings found");

            // Launch nvidia-settings for manual configuration
            let Ok(launch) = Confirm::new()
                .with_prompt("Launch nvidia-settings for manual configuration?")
                .default(true)
                .interact()
            else {
                return;
            };

            if launch {
                let _ = Command::new("nvidia-settings").spawn();
                println!("💡 In nvidia-settings:");
                println!("  1. Go to PowerMizer");
                println!("  2. Set 'Preferred Mode' to 'Prefer Maximum Performance'");
                println!("  3. Apply changes");
            }
        }
        _ => println!("❌ nvidia-settings not found. Install NVIDIA drivers first."),
    }
}

fn install_corectrl() {
    println!("🔴 Installing CoreCtrl for AMD GPU Management");
    println!("==============================================");

    // Try to install from AUR
    let aur_helpers = ["yay", "paru", "trizen"];
    for helper in &aur_helpers {
        let helper_check = Command::new("which").arg(helper).status();
        if let Ok(s) = helper_check
            && s.success()
        {
            println!("🔧 Using {} to install CoreCtrl...", helper);
            let install_status = Command::new(helper)
                .args(&["-S", "--noconfirm", "corectrl"])
                .status();

            match install_status {
                Ok(s) if s.success() => {
                    println!("✅ CoreCtrl installed successfully!");
                    println!("💡 Launch CoreCtrl to configure your AMD GPU");
                    return;
                }
                _ => println!("❌ Failed to install with {}", helper),
            }
        }
    }

    println!("❌ No AUR helper found. Install yay first:");
    println!("   sudo pacman -S --needed base-devel git");
    println!("   git clone https://aur.archlinux.org/yay.git && cd yay && makepkg -si");
}

fn advanced_overclocking() {
    println!("🔧 Advanced Overclocking (DANGEROUS)");
    println!("====================================");

    println!("⚠️  EXTREME WARNING:");
    println!("  • Overclocking can permanently damage hardware");
    println!("  • Void warranties");
    println!("  • Cause system instability");
    println!("  • Increase power consumption and heat");

    let Ok(acknowledge) = Confirm::new()
        .with_prompt("I understand the risks and want to proceed")
        .default(false)
        .interact()
    else {
        return;
    };

    if !acknowledge {
        println!("✅ Smart choice! Stick to basic tweaks for safer performance gains.");
        return;
    }

    println!("\n🧪 Advanced overclocking requires:");
    println!("  • Excellent cooling (preferably liquid)");
    println!("  • Quality power supply");
    println!("  • Stable system baseline");
    println!("  • Patience for testing");
    println!("  • Knowledge of recovery methods");

    println!("\n🔧 Tools for advanced overclocking:");
    println!("  • MSI Afterburner (via Wine)");
    println!("  • nvidia-settings (NVIDIA)");
    println!("  • CoreCtrl (AMD)");
    println!("  • GPU memory timing tools");

    println!("\n💡 This tool won't provide specific overclocking instructions");
    println!("   Research your specific GPU model and cooling setup first!");
}

fn gpu_stress_testing() {
    println!("🧪 GPU Stress Testing");
    println!("====================");

    println!("🔥 GPU stress testing tools:");

    let stress_tools = [
        ("glmark2", "OpenGL benchmark", "glmark2"),
        ("unigine-heaven", "3D stress test", ""),
        ("furmark", "GPU stress test", ""),
        ("vkcube", "Vulkan test", "vulkan-tools"),
    ];

    println!("Available stress testing tools:");
    for (tool, description, package) in &stress_tools {
        if !package.is_empty() {
            let status = Command::new("which").arg(tool).status();
            match status {
                Ok(s) if s.success() => println!("  ✅ {} - {}", tool, description),
                _ => println!("  ❌ {} - {} (install: {})", tool, description, package),
            }
        } else {
            println!("  💡 {} - {} (manual install required)", tool, description);
        }
    }

    let Ok(install_tools) = Confirm::new()
        .with_prompt("Install available stress testing tools?")
        .default(true)
        .interact()
    else {
        return;
    };

    if install_tools {
        let packages = ["glmark2", "vulkan-tools"];
        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Stress testing tools installed"),
            _ => println!("❌ Some tools may not be available"),
        }
    }

    let Ok(run_test) = Confirm::new()
        .with_prompt("Run a quick GPU test with glmark2?")
        .default(false)
        .interact()
    else {
        return;
    };

    if run_test {
        println!("🧪 Running glmark2 GPU test...");
        let _ = Command::new("glmark2").status();
    }
}

fn memory_storage_optimization_menu() {
    println!("💾 Memory & Storage Optimization");
    println!("================================");

    let options = [
        "🧠 Memory Optimization",
        "💿 Storage Performance",
        "🔄 Swap Configuration",
        "📊 Memory Analysis",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Memory & Storage Options")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => memory_optimization(),
        1 => storage_performance(),
        2 => swap_configuration(),
        3 => memory_analysis(),
        _ => return,
    }
}

fn memory_optimization() {
    println!("🧠 Memory Optimization");
    println!("======================");

    println!("🔧 Applying memory optimizations...");

    // Optimize vm parameters
    let _ = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg("echo 'vm.swappiness=1' >> /etc/sysctl.d/99-gaming.conf")
        .status();

    let _ = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg("echo 'vm.vfs_cache_pressure=50' >> /etc/sysctl.d/99-gaming.conf")
        .status();

    println!("✅ Memory optimizations applied");
}

fn storage_performance() {
    println!("💿 Storage Performance Optimization");
    println!("===================================");

    println!("📊 Current storage configuration:");
    let _ = Command::new("lsblk").status();

    println!("\n🔧 Optimizing I/O scheduler...");
    let _ = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg("echo mq-deadline > /sys/block/*/queue/scheduler")
        .status();

    println!("✅ Storage optimizations applied");
}

fn swap_configuration() {
    println!("🔄 Swap Configuration");
    println!("=====================");

    println!("📊 Current swap status:");
    let _ = Command::new("free").args(&["-h"]).status();

    let _ = Command::new("cat").arg("/proc/swaps").status();
}

fn memory_analysis() {
    println!("📊 Memory Analysis");
    println!("==================");

    let _ = Command::new("free").args(&["-h"]).status();
    let _ = Command::new("cat").arg("/proc/meminfo").status();
}

fn thermal_management() {
    println!("🌡️  Thermal Management");
    println!("======================");

    let thermal_options = [
        "🌡️  Monitor system temperatures",
        "💨 Configure fan curves",
        "🔥 Thermal throttling analysis",
        "❄️  Cooling optimization tips",
        "⚠️  Emergency thermal shutdown",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Thermal Management")
        .items(&thermal_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => monitor_temperatures(),
        1 => configure_fan_curves(),
        2 => thermal_throttling_analysis(),
        3 => cooling_optimization_tips(),
        4 => emergency_thermal_shutdown(),
        _ => return,
    }
}

fn monitor_temperatures() {
    println!("🌡️  Monitor System Temperatures");
    println!("===============================");

    let sensors_check = Command::new("which").arg("sensors").status();
    match sensors_check {
        Ok(s) if s.success() => {
            println!("📊 Current system temperatures:");
            let _ = Command::new("sensors").status();

            println!("\n🎯 Temperature guidelines:");
            println!("  CPU: < 85°C under load");
            println!("  GPU: < 80°C under load");
            println!("  NVMe SSD: < 70°C");
            println!("  Motherboard: < 50°C");
        }
        _ => {
            println!("❌ lm_sensors not installed");
            let Ok(install) = Confirm::new()
                .with_prompt("Install temperature monitoring tools?")
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
                        println!("🔧 Run 'sudo sensors-detect' and follow prompts");
                        println!("🔄 Then run 'sensors' to see temperatures");
                    }
                    _ => println!("❌ Failed to install lm_sensors"),
                }
            }
        }
    }

    println!("\n🔄 Continuous monitoring options:");
    println!("  watch -n 1 sensors           # Update every second");
    println!("  htop                          # Shows CPU temp");
    println!("  nvtop                         # GPU monitoring");
}

fn configure_fan_curves() {
    println!("💨 Configure Fan Curves");
    println!("=======================");

    println!("🌪️  Fan curve configuration options:");
    println!("  • BIOS/UEFI settings (most reliable)");
    println!("  • fancontrol (Linux software control)");
    println!("  • GPU-specific tools (MSI Afterburner, CoreCtrl)");
    println!("  • Motherboard vendor utilities");

    let Ok(setup_fancontrol) = Confirm::new()
        .with_prompt("Set up fancontrol for custom fan curves?")
        .default(false)
        .interact()
    else {
        return;
    };

    if setup_fancontrol {
        setup_linux_fancontrol();
    }

    println!("\n💡 General fan curve tips:");
    println!("  • Start fans at ~40°C to reduce noise");
    println!("  • Increase fan speed gradually");
    println!("  • Test stability under load");
    println!("  • Balance noise vs. cooling");
}

fn setup_linux_fancontrol() {
    println!("🔧 Setting up Linux fancontrol");
    println!("==============================");

    let fancontrol_check = Command::new("which").arg("fancontrol").status();
    match fancontrol_check {
        Ok(s) if s.success() => {
            println!("✅ fancontrol already installed");
        }
        _ => {
            let Ok(install) = Confirm::new()
                .with_prompt("Install fancontrol?")
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
                    Ok(s) if s.success() => println!("✅ fancontrol installed"),
                    _ => {
                        println!("❌ Failed to install fancontrol");
                        return;
                    }
                }
            } else {
                return;
            }
        }
    }

    println!("\n🔧 fancontrol setup process:");
    println!("  1. Run 'sudo sensors-detect' first");
    println!("  2. Run 'sudo pwmconfig' to configure");
    println!("  3. Test with 'sudo fancontrol'");
    println!("  4. Enable with 'sudo systemctl enable fancontrol'");

    let Ok(run_pwmconfig) = Confirm::new()
        .with_prompt("Run pwmconfig now? (requires interactive input)")
        .default(false)
        .interact()
    else {
        return;
    };

    if run_pwmconfig {
        println!("🔧 Starting pwmconfig...");
        let _ = Command::new("sudo").arg("pwmconfig").status();
    }
}

fn thermal_throttling_analysis() {
    println!("🔥 Thermal Throttling Analysis");
    println!("==============================");

    println!("🔍 Checking for thermal throttling...");

    // Check CPU frequency scaling
    println!("\n⚡ CPU frequency information:");
    let _ = Command::new("cat")
        .arg("/proc/cpuinfo")
        .output()
        .map(|output| {
            let cpu_info = String::from_utf8_lossy(&output.stdout);
            let mut cpu_mhz_lines: Vec<&str> = cpu_info
                .lines()
                .filter(|line| line.contains("cpu MHz"))
                .collect();
            cpu_mhz_lines.truncate(4); // Show first 4 cores
            for line in cpu_mhz_lines {
                println!("  {}", line);
            }
        });

    // Check thermal zones
    println!("\n🌡️  Thermal zones:");
    let thermal_zones = std::fs::read_dir("/sys/class/thermal/").map(|entries| {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str()
                && name.starts_with("thermal_zone")
            {
                let temp_path = entry.path().join("temp");
                if let Ok(temp) = std::fs::read_to_string(&temp_path)
                    && let Ok(temp_millic) = temp.trim().parse::<i32>()
                {
                    let temp_c = temp_millic / 1000;
                    println!("  {}: {}°C", name, temp_c);
                }
            }
        }
    });

    if thermal_zones.is_err() {
        println!("  ❌ Could not read thermal zones");
    }

    println!("\n💡 Signs of thermal throttling:");
    println!("  • CPU frequency below base clock under load");
    println!("  • Performance drops during sustained load");
    println!("  • High temperatures (>85°C CPU, >80°C GPU)");
    println!("  • System stuttering or crashes under load");
}

fn cooling_optimization_tips() {
    println!("❄️  Cooling Optimization Tips");
    println!("=============================");

    println!("🌪️  Airflow optimization:");
    println!("  • Intake fans at front/bottom");
    println!("  • Exhaust fans at rear/top");
    println!("  • Maintain positive pressure");
    println!("  • Clean dust from components regularly");

    println!("\n🖥️  Component-specific cooling:");
    println!("  CPU:");
    println!("    • Quality thermal paste application");
    println!("    • Proper heatsink mounting pressure");
    println!("    • Consider liquid cooling for high-end CPUs");

    println!("  GPU:");
    println!("    • Undervolting for lower temperatures");
    println!("    • Custom fan curves");
    println!("    • Case fans for GPU cooling");

    println!("  Storage:");
    println!("    • NVMe heatsinks for M.2 drives");
    println!("    • Airflow over storage drives");

    println!("\n🏠 Environmental factors:");
    println!("  • Room temperature affects cooling");
    println!("  • Case placement and ventilation");
    println!("  • Ambient dust and humidity");

    println!("\n🔧 Maintenance schedule:");
    println!("  • Monthly: Dust cleaning");
    println!("  • Quarterly: Fan inspection");
    println!("  • Yearly: Thermal paste replacement");
}

fn emergency_thermal_shutdown() {
    println!("⚠️  Emergency Thermal Shutdown");
    println!("==============================");

    println!("🚨 WARNING: This will immediately shut down the system!");
    println!("Only use if system is overheating and cannot be cooled normally.");

    let Ok(confirm_emergency) = Confirm::new()
        .with_prompt("⚠️  Really perform emergency shutdown?")
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm_emergency {
        let Ok(final_confirm) = Confirm::new()
            .with_prompt("🚨 FINAL WARNING: System will shutdown immediately!")
            .default(false)
            .interact()
        else {
            return;
        };

        if final_confirm {
            println!("🚨 Performing emergency thermal shutdown...");
            let _ = Command::new("sudo")
                .args(&["shutdown", "-h", "now"])
                .status();
        }
    }

    println!("\n💡 Prevention is better than emergency shutdown:");
    println!("  • Monitor temperatures regularly");
    println!("  • Set up automatic fan curves");
    println!("  • Improve case airflow");
    println!("  • Clean dust buildup");
}

fn performance_monitoring_benchmarking() {
    println!("📊 Performance Monitoring & Benchmarking");
    println!("========================================");

    let benchmark_options = [
        "🏃 CPU Benchmarks",
        "🎮 GPU Benchmarks",
        "💾 Storage Benchmarks",
        "🌐 Network Performance",
        "🔍 System Monitoring Setup",
        "📈 Performance Logging",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Performance Monitoring & Benchmarking")
        .items(&benchmark_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => cpu_benchmarks(),
        1 => gpu_benchmarks(),
        2 => storage_benchmarks(),
        3 => network_performance(),
        4 => system_monitoring_setup(),
        5 => performance_logging(),
        _ => return,
    }
}

fn cpu_benchmarks() {
    println!("🏃 CPU Benchmarks");
    println!("=================");

    let cpu_tools = [
        ("stress", "CPU stress testing", true),
        ("sysbench", "Multi-threaded CPU benchmark", true),
        ("7-zip", "Compression benchmark", false),
        ("blender", "3D rendering benchmark", false),
    ];

    println!("💻 Available CPU benchmark tools:");
    for (tool, description, lightweight) in &cpu_tools {
        let status = Command::new("which").arg(tool).status();
        let available = status.map(|s| s.success()).unwrap_or(false);
        let marker = if available { "✅" } else { "❌" };
        let weight = if *lightweight { "lightweight" } else { "heavy" };
        println!("  {} {} - {} ({})", marker, tool, description, weight);
    }

    let Ok(install_tools) = Confirm::new()
        .with_prompt("Install missing lightweight CPU benchmark tools?")
        .default(true)
        .interact()
    else {
        return;
    };

    if install_tools {
        let packages = ["stress", "sysbench"];
        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ CPU benchmark tools installed"),
            _ => println!("❌ Some tools may not be available"),
        }
    }

    let Ok(run_quick_test) = Confirm::new()
        .with_prompt("Run a quick CPU stress test? (30 seconds)")
        .default(false)
        .interact()
    else {
        return;
    };

    if run_quick_test {
        println!("🏃 Running 30-second CPU stress test...");
        let _ = Command::new("stress")
            .args(&["--cpu", "4", "--timeout", "30s"])
            .status();

        println!("✅ CPU stress test completed");
        println!("🌡️  Check temperatures with: sensors");
    }
}

fn gpu_benchmarks() {
    println!("🎮 GPU Benchmarks");
    println!("=================");

    let gpu_tools = [
        ("glmark2", "OpenGL benchmark", true),
        ("vkcube", "Vulkan demo", true),
        ("unigine-superposition", "3D benchmark", false),
        ("furmark", "GPU stress test", false),
    ];

    println!("🖥️  Available GPU benchmark tools:");
    for (tool, description, available_in_repos) in &gpu_tools {
        let status = Command::new("which").arg(tool).status();
        let installed = status.map(|s| s.success()).unwrap_or(false);
        let marker = if installed { "✅" } else { "❌" };
        let availability = if *available_in_repos {
            "repos"
        } else {
            "manual"
        };
        println!("  {} {} - {} ({})", marker, tool, description, availability);
    }

    let Ok(install_tools) = Confirm::new()
        .with_prompt("Install available GPU benchmark tools?")
        .default(true)
        .interact()
    else {
        return;
    };

    if install_tools {
        let packages = ["glmark2", "vulkan-tools"];
        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ GPU benchmark tools installed"),
            _ => println!("❌ Some tools may not be available"),
        }
    }

    let Ok(run_gpu_test) = Confirm::new()
        .with_prompt("Run OpenGL benchmark with glmark2?")
        .default(false)
        .interact()
    else {
        return;
    };

    if run_gpu_test {
        println!("🎮 Running GPU benchmark...");
        let _ = Command::new("glmark2").arg("--annotate").status();
    }
}

fn storage_benchmarks() {
    println!("💾 Storage Benchmarks");
    println!("====================");

    let storage_tools = [
        ("hdparm", "Hard drive info and basic tests", true),
        ("fio", "Advanced I/O benchmarking", true),
        ("dd", "Simple read/write test", true),
        ("iozone", "File system benchmark", false),
    ];

    println!("💿 Available storage benchmark tools:");
    for (tool, description, available) in &storage_tools {
        let status = Command::new("which").arg(tool).status();
        let installed = status.map(|s| s.success()).unwrap_or(false);
        let marker = if installed { "✅" } else { "❌" };
        let availability = if *available { "repos" } else { "manual" };
        println!("  {} {} - {} ({})", marker, tool, description, availability);
    }

    let Ok(install_tools) = Confirm::new()
        .with_prompt("Install storage benchmark tools?")
        .default(true)
        .interact()
    else {
        return;
    };

    if install_tools {
        let packages = ["hdparm", "fio"];
        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Storage benchmark tools installed"),
            _ => println!("❌ Some tools may not be available"),
        }
    }

    let Ok(run_storage_test) = Confirm::new()
        .with_prompt("Run simple storage benchmark? (creates 1GB test file)")
        .default(false)
        .interact()
    else {
        return;
    };

    if run_storage_test {
        run_simple_storage_benchmark();
    }
}

fn run_simple_storage_benchmark() {
    println!("💾 Running simple storage benchmark...");

    println!("\n📝 Write test (1GB):");
    let write_start = std::time::Instant::now();
    let write_result = Command::new("dd")
        .args(&[
            "if=/dev/zero",
            "of=/tmp/benchmark_test",
            "bs=1M",
            "count=1024",
            "conv=fdatasync",
        ])
        .output();

    match write_result {
        Ok(_) => {
            let write_duration = write_start.elapsed();
            let write_speed = 1024.0 / write_duration.as_secs_f64();
            println!("✅ Write speed: {:.2} MB/s", write_speed);
        }
        Err(_) => println!("❌ Write test failed"),
    }

    println!("\n📖 Read test:");
    let read_start = std::time::Instant::now();
    let read_result = Command::new("dd")
        .args(&["if=/tmp/benchmark_test", "of=/dev/null", "bs=1M"])
        .output();

    match read_result {
        Ok(_) => {
            let read_duration = read_start.elapsed();
            let read_speed = 1024.0 / read_duration.as_secs_f64();
            println!("✅ Read speed: {:.2} MB/s", read_speed);
        }
        Err(_) => println!("❌ Read test failed"),
    }

    // Clean up
    let _ = Command::new("rm").arg("/tmp/benchmark_test").status();
    println!("🧹 Cleaned up test file");
}

fn network_performance() {
    println!("🌐 Network Performance");
    println!("=====================");

    let network_tools = [
        ("iperf3", "Network bandwidth testing", true),
        ("speedtest-cli", "Internet speed test", true),
        ("ping", "Latency testing", true),
        ("traceroute", "Network path analysis", true),
    ];

    println!("📡 Network performance tools:");
    for (tool, description, available) in &network_tools {
        let status = Command::new("which").arg(tool).status();
        let installed = status.map(|s| s.success()).unwrap_or(false);
        let marker = if installed { "✅" } else { "❌" };
        let availability = if *available { "available" } else { "manual" };
        println!("  {} {} - {} ({})", marker, tool, description, availability);
    }

    let network_tests = [
        "🌐 Internet speed test",
        "🏓 Gaming server latency test",
        "📊 Local network performance",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Network Performance Tests")
        .items(&network_tests)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => internet_speed_test(),
        1 => gaming_latency_test(),
        2 => local_network_test(),
        _ => return,
    }
}

fn internet_speed_test() {
    println!("🌐 Internet Speed Test");
    println!("======================");

    let speedtest_check = Command::new("which").arg("speedtest-cli").status();
    match speedtest_check {
        Ok(s) if s.success() => {
            println!("🚀 Running internet speed test...");
            let _ = Command::new("speedtest-cli").status();
        }
        _ => {
            println!("❌ speedtest-cli not found");
            let Ok(install) = Confirm::new()
                .with_prompt("Install speedtest-cli?")
                .default(true)
                .interact()
            else {
                return;
            };

            if install {
                // Try pip install as speedtest-cli might not be in repos
                let status = Command::new("pip")
                    .args(&["install", "--user", "speedtest-cli"])
                    .status();

                match status {
                    Ok(s) if s.success() => {
                        println!("✅ speedtest-cli installed");
                        let _ = Command::new("speedtest-cli").status();
                    }
                    _ => println!("❌ Failed to install speedtest-cli"),
                }
            }
        }
    }
}

fn gaming_latency_test() {
    println!("🏓 Gaming Server Latency Test");
    println!("=============================");

    let gaming_servers = [
        ("Google DNS", "8.8.8.8"),
        ("Cloudflare DNS", "1.1.1.1"),
        ("Steam (Valve)", "208.78.164.9"),
        ("Discord", "162.159.130.233"),
        ("Custom server", ""),
    ];

    println!("🎮 Select server to test latency:");
    for (i, (name, ip)) in gaming_servers.iter().enumerate() {
        if !ip.is_empty() {
            println!("{}. {} ({})", i + 1, name, ip);
        } else {
            println!("{}. {}", i + 1, name);
        }
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select server")
        .items(
            &gaming_servers
                .iter()
                .map(|(name, ip)| {
                    if !ip.is_empty() {
                        format!("{} ({})", name, ip)
                    } else {
                        name.to_string()
                    }
                })
                .collect::<Vec<_>>(),
        )
        .default(2) // Steam
        .interact()
    else {
        return;
    };

    let target_ip = if choice == gaming_servers.len() - 1 {
        // Custom server
        let Ok(ip) = Input::<String>::new()
            .with_prompt("Enter server IP or hostname")
            .interact_text()
        else {
            return;
        };
        ip
    } else {
        gaming_servers[choice].1.to_string()
    };

    if !target_ip.is_empty() {
        println!("🏓 Testing latency to {}...", target_ip);
        let _ = Command::new("ping")
            .args(&["-c", "10", &target_ip])
            .status();
    }
}

fn local_network_test() {
    println!("📊 Local Network Performance");
    println!("============================");

    println!("🏠 Local network tests:");
    println!("  • Bandwidth to router/gateway");
    println!("  • Internal device communication");
    println!("  • Wi-Fi vs Ethernet comparison");

    println!("\n🔍 Network interface information:");
    let _ = Command::new("ip").arg("addr").status();

    println!("\n🏓 Gateway latency test:");
    // Get default gateway
    let gateway_result = Command::new("ip")
        .args(&["route", "show", "default"])
        .output();

    if let Ok(output) = gateway_result {
        let gateway_info = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = gateway_info.lines().next()
            && let Some(gateway_ip) = line.split_whitespace().nth(2)
        {
            println!("🎯 Testing latency to gateway ({})...", gateway_ip);
            let _ = Command::new("ping").args(&["-c", "5", gateway_ip]).status();
        }
    }
}

fn system_monitoring_setup() {
    println!("🔍 System Monitoring Setup");
    println!("==========================");

    println!("📊 System monitoring tools setup:");

    let monitoring_tools = [
        ("htop", "Interactive process viewer", true),
        ("btop", "Modern system monitor", true),
        ("iotop", "I/O monitoring", true),
        ("nethogs", "Network usage per process", true),
        ("nvtop", "GPU monitoring", true),
    ];

    println!("Available monitoring tools:");
    for (tool, description, _) in &monitoring_tools {
        let status = Command::new("which").arg(tool).status();
        let installed = status.map(|s| s.success()).unwrap_or(false);
        let marker = if installed { "✅" } else { "❌" };
        println!("  {} {} - {}", marker, tool, description);
    }

    let Ok(install_missing) = Confirm::new()
        .with_prompt("Install missing monitoring tools?")
        .default(true)
        .interact()
    else {
        return;
    };

    if install_missing {
        let packages: Vec<&str> = monitoring_tools.iter().map(|(tool, _, _)| *tool).collect();

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Monitoring tools installed"),
            _ => println!("❌ Some tools may not be available"),
        }
    }

    println!("\n🔧 Monitoring setup tips:");
    println!("  • Use htop for general system monitoring");
    println!("  • Use btop for modern interface");
    println!("  • Use nvtop for GPU monitoring during gaming");
    println!("  • Set up tmux for multiple monitoring views");
}

fn performance_logging() {
    println!("📈 Performance Logging");
    println!("=====================");

    println!("📝 Performance logging options:");
    println!("  • MangoHud for in-game performance logging");
    println!("  • Custom scripts for system monitoring");
    println!("  • Prometheus + Grafana for advanced monitoring");
    println!("  • Simple shell scripts for periodic logging");

    let logging_options = [
        "📊 Setup MangoHud logging",
        "📝 Create performance log script",
        "📈 Show existing logs",
        "🧹 Clean old log files",
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
        1 => create_performance_log_script(),
        2 => show_existing_logs(),
        3 => clean_log_files(),
        _ => return,
    }
}

fn setup_mangohud_logging() {
    println!("📊 Setup MangoHud Logging");
    println!("=========================");

    let mangohud_check = Command::new("which").arg("mangohud").status();
    match mangohud_check {
        Ok(s) if s.success() => {
            println!("✅ MangoHud is installed");

            let logs_dir = std::env::home_dir()
                .map(|h| h.join("Documents/MangoHud_Logs"))
                .unwrap_or_else(|| std::path::PathBuf::from("~/Documents/MangoHud_Logs"));

            if std::fs::create_dir_all(&logs_dir).is_err() {
                println!("❌ Failed to create logs directory");
                return;
            }

            println!("📁 Created logs directory: {}", logs_dir.display());

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

            println!("📝 MangoHud logging configuration:");
            println!("{}", config_addition);

            let Ok(update_config) = Confirm::new()
                .with_prompt("Add logging configuration to MangoHud.conf?")
                .default(true)
                .interact()
            else {
                return;
            };

            if update_config {
                let config_file = std::env::home_dir()
                    .map(|h| h.join(".config/MangoHud/MangoHud.conf"))
                    .unwrap_or_else(|| {
                        std::path::PathBuf::from("~/.config/MangoHud/MangoHud.conf")
                    });

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
                        println!("✅ MangoHud logging configuration added");
                        println!("🎮 Use F10 in-game to start/stop logging");
                    }
                }
            }
        }
        _ => {
            println!("❌ MangoHud not installed");
            println!("💡 Install MangoHud first for gaming performance logging");
        }
    }
}

fn create_performance_log_script() {
    println!("📝 Create Performance Log Script");
    println!("===============================");

    let script_dir = std::env::home_dir()
        .map(|h| h.join("bin"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/bin"));

    if std::fs::create_dir_all(&script_dir).is_err() {
        println!("❌ Failed to create scripts directory");
        return;
    }

    let script_path = script_dir.join("performance_logger.sh");
    let script_content = r#"#!/bin/bash
# Performance Logging Script

LOG_DIR="$HOME/Documents/PerformanceLogs"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
LOG_FILE="$LOG_DIR/performance_${TIMESTAMP}.log"

mkdir -p "$LOG_DIR"

echo "=== Performance Log Started at $(date) ===" > "$LOG_FILE"
echo "System: $(uname -a)" >> "$LOG_FILE"
echo "CPU: $(lscpu | grep 'Model name' | cut -d':' -f2 | xargs)" >> "$LOG_FILE"
echo "Memory: $(free -h | grep '^Mem' | awk '{print $2}')" >> "$LOG_FILE"
echo "" >> "$LOG_FILE"

# Log performance data every 5 seconds for 5 minutes
for i in {1..60}; do
    echo "=== Sample $i at $(date) ===" >> "$LOG_FILE"
    
    # CPU usage
    echo "CPU Usage:" >> "$LOG_FILE"
    top -bn1 | grep "Cpu(s)" >> "$LOG_FILE"
    
    # Memory usage
    echo "Memory Usage:" >> "$LOG_FILE"
    free -h >> "$LOG_FILE"
    
    # GPU usage (if available)
    if command -v nvidia-smi >/dev/null 2>&1; then
        echo "NVIDIA GPU:" >> "$LOG_FILE"
        nvidia-smi --query-gpu=utilization.gpu,memory.used,temperature.gpu --format=csv,noheader,nounits >> "$LOG_FILE"
    fi
    
    # Temperature
    if command -v sensors >/dev/null 2>&1; then
        echo "Temperatures:" >> "$LOG_FILE"
        sensors | grep -E "(Core|temp)" >> "$LOG_FILE"
    fi
    
    echo "" >> "$LOG_FILE"
    sleep 5
done

echo "=== Performance Log Ended at $(date) ===" >> "$LOG_FILE"
echo "Log saved to: $LOG_FILE"
"#;

    use std::fs::File;
    use std::io::Write;

    match File::create(&script_path) {
        Ok(mut file) => {
            if file.write_all(script_content.as_bytes()).is_err() {
                println!("❌ Failed to write performance script");
            } else {
                // Make script executable
                let _ = Command::new("chmod")
                    .args(&["+x", &script_path.to_string_lossy()])
                    .status();
                println!(
                    "✅ Performance logging script created: {}",
                    script_path.display()
                );
                println!("💡 Usage: {}", script_path.display());
                println!("📁 Logs will be saved to: ~/Documents/PerformanceLogs/");
            }
        }
        Err(_) => println!("❌ Failed to create performance script"),
    }
}

fn show_existing_logs() {
    println!("📈 Show Existing Performance Logs");
    println!("=================================");

    let log_directories = [
        "~/Documents/MangoHud_Logs",
        "~/Documents/PerformanceLogs",
        "~/Documents/GameLogs",
        "~/.local/share/lutris/logs",
    ];

    for log_dir in &log_directories {
        let expanded_path = if log_dir.starts_with("~/") {
            std::env::home_dir()
                .map(|h| h.join(&log_dir[2..]))
                .unwrap_or_else(|| std::path::PathBuf::from(log_dir))
        } else {
            std::path::PathBuf::from(log_dir)
        };

        if expanded_path.exists() {
            println!("\n📁 {}", expanded_path.display());
            let _ = Command::new("ls")
                .args(&["-lah", &expanded_path.to_string_lossy()])
                .status();
        }
    }
}

fn clean_log_files() {
    println!("🧹 Clean Old Log Files");
    println!("======================");

    let log_directories = [
        "~/Documents/MangoHud_Logs",
        "~/Documents/PerformanceLogs",
        "~/Documents/GameLogs",
    ];

    for log_dir in &log_directories {
        let expanded_path = if log_dir.starts_with("~/") {
            std::env::home_dir()
                .map(|h| h.join(&log_dir[2..]))
                .unwrap_or_else(|| std::path::PathBuf::from(log_dir))
        } else {
            std::path::PathBuf::from(log_dir)
        };

        if expanded_path.exists() {
            println!("\n📁 Checking: {}", expanded_path.display());

            let Ok(clean) = Confirm::new()
                .with_prompt(&format!(
                    "Clean log files older than 30 days in {}?",
                    expanded_path.display()
                ))
                .default(false)
                .interact()
            else {
                continue;
            };

            if clean {
                let status = Command::new("find")
                    .args(&[
                        &expanded_path.to_string_lossy(),
                        "-name",
                        "*.log",
                        "-mtime",
                        "+30",
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

fn custom_performance_profiles() {
    println!("🔧 Custom Performance Profiles");
    println!("==============================");

    println!("🎯 Performance profiles allow you to quickly switch between");
    println!("   different system configurations for various use cases:");
    println!("   • Gaming performance");
    println!("   • Power saving");
    println!("   • Balanced usage");
    println!("   • Maximum performance");

    let profile_options = [
        "🎮 Create gaming performance profile",
        "💡 Create power saving profile",
        "⚖️  Create balanced profile",
        "🚀 Create maximum performance profile",
        "📋 List existing profiles",
        "🔧 Apply performance profile",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Performance Profiles")
        .items(&profile_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => create_gaming_profile(),
        1 => create_power_saving_profile(),
        2 => create_balanced_profile(),
        3 => create_maximum_performance_profile(),
        4 => list_performance_profiles(),
        5 => apply_performance_profile(),
        _ => return,
    }
}

fn create_gaming_profile() {
    println!("🎮 Create Gaming Performance Profile");
    println!("===================================");

    let profile_content = r#"#!/bin/bash
# Gaming Performance Profile

echo "🎮 Applying gaming performance profile..."

# Set CPU governor to performance
echo "⚡ Setting CPU to performance mode..."
sudo cpupower frequency-set -g performance

# Set swappiness to low value for gaming
echo "💾 Optimizing memory management..."
sudo sysctl vm.swappiness=1

# Enable GameMode if available
if command -v gamemoded >/dev/null 2>&1; then
    echo "🚀 Starting GameMode daemon..."
    systemctl --user start gamemode
fi

# Disable unnecessary services (optional)
echo "🔧 Optimizing background services..."
# Add service management here

# Set I/O scheduler for gaming
echo "📁 Optimizing I/O scheduler..."
for dev in /sys/block/*/queue/scheduler; do
    if [[ -w "$dev" ]]; then
        echo kyber | sudo tee "$dev" >/dev/null
    fi
done

echo "✅ Gaming performance profile applied!"
echo "💡 Remember to apply this before gaming sessions"
"#;

    save_performance_profile("gaming", profile_content);
}

fn create_power_saving_profile() {
    println!("💡 Create Power Saving Profile");
    println!("==============================");

    let profile_content = r#"#!/bin/bash
# Power Saving Profile

echo "💡 Applying power saving profile..."

# Set CPU governor to powersave
echo "⚡ Setting CPU to power saving mode..."
sudo cpupower frequency-set -g powersave

# Increase swappiness for power saving
echo "💾 Configuring memory for power saving..."
sudo sysctl vm.swappiness=60

# Stop GameMode if running
if systemctl --user is-active gamemode >/dev/null 2>&1; then
    echo "🔋 Stopping GameMode..."
    systemctl --user stop gamemode
fi

# Set I/O scheduler for power efficiency
echo "📁 Setting power-efficient I/O scheduler..."
for dev in /sys/block/*/queue/scheduler; do
    if [[ -w "$dev" ]]; then
        echo bfq | sudo tee "$dev" >/dev/null
    fi
done

echo "✅ Power saving profile applied!"
echo "🔋 System optimized for battery life"
"#;

    save_performance_profile("powersave", profile_content);
}

fn create_balanced_profile() {
    println!("⚖️  Create Balanced Profile");
    println!("===========================");

    let profile_content = r#"#!/bin/bash
# Balanced Performance Profile

echo "⚖️  Applying balanced profile..."

# Set CPU governor to ondemand or schedutil
echo "⚡ Setting CPU to balanced mode..."
if grep -q schedutil /sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors; then
    sudo cpupower frequency-set -g schedutil
else
    sudo cpupower frequency-set -g ondemand
fi

# Set moderate swappiness
echo "💾 Configuring balanced memory management..."
sudo sysctl vm.swappiness=20

# Set I/O scheduler to deadline
echo "📁 Setting balanced I/O scheduler..."
for dev in /sys/block/*/queue/scheduler; do
    if [[ -w "$dev" ]]; then
        echo mq-deadline | sudo tee "$dev" >/dev/null
    fi
done

echo "✅ Balanced profile applied!"
echo "⚖️  Good balance of performance and efficiency"
"#;

    save_performance_profile("balanced", profile_content);
}

fn create_maximum_performance_profile() {
    println!("🚀 Create Maximum Performance Profile");
    println!("=====================================");

    let profile_content = r#"#!/bin/bash
# Maximum Performance Profile

echo "🚀 Applying maximum performance profile..."

# Set CPU governor to performance
echo "⚡ Setting CPU to maximum performance..."
sudo cpupower frequency-set -g performance

# Disable swap for maximum performance (dangerous!)
echo "💾 Optimizing memory for maximum performance..."
sudo sysctl vm.swappiness=1

# Enable all performance features
echo "🎮 Enabling performance features..."
if command -v gamemoded >/dev/null 2>&1; then
    systemctl --user start gamemode
fi

# Set high-performance I/O scheduler
echo "📁 Setting high-performance I/O..."
for dev in /sys/block/*/queue/scheduler; do
    if [[ -w "$dev" ]]; then
        echo none | sudo tee "$dev" >/dev/null 2>&1 || echo kyber | sudo tee "$dev" >/dev/null
    fi
done

# Disable CPU power saving features
echo "⚡ Disabling CPU power saving..."
for cpu in /sys/devices/system/cpu/cpu*/cpuidle/state*/disable; do
    if [[ -w "$cpu" ]]; then
        echo 1 | sudo tee "$cpu" >/dev/null
    fi
done

echo "✅ Maximum performance profile applied!"
echo "⚠️  High power consumption - use only when needed"
"#;

    save_performance_profile("maximum", profile_content);
}

fn save_performance_profile(name: &str, content: &str) {
    let profiles_dir = std::env::home_dir()
        .map(|h| h.join(".config/ghostctl/profiles"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config/ghostctl/profiles"));

    if std::fs::create_dir_all(&profiles_dir).is_err() {
        println!("❌ Failed to create profiles directory");
        return;
    }

    let profile_path = profiles_dir.join(format!("{}.sh", name));

    use std::fs::File;
    use std::io::Write;

    match File::create(&profile_path) {
        Ok(mut file) => {
            if file.write_all(content.as_bytes()).is_err() {
                println!("❌ Failed to write profile");
            } else {
                // Make script executable
                let _ = Command::new("chmod")
                    .args(&["+x", &profile_path.to_string_lossy()])
                    .status();
                println!(
                    "✅ Performance profile '{}' created: {}",
                    name,
                    profile_path.display()
                );
            }
        }
        Err(_) => println!("❌ Failed to create profile file"),
    }
}

fn list_performance_profiles() {
    println!("📋 List Existing Performance Profiles");
    println!("=====================================");

    let profiles_dir = std::env::home_dir()
        .map(|h| h.join(".config/ghostctl/profiles"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config/ghostctl/profiles"));

    if profiles_dir.exists() {
        println!(
            "📁 Performance profiles directory: {}",
            profiles_dir.display()
        );
        let _ = Command::new("ls")
            .args(&["-la", &profiles_dir.to_string_lossy()])
            .status();
    } else {
        println!("❌ No profiles directory found");
        println!("💡 Create some profiles first");
    }
}

fn apply_performance_profile() {
    println!("🔧 Apply Performance Profile");
    println!("============================");

    let profiles_dir = std::env::home_dir()
        .map(|h| h.join(".config/ghostctl/profiles"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config/ghostctl/profiles"));

    if !profiles_dir.exists() {
        println!("❌ No profiles directory found");
        println!("💡 Create some profiles first");
        return;
    }

    // List available profiles
    let profiles: Vec<String> = std::fs::read_dir(&profiles_dir)
        .map(|entries| {
            entries
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.extension().is_some_and(|ext| ext == "sh") {
                            path.file_stem()
                                .and_then(|stem| stem.to_str())
                                .map(|s| s.to_string())
                        } else {
                            None
                        }
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    if profiles.is_empty() {
        println!("❌ No profiles found");
        return;
    }

    println!("📋 Available profiles:");
    for (i, profile) in profiles.iter().enumerate() {
        println!("{}. {}", i + 1, profile);
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select profile to apply")
        .items(&profiles)
        .default(0)
        .interact()
    else {
        return;
    };

    let selected_profile = &profiles[choice];
    let profile_path = profiles_dir.join(format!("{}.sh", selected_profile));

    let Ok(confirm) = Confirm::new()
        .with_prompt(&format!(
            "Apply '{}' performance profile?",
            selected_profile
        ))
        .default(true)
        .interact()
    else {
        return;
    };

    if confirm {
        println!("🔧 Applying profile '{}'...", selected_profile);
        let status = Command::new("bash").arg(&profile_path).status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Profile '{}' applied successfully!", selected_profile)
            }
            _ => println!("❌ Failed to apply profile '{}'", selected_profile),
        }
    }
}

fn automatic_game_optimization() {
    println!("🚀 Automatic Game Optimization");
    println!("==============================");

    println!("🎯 Automatic game optimization features:");
    println!("  • Detect running games");
    println!("  • Apply performance profiles automatically");
    println!("  • Optimize system settings per game");
    println!("  • Monitor and adjust during gameplay");

    let auto_options = [
        "🔍 Detect currently running games",
        "⚙️  Setup automatic optimization rules",
        "📊 Configure optimization triggers",
        "🎮 Game-specific optimization database",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Automatic Game Optimization")
        .items(&auto_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => detect_running_games(),
        1 => setup_auto_optimization_rules(),
        2 => configure_optimization_triggers(),
        3 => game_optimization_database(),
        _ => return,
    }
}

fn detect_running_games() {
    println!("🔍 Detect Currently Running Games");
    println!("=================================");

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
        "factorio",
        "terraria",
        "rimworld",
        "cityskylinesii",
    ];

    println!("🎮 Scanning for running games...");
    let mut found_games = Vec::new();

    for game in &game_processes {
        let pgrep_output = Command::new("pgrep").args(&["-l", game]).output();

        if let Ok(output) = pgrep_output {
            let processes = String::from_utf8_lossy(&output.stdout);
            if !processes.trim().is_empty() {
                found_games.push(game);
                println!("  🎮 Found: {} - {}", game, processes.trim());
            }
        }
    }

    if found_games.is_empty() {
        println!("❌ No games currently detected");
        println!("💡 Games may not be in the detection list");
    } else {
        println!("\n✅ Detected {} running game(s)", found_games.len());

        let Ok(optimize_now) = Confirm::new()
            .with_prompt("Apply gaming optimizations for detected games?")
            .default(true)
            .interact()
        else {
            return;
        };

        if optimize_now {
            apply_gaming_optimizations();
        }
    }
}

fn apply_gaming_optimizations() {
    println!("🚀 Applying Gaming Optimizations");
    println!("================================");

    // Apply gaming performance profile if it exists
    let profiles_dir = std::env::home_dir()
        .map(|h| h.join(".config/ghostctl/profiles"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config/ghostctl/profiles"));

    let gaming_profile = profiles_dir.join("gaming.sh");
    if gaming_profile.exists() {
        println!("🎮 Applying gaming performance profile...");
        let _ = Command::new("bash").arg(&gaming_profile).status();
    }

    // Start GameMode if available
    if Command::new("which")
        .arg("gamemoderun")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        println!("🚀 Starting GameMode...");
        let _ = Command::new("systemctl")
            .args(&["--user", "start", "gamemode"])
            .status();
    }

    println!("✅ Gaming optimizations applied!");
}

fn setup_auto_optimization_rules() {
    println!("⚙️  Setup Automatic Optimization Rules");
    println!("======================================");

    println!("🤖 Automatic optimization rules can:");
    println!("  • Monitor for game launches");
    println!("  • Apply performance profiles automatically");
    println!("  • Restore normal settings when games close");
    println!("  • Send notifications about optimizations");

    println!("\n💡 This would typically involve:");
    println!("  • systemd user services");
    println!("  • Process monitoring scripts");
    println!("  • Configuration files for rules");
    println!("  • Integration with desktop environment");

    println!("\n🚧 Advanced feature - would require custom implementation");
    println!("💡 For now, use manual profile application");
}

fn configure_optimization_triggers() {
    println!("📊 Configure Optimization Triggers");
    println!("==================================");

    println!("⚡ Optimization triggers:");
    println!("  • Process name matching");
    println!("  • Window title detection");
    println!("  • Resource usage thresholds");
    println!("  • Time-based rules");
    println!("  • Manual activation");

    println!("\n🔧 Example trigger conditions:");
    println!("  • When 'steam' process starts → Apply gaming profile");
    println!("  • When GPU usage > 80% → Enable performance mode");
    println!("  • When on battery power → Apply power saving");
    println!("  • When plugged in → Enable performance mode");

    println!("\n💡 This feature requires advanced scripting and monitoring");
}

fn game_optimization_database() {
    println!("🎮 Game-specific Optimization Database");
    println!("======================================");

    let game_optimizations = [
        (
            "Counter-Strike 2",
            vec![
                "-high -threads 4 +fps_max 300",
                "DXVK_ASYNC=1",
                "gamemoderun %command%",
            ],
        ),
        (
            "Cyberpunk 2077",
            vec![
                "PROTON_NO_ESYNC=1",
                "DXVK_ASYNC=1",
                "Lower crowd density settings",
            ],
        ),
        (
            "Minecraft",
            vec![
                "Allocate 4-8GB RAM",
                "Use OptiFine or Sodium",
                "Disable VSync for higher FPS",
            ],
        ),
        (
            "Factorio",
            vec![
                "Disable autosave during gameplay",
                "Use faster graphics settings",
                "Enable multi-threading",
            ],
        ),
    ];

    println!("📚 Game-specific optimizations database:");
    for (game, optimizations) in &game_optimizations {
        println!("\n🎮 {}:", game);
        for opt in optimizations {
            println!("  • {}", opt);
        }
    }

    println!("\n💡 To add more games:");
    println!("  • Research game-specific optimizations");
    println!("  • Test configurations");
    println!("  • Document working solutions");
    println!("  • Share with the community");
}

fn performance_status_report() {
    println!("📋 Performance Status Report");
    println!("============================");

    println!("🖥️  System Information:");
    let _ = Command::new("uname").arg("-a").status();

    println!("\n💻 CPU Information:");
    let _ = Command::new("lscpu").status();

    println!("\n💾 Memory Status:");
    let _ = Command::new("free").arg("-h").status();

    println!("\n📊 Current CPU Governor:");
    let _ = Command::new("cat")
        .arg("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
        .status();

    println!("\n🌡️  System Temperatures:");
    let sensors_check = Command::new("which").arg("sensors").status();
    match sensors_check {
        Ok(s) if s.success() => {
            let _ = Command::new("sensors").status();
        }
        _ => println!("  ❌ lm_sensors not installed"),
    }

    println!("\n🎮 Gaming Tools Status:");
    let gaming_tools = [
        ("GameMode", "gamemoderun"),
        ("MangoHud", "mangohud"),
        ("Steam", "steam"),
        ("Lutris", "lutris"),
    ];

    for (tool, command) in &gaming_tools {
        let status = Command::new("which").arg(command).status();
        match status {
            Ok(s) if s.success() => println!("  ✅ {} available", tool),
            _ => println!("  ❌ {} not found", tool),
        }
    }

    println!("\n📈 Performance Recommendations:");
    generate_performance_recommendations();
}

fn generate_performance_recommendations() {
    println!("💡 Performance optimization recommendations:");

    // Check CPU governor
    let governor_output = Command::new("cat")
        .arg("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
        .output();

    if let Ok(output) = governor_output {
        let output_string = String::from_utf8_lossy(&output.stdout);
        let governor = output_string.trim().to_string();
        match governor.as_str() {
            "powersave" => println!(
                "  ⚡ Consider switching to 'performance' or 'schedutil' governor for gaming"
            ),
            "performance" => println!("  ✅ CPU governor optimized for performance"),
            _ => println!(
                "  💡 CPU governor: {} (consider 'performance' for gaming)",
                governor
            ),
        }
    }

    // Check GameMode
    let gamemode_check = Command::new("which").arg("gamemoderun").status();
    if gamemode_check.is_err() {
        println!("  🚀 Install GameMode for better gaming performance");
    }

    // Check MangoHud
    let mangohud_check = Command::new("which").arg("mangohud").status();
    if mangohud_check.is_err() {
        println!("  📊 Install MangoHud for performance monitoring");
    }

    // Check memory usage
    let free_output = Command::new("free").output();
    if let Ok(output) = free_output {
        let free_info = String::from_utf8_lossy(&output.stdout);
        if let Some(mem_line) = free_info.lines().nth(1) {
            let parts: Vec<&str> = mem_line.split_whitespace().collect();
            if parts.len() >= 3
                && let (Ok(total), Ok(used)) = (parts[1].parse::<u64>(), parts[2].parse::<u64>())
            {
                let usage_percent = (used * 100) / total;
                if usage_percent > 80 {
                    println!(
                        "  💾 High memory usage ({}%) - consider closing background applications",
                        usage_percent
                    );
                }
            }
        }
    }

    println!("  🔧 Run individual optimization tools for specific improvements");
    println!("  📚 Check game-specific optimization database for better performance");
}
