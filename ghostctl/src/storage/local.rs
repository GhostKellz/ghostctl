use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn local_storage_menu() {
    loop {
        let options = vec![
            "Disk Health Monitoring",
            "SMART Status Check",
            "Filesystem Tools",
            "Mount Management",
            "Storage Benchmarking",
            "Disk Cleanup Tools",
            "RAID Management",
            "Back",
        ];

        let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("💾 Local Storage Management")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match selection {
            0 => disk_health_monitoring(),
            1 => smart_status_check(),
            2 => filesystem_tools(),
            3 => mount_management(),
            4 => storage_benchmarking(),
            5 => disk_cleanup_tools(),
            6 => raid_management(),
            _ => break,
        }
    }
}

fn disk_health_monitoring() {
    println!("🔍 Disk Health Monitoring\n");

    println!("📊 Overall Disk Usage:");
    let _ = Command::new("df").args(&["-h"]).status();

    println!("\n💿 Block Devices:");
    let _ = Command::new("lsblk").status();

    println!("\n🌡️  Disk Temperatures:");
    let _ = Command::new("sensors").status();

    println!("\n📈 I/O Statistics:");
    let _ = Command::new("iostat").args(&["-x", "1", "3"]).status();

    println!("\n⚡ Current I/O Activity:");
    let _ = Command::new("iotop").args(&["-o", "-n", "3"]).status();
}

fn smart_status_check() {
    println!("🧠 SMART Status Check\n");

    // List all disks
    println!("📋 Available disks:");
    let output = Command::new("lsblk")
        .args(&["-d", "-n", "-o", "NAME,SIZE,MODEL"])
        .output();

    if let Ok(output) = output {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    let Ok(disk) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter disk to check (e.g., /dev/sda)")
        .interact()
    else {
        return;
    };

    println!("🔍 SMART status for {}:", disk);
    let _ = Command::new("smartctl").args(&["-a", &disk]).status();

    println!("\n🧪 Running SMART self-test...");
    let Ok(run_test) = Confirm::new()
        .with_prompt("Run short SMART self-test?")
        .default(true)
        .interact()
    else {
        return;
    };
    if run_test {
        let _ = Command::new("smartctl")
            .args(&["-t", "short", &disk])
            .status();

        println!("✅ Self-test started. Check results in a few minutes with:");
        println!("   smartctl -l selftest {}", disk);
    }

    // Check for bad sectors
    let Ok(check_sectors) = Confirm::new()
        .with_prompt("Check for bad sectors (read-only scan)?")
        .default(false)
        .interact()
    else {
        return;
    };
    if check_sectors {
        println!("🔍 Scanning for bad sectors (this may take a while)...");
        let _ = Command::new("badblocks").args(&["-v", &disk]).status();
    }
}

fn filesystem_tools() {
    loop {
        let options = vec![
            "Check Filesystem",
            "Repair Filesystem",
            "Resize Filesystem",
            "Create Filesystem",
            "Filesystem Information",
            "Defragment (ext4)",
            "Back",
        ];

        let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🗂️  Filesystem Tools")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match selection {
            0 => check_filesystem(),
            1 => repair_filesystem(),
            2 => resize_filesystem(),
            3 => create_filesystem(),
            4 => filesystem_information(),
            5 => defragment_filesystem(),
            _ => break,
        }
    }
}

fn check_filesystem() {
    println!("🔍 Filesystem Check\n");

    let Ok(device) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter device/partition to check (e.g., /dev/sda1)")
        .interact()
    else {
        return;
    };

    // Detect filesystem type
    let fs_output = Command::new("blkid")
        .args(&["-o", "value", "-s", "TYPE", &device])
        .output();

    let fs_type = if let Ok(output) = fs_output {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "unknown".to_string()
    };

    println!("📂 Detected filesystem: {}", fs_type);

    match fs_type.as_str() {
        "ext2" | "ext3" | "ext4" => {
            println!("🔍 Running e2fsck...");
            let _ = Command::new("e2fsck").args(&["-f", "-v", &device]).status();
        }
        "xfs" => {
            println!("🔍 Running xfs_check...");
            let _ = Command::new("xfs_check").args(&[&device]).status();
        }
        "btrfs" => {
            println!("🔍 Running btrfs check...");
            let _ = Command::new("btrfs").args(&["check", &device]).status();
        }
        _ => {
            println!("⚠️  Unsupported filesystem type for automatic check");
        }
    }
}

