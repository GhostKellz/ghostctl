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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Swap & Zram Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

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
    if Path::new("/proc/swaps").exists()
        && let Ok(content) = fs::read_to_string("/proc/swaps")
    {
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

                if Path::new(&comp_path).exists()
                    && let (Ok(comp), Ok(size)) = (
                        fs::read_to_string(&comp_path),
                        fs::read_to_string(&size_path),
                    )
                    && let Ok(size_bytes) = size.trim().parse::<u64>()
                    && size_bytes > 0
                {
                    let size_mb = size_bytes / 1024 / 1024;
                    println!("  zram{}: {} algorithm, {} MB", i, comp.trim(), size_mb);
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
    if let Ok(content) = fs::read_to_string("/proc/swaps")
        && content.contains("zram")
    {
        println!("⚠️  Zram is already active");
        let reconfigure = match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Reconfigure zram?")
            .interact_opt()
        {
            Ok(Some(c)) => c,
            _ => return,
        };

        if !reconfigure {
            return;
        }

        // Disable existing zram
        disable_zram();
    }

    // Get system memory
    let mem_info = get_system_memory();
    let recommended_size = mem_info / 2; // Use half of RAM

    println!("💡 System RAM: {} MB", mem_info);
    println!("💡 Recommended zram size: {} MB", recommended_size);

    let size: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Zram size in MB")
        .default(recommended_size.to_string())
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    let compression_algorithms = ["lzo", "lz4", "zstd", "lzo-rle"];
    let comp_choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Compression algorithm")
        .items(&compression_algorithms)
        .default(2) // zstd is usually best
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

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
            if line.starts_with("MemTotal:")
                && let Some(kb_str) = line.split_whitespace().nth(1)
                && let Ok(kb) = kb_str.parse::<u64>()
            {
                return kb / 1024; // Convert to MB
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
            if let Ok(current_size) = fs::read_to_string(&size_path)
                && current_size.trim() == "0"
            {
                // Device is available
                setup_zram_device(i, size, algorithm);
                return;
            }
        }
    }

    println!("❌ No available zram devices found");
}

