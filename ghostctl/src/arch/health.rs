use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect, Select};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn health_menu() {
    println!("🏥 System Health & Maintenance");
    println!("==============================");

    let options = [
        "💾 Check disk space",
        "🔍 Check package database",
        "🔨 Rebuild package database",
        "📋 Manage .pacnew/.pacsave files",
        "✓ Check system file integrity",
        "🧹 Clean system",
        "📊 Full health report",
        "📝 Show recent logs",
        "🔧 System diagnostics",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("System Health")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => check_disk_space(),
        1 => check_package_database(),
        2 => rebuild_package_database(),
        3 => manage_pacnew_files(),
        4 => check_system_integrity(),
        5 => clean_system(),
        6 => full_health_report(),
        7 => crate::logging::GhostLogger::show_recent_logs(),
        8 => system_diagnostics(),
        _ => return,
    }
}

pub fn check_disk_space() {
    println!("💾 Checking disk space...\n");

    // Show disk usage
    let _ = Command::new("df")
        .args(["-h", "-x", "tmpfs", "-x", "devtmpfs"])
        .status();

    println!("\n📊 Largest directories in /:");
    let _ = Command::new("du")
        .args([
            "-h",
            "/",
            "-d",
            "1",
            "--exclude=/proc",
            "--exclude=/sys",
            "--exclude=/dev",
            "--exclude=/run",
        ])
        .status();

    // Check if any partition is over 90% full
    let output = Command::new("df").args(["-P"]).output();

    if let Ok(output) = output {
        let content = String::from_utf8_lossy(&output.stdout);
        for line in content.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5
                && let Ok(usage) = parts[4].trim_end_matches('%').parse::<u32>()
                    && usage >= 90 {
                        println!("\n⚠️  WARNING: {} is {}% full!", parts[5], usage);
                    }
        }
    }

    // Offer cleanup options
    println!("\n🧹 Cleanup options:");
    let cleanup_options = [
        "Clean package cache",
        "Clean journal logs",
        "Find large files",
        "Clean build directories",
        "Skip cleanup",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cleanup action")
        .items(&cleanup_options)
        .default(4)
        .interact()
        .unwrap();

    match choice {
        0 => clean_package_cache(),
        1 => clean_journal_logs(),
        2 => find_large_files(),
        3 => clean_build_dirs(),
        _ => {}
    }
}

fn clean_package_cache() {
    println!("🧹 Cleaning package cache...");

    // Show current cache size
    let _ = Command::new("du")
        .args(["-sh", "/var/cache/pacman/pkg"])
        .status();

    // Clean all but last 3 versions
    let _ = Command::new("sudo").args(["paccache", "-r"]).status();

    // Remove uninstalled packages
    let _ = Command::new("sudo").args(["paccache", "-ruk0"]).status();

    println!("✅ Package cache cleaned");
}

fn clean_journal_logs() {
    println!("📰 Cleaning journal logs...");

    // Show current journal size
    let _ = Command::new("journalctl").args(&["--disk-usage"]).status();

    // Keep only last 2 weeks
    let _ = Command::new("sudo")
        .args(&["journalctl", "--vacuum-time=2weeks"])
        .status();

    println!("✅ Journal logs cleaned");
}

fn find_large_files() {
    println!("🔍 Finding large files...");

    println!("\n📦 Largest files in home directory:");
    let _ = Command::new("find")
        .args(&[
            std::env::var("HOME")
                .unwrap_or_else(|_| ".".to_string())
                .as_str(),
            "-type",
            "f",
            "-size",
            "+100M",
            "-exec",
            "ls",
            "-lh",
            "{}",
            ";",
        ])
        .status();

    println!("\n📦 Largest files in /var:");
    let _ = Command::new("sudo")
        .args(&[
            "find", "/var", "-type", "f", "-size", "+100M", "-exec", "ls", "-lh", "{}", ";",
        ])
        .status();
}

fn clean_build_dirs() {
    println!("🏗️  Cleaning build directories...");

    let dirs_to_clean = [
        "/tmp/makepkg-*",
        "/tmp/yaourt-*",
        "/tmp/pamac-build-*",
        "/tmp/yay-*",
        "/tmp/paru-*",
        "/var/tmp/pamac-build-*",
    ];

    for dir in &dirs_to_clean {
        let _ = Command::new("sudo").args(&["rm", "-rf", dir]).status();
    }

    // Clean user cache
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let user_dirs = [
        format!("{}/.cache/yay", home),
        format!("{}/.cache/paru", home),
    ];

    for dir in &user_dirs {
        if Path::new(dir).exists() {
            let clean = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("Clean {}?", dir))
                .interact()
                .unwrap();

            if clean {
                let _ = Command::new("rm").args(&["-rf", dir]).status();
            }
        }
    }

    println!("✅ Build directories cleaned");
}

