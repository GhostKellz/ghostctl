use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn swap_menu() {
    println!("💾 Swap & Zram Management");
    println!("========================");

    let options = [
        "📊 Show current status",
        "⚡ Setup zram",
        "💾 Setup swap file",
        "🎛️  Tune swap settings",
        "❌ Disable swap/zram",
        "🔄 Configure swap priority",
        "📈 Performance analysis",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Swap & Zram Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => show_swap_status(),
        1 => setup_zram(),
        2 => setup_swap_file(),
        3 => tune_swap_settings(),
        4 => disable_swap(),
        5 => configure_swap_priority(),
        6 => performance_analysis(),
        _ => return,
    }
}

pub fn show_swap_status() {
    println!("📊 Current swap/zram status:\n");

    // Show swap devices
    println!("=== SWAP DEVICES ===");
    let _ = Command::new("swapon").args(&["--show"]).status();

    // Show memory usage
    println!("\n=== MEMORY USAGE ===");
    let _ = Command::new("free").args(&["-h"]).status();

    // Show zram status if available
    println!("\n=== ZRAM STATUS ===");
    if Path::new("/proc/swaps").exists() {
        if let Ok(content) = fs::read_to_string("/proc/swaps") {
            let zram_devices: Vec<&str> = content
                .lines()
                .filter(|line| line.contains("zram"))
                .collect();

            if zram_devices.is_empty() {
                println!("No zram devices active");
            } else {
                for device in zram_devices {
                    println!("{}", device);
                }

                // Show zram details
                for i in 0..8 {
                    let comp_path = format!("/sys/block/zram{}/comp_algorithm", i);
                    let size_path = format!("/sys/block/zram{}/disksize", i);

                    if Path::new(&comp_path).exists() {
                        if let (Ok(comp), Ok(size)) = (
                            fs::read_to_string(&comp_path),
                            fs::read_to_string(&size_path),
                        ) {
                            if let Ok(size_bytes) = size.trim().parse::<u64>() {
                                if size_bytes > 0 {
                                    let size_mb = size_bytes / 1024 / 1024;
                                    println!(
                                        "  zram{}: {} algorithm, {} MB",
                                        i,
                                        comp.trim(),
                                        size_mb
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Show current swappiness
    if let Ok(swappiness) = fs::read_to_string("/proc/sys/vm/swappiness") {
        println!("\n=== SWAP SETTINGS ===");
        println!("Swappiness: {}", swappiness.trim());
    }

    if let Ok(cache_pressure) = fs::read_to_string("/proc/sys/vm/vfs_cache_pressure") {
        println!("Cache pressure: {}", cache_pressure.trim());
    }
}

pub fn setup_zram() {
    println!("⚡ Setting up zram...");

    // Check if zram is already active
    if let Ok(content) = fs::read_to_string("/proc/swaps") {
        if content.contains("zram") {
            println!("⚠️  Zram is already active");
            let reconfigure = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Reconfigure zram?")
                .interact()
                .unwrap();

            if !reconfigure {
                return;
            }

            // Disable existing zram
            disable_zram();
        }
    }

    // Get system memory
    let mem_info = get_system_memory();
    let recommended_size = mem_info / 2; // Use half of RAM

    println!("💡 System RAM: {} MB", mem_info);
    println!("💡 Recommended zram size: {} MB", recommended_size);

    let size: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Zram size in MB")
        .default(recommended_size.to_string())
        .interact()
        .unwrap();

    let compression_algorithms = ["lzo", "lz4", "zstd", "lzo-rle"];
    let comp_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Compression algorithm")
        .items(&compression_algorithms)
        .default(2) // zstd is usually best
        .interact()
        .unwrap();

    let algorithm = compression_algorithms[comp_choice];

    // Create zram device
    create_zram_device(&size, algorithm);

    // Create systemd service for persistence
    create_zram_service(&size, algorithm);

    println!("✅ Zram setup completed");
}

fn get_system_memory() -> u64 {
    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                if let Some(kb_str) = line.split_whitespace().nth(1) {
                    if let Ok(kb) = kb_str.parse::<u64>() {
                        return kb / 1024; // Convert to MB
                    }
                }
            }
        }
    }
    4096 // Default fallback
}

fn create_zram_device(size: &str, algorithm: &str) {
    println!("Creating zram device...");

    // Load zram module
    let _ = Command::new("sudo").args(&["modprobe", "zram"]).status();

    // Find available zram device
    for i in 0..8 {
        let device_path = format!("/dev/zram{}", i);
        let size_path = format!("/sys/block/zram{}/disksize", i);

        if Path::new(&device_path).exists() {
            // Check if device is available
            if let Ok(current_size) = fs::read_to_string(&size_path) {
                if current_size.trim() == "0" {
                    // Device is available
                    setup_zram_device(i, size, algorithm);
                    return;
                }
            }
        }
    }

    println!("❌ No available zram devices found");
}

fn setup_zram_device(device_num: u32, size: &str, algorithm: &str) {
    let device = format!("zram{}", device_num);
    let size_bytes = size.parse::<u64>().unwrap_or(1024) * 1024 * 1024;

    // Set compression algorithm
    let _ = Command::new("sudo")
        .args(&[
            "sh",
            "-c",
            &format!("echo {} > /sys/block/{}/comp_algorithm", algorithm, device),
        ])
        .status();

    // Set size
    let _ = Command::new("sudo")
        .args(&[
            "sh",
            "-c",
            &format!("echo {} > /sys/block/{}/disksize", size_bytes, device),
        ])
        .status();

    // Create swap
    let _ = Command::new("sudo")
        .args(&["mkswap", &format!("/dev/{}", device)])
        .status();

    // Enable swap
    let _ = Command::new("sudo")
        .args(&["swapon", &format!("/dev/{}", device)])
        .status();

    println!(
        "✅ Zram device {} configured with {} compression",
        device, algorithm
    );
}

fn create_zram_service(size: &str, algorithm: &str) {
    let service_content = format!(
        r#"[Unit]
Description=Zram swap
After=multi-user.target

[Service]
Type=oneshot
RemainAfterExit=yes
ExecStart=/bin/sh -c 'modprobe zram && echo {} > /sys/block/zram0/comp_algorithm && echo {} > /sys/block/zram0/disksize && mkswap /dev/zram0 && swapon /dev/zram0'
ExecStop=/bin/sh -c 'swapoff /dev/zram0 && echo 1 > /sys/block/zram0/reset'

[Install]
WantedBy=multi-user.target
"#,
        algorithm,
        size.parse::<u64>().unwrap_or(1024) * 1024 * 1024
    );

    if fs::write("/tmp/zram.service", &service_content).is_ok() {
        let _ = Command::new("sudo")
            .args(&["mv", "/tmp/zram.service", "/etc/systemd/system/"])
            .status();

        let _ = Command::new("sudo")
            .args(&["systemctl", "enable", "zram.service"])
            .status();

        println!("✅ Zram service created and enabled");
    }
}

pub fn setup_swap_file() {
    println!("💾 Setting up swap file...");

    // Check existing swap
    if let Ok(content) = fs::read_to_string("/proc/swaps") {
        let swap_files: Vec<&str> = content
            .lines()
            .filter(|line| line.contains("/swapfile") || line.contains("/swap"))
            .collect();

        if !swap_files.is_empty() {
            println!("⚠️  Existing swap files found:");
            for swap in swap_files {
                println!("  {}", swap);
            }

            let proceed = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Continue with creating new swap file?")
                .interact()
                .unwrap();

            if !proceed {
                return;
            }
        }
    }

    let mem_info = get_system_memory();
    let recommended_size = if mem_info < 2048 {
        mem_info * 2 // Double RAM if less than 2GB
    } else if mem_info < 8192 {
        mem_info // Equal to RAM if 2-8GB
    } else {
        mem_info / 2 // Half RAM if more than 8GB
    };

    println!("💡 System RAM: {} MB", mem_info);
    println!("💡 Recommended swap size: {} MB", recommended_size);

    let size: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Swap file size in MB")
        .default(recommended_size.to_string())
        .interact()
        .unwrap();

    let location: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Swap file location")
        .default("/swapfile".to_string())
        .interact()
        .unwrap();

    create_swap_file(&size, &location);
}

fn create_swap_file(size: &str, location: &str) {
    println!("Creating swap file at {}...", location);

    let size_mb = size.parse::<u64>().unwrap_or(1024);

    // Create swap file
    let _ = Command::new("sudo")
        .args(&["fallocate", "-l", &format!("{}M", size_mb), location])
        .status();

    // Set permissions
    let _ = Command::new("sudo")
        .args(&["chmod", "600", location])
        .status();

    // Make swap
    let _ = Command::new("sudo").args(&["mkswap", location]).status();

    // Enable swap
    let _ = Command::new("sudo").args(&["swapon", location]).status();

    // Add to fstab
    let fstab_entry = format!("{} none swap sw 0 0\n", location);
    if fs::write("/tmp/fstab_entry", &fstab_entry).is_ok() {
        let _ = Command::new("sudo")
            .args(&["sh", "-c", &format!("cat /tmp/fstab_entry >> /etc/fstab")])
            .status();

        let _ = fs::remove_file("/tmp/fstab_entry");
    }

    println!("✅ Swap file created and enabled");
}

pub fn tune_swap_settings() {
    println!("🎛️  Tuning swap settings...");

    // Show current settings
    if let Ok(swappiness) = fs::read_to_string("/proc/sys/vm/swappiness") {
        println!("Current swappiness: {}", swappiness.trim());
    }

    if let Ok(cache_pressure) = fs::read_to_string("/proc/sys/vm/vfs_cache_pressure") {
        println!("Current cache pressure: {}", cache_pressure.trim());
    }

    // Get new swappiness value
    let swappiness: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Swappiness (0-100, 10 recommended for SSD)")
        .default("10".to_string())
        .interact()
        .unwrap();

    let cache_pressure: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Cache pressure (default 100, 50 for performance)")
        .default("50".to_string())
        .interact()
        .unwrap();

    // Apply settings
    let _ = Command::new("sudo")
        .args(&[
            "sh",
            "-c",
            &format!("echo {} > /proc/sys/vm/swappiness", swappiness),
        ])
        .status();

    let _ = Command::new("sudo")
        .args(&[
            "sh",
            "-c",
            &format!("echo {} > /proc/sys/vm/vfs_cache_pressure", cache_pressure),
        ])
        .status();

    // Make persistent
    let sysctl_content = format!(
        "vm.swappiness={}\nvm.vfs_cache_pressure={}\n",
        swappiness, cache_pressure
    );
    if fs::write("/tmp/99-ghostctl-swap.conf", &sysctl_content).is_ok() {
        let _ = Command::new("sudo")
            .args(&["mv", "/tmp/99-ghostctl-swap.conf", "/etc/sysctl.d/"])
            .status();

        println!("✅ Swap settings applied and made persistent");
    }
}

pub fn disable_swap() {
    println!("❌ Disabling swap/zram...");

    let options = [
        "Disable all swap",
        "Disable zram only",
        "Disable swap files only",
        "Cancel",
    ];
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to disable?")
        .items(&options)
        .default(3)
        .interact()
        .unwrap();

    match choice {
        0 => {
            // Disable all swap
            let _ = Command::new("sudo").args(&["swapoff", "-a"]).status();
            println!("✅ All swap disabled");
        }
        1 => disable_zram(),
        2 => disable_swap_files(),
        _ => return,
    }
}

fn disable_zram() {
    println!("Disabling zram...");

    // Find and disable zram devices
    for i in 0..8 {
        let device = format!("/dev/zram{}", i);
        if Path::new(&device).exists() {
            let _ = Command::new("sudo").args(&["swapoff", &device]).status();

            let _ = Command::new("sudo")
                .args(&["sh", "-c", &format!("echo 1 > /sys/block/zram{}/reset", i)])
                .status();
        }
    }

    // Disable service
    let _ = Command::new("sudo")
        .args(&["systemctl", "disable", "zram.service"])
        .status();

    let _ = Command::new("sudo")
        .args(&["rm", "-f", "/etc/systemd/system/zram.service"])
        .status();

    println!("✅ Zram disabled");
}

fn disable_swap_files() {
    println!("Disabling swap files...");

    // Get list of swap files from /proc/swaps
    if let Ok(content) = fs::read_to_string("/proc/swaps") {
        for line in content.lines().skip(1) {
            if let Some(device) = line.split_whitespace().next() {
                if device.starts_with('/') && !device.contains("zram") {
                    let _ = Command::new("sudo").args(&["swapoff", device]).status();
                    println!("  Disabled: {}", device);
                }
            }
        }
    }

    println!("⚠️  Remember to remove swap entries from /etc/fstab manually");
}

pub fn configure_swap_priority() {
    println!("🔄 Configuring swap priority...");

    // Show current swap devices with priorities
    println!("Current swap devices:");
    let _ = Command::new("swapon")
        .args(&["--show=NAME,SIZE,PRIO"])
        .status();

    // Get device to modify
    let device: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Device to modify (e.g., /dev/zram0, /swapfile)")
        .interact()
        .unwrap();

    let priority: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Priority (-1 to 32767, higher = more preferred)")
        .default("100".to_string())
        .interact()
        .unwrap();

    // Disable and re-enable with new priority
    let _ = Command::new("sudo").args(&["swapoff", &device]).status();

    let _ = Command::new("sudo")
        .args(&["swapon", "--priority", &priority, &device])
        .status();

    println!("✅ Priority set for {}", device);
}

pub fn performance_analysis() {
    println!("📈 Swap performance analysis...");

    println!("\n=== MEMORY PRESSURE ===");
    if let Ok(content) = fs::read_to_string("/proc/vmstat") {
        for line in content.lines() {
            if line.starts_with("pswpin") || line.starts_with("pswpout") {
                println!("{}", line);
            }
        }
    }

    println!("\n=== ZRAM STATS ===");
    for i in 0..4 {
        let stats_path = format!("/sys/block/zram{}/stat", i);
        if Path::new(&stats_path).exists() {
            if let Ok(stats) = fs::read_to_string(&stats_path) {
                println!("zram{}: {}", i, stats.trim());
            }
        }
    }

    println!("\n=== RECOMMENDATIONS ===");
    let mem_info = get_system_memory();

    if mem_info < 4096 {
        println!(
            "💡 System has {}MB RAM - consider using zram for better performance",
            mem_info
        );
    } else if mem_info > 16384 {
        println!(
            "💡 System has {}MB RAM - you may not need much swap",
            mem_info
        );
    }

    // Check current swappiness
    if let Ok(swappiness) = fs::read_to_string("/proc/sys/vm/swappiness") {
        let swap_val = swappiness.trim().parse::<u32>().unwrap_or(60);
        if swap_val > 10 {
            println!("💡 Consider lowering swappiness to 10 for SSD systems");
        }
    }
}
