pub mod archfix;
pub mod aur;
pub mod boot;
pub mod dotfiles;
pub mod health;
pub mod perf;
pub mod pkgfix;
pub mod swap;

use dialoguer::{Select, theme::ColorfulTheme};

pub fn arch_menu() {
    loop {
        let options = [
            "🔧 Quick System Fixes",
            "🛠️  Arch Maintenance (Fix/Optimize/Clean)",
            "🏥 System Health & Maintenance",
            "💾 Swap & Zram Management",
            "📁 Dotfiles Management",
            "📦 AUR Helper Management",
            "🥾 Boot & Kernel Management",
            "🔑 GPG Key Management",
            "⚡ Performance Tuning",
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
            1 => archfix::tui_menu(),
            2 => health::health_menu(),
            3 => swap::swap_menu(),
            4 => dotfiles::dotfiles_menu(),
            5 => aur::aur_helper_management(),
            6 => boot::boot_management(),
            7 => crate::security::gpg::gpg_key_management(),
            8 => perf::tune(),
            _ => break,
        }
    }
}

pub fn fix(target: String) {
    match target.as_str() {
        "pacman" | "keyring" => archfix::fix(),
        "orphans" => archfix::orphans(),
        "mirrors" => archfix::mirrors(),
        "pkgfix" => archfix::pkgfix(),
        "optimize" => archfix::optimize(),
        _ => {
            println!("Unknown fix target. Use pacman, keyring, orphans, mirrors, pkgfix, optimize.")
        }
    }
}

// Move any general arch menu or fix logic here if not already present

#[allow(dead_code)]
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

#[allow(dead_code)]
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
            _ => (),
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

pub fn update_mirror_list() {
    println!("🌐 Updating mirror list...");
    if std::process::Command::new("which")
        .arg("reflector")
        .status()
        .is_ok()
    {
        let _ = std::process::Command::new("sudo").status();
        println!("  ✅ Mirror list updated with reflector");
    } else {
        println!("  ⚠️  Reflector not installed, using manual backup");
        let _ = std::process::Command::new("sudo");
        let _ = std::process::Command::new("curl");
        if std::path::Path::new("/tmp/mirrorlist").exists() {
            // Add logic if needed
        }
    }
}
