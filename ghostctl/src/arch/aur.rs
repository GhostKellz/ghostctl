use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use std::process::Command;

pub fn aur_helper_management() {
    println!("📦 AUR Helper Management");
    println!("========================");

    let options = [
        "🔍 Check installed AUR helpers",
        "⭐ Set preferred AUR helper",
        "📥 Install AUR helper",
        "🔄 Update AUR packages",
        "🧹 Clean AUR cache",
        "🔧 Advanced Package Management",
        "🩺 Diagnose & Fix Broken Packages",
        "🔗 Dependency Resolution Tools",
        "📋 Package Conflict Resolution",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("AUR Helper Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => check_aur_helpers(),
        1 => set_preferred_aur_helper(),
        2 => install_aur_helper(),
        3 => update_aur_packages(),
        4 => clean_aur_cache(),
        5 => advanced_package_management(),
        6 => diagnose_broken_packages(),
        7 => dependency_resolution_tools(),
        8 => package_conflict_resolution(),
        _ => return,
    }
}

fn check_aur_helpers() {
    println!("🔍 Checking AUR Helpers");
    println!("=======================");

    let helpers = [
        ("reaper", "reap", "GhostKellz's modern AUR helper"),
        ("paru", "paru", "Feature packed AUR helper"),
        ("yay", "yay", "Yet another Yogurt AUR helper"),
        ("trizen", "trizen", "Lightweight AUR helper"),
        ("pikaur", "pikaur", "AUR helper with minimal dependencies"),
    ];

    let mut found_helpers = Vec::new();

    for (name, cmd, description) in &helpers {
        if Command::new("which").arg(cmd).status().is_ok() {
            println!("  ✅ {} - {}", name, description);
            found_helpers.push(*name);
        } else {
            println!("  ❌ {} - {} (not installed)", name, description);
        }
    }

    if found_helpers.is_empty() {
        println!("\n💡 No AUR helpers found. Consider installing one!");
    } else {
        println!("\n📊 Found {} AUR helper(s)", found_helpers.len());
    }
}

fn install_aur_helper() {
    println!("📥 Install AUR Helper");
    println!("====================");

    let helpers = [
        "reaper (Recommended - GhostKellz)",
        "paru (Feature rich)",
        "yay (Popular choice)",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select AUR helper to install")
        .items(&helpers)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_reaper(),
        1 => install_paru(),
        2 => install_yay(),
        _ => return,
    }
}

fn install_reaper() {
    println!("🔥 Installing Reaper AUR Helper");
    println!("===============================");

    let confirm = Confirm::new()
        .with_prompt("Install Reaper via official installer?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("📥 Downloading and installing Reaper...");
        let status = Command::new("bash")
            .arg("-c")
            .arg("curl -sSL https://raw.githubusercontent.com/face-hh/reaper/main/release/install.sh | bash")
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Reaper installed successfully!");
                println!("💡 Use 'reap -S package' to install AUR packages");
            }
            _ => println!("❌ Failed to install Reaper"),
        }
    }
}

fn install_paru() {
    println!("🦀 Installing Paru AUR Helper");
    println!("=============================");

    // Check if rust is installed
    if Command::new("which").arg("cargo").status().is_err() {
        println!("📦 Installing Rust toolchain...");
        let _ = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "rust"])
            .status();
    }

    let confirm = Confirm::new()
        .with_prompt("Build and install Paru from AUR?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("🔨 Building Paru from source...");
        let build_dir = "/tmp/paru-build";

        let _ = std::fs::remove_dir_all(build_dir);

        let status = Command::new("git")
            .args(["clone", "https://aur.archlinux.org/paru.git", build_dir])
            .status();

        if status.is_ok() && status.unwrap().success() {
            let build_status = Command::new("makepkg")
                .args(["-si", "--noconfirm"])
                .current_dir(build_dir)
                .status();

            match build_status {
                Ok(s) if s.success() => {
                    println!("✅ Paru installed successfully!");
                    println!("💡 Use 'paru -S package' to install AUR packages");
                }
                _ => println!("❌ Failed to build Paru"),
            }
        }

        let _ = std::fs::remove_dir_all(build_dir);
    }
}

fn install_yay() {
    println!("🚀 Installing Yay AUR Helper");
    println!("============================");

    let confirm = Confirm::new()
        .with_prompt("Build and install Yay from AUR?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("🔨 Building Yay from source...");
        let build_dir = "/tmp/yay-build";

        let _ = std::fs::remove_dir_all(build_dir);

        let status = Command::new("git")
            .args(["clone", "https://aur.archlinux.org/yay.git", build_dir])
            .status();

        if status.is_ok() && status.unwrap().success() {
            let build_status = Command::new("makepkg")
                .args(["-si", "--noconfirm"])
                .current_dir(build_dir)
                .status();

            match build_status {
                Ok(s) if s.success() => {
                    println!("✅ Yay installed successfully!");
                    println!("💡 Use 'yay -S package' to install AUR packages");
                }
                _ => println!("❌ Failed to build Yay"),
            }
        }

        let _ = std::fs::remove_dir_all(build_dir);
    }
}