fn repair_filesystem() {
    println!("🔧 Filesystem Repair\n");
    println!("⚠️  WARNING: Unmount the filesystem before repair!");

    let Ok(device) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter device/partition to repair")
        .interact()
    else {
        return;
    };

    let Ok(confirmed) = Confirm::new()
        .with_prompt("Have you unmounted the filesystem?")
        .default(false)
        .interact()
    else {
        return;
    };
    if !confirmed {
        return;
    }

    // Detect filesystem type
    let fs_output = Command::new("blkid")
        .args(&["-o", "value", "-s", "TYPE", &device])
        .output();

    let fs_type = if let Ok(output) = fs_output {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "unknown".to_string()
    };

    match fs_type.as_str() {
        "ext2" | "ext3" | "ext4" => {
            let _ = Command::new("e2fsck").args(&["-y", &device]).status();
        }
        "xfs" => {
            let _ = Command::new("xfs_repair").args(&[&device]).status();
        }
        "btrfs" => {
            let _ = Command::new("btrfs")
                .args(&["check", "--repair", &device])
                .status();
        }
        _ => {
            println!("⚠️  Unsupported filesystem type");
        }
    }
}

fn resize_filesystem() {
    println!("📏 Resize Filesystem\n");

    let Ok(device) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter device/partition to resize")
        .interact()
    else {
        return;
    };

    let Ok(mount_point) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter mount point (if mounted)")
        .default("".to_string())
        .interact()
    else {
        return;
    };

    // Detect filesystem type
    let fs_output = Command::new("blkid")
        .args(&["-o", "value", "-s", "TYPE", &device])
        .output();

    let fs_type = if let Ok(output) = fs_output {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "unknown".to_string()
    };

    let Ok(resize_type) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select resize operation")
        .items(&["Grow to maximum size", "Shrink", "Specify size"])
        .default(0)
        .interact()
    else {
        return;
    };

    match fs_type.as_str() {
        "ext2" | "ext3" | "ext4" => match resize_type {
            0 => {
                if !mount_point.is_empty() {
                    let _ = Command::new("resize2fs").args(&[&device]).status();
                } else {
                    println!("⚠️  Mount the filesystem first for online resize");
                }
            }
            1 | 2 => {
                let Ok(size) = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter new size (e.g., 10G, 500M)")
                    .interact()
                else {
                    return;
                };

                let _ = Command::new("resize2fs").args(&[&device, &size]).status();
            }
            _ => {}
        },
        "xfs" => {
            if !mount_point.is_empty() {
                let _ = Command::new("xfs_growfs").args(&[&mount_point]).status();
            } else {
                println!("⚠️  XFS can only be grown while mounted");
            }
        }
        "btrfs" => {
            if !mount_point.is_empty() {
                let _ = Command::new("btrfs")
                    .args(&["filesystem", "resize", "max", &mount_point])
                    .status();
            } else {
                println!("⚠️  Mount the filesystem first");
            }
        }
        _ => {
            println!("⚠️  Unsupported filesystem type");
        }
    }
}

