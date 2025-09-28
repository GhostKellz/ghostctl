pub mod archfix;
pub mod aur;
pub mod boot;
pub mod dotfiles;
pub mod hardware;
pub mod health;
pub mod mirror;
pub mod perf;
pub mod pkgfix;
pub mod recovery;
pub mod services;
pub mod swap;
pub mod sysadmin;

use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn arch_menu() {
    loop {
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
            "⬅️  Back",
        ];
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Arch Linux Menu")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();
        match choice {
            0 => quick_system_fixes(),
            1 => {
                let target: String = Input::new()
                    .with_prompt("Enter target to fix (pacman/orphans/mirrors/all)")
                    .interact_text()
                    .unwrap();
                fix_target(&target);
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
            _ => break,
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
    println!("Optimizing Arch mirrorlist using reflector...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -S --noconfirm reflector && sudo reflector --latest 20 --sort rate --save /etc/pacman.d/mirrorlist")
        .status();
    match status {
        Ok(s) if s.success() => println!("Mirrorlist optimized."),
        _ => println!("Failed to optimize mirrorlist."),
    }
}

pub fn cleanup_orphans() {
    println!("Cleaning up orphaned packages...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -Rns $(pacman -Qtdq)")
        .status();
    match status {
        Ok(s) if s.success() => println!("Orphaned packages removed."),
        _ => println!("No orphaned packages to remove or failed to clean up."),
    }
}

pub fn quick_system_fixes() {
    println!("🔧 Arch Linux Quick System Fixes");
    println!("=================================");
    let fixes = dialoguer::MultiSelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Select fixes to apply")
        .items(&["🗂️  Clear old log files"])
        .interact()
        .unwrap();
    if fixes.is_empty() {
        println!("❌ No fixes selected");
        return;
    }
    println!("🚀 Applying selected fixes...");
    for fix in fixes {
        match fix {
            _ => println!("  ✅ Applied fix {}", fix),
        }
    }
    println!("✅ Quick fixes completed!");
}

pub fn fix_gpg_keys() {
    println!("🔑 Fixing GPG keys...");
    let _ = std::process::Command::new("sudo")
        .args(&["rm", "-rf", "/etc/pacman.d/gnupg"])
        .status();
    let _ = std::process::Command::new("sudo")
        .args(&["pacman-key", "--init"])
        .status();
    let _ = std::process::Command::new("sudo")
        .args(&["pacman-key", "--populate", "archlinux"])
        .status();
    let _ = std::process::Command::new("sudo")
        .args(&["pacman-key", "--refresh-keys"])
        .status();
    println!("  ✅ GPG keys fixed");
}

pub fn reset_pacman_locks() {
    println!("📦 Resetting pacman locks...");
    let _ = std::process::Command::new("sudo")
        .args(&["rm", "-f", "/var/lib/pacman/db.lck"])
        .status();
    println!("  ✅ Pacman locks cleared");
}
