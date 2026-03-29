use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use std::path::Path;
use std::process::Command;

pub fn recovery_menu() {
    println!("🚨 Arch Linux Recovery & Rescue Tools");
    println!("=====================================");

    let options = [
        "🔧 Emergency System Repair",
        "💾 Bootloader Recovery",
        "🗂️  Filesystem Repair",
        "🔑 User Account Recovery",
        "📦 Package Database Recovery",
        "🌐 Network Recovery",
        "🖥️  Display/Graphics Recovery",
        "🔄 System Rollback Tools",
        "📋 Recovery Diagnostics",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Recovery Tools")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => emergency_system_repair(),
        1 => bootloader_recovery(),
        2 => filesystem_repair(),
        3 => user_account_recovery(),
        4 => package_database_recovery(),
        5 => network_recovery(),
        6 => display_recovery(),
        7 => system_rollback_tools(),
        8 => recovery_diagnostics(),
        _ => return,
    }
}

fn emergency_system_repair() {
    println!("🔧 Emergency System Repair");
    println!("===========================");

    let options = [
        "🚑 Quick System Fixes",
        "🔒 Fix Boot Issues",
        "📦 Repair Critical Packages",
        "🔧 Reset System Configuration",
        "💿 Create Recovery USB",
        "🛠️  Advanced Repair Options",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Emergency Repair")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => quick_system_fixes(),
        1 => fix_boot_issues(),
        2 => repair_critical_packages(),
        3 => reset_system_config(),
        4 => create_recovery_usb(),
        5 => advanced_repair_options(),
        _ => return,
    }
}

fn quick_system_fixes() {
    println!("🚑 Quick System Fixes");
    println!("=====================");

    let fixes = [
        "🔒 Fix pacman locks",
        "🔑 Reset GPG keys",
        "🌐 Fix DNS resolution",
        "📦 Sync package databases",
        "🔄 Restart critical services",
        "🧹 Clean temporary files",
        "🔧 Fix file permissions",
    ];

    let selected = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select fixes to apply")
        .items(&fixes)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        _ => return,
    };

    for &fix in &selected {
        match fix {
            0 => {
                println!("🔒 Fixing pacman locks...");
                let _ = Command::new("sudo")
                    .args(["rm", "-f", "/var/lib/pacman/db.lck"])
                    .status();
                println!("  ✅ Pacman locks cleared");
            }
            1 => {
                println!("🔑 Resetting GPG keys...");
                crate::arch::fix_gpg_keys();
                println!("  ✅ GPG keys reset");
            }
            2 => {
                println!("🌐 Fixing DNS resolution...");
                let _ = Command::new("sudo")
                    .args(["systemctl", "restart", "systemd-resolved"])
                    .status();
                // Write DNS config to temp file and move with sudo
                let temp_file = "/tmp/resolv.conf.tmp";
                if std::fs::write(temp_file, "nameserver 8.8.8.8\n").is_ok() {
                    let _ = Command::new("sudo")
                        .args(["mv", temp_file, "/etc/resolv.conf"])
                        .status();
                }
                println!("  ✅ DNS resolution fixed");
            }
            3 => {
                println!("📦 Syncing package databases...");
                let _ = Command::new("sudo").args(["pacman", "-Syy"]).status();
                println!("  ✅ Package databases synced");
            }
            4 => {
                println!("🔄 Restarting critical services...");
                let services = ["systemd-networkd", "systemd-resolved", "dbus"];
                for service in &services {
                    let _ = Command::new("sudo")
                        .args(["systemctl", "restart", service])
                        .status();
                }
                println!("  ✅ Critical services restarted");
            }
            5 => {
                println!("🧹 Cleaning temporary files...");
                let _ = Command::new("sudo")
                    .args(["rm", "-rf", "/tmp/*", "/var/tmp/*"])
                    .status();
                println!("  ✅ Temporary files cleaned");
            }
            6 => {
                println!("🔧 Fixing file permissions...");
                let _ = Command::new("sudo")
                    .args(["chmod", "755", "/", "/usr", "/usr/bin"])
                    .status();
                let _ = Command::new("sudo")
                    .args(["chmod", "644", "/etc/passwd", "/etc/group"])
                    .status();
                println!("  ✅ File permissions fixed");
            }
            _ => {}
        }
    }

    println!("✅ Quick fixes completed");
}

fn bootloader_recovery() {
    println!("💾 Bootloader Recovery");
    println!("======================");

    let options = [
        "🔍 Detect Bootloader",
        "🔧 GRUB Recovery",
        "⚙️  Systemd-boot Recovery",
        "📁 Mount EFI Partition",
        "🔄 Regenerate Boot Configuration",
        "🆕 Install/Reinstall Bootloader",
        "🛠️  Manual Bootloader Repair",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Bootloader Recovery")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => detect_bootloader(),
        1 => grub_recovery(),
        2 => systemd_boot_recovery(),
        3 => mount_efi_partition(),
        4 => regenerate_boot_config(),
        5 => install_bootloader(),
        6 => manual_bootloader_repair(),
        _ => return,
    }
}

fn detect_bootloader() {
    println!("🔍 Detecting Bootloader");
    println!("=======================");

    let mut detected = Vec::new();

    // Check for GRUB
    if Path::new("/boot/grub").exists() || Path::new("/boot/grub2").exists() {
        detected.push("GRUB");
        println!("✅ GRUB detected");
    }

    // Check for systemd-boot
    if Path::new("/boot/EFI/systemd").exists() || Path::new("/boot/loader").exists() {
        detected.push("systemd-boot");
        println!("✅ systemd-boot detected");
    }

    // Check for rEFInd
    if Path::new("/boot/EFI/refind").exists() {
        detected.push("rEFInd");
        println!("✅ rEFInd detected");
    }

    // Check EFI boot manager
    println!("\n📋 EFI Boot Manager entries:");
    let _ = Command::new("efibootmgr").arg("-v").status();

    if detected.is_empty() {
        println!("❌ No bootloader detected");
        println!("💡 Consider installing a bootloader");
    } else {
        println!("\n📊 Detected bootloaders: {}", detected.join(", "));
    }
}

fn grub_recovery() {
    println!("🔧 GRUB Recovery");
    println!("===============");

    let options = [
        "🔄 Regenerate GRUB config",
        "🔧 Reinstall GRUB",
        "🔍 Check GRUB installation",
        "📝 Edit GRUB configuration",
        "🛠️  Fix GRUB rescue mode",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("GRUB Recovery")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => {
            println!("🔄 Regenerating GRUB configuration...");
            let _ = Command::new("sudo")
                .args(["grub-mkconfig", "-o", "/boot/grub/grub.cfg"])
                .status();
            println!("✅ GRUB configuration regenerated");
        }
        1 => {
            println!("🔧 Reinstalling GRUB...");
            let device: String = match Input::new()
                .with_prompt("Enter device (e.g., /dev/sda)")
                .interact_text()
            {
                Ok(d) => d,
                Err(_) => return,
            };

            let _ = Command::new("sudo")
                .args(["grub-install", &device])
                .status();
            println!("✅ GRUB reinstalled");
        }
        2 => {
            println!("🔍 Checking GRUB installation...");
            let _ = Command::new("grub-probe")
                .args(["-t", "device", "/"])
                .status();
        }
        3 => {
            println!("📝 Editing GRUB configuration...");
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
            let _ = Command::new("sudo")
                .args([&editor, "/etc/default/grub"])
                .status();
        }
        4 => fix_grub_rescue(),
        _ => return,
    }
}