fn setup_zram_device(device_num: u32, size: &str, algorithm: &str) {
    use std::io::Write;
    let device = format!("zram{}", device_num);
    let size_bytes = size.parse::<u64>().unwrap_or(1024) * 1024 * 1024;

    // Set compression algorithm using sudo tee
    let comp_path = format!("/sys/block/{}/comp_algorithm", device);
    let _ = Command::new("sudo")
        .args(["tee", &comp_path])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .spawn()
        .and_then(|mut child| {
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(algorithm.as_bytes());
            }
            child.wait()
        });

    // Set size using sudo tee
    let size_path = format!("/sys/block/{}/disksize", device);
    let _ = Command::new("sudo")
        .args(["tee", &size_path])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .spawn()
        .and_then(|mut child| {
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(size_bytes.to_string().as_bytes());
            }
            child.wait()
        });

    // Create swap
    let _ = Command::new("sudo")
        .args(["mkswap", &format!("/dev/{}", device)])
        .status();

    // Enable swap
    let _ = Command::new("sudo")
        .args(["swapon", &format!("/dev/{}", device)])
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

            let proceed = match Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Continue with creating new swap file?")
                .interact_opt()
            {
                Ok(Some(c)) => c,
                _ => return,
            };

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

    let size: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Swap file size in MB")
        .default(recommended_size.to_string())
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    let location: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Swap file location")
        .default("/swapfile".to_string())
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

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

    // Add to fstab by reading, appending, and writing back
    let fstab_entry = format!("{} none swap sw 0 0\n", location);
    if let Ok(mut fstab_content) = fs::read_to_string("/etc/fstab") {
        // Check if entry already exists
        if !fstab_content.contains(location) {
            fstab_content.push_str(&fstab_entry);
            let temp_file = "/tmp/fstab.tmp";
            if fs::write(temp_file, &fstab_content).is_ok() {
                let _ = Command::new("sudo")
                    .args(["mv", temp_file, "/etc/fstab"])
                    .status();
            }
        }
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
    let swappiness: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Swappiness (0-100, 10 recommended for SSD)")
        .default("10".to_string())
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    let cache_pressure: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Cache pressure (default 100, 50 for performance)")
        .default("50".to_string())
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    // Apply settings using sudo tee
    use std::io::Write;
    let _ = Command::new("sudo")
        .args(["tee", "/proc/sys/vm/swappiness"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .spawn()
        .and_then(|mut child| {
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(swappiness.as_bytes());
            }
            child.wait()
        });

    let _ = Command::new("sudo")
        .args(["tee", "/proc/sys/vm/vfs_cache_pressure"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .spawn()
        .and_then(|mut child| {
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(cache_pressure.as_bytes());
            }
            child.wait()
        });

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
    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to disable?")
        .items(&options)
        .default(3)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

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
    use std::io::Write;
    println!("Disabling zram...");

    // Find and disable zram devices
    for i in 0..8 {
        let device = format!("/dev/zram{}", i);
        if Path::new(&device).exists() {
            let _ = Command::new("sudo").args(["swapoff", &device]).status();

            // Reset zram using sudo tee
            let reset_path = format!("/sys/block/zram{}/reset", i);
            let _ = Command::new("sudo")
                .args(["tee", &reset_path])
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::null())
                .spawn()
                .and_then(|mut child| {
                    if let Some(ref mut stdin) = child.stdin {
                        let _ = stdin.write_all(b"1");
                    }
                    child.wait()
                });
        }
    }

    // Disable service
    let _ = Command::new("sudo")
        .args(["systemctl", "disable", "zram.service"])
        .status();

    let _ = Command::new("sudo")
        .args(["rm", "-f", "/etc/systemd/system/zram.service"])
        .status();

    println!("✅ Zram disabled");
}

fn disable_swap_files() {
    println!("Disabling swap files...");

    // Get list of swap files from /proc/swaps
    if let Ok(content) = fs::read_to_string("/proc/swaps") {
        for line in content.lines().skip(1) {
            if let Some(device) = line.split_whitespace().next()
                && device.starts_with('/')
                && !device.contains("zram")
            {
                let _ = Command::new("sudo").args(&["swapoff", device]).status();
                println!("  Disabled: {}", device);
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
    let device: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Device to modify (e.g., /dev/zram0, /swapfile)")
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    let priority: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Priority (-1 to 32767, higher = more preferred)")
        .default("100".to_string())
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

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
        if Path::new(&stats_path).exists()
            && let Ok(stats) = fs::read_to_string(&stats_path)
        {
            println!("zram{}: {}", i, stats.trim());
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

// ============= Utility functions for testing =============

/// Parse MemTotal from /proc/meminfo content
pub fn parse_meminfo_total(content: &str) -> Option<u64> {
    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            if let Some(kb_str) = line.split_whitespace().nth(1) {
                if let Ok(kb) = kb_str.parse::<u64>() {
                    return Some(kb / 1024); // Convert to MB
                }
            }
        }
    }
    None
}

/// Calculate recommended swap size based on RAM
pub fn calculate_recommended_swap(ram_mb: u64) -> u64 {
    if ram_mb < 2048 {
        ram_mb * 2 // Double RAM if less than 2GB
    } else if ram_mb < 8192 {
        ram_mb // Equal to RAM if 2-8GB
    } else {
        ram_mb / 2 // Half RAM if more than 8GB
    }
}

/// Calculate recommended zram size (typically half of RAM)
pub fn calculate_recommended_zram(ram_mb: u64) -> u64 {
    ram_mb / 2
}

/// Validate swappiness value (0-100)
pub fn is_valid_swappiness(value: u32) -> bool {
    value <= 100
}

/// Validate cache pressure value (typically 0-200)
pub fn is_valid_cache_pressure(value: u32) -> bool {
    value <= 500 // Allow up to 500 for edge cases
}

/// Parse swap device info from /proc/swaps line
pub fn parse_swap_line(line: &str) -> Option<SwapDeviceInfo> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 5 {
        let filename = parts[0].to_string();
        let swap_type = parts[1].to_string();
        let size = parts[2].parse::<u64>().ok()?;
        let used = parts[3].parse::<u64>().ok()?;
        let priority = parts[4].parse::<i32>().ok()?;
        return Some(SwapDeviceInfo {
            filename,
            swap_type,
            size_kb: size,
            used_kb: used,
            priority,
        });
    }
    None
}

/// Represents a swap device
#[derive(Debug, Clone, PartialEq)]
pub struct SwapDeviceInfo {
    pub filename: String,
    pub swap_type: String,
    pub size_kb: u64,
    pub used_kb: u64,
    pub priority: i32,
}

impl SwapDeviceInfo {
    /// Check if this is a zram device
    pub fn is_zram(&self) -> bool {
        self.filename.contains("zram")
    }

    /// Check if this is a swap file
    pub fn is_file(&self) -> bool {
        self.swap_type == "file"
    }

    /// Get usage percentage
    pub fn usage_percent(&self) -> f64 {
        if self.size_kb == 0 {
            0.0
        } else {
            (self.used_kb as f64 / self.size_kb as f64) * 100.0
        }
    }
}

/// Validate compression algorithm for zram
pub fn is_valid_zram_algorithm(algorithm: &str) -> bool {
    matches!(
        algorithm.to_lowercase().as_str(),
        "lzo" | "lz4" | "zstd" | "lzo-rle"
    )
}

/// Parse zram disksize from sysfs (returns bytes)
pub fn parse_zram_disksize(content: &str) -> Option<u64> {
    content.trim().parse::<u64>().ok()
}

/// Convert bytes to megabytes
pub fn bytes_to_mb(bytes: u64) -> u64 {
    bytes / 1024 / 1024
}

/// Convert megabytes to bytes
pub fn mb_to_bytes(mb: u64) -> u64 {
    mb * 1024 * 1024
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_meminfo_total_valid() {
        let content = "MemTotal:       16384000 kB\nMemFree:         8000000 kB";
        assert_eq!(parse_meminfo_total(content), Some(16000)); // ~16GB in MB
    }

    #[test]
    fn test_parse_meminfo_total_8gb() {
        let content = "MemTotal:        8192000 kB\nMemFree:         4000000 kB";
        assert_eq!(parse_meminfo_total(content), Some(8000));
    }

    #[test]
    fn test_parse_meminfo_total_missing() {
        let content = "MemFree:         4000000 kB\nBuffers:         1000000 kB";
        assert_eq!(parse_meminfo_total(content), None);
    }

    #[test]
    fn test_calculate_recommended_swap_low_ram() {
        assert_eq!(calculate_recommended_swap(1024), 2048); // 1GB -> 2GB
        assert_eq!(calculate_recommended_swap(512), 1024); // 512MB -> 1GB
    }

    #[test]
    fn test_calculate_recommended_swap_medium_ram() {
        assert_eq!(calculate_recommended_swap(4096), 4096); // 4GB -> 4GB
        assert_eq!(calculate_recommended_swap(8000), 8000); // 8GB -> 8GB
    }

    #[test]
    fn test_calculate_recommended_swap_high_ram() {
        assert_eq!(calculate_recommended_swap(16384), 8192); // 16GB -> 8GB
        assert_eq!(calculate_recommended_swap(32768), 16384); // 32GB -> 16GB
    }

    #[test]
    fn test_calculate_recommended_zram() {
        assert_eq!(calculate_recommended_zram(16384), 8192);
        assert_eq!(calculate_recommended_zram(8192), 4096);
        assert_eq!(calculate_recommended_zram(4096), 2048);
    }

    #[test]
    fn test_is_valid_swappiness() {
        assert!(is_valid_swappiness(0));
        assert!(is_valid_swappiness(10));
        assert!(is_valid_swappiness(60));
        assert!(is_valid_swappiness(100));
        assert!(!is_valid_swappiness(101));
        assert!(!is_valid_swappiness(200));
    }

    #[test]
    fn test_is_valid_cache_pressure() {
        assert!(is_valid_cache_pressure(0));
        assert!(is_valid_cache_pressure(50));
        assert!(is_valid_cache_pressure(100));
        assert!(is_valid_cache_pressure(500));
        assert!(!is_valid_cache_pressure(501));
    }

    #[test]
    fn test_parse_swap_line_zram() {
        let line = "/dev/zram0                              partition	4194300	0	100";
        let info = parse_swap_line(line);
        assert!(info.is_some());
        if let Some(info) = info {
            assert_eq!(info.filename, "/dev/zram0");
            assert_eq!(info.swap_type, "partition");
            assert_eq!(info.size_kb, 4194300);
            assert_eq!(info.used_kb, 0);
            assert_eq!(info.priority, 100);
            assert!(info.is_zram());
        }
    }

    #[test]
    fn test_parse_swap_line_file() {
        let line = "/swapfile                               file		8388604	1048576	-2";
        let info = parse_swap_line(line);
        assert!(info.is_some());
        if let Some(info) = info {
            assert_eq!(info.filename, "/swapfile");
            assert_eq!(info.swap_type, "file");
            assert!(info.is_file());
            assert!(!info.is_zram());
        }
    }

    #[test]
    fn test_parse_swap_line_header() {
        let line = "Filename				Type		Size		Used		Priority";
        let info = parse_swap_line(line);
        assert!(info.is_none());
    }

    #[test]
    fn test_swap_device_info_usage_percent() {
        let info = SwapDeviceInfo {
            filename: "/swapfile".to_string(),
            swap_type: "file".to_string(),
            size_kb: 8000000,
            used_kb: 4000000,
            priority: -2,
        };
        assert!((info.usage_percent() - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_swap_device_info_usage_percent_empty() {
        let info = SwapDeviceInfo {
            filename: "/dev/zram0".to_string(),
            swap_type: "partition".to_string(),
            size_kb: 4000000,
            used_kb: 0,
            priority: 100,
        };
        assert!((info.usage_percent() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_swap_device_info_usage_percent_zero_size() {
        let info = SwapDeviceInfo {
            filename: "/dev/zram0".to_string(),
            swap_type: "partition".to_string(),
            size_kb: 0,
            used_kb: 0,
            priority: 100,
        };
        assert!((info.usage_percent() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_is_valid_zram_algorithm() {
        assert!(is_valid_zram_algorithm("lzo"));
        assert!(is_valid_zram_algorithm("lz4"));
        assert!(is_valid_zram_algorithm("zstd"));
        assert!(is_valid_zram_algorithm("lzo-rle"));
        assert!(is_valid_zram_algorithm("ZSTD")); // case insensitive
        assert!(!is_valid_zram_algorithm("gzip"));
        assert!(!is_valid_zram_algorithm(""));
    }

    #[test]
    fn test_parse_zram_disksize() {
        assert_eq!(parse_zram_disksize("4294967296"), Some(4294967296)); // 4GB
        assert_eq!(parse_zram_disksize("8589934592\n"), Some(8589934592)); // 8GB with newline
        assert_eq!(parse_zram_disksize("0"), Some(0));
        assert_eq!(parse_zram_disksize("invalid"), None);
    }

    #[test]
    fn test_bytes_to_mb() {
        assert_eq!(bytes_to_mb(1048576), 1); // 1MB
        assert_eq!(bytes_to_mb(1073741824), 1024); // 1GB
        assert_eq!(bytes_to_mb(4294967296), 4096); // 4GB
    }

    #[test]
    fn test_mb_to_bytes() {
        assert_eq!(mb_to_bytes(1), 1048576); // 1MB
        assert_eq!(mb_to_bytes(1024), 1073741824); // 1GB
        assert_eq!(mb_to_bytes(4096), 4294967296); // 4GB
    }
}