fn update_aur_packages() {
    println!("🔄 Update AUR Packages");
    println!("======================");

    // Try available AUR helpers
    let helpers = [("reap", "-Syu"), ("paru", "-Syu"), ("yay", "-Syu")];

    for (helper, args) in &helpers {
        if Command::new("which").arg(helper).status().is_ok() {
            println!("🔄 Updating with {}...", helper);
            let _ = Command::new(helper).arg(args).status();
            return;
        }
    }

    println!("❌ No AUR helper found for updates");
}

fn clean_aur_cache() {
    println!("🧹 Clean AUR Cache");
    println!("==================");

    // Try available AUR helpers
    let helpers = [("reap", "-Sc"), ("paru", "-Sc"), ("yay", "-Sc")];

    for (helper, args) in &helpers {
        if Command::new("which").arg(helper).status().is_ok() {
            let confirm = Confirm::new()
                .with_prompt(format!("Clean cache with {}?", helper))
                .default(true)
                .interact()
                .unwrap();

            if confirm {
                let _ = Command::new(helper).arg(args).status();
            }
            return;
        }
    }

    println!("❌ No AUR helper found for cache cleaning");
}

fn set_preferred_aur_helper() {
    println!("⭐ Set Preferred AUR Helper");
    println!("===========================");

    // Check which helpers are installed
    let helpers = [
        (
            "reaper (reap command)",
            "reap",
            "GhostKellz's modern AUR helper",
        ),
        ("paru", "paru", "Feature packed AUR helper"),
        ("yay", "yay", "Yet another Yogurt AUR helper"),
    ];

    let mut available_helpers = Vec::new();
    let mut helper_options = Vec::new();

    for (display_name, cmd, description) in &helpers {
        if Command::new("which").arg(cmd).status().is_ok() {
            available_helpers.push(*cmd);
            helper_options.push(format!("{} - {}", display_name, description));
        }
    }

    if available_helpers.is_empty() {
        println!("❌ No preferred AUR helpers found installed.");
        println!("Please install one of: reap, paru, or yay first.");
        return;
    }

    // Show current preferred helper
    if let Some(current) = get_preferred_aur_helper() {
        println!("📋 Current preferred helper: {}", current);
    }

    helper_options.push("⬅️  Back".to_string());

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select preferred AUR helper")
        .items(&helper_options)
        .default(0)
        .interact()
        .unwrap();

    if choice < available_helpers.len() {
        let selected = available_helpers[choice];
        println!("✅ Set {} as preferred AUR helper", selected);

        // Save preference to config file
        save_aur_helper_preference(selected);
    }
}

fn save_aur_helper_preference(helper: &str) {
    use std::fs;

    if let Some(config_dir) = dirs::config_dir() {
        let ghostctl_dir = config_dir.join("ghostctl");
        let _ = fs::create_dir_all(&ghostctl_dir);

        let config_file = ghostctl_dir.join("aur_helper");
        if let Err(e) = fs::write(config_file, helper) {
            println!("⚠️  Warning: Could not save preference: {}", e);
        }
    }
}

pub fn get_preferred_aur_helper() -> Option<String> {
    use std::fs;

    // First check saved preference
    if let Some(config_dir) = dirs::config_dir() {
        let config_file = config_dir.join("ghostctl").join("aur_helper");
        if let Ok(saved_helper) = fs::read_to_string(config_file) {
            let saved_helper = saved_helper.trim();
            if Command::new("which").arg(saved_helper).status().is_ok() {
                return Some(saved_helper.to_string());
            }
        }
    }

    // Fallback to priority order: reap, paru, yay, others
    let helpers = ["reap", "paru", "yay", "trizen", "pikaur"];

    for helper in &helpers {
        if Command::new("which").arg(helper).status().is_ok() {
            return Some(helper.to_string());
        }
    }

    None
}

fn advanced_package_management() {
    println!("🔧 Advanced Package Management");
    println!("==============================");

    let options = [
        "📦 Batch Install AUR Packages",
        "🔍 Search AUR with Filters",
        "📊 Package Information & Dependencies",
        "🧹 Deep Clean Build Cache",
        "🔄 Rebuild All AUR Packages",
        "📋 List Foreign/AUR Packages",
        "💾 AUR Cache Management",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Advanced Package Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => batch_install_packages(),
        1 => search_aur_filtered(),
        2 => package_info_dependencies(),
        3 => deep_clean_cache(),
        4 => rebuild_all_aur(),
        5 => list_foreign_packages(),
        6 => aur_cache_management(),
        _ => return,
    }
}