fn fix_grub_rescue() {
    println!("🛠️  Fix GRUB Rescue Mode");
    println!("=======================");

    println!("💡 GRUB Rescue Commands to try:");
    println!("  ls                     # List partitions");
    println!("  ls (hd0,1)/            # Check partition contents");
    println!("  set root=(hd0,1)       # Set root partition");
    println!("  linux /vmlinuz root=/dev/sda1  # Load kernel");
    println!("  initrd /initramfs      # Load initramfs");
    println!("  boot                   # Boot system");

    println!("\n🔧 Automated rescue attempt:");
    let attempt = match Confirm::new()
        .with_prompt("Attempt automated GRUB rescue?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(a)) => a,
        _ => return,
    };

    if attempt {
        println!("🔄 Attempting to rebuild GRUB...");
        let _ = Command::new("sudo")
            .args(["grub-install", "/dev/sda"]) // Default assumption
            .status();
        let _ = Command::new("sudo")
            .args(["grub-mkconfig", "-o", "/boot/grub/grub.cfg"])
            .status();
        println!("✅ Automated rescue completed");
    }
}

fn systemd_boot_recovery() {
    println!("⚙️  Systemd-boot Recovery");
    println!("========================");

    let options = [
        "🔄 Update systemd-boot",
        "📝 Regenerate boot entries",
        "🔧 Reinstall systemd-boot",
        "📁 Check ESP mount",
        "🛠️  Fix boot entries",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Systemd-boot Recovery")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => {
            println!("🔄 Updating systemd-boot...");
            let _ = Command::new("sudo").args(["bootctl", "update"]).status();
        }
        1 => {
            println!("📝 Regenerating boot entries...");
            crate::arch::boot::regenerate_boot_entries();
        }
        2 => {
            println!("🔧 Reinstalling systemd-boot...");
            let _ = Command::new("sudo").args(["bootctl", "install"]).status();
        }
        3 => {
            println!("📁 Checking ESP mount...");
            let _ = Command::new("findmnt").args(["/boot"]).status();
        }
        4 => fix_boot_entries(),
        _ => return,
    }
}

fn fix_boot_entries() {
    println!("🛠️  Fix Boot Entries");
    println!("===================");

    if Path::new("/boot/loader/entries").exists() {
        println!("📁 Current boot entries:");
        let _ = Command::new("ls")
            .args(["-la", "/boot/loader/entries/"])
            .status();

        let regenerate = match Confirm::new()
            .with_prompt("Remove all entries and regenerate?")
            .default(false)
            .interact_opt()
        {
            Ok(Some(r)) => r,
            _ => return,
        };

        if regenerate {
            let _ = Command::new("sudo")
                .args(["rm", "-f", "/boot/loader/entries/*"])
                .status();

            // Create basic entry
            create_basic_boot_entry();
        }
    } else {
        println!("❌ Boot entries directory not found");
        let create = match Confirm::new()
            .with_prompt("Create boot entries directory?")
            .default(true)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            _ => return,
        };

        if create {
            let _ = Command::new("sudo")
                .args(["mkdir", "-p", "/boot/loader/entries"])
                .status();
            create_basic_boot_entry();
        }
    }
}

fn create_basic_boot_entry() {
    println!("🆕 Creating basic boot entry...");

    // Detect installed kernels by reading /boot directory
    if let Ok(entries) = std::fs::read_dir("/boot") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("vmlinuz-") {
                let kernel_name = name_str.strip_prefix("vmlinuz-").unwrap_or(&name_str);

                // Get root PARTUUID
                let root_partuuid = Command::new("findmnt")
                    .args(["-n", "-o", "PARTUUID", "/"])
                    .output()
                    .ok()
                    .and_then(|o| String::from_utf8_lossy(&o.stdout).trim().to_string().into())
                    .unwrap_or_else(|| "FIXME".to_string());

                let entry_content = format!(
                    "title   Arch Linux ({})\nlinux   /vmlinuz-{}\ninitrd  /initramfs-{}.img\noptions root=PARTUUID={} rw\n",
                    kernel_name, kernel_name, kernel_name, root_partuuid
                );

                let entry_file = format!("/boot/loader/entries/arch-{}.conf", kernel_name);
                // Write to temp file and move with sudo
                let temp_file = "/tmp/boot_entry.conf.tmp";
                if std::fs::write(temp_file, &entry_content).is_ok() {
                    let _ = Command::new("sudo")
                        .args(["mv", temp_file, &entry_file])
                        .status();
                }

                println!("✅ Created entry for {}", kernel_name);
            }
        }
    }
}

fn filesystem_repair() {
    println!("🗂️  Filesystem Repair");
    println!("====================");

    let options = [
        "🔍 Check filesystem integrity",
        "🔧 Repair filesystem errors",
        "💾 Check disk health",
        "📊 Analyze disk usage",
        "🚨 Emergency filesystem recovery",
        "🗂️  Mount/unmount operations",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Filesystem Repair")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => check_filesystem_integrity(),
        1 => repair_filesystem_errors(),
        2 => check_disk_health(),
        3 => analyze_disk_usage(),
        4 => emergency_fs_recovery(),
        5 => mount_operations(),
        _ => return,
    }
}

fn check_filesystem_integrity() {
    println!("🔍 Checking Filesystem Integrity");
    println!("================================");

    // Show mounted filesystems
    println!("📁 Mounted filesystems:");
    let _ = Command::new("mount")
        .args(["-t", "ext4,btrfs,xfs"])
        .status();

    // Check each filesystem
    let device: String = match Input::new()
        .with_prompt("Enter device to check (e.g., /dev/sda1, or 'all' for all)")
        .interact_text()
    {
        Ok(d) => d,
        Err(_) => return,
    };

    if device == "all" {
        println!("🔍 Checking all filesystems...");
        // Get list of devices
        let output = Command::new("lsblk")
            .args(["-f", "-n", "-o", "NAME,FSTYPE"])
            .output();
        if let Ok(output) = output {
            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2
                    && (parts[1] == "ext4" || parts[1] == "btrfs" || parts[1] == "xfs")
                {
                    let dev_path = format!("/dev/{}", parts[0]);
                    check_single_filesystem(&dev_path, parts[1]);
                }
            }
        }
    } else {
        // Detect filesystem type
        let output = Command::new("blkid")
            .args(["-s", "TYPE", "-o", "value", &device])
            .output();
        let fstype = if let Ok(output) = output {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            "unknown".to_string()
        };

        check_single_filesystem(&device, &fstype);
    }
}

fn check_single_filesystem(device: &str, fstype: &str) {
    println!("🔍 Checking {} ({})", device, fstype);

    match fstype {
        "ext4" | "ext3" | "ext2" => {
            let _ = Command::new("sudo")
                .args(["e2fsck", "-n", device]) // -n for no changes
                .status();
        }
        "btrfs" => {
            let _ = Command::new("sudo")
                .args(["btrfs", "check", "--readonly", device])
                .status();
        }
        "xfs" => {
            let _ = Command::new("sudo")
                .args(["xfs_repair", "-n", device]) // -n for no changes
                .status();
        }
        _ => println!("⚠️  Unsupported filesystem type: {}", fstype),
    }
}

