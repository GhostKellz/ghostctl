pub mod archfix;
pub mod aur;
pub mod aur_cache;
pub mod boot;
pub mod diagnostics;
pub mod dotfiles;
pub mod hardware;
pub mod health;
pub mod mirror;
pub mod perf;
pub mod pkgfix;
pub mod progress;
pub mod recovery;
pub mod services;
pub mod swap;
pub mod sysadmin;

use crate::tui;
use crate::utils::{is_headless, sudo_rm, sudo_run, sudo_run_interactive};

pub fn arch_menu() {
    // Check for headless mode
    if is_headless() {
        tui::warn("Arch menu cannot be displayed in headless mode. Use CLI subcommands instead.");
        tui::info("Example: ghostctl arch fix pacman");
        return;
    }

    let options = [
        "🔧 Quick System Fixes",
        "🎯 Fix Specific Target",
        "🛠️  Arch Maintenance (Fix/Optimize/Clean)",
        "🚨 Emergency Recovery & Rescue Tools",
        "🏥 System Health & Maintenance",
        "⚙️  SystemD Service Management",
        "🖥️  Hardware Detection & Drivers",
        "💾 Swap & Zram Management",
        "📁 Dotfiles Management",
        "📦 AUR Helper Management",
        "📋 PKGBUILD Management & Validation",
        "🌐 Mirror List Management",
        "🥾 Boot & Kernel Management",
        "🔑 GPG Key Management",
        "⚡ Performance Tuning",
        "🛠️  Advanced System Administration",
    ];

    while let Some(choice) = tui::select_with_back("Arch Linux Menu", &options, 0) {
        match choice {
            0 => quick_system_fixes(),
            1 => {
                if let Some(target) =
                    tui::input("Enter target to fix (pacman/orphans/mirrors/all)", None)
                    && !target.is_empty()
                {
                    fix_target(&target);
                }
            }
            2 => archfix::tui_menu(),
            3 => recovery::recovery_menu(),
            4 => health::health_menu(),
            5 => services::systemd_service_management(),
            6 => hardware::hardware_management(),
            7 => swap::swap_menu(),
            8 => dotfiles::dotfiles_menu(),
            9 => aur::aur_helper_management(),
            10 => pkgfix::pkgbuild_management(),
            11 => mirror::update_mirror_list(),
            12 => boot::boot_management(),
            13 => crate::security::gpg::gpg_key_management(),
            14 => perf::tune(),
            15 => sysadmin::sysadmin_menu(),
            _ => {}
        }
    }
}

pub fn fix_target(target: &str) {
    match target {
        "pacman" | "keyring" => archfix::fix(),
        "orphans" => archfix::orphans(),
        "mirrors" => archfix::mirrors(),
        "pkgfix" => pkgfix::pkgbuild_management(),
        "optimize" => archfix::optimize(),
        "gpg" => fix_gpg_keys(),
        "locks" => reset_pacman_locks(),
        "all" => {
            reset_pacman_locks();
            fix_gpg_keys();
            mirror::update_mirror_list();
            archfix::fix();
        }
        _ => {
            println!("❌ Unknown fix target: {}", target);
            println!("📋 Available targets:");
            println!("  pacman   - Fix pacman database issues");
            println!("  keyring  - Fix keyring issues");
            println!("  orphans  - Remove orphaned packages");
            println!("  mirrors  - Update mirror list");
            println!("  pkgfix   - Fix PKGBUILD issues");
            println!("  optimize - Optimize system");
            println!("  gpg      - Fix GPG keys");
            println!("  locks    - Clear pacman locks");
            println!("  all      - Apply all fixes");
        }
    }
}

