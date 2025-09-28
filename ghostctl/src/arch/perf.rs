use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::process::Command;

pub fn tune() {
    let options = [
        "⚡ Quick Performance Optimization",
        "🔧 Advanced System Tuning",
        "📊 Performance Analysis",
        "⚙️  Kernel Parameters",
        "💾 Memory & Swap Optimization",
        "🚀 Boot Time Optimization",
        "🔋 Power Management",
        "📈 System Monitoring Setup",
        "🎯 Custom Performance Profiles",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🚀 Arch Performance Tuning")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => quick_optimization(),
        1 => advanced_tuning(),
        2 => performance_analysis(),
        3 => kernel_parameters(),
        4 => memory_swap_optimization(),
        5 => boot_optimization(),
        6 => power_management(),
        7 => monitoring_setup(),
        8 => custom_profiles(),
        _ => return,
    }
}

fn quick_optimization() {
    println!("⚡ Quick System Performance Optimization");
    println!("========================================");

    let confirm = Confirm::new()
        .with_prompt("Apply basic performance optimizations?")
        .default(true)
        .interact()
        .unwrap();

    if !confirm {
        return;
    }

    // Apply basic sysctl tweaks
    apply_basic_sysctl();

    // Clean pacman cache
    clean_pacman_cache();

    // Remove orphaned packages
    remove_orphaned_packages();

    // Enable zram if available
    enable_zram();

    // Optimize mirrors
    optimize_mirrors();

    println!("✅ Quick optimization completed!");
}

fn apply_basic_sysctl() {
    println!("🔧 Applying basic sysctl optimizations...");

    let sysctl_conf = "/etc/sysctl.d/99-ghostctl-perf.conf";
    let tweaks = r#"# GhostCTL Performance Optimizations
# Virtual memory settings
vm.swappiness=10
vm.vfs_cache_pressure=50
vm.dirty_ratio=15
vm.dirty_background_ratio=5

# Network optimizations
net.core.rmem_max = 16777216
net.core.wmem_max = 16777216
net.ipv4.tcp_rmem = 4096 87380 16777216
net.ipv4.tcp_wmem = 4096 65536 16777216
net.ipv4.tcp_congestion_control = bbr

# File system optimizations
fs.file-max = 2097152
kernel.pid_max = 4194304
"#;

    if let Ok(mut file) = std::fs::File::create(sysctl_conf) {
        use std::io::Write;
        if file.write_all(tweaks.as_bytes()).is_ok() {
            let _ = Command::new("sudo").args(&["sysctl", "--system"]).status();
            println!("  ✅ Sysctl optimizations applied");
        } else {
            println!("  ❌ Failed to write sysctl tweaks");
        }
    } else {
        println!("  ❌ Failed to create sysctl configuration");
    }
}

fn clean_pacman_cache() {
    println!("🧹 Cleaning pacman cache...");

    let status = Command::new("sudo").args(&["paccache", "-r"]).status();

    match status {
        Ok(s) if s.success() => println!("  ✅ Pacman cache cleaned"),
        _ => println!("  ⚠️  paccache not available or failed"),
    }
}

fn remove_orphaned_packages() {
    println!("📦 Removing orphaned packages...");

    let orphans_check = Command::new("pacman").args(&["-Qtdq"]).output();

    match orphans_check {
        Ok(output) if !output.stdout.is_empty() => {
            let status = Command::new("sudo")
                .args(&["pacman", "-Rns", "--noconfirm"])
                .arg(String::from_utf8_lossy(&output.stdout).trim())
                .status();

            match status {
                Ok(s) if s.success() => println!("  ✅ Orphaned packages removed"),
                _ => println!("  ❌ Failed to remove orphaned packages"),
            }
        }
        _ => println!("  ℹ️  No orphaned packages found"),
    }
}

fn enable_zram() {
    println!("💾 Configuring zram...");

    // Check if zram module is available
    let zram_check = Command::new("modinfo").arg("zram").status();

    if zram_check.is_ok() && zram_check.unwrap().success() {
        let status = Command::new("sudo")
            .args(&[
                "systemctl",
                "enable",
                "--now",
                "systemd-zram-setup@zram0.service",
            ])
            .status();

        match status {
            Ok(s) if s.success() => println!("  ✅ zram enabled"),
            _ => println!("  ⚠️  Failed to enable zram"),
        }
    } else {
        println!("  ℹ️  zram module not available");
    }
}