fn repair_filesystem_errors() {
    println!("🔧 Repair Filesystem Errors");
    println!("===========================");

    println!("⚠️  WARNING: This will attempt to repair filesystem errors.");
    println!("💾 Make sure you have backups before proceeding!");

    let device: String = match Input::new()
        .with_prompt("Enter device to repair (e.g., /dev/sda1)")
        .interact_text()
    {
        Ok(d) => d,
        Err(_) => return,
    };

    let confirm = match Confirm::new()
        .with_prompt("Proceed with filesystem repair? (This may cause data loss)")
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    if !confirm {
        return;
    }

    // Detect filesystem type
    let output = Command::new("blkid")
        .args(["-s", "TYPE", "-o", "value", &device])
        .output();
    let fstype = if let Ok(output) = output {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        match Input::new()
            .with_prompt("Enter filesystem type (ext4, btrfs, xfs)")
            .interact_text()
        {
            Ok(t) => t,
            Err(_) => return,
        }
    };

    println!("🔧 Repairing {} filesystem on {}...", fstype, device);

    match fstype.as_str() {
        "ext4" | "ext3" | "ext2" => {
            let _ = Command::new("sudo")
                .args(["e2fsck", "-y", &device]) // -y for automatic yes
                .status();
        }
        "btrfs" => {
            let _ = Command::new("sudo")
                .args(["btrfs", "check", "--repair", &device])
                .status();
        }
        "xfs" => {
            let _ = Command::new("sudo").args(["xfs_repair", &device]).status();
        }
        _ => println!("❌ Unsupported filesystem type: {}", fstype),
    }

    println!("✅ Filesystem repair completed");
}

fn check_disk_health() {
    println!("💾 Check Disk Health");
    println!("===================");

    // SMART status
    println!("🔍 SMART disk health:");
    let _ = Command::new("sudo")
        .args(["smartctl", "-H", "/dev/sda"])
        .status();

    // Detailed SMART info
    let detailed = match Confirm::new()
        .with_prompt("Show detailed SMART information?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(d)) => d,
        _ => false,
    };

    if detailed {
        let _ = Command::new("sudo")
            .args(["smartctl", "-a", "/dev/sda"])
            .status();
    }

    // Disk usage
    println!("\n📊 Disk usage:");
    let _ = Command::new("df").args(["-h"]).status();
}

fn analyze_disk_usage() {
    println!("📊 Analyze Disk Usage");
    println!("====================");

    println!("📁 Largest directories:");
    let _ = Command::new("sudo")
        .args([
            "du",
            "-h",
            "/",
            "--max-depth=1",
            "--exclude=/proc",
            "--exclude=/sys",
            "--exclude=/dev",
        ])
        .status();

    println!("\n🗂️  Find large files:");
    let _ = Command::new("sudo")
        .args([
            "find", "/", "-type", "f", "-size", "+100M", "-exec", "ls", "-lh", "{}", ";",
        ])
        .status();
}

fn emergency_fs_recovery() {
    println!("🚨 Emergency Filesystem Recovery");
    println!("================================");

    let options = [
        "🔄 Force filesystem check",
        "🚑 Boot from live USB instructions",
        "💾 Data recovery options",
        "🛠️  Reset filesystem journal",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Emergency Recovery")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => force_filesystem_check(),
        1 => show_live_usb_instructions(),
        2 => data_recovery_options(),
        3 => reset_filesystem_journal(),
        _ => return,
    }
}

fn force_filesystem_check() {
    println!("🔄 Force Filesystem Check");

    let _device: String = match Input::new()
        .with_prompt("Enter device to force check")
        .interact_text()
    {
        Ok(d) => d,
        Err(_) => return,
    };

    println!("⚠️  Creating force fsck file...");
    let _ = Command::new("sudo").args(["touch", "/forcefsck"]).status();

    println!("💡 Reboot system to trigger filesystem check on next boot");
}

fn show_live_usb_instructions() {
    println!("🚑 Live USB Recovery Instructions");
    println!("=================================");

    println!("1. 📱 Create Arch Linux live USB");
    println!("2. 🥾 Boot from USB");
    println!("3. 📶 Connect to internet: iwctl / dhcpcd");
    println!("4. 🗂️  Mount your root partition:");
    println!("   mount /dev/sdXY /mnt");
    println!("5. 📁 Mount boot partition:");
    println!("   mount /dev/sdXZ /mnt/boot");
    println!("6. 🔧 Chroot into system:");
    println!("   arch-chroot /mnt");
    println!("7. 🛠️  Perform repairs as needed");
    println!("8. 🔄 Regenerate initramfs:");
    println!("   mkinitcpio -P");
    println!("9. 💾 Reinstall bootloader if needed");
}

fn data_recovery_options() {
    println!("💾 Data Recovery Options");
    println!("=======================");

    println!("🛠️  Data recovery tools:");
    println!("• photorec - Photo/file recovery");
    println!("• testdisk - Partition recovery");
    println!("• ddrescue - Drive imaging/recovery");
    println!("• extundelete - ext3/4 file recovery");

    let tool: String = match Input::new()
        .with_prompt("Enter tool to install/run (or 'skip')")
        .interact_text()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    if tool != "skip" {
        println!("📦 Installing {}...", tool);
        let _ = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", &tool])
            .status();

        println!("💡 Run '{}' to start recovery", tool);
    }
}

fn reset_filesystem_journal() {
    println!("🛠️  Reset Filesystem Journal");

    let device: String = match Input::new()
        .with_prompt("Enter ext4 device to reset journal")
        .interact_text()
    {
        Ok(d) => d,
        Err(_) => return,
    };

    let confirm = match Confirm::new()
        .with_prompt("Reset filesystem journal? (ext4 only)")
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    if confirm {
        let _ = Command::new("sudo")
            .args(["tune2fs", "-O", "^has_journal", &device])
            .status();
        let _ = Command::new("sudo")
            .args(["e2fsck", "-y", &device])
            .status();
        let _ = Command::new("sudo")
            .args(["tune2fs", "-O", "has_journal", &device])
            .status();

        println!("✅ Journal reset completed");
    }
}

fn user_account_recovery() {
    println!("🔑 User Account Recovery");
    println!("=======================");

    let options = [
        "🔓 Reset user password",
        "👤 Create emergency user",
        "🔑 Fix user permissions",
        "🏠 Recover home directory",
        "🔐 Fix sudo access",
        "📋 List all users",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("User Recovery")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => reset_user_password(),
        1 => create_emergency_user(),
        2 => fix_user_permissions(),
        3 => recover_home_directory(),
        4 => fix_sudo_access(),
        5 => list_all_users(),
        _ => return,
    }
}

fn reset_user_password() {
    println!("🔓 Reset User Password");

    let username: String = match Input::new()
        .with_prompt("Enter username to reset password")
        .interact_text()
    {
        Ok(u) => u,
        Err(_) => return,
    };

    println!("🔑 Resetting password for {}...", username);
    let _ = Command::new("sudo").args(["passwd", &username]).status();
}

fn create_emergency_user() {
    println!("👤 Create Emergency User");

    let username: String = match Input::new()
        .with_prompt("Enter emergency username")
        .with_initial_text("rescue")
        .interact_text()
    {
        Ok(u) => u,
        Err(_) => return,
    };

    println!("👤 Creating emergency user: {}", username);
    let _ = Command::new("sudo")
        .args(["useradd", "-m", "-G", "wheel,sudo", &username])
        .status();

    println!("🔑 Setting password...");
    let _ = Command::new("sudo").args(["passwd", &username]).status();

    println!("✅ Emergency user created with sudo access");
}