// Helper functions now integrated into main commands
pub fn optimize_mirrors() {
    tui::status("🔄", "Optimizing Arch mirrorlist using reflector...");

    // Check if reflector is installed
    let reflector_installed = std::process::Command::new("which")
        .arg("reflector")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !reflector_installed {
        tui::info("Installing reflector...");
        match sudo_run_interactive("pacman", &["-S", "--noconfirm", "reflector"]) {
            Ok(result) if result.success => tui::success("Reflector installed"),
            _ => {
                tui::error("Failed to install reflector");
                return;
            }
        }
    }

    // Run reflector
    match sudo_run(
        "reflector",
        &[
            "--latest",
            "20",
            "--sort",
            "rate",
            "--save",
            "/etc/pacman.d/mirrorlist",
        ],
    ) {
        Ok(result) if result.success => tui::success("Mirrorlist optimized"),
        Ok(result) => {
            tui::error("Failed to optimize mirrorlist");
            if !result.stderr.is_empty() {
                eprintln!("{}", result.stderr);
            }
        }
        Err(e) => tui::error(&format!("Failed to run reflector: {}", e)),
    }
}

pub fn cleanup_orphans() {
    tui::status("🧹", "Checking for orphaned packages...");

    // First list orphans
    let orphan_check = std::process::Command::new("pacman")
        .args(&["-Qtdq"])
        .output();

    match orphan_check {
        Ok(output) if output.status.success() && !output.stdout.is_empty() => {
            let orphans = String::from_utf8_lossy(&output.stdout);
            let count = orphans.lines().count();
            tui::info(&format!("Found {} orphaned package(s):", count));
            for pkg in orphans.lines() {
                println!("  - {}", pkg);
            }

            if !tui::confirm("Remove these orphaned packages?", false) {
                tui::info("Cleanup cancelled");
                return;
            }

            match sudo_run_interactive("pacman", &["-Rns", "--noconfirm", "$(pacman -Qtdq)"]) {
                Ok(result) if result.success => tui::success("Orphaned packages removed"),
                _ => tui::warn("Some packages may not have been removed"),
            }
        }
        _ => tui::success("No orphaned packages found"),
    }
}

pub fn quick_system_fixes() {
    tui::header("Arch Linux Quick System Fixes");

    let fix_options = [
        "🗂️  Clear old log files (journalctl --vacuum-time=7d)",
        "📦 Clear package cache (keep last 2 versions)",
        "🧹 Remove orphaned packages",
        "🔄 Update mirrorlist",
    ];

    let selected = tui::multi_select("Select fixes to apply", &fix_options, None);

    if selected.is_empty() {
        tui::info("No fixes selected");
        return;
    }

    tui::status("🚀", "Applying selected fixes...");

    for fix_idx in selected {
        match fix_idx {
            0 => {
                // Clear old logs
                tui::status("🗂️", "Clearing old journal logs...");
                match sudo_run("journalctl", &["--vacuum-time=7d"]) {
                    Ok(result) if result.success => tui::success("Journal logs cleaned"),
                    _ => tui::warn("Failed to clean journal logs"),
                }
            }
            1 => {
                // Clear package cache
                tui::status("📦", "Clearing package cache...");
                match sudo_run("paccache", &["-rk2"]) {
                    Ok(result) if result.success => tui::success("Package cache cleaned"),
                    _ => {
                        // paccache not installed, try alternative
                        tui::warn("paccache not found, installing pacman-contrib...");
                        let _ = sudo_run_interactive(
                            "pacman",
                            &["-S", "--noconfirm", "pacman-contrib"],
                        );
                        match sudo_run("paccache", &["-rk2"]) {
                            Ok(result) if result.success => tui::success("Package cache cleaned"),
                            _ => tui::error("Failed to clean package cache"),
                        }
                    }
                }
            }
            2 => {
                // Remove orphans
                cleanup_orphans();
            }
            3 => {
                // Update mirrorlist
                optimize_mirrors();
            }
            _ => {}
        }
    }

    tui::success("Quick fixes completed!");
}