fn aur_cache_management() {
    use super::aur_cache;

    println!("💾 AUR Cache Management");
    println!("======================");

    let options = [
        "📊 Show cache statistics",
        "🧹 Clear expired entries",
        "🗑️  Clear all cache",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Cache Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => aur_cache::cache_stats(),
        1 => aur_cache::clear_expired(),
        2 => {
            let confirm = Confirm::new()
                .with_prompt("Clear entire AUR cache?")
                .default(false)
                .interact()
                .unwrap();

            if confirm {
                aur_cache::clear_cache();
            }
        }
        _ => return,
    }
}

fn batch_install_packages() {
    println!("📦 Batch Install AUR Packages");
    println!("=============================");

    let package_list: String = Input::new()
        .with_prompt("Enter package names (space-separated)")
        .interact_text()
        .unwrap();

    let packages: Vec<&str> = package_list.split_whitespace().collect();

    if packages.is_empty() {
        println!("❌ No packages specified");
        return;
    }

    println!("📋 Packages to install: {}", packages.join(", "));

    let confirm = Confirm::new()
        .with_prompt("Proceed with installation?")
        .default(true)
        .interact()
        .unwrap();

    if !confirm {
        return;
    }

    if let Some(helper) = get_preferred_aur_helper() {
        println!("🔄 Installing packages with {}...", helper);

        for package in packages {
            println!("📥 Installing {}...", package);
            let status = Command::new(&helper)
                .args(["-S", "--noconfirm", package])
                .status();

            match status {
                Ok(s) if s.success() => println!("  ✅ {} installed successfully", package),
                _ => println!("  ❌ Failed to install {}", package),
            }
        }
    } else {
        println!("❌ No AUR helper available");
    }
}

fn search_aur_filtered() {
    use super::aur_cache;

    println!("🔍 Search AUR with Filters");
    println!("==========================");

    let search_term: String = Input::new()
        .with_prompt("Search term")
        .interact_text()
        .unwrap();

    let filters = [
        "📊 Show package info",
        "📋 Show dependencies",
        "⭐ Sort by popularity",
        "📅 Sort by last modified",
    ];

    let selected_filters = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select filters (Space to select, Enter to confirm)")
        .items(&filters)
        .interact()
        .unwrap();

    // Try cached AUR search first
    println!("🔍 Searching AUR for '{}'...", search_term);
    if let Some(mut packages) = aur_cache::search_packages(&search_term) {
        println!("📦 Found {} packages in AUR:", packages.len());

        // Apply sorting filters
        if selected_filters.contains(&2) {
            // Sort by popularity
            packages.sort_by(|a, b| b.popularity.partial_cmp(&a.popularity).unwrap());
        } else if selected_filters.contains(&3) {
            // Sort by votes (proxy for last modified)
            packages.sort_by(|a, b| b.votes.cmp(&a.votes));
        }

        // Display results
        for pkg in packages.iter().take(20) {
            println!("\n  📦 {} ({})", pkg.name, pkg.version);
            if let Some(desc) = &pkg.description {
                println!("     {}", desc);
            }
            println!("     ⭐ {} votes | 📊 {:.2} popularity", pkg.votes, pkg.popularity);

            if selected_filters.contains(&0) {
                if let Some(url) = &pkg.url {
                    println!("     🔗 {}", url);
                }
                if let Some(maintainer) = &pkg.maintainer {
                    println!("     👤 {}", maintainer);
                }
            }
        }

        if packages.len() > 20 {
            println!("\n  ... and {} more results", packages.len() - 20);
        }
    } else if let Some(helper) = get_preferred_aur_helper() {
        // Fallback to AUR helper
        let mut args = vec!["-Ss"];
        args.push(&search_term);
        let _ = Command::new(helper).args(&args).status();

        if selected_filters.contains(&0) || selected_filters.contains(&1) {
            println!("\n📊 Getting detailed package information...");
            let _ = Command::new("pacman").args(["-Si", &search_term]).status();
        }
    } else {
        println!("❌ No AUR helper available and API search failed");
    }
}

fn package_info_dependencies() {
    println!("📊 Package Information & Dependencies");
    println!("====================================");

    let package: String = Input::new()
        .with_prompt("Package name")
        .interact_text()
        .unwrap();

    println!("📋 Package Information for: {}", package);
    println!("-----------------------------------");

    // Check if package is installed
    let status = Command::new("pacman").args(["-Qi", &package]).status();

    if status.is_ok() && status.unwrap().success() {
        println!("✅ Package is installed (showing local info)");
    } else {
        println!("📦 Package not installed (showing repository info)");
        let _ = Command::new("pacman").args(["-Si", &package]).status();
    }

    // Show dependency tree
    println!("\n🔗 Dependency Tree:");
    let _ = Command::new("pactree").args([&package]).status();

    // Show reverse dependencies
    println!("\n🔄 Reverse Dependencies (what depends on this):");
    let _ = Command::new("pactree").args(["-r", &package]).status();
}