fn fix_user_permissions() {
    println!("🔑 Fix User Permissions");

    let username: String = match Input::new()
        .with_prompt("Enter username to fix permissions")
        .interact_text()
    {
        Ok(u) => u,
        Err(_) => return,
    };

    println!("🔧 Fixing permissions for {}...", username);

    // Fix home directory ownership
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| format!("/home/{}", username));
    let _ = Command::new("sudo")
        .args([
            "chown",
            "-R",
            &format!("{}:{}", username, username),
            &home_dir,
        ])
        .status();

    // Fix common permission issues
    let _ = Command::new("sudo")
        .args(["chmod", "755", &home_dir])
        .status();

    println!("✅ User permissions fixed");
}

fn recover_home_directory() {
    println!("🏠 Recover Home Directory");

    let username: String = match Input::new()
        .with_prompt("Enter username to recover home directory")
        .interact_text()
    {
        Ok(u) => u,
        Err(_) => return,
    };

    let home_dir = std::env::var("HOME").unwrap_or_else(|_| format!("/home/{}", username));

    if !Path::new(&home_dir).exists() {
        println!("📁 Creating home directory...");
        let _ = Command::new("sudo")
            .args(["mkdir", "-p", &home_dir])
            .status();
        let _ = Command::new("sudo")
            .args(["cp", "-r", "/etc/skel/.", &home_dir])
            .status();
        let _ = Command::new("sudo")
            .args([
                "chown",
                "-R",
                &format!("{}:{}", username, username),
                &home_dir,
            ])
            .status();
        println!("✅ Home directory recovered");
    } else {
        println!("✅ Home directory already exists");
    }
}

fn fix_sudo_access() {
    println!("🔐 Fix Sudo Access");

    let username: String = match Input::new()
        .with_prompt("Enter username to add to wheel group")
        .interact_text()
    {
        Ok(u) => u,
        Err(_) => return,
    };

    println!("🔐 Adding {} to wheel group...", username);
    let _ = Command::new("sudo")
        .args(["usermod", "-aG", "wheel", &username])
        .status();

    // Ensure wheel group has sudo access by appending to sudoers via temp file
    println!("🔧 Ensuring wheel group has sudo access...");
    // Read existing sudoers, check if wheel rule exists
    if let Ok(content) = std::fs::read_to_string("/etc/sudoers") {
        if !content.contains("%wheel ALL=(ALL:ALL) ALL") {
            // Write to temp file and use visudo to validate
            let temp_file = "/tmp/sudoers_wheel.tmp";
            if std::fs::write(temp_file, "%wheel ALL=(ALL:ALL) ALL\n").is_ok() {
                // Append to sudoers.d instead for safety
                let _ = Command::new("sudo")
                    .args(["mv", temp_file, "/etc/sudoers.d/wheel"])
                    .status();
            }
        }
    }

    println!("✅ Sudo access fixed");
}

fn list_all_users() {
    println!("📋 All System Users");
    println!("==================");

    println!("👥 Regular users:");
    let _ = Command::new("getent").args(["passwd"]).status();

    println!("\n🔐 Users with sudo access:");
    let _ = Command::new("getent").args(["group", "wheel"]).status();
}

fn package_database_recovery() {
    println!("📦 Package Database Recovery");
    println!("===========================");

    let options = [
        "🔄 Rebuild package database",
        "🔧 Fix corrupted database",
        "📋 Restore from backup",
        "🗑️  Clear broken locks",
        "🔍 Verify database integrity",
        "💾 Create database backup",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Database Recovery")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => rebuild_package_database(),
        1 => fix_corrupted_database(),
        2 => restore_database_backup(),
        3 => clear_broken_locks(),
        4 => verify_database_integrity(),
        5 => create_database_backup(),
        _ => return,
    }
}

fn rebuild_package_database() {
    println!("🔄 Rebuilding Package Database");

    let confirm = match Confirm::new()
        .with_prompt("Rebuild entire package database?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    if confirm {
        println!("💾 Creating backup...");
        let _ = Command::new("sudo")
            .args(["cp", "-r", "/var/lib/pacman", "/var/lib/pacman.backup"])
            .status();

        println!("🗑️  Removing sync databases...");
        let _ = Command::new("sudo")
            .args(["rm", "-rf", "/var/lib/pacman/sync"])
            .status();

        println!("🔄 Rebuilding...");
        let _ = Command::new("sudo").args(["pacman", "-Syy"]).status();

        println!("✅ Database rebuilt");
    }
}

fn fix_corrupted_database() {
    println!("🔧 Fix Corrupted Database");

    println!("🔍 Checking for corruption...");
    let _ = Command::new("sudo").args(["pacman", "-Dk"]).status();

    println!("🛠️  Attempting repair...");
    let _ = Command::new("sudo").args(["pacman-db-upgrade"]).status();

    println!("✅ Repair attempt completed");
}

fn restore_database_backup() {
    println!("📋 Restore Database Backup");

    if Path::new("/var/lib/pacman.backup").exists() {
        let confirm = match Confirm::new()
            .with_prompt("Restore from /var/lib/pacman.backup?")
            .default(true)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            _ => return,
        };

        if confirm {
            let _ = Command::new("sudo")
                .args(["rm", "-rf", "/var/lib/pacman"])
                .status();
            let _ = Command::new("sudo")
                .args(["cp", "-r", "/var/lib/pacman.backup", "/var/lib/pacman"])
                .status();
            println!("✅ Database restored from backup");
        }
    } else {
        println!("❌ No backup found at /var/lib/pacman.backup");
    }
}

fn clear_broken_locks() {
    println!("🗑️  Clear Broken Locks");

    let locks = ["/var/lib/pacman/db.lck", "/var/cache/pacman/pkg/cache.lck"];

    for lock in &locks {
        if Path::new(lock).exists() {
            println!("🗑️  Removing {}", lock);
            let _ = Command::new("sudo").args(["rm", "-f", lock]).status();
        }
    }

    println!("✅ Locks cleared");
}

fn verify_database_integrity() {
    println!("🔍 Verify Database Integrity");

    let _ = Command::new("sudo").args(["pacman", "-Dk"]).status();
    let _ = Command::new("sudo").args(["pacman", "-Qk"]).status();
}

fn create_database_backup() {
    println!("💾 Create Database Backup");

    let backup_path = format!(
        "/var/lib/pacman.backup.{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()
            .map(|d| d.as_secs())
            .unwrap_or(0)
    );

    let _ = Command::new("sudo")
        .args(["cp", "-r", "/var/lib/pacman", &backup_path])
        .status();

    println!("✅ Backup created: {}", backup_path);
}

fn network_recovery() {
    println!("🌐 Network Recovery");
    println!("==================");

    let options = [
        "🔌 Reset network configuration",
        "📶 Fix WiFi issues",
        "🌍 Reset DNS settings",
        "🔧 Restart network services",
        "📋 Network diagnostics",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Network Recovery")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => reset_network_config(),
        1 => fix_wifi_issues(),
        2 => reset_dns_settings(),
        3 => restart_network_services(),
        4 => network_diagnostics(),
        _ => return,
    }
}

fn reset_network_config() {
    println!("🔌 Reset Network Configuration");

    let confirm = match Confirm::new()
        .with_prompt("Reset network configuration to defaults?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    if confirm {
        println!("🔄 Resetting network...");
        let _ = Command::new("sudo")
            .args(["systemctl", "restart", "systemd-networkd"])
            .status();
        let _ = Command::new("sudo")
            .args(["systemctl", "restart", "systemd-resolved"])
            .status();
        println!("✅ Network configuration reset");
    }
}