fn optimize_mirrors() {
    println!("🌐 Optimizing package mirrors...");

    let reflector_check = Command::new("which").arg("reflector").status();

    if reflector_check.is_ok() && reflector_check.unwrap().success() {
        let status = Command::new("sudo")
            .args(&[
                "reflector",
                "--latest",
                "20",
                "--sort",
                "rate",
                "--save",
                "/etc/pacman.d/mirrorlist",
            ])
            .status();

        match status {
            Ok(s) if s.success() => println!("  ✅ Mirrors optimized"),
            _ => println!("  ❌ Failed to optimize mirrors"),
        }
    } else {
        println!("  ℹ️  Installing reflector for mirror optimization...");
        let _ = Command::new("sudo")
            .args(&["pacman", "-S", "--noconfirm", "reflector"])
            .status();
    }
}

fn advanced_tuning() {
    println!("🔧 Advanced System Tuning");
    println!("=========================");

    let tuning_options = [
        "🎯 CPU Governor & Scaling",
        "💾 I/O Scheduler Optimization",
        "🌡️  Thermal Management",
        "⚡ CPU Frequency Scaling",
        "🧠 NUMA Optimization",
        "🔧 Kernel Scheduler Tuning",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Advanced Tuning Options")
        .items(&tuning_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => cpu_governor_tuning(),
        1 => io_scheduler_optimization(),
        2 => thermal_management(),
        3 => cpu_frequency_scaling(),
        4 => numa_optimization(),
        5 => scheduler_tuning(),
        _ => return,
    }
}

fn cpu_governor_tuning() {
    println!("🎯 CPU Governor & Scaling Configuration");
    println!("=======================================");

    // Show current governor
    if let Ok(output) = Command::new("cat")
        .arg("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
        .output()
    {
        println!(
            "📊 Current governor: {}",
            String::from_utf8_lossy(&output.stdout).trim()
        );
    }

    // Show available governors
    if let Ok(output) = Command::new("cat")
        .arg("/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors")
        .output()
    {
        println!(
            "📋 Available governors: {}",
            String::from_utf8_lossy(&output.stdout).trim()
        );
    }

    let governors = [
        "performance",
        "powersave",
        "ondemand",
        "conservative",
        "schedutil",
    ];
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select CPU governor")
        .items(&governors)
        .default(0)
        .interact()
        .unwrap();

    let selected_governor = governors[choice];

    // Apply governor to all CPUs
    let status = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(&format!(
            "echo {} | tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor",
            selected_governor
        ))
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ CPU governor set to {}", selected_governor),
        _ => println!("❌ Failed to set CPU governor"),
    }
}

fn io_scheduler_optimization() {
    println!("💾 I/O Scheduler Optimization");
    println!("=============================");

    println!("📊 Current I/O schedulers:");
    let _ = Command::new("sh")
        .arg("-c")
        .arg("for dev in /sys/block/*/queue/scheduler; do echo \"$(basename $(dirname $(dirname $dev))): $(cat $dev)\"; done")
        .status();

    let schedulers = ["mq-deadline", "kyber", "bfq", "none"];
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select I/O scheduler for all drives")
        .items(&schedulers)
        .default(0)
        .interact()
        .unwrap();

    let selected_scheduler = schedulers[choice];

    let status = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(&format!(
            "for dev in /sys/block/*/queue/scheduler; do echo {} > $dev; done",
            selected_scheduler
        ))
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ I/O scheduler set to {}", selected_scheduler),
        _ => println!("❌ Failed to set I/O scheduler"),
    }
}

fn thermal_management() {
    println!("🌡️  Thermal Management Configuration");
    println!("====================================");

    // Check current thermal zones
    println!("📊 Current thermal status:");
    let _ = Command::new("cat")
        .arg("/proc/acpi/thermal_zone/*/temperature")
        .status();

    // Install thermal management tools
    let confirm = Confirm::new()
        .with_prompt("Install thermal management tools (thermald, auto-cpufreq)?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        let tools = ["thermald", "auto-cpufreq"];
        for tool in &tools {
            let status = Command::new("sudo")
                .args(&["pacman", "-S", "--needed", "--noconfirm", tool])
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("  ✅ {} installed", tool);
                    let _ = Command::new("sudo")
                        .args(&["systemctl", "enable", "--now", tool])
                        .status();
                }
                _ => println!("  ❌ Failed to install {}", tool),
            }
        }
    }
}