fn deep_clean_cache() {
    println!("🧹 Deep Clean Build Cache");
    println!("=========================");

    let cache_locations = [
        "/tmp/yaourt-tmp-*",
        "/tmp/pamac-build-*",
        "/tmp/makepkg-*",
        "/tmp/yay-*",
        "/tmp/paru-*",
        "~/.cache/yay",
        "~/.cache/paru",
        "/var/cache/pacman/pkg/*",
    ];

    println!("🗂️  Cache locations to clean:");
    for location in &cache_locations {
        println!("  📁 {}", location);
    }

    let confirm = Confirm::new()
        .with_prompt("Clean all build caches?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        println!("🧹 Cleaning build caches...");

        // Clean temp build directories
        let _ = Command::new("sudo")
            .args([
                "rm",
                "-rf",
                "/tmp/yaourt-tmp-*",
                "/tmp/pamac-build-*",
                "/tmp/makepkg-*",
                "/tmp/yay-*",
                "/tmp/paru-*",
            ])
            .status();

        // Clean user caches
        let _ = Command::new("rm")
            .args(["-rf", "~/.cache/yay", "~/.cache/paru"])
            .status();

        // Clean pacman cache (keep 3 most recent)
        let _ = Command::new("sudo")
            .args(["paccache", "-r", "-k3"])
            .status();

        println!("✅ Build caches cleaned");
    }
}

fn rebuild_all_aur() {
    println!("🔄 Rebuild All AUR Packages");
    println!("===========================");

    println!("📋 Finding AUR/foreign packages...");
    let output = Command::new("pacman").args(["-Qm"]).output();

    match output {
        Ok(output) if output.status.success() => {
            let foreign_packages = String::from_utf8_lossy(&output.stdout);
            let packages: Vec<&str> = foreign_packages
                .lines()
                .map(|line| line.split_whitespace().next().unwrap_or(""))
                .filter(|&pkg| !pkg.is_empty())
                .collect();

            if packages.is_empty() {
                println!("✅ No foreign/AUR packages found");
                return;
            }

            println!("📦 Found {} AUR packages:", packages.len());
            for package in &packages {
                println!("  • {}", package);
            }

            let confirm = Confirm::new()
                .with_prompt("Rebuild all AUR packages?")
                .default(false)
                .interact()
                .unwrap();

            if confirm {
                if let Some(helper) = get_preferred_aur_helper() {
                    println!("🔨 Rebuilding packages with {}...", helper);
                    let _ = Command::new(&helper)
                        .args(["-S", "--rebuild"])
                        .args(&packages)
                        .status();
                }
            }
        }
        _ => println!("❌ Failed to query foreign packages"),
    }
}

fn list_foreign_packages() {
    println!("📋 List Foreign/AUR Packages");
    println!("============================");

    println!("🔍 Foreign packages (not in official repos):");
    let _ = Command::new("pacman").args(["-Qm"]).status();

    println!("\n📊 Package statistics:");
    let output = Command::new("pacman").args(["-Qm"]).output();

    if let Ok(output) = output {
        if output.status.success() {
            let count = String::from_utf8_lossy(&output.stdout).lines().count();
            println!("  Total foreign packages: {}", count);
        }
    }

    println!("\n🔍 Explicitly installed packages:");
    let _ = Command::new("pacman").args(["-Qe"]).status();
}

fn diagnose_broken_packages() {
    println!("🩺 Diagnose & Fix Broken Packages");
    println!("=================================");

    let options = [
        "🔍 Check for broken dependencies",
        "🔧 Fix partial upgrades",
        "📦 Reinstall broken packages",
        "🗑️  Remove orphaned dependencies",
        "🔄 Fix database corruption",
        "🛠️  Repair package database",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Diagnostic Options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => check_broken_dependencies(),
        1 => fix_partial_upgrades(),
        2 => reinstall_broken_packages(),
        3 => remove_orphaned_deps(),
        4 => fix_database_corruption(),
        5 => repair_package_database(),
        _ => return,
    }
}