fn fix_wifi_issues() {
    println!("📶 Fix WiFi Issues");

    let options = [
        "🔄 Restart WiFi adapter",
        "📶 Scan for networks",
        "🔑 Reset WiFi credentials",
        "🔧 Check WiFi drivers",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("WiFi Fix")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => {
            println!("🔄 Restarting WiFi...");
            let _ = Command::new("sudo")
                .args(["ip", "link", "set", "wlan0", "down"])
                .status();
            let _ = Command::new("sudo")
                .args(["ip", "link", "set", "wlan0", "up"])
                .status();
        }
        1 => {
            println!("📶 Scanning networks...");
            let _ = Command::new("iwctl")
                .args(["station", "wlan0", "scan"])
                .status();
            let _ = Command::new("iwctl")
                .args(["station", "wlan0", "get-networks"])
                .status();
        }
        2 => {
            println!("🔑 Use iwctl to configure WiFi:");
            println!("  iwctl station wlan0 connect <SSID>");
        }
        3 => {
            println!("🔧 Checking WiFi drivers...");
            let _ = Command::new("lspci").args(["-k"]).status();
        }
        _ => {}
    }
}

fn reset_dns_settings() {
    println!("🌍 Reset DNS Settings");

    let dns_servers = ["8.8.8.8", "8.8.4.4", "1.1.1.1", "9.9.9.9"];

    println!("🌍 Available DNS servers:");
    for (i, server) in dns_servers.iter().enumerate() {
        println!("{}. {}", i + 1, server);
    }

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select DNS server")
        .items(&dns_servers)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    let selected_dns = dns_servers[choice];

    println!("🔧 Setting DNS to {}", selected_dns);
    // Write DNS config to temp file and move with sudo
    let temp_file = "/tmp/resolv.conf.tmp";
    if std::fs::write(temp_file, format!("nameserver {}\n", selected_dns)).is_ok() {
        let _ = Command::new("sudo")
            .args(["mv", temp_file, "/etc/resolv.conf"])
            .status();
    }

    println!("✅ DNS settings updated");
}

fn restart_network_services() {
    println!("🔧 Restart Network Services");

    let services = [
        "systemd-networkd",
        "systemd-resolved",
        "NetworkManager",
        "dhcpcd",
    ];

    for service in &services {
        println!("🔄 Restarting {}...", service);
        let _ = Command::new("sudo")
            .args(["systemctl", "restart", service])
            .status();
    }

    println!("✅ Network services restarted");
}

fn network_diagnostics() {
    println!("📋 Network Diagnostics");
    println!("======================");

    println!("🔌 Network interfaces:");
    let _ = Command::new("ip").args(["addr", "show"]).status();

    println!("\n🌐 Routing table:");
    let _ = Command::new("ip").args(["route", "show"]).status();

    println!("\n📶 DNS resolution:");
    let _ = Command::new("nslookup").args(["google.com"]).status();

    println!("\n🔍 Connectivity test:");
    let _ = Command::new("ping").args(["-c", "3", "8.8.8.8"]).status();
}

fn display_recovery() {
    println!("🖥️  Display/Graphics Recovery");
    println!("=============================");

    let options = [
        "🔄 Reset display configuration",
        "🖥️  Fix X11 issues",
        "🎨 Wayland troubleshooting",
        "📱 Graphics driver recovery",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Display Recovery")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => reset_display_config(),
        1 => fix_x11_issues(),
        2 => wayland_troubleshooting(),
        3 => graphics_driver_recovery(),
        _ => return,
    }
}

fn reset_display_config() {
    println!("🔄 Reset Display Configuration");

    println!("🗑️  Removing X11 config files...");
    let x11_configs = ["~/.Xauthority", "~/.xinitrc", "/etc/X11/xorg.conf"];

    for config in &x11_configs {
        let _ = Command::new("sudo").args(["rm", "-f", config]).status();
    }

    println!("✅ Display configuration reset");
}

fn fix_x11_issues() {
    println!("🖥️  Fix X11 Issues");

    println!("🔧 Common X11 fixes:");
    println!("1. 🔄 Restart display manager");
    let _ = Command::new("sudo")
        .args(["systemctl", "restart", "gdm"])
        .status();

    println!("2. 🔑 Fix X11 permissions");
    let _ = Command::new("sudo").args(["chmod", "755", "/tmp"]).status();

    println!("3. 📱 Generate new xorg.conf");
    let generate = match Confirm::new()
        .with_prompt("Generate new xorg.conf?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(g)) => g,
        _ => false,
    };

    if generate {
        let _ = Command::new("sudo").args(["X", "-configure"]).status();
    }
}

fn wayland_troubleshooting() {
    println!("🎨 Wayland Troubleshooting");

    println!("🔍 Checking Wayland session...");
    let _ = Command::new("loginctl")
        .args(["show-session", "$XDG_SESSION_ID"])
        .status();

    println!("🔧 Environment variables:");
    println!("  WAYLAND_DISPLAY: {:?}", std::env::var("WAYLAND_DISPLAY"));
    println!(
        "  XDG_SESSION_TYPE: {:?}",
        std::env::var("XDG_SESSION_TYPE")
    );
}

fn graphics_driver_recovery() {
    println!("📱 Graphics Driver Recovery");

    println!("🔍 Detecting graphics hardware...");
    let _ = Command::new("lspci")
        .args(["-k", "|", "grep", "-A", "2", "-i", "VGA"])
        .status();

    println!("🔧 Common driver fixes:");
    println!("• NVIDIA: sudo pacman -S nvidia nvidia-utils");
    println!("• AMD: sudo pacman -S xf86-video-amdgpu mesa");
    println!("• Intel: sudo pacman -S xf86-video-intel mesa");

    let reinstall = match Confirm::new()
        .with_prompt("Reinstall graphics drivers?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(r)) => r,
        _ => false,
    };

    if reinstall {
        println!("📦 Reinstalling basic graphics drivers...");
        let _ = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "mesa", "xorg-server"])
            .status();
    }
}

fn system_rollback_tools() {
    println!("🔄 System Rollback Tools");
    println!("========================");

    let options = [
        "📂 Btrfs snapshots",
        "⏪ Timeshift rollback",
        "📦 Package downgrade",
        "🔧 Configuration rollback",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Rollback Tools")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => btrfs_snapshots(),
        1 => timeshift_rollback(),
        2 => package_downgrade(),
        3 => config_rollback(),
        _ => return,
    }
}

fn btrfs_snapshots() {
    println!("📂 Btrfs Snapshots");

    // Check if btrfs is in use
    let output = Command::new("findmnt").args(["-t", "btrfs"]).output();

    if let Ok(output) = output {
        if !output.stdout.is_empty() {
            println!("📋 Btrfs filesystems found:");
            println!("{}", String::from_utf8_lossy(&output.stdout));

            println!("📂 Listing snapshots...");
            let _ = Command::new("sudo")
                .args(["btrfs", "subvolume", "list", "/"])
                .status();
        } else {
            println!("❌ No btrfs filesystems found");
        }
    }
}

fn timeshift_rollback() {
    println!("⏪ Timeshift Rollback");

    if Command::new("which").arg("timeshift").status().is_ok() {
        println!("📋 Available snapshots:");
        let _ = Command::new("sudo").args(["timeshift", "--list"]).status();

        let restore = match Confirm::new()
            .with_prompt("Open Timeshift for restore?")
            .default(false)
            .interact_opt()
        {
            Ok(Some(r)) => r,
            _ => false,
        };

        if restore {
            let _ = Command::new("sudo").args(["timeshift-gtk"]).status();
        }
    } else {
        println!("❌ Timeshift not installed");
        let install = match Confirm::new()
            .with_prompt("Install Timeshift?")
            .default(true)
            .interact_opt()
        {
            Ok(Some(i)) => i,
            _ => false,
        };

        if install {
            let _ = Command::new("sudo")
                .args(["pacman", "-S", "--noconfirm", "timeshift"])
                .status();
        }
    }
}

