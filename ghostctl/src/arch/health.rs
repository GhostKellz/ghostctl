use dialoguer::{Confirm, MultiSelect, Select, theme::ColorfulTheme};
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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("System Health")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

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
                && usage >= 90
            {
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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cleanup action")
        .items(&cleanup_options)
        .default(4)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

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
            let clean = match Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("Clean {}?", dir))
                .interact_opt()
            {
                Ok(Some(c)) => c,
                _ => return,
            };

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

    let confirm = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will rebuild the entire package database. Continue?")
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

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
    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("How would you like to handle these?")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Action")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

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
            let reinstall = match Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Reinstall affected packages?")
                .interact_opt()
            {
                Ok(Some(c)) => c,
                _ => return,
            };

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

    let selections = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select items to clean")
        .items(&options)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

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
                let dir_name = match path.file_name() {
                    Some(n) => n.to_string_lossy(),
                    None => continue,
                };
                let output = Command::new("pacman").args(&["-Qs", &dir_name]).output();

                if let Ok(output) = output
                    && output.stdout.is_empty()
                {
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
    // Get package info and parse sizes in Rust instead of shell pipeline
    let output = Command::new("pacman").args(["-Qi"]).output();
    if let Ok(output) = output {
        let content = String::from_utf8_lossy(&output.stdout);
        let mut packages: Vec<(String, f64)> = Vec::new();
        let mut current_name = String::new();

        for line in content.lines() {
            if line.starts_with("Name") {
                if let Some(name) = line.split(':').nth(1) {
                    current_name = name.trim().to_string();
                }
            } else if line.starts_with("Installed Size") {
                if let Some(size_str) = line.split(':').nth(1) {
                    let size_str = size_str.trim();
                    // Parse size like "123.45 MiB" or "1.23 GiB"
                    let parts: Vec<&str> = size_str.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(num) = parts[0].parse::<f64>() {
                            let multiplier = match parts[1] {
                                "KiB" => 1.0 / 1024.0,
                                "MiB" => 1.0,
                                "GiB" => 1024.0,
                                _ => 1.0,
                            };
                            packages.push((current_name.clone(), num * multiplier));
                        }
                    }
                }
            }
        }

        // Sort by size descending and show top 20
        packages.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        for (name, size) in packages.iter().take(20) {
            if *size >= 1024.0 {
                println!("  {:.2} GiB  {}", size / 1024.0, name);
            } else {
                println!("  {:.2} MiB  {}", size, name);
            }
        }
    }

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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Diagnostics Options")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

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

/// Parse disk usage percentage from df output line
/// Returns (mountpoint, percentage) if successfully parsed
pub fn parse_disk_usage_line(line: &str) -> Option<(String, u32)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 6 {
        if let Ok(usage) = parts[4].trim_end_matches('%').parse::<u32>() {
            return Some((parts[5].to_string(), usage));
        }
    }
    None
}

/// Check if a partition is critically full (>= threshold%)
pub fn is_partition_critical(usage_percent: u32, threshold: u32) -> bool {
    usage_percent >= threshold
}

/// Parse pacnew/pacsave file path to get original config path
pub fn get_original_config_path(pacnew_path: &str) -> String {
    pacnew_path
        .trim_end_matches(".pacnew")
        .trim_end_matches(".pacsave")
        .to_string()
}

/// Validate that a path looks like a valid config file path
pub fn is_valid_config_path(path: &str) -> bool {
    !path.is_empty()
        && (path.starts_with("/etc/") || path.starts_with("/usr/"))
        && !path.contains("..")
}

/// Parse integrity check output to extract warnings/errors
pub fn parse_integrity_errors(output: &str) -> Vec<String> {
    output
        .lines()
        .filter(|line| line.contains("warning") || line.contains("error"))
        .map(|s| s.to_string())
        .collect()
}