pub fn check_package_database() {
    println!("🔍 Checking package database integrity...");

    // Check for database errors
    let output = Command::new("pacman").args(&["-Dk"]).output();

    match output {
        Ok(output) => {
            if output.status.success() && output.stdout.is_empty() {
                println!("✅ Package database is healthy");
            } else {
                println!("⚠️  Database issues found:");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
        }
        Err(_) => println!("❌ Failed to check database"),
    }

    // Check for missing files
    println!("\n🔍 Checking for missing files...");
    let _ = Command::new("sudo").args(&["pacman", "-Qk"]).status();
}

pub fn rebuild_package_database() {
    println!("🔨 Rebuilding package database...");

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will rebuild the entire package database. Continue?")
        .interact()
        .unwrap();

    if !confirm {
        return;
    }

    // Backup current database
    println!("💾 Backing up current database...");
    let _ = Command::new("sudo")
        .args(&["cp", "-r", "/var/lib/pacman/", "/var/lib/pacman.backup"])
        .status();

    // Remove sync databases
    println!("🗑️  Removing sync databases...");
    let _ = Command::new("sudo")
        .args(&["rm", "-rf", "/var/lib/pacman/sync/*"])
        .status();

    // Rebuild database
    println!("🔨 Rebuilding database...");
    let _ = Command::new("sudo").args(&["pacman", "-Syy"]).status();

    println!("✅ Database rebuilt successfully");
}

pub fn manage_pacnew_files() {
    println!("📋 Managing .pacnew/.pacsave files...");

    // Find all pacnew/pacsave files
    let output = Command::new("find")
        .args(&["/etc", "-name", "*.pacnew", "-o", "-name", "*.pacsave"])
        .output();

    let files: Vec<String> = match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        Err(_) => Vec::new(),
    };

    if files.is_empty() {
        println!("✅ No .pacnew or .pacsave files found");
        return;
    }

    println!("Found {} config files to review:", files.len());
    for file in &files {
        println!("  • {}", file);
    }

    // Offer to use pacdiff or manual review
    let options = ["Use pacdiff (recommended)", "Manual review", "Skip"];
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("How would you like to handle these?")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            // Check if pacdiff is available
            if Command::new("which").arg("pacdiff").status().is_ok() {
                let _ = Command::new("sudo").args(&["pacdiff"]).status();
            } else {
                println!("⚠️  pacdiff not found. Install pacman-contrib package.");
            }
        }
        1 => {
            for file in files {
                handle_pacnew_file(&file);
            }
        }
        _ => {}
    }
}

fn handle_pacnew_file(file: &str) {
    println!("\n📄 Handling: {}", file);

    let original = file
        .trim_end_matches(".pacnew")
        .trim_end_matches(".pacsave");

    let options = [
        "View diff",
        "Keep current",
        "Use new",
        "Merge manually",
        "Skip",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Action")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            let _ = Command::new("diff").args(&["-u", original, file]).status();
            handle_pacnew_file(file); // Ask again after viewing
        }
        1 => {
            // Keep current, remove pacnew
            let _ = Command::new("sudo").args(&["rm", file]).status();
            println!("✅ Kept current configuration");
        }
        2 => {
            // Use new
            let _ = Command::new("sudo").args(&["mv", file, original]).status();
            println!("✅ Updated to new configuration");
        }
        3 => {
            // Open in editor
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
            let _ = Command::new("sudo")
                .args(&[&editor, original, file])
                .status();
        }
        _ => {}
    }
}

pub fn check_system_integrity() {
    println!("✓ Checking system file integrity...");

    // Check all packages
    println!("🔍 Checking all installed packages (this may take a while)...");

    let output = Command::new("sudo").args(&["pacman", "-Qkk"]).output();

    if let Ok(output) = output {
        let content = String::from_utf8_lossy(&output.stdout);
        let errors: Vec<&str> = content
            .lines()
            .filter(|line| line.contains("warning") || line.contains("error"))
            .collect();

        if errors.is_empty() {
            println!("✅ All system files are intact");
        } else {
            println!("⚠️  Found {} integrity issues:", errors.len());
            for (i, error) in errors.iter().take(10).enumerate() {
                println!("  {}. {}", i + 1, error);
            }
            if errors.len() > 10 {
                println!("  ... and {} more", errors.len() - 10);
            }

            // Offer to reinstall affected packages
            let reinstall = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Reinstall affected packages?")
                .interact()
                .unwrap();

            if reinstall {
                // Extract package names
                let packages: Vec<&str> = errors
                    .iter()
                    .filter_map(|line| line.split(':').next())
                    .collect();

                let _ = Command::new("sudo")
                    .args(&["pacman", "-S", "--noconfirm"])
                    .args(&packages)
                    .status();
            }
        }
    }
}