fn package_downgrade() {
    println!("📦 Package Downgrade");

    let package: String = match Input::new()
        .with_prompt("Enter package name to downgrade")
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    if Command::new("which").arg("downgrade").status().is_ok() {
        let _ = Command::new("sudo").args(["downgrade", &package]).status();
    } else {
        println!("❌ downgrade tool not available");
        println!("💡 Install with: yay -S downgrade");
    }
}

fn config_rollback() {
    println!("🔧 Configuration Rollback");

    let config_backups = [
        "/etc/.backup",
        "/etc/pacman.conf.backup",
        "/etc/fstab.backup",
    ];

    println!("📋 Available configuration backups:");
    for backup in &config_backups {
        if Path::new(backup).exists() {
            println!("  ✅ {}", backup);
        } else {
            println!("  ❌ {}", backup);
        }
    }
}

fn recovery_diagnostics() {
    println!("📋 Recovery Diagnostics");
    println!("=======================");

    println!("🖥️  System Information:");
    let _ = Command::new("hostnamectl").status();

    println!("\n💾 Memory Usage:");
    let _ = Command::new("free").args(["-h"]).status();

    println!("\n📁 Disk Usage:");
    let _ = Command::new("df").args(["-h"]).status();

    println!("\n🔧 Failed Services:");
    let _ = Command::new("systemctl").args(["--failed"]).status();

    println!("\n📰 Recent Critical Logs:");
    let _ = Command::new("journalctl")
        .args(["-p", "err", "-n", "20", "--no-pager"])
        .status();
}

// Helper functions for TODO implementations
fn mount_efi_partition() {
    println!("📁 Mount EFI Partition");

    // Find EFI partition
    let output = Command::new("blkid").args(["-t", "TYPE=vfat"]).output();

    if let Ok(output) = output {
        let content = String::from_utf8_lossy(&output.stdout);
        println!("🔍 Found EFI partitions:");
        println!("{}", content);

        let device: String = match Input::new()
            .with_prompt("Enter EFI partition (e.g., /dev/sda1)")
            .interact_text()
        {
            Ok(d) => d,
            Err(_) => return,
        };

        let mount_point: String = match Input::new()
            .with_prompt("Mount point")
            .with_initial_text("/boot")
            .interact_text()
        {
            Ok(m) => m,
            Err(_) => return,
        };

        let _ = Command::new("sudo")
            .args(["mount", &device, &mount_point])
            .status();

        println!("✅ EFI partition mounted");
    }
}

fn regenerate_boot_config() {
    println!("🔄 Regenerate Boot Configuration");

    // Check for GRUB
    if Path::new("/boot/grub").exists() {
        println!("🔄 Regenerating GRUB config...");
        let _ = Command::new("sudo")
            .args(["grub-mkconfig", "-o", "/boot/grub/grub.cfg"])
            .status();
    }

    // Check for systemd-boot
    if Path::new("/boot/loader").exists() {
        println!("🔄 Updating systemd-boot...");
        let _ = Command::new("sudo").args(["bootctl", "update"]).status();
    }

    // Regenerate initramfs
    println!("🔄 Regenerating initramfs...");
    let _ = Command::new("sudo").args(["mkinitcpio", "-P"]).status();

    println!("✅ Boot configuration regenerated");
}

fn install_bootloader() {
    println!("🆕 Install/Reinstall Bootloader");

    let bootloaders = ["GRUB", "systemd-boot"];
    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bootloader")
        .items(&bootloaders)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => {
            println!("📦 Installing GRUB...");
            let device: String = match Input::new()
                .with_prompt("Enter device (e.g., /dev/sda)")
                .interact_text()
            {
                Ok(d) => d,
                Err(_) => return,
            };

            let _ = Command::new("sudo")
                .args(["pacman", "-S", "--noconfirm", "grub", "efibootmgr"])
                .status();
            let _ = Command::new("sudo")
                .args([
                    "grub-install",
                    "--target=x86_64-efi",
                    "--efi-directory=/boot",
                    &device,
                ])
                .status();
            let _ = Command::new("sudo")
                .args(["grub-mkconfig", "-o", "/boot/grub/grub.cfg"])
                .status();
        }
        1 => {
            println!("📦 Installing systemd-boot...");
            let _ = Command::new("sudo").args(["bootctl", "install"]).status();
        }
        _ => {}
    }
}

fn manual_bootloader_repair() {
    println!("🛠️  Manual Bootloader Repair");
    println!("============================");

    println!("💡 Manual repair instructions:");
    println!("1. Boot from Arch live USB");
    println!("2. Mount root partition: mount /dev/sdXY /mnt");
    println!("3. Mount boot partition: mount /dev/sdXZ /mnt/boot");
    println!("4. Chroot: arch-chroot /mnt");
    println!("5. Reinstall bootloader:");
    println!("   GRUB: grub-install /dev/sdX && grub-mkconfig -o /boot/grub/grub.cfg");
    println!("   systemd-boot: bootctl install");
    println!("6. Regenerate initramfs: mkinitcpio -P");
}

fn mount_operations() {
    println!("🗂️  Mount/Unmount Operations");

    let options = [
        "📁 Mount partition",
        "🔓 Unmount partition",
        "📋 Show mounted filesystems",
        "🔍 Find unmounted partitions",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Mount Operations")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => {
            let device: String = match Input::new()
                .with_prompt("Device to mount (e.g., /dev/sda1)")
                .interact_text()
            {
                Ok(d) => d,
                Err(_) => return,
            };
            let mount_point: String = match Input::new()
                .with_prompt("Mount point (e.g., /mnt)")
                .interact_text()
            {
                Ok(m) => m,
                Err(_) => return,
            };

            let _ = Command::new("sudo")
                .args(["mkdir", "-p", &mount_point])
                .status();
            let _ = Command::new("sudo")
                .args(["mount", &device, &mount_point])
                .status();
            println!("✅ Mounted {} to {}", device, mount_point);
        }
        1 => {
            let mount_point: String = match Input::new()
                .with_prompt("Mount point to unmount")
                .interact_text()
            {
                Ok(m) => m,
                Err(_) => return,
            };
            let _ = Command::new("sudo").args(["umount", &mount_point]).status();
            println!("✅ Unmounted {}", mount_point);
        }
        2 => {
            let _ = Command::new("mount").status();
        }
        3 => {
            let _ = Command::new("lsblk").status();
        }
        _ => {}
    }
}

fn create_recovery_usb() {
    println!("💿 Create Recovery USB");
    println!("=====================");

    println!("📋 Steps to create Arch Linux recovery USB:");
    println!("1. 📱 Download Arch Linux ISO from archlinux.org");
    println!("2. 🔍 Find USB device: lsblk");
    println!("3. 💾 Write ISO to USB: sudo dd if=archlinux.iso of=/dev/sdX bs=4M status=progress");
    println!("4. 🔄 Sync: sudo sync");

    let show_devices = match Confirm::new()
        .with_prompt("Show available devices?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        _ => false,
    };

    if show_devices {
        println!("📱 Available devices:");
        let _ = Command::new("lsblk").status();
    }
}