fn cpu_frequency_scaling() {
    println!("⚡ CPU Frequency Scaling Configuration");
    println!("=====================================");

    // Show CPU frequency info
    println!("📊 CPU frequency information:");
    let _ = Command::new("cpupower").arg("frequency-info").status();

    // Install cpupower if not available
    let cpupower_check = Command::new("which").arg("cpupower").status();
    if cpupower_check.is_err() || !cpupower_check.unwrap().success() {
        println!("📦 Installing cpupower...");
        let _ = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm", "cpupower"])
            .status();
    }
}

fn numa_optimization() {
    println!("🧠 NUMA Optimization");
    println!("====================");

    // Check if NUMA is available
    let numa_check = Command::new("numactl").arg("--hardware").status();

    if numa_check.is_ok() && numa_check.unwrap().success() {
        println!("📊 NUMA topology detected");
        let _ = Command::new("numactl").arg("--hardware").status();
    } else {
        println!("ℹ️  NUMA not available or numactl not installed");
        let confirm = Confirm::new()
            .with_prompt("Install numactl for NUMA management?")
            .default(false)
            .interact()
            .unwrap();

        if confirm {
            let _ = Command::new("sudo")
                .args(&["pacman", "-S", "--needed", "--noconfirm", "numactl"])
                .status();
        }
    }
}

fn scheduler_tuning() {
    println!("🔧 Kernel Scheduler Tuning");
    println!("==========================");

    let scheduler_tweaks = r#"# Kernel Scheduler Optimizations
kernel.sched_migration_cost_ns = 5000000
kernel.sched_autogroup_enabled = 1
kernel.sched_tunable_scaling = 0
"#;

    let confirm = Confirm::new()
        .with_prompt("Apply kernel scheduler optimizations?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        let sysctl_file = "/etc/sysctl.d/99-ghostctl-scheduler.conf";
        if let Ok(mut file) = std::fs::File::create(sysctl_file) {
            use std::io::Write;
            if file.write_all(scheduler_tweaks.as_bytes()).is_ok() {
                let _ = Command::new("sudo").args(&["sysctl", "--system"]).status();
                println!("✅ Scheduler optimizations applied");
            }
        }
    }
}