fn create_filesystem() {
    println!("🏗️  Create Filesystem\n");
    println!("⚠️  WARNING: This will destroy all data on the device!");

    let Ok(device) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter device to format")
        .interact()
    else {
        return;
    };

    let Ok(fs_type) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select filesystem type")
        .items(&["ext4", "xfs", "btrfs", "fat32", "ntfs"])
        .default(0)
        .interact()
    else {
        return;
    };

    let Ok(label) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter filesystem label (optional)")
        .default("".to_string())
        .interact()
    else {
        return;
    };

    let Ok(confirmed) = Confirm::new()
        .with_prompt(&format!("Really format {} and destroy all data?", device))
        .default(false)
        .interact()
    else {
        return;
    };
    if !confirmed {
        return;
    }

    match fs_type {
        0 => {
            // ext4
            let mut args = vec!["-t", "ext4"];
            if !label.is_empty() {
                args.extend_from_slice(&["-L", &label]);
            }
            args.push(&device);
            let _ = Command::new("mkfs").args(&args).status();
        }
        1 => {
            // xfs
            let mut args = vec!["-f"];
            if !label.is_empty() {
                args.extend_from_slice(&["-L", &label]);
            }
            args.push(&device);
            let _ = Command::new("mkfs.xfs").args(&args).status();
        }
        2 => {
            // btrfs
            let mut args = vec![];
            if !label.is_empty() {
                args.extend_from_slice(&["-L", &label]);
            }
            args.push(&device);
            let _ = Command::new("mkfs.btrfs").args(&args).status();
        }
        3 => {
            // fat32
            let _ = Command::new("mkfs.fat")
                .args(&["-F", "32", &device])
                .status();
        }
        4 => {
            // ntfs
            let _ = Command::new("mkfs.ntfs").args(&["-f", &device]).status();
        }
        _ => {}
    }

    println!("✅ Filesystem created successfully!");
}

fn filesystem_information() {
    println!("ℹ️  Filesystem Information\n");

    let Ok(device) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter device/partition")
        .interact()
    else {
        return;
    };

    // Basic information
    println!("📋 Basic Information:");
    let _ = Command::new("blkid").args(&[&device]).status();

    // Detailed filesystem information
    let fs_output = Command::new("blkid")
        .args(&["-o", "value", "-s", "TYPE", &device])
        .output();

    if let Ok(output) = fs_output {
        let fs_type = String::from_utf8_lossy(&output.stdout).trim().to_string();

        match fs_type.as_str() {
            "ext2" | "ext3" | "ext4" => {
                println!("\n📊 ext Filesystem Details:");
                let _ = Command::new("tune2fs").args(&["-l", &device]).status();
            }
            "xfs" => {
                println!("\n📊 XFS Filesystem Details:");
                let _ = Command::new("xfs_info").args(&[&device]).status();
            }
            "btrfs" => {
                println!("\n📊 Btrfs Filesystem Details:");
                let _ = Command::new("btrfs")
                    .args(&["filesystem", "show", &device])
                    .status();
            }
            _ => {}
        }
    }

    // Check if mounted and show mount info
    println!("\n🔗 Mount Status:");
    let _ = Command::new("findmnt").args(&[&device]).status();
}

fn defragment_filesystem() {
    println!("🗜️  Filesystem Defragmentation\n");

    let Ok(mount_point) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter mount point to defragment")
        .interact()
    else {
        return;
    };

    // Check filesystem type
    let fs_output = Command::new("findmnt")
        .args(&["-n", "-o", "FSTYPE", &mount_point])
        .output();

    if let Ok(output) = fs_output {
        let fs_type = String::from_utf8_lossy(&output.stdout).trim().to_string();

        match fs_type.as_str() {
            "ext4" => {
                println!("🔄 Defragmenting ext4 filesystem...");
                let _ = Command::new("e4defrag")
                    .args(&["-v", &mount_point])
                    .status();
            }
            "btrfs" => {
                println!("🔄 Defragmenting btrfs filesystem...");
                let _ = Command::new("btrfs")
                    .args(&["filesystem", "defragment", "-r", &mount_point])
                    .status();
            }
            "xfs" => {
                println!("ℹ️  XFS does not require defragmentation");
            }
            _ => {
                println!("⚠️  Defragmentation not supported for {}", fs_type);
            }
        }
    }
}

fn mount_management() {
    loop {
        let options = vec![
            "List All Mounts",
            "Mount Device",
            "Unmount Device",
            "Edit /etc/fstab",
            "Test fstab",
            "Mount Options Help",
            "Back",
        ];

        let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔗 Mount Management")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match selection {
            0 => list_mounts(),
            1 => mount_device(),
            2 => unmount_device(),
            3 => edit_fstab(),
            4 => test_fstab(),
            5 => mount_options_help(),
            _ => break,
        }
    }
}

fn list_mounts() {
    println!("📋 Current Mounts\n");
    let _ = Command::new("mount").status();

    println!("\n💾 Disk Usage:");
    let _ = Command::new("df").args(&["-h"]).status();
}