pub fn fix_gpg_keys() {
    tui::header("Fixing GPG/Pacman Keys");

    // Step 1: Try refreshing keys first (least destructive)
    tui::status("📡", "Attempting to refresh keys from keyserver...");
    if let Ok(result) = sudo_run("pacman-key", &["--refresh-keys"])
        && result.success
    {
        tui::success("Keys refreshed successfully from keyserver");
        return;
    }
    tui::warn("Key refresh failed, trying keyring reinstall...");

    // Step 2: Try reinstalling archlinux-keyring
    tui::status("📦", "Reinstalling archlinux-keyring...");
    if let Ok(result) = sudo_run_interactive("pacman", &["-S", "--noconfirm", "archlinux-keyring"])
        && result.success
    {
        tui::success("Keyring reinstalled successfully");
        // Try refresh again after reinstall
        let _ = sudo_run("pacman-key", &["--populate", "archlinux"]);
        return;
    }
    tui::warn("Keyring reinstall failed, full reset required...");

    // Step 3: Full keyring reset (most destructive - requires confirmation)
    if !tui::confirm_dangerous(
        "Full keyring reset required. This will backup and wipe /etc/pacman.d/gnupg. Continue?",
    ) {
        tui::info("Keyring reset cancelled");
        return;
    }

    // Create backup before wiping
    let backup_path = "/etc/pacman.d/gnupg.backup";
    tui::status("💾", &format!("Creating backup at {}...", backup_path));
    if let Ok(result) = sudo_run("cp", &["-a", "/etc/pacman.d/gnupg", backup_path]) {
        if result.success {
            tui::success("Backup created successfully");
        } else {
            tui::warn("Backup may have failed, but continuing...");
        }
    }

    // Remove the keyring
    tui::status("🗑️", "Removing old keyring...");
    let _ = sudo_rm("/etc/pacman.d/gnupg");

    // Initialize new keyring
    tui::status("🔧", "Initializing new keyring...");
    if let Ok(result) = sudo_run("pacman-key", &["--init"])
        && !result.success
    {
        tui::error("Failed to initialize keyring");
        tui::info(&format!(
            "You can restore from backup: sudo cp -a {} /etc/pacman.d/gnupg",
            backup_path
        ));
        return;
    }

    // Populate keyring
    tui::status("📥", "Populating keyring...");
    if let Ok(result) = sudo_run("pacman-key", &["--populate", "archlinux"]) {
        if result.success {
            tui::success("Keyring populated successfully");
        } else {
            tui::warn("Keyring population may have failed");
        }
    }

    // Final refresh attempt
    tui::status("📡", "Final key refresh...");
    let _ = sudo_run("pacman-key", &["--refresh-keys"]);

    tui::success("GPG keyring reset complete");
    tui::info(&format!("Backup available at: {}", backup_path));
}

pub fn reset_pacman_locks() {
    tui::header("Pacman Lock Management");

    // Check if lock file exists
    let lock_path = std::path::Path::new("/var/lib/pacman/db.lck");
    if !lock_path.exists() {
        tui::success("No pacman lock file found - nothing to do");
        return;
    }

    // Check if pacman is actually running
    let pacman_running = std::process::Command::new("pgrep")
        .arg("-x")
        .arg("pacman")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if pacman_running {
        tui::warn("Pacman appears to be running!");
        tui::info(
            "Wait for the current pacman process to finish, or check with: ps aux | grep pacman",
        );

        if !tui::confirm_dangerous("Force remove lock anyway? (may corrupt database)") {
            tui::info("Lock removal cancelled");
            return;
        }
    } else {
        // Pacman not running, but lock exists - likely stale
        tui::info("Stale lock file detected (pacman not running)");

        if !tui::confirm("Remove stale pacman lock?", true) {
            tui::info("Lock removal cancelled");
            return;
        }
    }

    match sudo_rm("/var/lib/pacman/db.lck") {
        Ok(()) => tui::success("Pacman lock cleared"),
        Err(e) => {
            tui::error(&format!("Failed to remove lock file: {}", e));
            tui::info("Try manually: sudo rm -f /var/lib/pacman/db.lck");
        }
    }
}