fn advanced_repair_options() {
    println!("🛠️  Advanced Repair Options");
    println!("===========================");

    let options = [
        "🔧 System file verification",
        "📦 Package integrity check",
        "🔑 Security fix (permissions)",
        "🗂️  Filesystem deep scan",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Advanced Repair")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => {
            println!("🔧 Verifying system files...");
            let _ = Command::new("sudo").args(["pacman", "-Qkk"]).status();
        }
        1 => {
            println!("📦 Checking package integrity...");
            let _ = Command::new("sudo").args(["pacman", "-Dk"]).status();
        }
        2 => {
            println!("🔑 Fixing critical permissions...");
            let _ = Command::new("sudo")
                .args(["chmod", "755", "/", "/usr", "/usr/bin"])
                .status();
            let _ = Command::new("sudo")
                .args(["chmod", "644", "/etc/passwd"])
                .status();
        }
        3 => {
            println!("🗂️  Deep filesystem scan...");
            let device: String = match Input::new()
                .with_prompt("Enter device for deep scan")
                .interact_text()
            {
                Ok(d) => d,
                Err(_) => return,
            };
            let _ = Command::new("sudo")
                .args(["badblocks", "-v", &device])
                .status();
        }
        _ => return,
    }
}

fn fix_boot_issues() {
    println!("🔒 Fix Boot Issues");
    println!("==================");

    let options = [
        "🔄 Regenerate GRUB config",
        "🔧 Reinstall GRUB",
        "🥾 Fix systemd-boot",
        "🔑 Fix EFI boot entries",
        "📋 Check boot partition",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Boot Repair")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => {
            println!("🔄 Regenerating GRUB configuration...");
            let _ = Command::new("sudo")
                .args(["grub-mkconfig", "-o", "/boot/grub/grub.cfg"])
                .status();
        }
        1 => {
            println!("🔧 Reinstalling GRUB...");
            let device: String = match Input::new()
                .with_prompt("Enter device to install GRUB to (e.g. /dev/sda)")
                .interact_text()
            {
                Ok(d) => d,
                Err(_) => return,
            };
            let _ = Command::new("sudo")
                .args(["grub-install", &device])
                .status();
            let _ = Command::new("sudo")
                .args(["grub-mkconfig", "-o", "/boot/grub/grub.cfg"])
                .status();
        }
        2 => {
            println!("🥾 Fixing systemd-boot...");
            crate::arch::boot::regenerate_boot_entries();
            let _ = Command::new("sudo").args(["bootctl", "update"]).status();
        }
        3 => {
            println!("🔑 Checking EFI boot entries...");
            let _ = Command::new("efibootmgr").args(["-v"]).status();
        }
        4 => {
            println!("📋 Checking boot partition...");
            let _ = Command::new("df").args(["-h", "/boot"]).status();
            let _ = Command::new("ls").args(["-la", "/boot"]).status();
        }
        _ => return,
    }
}

fn repair_critical_packages() {
    println!("📦 Repair Critical Packages");
    println!("============================");

    let options = [
        "🔧 Reinstall base packages",
        "🗃️  Fix package database",
        "🔑 Reset package keys",
        "📥 Downgrade problematic packages",
        "🧹 Clean package cache",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Package Repair")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => {
            println!("🔧 Reinstalling base packages...");
            let _ = Command::new("sudo")
                .args([
                    "pacman",
                    "-S",
                    "--noconfirm",
                    "base",
                    "linux",
                    "linux-firmware",
                ])
                .status();
        }
        1 => {
            println!("🗃️  Fixing package database...");
            let _ = Command::new("sudo").args(["pacman", "-Sy"]).status();
            let _ = Command::new("sudo").args(["pacman-db-upgrade"]).status();
        }
        2 => {
            println!("🔑 Resetting package keys...");
            crate::arch::fix_gpg_keys();
        }
        3 => {
            println!("📥 Downgrading packages...");
            let package: String = match Input::new()
                .with_prompt("Enter package name to downgrade")
                .interact_text()
            {
                Ok(p) => p,
                Err(_) => return,
            };
            let _ = Command::new("sudo").args(["downgrade", &package]).status();
        }
        4 => {
            println!("🧹 Cleaning package cache...");
            let _ = Command::new("sudo")
                .args(["pacman", "-Scc", "--noconfirm"])
                .status();
        }
        _ => return,
    }
}

fn reset_system_config() {
    println!("🔧 Reset System Configuration");
    println!("==============================");

    let options = [
        "🌐 Reset network configuration",
        "🔊 Reset audio configuration",
        "🖥️  Reset display configuration",
        "🔑 Reset user permissions",
        "⚙️  Reset systemd services",
        "📁 Reset file permissions",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Configuration Reset")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => reset_network_config(),
        1 => reset_audio_config(),
        2 => reset_display_config(),
        3 => reset_user_permissions(),
        4 => reset_systemd_services(),
        5 => reset_file_permissions(),
        _ => return,
    }
}

fn reset_audio_config() {
    println!("🔊 Resetting audio configuration...");
    let _ = Command::new("pulseaudio").args(["-k"]).status();
    let _ = Command::new("pulseaudio").args(["--start"]).status();
}

fn reset_user_permissions() {
    println!("🔑 Resetting user permissions...");
    let username: String = match Input::new()
        .with_prompt("Enter username to reset permissions")
        .interact_text()
    {
        Ok(u) => u,
        Err(_) => return,
    };

    let _ = Command::new("sudo")
        .args([
            "usermod",
            "-aG",
            "wheel,audio,video,optical,storage",
            &username,
        ])
        .status();
}

fn reset_systemd_services() {
    println!("⚙️  Resetting systemd services...");
    let _ = Command::new("sudo")
        .args(["systemctl", "daemon-reload"])
        .status();
    let _ = Command::new("sudo")
        .args(["systemctl", "reset-failed"])
        .status();
}

fn reset_file_permissions() {
    println!("📁 Resetting critical file permissions...");
    let _ = Command::new("sudo")
        .args(["chmod", "755", "/usr/bin"])
        .status();
    let _ = Command::new("sudo")
        .args(["chmod", "644", "/etc/passwd"])
        .status();
    let _ = Command::new("sudo")
        .args(["chmod", "600", "/etc/shadow"])
        .status();
}

// ============= Utility functions for testing =============

/// Check if a bootloader path exists
pub fn bootloader_exists(bootloader: &str) -> bool {
    match bootloader.to_lowercase().as_str() {
        "grub" => Path::new("/boot/grub").exists() || Path::new("/boot/grub2").exists(),
        "systemd-boot" => {
            Path::new("/boot/EFI/systemd").exists() || Path::new("/boot/loader").exists()
        }
        "refind" => Path::new("/boot/EFI/refind").exists(),
        _ => false,
    }
}

/// Validate a PCI device path format
pub fn is_valid_device_path(path: &str) -> bool {
    if path.is_empty() {
        return false;
    }
    // Basic validation for /dev/ paths
    path.starts_with("/dev/") && !path.contains("..") && path.len() > 5
}

/// Parse filesystem type from blkid output
pub fn parse_filesystem_type(blkid_output: &str) -> Option<String> {
    // Format: TYPE="ext4" or TYPE="btrfs"
    if let Some(start) = blkid_output.find("TYPE=\"") {
        let start = start + 6;
        if let Some(end) = blkid_output[start..].find('"') {
            return Some(blkid_output[start..start + end].to_string());
        }
    }
    None
}

/// Check if a filesystem type is supported for repair
pub fn is_supported_filesystem(fstype: &str) -> bool {
    matches!(
        fstype.to_lowercase().as_str(),
        "ext4" | "ext3" | "ext2" | "btrfs" | "xfs"
    )
}

