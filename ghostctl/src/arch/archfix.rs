use super::diagnostics::SystemDiagnostics;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

#[allow(dead_code)]
pub fn fix_pacman() {
    println!("ghostctl :: Arch Pacman Fix");

    // Later: run commands using `duct` or `std::process::Command`
    println!("- sudo pacman -Syyu");
    println!("- sudo pacman -S archlinux-keyring");
    println!("- Optionally refresh mirrorlist with reflector");
}

pub fn fix() {
    println!("ghostctl :: Arch System Fix with Auto-Detection");
    println!("==============================================\n");

    // Run diagnostics
    let diag = SystemDiagnostics::scan();
    diag.print_summary();

    if !diag.has_issues() {
        println!("🎉 System is healthy! Running standard upgrade...");
        upgrade_system();
        return;
    }

    // Get fix sequence
    let actions = diag.get_fix_sequence();

    println!("🔧 Applying fixes in optimal order...\n");

    for action in actions {
        println!("▶ {}", action.description());
        action.execute();
        println!();
    }

    // After fixes, run upgrade
    println!("🚀 Running system upgrade...");
    upgrade_system();

    println!("\n✅ System fix complete!");
}

fn upgrade_system() {
    let status = std::process::Command::new("sudo")
        .args(&["pacman", "-Syyu", "--noconfirm"])
        .status();
    match status {
        Ok(s) if s.success() => println!("  ✅ System fully upgraded"),
        _ => println!("  ⚠️  Upgrade had issues"),
    }

    // Refresh keyring after upgrade
    let status = std::process::Command::new("sudo")
        .args(&["pacman", "-S", "--noconfirm", "archlinux-keyring"])
        .status();
    match status {
        Ok(s) if s.success() => println!("  ✅ Keyring refreshed"),
        _ => {}
    }
}

pub fn mirrors() {
    println!("ghostctl :: Arch Mirror Optimization");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("command -v reflector && sudo reflector --latest 20 --sort rate --save /etc/pacman.d/mirrorlist || echo 'reflector not installed'")
        .status();
    match status {
        Ok(s) if s.success() => println!("Mirrorlist refreshed (if reflector installed)."),
        _ => println!("Could not refresh mirrorlist (reflector missing?)."),
    }
}

pub fn orphans() {
    println!("ghostctl :: Arch Orphan Cleanup");
    println!("================================");

    // First, list orphaned packages
    let output = std::process::Command::new("pacman")
        .args(&["-Qtdq"])
        .output();

    match output {
        Ok(out) => {
            let orphans = String::from_utf8_lossy(&out.stdout);
            let orphan_list: Vec<&str> = orphans.lines().filter(|l| !l.is_empty()).collect();

            if orphan_list.is_empty() {
                println!("  ✅ No orphaned packages found");
                return;
            }

            println!("  📦 Found {} orphaned package(s):", orphan_list.len());
            for pkg in &orphan_list {
                println!("     - {}", pkg);
            }
            println!();

            let confirm = dialoguer::Confirm::new()
                .with_prompt(format!("Remove {} orphaned package(s)?", orphan_list.len()))
                .default(false)
                .interact()
                .unwrap_or(false);

            if !confirm {
                println!("  ❌ Orphan cleanup cancelled");
                return;
            }

            println!("  🗑️  Removing orphaned packages...");
            let status = std::process::Command::new("sudo")
                .args(&["pacman", "-Rns", "--noconfirm"])
                .args(&orphan_list)
                .status();

            match status {
                Ok(s) if s.success() => println!("  ✅ Orphaned packages removed"),
                _ => println!("  ❌ Failed to remove some orphaned packages"),
            }
        }
        Err(_) => {
            println!("  ❌ Failed to query orphaned packages");
        }
    }
}