fn performance_analysis() {
    println!("📊 System Performance Analysis");
    println!("==============================");

    let analysis_options = [
        "⏱️  Boot Time Analysis",
        "💾 Memory Usage Analysis",
        "🔄 CPU Performance Check",
        "💿 Disk I/O Analysis",
        "🌐 Network Performance",
        "📈 System Resource Monitor",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Performance Analysis")
        .items(&analysis_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => boot_time_analysis(),
        1 => memory_analysis(),
        2 => cpu_performance_check(),
        3 => disk_io_analysis(),
        4 => network_performance(),
        5 => system_resource_monitor(),
        _ => return,
    }
}

fn boot_time_analysis() {
    println!("⏱️  Boot Time Analysis");
    println!("======================");

    println!("📊 Boot performance:");
    let _ = Command::new("systemd-analyze").status();

    println!("\n🔍 Critical chain:");
    let _ = Command::new("systemd-analyze")
        .arg("critical-chain")
        .status();

    println!("\n⏰ Service times:");
    let _ = Command::new("systemd-analyze").arg("blame").status();
}

fn memory_analysis() {
    println!("💾 Memory Usage Analysis");
    println!("=======================");

    println!("📊 Memory overview:");
    let _ = Command::new("free").args(&["-h", "-w"]).status();

    println!("\n📈 Memory details:");
    let _ = Command::new("cat").arg("/proc/meminfo").status();

    println!("\n🔍 Top memory consumers:");
    let _ = Command::new("ps").args(&["aux", "--sort=-%mem"]).status();
}

fn cpu_performance_check() {
    println!("🔄 CPU Performance Check");
    println!("========================");

    println!("📊 CPU information:");
    let _ = Command::new("lscpu").status();

    println!("\n⚡ CPU frequency:");
    let _ = Command::new("cat").arg("/proc/cpuinfo").status();

    println!("\n📈 Load averages:");
    let _ = Command::new("cat").arg("/proc/loadavg").status();
}

fn disk_io_analysis() {
    println!("💿 Disk I/O Analysis");
    println!("====================");

    println!("📊 Disk usage:");
    let _ = Command::new("df").args(&["-h"]).status();

    println!("\n⚡ I/O statistics:");
    let iostat_check = Command::new("which").arg("iostat").status();
    if iostat_check.is_ok() && iostat_check.unwrap().success() {
        let _ = Command::new("iostat").args(&["-x", "1", "3"]).status();
    } else {
        println!("💡 Install sysstat for detailed I/O analysis");
    }
}

fn network_performance() {
    println!("🌐 Network Performance Analysis");
    println!("==============================");

    println!("📊 Network interfaces:");
    let _ = Command::new("ip").args(&["addr", "show"]).status();

    println!("\n📈 Network statistics:");
    let _ = Command::new("cat").arg("/proc/net/dev").status();
}

fn system_resource_monitor() {
    println!("📈 System Resource Monitor");
    println!("==========================");

    println!("💡 Starting htop for real-time monitoring...");
    let _ = Command::new("htop").status();
}

fn kernel_parameters() {
    println!("⚙️  Kernel Parameters Configuration");
    println!("===================================");

    let param_options = [
        "📋 View Current Parameters",
        "⚡ Gaming Optimizations",
        "🛡️  Security Hardening",
        "💾 Memory Management",
        "🌐 Network Optimizations",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Kernel Parameters")
        .items(&param_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => view_current_parameters(),
        1 => gaming_kernel_optimizations(),
        2 => security_hardening_parameters(),
        3 => memory_management_parameters(),
        4 => network_kernel_optimizations(),
        _ => return,
    }
}

fn view_current_parameters() {
    println!("📋 Current Kernel Parameters");
    println!("============================");

    println!("⚙️  sysctl parameters:");
    let _ = Command::new("sysctl").arg("-a").status();
}

fn gaming_kernel_optimizations() {
    println!("⚡ Gaming Kernel Optimizations");
    println!("==============================");

    let gaming_params = r#"# Gaming Performance Optimizations
vm.swappiness=1
vm.vfs_cache_pressure=50
vm.dirty_ratio=15
vm.dirty_background_ratio=5
kernel.sched_min_granularity_ns = 10000000
kernel.sched_wakeup_granularity_ns = 15000000
net.core.netdev_max_backlog = 5000
"#;

    let confirm = Confirm::new()
        .with_prompt("Apply gaming kernel optimizations?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        apply_kernel_parameters(gaming_params, "gaming");
    }
}

fn security_hardening_parameters() {
    println!("🛡️  Security Hardening Parameters");
    println!("==================================");

    let security_params = r#"# Security Hardening
kernel.dmesg_restrict = 1
kernel.kptr_restrict = 2
kernel.yama.ptrace_scope = 1
net.ipv4.conf.all.rp_filter = 1
net.ipv4.conf.default.rp_filter = 1
net.ipv4.conf.all.accept_redirects = 0
net.ipv4.conf.default.accept_redirects = 0
net.ipv6.conf.all.accept_redirects = 0
net.ipv6.conf.default.accept_redirects = 0
"#;

    let confirm = Confirm::new()
        .with_prompt("Apply security hardening parameters?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        apply_kernel_parameters(security_params, "security");
    }
}

fn memory_management_parameters() {
    println!("💾 Memory Management Parameters");
    println!("===============================");

    let memory_params = r#"# Memory Management Optimizations
vm.swappiness=10
vm.vfs_cache_pressure=50
vm.dirty_ratio=15
vm.dirty_background_ratio=5
vm.overcommit_memory=1
vm.overcommit_ratio=50
"#;

    let confirm = Confirm::new()
        .with_prompt("Apply memory management optimizations?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        apply_kernel_parameters(memory_params, "memory");
    }
}

fn network_kernel_optimizations() {
    println!("🌐 Network Kernel Optimizations");
    println!("===============================");

    let network_params = r#"# Network Performance Optimizations
net.core.rmem_max = 16777216
net.core.wmem_max = 16777216
net.ipv4.tcp_rmem = 4096 87380 16777216
net.ipv4.tcp_wmem = 4096 65536 16777216
net.ipv4.tcp_congestion_control = bbr
net.core.netdev_max_backlog = 5000
net.ipv4.tcp_window_scaling = 1
"#;

    let confirm = Confirm::new()
        .with_prompt("Apply network optimizations?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        apply_kernel_parameters(network_params, "network");
    }
}

fn apply_kernel_parameters(params: &str, category: &str) {
    let config_file = format!("/etc/sysctl.d/99-ghostctl-{}.conf", category);

    if let Ok(mut file) = std::fs::File::create(&config_file) {
        use std::io::Write;
        if file.write_all(params.as_bytes()).is_ok() {
            let _ = Command::new("sudo").args(&["sysctl", "--system"]).status();
            println!("✅ {} parameters applied", category);
        } else {
            println!("❌ Failed to write {} parameters", category);
        }
    } else {
        println!("❌ Failed to create configuration file");
    }
}

fn memory_swap_optimization() {
    println!("💾 Memory & Swap Optimization");
    println!("=============================");

    let memory_options = [
        "🔧 Configure Swappiness",
        "💾 Zram Configuration",
        "🔄 Zswap Configuration",
        "📊 Memory Analysis",
        "🧹 Memory Cleanup",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Memory & Swap Options")
        .items(&memory_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => configure_swappiness(),
        1 => configure_zram(),
        2 => configure_zswap(),
        3 => memory_analysis(),
        4 => memory_cleanup(),
        _ => return,
    }
}

fn configure_swappiness() {
    println!("🔧 Configure VM Swappiness");
    println!("===========================");

    // Show current swappiness
    if let Ok(output) = Command::new("cat").arg("/proc/sys/vm/swappiness").output() {
        println!(
            "📊 Current swappiness: {}",
            String::from_utf8_lossy(&output.stdout).trim()
        );
    }

    let swappiness_options = [
        "1 (Minimal swapping)",
        "10 (Gaming optimized)",
        "60 (Default)",
        "100 (Aggressive swapping)",
    ];
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select swappiness value")
        .items(&swappiness_options)
        .default(1)
        .interact()
        .unwrap();

    let swappiness_values = [1, 10, 60, 100];
    let selected_value = swappiness_values[choice];

    let status = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(&format!("echo 'vm.swappiness={}' > /etc/sysctl.d/99-ghostctl-swappiness.conf && sysctl vm.swappiness={}", selected_value, selected_value))
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Swappiness set to {}", selected_value),
        _ => println!("❌ Failed to set swappiness"),
    }
}

fn configure_zram() {
    println!("💾 Zram Configuration");
    println!("=====================");

    // Check if zram is available
    let zram_check = Command::new("modinfo").arg("zram").status();

    if zram_check.is_ok() && zram_check.unwrap().success() {
        let zram_options = [
            "🔧 Enable zram",
            "📊 Zram Status",
            "⚙️  Configure zram size",
            "🛑 Disable zram",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Zram Options")
            .items(&zram_options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => {
                let _ = Command::new("sudo")
                    .args(&[
                        "systemctl",
                        "enable",
                        "--now",
                        "systemd-zram-setup@zram0.service",
                    ])
                    .status();
                println!("✅ Zram enabled");
            }
            1 => {
                let _ = Command::new("cat").arg("/proc/swaps").status();
            }
            2 => {
                println!(
                    "💡 Zram size is typically configured via /etc/systemd/zram-generator.conf"
                );
            }
            3 => {
                let _ = Command::new("sudo")
                    .args(&[
                        "systemctl",
                        "disable",
                        "--now",
                        "systemd-zram-setup@zram0.service",
                    ])
                    .status();
                println!("🛑 Zram disabled");
            }
            _ => {}
        }
    } else {
        println!("❌ Zram module not available in this kernel");
    }
}

fn configure_zswap() {
    println!("🔄 Zswap Configuration");
    println!("======================");

    // Check if zswap is available
    let zswap_check = std::path::Path::new("/sys/module/zswap").exists();

    if zswap_check {
        println!("📊 Current zswap status:");
        let _ = Command::new("cat")
            .arg("/sys/module/zswap/parameters/enabled")
            .status();

        let enable = Confirm::new()
            .with_prompt("Enable zswap?")
            .default(true)
            .interact()
            .unwrap();

        if enable {
            let _ = Command::new("sudo")
                .arg("sh")
                .arg("-c")
                .arg("echo 1 > /sys/module/zswap/parameters/enabled")
                .status();
            println!("✅ Zswap enabled");
        }
    } else {
        println!("❌ Zswap not available in this kernel");
    }
}

fn memory_cleanup() {
    println!("🧹 Memory Cleanup");
    println!("=================");

    let cleanup_options = [
        "🗑️  Drop caches",
        "🔄 Compact memory",
        "📦 Clean package cache",
        "🧹 Clear logs",
    ];

    for (i, option) in cleanup_options.iter().enumerate() {
        let confirm = Confirm::new()
            .with_prompt(format!("Execute: {}", option))
            .default(false)
            .interact()
            .unwrap();

        if confirm {
            match i {
                0 => {
                    let _ = Command::new("sudo")
                        .arg("sh")
                        .arg("-c")
                        .arg("echo 3 > /proc/sys/vm/drop_caches")
                        .status();
                    println!("  ✅ Caches dropped");
                }
                1 => {
                    let _ = Command::new("sudo")
                        .arg("sh")
                        .arg("-c")
                        .arg("echo 1 > /proc/sys/vm/compact_memory")
                        .status();
                    println!("  ✅ Memory compacted");
                }
                2 => {
                    let _ = Command::new("sudo").args(&["paccache", "-r"]).status();
                    println!("  ✅ Package cache cleaned");
                }
                3 => {
                    let _ = Command::new("sudo")
                        .args(&["journalctl", "--vacuum-time=7d"])
                        .status();
                    println!("  ✅ Logs cleaned");
                }
                _ => {}
            }
        }
    }
}

fn boot_optimization() {
    println!("🚀 Boot Time Optimization");
    println!("=========================");

    let boot_options = [
        "⏱️  Analyze boot time",
        "🔧 Disable slow services",
        "⚡ Enable parallel boot",
        "🎯 Optimize initramfs",
        "📊 Service analysis",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Boot Optimization")
        .items(&boot_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => boot_time_analysis(),
        1 => disable_slow_services(),
        2 => enable_parallel_boot(),
        3 => optimize_initramfs(),
        4 => service_analysis(),
        _ => return,
    }
}

fn disable_slow_services() {
    println!("🔧 Disable Slow Services");
    println!("========================");

    let slow_services = [
        "NetworkManager-wait-online.service",
        "systemd-networkd-wait-online.service",
        "plymouth-quit-wait.service",
    ];

    for service in &slow_services {
        let status_check = Command::new("systemctl")
            .args(&["is-enabled", service])
            .output();

        if let Ok(output) = status_check {
            let status_text = String::from_utf8_lossy(&output.stdout);
            let status = status_text.trim();
            if status == "enabled" {
                let confirm = Confirm::new()
                    .with_prompt(format!("Disable {}?", service))
                    .default(false)
                    .interact()
                    .unwrap();

                if confirm {
                    let _ = Command::new("sudo")
                        .args(&["systemctl", "disable", service])
                        .status();
                    println!("  ✅ {} disabled", service);
                }
            }
        }
    }
}

fn enable_parallel_boot() {
    println!("⚡ Enable Parallel Boot");
    println!("=======================");

    let confirm = Confirm::new()
        .with_prompt("Enable systemd parallel boot optimization?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        // This is mostly enabled by default in modern systemd
        println!("💡 Parallel boot is typically enabled by default in modern systemd");
        println!("🔧 You can verify with: systemd-analyze critical-chain");
    }
}

fn optimize_initramfs() {
    println!("🎯 Optimize Initramfs");
    println!("=====================");

    println!("💡 Current mkinitcpio configuration:");
    let _ = Command::new("cat").arg("/etc/mkinitcpio.conf").status();

    let confirm = Confirm::new()
        .with_prompt("Regenerate initramfs with optimizations?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let _ = Command::new("sudo")
            .args(&["mkinitcpio", "-p", "linux"])
            .status();
        println!("✅ Initramfs regenerated");
    }
}

fn service_analysis() {
    println!("📊 Service Analysis");
    println!("===================");

    println!("⏰ Service boot times:");
    let _ = Command::new("systemd-analyze").arg("blame").status();

    println!("\n🔗 Critical chain:");
    let _ = Command::new("systemd-analyze")
        .arg("critical-chain")
        .status();

    println!("\n🐌 Slowest services:");
    let _ = Command::new("systemd-analyze")
        .args(&["blame", "|", "head", "-10"])
        .status();
}

fn power_management() {
    println!("🔋 Power Management Configuration");
    println!("=================================");

    let power_options = [
        "⚡ CPU Power Management",
        "🖥️  Display Power Settings",
        "💾 Storage Power Management",
        "🌐 Network Power Management",
        "📊 Power Status",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Power Management")
        .items(&power_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => cpu_power_management(),
        1 => display_power_settings(),
        2 => storage_power_management(),
        3 => network_power_management(),
        4 => power_status(),
        _ => return,
    }
}

fn cpu_power_management() {
    println!("⚡ CPU Power Management");
    println!("======================");

    // Install power management tools
    let tools = ["tlp", "powertop", "auto-cpufreq"];

    for tool in &tools {
        let check = Command::new("which").arg(tool).status();
        if check.is_err() || !check.unwrap().success() {
            let confirm = Confirm::new()
                .with_prompt(format!("Install {}?", tool))
                .default(false)
                .interact()
                .unwrap();

            if confirm {
                let _ = Command::new("sudo")
                    .args(&["pacman", "-S", "--needed", "--noconfirm", tool])
                    .status();

                if *tool == "tlp" || *tool == "auto-cpufreq" {
                    let _ = Command::new("sudo")
                        .args(&["systemctl", "enable", "--now", tool])
                        .status();
                }
            }
        }
    }
}

fn display_power_settings() {
    println!("🖥️  Display Power Settings");
    println!("===========================");

    println!("💡 Configure display power management in your desktop environment");
    println!("🔧 GNOME: Settings > Power");
    println!("🔧 KDE: System Settings > Power Management");
    println!("🔧 Command line: xset dpms [standby] [suspend] [off]");
}

fn storage_power_management() {
    println!("💾 Storage Power Management");
    println!("===========================");

    println!("📊 Current storage power settings:");
    let _ = Command::new("cat")
        .arg("/sys/class/scsi_host/host*/link_power_management_policy")
        .status();

    let confirm = Confirm::new()
        .with_prompt("Enable aggressive storage power management?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let _ = Command::new("sudo")
            .arg("sh")
            .arg("-c")
            .arg("echo 'min_power' | tee /sys/class/scsi_host/host*/link_power_management_policy")
            .status();
        println!("✅ Aggressive storage power management enabled");
    }
}

fn network_power_management() {
    println!("🌐 Network Power Management");
    println!("===========================");

    println!("📊 Network interface power settings:");
    let _ = Command::new("ethtool")
        .args(&["--show-features", "eth0"])
        .status();
}

fn power_status() {
    println!("📊 Power Status");
    println!("===============");

    println!("🔋 Battery information:");
    let _ = Command::new("upower")
        .args(&["-i", "/org/freedesktop/UPower/devices/BAT0"])
        .status();

    println!("\n⚡ Power consumption:");
    let powertop_check = Command::new("which").arg("powertop").status();
    if powertop_check.is_ok() && powertop_check.unwrap().success() {
        let _ = Command::new("sudo")
            .args(&["powertop", "--time=10"])
            .status();
    }
}

fn monitoring_setup() {
    println!("📈 System Monitoring Setup");
    println!("==========================");

    let monitoring_tools = [
        ("htop", "Enhanced process viewer"),
        ("iotop", "I/O monitoring"),
        ("nethogs", "Network monitoring per process"),
        ("glances", "Comprehensive system monitor"),
        ("sysstat", "System statistics collection"),
    ];

    for (tool, description) in &monitoring_tools {
        let check = Command::new("which").arg(tool).status();
        if check.is_err() || !check.unwrap().success() {
            let confirm = Confirm::new()
                .with_prompt(format!("Install {} - {}?", tool, description))
                .default(false)
                .interact()
                .unwrap();

            if confirm {
                let _ = Command::new("sudo")
                    .args(&["pacman", "-S", "--needed", "--noconfirm", tool])
                    .status();
                println!("  ✅ {} installed", tool);
            }
        } else {
            println!("  ✅ {} already installed", tool);
        }
    }
}

fn custom_profiles() {
    println!("🎯 Custom Performance Profiles");
    println!("==============================");

    let profile_options = [
        "🎮 Gaming Profile",
        "⚡ High Performance Profile",
        "🔋 Power Saving Profile",
        "🖥️  Workstation Profile",
        "📱 Default Profile",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Performance Profiles")
        .items(&profile_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => apply_gaming_profile(),
        1 => apply_performance_profile(),
        2 => apply_power_saving_profile(),
        3 => apply_workstation_profile(),
        4 => apply_default_profile(),
        _ => return,
    }
}

fn apply_gaming_profile() {
    println!("🎮 Applying Gaming Performance Profile");
    println!("======================================");

    let confirm = Confirm::new()
        .with_prompt("Apply gaming optimizations? (CPU governor, swappiness, I/O scheduler)")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        // Set performance governor
        let _ = Command::new("sudo")
            .arg("sh")
            .arg("-c")
            .arg("echo performance | tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor")
            .status();

        // Set low swappiness
        let _ = Command::new("sudo")
            .arg("sh")
            .arg("-c")
            .arg("echo 1 > /proc/sys/vm/swappiness")
            .status();

        // Set deadline scheduler for SSDs
        let _ = Command::new("sudo")
            .arg("sh")
            .arg("-c")
            .arg("echo mq-deadline | tee /sys/block/*/queue/scheduler")
            .status();

        println!("✅ Gaming profile applied");
    }
}

fn apply_performance_profile() {
    println!("⚡ Applying High Performance Profile");
    println!("====================================");

    let performance_params = r#"# High Performance Profile
vm.swappiness=10
vm.dirty_ratio=15
vm.dirty_background_ratio=5
kernel.sched_migration_cost_ns = 5000000
net.core.netdev_max_backlog = 5000
"#;

    apply_kernel_parameters(performance_params, "performance");

    // Set performance governor
    let _ = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg("echo performance | tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor")
        .status();

    println!("✅ High performance profile applied");
}

fn apply_power_saving_profile() {
    println!("🔋 Applying Power Saving Profile");
    println!("=================================");

    // Set powersave governor
    let _ = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg("echo powersave | tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor")
        .status();

    // Enable power saving features
    let _ = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg("echo 'min_power' | tee /sys/class/scsi_host/host*/link_power_management_policy")
        .status();

    println!("✅ Power saving profile applied");
}

fn apply_workstation_profile() {
    println!("🖥️  Applying Workstation Profile");
    println!("=================================");

    let workstation_params = r#"# Workstation Profile
vm.swappiness=30
vm.dirty_ratio=20
vm.dirty_background_ratio=10
kernel.sched_autogroup_enabled = 1
"#;

    apply_kernel_parameters(workstation_params, "workstation");

    // Set ondemand governor
    let _ = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg("echo ondemand | tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor")
        .status();

    println!("✅ Workstation profile applied");
}

fn apply_default_profile() {
    println!("📱 Restoring Default Profile");
    println!("============================");

    // Remove custom configurations
    let configs = [
        "/etc/sysctl.d/99-ghostctl-gaming.conf",
        "/etc/sysctl.d/99-ghostctl-performance.conf",
        "/etc/sysctl.d/99-ghostctl-workstation.conf",
    ];

    for config in &configs {
        let _ = std::fs::remove_file(config);
    }

    // Reset to default governor
    let _ = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg("echo schedutil | tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor")
        .status();

    // Reload sysctl
    let _ = Command::new("sudo").args(&["sysctl", "--system"]).status();

    println!("✅ Default profile restored");
}