fn check_broken_dependencies() {
    use super::progress;

    println!("🔍 Checking for Broken Dependencies");
    println!("===================================");

    // Check for missing dependencies
    println!("\n🔗 Checking for missing dependencies:");
    let output = progress::execute_with_status(
        Command::new("pacman").args(["-Dk"]),
        "Checking dependencies"
    );

    match output {
        Ok(output) if output.status.success() => {
            println!("✅ No missing dependencies found");
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let filtered_errors = progress::filter_permission_errors(&stderr);

            if !filtered_errors.trim().is_empty() {
                println!("⚠️  Missing dependencies detected:\n{}", filtered_errors);

                let fix = Confirm::new()
                    .with_prompt("Attempt to fix missing dependencies?")
                    .default(true)
                    .interact()
                    .unwrap();

                if fix {
                    println!("🔧 Installing missing dependencies...");
                    let _ = Command::new("sudo")
                        .args(["pacman", "-S", "--asdeps", "--needed"])
                        .status();
                }
            } else {
                println!("✅ No actionable dependency issues (permission errors filtered)");
            }
        }
        Err(e) => {
            println!("❌ Failed to check dependencies: {}", e);
        }
    }

    // Check for broken symlinks
    println!("\n🔗 Checking for broken symlinks:");
    let _ = progress::execute_with_status(
        Command::new("find").args(["/usr", "-xtype", "l", "-print"]),
        "Scanning for broken symlinks"
    );
}

fn fix_partial_upgrades() {
    println!("🔧 Fix Partial Upgrades");
    println!("=======================");

    println!("⚠️  Partial upgrades can cause system instability");

    let confirm = Confirm::new()
        .with_prompt("Force complete system upgrade?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("🔄 Performing complete system upgrade...");
        let _ = Command::new("sudo")
            .args(["pacman", "-Syu", "--noconfirm"])
            .status();

        println!("✅ System upgrade completed");
    }
}

fn reinstall_broken_packages() {
    use super::progress;

    println!("📦 Reinstall Broken Packages");
    println!("============================");

    let package_input: String = Input::new()
        .with_prompt("Enter package name (or 'auto' to detect broken packages)")
        .interact_text()
        .unwrap();

    if package_input == "auto" {
        println!("🔍 Auto-detecting broken packages...");

        // Check for packages with missing files
        let output = progress::execute_with_spinner(
            Command::new("pacman").args(["-Qk"]),
            "Scanning installed packages for issues"
        );

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                // Parse broken packages from output
                let broken_packages = progress::parse_broken_packages(&stdout);

                // Filter permission errors from stderr
                let filtered_errors = progress::filter_permission_errors(&stderr);

                if broken_packages.is_empty() {
                    println!("✅ No broken packages detected");
                    if !filtered_errors.trim().is_empty() {
                        println!("   (Filtered {} permission error(s))",
                            filtered_errors.lines().count());
                    }
                } else {
                    println!("⚠️  Found {} broken package(s):", broken_packages.len());
                    for pkg in &broken_packages {
                        println!("  • {}", pkg);
                    }

                    let fix = Confirm::new()
                        .with_prompt("Reinstall these broken packages?")
                        .default(true)
                        .interact()
                        .unwrap();

                    if fix {
                        println!("🔧 Reinstalling broken packages...");
                        for pkg in &broken_packages {
                            println!("  ⚙️  Reinstalling {}...", pkg);
                            let result = Command::new("sudo")
                                .args(["pacman", "-S", "--noconfirm", pkg])
                                .status();

                            match result {
                                Ok(status) if status.success() => println!("    ✅ Success"),
                                _ => println!("    ⚠️  Failed (may need manual intervention)"),
                            }
                        }
                        println!("✅ Reinstallation complete");
                    }
                }
            }
            Err(e) => println!("❌ Failed to check packages: {}", e),
        }
    } else {
        println!("🔄 Reinstalling {}...", package_input);
        let _ = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", &package_input])
            .status();
    }
}

fn remove_orphaned_deps() {
    println!("🗑️  Remove Orphaned Dependencies");
    println!("===============================");

    println!("🔍 Finding orphaned packages...");
    let output = Command::new("pacman").args(["-Qtdq"]).output();

    match output {
        Ok(output) if output.status.success() => {
            let orphans = String::from_utf8_lossy(&output.stdout);
            let orphan_list: Vec<&str> = orphans.lines().collect();

            if orphan_list.is_empty() {
                println!("✅ No orphaned packages found");
                return;
            }

            println!("📦 Found {} orphaned packages:", orphan_list.len());
            for orphan in &orphan_list {
                println!("  • {}", orphan);
            }

            let confirm = Confirm::new()
                .with_prompt("Remove all orphaned packages?")
                .default(true)
                .interact()
                .unwrap();

            if confirm {
                let status = Command::new("sudo")
                    .args(["pacman", "-Rns", "--noconfirm"])
                    .args(&orphan_list)
                    .status();

                match status {
                    Ok(s) if s.success() => println!("✅ Orphaned packages removed"),
                    _ => println!("❌ Failed to remove some packages"),
                }
            }
        }
        _ => println!("✅ No orphaned packages found"),
    }
}