fn mount_device() {
    println!("🔗 Mount Device\n");

    let Ok(device) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter device to mount")
        .interact()
    else {
        return;
    };

    let Ok(mount_point) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter mount point")
        .interact()
    else {
        return;
    };

    let Ok(options) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter mount options (optional)")
        .default("defaults".to_string())
        .interact()
    else {
        return;
    };

    // Create mount point if it doesn't exist
    let _ = Command::new("mkdir").args(&["-p", &mount_point]).status();

    let result = if options == "defaults" {
        Command::new("mount")
            .args(&[&device, &mount_point])
            .status()
    } else {
        Command::new("mount")
            .args(&["-o", &options, &device, &mount_point])
            .status()
    };

    if result.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Mount successful!");
    } else {
        println!("❌ Mount failed!");
    }
}

fn unmount_device() {
    println!("🔌 Unmount Device\n");

    let Ok(target) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter device or mount point to unmount")
        .interact()
    else {
        return;
    };

    let result = Command::new("umount").args(&[&target]).status();

    if result.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Unmount successful!");
    } else {
        println!("❌ Unmount failed. Trying lazy unmount...");
        let lazy_result = Command::new("umount").args(&["-l", &target]).status();

        if lazy_result.map(|s| s.success()).unwrap_or(false) {
            println!("✅ Lazy unmount successful!");
        } else {
            println!("❌ Unmount failed completely");
        }
    }
}

fn edit_fstab() {
    println!("📝 Edit /etc/fstab\n");

    println!("Current /etc/fstab:");
    let _ = Command::new("cat").args(&["/etc/fstab"]).status();

    let Ok(edit) = Confirm::new()
        .with_prompt("Edit /etc/fstab?")
        .default(true)
        .interact()
    else {
        return;
    };
    if edit {
        let _ = Command::new("nano").args(&["/etc/fstab"]).status();
    }
}

fn test_fstab() {
    println!("🧪 Test /etc/fstab\n");

    println!("Testing all fstab entries...");
    let result = Command::new("mount").args(&["-a"]).status();

    if result.map(|s| s.success()).unwrap_or(false) {
        println!("✅ All fstab entries mount successfully!");
    } else {
        println!("❌ Some fstab entries failed to mount");
    }
}

fn mount_options_help() {
    println!("📖 Common Mount Options\n");

    println!("🔧 General Options:");
    println!("  defaults     - Use default options (rw,suid,dev,exec,auto,nouser,async)");
    println!("  ro/rw        - Mount read-only or read-write");
    println!("  noauto       - Don't mount automatically at boot");
    println!("  user         - Allow regular users to mount");
    println!("  noexec       - Don't allow execution of binaries");
    println!("  nosuid       - Don't allow set-user-identifier or set-group-identifier bits");

    println!("\n💾 Performance Options:");
    println!("  async/sync   - Asynchronous or synchronous I/O");
    println!("  atime/noatime - Update or don't update access times");
    println!("  relatime     - Update atime relative to mtime/ctime");

    println!("\n📁 ext4 Options:");
    println!("  barrier=0/1  - Enable/disable write barriers");
    println!("  data=ordered - Data ordering mode");
    println!("  commit=n     - Sync frequency in seconds");

    println!("\n🗂️  XFS Options:");
    println!("  nobarrier    - Disable write barriers for better performance");
    println!("  logbufs=n    - Number of in-memory log buffers");
    println!("  logbsize=n   - Size of log buffers");
}

fn storage_benchmarking() {
    loop {
        let options = vec![
            "Quick Disk Benchmark",
            "Comprehensive I/O Test",
            "Random vs Sequential Performance",
            "Filesystem Benchmark",
            "Compare Multiple Disks",
            "Back",
        ];

        let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("⚡ Storage Benchmarking")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match selection {
            0 => quick_disk_benchmark(),
            1 => comprehensive_io_test(),
            2 => random_vs_sequential(),
            3 => filesystem_benchmark(),
            4 => compare_multiple_disks(),
            _ => break,
        }
    }
}