pub fn pkgfix() {
    println!("ghostctl :: Arch PKGBUILD/Build Environment Fix");
    println!("================================================");

    // Find build directories that match known patterns
    let patterns = [
        "/tmp/yaourt-tmp-*",
        "/tmp/pamac-build-*",
        "/tmp/makepkg-*",
        "/tmp/yay-*",
        "/tmp/paru-*",
        "/tmp/pikaur-*",
    ];

    let mut found_dirs: Vec<String> = Vec::new();

    for pattern in &patterns {
        // Use glob to find matching directories
        let base_pattern = pattern.trim_start_matches("/tmp/").trim_end_matches("*");
        if let Ok(entries) = std::fs::read_dir("/tmp") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with(base_pattern) && entry.path().is_dir() {
                    found_dirs.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
    }

    if found_dirs.is_empty() {
        println!("  ✅ No build directories found to clean");
        return;
    }

    // Calculate total size
    let mut total_size: u64 = 0;
    for dir in &found_dirs {
        if let Ok(output) = std::process::Command::new("du")
            .args(&["-sb", dir])
            .output()
            && let Ok(size_str) = String::from_utf8(output.stdout)
            && let Some(size) = size_str.split_whitespace().next()
        {
            total_size += size.parse::<u64>().unwrap_or(0);
        }
    }

    println!("  🗂️  Found {} build director(ies):", found_dirs.len());
    for dir in &found_dirs {
        println!("     - {}", dir);
    }
    println!("  📊 Total size: {} MB", total_size / 1024 / 1024);
    println!();

    let confirm = dialoguer::Confirm::new()
        .with_prompt(format!("Remove {} build director(ies)?", found_dirs.len()))
        .default(true)
        .interact()
        .unwrap_or(false);

    if !confirm {
        println!("  ❌ Cleanup cancelled");
        return;
    }

    println!("  🧹 Cleaning build directories...");
    let mut cleaned = 0;
    for dir in &found_dirs {
        let result = std::process::Command::new("rm")
            .args(&["-rf", dir])
            .status();

        match result {
            Ok(s) if s.success() => {
                cleaned += 1;
            }
            _ => {
                println!("  ⚠️  Failed to remove: {}", dir);
            }
        }
    }

    println!("  ✅ Cleaned {} build director(ies)", cleaned);
}

pub fn keyring() {
    println!("ghostctl :: Arch Keyring Refresh");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -S --noconfirm archlinux-keyring")
        .status();
    match status {
        Ok(s) if s.success() => println!("Keyring refreshed."),
        _ => println!("Failed to refresh keyring."),
    }
}

pub fn optimize() {
    println!("ghostctl :: Arch System Optimization");
    // Enable zram
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo systemctl enable --now systemd-zram-setup@zram0.service")
        .status();
    match status {
        Ok(s) if s.success() => println!("zram enabled."),
        _ => println!("Failed to enable zram."),
    }
    // Enable zswap (if kernel supports)
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("echo 1 | sudo tee /sys/module/zswap/parameters/enabled")
        .status();
    match status {
        Ok(s) if s.success() => println!("zswap enabled (if supported)."),
        _ => println!("Failed to enable zswap (may not be supported)."),
    }
}

pub fn full() {
    println!("ghostctl :: Full Arch Maintenance");
    fix();
    keyring();
    mirrors();
    orphans();
    optimize();
}

pub fn tui_menu() {
    let opts = [
        "System Fix (Upgrade, Keyring, Mirrors)",
        "Keyring Refresh",
        "Mirror Optimization",
        "Orphan Cleanup",
        "PKGBUILD/Build Env Fix",
        "Performance Optimize (zram/zswap)",
        "Full Maintenance",
        "Install Ghost Tools",
        "Back",
    ];
    match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Arch Maintenance")
        .items(&opts)
        .default(0)
        .interact()
        .unwrap()
    {
        0 => fix(),
        1 => keyring(),
        2 => mirrors(),
        3 => orphans(),
        4 => pkgfix(),
        5 => optimize(),
        6 => full(),
        _ => return,
    }
}