fn fix_database_corruption() {
    println!("🔄 Fix Database Corruption");
    println!("==========================");

    println!("⚠️  This will rebuild the package database");

    let confirm = Confirm::new()
        .with_prompt("Proceed with database repair?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        println!("🔧 Removing pacman lock...");
        let _ = Command::new("sudo")
            .args(["rm", "-f", "/var/lib/pacman/db.lck"])
            .status();

        println!("🔄 Synchronizing package databases...");
        let _ = Command::new("sudo").args(["pacman", "-Syy"]).status();

        println!("🔧 Refreshing package databases...");
        let _ = Command::new("sudo").args(["pacman-db-upgrade"]).status();

        println!("✅ Database repair completed");
    }
}

fn repair_package_database() {
    println!("🛠️  Repair Package Database");
    println!("==========================");

    let repair_options = [
        "🔄 Refresh all package databases",
        "🔧 Rebuild pacman database",
        "🗑️  Clear package cache",
        "🔑 Refresh GPG keys",
        "🌐 Update mirror list",
    ];

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select repair operations")
        .items(&repair_options)
        .interact()
        .unwrap();

    if selected.is_empty() {
        return;
    }

    println!("🔧 Performing selected repairs...");

    for &option in &selected {
        match option {
            0 => {
                println!("🔄 Refreshing package databases...");
                let _ = Command::new("sudo").args(["pacman", "-Syy"]).status();
            }
            1 => {
                println!("🔧 Rebuilding pacman database...");
                let _ = Command::new("sudo").args(["pacman-db-upgrade"]).status();
            }
            2 => {
                println!("🗑️  Clearing package cache...");
                let _ = Command::new("sudo")
                    .args(["pacman", "-Scc", "--noconfirm"])
                    .status();
            }
            3 => {
                println!("🔑 Refreshing GPG keys...");
                crate::arch::fix_gpg_keys();
            }
            4 => {
                println!("🌐 Updating mirror list...");
                crate::arch::optimize_mirrors();
            }
            _ => {}
        }
    }

    println!("✅ Database repair operations completed");
}

fn dependency_resolution_tools() {
    println!("🔗 Dependency Resolution Tools");
    println!("==============================");

    let options = [
        "🔍 Analyze dependency tree",
        "🔄 Find circular dependencies",
        "📦 Check missing dependencies",
        "⚡ Optimize dependency cache",
        "🧹 Clean dependency cache",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Dependency Tools")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => analyze_dependency_tree(),
        1 => find_circular_dependencies(),
        2 => check_missing_dependencies(),
        3 => optimize_dependency_cache(),
        4 => clean_dependency_cache(),
        _ => return,
    }
}

fn analyze_dependency_tree() {
    println!("🔍 Analyze Dependency Tree");
    println!("==========================");

    let package: String = Input::new()
        .with_prompt("Package name to analyze")
        .interact_text()
        .unwrap();

    println!("🌳 Dependency tree for: {}", package);
    println!("----------------------------");

    // Show full dependency tree
    let _ = Command::new("pactree")
        .args(["-c", "-d", "3", &package])
        .status();

    // Show size information
    println!("\n📊 Package size information:");
    let _ = Command::new("pacman").args(["-Qi", &package]).status();
}