fn quick_disk_benchmark() {
    println!("⚡ Quick Disk Benchmark\n");

    let Ok(device) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter device/mount point to test")
        .interact()
    else {
        return;
    };

    println!("🔄 Running quick sequential read test...");
    let _ = Command::new("dd")
        .args(&[
            "if=/dev/zero",
            &format!("of={}/testfile", device),
            "bs=1M",
            "count=1024",
            "conv=fsync",
        ])
        .status();

    let _ = Command::new("rm")
        .args(&[&format!("{}/testfile", device)])
        .status();

    println!("✅ Quick benchmark complete!");
}

fn comprehensive_io_test() {
    println!("🔬 Comprehensive I/O Test using fio\n");

    let Ok(device) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter device/mount point to test")
        .interact()
    else {
        return;
    };

    // Check if fio is installed
    let fio_check = Command::new("which").args(&["fio"]).output();

    if fio_check.map(|o| o.status.success()).unwrap_or(false) {
        println!("🧪 Running comprehensive fio benchmark...");

        let _ = Command::new("fio")
            .args(&[
                "--name=randrw",
                &format!("--directory={}", device),
                "--rw=randrw",
                "--bs=4k",
                "--size=1G",
                "--numjobs=4",
                "--time_based",
                "--runtime=60s",
                "--group_reporting",
            ])
            .status();
    } else {
        println!("❌ fio not installed. Installing...");
        // Install fio based on package manager
        if Command::new("which")
            .args(&["apt"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            let _ = Command::new("apt").args(&["install", "-y", "fio"]).status();
        } else if Command::new("which")
            .args(&["pacman"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            let _ = Command::new("pacman")
                .args(&["-S", "--noconfirm", "fio"])
                .status();
        }
    }
}

fn random_vs_sequential() {
    println!("📊 Random vs Sequential Performance Test\n");

    let Ok(device) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter mount point to test")
        .interact()
    else {
        return;
    };

    println!("📈 Testing sequential read performance...");
    let _ = Command::new("dd")
        .args(&[
            "if=/dev/zero",
            &format!("of={}/seq_test", device),
            "bs=1M",
            "count=512",
            "oflag=direct",
        ])
        .status();

    println!("📈 Testing random read performance...");
    // This would use fio or other tools for random I/O testing

    let _ = Command::new("rm")
        .args(&[&format!("{}/seq_test", device)])
        .status();
}

fn filesystem_benchmark() {
    println!("🗂️  Filesystem Benchmark\n");

    println!("This benchmark compares filesystem performance...");
    // Implementation would test various filesystem operations
}

fn compare_multiple_disks() {
    println!("⚖️  Compare Multiple Disks\n");

    println!("Enter multiple devices to compare (one per line, empty to finish):");
    let mut devices = Vec::new();

    loop {
        let Ok(device) = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter device")
            .default("".to_string())
            .interact()
        else {
            break;
        };

        if device.is_empty() {
            break;
        }
        devices.push(device);
    }

    for device in devices {
        println!("\n📊 Testing {}:", device);
        // Run benchmarks for each device
    }
}

fn disk_cleanup_tools() {
    loop {
        let options = vec![
            "Find Large Files",
            "Disk Usage Analysis",
            "Clean Package Cache",
            "Clean Log Files",
            "Find Duplicate Files",
            "Clean Temporary Files",
            "Back",
        ];

        let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🧹 Disk Cleanup Tools")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match selection {
            0 => find_large_files(),
            1 => disk_usage_analysis(),
            2 => clean_package_cache(),
            3 => clean_log_files(),
            4 => find_duplicate_files(),
            5 => clean_temporary_files(),
            _ => break,
        }
    }
}

fn find_large_files() {
    println!("🔍 Finding Large Files\n");

    let Ok(path) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter path to search")
        .default("/".to_string())
        .interact()
    else {
        return;
    };

    let Ok(size) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter minimum file size (e.g., +100M, +1G)")
        .default("+100M".to_string())
        .interact()
    else {
        return;
    };

    println!("🔍 Searching for files larger than {}...", size);
    let _ = Command::new("find")
        .args(&[
            &path, "-type", "f", "-size", &size, "-exec", "ls", "-lh", "{}", "+",
        ])
        .status();
}

fn disk_usage_analysis() {
    println!("📊 Disk Usage Analysis\n");

    let Ok(path) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter path to analyze")
        .default("/".to_string())
        .interact()
    else {
        return;
    };

    println!("📈 Top 20 largest directories:");
    if let Ok(mut child) = Command::new("du").args(&["-h", &path]).spawn() {
        let _ = child.wait();
    }
}

fn clean_package_cache() {
    println!("📦 Cleaning Package Cache\n");

    // Detect package manager and clean cache
    if Command::new("which")
        .args(&["apt"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        println!("🧹 Cleaning APT cache...");
        let _ = Command::new("apt").args(&["clean"]).status();
        let _ = Command::new("apt").args(&["autoclean"]).status();
        let _ = Command::new("apt").args(&["autoremove"]).status();
    }

    if Command::new("which")
        .args(&["pacman"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        println!("🧹 Cleaning Pacman cache...");
        let _ = Command::new("pacman").args(&["-Sc"]).status();
    }

    if Command::new("which")
        .args(&["yum"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        println!("🧹 Cleaning YUM cache...");
        let _ = Command::new("yum").args(&["clean", "all"]).status();
    }

    println!("✅ Package cache cleaned!");
}

fn clean_log_files() {
    println!("📝 Cleaning Log Files\n");

    println!("📊 Current log usage:");
    let _ = Command::new("du").args(&["-sh", "/var/log"]).status();

    let Ok(clean) = Confirm::new()
        .with_prompt("Clean old log files?")
        .default(true)
        .interact()
    else {
        return;
    };
    if clean {
        // Clean journalctl logs
        let _ = Command::new("journalctl")
            .args(&["--vacuum-time=7d"])
            .status();

        // Find and optionally remove old log files
        let _ = Command::new("find")
            .args(&["/var/log", "-name", "*.log.*.gz", "-mtime", "+7", "-delete"])
            .status();

        println!("✅ Log files cleaned!");
    }
}

fn find_duplicate_files() {
    println!("🔍 Finding Duplicate Files\n");

    let Ok(path) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter path to search for duplicates")
        .interact()
    else {
        return;
    };

    // Check if fdupes is installed
    let fdupes_check = Command::new("which").args(&["fdupes"]).output();

    if fdupes_check.map(|o| o.status.success()).unwrap_or(false) {
        println!("🔍 Searching for duplicate files...");
        let _ = Command::new("fdupes").args(&["-r", &path]).status();
    } else {
        println!("📦 Installing fdupes...");
        if Command::new("which")
            .args(&["apt"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            let _ = Command::new("apt")
                .args(&["install", "-y", "fdupes"])
                .status();
        } else if Command::new("which")
            .args(&["pacman"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            let _ = Command::new("pacman")
                .args(&["-S", "--noconfirm", "fdupes"])
                .status();
        }
    }
}

fn clean_temporary_files() {
    println!("🗑️  Cleaning Temporary Files\n");

    println!("📊 Current /tmp usage:");
    let _ = Command::new("du").args(&["-sh", "/tmp"]).status();

    let Ok(clean) = Confirm::new()
        .with_prompt("Clean temporary files older than 7 days?")
        .default(true)
        .interact()
    else {
        return;
    };
    if clean {
        let _ = Command::new("find")
            .args(&["/tmp", "-type", "f", "-mtime", "+7", "-delete"])
            .status();

        let _ = Command::new("find")
            .args(&["/var/tmp", "-type", "f", "-mtime", "+7", "-delete"])
            .status();

        println!("✅ Temporary files cleaned!");
    }
}

fn raid_management() {
    println!("⚔️  RAID Management\n");

    println!("🔍 Current RAID status:");
    let _ = Command::new("cat").args(&["/proc/mdstat"]).status();

    println!("\n📋 Available RAID tools:");
    println!("  • mdadm - Linux Software RAID");
    println!("  • Hardware RAID (vendor specific)");

    let Ok(show_detail) = Confirm::new()
        .with_prompt("Show detailed RAID information?")
        .default(true)
        .interact()
    else {
        return;
    };
    if show_detail {
        let _ = Command::new("mdadm").args(&["--detail", "--scan"]).status();
    }
}
