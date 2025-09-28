use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use std::fs;
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Recovery Tools")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Emergency Repair")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select fixes to apply")
        .items(&fixes)
        .interact()
        .unwrap();

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
                let _ = Command::new("sudo")
                    .args(["bash", "-c", "echo 'nameserver 8.8.8.8' > /etc/resolv.conf"])
                    .status();
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Bootloader Recovery")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("GRUB Recovery")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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
            let device: String = Input::new()
                .with_prompt("Enter device (e.g., /dev/sda)")
                .interact_text()
                .unwrap();

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
    let attempt = Confirm::new()
        .with_prompt("Attempt automated GRUB rescue?")
        .default(true)
        .interact()
        .unwrap();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Systemd-boot Recovery")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

        let regenerate = Confirm::new()
            .with_prompt("Remove all entries and regenerate?")
            .default(false)
            .interact()
            .unwrap();

        if regenerate {
            let _ = Command::new("sudo")
                .args(["rm", "-f", "/boot/loader/entries/*"])
                .status();

            // Create basic entry
            create_basic_boot_entry();
        }
    } else {
        println!("❌ Boot entries directory not found");
        let create = Confirm::new()
            .with_prompt("Create boot entries directory?")
            .default(true)
            .interact()
            .unwrap();

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

    // Detect installed kernels
    let output = Command::new("ls").args(["/boot/vmlinuz-*"]).output();

    if let Ok(output) = output {
        let kernels = String::from_utf8_lossy(&output.stdout);
        for kernel in kernels.lines() {
            if !kernel.is_empty() {
                let kernel_name = kernel.replace("/boot/vmlinuz-", "");
                let entry_content = format!(
                    "title   Arch Linux ({})\nlinux   /vmlinuz-{}\ninitrd  /initramfs-{}.img\noptions root=PARTUUID=$(blkid -s PARTUUID -o value /dev/$(findmnt -n -o SOURCE /) | head -1) rw\n",
                    kernel_name, kernel_name, kernel_name
                );

                let entry_file = format!("/boot/loader/entries/arch-{}.conf", kernel_name);
                let _ = Command::new("sudo")
                    .args([
                        "bash",
                        "-c",
                        &format!("echo '{}' > '{}'", entry_content, entry_file),
                    ])
                    .status();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Filesystem Repair")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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
    let device: String = Input::new()
        .with_prompt("Enter device to check (e.g., /dev/sda1, or 'all' for all)")
        .interact_text()
        .unwrap();

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

    let device: String = Input::new()
        .with_prompt("Enter device to repair (e.g., /dev/sda1)")
        .interact_text()
        .unwrap();

    let confirm = Confirm::new()
        .with_prompt("Proceed with filesystem repair? (This may cause data loss)")
        .default(false)
        .interact()
        .unwrap();

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
        Input::new()
            .with_prompt("Enter filesystem type (ext4, btrfs, xfs)")
            .interact_text()
            .unwrap()
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
    let detailed = Confirm::new()
        .with_prompt("Show detailed SMART information?")
        .default(false)
        .interact()
        .unwrap();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Emergency Recovery")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let device: String = Input::new()
        .with_prompt("Enter device to force check")
        .interact_text()
        .unwrap();

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

    let tool: String = Input::new()
        .with_prompt("Enter tool to install/run (or 'skip')")
        .interact_text()
        .unwrap();

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

    let device: String = Input::new()
        .with_prompt("Enter ext4 device to reset journal")
        .interact_text()
        .unwrap();

    let confirm = Confirm::new()
        .with_prompt("Reset filesystem journal? (ext4 only)")
        .default(false)
        .interact()
        .unwrap();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("User Recovery")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let username: String = Input::new()
        .with_prompt("Enter username to reset password")
        .interact_text()
        .unwrap();

    println!("🔑 Resetting password for {}...", username);
    let _ = Command::new("sudo").args(["passwd", &username]).status();
}

fn create_emergency_user() {
    println!("👤 Create Emergency User");

    let username: String = Input::new()
        .with_prompt("Enter emergency username")
        .with_initial_text("rescue")
        .interact_text()
        .unwrap();

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

    let username: String = Input::new()
        .with_prompt("Enter username to fix permissions")
        .interact_text()
        .unwrap();

    println!("🔧 Fixing permissions for {}...", username);

    // Fix home directory ownership
    let home_dir = format!("/home/{}", username);
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

    let username: String = Input::new()
        .with_prompt("Enter username to recover home directory")
        .interact_text()
        .unwrap();

    let home_dir = format!("/home/{}", username);

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

    let username: String = Input::new()
        .with_prompt("Enter username to add to wheel group")
        .interact_text()
        .unwrap();

    println!("🔐 Adding {} to wheel group...", username);
    let _ = Command::new("sudo")
        .args(["usermod", "-aG", "wheel", &username])
        .status();

    // Ensure wheel group has sudo access
    println!("🔧 Ensuring wheel group has sudo access...");
    let _ = Command::new("sudo")
        .args([
            "bash",
            "-c",
            "echo '%wheel ALL=(ALL:ALL) ALL' >> /etc/sudoers",
        ])
        .status();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Database Recovery")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let confirm = Confirm::new()
        .with_prompt("Rebuild entire package database?")
        .default(true)
        .interact()
        .unwrap();

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
        let confirm = Confirm::new()
            .with_prompt("Restore from /var/lib/pacman.backup?")
            .default(true)
            .interact()
            .unwrap();

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
            .unwrap()
            .as_secs()
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Network Recovery")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let confirm = Confirm::new()
        .with_prompt("Reset network configuration to defaults?")
        .default(true)
        .interact()
        .unwrap();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("WiFi Fix")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select DNS server")
        .items(&dns_servers)
        .default(0)
        .interact()
        .unwrap();

    let selected_dns = dns_servers[choice];

    println!("🔧 Setting DNS to {}", selected_dns);
    let _ = Command::new("sudo")
        .args([
            "bash",
            "-c",
            &format!("echo 'nameserver {}' > /etc/resolv.conf", selected_dns),
        ])
        .status();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Display Recovery")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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
    let generate = Confirm::new()
        .with_prompt("Generate new xorg.conf?")
        .default(false)
        .interact()
        .unwrap();

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

    let reinstall = Confirm::new()
        .with_prompt("Reinstall graphics drivers?")
        .default(false)
        .interact()
        .unwrap();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Rollback Tools")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

        let restore = Confirm::new()
            .with_prompt("Open Timeshift for restore?")
            .default(false)
            .interact()
            .unwrap();

        if restore {
            let _ = Command::new("sudo").args(["timeshift-gtk"]).status();
        }
    } else {
        println!("❌ Timeshift not installed");
        let install = Confirm::new()
            .with_prompt("Install Timeshift?")
            .default(true)
            .interact()
            .unwrap();

        if install {
            let _ = Command::new("sudo")
                .args(["pacman", "-S", "--noconfirm", "timeshift"])
                .status();
        }
    }
}

fn package_downgrade() {
    println!("📦 Package Downgrade");

    let package: String = Input::new()
        .with_prompt("Enter package name to downgrade")
        .interact_text()
        .unwrap();

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

        let device: String = Input::new()
            .with_prompt("Enter EFI partition (e.g., /dev/sda1)")
            .interact_text()
            .unwrap();

        let mount_point: String = Input::new()
            .with_prompt("Mount point")
            .with_initial_text("/boot")
            .interact_text()
            .unwrap();

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
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bootloader")
        .items(&bootloaders)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("📦 Installing GRUB...");
            let device: String = Input::new()
                .with_prompt("Enter device (e.g., /dev/sda)")
                .interact_text()
                .unwrap();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Mount Operations")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            let device: String = Input::new()
                .with_prompt("Device to mount (e.g., /dev/sda1)")
                .interact_text()
                .unwrap();
            let mount_point: String = Input::new()
                .with_prompt("Mount point (e.g., /mnt)")
                .interact_text()
                .unwrap();

            let _ = Command::new("sudo")
                .args(["mkdir", "-p", &mount_point])
                .status();
            let _ = Command::new("sudo")
                .args(["mount", &device, &mount_point])
                .status();
            println!("✅ Mounted {} to {}", device, mount_point);
        }
        1 => {
            let mount_point: String = Input::new()
                .with_prompt("Mount point to unmount")
                .interact_text()
                .unwrap();
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

    let show_devices = Confirm::new()
        .with_prompt("Show available devices?")
        .default(true)
        .interact()
        .unwrap();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Advanced Repair")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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
            let device: String = Input::new()
                .with_prompt("Enter device for deep scan")
                .interact_text()
                .unwrap();
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Boot Repair")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("🔄 Regenerating GRUB configuration...");
            let _ = Command::new("sudo")
                .args(["grub-mkconfig", "-o", "/boot/grub/grub.cfg"])
                .status();
        }
        1 => {
            println!("🔧 Reinstalling GRUB...");
            let device: String = Input::new()
                .with_prompt("Enter device to install GRUB to (e.g. /dev/sda)")
                .interact_text()
                .unwrap();
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Package Repair")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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
            let package: String = Input::new()
                .with_prompt("Enter package name to downgrade")
                .interact_text()
                .unwrap();
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Configuration Reset")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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
    let username: String = Input::new()
        .with_prompt("Enter username to reset permissions")
        .interact_text()
        .unwrap();

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