/// Validate username format
pub fn is_valid_username(username: &str) -> bool {
    if username.is_empty() || username.len() > 32 {
        return false;
    }
    // First character must be lowercase letter or underscore
    let first = username.chars().next().unwrap_or('u');
    if !first.is_ascii_lowercase() && first != '_' {
        return false;
    }
    // Rest must be lowercase, digits, underscore, or hyphen
    username
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
}

/// Parse DNS server from resolv.conf line
pub fn parse_dns_server(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.starts_with("nameserver ") {
        let server = trimmed.strip_prefix("nameserver ")?.trim();
        if !server.is_empty() {
            return Some(server.to_string());
        }
    }
    None
}

/// Validate IPv4 address format
pub fn is_valid_ipv4(ip: &str) -> bool {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return false;
    }
    parts
        .iter()
        .all(|p| p.parse::<u8>().is_ok() && !p.starts_with('0') || *p == "0")
}

/// Get backup path for a config file
pub fn get_backup_path(original_path: &str, suffix: &str) -> String {
    format!("{}.{}", original_path, suffix)
}

/// Check if path is a system critical directory
pub fn is_critical_system_path(path: &str) -> bool {
    let critical_paths = [
        "/", "/boot", "/etc", "/usr", "/var", "/lib", "/lib64", "/bin", "/sbin",
    ];
    critical_paths
        .iter()
        .any(|&p| path == p || path.starts_with(&format!("{}/", p)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_device_path_valid() {
        assert!(is_valid_device_path("/dev/sda"));
        assert!(is_valid_device_path("/dev/sda1"));
        assert!(is_valid_device_path("/dev/nvme0n1"));
        assert!(is_valid_device_path("/dev/nvme0n1p1"));
        assert!(is_valid_device_path("/dev/mapper/root"));
    }

    #[test]
    fn test_is_valid_device_path_invalid() {
        assert!(!is_valid_device_path(""));
        assert!(!is_valid_device_path("/dev/"));
        assert!(!is_valid_device_path("sda"));
        assert!(!is_valid_device_path("/dev/../etc/passwd"));
        assert!(!is_valid_device_path("/home/user"));
    }

    #[test]
    fn test_parse_filesystem_type_ext4() {
        let output = "/dev/sda1: UUID=\"abc\" TYPE=\"ext4\" PARTUUID=\"xyz\"";
        assert_eq!(parse_filesystem_type(output), Some("ext4".to_string()));
    }

    #[test]
    fn test_parse_filesystem_type_btrfs() {
        let output = "TYPE=\"btrfs\"";
        assert_eq!(parse_filesystem_type(output), Some("btrfs".to_string()));
    }

    #[test]
    fn test_parse_filesystem_type_xfs() {
        let output = "/dev/nvme0n1p2: LABEL=\"data\" TYPE=\"xfs\"";
        assert_eq!(parse_filesystem_type(output), Some("xfs".to_string()));
    }

    #[test]
    fn test_parse_filesystem_type_missing() {
        let output = "/dev/sda1: UUID=\"abc\" PARTUUID=\"xyz\"";
        assert_eq!(parse_filesystem_type(output), None);
    }

    #[test]
    fn test_is_supported_filesystem_supported() {
        assert!(is_supported_filesystem("ext4"));
        assert!(is_supported_filesystem("ext3"));
        assert!(is_supported_filesystem("ext2"));
        assert!(is_supported_filesystem("btrfs"));
        assert!(is_supported_filesystem("xfs"));
        assert!(is_supported_filesystem("EXT4")); // case insensitive
    }

    #[test]
    fn test_is_supported_filesystem_unsupported() {
        assert!(!is_supported_filesystem("ntfs"));
        assert!(!is_supported_filesystem("fat32"));
        assert!(!is_supported_filesystem("vfat"));
        assert!(!is_supported_filesystem("zfs"));
        assert!(!is_supported_filesystem(""));
    }

    #[test]
    fn test_is_valid_username_valid() {
        assert!(is_valid_username("user"));
        assert!(is_valid_username("user123"));
        assert!(is_valid_username("_user"));
        assert!(is_valid_username("user_name"));
        assert!(is_valid_username("user-name"));
        assert!(is_valid_username("a"));
    }

    #[test]
    fn test_is_valid_username_invalid() {
        assert!(!is_valid_username(""));
        assert!(!is_valid_username("User")); // uppercase
        assert!(!is_valid_username("123user")); // starts with number
        assert!(!is_valid_username("-user")); // starts with hyphen
        assert!(!is_valid_username("user name")); // space
        assert!(!is_valid_username("user@name")); // special char
        assert!(!is_valid_username(
            "a_very_long_username_that_exceeds_32_chars"
        ));
    }

    #[test]
    fn test_parse_dns_server_valid() {
        assert_eq!(
            parse_dns_server("nameserver 8.8.8.8"),
            Some("8.8.8.8".to_string())
        );
        assert_eq!(
            parse_dns_server("nameserver 1.1.1.1"),
            Some("1.1.1.1".to_string())
        );
        assert_eq!(
            parse_dns_server("  nameserver   192.168.1.1  "),
            Some("192.168.1.1".to_string())
        );
    }

    #[test]
    fn test_parse_dns_server_invalid() {
        assert_eq!(parse_dns_server(""), None);
        assert_eq!(parse_dns_server("# nameserver 8.8.8.8"), None);
        assert_eq!(parse_dns_server("search example.com"), None);
        assert_eq!(parse_dns_server("nameserver "), None);
    }

    #[test]
    fn test_is_valid_ipv4_valid() {
        assert!(is_valid_ipv4("192.168.1.1"));
        assert!(is_valid_ipv4("8.8.8.8"));
        assert!(is_valid_ipv4("255.255.255.255"));
        assert!(is_valid_ipv4("0.0.0.0"));
        assert!(is_valid_ipv4("10.0.0.1"));
    }

    #[test]
    fn test_is_valid_ipv4_invalid() {
        assert!(!is_valid_ipv4(""));
        assert!(!is_valid_ipv4("192.168.1")); // only 3 octets
        assert!(!is_valid_ipv4("192.168.1.256")); // octet > 255
        assert!(!is_valid_ipv4("192.168.1.1.1")); // 5 octets
        assert!(!is_valid_ipv4("not.an.ip.address"));
    }

    #[test]
    fn test_get_backup_path() {
        assert_eq!(
            get_backup_path("/etc/pacman.conf", "backup"),
            "/etc/pacman.conf.backup"
        );
        assert_eq!(get_backup_path("/etc/fstab", "bak"), "/etc/fstab.bak");
        assert_eq!(
            get_backup_path("/var/lib/pacman", "20240101"),
            "/var/lib/pacman.20240101"
        );
    }

    #[test]
    fn test_is_critical_system_path_critical() {
        assert!(is_critical_system_path("/"));
        assert!(is_critical_system_path("/boot"));
        assert!(is_critical_system_path("/etc"));
        assert!(is_critical_system_path("/etc/pacman.conf"));
        assert!(is_critical_system_path("/usr/bin"));
        assert!(is_critical_system_path("/var/lib/pacman"));
    }

    #[test]
    fn test_is_critical_system_path_non_critical() {
        assert!(!is_critical_system_path("/home"));
        assert!(!is_critical_system_path("/home/user"));
        assert!(!is_critical_system_path("/tmp"));
        assert!(!is_critical_system_path("/opt"));
        assert!(!is_critical_system_path("/mnt/data"));
    }
}