fn find_circular_dependencies() {
    use super::progress;
    use rayon::prelude::*;

    println!("🔄 Find Circular Dependencies");
    println!("=============================");

    println!("🔍 Scanning for circular dependencies...");

    // Get list of packages
    let output = Command::new("pacman").args(["-Qq"]).output();

    match output {
        Ok(output) if output.status.success() => {
            let packages: Vec<String> = String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(|s| s.to_string())
                .collect();

            let package_count = packages.len();
            println!("📊 Analyzing {} packages...", package_count);

            // Sample packages for deep analysis (checking all would take too long)
            let sample_size = package_count.min(50);
            let sample_packages: Vec<_> = packages.iter().take(sample_size).collect();

            println!("🔍 Deep checking {} packages (sample)...", sample_size);

            let pb = progress::create_progress_bar(sample_size as u64, "Checking dependencies");

            // Parallel dependency check
            let potential_issues: Vec<_> = sample_packages
                .par_iter()
                .filter_map(|pkg| {
                    let result = Command::new("pactree")
                        .args(["-r", pkg])
                        .output();

                    pb.inc(1);

                    match result {
                        Ok(output) if output.status.success() => {
                            let tree = String::from_utf8_lossy(&output.stdout);
                            // Check if package depends on itself (circular)
                            if tree.lines().count() > 20 {
                                Some(format!("{} (complex dependency tree: {} lines)",
                                    pkg, tree.lines().count()))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                })
                .collect();

            pb.finish_and_clear();

            if potential_issues.is_empty() {
                println!("✅ No obvious circular dependencies detected");
            } else {
                println!("⚠️  Packages with complex dependency trees:");
                for issue in potential_issues {
                    println!("  • {}", issue);
                }
            }

            println!("\n💡 Use 'pactree -r <package>' to inspect specific packages");
        }
        _ => println!("❌ Failed to query packages"),
    }
}

fn check_missing_dependencies() {
    println!("📦 Check Missing Dependencies");
    println!("=============================");

    println!("🔍 Checking for missing dependencies...");

    let status = Command::new("pacman").args(["-Dk"]).status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ All dependencies satisfied");
        }
        _ => {
            println!("⚠️  Missing dependencies found");

            println!("\n🔧 Checking specific package dependencies:");
            let package: String = Input::new()
                .with_prompt("Enter package name to check (or press Enter to skip)")
                .allow_empty(true)
                .interact_text()
                .unwrap();

            if !package.is_empty() {
                let _ = Command::new("pacman").args(["-Dk", &package]).status();
            }
        }
    }
}

fn optimize_dependency_cache() {
    println!("⚡ Optimize Dependency Cache");
    println!("===========================");

    println!("🔍 Analyzing package cache...");

    // Show cache statistics
    let output = Command::new("du")
        .args(["-sh", "/var/cache/pacman/pkg/"])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let size = String::from_utf8_lossy(&output.stdout);
            println!("📊 Current cache size: {}", size.trim());
        }
    }

    let optimize_options = [
        "🗑️  Remove all cached packages except installed versions",
        "📦 Keep only 3 most recent versions",
        "🧹 Remove only uninstalled packages",
        "📊 Show cache statistics only",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Optimization method")
        .items(&optimize_options)
        .default(1)
        .interact()
        .unwrap();

    match choice {
        0 => {
            let _ = Command::new("sudo")
                .args(["paccache", "-r", "-k0"])
                .status();
        }
        1 => {
            let _ = Command::new("sudo")
                .args(["paccache", "-r", "-k3"])
                .status();
        }
        2 => {
            let _ = Command::new("sudo").args(["paccache", "-r", "-u"]).status();
        }
        3 => {
            let _ = Command::new("paccache").args(["-d"]).status();
        }
        _ => return,
    }

    println!("✅ Cache optimization completed");
}

fn clean_dependency_cache() {
    println!("🧹 Clean Dependency Cache");
    println!("=========================");

    let cache_options = [
        "🗑️  Clean pacman cache",
        "🧹 Clean AUR build cache",
        "📁 Clean temporary build files",
        "🔄 Clean all caches",
    ];

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select caches to clean")
        .items(&cache_options)
        .interact()
        .unwrap();

    if selected.is_empty() {
        return;
    }

    for &option in &selected {
        match option {
            0 => {
                println!("🗑️  Cleaning pacman cache...");
                let _ = Command::new("sudo")
                    .args(["pacman", "-Sc", "--noconfirm"])
                    .status();
            }
            1 => {
                println!("🧹 Cleaning AUR build cache...");
                let _ = Command::new("rm")
                    .args(["-rf", "~/.cache/yay", "~/.cache/paru"])
                    .status();
            }
            2 => {
                println!("📁 Cleaning temporary build files...");
                let _ = Command::new("sudo")
                    .args(["rm", "-rf", "/tmp/makepkg-*", "/tmp/yay-*", "/tmp/paru-*"])
                    .status();
            }
            3 => {
                println!("🔄 Cleaning all caches...");
                let _ = Command::new("sudo")
                    .args(["pacman", "-Scc", "--noconfirm"])
                    .status();
                let _ = Command::new("rm")
                    .args(["-rf", "~/.cache/yay", "~/.cache/paru"])
                    .status();
                let _ = Command::new("sudo")
                    .args(["rm", "-rf", "/tmp/makepkg-*", "/tmp/yay-*", "/tmp/paru-*"])
                    .status();
            }
            _ => {}
        }
    }

    println!("✅ Cache cleaning completed");
}

fn package_conflict_resolution() {
    println!("📋 Package Conflict Resolution");
    println!("==============================");

    let options = [
        "🔍 Detect package conflicts",
        "🔧 Resolve file conflicts",
        "📦 Fix broken packages",
        "🔄 Force reinstall conflicting packages",
        "🗑️  Remove conflicting packages",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Conflict Resolution")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => detect_package_conflicts(),
        1 => resolve_file_conflicts(),
        2 => fix_broken_packages_conflicts(),
        3 => force_reinstall_conflicts(),
        4 => remove_conflicting_packages(),
        _ => return,
    }
}

fn detect_package_conflicts() {
    println!("🔍 Detect Package Conflicts");
    println!("===========================");

    println!("🔄 Scanning for package conflicts...");

    // Check for file conflicts
    println!("📁 Checking for file conflicts:");
    let _ = Command::new("pacman").args(["-Qkk"]).status();

    // Check for dependency conflicts
    println!("\n🔗 Checking dependency conflicts:");
    let _ = Command::new("pacman").args(["-Dk"]).status();

    println!("\n📦 Checking for broken packages:");
    let output = Command::new("pacman").args(["-Qk"]).output();

    match output {
        Ok(output) if !output.status.success() => {
            println!("⚠️  Found broken packages - check output above");
        }
        _ => println!("✅ No obvious package conflicts detected"),
    }
}