pub fn clean_system() {
    println!("🧹 System cleanup menu");

    let options = vec![
        "Package cache",
        "Orphaned packages",
        "Old journal logs",
        "Build directories",
        "Broken symlinks",
        "Old config files",
        "Thumbnail cache",
    ];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select items to clean")
        .items(&options)
        .interact()
        .unwrap();

    for &idx in &selections {
        match idx {
            0 => clean_package_cache(),
            1 => {
                println!("🧹 Removing orphaned packages...");
                let _ = Command::new("sudo")
                    .args(&["pacman", "-Rns", "$(pacman -Qtdq)"])
                    .status();
            }
            2 => clean_journal_logs(),
            3 => clean_build_dirs(),
            4 => find_broken_symlinks(),
            5 => clean_old_configs(),
            6 => clean_thumbnail_cache(),
            _ => {}
        }
    }

    println!("✅ System cleanup completed");
}

fn find_broken_symlinks() {
    println!("🔍 Finding broken symlinks...");

    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let _ = Command::new("find")
        .args(&[&home, "-xtype", "l", "-print"])
        .status();

    println!("\n/etc broken symlinks:");
    let _ = Command::new("sudo")
        .args(&["find", "/etc", "-xtype", "l", "-print"])
        .status();
}

fn clean_old_configs() {
    println!("📋 Finding old config files...");

    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let config_dir = format!("{}/.config", home);

    // Find directories in .config that don't have corresponding packages
    if let Ok(entries) = fs::read_dir(&config_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Check if package exists
                let dir_name = path.file_name().unwrap().to_string_lossy();
                let output = Command::new("pacman").args(&["-Qs", &dir_name]).output();

                if let Ok(output) = output
                    && output.stdout.is_empty() {
                        println!("  ? {} (no matching package)", path.display());
                    }
            }
        }
    }
}

fn clean_thumbnail_cache() {
    println!("🖼️  Cleaning thumbnail cache...");

    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let cache_dir = format!("{}/.cache/thumbnails", home);

    if Path::new(&cache_dir).exists() {
        let _ = Command::new("rm").args(&["-rf", &cache_dir]).status();
        println!("✅ Thumbnail cache cleaned");
    }
}

pub fn full_health_report() {
    println!("📊 Generating full system health report...\n");

    println!("=== DISK SPACE ===");
    let _ = Command::new("df")
        .args(["-h", "-x", "tmpfs", "-x", "devtmpfs"])
        .status();

    println!("\n=== FAILED SERVICES ===");
    let _ = Command::new("systemctl").args(&["--failed"]).status();

    println!("\n=== PACKAGE DATABASE ===");
    let _ = Command::new("pacman").args(&["-Dk"]).status();

    println!("\n=== ORPHANED PACKAGES ===");
    let _ = Command::new("pacman").args(&["-Qtd"]).status();

    println!("\n=== PACNEW/PACSAVE FILES ===");
    let _ = Command::new("find")
        .args(&["/etc", "-name", "*.pacnew", "-o", "-name", "*.pacsave"])
        .status();

    println!("\n=== JOURNAL SIZE ===");
    let _ = Command::new("journalctl").args(&["--disk-usage"]).status();

    println!("\n=== LARGEST PACKAGES ===");
    let _ = Command::new("sh")
        .arg("-c")
        .arg("pacman -Qi | awk '/^Name/{name=$3} /^Installed Size/{print $4$5, name}' | sort -h | tail -20")
        .status();

    println!("\n📊 Health report complete");
}

fn system_diagnostics() {
    println!("🔧 System Diagnostics");
    println!("=====================");

    let options = [
        "📝 Execute command with logging",
        "🔒 Safe command execution",
        "📊 Show system information",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Diagnostics Options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            crate::logging::execute_with_logging("System Check", || {
                println!("Running system diagnostics...");
                let _ = Command::new("uname").arg("-a").status();
                Ok(())
            })
            .unwrap_or_else(|e| println!("Error: {}", e));
        }
        1 => {
            crate::logging::safe_command("systemctl", &["status", "docker"], "Check Docker Status")
                .unwrap_or_else(|e| println!("Error: {}", e));
        }
        2 => {
            println!("🔍 System Information:");
            let _ = Command::new("hostnamectl").status();
            let _ = Command::new("uptime").status();
        }
        _ => return,
    }
}