/// Extract package name from integrity error line
pub fn extract_package_from_error(error_line: &str) -> Option<String> {
    error_line
        .split(':')
        .next()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_disk_usage_line_valid() {
        let line = "/dev/sda1       100G   80G   20G  80% /";
        let result = parse_disk_usage_line(line);
        let Some((mountpoint, usage)) = result else {
            panic!("Expected Some result for valid disk usage line");
        };
        assert_eq!(mountpoint, "/");
        assert_eq!(usage, 80);
    }

    #[test]
    fn test_parse_disk_usage_line_high_usage() {
        let line = "/dev/nvme0n1p2  500G  475G   25G  95% /home";
        let result = parse_disk_usage_line(line);
        let Some((mountpoint, usage)) = result else {
            panic!("Expected Some result for high usage disk line");
        };
        assert_eq!(mountpoint, "/home");
        assert_eq!(usage, 95);
    }

    #[test]
    fn test_parse_disk_usage_line_full() {
        let line = "/dev/sdb1       50G   50G     0 100% /mnt/data";
        let result = parse_disk_usage_line(line);
        let Some((_, usage)) = result else {
            panic!("Expected Some result for full disk line");
        };
        assert_eq!(usage, 100);
    }

    #[test]
    fn test_parse_disk_usage_line_invalid() {
        let line = "Filesystem      Size  Used Avail Use% Mounted on";
        let result = parse_disk_usage_line(line);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_disk_usage_line_empty() {
        let result = parse_disk_usage_line("");
        assert!(result.is_none());
    }

    #[test]
    fn test_is_partition_critical_above_threshold() {
        assert!(is_partition_critical(95, 90));
        assert!(is_partition_critical(100, 90));
        assert!(is_partition_critical(90, 90));
    }

    #[test]
    fn test_is_partition_critical_below_threshold() {
        assert!(!is_partition_critical(89, 90));
        assert!(!is_partition_critical(50, 90));
        assert!(!is_partition_critical(0, 90));
    }

    #[test]
    fn test_get_original_config_path_pacnew() {
        let path = "/etc/pacman.conf.pacnew";
        assert_eq!(get_original_config_path(path), "/etc/pacman.conf");
    }

    #[test]
    fn test_get_original_config_path_pacsave() {
        let path = "/etc/fstab.pacsave";
        assert_eq!(get_original_config_path(path), "/etc/fstab");
    }

    #[test]
    fn test_get_original_config_path_no_extension() {
        let path = "/etc/hosts";
        assert_eq!(get_original_config_path(path), "/etc/hosts");
    }

    #[test]
    fn test_is_valid_config_path_etc() {
        assert!(is_valid_config_path("/etc/pacman.conf"));
        assert!(is_valid_config_path("/etc/fstab"));
        assert!(is_valid_config_path("/etc/systemd/system.conf"));
    }

    #[test]
    fn test_is_valid_config_path_usr() {
        assert!(is_valid_config_path("/usr/lib/systemd/system/foo.service"));
        assert!(is_valid_config_path("/usr/share/config/app.conf"));
    }

    #[test]
    fn test_is_valid_config_path_invalid() {
        assert!(!is_valid_config_path(""));
        assert!(!is_valid_config_path("/home/user/.config"));
        assert!(!is_valid_config_path("/etc/../home/user"));
        assert!(!is_valid_config_path("relative/path"));
    }

    #[test]
    fn test_parse_integrity_errors_with_warnings() {
        let output =
            "package1: all files ok\npackage2: warning: file missing\npackage3: all files ok";
        let errors = parse_integrity_errors(output);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("warning"));
    }

    #[test]
    fn test_parse_integrity_errors_with_errors() {
        let output = "error: could not read package\nwarning: file modified";
        let errors = parse_integrity_errors(output);
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_parse_integrity_errors_clean() {
        let output = "package1: all files ok\npackage2: all files ok";
        let errors = parse_integrity_errors(output);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_extract_package_from_error_valid() {
        let error = "linux: warning: some file missing";
        let pkg = extract_package_from_error(error);
        assert_eq!(pkg, Some("linux".to_string()));
    }

    #[test]
    fn test_extract_package_from_error_with_spaces() {
        let error = "  nvidia-dkms  : error: file corrupted";
        let pkg = extract_package_from_error(error);
        assert_eq!(pkg, Some("nvidia-dkms".to_string()));
    }

    #[test]
    fn test_extract_package_from_error_empty() {
        let error = ": error without package name";
        let pkg = extract_package_from_error(error);
        assert!(pkg.is_none());
    }
}