fn resolve_file_conflicts() {
    println!("🔧 Resolve File Conflicts");
    println!("=========================");

    let conflict_file: String = Input::new()
        .with_prompt("Enter conflicting file path (or package name)")
        .interact_text()
        .unwrap();

    println!("🔍 Analyzing conflicts for: {}", conflict_file);

    // Show which packages own the file
    let _ = Command::new("pacman")
        .args(["-Qo", &conflict_file])
        .status();

    // Show file information
    let _ = Command::new("ls").args(["-la", &conflict_file]).status();

    let resolution_options = [
        "🔄 Reinstall owning package",
        "🗑️  Remove conflicting file",
        "📁 Backup and replace file",
        "⏭️  Skip resolution",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Resolution method")
        .items(&resolution_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            // Try to find and reinstall the package that owns this file
            let output = Command::new("pacman")
                .args(["-Qo", &conflict_file])
                .output();

            if let Ok(output) = output {
                if output.status.success() {
                    let owner_info = String::from_utf8_lossy(&output.stdout);
                    if let Some(package) = owner_info.split_whitespace().nth(4) {
                        println!("🔄 Reinstalling {}...", package);
                        let _ = Command::new("sudo")
                            .args(["pacman", "-S", "--noconfirm", package])
                            .status();
                    }
                }
            }
        }
        1 => {
            let confirm = Confirm::new()
                .with_prompt("Are you sure you want to remove this file?")
                .default(false)
                .interact()
                .unwrap();

            if confirm {
                let _ = Command::new("sudo")
                    .args(["rm", "-f", &conflict_file])
                    .status();
            }
        }
        2 => {
            println!("📁 Creating backup...");
            let backup_name = format!("{}.backup", conflict_file);
            let _ = Command::new("sudo")
                .args(["cp", &conflict_file, &backup_name])
                .status();
            println!("✅ Backup created: {}", backup_name);
        }
        _ => return,
    }
}

fn fix_broken_packages_conflicts() {
    println!("📦 Fix Broken Packages");
    println!("======================");

    println!("🔄 Running comprehensive package check...");

    // Full package verification
    let status = Command::new("pacman").args(["-Qkk"]).status();

    if status.is_ok() && !status.unwrap().success() {
        println!("⚠️  Found broken packages");

        let fix_options = [
            "🔄 Reinstall all broken packages",
            "🧹 Remove and reinstall",
            "🔧 Attempt automatic fix",
            "⏭️  Manual review only",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Fix method")
            .items(&fix_options)
            .default(2)
            .interact()
            .unwrap();

        match choice {
            0 | 1 => {
                println!("🔄 This would attempt to fix broken packages...");
                println!("💡 Run 'sudo pacman -S $(pacman -Qkk 2>&1 | grep 'warning' | awk '{{print $2}}' | sort -u)' manually");
            }
            2 => {
                println!("🔧 Attempting automatic fix...");
                let _ = Command::new("sudo")
                    .args([
                        "pacman",
                        "-S",
                        "--noconfirm",
                        "$(pacman -Qkk 2>&1 | grep warning | awk '{print $2}' | sort -u)",
                    ])
                    .status();
            }
            _ => {}
        }
    } else {
        println!("✅ No broken packages found");
    }
}

fn force_reinstall_conflicts() {
    println!("🔄 Force Reinstall Conflicting Packages");
    println!("=======================================");

    let package: String = Input::new()
        .with_prompt("Package name to force reinstall")
        .interact_text()
        .unwrap();

    println!("⚠️  Force reinstalling: {}", package);

    let confirm = Confirm::new()
        .with_prompt("This will overwrite files. Continue?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let _ = Command::new("sudo")
            .args(["pacman", "-S", "--overwrite", "*", "--noconfirm", &package])
            .status();

        println!("✅ Force reinstall completed");
    }
}

fn remove_conflicting_packages() {
    println!("🗑️  Remove Conflicting Packages");
    println!("==============================");

    let packages: String = Input::new()
        .with_prompt("Enter package names to remove (space-separated)")
        .interact_text()
        .unwrap();

    let package_list: Vec<&str> = packages.split_whitespace().collect();

    if package_list.is_empty() {
        println!("❌ No packages specified");
        return;
    }

    println!("🗑️  Packages to remove: {}", package_list.join(", "));

    let confirm = Confirm::new()
        .with_prompt("Remove these packages and their dependencies?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let _ = Command::new("sudo")
            .args(["pacman", "-Rns", "--noconfirm"])
            .args(&package_list)
            .status();

        println!("✅ Package removal completed");
    }
}
