use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

// Cache for commonly accessed paths and values
static HOME_DIR: OnceLock<String> = OnceLock::new();
static GAMES_DIR: OnceLock<String> = OnceLock::new();
static USER_NAME: OnceLock<String> = OnceLock::new();

fn get_home_dir() -> &'static str {
    HOME_DIR.get_or_init(|| std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string()))
}

fn get_games_dir() -> &'static str {
    GAMES_DIR.get_or_init(|| format!("{}/Games", get_home_dir()))
}

fn get_user_name() -> &'static str {
    USER_NAME.get_or_init(|| std::env::var("USER").unwrap_or_else(|_| "user".to_string()))
}

pub fn game_management_menu() {
    loop {
        let options = [
            "🎮 Game Library Management",
            "🍷 Wine/Proton Cleanup & Repair",
            "📦 Bottles Management",
            "🎯 Lutris Management",
            "🚀 Steam/Proton Management",
            "🔧 System Optimization Profiles",
            "🩺 Gaming System Health Check",
            "🧹 Deep Cleanup & Reset",
            "⬅️ Back",
        ];

        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🎮 Game Management & Optimization")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match choice {
            0 => game_library_management(),
            1 => wine_proton_cleanup(),
            2 => bottles_management(),
            3 => lutris_management(),
            4 => steam_proton_management(),
            5 => optimization_profiles(),
            6 => gaming_health_check(),
            7 => deep_cleanup(),
            _ => break,
        }
    }
}

fn game_library_management() {
    let options = [
        "📋 Scan Game Libraries",
        "🔍 Find Duplicate Games",
        "📊 Storage Usage Analysis",
        "🏷️ Game Categorization",
        "🔗 Symbolic Link Management",
        "📝 Game Database Export",
        "⬅️ Back",
    ];

    loop {
        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🎮 Game Library Management")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match choice {
            0 => scan_game_libraries(),
            1 => find_duplicate_games(),
            2 => storage_usage_analysis(),
            3 => game_categorization(),
            4 => symlink_management(),
            5 => game_database_export(),
            _ => break,
        }
    }
}

fn scan_game_libraries() {
    println!("📋 Scanning Game Libraries");
    println!("===========================\n");

    let mut common_paths = Vec::with_capacity(6);
    let home = get_home_dir();
    common_paths.extend_from_slice(&[
        ("Steam", format!("{}/.steam/steam/steamapps/common", home)),
        ("Lutris", format!("{}/.local/share/lutris/prefixes", home)),
        ("Bottles", format!("{}/.local/share/bottles/bottles", home)),
        ("Wine", format!("{}/.wine/drive_c/Program Files", home)),
        (
            "Wine x86",
            format!("{}/.wine/drive_c/Program Files (x86)", home),
        ),
        ("Games", get_games_dir().to_string()),
    ]);

    let mut total_games = 0;
    let mut total_size = 0u64;

    for (platform, path) in &common_paths {
        println!("🔍 Scanning {}: {}", platform, path);

        if Path::new(path).exists() {
            let scan_result = Command::new("find")
                .args(&[path, "-maxdepth", "2", "-type", "d"])
                .output();

            match scan_result {
                Ok(out) => {
                    let output_string = String::from_utf8_lossy(&out.stdout);
                    let dirs: Vec<&str> = output_string.lines().collect();
                    let game_count = dirs.len().saturating_sub(1); // Subtract root directory

                    if game_count > 0 {
                        println!("  ✅ Found {} games/prefixes", game_count);
                        total_games += game_count;

                        // Calculate size
                        if let Ok(size_out) = Command::new("du").args(&["-sb", path]).output()
                            && let Ok(size_str) = String::from_utf8(size_out.stdout)
                            && let Some(size_part) = size_str.split_whitespace().next()
                            && let Ok(size) = size_part.parse::<u64>()
                        {
                            total_size += size;
                            println!("  📊 Size: {} GB", size / 1024 / 1024 / 1024);
                        }

                        // Show top 5 largest games/prefixes
                        println!("  📂 Largest items:");
                        if let Ok(du_out) = Command::new("du")
                            .args(&["-h", "--max-depth=1", path])
                            .output()
                        {
                            let du_output_string = String::from_utf8_lossy(&du_out.stdout);
                            let mut entries: Vec<&str> = du_output_string.lines().collect();
                            if entries.len() > 1 {
                                entries.sort_by(|a, b| {
                                    let a_size = a.split_whitespace().next().unwrap_or("0");
                                    let b_size = b.split_whitespace().next().unwrap_or("0");
                                    b_size.cmp(a_size)
                                });

                                for entry in entries.iter().skip(1).take(5) {
                                    println!("    {}", entry);
                                }
                            }
                        }
                    } else {
                        println!("  📭 No games found");
                    }
                }
                _ => println!("  ❌ Cannot access directory"),
            }
        } else {
            println!("  📭 Directory does not exist");
        }

        println!();
    }

    println!("📊 Summary:");
    println!("  Total Games/Prefixes: {}", total_games);
    println!(
        "  Total Storage Used: {} GB",
        total_size / 1024 / 1024 / 1024
    );

    // Custom directory scan
    let Ok(custom_scan) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Scan custom directory?")
        .default(false)
        .interact()
    else {
        return;
    };

    if custom_scan {
        let Ok(custom_path) = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter custom path to scan")
            .interact()
        else {
            return;
        };

        if Path::new(&custom_path).exists() {
            println!("\n🔍 Scanning custom path: {}", custom_path);

            let find_result = Command::new("find")
                .args(&[
                    &custom_path,
                    "-name",
                    "*.exe",
                    "-o",
                    "-name",
                    "*.msi",
                    "-o",
                    "-name",
                    "setup*",
                ])
                .output();

            if let Ok(out) = find_result {
                let output_string = String::from_utf8_lossy(&out.stdout);
                let executables: Vec<&str> = output_string.lines().collect();
                println!(
                    "  🎮 Found {} potential game executables",
                    executables.len()
                );

                for (i, exe) in executables.iter().take(10).enumerate() {
                    println!("    {}. {}", i + 1, exe);
                }

                if executables.len() > 10 {
                    println!("    ... and {} more", executables.len() - 10);
                }
            }
        }
    }
}

fn find_duplicate_games() {
    println!("🔍 Finding Duplicate Games");
    println!("===========================\n");

    let Ok(search_method) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select duplicate detection method")
        .items(&[
            "📝 By name similarity",
            "🗂️ By file size",
            "🔗 By executable hash",
            "📊 Comprehensive scan",
        ])
        .default(0)
        .interact()
    else {
        return;
    };

    let search_paths = vec![
        format!("{}/.steam/steam/steamapps/common", get_home_dir()),
        format!("{}/.local/share/lutris/prefixes", get_home_dir()),
        format!("{}/.local/share/bottles/bottles", get_home_dir()),
        format!("{}/Games", get_home_dir()),
    ];

    match search_method {
        0 => find_duplicates_by_name(&search_paths),
        1 => find_duplicates_by_size(&search_paths),
        2 => find_duplicates_by_hash(&search_paths),
        3 => comprehensive_duplicate_scan(&search_paths),
        _ => {}
    }
}

fn find_duplicates_by_name(paths: &[String]) {
    println!("📝 Finding duplicates by name similarity...\n");

    let mut all_games = Vec::with_capacity(200); // Pre-allocate for typical game library size

    for path in paths {
        if Path::new(path).exists()
            && let Ok(out) = Command::new("find")
                .args(&[path, "-maxdepth", "2", "-type", "d"])
                .output()
        {
            for line in String::from_utf8_lossy(&out.stdout).lines() {
                if let Some(dir_name) = Path::new(line).file_name()
                    && let Some(name) = dir_name.to_str()
                    && !name.is_empty()
                    && name != "common"
                    && name != "prefixes"
                {
                    all_games.push((name.to_lowercase(), line.to_string()));
                }
            }
        }
    }

    // Simple duplicate detection by exact name match
    let mut duplicates = Vec::with_capacity(10); // Most users don't have many duplicates
    for i in 0..all_games.len() {
        for j in (i + 1)..all_games.len() {
            let similarity = calculate_similarity(&all_games[i].0, &all_games[j].0);
            if similarity > 0.8 {
                // 80% similarity threshold
                duplicates.push((all_games[i].1.clone(), all_games[j].1.clone(), similarity));
            }
        }
    }

    if duplicates.is_empty() {
        println!("✅ No duplicate games found by name");
    } else {
        println!("⚠️ Found {} potential duplicate pairs:", duplicates.len());
        for (i, (path1, path2, similarity)) in duplicates.iter().enumerate() {
            println!("\n{}. Similarity: {:.1}%", i + 1, similarity * 100.0);
            println!("   📁 {}", path1);
            println!("   📁 {}", path2);

            // Show sizes
            let size1 = get_directory_size(path1);
            let size2 = get_directory_size(path2);
            println!(
                "   📊 Sizes: {} MB vs {} MB",
                size1 / 1024 / 1024,
                size2 / 1024 / 1024
            );
        }

        let Ok(cleanup) = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Review and remove duplicates?")
            .default(false)
            .interact()
        else {
            return;
        };

        if cleanup {
            cleanup_duplicate_games(duplicates);
        }
    }
}

fn calculate_similarity(s1: &str, s2: &str) -> f64 {
    let s1_clean = s1.replace(&['-', '_', ' '], "").to_lowercase();
    let s2_clean = s2.replace(&['-', '_', ' '], "").to_lowercase();

    if s1_clean == s2_clean {
        return 1.0;
    }

    if s1_clean.contains(&s2_clean) || s2_clean.contains(&s1_clean) {
        return 0.85;
    }

    // Simple Levenshtein-like comparison
    let common_chars = s1_clean.chars().filter(|c| s2_clean.contains(*c)).count();

    let max_len = s1_clean.len().max(s2_clean.len());
    if max_len == 0 {
        return 0.0;
    }

    common_chars as f64 / max_len as f64
}

fn get_directory_size(path: &str) -> u64 {
    let du_result = Command::new("du").args(&["-sb", path]).output();

    if let Ok(out) = du_result
        && let Ok(output_str) = String::from_utf8(out.stdout)
        && let Some(size_str) = output_str.split_whitespace().next()
    {
        return size_str.parse().unwrap_or(0);
    }
    0
}

fn find_duplicates_by_size(paths: &[String]) {
    println!("🗂️ Finding duplicates by file size...\n");

    let mut size_map: std::collections::HashMap<u64, Vec<String>> =
        std::collections::HashMap::new();

    for path in paths {
        if Path::new(path).exists() {
            let find_result = Command::new("find")
                .args(&[path, "-maxdepth", "2", "-type", "d"])
                .output();

            if let Ok(out) = find_result {
                for line in String::from_utf8_lossy(&out.stdout).lines() {
                    if line != *path {
                        let size = get_directory_size(line);
                        if size > 100 * 1024 * 1024 {
                            // Only consider directories > 100MB
                            size_map.entry(size).or_default().push(line.to_string());
                        }
                    }
                }
            }
        }
    }

    let mut duplicates_found = false;
    for (size, paths) in size_map {
        if paths.len() > 1 {
            duplicates_found = true;
            println!(
                "📊 {} directories with identical size {} MB:",
                paths.len(),
                size / 1024 / 1024
            );
            for path in paths {
                println!("  📁 {}", path);
            }
            println!();
        }
    }

    if !duplicates_found {
        println!("✅ No duplicate games found by size");
    }
}

fn find_duplicates_by_hash(paths: &[String]) {
    println!("🔗 Finding duplicates by executable hash...\n");
    println!("⏱️ This may take a while for large game libraries...\n");

    let mut hash_map: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for path in paths {
        if Path::new(path).exists() {
            println!("🔍 Scanning: {}", path);

            let find_result = Command::new("find")
                .args(&[path, "-name", "*.exe", "-type", "f"])
                .output();

            if let Ok(out) = find_result {
                let output_string = String::from_utf8_lossy(&out.stdout);
                let executables: Vec<&str> = output_string.lines().collect();
                println!("  📦 Found {} executables", executables.len());

                for (i, exe_path) in executables.iter().enumerate() {
                    if i % 10 == 0 {
                        println!("  Progress: {}/{}", i, executables.len());
                    }

                    let hash_result = Command::new("sha256sum").arg(exe_path).output();

                    if let Ok(hash_out) = hash_result
                        && let Some(hash) = String::from_utf8_lossy(&hash_out.stdout)
                            .split_whitespace()
                            .next()
                    {
                        hash_map
                            .entry(hash.to_string())
                            .or_default()
                            .push(exe_path.to_string());
                    }
                }
            }
        }
    }

    let mut duplicates_found = false;
    for (hash, paths) in hash_map {
        if paths.len() > 1 {
            duplicates_found = true;
            println!(
                "🔗 {} identical executables (hash: {}):",
                paths.len(),
                &hash[..16]
            );
            for path in paths {
                println!("  🎮 {}", path);
            }
            println!();
        }
    }

    if !duplicates_found {
        println!("✅ No duplicate executables found by hash");
    }
}

fn comprehensive_duplicate_scan(paths: &[String]) {
    println!("📊 Comprehensive Duplicate Scan");
    println!("================================\n");

    println!("🔍 Phase 1: Name similarity...");
    find_duplicates_by_name(paths);

    println!("\n🔍 Phase 2: Size comparison...");
    find_duplicates_by_size(paths);

    println!("\n🔍 Phase 3: Content analysis...");
    // Simplified content analysis - check for common game files
    analyze_game_content(paths);
}

fn analyze_game_content(paths: &[String]) {
    println!("📂 Analyzing game content patterns...\n");

    let common_game_files = vec![
        "*.dll",
        "*.exe",
        "Data",
        "data",
        "assets",
        "Assets",
        "config.ini",
        "settings.cfg",
        "save",
        "saves",
    ];

    let mut content_signatures: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for path in paths {
        if Path::new(path).exists() {
            let find_result = Command::new("find")
                .args(&[path, "-maxdepth", "2", "-type", "d"])
                .output();

            if let Ok(out) = find_result {
                for line in String::from_utf8_lossy(&out.stdout).lines() {
                    if line != *path {
                        let mut signature = String::new();

                        for pattern in &common_game_files {
                            let count_result = Command::new("find")
                                .args(&[line, "-name", pattern, "-type", "f", "|", "wc", "-l"])
                                .output();

                            if let Ok(count_out) = count_result {
                                let output_string = String::from_utf8_lossy(&count_out.stdout);
                                let count = output_string.trim();
                                signature.push_str(&format!("{}:{},", pattern, count));
                            }
                        }

                        if !signature.is_empty() {
                            content_signatures
                                .entry(signature)
                                .or_default()
                                .push(line.to_string());
                        }
                    }
                }
            }
        }
    }

    let mut found_content_duplicates = false;
    for (signature, dirs) in content_signatures {
        if dirs.len() > 1 {
            found_content_duplicates = true;
            println!(
                "📂 {} directories with similar content pattern:",
                dirs.len()
            );
            for dir in dirs {
                println!("  📁 {}", dir);
            }
            println!("  🔍 Pattern: {}\n", signature);
        }
    }

    if !found_content_duplicates {
        println!("✅ No content-based duplicates found");
    }
}

fn cleanup_duplicate_games(duplicates: Vec<(String, String, f64)>) {
    println!("🧹 Duplicate Game Cleanup");
    println!("==========================\n");

    for (i, (path1, path2, similarity)) in duplicates.iter().enumerate() {
        println!("Duplicate pair {}/{}", i + 1, duplicates.len());
        println!("Similarity: {:.1}%", similarity * 100.0);
        println!("1. {}", path1);
        println!("2. {}", path2);

        let size1 = get_directory_size(path1);
        let size2 = get_directory_size(path2);
        println!(
            "Sizes: {} MB vs {} MB",
            size1 / 1024 / 1024,
            size2 / 1024 / 1024
        );

        let Ok(action) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose action")
            .items(&[
                "⏭️ Skip this pair",
                "🗑️ Remove smaller directory",
                "🗑️ Remove larger directory",
                "🗑️ Remove first directory",
                "🗑️ Remove second directory",
                "📊 Show detailed comparison",
            ])
            .default(0)
            .interact()
        else {
            break;
        };

        match action {
            1 => {
                let to_remove = if size1 < size2 { path1 } else { path2 };
                remove_game_directory(to_remove);
            }
            2 => {
                let to_remove = if size1 > size2 { path1 } else { path2 };
                remove_game_directory(to_remove);
            }
            3 => remove_game_directory(path1),
            4 => remove_game_directory(path2),
            5 => show_detailed_comparison(path1, path2),
            _ => continue,
        }
    }
}

fn remove_game_directory(path: &str) {
    println!("🗑️ Removing: {}", path);

    let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!("⚠️ Permanently delete '{}'?", path))
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        let Ok(backup) = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Create backup before deletion?")
            .default(true)
            .interact()
        else {
            return;
        };

        if backup {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let backup_path = format!("{}.backup.{}", path, timestamp);

            println!("📦 Creating backup: {}", backup_path);
            let backup_result = Command::new("mv").args(&[path, &backup_path]).status();

            match backup_result {
                Ok(s) if s.success() => println!("✅ Backed up and removed"),
                _ => println!("❌ Backup failed"),
            }
        } else {
            let remove_result = Command::new("rm").args(&["-rf", path]).status();

            match remove_result {
                Ok(s) if s.success() => println!("✅ Directory removed"),
                _ => println!("❌ Removal failed"),
            }
        }
    }
}

fn show_detailed_comparison(path1: &str, path2: &str) {
    println!("📊 Detailed Comparison");
    println!("======================\n");

    println!("Directory 1: {}", path1);
    println!("Directory 2: {}", path2);

    // File counts
    let count1 = count_files(path1);
    let count2 = count_files(path2);
    println!("\nFile counts:");
    println!("  Dir 1: {} files", count1);
    println!("  Dir 2: {} files", count2);

    // Modification times
    println!("\nLast modified:");
    show_last_modified(path1);
    show_last_modified(path2);

    // Directory structure comparison
    println!("\nDirectory structure:");
    show_directory_structure(path1, "Dir 1");
    show_directory_structure(path2, "Dir 2");
}

fn count_files(path: &str) -> usize {
    let find_result = Command::new("find").args(&[path, "-type", "f"]).output();

    if let Ok(out) = find_result {
        String::from_utf8_lossy(&out.stdout).lines().count()
    } else {
        0
    }
}

fn show_last_modified(path: &str) {
    let stat_result = Command::new("stat").args(&["-c", "%Y %n", path]).output();

    if let Ok(out) = stat_result {
        let output_string = String::from_utf8_lossy(&out.stdout);
        println!("  {}: {}", path, output_string.trim());
    }
}

fn show_directory_structure(path: &str, label: &str) {
    println!("\n{} structure:", label);
    let tree_result = Command::new("ls").args(&["-la", path]).output();

    if let Ok(out) = tree_result {
        let output_string = String::from_utf8_lossy(&out.stdout);
        let lines: Vec<&str> = output_string.lines().take(10).collect();
        for line in lines {
            println!("  {}", line);
        }
    }
}

fn storage_usage_analysis() {
    println!("📊 Storage Usage Analysis");
    println!("=========================\n");

    let platforms = vec![
        ("Steam", format!("{}/.steam", get_home_dir())),
        ("Lutris", format!("{}/.local/share/lutris", get_home_dir())),
        (
            "Bottles",
            format!("{}/.local/share/bottles", get_home_dir()),
        ),
        ("Wine", format!("{}/.wine", get_home_dir())),
        ("Games", format!("{}/Games", get_home_dir())),
    ];

    let mut total_usage = 0u64;

    for (platform, path) in &platforms {
        if Path::new(path).exists() {
            println!("📊 Analyzing {} storage...", platform);

            let du_result = Command::new("du")
                .args(&["-h", "--max-depth=2", path])
                .output();

            if let Ok(out) = du_result {
                let output_string = String::from_utf8_lossy(&out.stdout);
                let lines: Vec<&str> = output_string.lines().collect();
                let mut platform_total = 0u64;

                for line in &lines {
                    if line.ends_with(path)
                        && let Some(size_str) = line.split_whitespace().next()
                    {
                        platform_total = parse_size_string(size_str);
                        total_usage += platform_total;
                    }
                }

                println!("  📦 Total: {} GB", platform_total / 1024 / 1024 / 1024);

                // Show largest subdirectories
                let mut sorted_lines = lines.clone();
                sorted_lines.sort_by(|a, b| {
                    let a_size = parse_size_string(a.split_whitespace().next().unwrap_or("0"));
                    let b_size = parse_size_string(b.split_whitespace().next().unwrap_or("0"));
                    b_size.cmp(&a_size)
                });

                println!("  🏆 Top consumers:");
                for line in sorted_lines.iter().take(5) {
                    if !line.ends_with(path) {
                        println!("    {}", line);
                    }
                }
            }
        } else {
            println!("📊 {} not found", platform);
        }
        println!();
    }

    println!("📊 Summary:");
    println!(
        "  Total gaming storage: {} GB",
        total_usage / 1024 / 1024 / 1024
    );

    // Cleanup suggestions
    suggest_cleanup_opportunities(&platforms);
}

fn parse_size_string(size_str: &str) -> u64 {
    let size_str = size_str.to_uppercase();
    let (number_part, unit) = if size_str.ends_with('G') {
        (size_str.trim_end_matches('G'), 1024 * 1024 * 1024)
    } else if size_str.ends_with('M') {
        (size_str.trim_end_matches('M'), 1024 * 1024)
    } else if size_str.ends_with('K') {
        (size_str.trim_end_matches('K'), 1024)
    } else {
        (size_str.as_str(), 1)
    };

    number_part.parse::<f64>().unwrap_or(0.0) as u64 * unit
}

fn suggest_cleanup_opportunities(_platforms: &[(&str, String)]) {
    println!("💡 Cleanup Opportunities:");
    println!("=========================\n");

    // Check for shader caches
    println!("🎨 Shader Caches:");
    let shader_paths = vec![
        format!("{}/.cache/mesa_shader_cache", get_home_dir()),
        format!("{}/.cache/nvidia", get_home_dir()),
        format!("{}/.steam/steam/steamapps/shadercache", get_home_dir()),
    ];

    let mut shader_total = 0u64;
    for path in &shader_paths {
        if Path::new(path).exists() {
            let size = get_directory_size(path);
            if size > 100 * 1024 * 1024 {
                // > 100MB
                shader_total += size;
                println!("  📁 {}: {} MB", path, size / 1024 / 1024);
            }
        }
    }

    if shader_total > 500 * 1024 * 1024 {
        // > 500MB
        println!(
            "  💡 Consider clearing shader caches: {} MB total",
            shader_total / 1024 / 1024
        );
    }

    // Check for temp files
    println!("\n🗑️ Temporary Files:");
    let temp_paths = vec![
        format!(
            "{}/.wine/drive_c/users/{}/Temp",
            get_home_dir(),
            get_user_name()
        ),
        format!("{}/.local/share/Steam/logs", get_home_dir()),
    ];

    for path in &temp_paths {
        if Path::new(path).exists() {
            let size = get_directory_size(path);
            if size > 50 * 1024 * 1024 {
                // > 50MB
                println!("  📁 {}: {} MB", path, size / 1024 / 1024);
            }
        }
    }

    // Check for old Wine prefixes
    println!("\n🍷 Wine Prefix Analysis:");
    let wine_prefixes_path = format!("{}/.local/share/wineprefixes", get_home_dir());
    if Path::new(&wine_prefixes_path).exists() {
        let find_result = Command::new("find")
            .args(&[
                &wine_prefixes_path,
                "-maxdepth",
                "1",
                "-type",
                "d",
                "-mtime",
                "+30",
            ])
            .output();

        if let Ok(out) = find_result {
            let output_string = String::from_utf8_lossy(&out.stdout);
            let old_prefixes: Vec<&str> = output_string.lines().collect();
            if !old_prefixes.is_empty() {
                println!(
                    "  📦 Found {} prefixes not used in 30+ days",
                    old_prefixes.len()
                );
                for prefix in old_prefixes.iter().take(5) {
                    let size = get_directory_size(prefix);
                    println!("    📁 {}: {} MB", prefix, size / 1024 / 1024);
                }
            }
        }
    }

    println!("\n🧹 Cleanup recommendations:");
    println!("  1. Run 'ghostctl repair wine' to clean up Wine installations");
    println!("  2. Clear shader caches if experiencing graphics issues");
    println!("  3. Remove unused Wine prefixes and game installations");
    println!("  4. Use the Deep Cleanup option for comprehensive cleaning");
}

fn wine_proton_cleanup() {
    let options = [
        "🩺 Wine Health Check",
        "🔧 Repair Wine Installation",
        "🧹 Clean Wine Prefixes",
        "🔄 Reset Wine Configuration",
        "📦 Reinstall Wine Dependencies",
        "🍷 Wine Registry Cleanup",
        "🚀 Update Proton Versions",
        "⬅️ Back",
    ];

    loop {
        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🍷 Wine/Proton Cleanup & Repair")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match choice {
            0 => wine_health_check(),
            1 => repair_wine_installation(),
            2 => clean_wine_prefixes(),
            3 => reset_wine_config(),
            4 => check_wine_dependencies(),
            5 => wine_registry_cleanup(),
            6 => update_proton_versions(),
            _ => break,
        }
    }
}

fn wine_health_check() {
    println!("🩺 Wine Health Check");
    println!("====================\n");

    // Check Wine installation
    println!("1️⃣ Wine Installation Status:");
    let wine_version = Command::new("wine").arg("--version").output();

    match wine_version {
        Ok(out) if out.status.success() => {
            let output_string = String::from_utf8_lossy(&out.stdout);
            println!("  ✅ Wine installed: {}", output_string.trim());
        }
        _ => {
            println!("  ❌ Wine not found or not working");
            return;
        }
    }

    // Check Wine architecture support
    println!("\n2️⃣ Architecture Support:");
    let arch_check = Command::new("wine").args(&["--help"]).output();

    if let Ok(out) = arch_check {
        let help_text = String::from_utf8_lossy(&out.stdout);
        if help_text.contains("64") {
            println!("  ✅ 64-bit support available");
        }
        if help_text.contains("32") {
            println!("  ✅ 32-bit support available");
        }
    }

    // Check critical Wine components
    println!("\n3️⃣ Critical Components:");
    let components = vec![
        ("winecfg", "Wine configuration tool"),
        ("winetricks", "Wine tricks utility"),
        ("wineconsole", "Wine console"),
        ("wineserver", "Wine server"),
    ];

    for (cmd, desc) in components {
        let check = Command::new("which").arg(cmd).status();
        match check {
            Ok(s) if s.success() => println!("  ✅ {}: {}", desc, cmd),
            _ => println!("  ❌ {}: {} not found", desc, cmd),
        }
    }

    // Check Wine dependencies
    println!("\n4️⃣ System Dependencies:");
    check_wine_dependencies();

    // Check Wine prefixes
    println!("\n5️⃣ Wine Prefixes:");
    check_wine_prefixes();

    // Check for common issues
    println!("\n6️⃣ Common Issues Check:");
    check_common_wine_issues();

    println!("\n📊 Health Check Summary:");
    println!("  Run 'ghostctl repair wine' if any issues were found");
}

fn check_wine_dependencies() {
    let deps = vec![
        ("lib32-gnutls", "32-bit security library"),
        ("lib32-libldap", "32-bit LDAP library"),
        ("lib32-libgpg-error", "32-bit GPG error library"),
        ("lib32-sqlite", "32-bit SQLite library"),
        ("lib32-libpulse", "32-bit PulseAudio library"),
        ("lib32-alsa-lib", "32-bit ALSA library"),
    ];

    for (package, desc) in deps {
        let check = Command::new("pacman").args(&["-Q", package]).output();

        match check {
            Ok(out) if out.status.success() => {
                let output_string = String::from_utf8_lossy(&out.stdout);
                let version = output_string.trim();
                println!("  ✅ {}", version);
            }
            _ => println!("  ❌ {} not installed ({})", package, desc),
        }
    }
}

fn check_wine_prefixes() {
    let default_prefix = format!("{}/.wine", get_home_dir());

    if Path::new(&default_prefix).exists() {
        println!("  ✅ Default prefix exists: {}", default_prefix);

        // Check prefix health
        let system32_path = format!("{}/drive_c/windows/system32", default_prefix);
        if Path::new(&system32_path).exists() {
            println!("    ✅ System32 directory present");
        } else {
            println!("    ❌ System32 directory missing - prefix may be corrupted");
        }

        // Check for DXVK/VKD3D
        let dxvk_dll = format!("{}/drive_c/windows/system32/d3d11.dll", default_prefix);
        if Path::new(&dxvk_dll).exists() {
            println!("    ✅ DXVK installed");
        } else {
            println!("    ⚠️ DXVK not detected");
        }
    } else {
        println!("  ⚠️ No default prefix found");
    }

    // Check for multiple prefixes
    let prefixes_dir = format!("{}/.local/share/wineprefixes", get_home_dir());
    if Path::new(&prefixes_dir).exists() {
        let count_result = Command::new("find")
            .args(&[&prefixes_dir, "-maxdepth", "1", "-type", "d"])
            .output();

        if let Ok(out) = count_result {
            let count = String::from_utf8_lossy(&out.stdout).lines().count();
            if count > 1 {
                println!("  📦 {} additional Wine prefixes found", count - 1);
            }
        }
    }
}

fn check_common_wine_issues() {
    // Check for ntlm_auth issues
    println!("🔍 Checking ntlm_auth...");
    let ntlm_check = Command::new("which").arg("ntlm_auth").status();

    match ntlm_check {
        Ok(s) if s.success() => println!("  ✅ ntlm_auth found"),
        _ => println!("  ❌ ntlm_auth missing (may cause authentication issues)"),
    }

    // Check fonts
    println!("\n🔍 Checking fonts...");
    let fonts_path = format!("{}/.wine/drive_c/windows/Fonts", get_home_dir());
    if Path::new(&fonts_path).exists() {
        let font_count = Command::new("find")
            .args(&[&fonts_path, "-name", "*.ttf", "-o", "-name", "*.otf"])
            .output();

        if let Ok(out) = font_count {
            let count = String::from_utf8_lossy(&out.stdout).lines().count();
            if count > 10 {
                println!("  ✅ {} fonts installed", count);
            } else {
                println!("  ⚠️ Only {} fonts found (may cause display issues)", count);
            }
        }
    } else {
        println!("  ❌ Fonts directory not found");
    }

    // Check for audio issues
    println!("\n🔍 Checking audio configuration...");
    let pulse_check = Command::new("pactl").args(&["info"]).status();

    match pulse_check {
        Ok(s) if s.success() => println!("  ✅ PulseAudio working"),
        _ => println!("  ⚠️ PulseAudio issues detected"),
    }
}

fn repair_wine_installation() {
    println!("🔧 Wine Installation Repair");
    println!("============================\n");

    let repair_options = [
        "🔧 Fix ntlm_auth authentication",
        "🎨 Install/Update Wine fonts",
        "📦 Reinstall 32-bit libraries",
        "🍷 Recreate Wine prefix",
        "🔄 Update Wine to latest version",
        "🧹 Clean Wine temporary files",
        "⚙️ Reset Wine configuration",
        "🔧 Full Wine repair (all above)",
        "⬅️ Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select repair option")
        .items(&repair_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => fix_ntlm_auth(),
        1 => install_wine_fonts(),
        2 => reinstall_32bit_libs(),
        3 => recreate_wine_prefix(),
        4 => update_wine_version(),
        5 => clean_wine_temp(),
        6 => reset_wine_config(),
        7 => full_wine_repair(),
        _ => {}
    }
}

fn fix_ntlm_auth() {
    println!("🔧 Fixing ntlm_auth Authentication");
    println!("===================================\n");

    // Check if samba is installed
    let samba_available = Command::new("which")
        .arg("ntlm_auth")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !samba_available {
        println!("📦 Installing samba for ntlm_auth...");

        let install_result = Command::new("sudo")
            .args(&["pacman", "-S", "samba"])
            .status();

        match install_result {
            Ok(s) if s.success() => println!("✅ Samba installed successfully"),
            _ => {
                println!("❌ Failed to install samba automatically");
                println!("💡 Please install manually: sudo pacman -S samba");
                return;
            }
        }
    }

    // Configure Wine to use system ntlm_auth
    let wine_prefix = format!("{}/.wine", get_home_dir());

    println!("🔧 Configuring Wine to use system ntlm_auth...");

    let reg_cmd = format!(
        "WINEPREFIX={} wine reg add 'HKEY_LOCAL_MACHINE\\System\\CurrentControlSet\\Services\\Winlogon' /v 'System' /d 'ntlm_auth' /f",
        wine_prefix
    );

    Command::new("sh").arg("-c").arg(&reg_cmd).status().ok();

    println!("✅ ntlm_auth configuration completed");
}

fn install_wine_fonts() {
    println!("🎨 Installing/Updating Wine Fonts");
    println!("==================================\n");

    let wine_prefix = format!("{}/.wine", get_home_dir());

    println!("📥 Installing core fonts with winetricks...");

    let core_fonts = vec!["corefonts", "tahoma", "liberation", "dejavu"];

    for font in &core_fonts {
        println!("  Installing {}...", font);

        let cmd = format!("WINEPREFIX={} winetricks -q {}", wine_prefix, font);
        let result = Command::new("sh").arg("-c").arg(&cmd).status();

        match result {
            Ok(s) if s.success() => println!("    ✅ {} installed", font),
            _ => println!("    ⚠️ {} installation failed", font),
        }
    }

    // Install Windows fonts if available
    let Ok(install_windows_fonts) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Install additional Windows fonts? (requires Windows partition/files)")
        .default(false)
        .interact()
    else {
        return;
    };

    if install_windows_fonts {
        let Ok(windows_fonts_path) = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter path to Windows Fonts directory")
            .interact()
        else {
            return;
        };

        if Path::new(&windows_fonts_path).exists() {
            println!("📁 Copying Windows fonts...");
            let copy_cmd = format!(
                "cp -r {}/* {}/.wine/drive_c/windows/Fonts/",
                windows_fonts_path,
                get_home_dir()
            );

            Command::new("sh").arg("-c").arg(&copy_cmd).status().ok();

            println!("✅ Windows fonts copied");
        }
    }

    println!("✅ Font installation completed");
}

fn reinstall_32bit_libs() {
    println!("📦 Reinstalling 32-bit Libraries");
    println!("=================================\n");

    let libs_32bit = vec![
        "lib32-gnutls",
        "lib32-libldap",
        "lib32-libgpg-error",
        "lib32-sqlite",
        "lib32-libpulse",
        "lib32-alsa-lib",
        "lib32-libxml2",
        "lib32-mpg123",
        "lib32-lcms2",
        "lib32-giflib",
        "lib32-libpng",
        "lib32-libjpeg-turbo",
    ];

    println!("📥 Installing 32-bit Wine dependencies...");

    for lib in &libs_32bit {
        println!("  Installing {}...", lib);

        let result = Command::new("sudo")
            .args(&["pacman", "-S", "--noconfirm", lib])
            .status();

        match result {
            Ok(s) if s.success() => println!("    ✅ {} installed", lib),
            _ => println!("    ⚠️ {} installation failed", lib),
        }
    }

    println!("✅ 32-bit libraries installation completed");
}

fn recreate_wine_prefix() {
    println!("🍷 Recreating Wine Prefix");
    println!("=========================\n");

    let wine_prefix = format!("{}/.wine", get_home_dir());

    if Path::new(&wine_prefix).exists() {
        let Ok(backup) = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Backup existing Wine prefix before recreation?")
            .default(true)
            .interact()
        else {
            return;
        };

        if backup {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let backup_path = format!("{}.backup.{}", wine_prefix, timestamp);

            println!("📦 Creating backup: {}", backup_path);
            Command::new("mv")
                .args(&[&wine_prefix, &backup_path])
                .status()
                .ok();
        } else {
            println!("🗑️ Removing existing prefix...");
            Command::new("rm")
                .args(&["-rf", &wine_prefix])
                .status()
                .ok();
        }
    }

    println!("🔧 Creating new Wine prefix...");
    let result = Command::new("winecfg").status();

    match result {
        Ok(s) if s.success() => {
            println!("✅ Wine prefix recreated successfully");

            // Ask about automatic configuration
            if let Ok(auto_config) = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Apply recommended gaming configuration?")
                .default(true)
                .interact()
            {
                if auto_config {
                    apply_gaming_wine_config();
                }
            }
        }
        _ => println!("❌ Failed to recreate Wine prefix"),
    }
}

fn apply_gaming_wine_config() {
    println!("⚙️ Applying gaming configuration...");

    let wine_prefix = format!("{}/.wine", get_home_dir());

    // Set Windows version to Windows 10
    let win_version_cmd = format!("WINEPREFIX={} winecfg /v win10", wine_prefix);
    Command::new("sh")
        .arg("-c")
        .arg(&win_version_cmd)
        .status()
        .ok();

    // Enable CSMT
    let csmt_cmd = format!(
        "WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Software\\Wine\\Direct3D' /v csmt /t REG_DWORD /d 1 /f",
        wine_prefix
    );
    Command::new("sh").arg("-c").arg(&csmt_cmd).status().ok();

    // Set environment variables
    unsafe { std::env::set_var("WINE_LARGE_ADDRESS_AWARE", "1") };
    unsafe { std::env::set_var("WINEESYNC", "1") };
    unsafe { std::env::set_var("WINEFSYNC", "1") };

    println!("✅ Gaming configuration applied");
}

fn update_wine_version() {
    println!("🔄 Updating Wine Version");
    println!("========================\n");

    println!("📋 Current Wine version:");
    Command::new("wine").args(&["--version"]).status().ok();

    println!("\n🔄 Updating system packages...");
    let update_result = Command::new("sudo")
        .args(&["pacman", "-Syu", "wine", "wine-staging"])
        .status();

    match update_result {
        Ok(s) if s.success() => {
            println!("✅ Wine updated successfully");

            println!("\n📋 New Wine version:");
            Command::new("wine").args(&["--version"]).status().ok();
        }
        _ => println!("❌ Wine update failed"),
    }
}

fn clean_wine_temp() {
    println!("🧹 Cleaning Wine Temporary Files");
    println!("=================================\n");

    let cleanup_paths = vec![
        format!(
            "{}/.wine/drive_c/users/{}/Temp/*",
            get_home_dir(),
            get_user_name()
        ),
        format!("{}/.wine/drive_c/windows/temp/*", get_home_dir()),
        format!("{}/tmp/wine*", get_home_dir()),
    ];

    for path in &cleanup_paths {
        if std::path::Path::new(&path.replace("*", "")).exists() {
            println!("🧹 Cleaning: {}", path);

            let clean_cmd = format!("rm -rf {}", path);
            Command::new("sh").arg("-c").arg(&clean_cmd).status().ok();
        }
    }

    println!("✅ Temporary files cleaned");
}

fn reset_wine_config() {
    println!("⚙️ Resetting Wine Configuration");
    println!("===============================\n");

    let _wine_prefix = format!("{}/.wine", get_home_dir());

    let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will reset all Wine settings. Continue?")
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        // Remove Wine configuration files
        let config_paths = vec![
            format!("{}/.wine/user.reg", get_home_dir()),
            format!("{}/.wine/system.reg", get_home_dir()),
            format!("{}/.wine/userdef.reg", get_home_dir()),
        ];

        for path in &config_paths {
            if Path::new(path).exists() {
                fs::remove_file(path).ok();
            }
        }

        println!("🔧 Opening Wine configuration...");
        Command::new("winecfg").status().ok();

        println!("✅ Wine configuration reset");
    }
}

fn full_wine_repair() {
    println!("🔧 Full Wine Repair");
    println!("===================\n");

    let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will perform a complete Wine repair. Continue?")
        .default(false)
        .interact()
    else {
        return;
    };

    if !confirm {
        return;
    }

    println!("🚀 Starting comprehensive Wine repair...\n");

    println!("1/7 Fixing ntlm_auth...");
    fix_ntlm_auth();

    println!("\n2/7 Installing fonts...");
    install_wine_fonts();

    println!("\n3/7 Reinstalling 32-bit libraries...");
    reinstall_32bit_libs();

    println!("\n4/7 Cleaning temporary files...");
    clean_wine_temp();

    println!("\n5/7 Updating Wine...");
    update_wine_version();

    println!("\n6/7 Resetting configuration...");
    reset_wine_config();

    println!("\n7/7 Applying gaming optimizations...");
    apply_gaming_wine_config();

    println!("\n✅ Full Wine repair completed!");
    println!("💡 Consider running a Wine health check to verify repairs");
}

fn update_proton_versions() {
    println!("🚀 Update Proton Versions");
    println!("=========================\n");

    let proton_options = [
        "🔄 Update Proton-GE",
        "🔄 Update Proton-TKG",
        "📦 Install latest Wine-GE",
        "📋 Check current Proton versions",
        "🗑️ Remove old Proton versions",
        "⬅️ Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Proton update option")
        .items(&proton_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => update_proton_ge(),
        1 => update_proton_tkg(),
        2 => install_wine_ge(),
        3 => check_proton_versions(),
        4 => cleanup_old_proton(),
        _ => {}
    }
}

fn update_proton_ge() {
    println!("🔄 Updating Proton-GE");
    println!("=====================\n");

    // Check if Steam is installed
    let steam_path = format!("{}/.steam", get_home_dir());
    if !Path::new(&steam_path).exists() {
        println!("❌ Steam not found. Proton-GE requires Steam to be installed.");
        return;
    }

    println!("📥 Fetching latest Proton-GE release...");

    // Get latest release info from GitHub API
    let api_result = Command::new("curl")
        .args(&[
            "-s",
            "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases/latest",
        ])
        .output();

    let download_url = match api_result {
        Ok(out) => {
            let json_str = String::from_utf8_lossy(&out.stdout);
            // Simple JSON parsing to get download URL
            if let Some(start) = json_str.find("\"browser_download_url\":\"") {
                let start_pos = start + 24;
                if let Some(end) = json_str[start_pos..].find(".tar.gz\"") {
                    let url = &json_str[start_pos..start_pos + end + 7];
                    url.to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        }
        _ => {
            println!("❌ Failed to fetch release information");
            return;
        }
    };

    if download_url.is_empty() {
        println!("❌ Could not find download URL");
        return;
    }

    println!("📥 Downloading latest Proton-GE...");
    println!("URL: {}", download_url);

    let download_result = Command::new("wget")
        .args(&["-P", "/tmp", &download_url])
        .status();

    let download_ok = download_result.map(|s| s.success()).unwrap_or(false);
    if !download_ok {
        println!("❌ Download failed");
        return;
    }

    // Extract filename from URL
    let filename = download_url
        .split('/')
        .next_back()
        .unwrap_or("proton-ge.tar.gz");
    let temp_path = format!("/tmp/{}", filename);

    println!("📂 Extracting Proton-GE...");
    let extract_result = Command::new("tar")
        .args(&["-xzf", &temp_path, "-C", "/tmp"])
        .status();

    let extract_ok = extract_result.map(|s| s.success()).unwrap_or(false);
    if !extract_ok {
        println!("❌ Extraction failed");
        return;
    }

    // Find extracted directory
    let extracted_name = filename.replace(".tar.gz", "");
    let extracted_path = format!("/tmp/{}", extracted_name);

    // Install to Steam directory
    let steam_compat_path = format!("{}/steam/steamapps/common", steam_path);
    fs::create_dir_all(&steam_compat_path).ok();

    let install_result = Command::new("mv")
        .args(&[
            &extracted_path,
            &format!("{}/{}", steam_compat_path, extracted_name),
        ])
        .status();

    match install_result {
        Ok(s) if s.success() => {
            println!("✅ Proton-GE installed successfully");

            // Cleanup
            fs::remove_file(&temp_path).ok();

            println!("💡 Restart Steam to see the new Proton version");
            println!("💡 Enable it in Steam > Settings > Steam Play > Proton version");
        }
        _ => println!("❌ Installation failed"),
    }
}

fn update_proton_tkg() {
    println!("🔄 Updating Proton-TKG");
    println!("======================\n");

    println!("⚠️ Proton-TKG requires building from source");
    println!("📖 Visit: https://github.com/Frogging-Family/wine-tkg-git");

    let Ok(proceed) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to clone and build Proton-TKG?")
        .default(false)
        .interact()
    else {
        return;
    };

    if proceed {
        println!("📥 Cloning Proton-TKG repository...");

        let clone_result = Command::new("git")
            .args(&[
                "clone",
                "https://github.com/Frogging-Family/wine-tkg-git.git",
                "/tmp/wine-tkg-git",
            ])
            .status();

        match clone_result {
            Ok(s) if s.success() => {
                println!("✅ Repository cloned");
                println!("📂 Build directory: /tmp/wine-tkg-git");
                println!("🔧 Run the build script manually:");
                println!("   cd /tmp/wine-tkg-git/wine-tkg-git");
                println!("   ./non-makepkg-build.sh");
            }
            _ => println!("❌ Failed to clone repository"),
        }
    }
}

fn install_wine_ge() {
    println!("📦 Installing Wine-GE");
    println!("=====================\n");

    // Wine-GE for Lutris
    let lutris_path = format!("{}/.local/share/lutris/runners/wine", get_home_dir());
    fs::create_dir_all(&lutris_path).ok();

    println!("📥 Fetching latest Wine-GE for Lutris...");

    let api_result = Command::new("curl")
        .args(&[
            "-s",
            "https://api.github.com/repos/GloriousEggroll/wine-ge-custom/releases/latest",
        ])
        .output();

    // Similar process to Proton-GE but for Lutris
    if let Ok(out) = api_result {
        let _json_str = String::from_utf8_lossy(&out.stdout);
        println!("📋 Latest Wine-GE release information fetched");

        println!("💡 Manual installation:");
        println!("1. Visit: https://github.com/GloriousEggroll/wine-ge-custom/releases");
        println!("2. Download the latest Wine-GE release");
        println!("3. Extract to: {}", lutris_path);
        println!("4. Restart Lutris and select Wine-GE in runner options");
    }
}

fn check_proton_versions() {
    println!("📋 Checking Current Proton Versions");
    println!("===================================\n");

    // Check Steam Proton versions
    let steam_path = format!("{}/.steam/steam/steamapps/common", get_home_dir());

    if Path::new(&steam_path).exists() {
        println!("🚀 Steam Proton Versions:");

        let find_result = Command::new("find")
            .args(&[
                &steam_path,
                "-maxdepth",
                "1",
                "-type",
                "d",
                "-name",
                "*roton*",
            ])
            .output();

        if let Ok(out) = find_result {
            let output_string = String::from_utf8_lossy(&out.stdout);
            let proton_dirs: Vec<&str> = output_string.lines().collect();

            if proton_dirs.is_empty() {
                println!("  📭 No Proton versions found");
            } else {
                for dir in proton_dirs {
                    if let Some(name) = Path::new(dir).file_name() {
                        println!("  ✅ {}", name.to_string_lossy());
                    }
                }
            }
        }
    } else {
        println!("🚀 Steam not found or not installed");
    }

    // Check Lutris Wine versions
    let lutris_path = format!("{}/.local/share/lutris/runners/wine", get_home_dir());

    if Path::new(&lutris_path).exists() {
        println!("\n🍷 Lutris Wine Versions:");

        let find_result = Command::new("find")
            .args(&[&lutris_path, "-maxdepth", "1", "-type", "d"])
            .output();

        if let Ok(out) = find_result {
            let output_string = String::from_utf8_lossy(&out.stdout);
            let wine_dirs: Vec<&str> = output_string.lines().collect();

            for dir in wine_dirs {
                if dir != lutris_path
                    && let Some(name) = Path::new(dir).file_name()
                {
                    println!("  ✅ {}", name.to_string_lossy());
                }
            }
        }
    } else {
        println!("\n🍷 Lutris not found or not installed");
    }

    // Check system Wine
    println!("\n🍷 System Wine Version:");
    Command::new("wine").args(&["--version"]).status().ok();
}

fn cleanup_old_proton() {
    println!("🗑️ Cleanup Old Proton Versions");
    println!("===============================\n");

    // List Steam Proton versions with sizes
    let steam_path = format!("{}/.steam/steam/steamapps/common", get_home_dir());

    if Path::new(&steam_path).exists() {
        println!("📊 Steam Proton Versions and Sizes:");

        let find_result = Command::new("find")
            .args(&[
                &steam_path,
                "-maxdepth",
                "1",
                "-type",
                "d",
                "-name",
                "*roton*",
            ])
            .output();

        if let Ok(out) = find_result {
            let output_string = String::from_utf8_lossy(&out.stdout);
            let proton_dirs: Vec<&str> = output_string.lines().collect();
            let mut version_info = Vec::new();

            for dir in proton_dirs {
                let size = get_directory_size(dir);
                if let Some(name) = Path::new(dir).file_name() {
                    version_info.push((name.to_string_lossy().to_string(), dir.to_string(), size));
                    println!(
                        "  📦 {} - {} MB",
                        name.to_string_lossy(),
                        size / 1024 / 1024
                    );
                }
            }

            if !version_info.is_empty() {
                let Ok(versions_to_remove) = MultiSelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select Proton versions to remove")
                    .items(
                        &version_info
                            .iter()
                            .map(|(name, _, size)| format!("{} ({} MB)", name, size / 1024 / 1024))
                            .collect::<Vec<_>>(),
                    )
                    .interact()
                else {
                    return;
                };

                for idx in versions_to_remove {
                    let (name, path, _) = &version_info[idx];

                    let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt(&format!("Remove {}?", name))
                        .default(false)
                        .interact()
                    else {
                        continue;
                    };

                    if confirm {
                        let result = Command::new("rm").args(&["-rf", path]).status();

                        match result {
                            Ok(s) if s.success() => println!("  ✅ {} removed", name),
                            _ => println!("  ❌ Failed to remove {}", name),
                        }
                    }
                }
            }
        }
    }
}

fn clean_wine_prefixes() {
    println!("🧹 Clean Wine Prefixes");
    println!("======================\n");

    let cleanup_options = [
        "📊 Analyze prefix usage",
        "🗑️ Remove unused prefixes",
        "🧹 Clean temporary files from all prefixes",
        "📦 Compact prefix registries",
        "🔄 Reset specific prefix",
        "⬅️ Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cleanup option")
        .items(&cleanup_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => analyze_prefix_usage(),
        1 => remove_unused_prefixes(),
        2 => clean_all_prefix_temps(),
        3 => compact_prefix_registries(),
        4 => reset_specific_prefix(),
        _ => {}
    }
}

fn analyze_prefix_usage() {
    println!("📊 Analyzing Wine Prefix Usage");
    println!("==============================\n");

    let common_prefix_locations = vec![
        format!("{}/.wine", get_home_dir()),
        format!("{}/.local/share/wineprefixes", get_home_dir()),
        format!("{}/.local/share/lutris/prefixes", get_home_dir()),
    ];

    let mut total_prefixes = 0;
    let mut total_size = 0u64;

    for location in &common_prefix_locations {
        if Path::new(location).exists() {
            println!("📁 Scanning: {}", location);

            if location.ends_with(".wine") {
                // Single default prefix
                let size = get_directory_size(location);
                let modified = get_last_access_time(location);

                println!("  📦 Default Wine prefix: {} MB", size / 1024 / 1024);
                println!("  📅 Last accessed: {}", modified);

                total_prefixes += 1;
                total_size += size;
            } else {
                // Multiple prefixes directory
                let find_result = Command::new("find")
                    .args(&[location, "-maxdepth", "1", "-type", "d"])
                    .output();

                if let Ok(out) = find_result {
                    let output_string = String::from_utf8_lossy(&out.stdout);
                    let dirs: Vec<&str> = output_string.lines().collect();

                    for dir in dirs {
                        if dir != *location {
                            let size = get_directory_size(dir);
                            let modified = get_last_access_time(dir);

                            if let Some(name) = Path::new(dir).file_name() {
                                println!(
                                    "  📦 {}: {} MB, Last accessed: {}",
                                    name.to_string_lossy(),
                                    size / 1024 / 1024,
                                    modified
                                );

                                total_prefixes += 1;
                                total_size += size;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\n📊 Summary:");
    println!("  Total prefixes: {}", total_prefixes);
    println!("  Total storage: {} GB", total_size / 1024 / 1024 / 1024);

    // Identify old/unused prefixes
    println!("\n🔍 Potentially Unused Prefixes (>30 days old):");
    identify_old_prefixes(&common_prefix_locations);
}

fn get_last_access_time(path: &str) -> String {
    let stat_result = Command::new("stat").args(&["-c", "%x", path]).output();

    if let Ok(out) = stat_result {
        let output_string = String::from_utf8_lossy(&out.stdout);
        output_string.trim().to_string()
    } else {
        "Unknown".to_string()
    }
}

fn identify_old_prefixes(locations: &[String]) {
    for location in locations {
        if Path::new(location).exists() {
            let find_result = Command::new("find")
                .args(&[location, "-maxdepth", "1", "-type", "d", "-mtime", "+30"])
                .output();

            if let Ok(out) = find_result {
                let output_string = String::from_utf8_lossy(&out.stdout);
                let old_dirs: Vec<&str> = output_string.lines().collect();

                for dir in old_dirs {
                    if dir != *location {
                        let size = get_directory_size(dir);
                        if let Some(name) = Path::new(dir).file_name() {
                            println!(
                                "  ⏰ {}: {} MB (not used in 30+ days)",
                                name.to_string_lossy(),
                                size / 1024 / 1024
                            );
                        }
                    }
                }
            }
        }
    }
}

fn remove_unused_prefixes() {
    println!("🗑️ Remove Unused Wine Prefixes");
    println!("==============================\n");

    let locations = vec![
        format!("{}/.local/share/wineprefixes", get_home_dir()),
        format!("{}/.local/share/lutris/prefixes", get_home_dir()),
    ];

    let mut prefixes_to_remove = Vec::new();

    for location in &locations {
        if Path::new(location).exists() {
            println!("🔍 Scanning: {}", location);

            let find_result = Command::new("find")
                .args(&[location, "-maxdepth", "1", "-type", "d", "-mtime", "+60"])
                .output();

            if let Ok(out) = find_result {
                for dir in String::from_utf8_lossy(&out.stdout).lines() {
                    if dir != *location && !dir.is_empty() {
                        let size = get_directory_size(dir);
                        let name = Path::new(dir)
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| "Unknown".to_string());

                        prefixes_to_remove.push((name.clone(), dir.to_string(), size));
                        println!(
                            "  📦 {}: {} MB (unused for 60+ days)",
                            name,
                            size / 1024 / 1024
                        );
                    }
                }
            }
        }
    }

    if prefixes_to_remove.is_empty() {
        println!("✅ No unused prefixes found");
        return;
    }

    let Ok(selected) = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select prefixes to remove")
        .items(
            &prefixes_to_remove
                .iter()
                .map(|(name, _, size)| format!("{} ({} MB)", name, size / 1024 / 1024))
                .collect::<Vec<_>>(),
        )
        .interact()
    else {
        return;
    };

    for idx in selected {
        let (name, path, size) = &prefixes_to_remove[idx];

        println!("\n🗑️ Removing: {} ({} MB)", name, size / 1024 / 1024);

        let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(&format!("Permanently delete prefix '{}'?", name))
            .default(false)
            .interact()
        else {
            continue;
        };

        if confirm {
            let result = Command::new("rm").args(&["-rf", path]).status();

            match result {
                Ok(s) if s.success() => println!("  ✅ {} removed", name),
                _ => println!("  ❌ Failed to remove {}", name),
            }
        }
    }
}

fn clean_all_prefix_temps() {
    println!("🧹 Cleaning Temporary Files from All Prefixes");
    println!("==============================================\n");

    let locations = vec![
        format!("{}/.wine", get_home_dir()),
        format!("{}/.local/share/wineprefixes", get_home_dir()),
        format!("{}/.local/share/lutris/prefixes", get_home_dir()),
    ];

    let mut total_cleaned = 0u64;

    for location in &locations {
        if Path::new(location).exists() {
            println!("🧹 Processing: {}", location);

            // Find all Wine prefixes
            let find_result = if location.ends_with(".wine") {
                // Single prefix - use echo command instead of manually constructing Output
                Command::new("echo").arg(location).output()
            } else {
                // Multiple prefixes
                Command::new("find")
                    .args(&[location, "-maxdepth", "1", "-type", "d"])
                    .output()
            };

            if let Ok(out) = find_result {
                for dir in String::from_utf8_lossy(&out.stdout).lines() {
                    if dir != *location || location.ends_with(".wine") {
                        let cleaned = clean_prefix_temp_files(dir);
                        if cleaned > 0 {
                            println!(
                                "  🧹 {}: cleaned {} MB",
                                Path::new(dir)
                                    .file_name()
                                    .map(|n| n.to_string_lossy().to_string())
                                    .unwrap_or_else(|| "Default".to_string()),
                                cleaned / 1024 / 1024
                            );
                            total_cleaned += cleaned;
                        }
                    }
                }
            }
        }
    }

    println!("\n✅ Cleaning completed");
    println!("📊 Total space freed: {} MB", total_cleaned / 1024 / 1024);
}

fn clean_prefix_temp_files(prefix_path: &str) -> u64 {
    let temp_patterns = vec![
        format!("{}/drive_c/users/*/Temp", prefix_path),
        format!("{}/drive_c/windows/temp", prefix_path),
        format!("{}/drive_c/users/*/AppData/Local/Temp", prefix_path),
    ];

    let mut total_cleaned = 0u64;

    for pattern in temp_patterns {
        // Remove the wildcard for directory existence check
        let base_pattern = pattern.replace("*/", "");
        let parent_dir: String = if pattern.contains("*/") {
            Path::new(&base_pattern)
                .parent()
                .and_then(|p| p.to_str())
                .unwrap_or(&base_pattern)
                .to_string()
        } else {
            base_pattern.clone()
        };

        if Path::new(&parent_dir).exists() {
            // Get size before cleaning
            let before_size = get_directory_size(&parent_dir);

            // Clean temp files
            let clean_cmd = format!(
                "find {} -name 'tmp*' -o -name '*.tmp' -o -name '*.temp' | head -100 | xargs rm -f 2>/dev/null || true",
                &parent_dir
            );
            Command::new("sh").arg("-c").arg(&clean_cmd).status().ok();

            // Get size after cleaning
            let after_size = get_directory_size(&parent_dir);
            total_cleaned += before_size.saturating_sub(after_size);
        }
    }

    total_cleaned
}

fn compact_prefix_registries() {
    println!("📦 Compacting Wine Prefix Registries");
    println!("====================================\n");

    println!("⚠️ This feature compacts Wine registry files to save space");
    println!("🔄 Registry compaction will be implemented in a future version");

    // For now, show registry file sizes
    let wine_prefix = format!("{}/.wine", get_home_dir());

    if Path::new(&wine_prefix).exists() {
        let registry_files = vec!["system.reg", "user.reg", "userdef.reg"];

        println!("\n📋 Current registry file sizes:");
        for reg_file in registry_files {
            let reg_path = format!("{}/{}", wine_prefix, reg_file);
            if Path::new(&reg_path).exists() {
                let size = get_directory_size(&reg_path);
                println!("  📄 {}: {} KB", reg_file, size / 1024);
            }
        }
    }
}

fn reset_specific_prefix() {
    println!("🔄 Reset Specific Wine Prefix");
    println!("=============================\n");

    // List available prefixes
    let locations = vec![
        format!("{}/.wine", get_home_dir()),
        format!("{}/.local/share/wineprefixes", get_home_dir()),
        format!("{}/.local/share/lutris/prefixes", get_home_dir()),
    ];

    let mut available_prefixes = Vec::new();

    for location in &locations {
        if Path::new(location).exists() {
            if location.ends_with(".wine") {
                available_prefixes.push(("Default Wine Prefix".to_string(), location.clone()));
            } else {
                let find_result = Command::new("find")
                    .args(&[location, "-maxdepth", "1", "-type", "d"])
                    .output();

                if let Ok(out) = find_result {
                    for dir in String::from_utf8_lossy(&out.stdout).lines() {
                        if dir != *location
                            && let Some(name) = Path::new(dir).file_name()
                        {
                            available_prefixes
                                .push((name.to_string_lossy().to_string(), dir.to_string()));
                        }
                    }
                }
            }
        }
    }

    if available_prefixes.is_empty() {
        println!("📭 No Wine prefixes found");
        return;
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select prefix to reset")
        .items(
            &available_prefixes
                .iter()
                .map(|(name, _)| name)
                .collect::<Vec<_>>(),
        )
        .default(0)
        .interact()
    else {
        return;
    };

    let (name, path) = &available_prefixes[choice];

    println!("\n⚠️ Resetting prefix: {}", name);
    println!("📁 Path: {}", path);

    let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will completely reset the selected prefix. Continue?")
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        // Create backup first
        let backup_path = format!(
            "{}.backup.{}",
            path,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        );

        println!("📦 Creating backup: {}", backup_path);
        Command::new("cp")
            .args(&["-r", path, &backup_path])
            .status()
            .ok();

        // Remove current prefix
        Command::new("rm").args(&["-rf", path]).status().ok();

        // Recreate prefix
        println!("🔧 Recreating prefix...");
        let recreate_cmd = format!("WINEPREFIX={} winecfg", path);
        Command::new("sh")
            .arg("-c")
            .arg(&recreate_cmd)
            .status()
            .ok();

        println!("✅ Prefix '{}' has been reset", name);
        println!("📦 Backup available at: {}", backup_path);
    }
}

fn bottles_management() {
    println!("📦 Bottles Management");
    println!("=====================\n");

    // Check if Bottles is installed
    let bottles_check = Command::new("which").arg("bottles").status();

    let bottles_available = bottles_check.map(|s| s.success()).unwrap_or(false);
    if !bottles_available {
        println!("❌ Bottles not found");

        let Ok(install) = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Install Bottles?")
            .default(true)
            .interact()
        else {
            return;
        };

        if install {
            println!("📦 Installing Bottles...");
            Command::new("yay").args(&["-S", "bottles"]).status().ok();
        }
        return;
    }

    let bottles_options = [
        "📋 List Bottles",
        "➕ Create New Bottle",
        "🗑️ Remove Bottle",
        "📊 Bottles Storage Analysis",
        "🔧 Bottle Maintenance",
        "⬅️ Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Bottles Management")
        .items(&bottles_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => list_bottles(),
        1 => create_new_bottle(),
        2 => remove_bottle(),
        3 => bottles_storage_analysis(),
        4 => bottle_maintenance(),
        _ => {}
    }
}

fn list_bottles() {
    println!("📋 Listing Bottles");
    println!("==================\n");

    let bottles_path = format!("{}/.local/share/bottles/bottles", get_home_dir());

    if !Path::new(&bottles_path).exists() {
        println!("📭 No bottles directory found");
        return;
    }

    let find_result = Command::new("find")
        .args(&[&bottles_path, "-maxdepth", "1", "-type", "d"])
        .output();

    if let Ok(out) = find_result {
        let output_string = String::from_utf8_lossy(&out.stdout);
        let bottles: Vec<&str> = output_string.lines().collect();

        if bottles.len() <= 1 {
            println!("📭 No bottles found");
            return;
        }

        println!("🍾 Found {} bottles:", bottles.len() - 1);

        for bottle in bottles {
            if bottle != bottles_path
                && let Some(name) = Path::new(bottle).file_name()
            {
                let size = get_directory_size(bottle);
                let config_path = format!("{}/bottle.yml", bottle);

                println!("\n📦 {}", name.to_string_lossy());
                println!("  📁 Path: {}", bottle);
                println!("  📊 Size: {} MB", size / 1024 / 1024);

                if Path::new(&config_path).exists() {
                    println!("  ✅ Configuration exists");

                    // Try to read some basic info from config
                    if let Ok(config_content) = std::fs::read_to_string(&config_path) {
                        for line in config_content.lines().take(10) {
                            if line.contains("name:")
                                || line.contains("runner:")
                                || line.contains("arch:")
                            {
                                println!("  📋 {}", line.trim());
                            }
                        }
                    }
                } else {
                    println!("  ⚠️ Configuration missing");
                }
            }
        }
    }
}

fn create_new_bottle() {
    println!("➕ Create New Bottle");
    println!("===================\n");

    println!("🔧 Creating bottle using Bottles GUI...");
    println!("💡 This will open the Bottles application");

    let Ok(launch) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Launch Bottles to create a new bottle?")
        .default(true)
        .interact()
    else {
        return;
    };

    if launch {
        Command::new("bottles").spawn().ok();

        println!("✅ Bottles launched");
        println!("💡 Use the GUI to create and configure your new bottle");
    }
}

fn remove_bottle() {
    println!("🗑️ Remove Bottle");
    println!("================\n");

    let bottles_path = format!("{}/.local/share/bottles/bottles", get_home_dir());

    if !Path::new(&bottles_path).exists() {
        println!("📭 No bottles directory found");
        return;
    }

    let find_result = Command::new("find")
        .args(&[&bottles_path, "-maxdepth", "1", "-type", "d"])
        .output();

    let mut available_bottles = Vec::new();

    if let Ok(out) = find_result {
        for bottle in String::from_utf8_lossy(&out.stdout).lines() {
            if bottle != bottles_path
                && let Some(name) = Path::new(bottle).file_name()
            {
                let size = get_directory_size(bottle);
                available_bottles.push((
                    name.to_string_lossy().to_string(),
                    bottle.to_string(),
                    size,
                ));
            }
        }
    }

    if available_bottles.is_empty() {
        println!("📭 No bottles found to remove");
        return;
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bottle to remove")
        .items(
            &available_bottles
                .iter()
                .map(|(name, _, size)| format!("{} ({} MB)", name, size / 1024 / 1024))
                .collect::<Vec<_>>(),
        )
        .default(0)
        .interact()
    else {
        return;
    };

    let (name, path, size) = &available_bottles[choice];

    println!("\n🗑️ Removing bottle: {}", name);
    println!("📁 Path: {}", path);
    println!("📊 Size: {} MB", size / 1024 / 1024);

    let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!("Permanently delete bottle '{}'?", name))
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        let result = Command::new("rm").args(&["-rf", path]).status();

        match result {
            Ok(s) if s.success() => println!("✅ Bottle '{}' removed successfully", name),
            _ => println!("❌ Failed to remove bottle '{}'", name),
        }
    }
}

fn bottles_storage_analysis() {
    println!("📊 Bottles Storage Analysis");
    println!("===========================\n");

    let bottles_path = format!("{}/.local/share/bottles", get_home_dir());

    if !Path::new(&bottles_path).exists() {
        println!("📭 Bottles not installed or no data found");
        return;
    }

    println!("📊 Bottles Installation Analysis:");

    // Total bottles storage
    let total_size = get_directory_size(&bottles_path);
    println!(
        "  📦 Total Bottles storage: {} MB",
        total_size / 1024 / 1024
    );

    // Individual bottle sizes
    let bottles_dir = format!("{}/bottles", bottles_path);
    if Path::new(&bottles_dir).exists() {
        println!("\n🍾 Individual Bottle Sizes:");

        let find_result = Command::new("find")
            .args(&[&bottles_dir, "-maxdepth", "1", "-type", "d"])
            .output();

        if let Ok(out) = find_result {
            let mut bottle_info = Vec::new();

            for bottle in String::from_utf8_lossy(&out.stdout).lines() {
                if bottle != bottles_dir
                    && let Some(name) = Path::new(bottle).file_name()
                {
                    let size = get_directory_size(bottle);
                    bottle_info.push((name.to_string_lossy().to_string(), size));
                }
            }

            // Sort by size
            bottle_info.sort_by(|a, b| b.1.cmp(&a.1));

            for (name, size) in bottle_info {
                println!("  📦 {}: {} MB", name, size / 1024 / 1024);
            }
        }
    }

    // Runtime storage
    let runners_dir = format!("{}/runners", bottles_path);
    if Path::new(&runners_dir).exists() {
        let runners_size = get_directory_size(&runners_dir);
        println!("\n🏃 Wine Runners: {} MB", runners_size / 1024 / 1024);
    }

    // Temp storage
    let temp_dir = format!("{}/temp", bottles_path);
    if Path::new(&temp_dir).exists() {
        let temp_size = get_directory_size(&temp_dir);
        if temp_size > 10 * 1024 * 1024 {
            // > 10MB
            println!(
                "\n🗑️ Temporary files: {} MB (consider cleaning)",
                temp_size / 1024 / 1024
            );
        }
    }
}

fn bottle_maintenance() {
    println!("🔧 Bottle Maintenance");
    println!("=====================\n");

    let maintenance_options = [
        "🧹 Clean bottle temp files",
        "📦 Optimize bottle storage",
        "🔄 Update bottle runners",
        "🩺 Check bottle health",
        "⬅️ Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select maintenance task")
        .items(&maintenance_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => clean_bottles_temp(),
        1 => optimize_bottle_storage(),
        2 => update_bottle_runners(),
        3 => check_bottle_health(),
        _ => {}
    }
}

fn clean_bottles_temp() {
    println!("🧹 Cleaning Bottle Temporary Files");
    println!("==================================\n");

    let bottles_path = format!("{}/.local/share/bottles", get_home_dir());
    let temp_paths = vec![
        format!("{}/temp", bottles_path),
        format!("{}/cache", bottles_path),
    ];

    let mut total_cleaned = 0u64;

    for temp_path in temp_paths {
        if Path::new(&temp_path).exists() {
            let before_size = get_directory_size(&temp_path);

            Command::new("rm")
                .args(&["-rf", &format!("{}/*", temp_path)])
                .status()
                .ok();

            let after_size = get_directory_size(&temp_path);
            let cleaned = before_size.saturating_sub(after_size);

            if cleaned > 0 {
                println!("🧹 Cleaned {}: {} MB", temp_path, cleaned / 1024 / 1024);
                total_cleaned += cleaned;
            }
        }
    }

    // Clean individual bottle temp files
    let bottles_dir = format!("{}/bottles", bottles_path);
    if Path::new(&bottles_dir).exists() {
        let find_result = Command::new("find")
            .args(&[&bottles_dir, "-maxdepth", "1", "-type", "d"])
            .output();

        if let Ok(out) = find_result {
            for bottle in String::from_utf8_lossy(&out.stdout).lines() {
                if bottle != bottles_dir {
                    let cleaned = clean_prefix_temp_files(bottle);
                    total_cleaned += cleaned;
                }
            }
        }
    }

    println!("\n✅ Cleanup completed");
    println!("📊 Total space freed: {} MB", total_cleaned / 1024 / 1024);
}

fn optimize_bottle_storage() {
    println!("📦 Optimizing Bottle Storage");
    println!("============================\n");

    println!("💡 Bottle storage optimization tips:");
    println!("  1. Remove unused bottles regularly");
    println!("  2. Use shared Wine runners when possible");
    println!("  3. Clean temporary files periodically");
    println!("  4. Consider using symlinks for shared game files");

    println!("\n🔧 Available optimizations:");
    println!("  • Deep clean implemented above");
    println!("  • Advanced compression requires manual setup");
    println!("  • Deduplication tools can be used externally");
}

fn update_bottle_runners() {
    println!("🔄 Updating Bottle Runners");
    println!("==========================\n");

    println!("🔧 Opening Bottles to update runners...");
    println!("💡 Use Bottles GUI to manage and update Wine runners");

    let Ok(launch) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Launch Bottles?")
        .default(true)
        .interact()
    else {
        return;
    };

    if launch {
        Command::new("bottles").spawn().ok();
    }
}

fn check_bottle_health() {
    println!("🩺 Checking Bottle Health");
    println!("=========================\n");

    let bottles_dir = format!("{}/.local/share/bottles/bottles", get_home_dir());

    if !Path::new(&bottles_dir).exists() {
        println!("📭 No bottles found");
        return;
    }

    let find_result = Command::new("find")
        .args(&[&bottles_dir, "-maxdepth", "1", "-type", "d"])
        .output();

    if let Ok(out) = find_result {
        for bottle in String::from_utf8_lossy(&out.stdout).lines() {
            if bottle != bottles_dir
                && let Some(name) = Path::new(bottle).file_name()
            {
                println!("🔍 Checking bottle: {}", name.to_string_lossy());

                // Check for essential files
                let config_file = format!("{}/bottle.yml", bottle);
                let drive_c = format!("{}/drive_c", bottle);
                let system32 = format!("{}/drive_c/windows/system32", bottle);

                if Path::new(&config_file).exists() {
                    println!("  ✅ Configuration file present");
                } else {
                    println!("  ❌ Configuration file missing");
                }

                if Path::new(&drive_c).exists() {
                    println!("  ✅ drive_c directory present");
                } else {
                    println!("  ❌ drive_c directory missing");
                }

                if Path::new(&system32).exists() {
                    println!("  ✅ system32 directory present");
                } else {
                    println!("  ❌ system32 directory missing");
                }

                println!();
            }
        }
    }
}
fn lutris_management() {
    loop {
        let options = [
            "📦 Install/Update Lutris",
            "🎮 Game Management",
            "🍷 Wine Runner Management",
            "🎯 World of Warcraft Complete Setup",
            "⚔️ Diablo 4 Complete Setup",
            "🔧 Lutris Configuration",
            "🧹 Lutris Cleanup & Maintenance",
            "⬅️ Back",
        ];

        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🎯 Lutris Management")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match choice {
            0 => install_update_lutris(),
            1 => lutris_game_management(),
            2 => lutris_wine_runners(),
            3 => setup_wow_complete(),
            4 => setup_diablo4_complete(),
            5 => lutris_configuration(),
            6 => lutris_cleanup_maintenance(),
            _ => break,
        }
    }
}

fn install_update_lutris() {
    println!("📦 Install/Update Lutris");
    println!("========================\n");

    let lutris_check = Command::new("which").arg("lutris").output();

    match lutris_check {
        Ok(out) if !out.stdout.is_empty() => {
            println!("✅ Lutris is installed");

            let version_check = Command::new("lutris").arg("--version").output();

            if let Ok(ver_out) = version_check {
                let output_string = String::from_utf8_lossy(&ver_out.stdout);
                println!("📋 Version: {}", output_string.trim());
            }

            let Ok(update) = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Update Lutris?")
                .default(false)
                .interact()
            else {
                return;
            };

            if update {
                update_lutris();
            }
        }
        _ => {
            println!("❌ Lutris not found");

            let Ok(install) = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Install Lutris?")
                .default(true)
                .interact()
            else {
                return;
            };

            if install {
                install_lutris_fresh();
            }
        }
    }

    check_lutris_dependencies();
}

fn install_lutris_fresh() {
    println!("📥 Installing Lutris");
    println!("====================\n");

    let package_managers = [
        ("pacman", vec!["-S", "lutris"]),
        ("yay", vec!["-S", "lutris"]),
        ("apt", vec!["install", "lutris"]),
        ("dnf", vec!["install", "lutris"]),
        ("zypper", vec!["install", "lutris"]),
    ];

    let mut installed = false;

    for (pm, args) in &package_managers {
        let pm_available = Command::new("which")
            .arg(pm)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if pm_available {
            println!("🔧 Using {} package manager", pm);

            let mut cmd = Command::new(pm);
            if pm != &"yay" {
                cmd.arg("sudo");
            }
            cmd.args(args);

            let install_result = cmd.status();

            match install_result {
                Ok(s) if s.success() => {
                    println!("✅ Lutris installed successfully");
                    installed = true;
                    break;
                }
                _ => println!("❌ Installation failed with {}", pm),
            }
        }
    }

    if !installed {
        println!("⚠️ Automatic installation failed");
        println!("💡 Please install Lutris manually:");
        println!("   • Arch: sudo pacman -S lutris");
        println!("   • Ubuntu/Debian: sudo apt install lutris");
        println!("   • Fedora: sudo dnf install lutris");
        println!("   • openSUSE: sudo zypper install lutris");
        println!("   • Flatpak: flatpak install flathub net.lutris.Lutris");
    }
}

fn update_lutris() {
    println!("🔄 Updating Lutris");
    println!("==================\n");

    let package_managers = [
        ("pacman", vec!["-Syu", "lutris"]),
        ("yay", vec!["-Syu", "lutris"]),
        ("apt", vec!["update", "&&", "apt", "upgrade", "lutris"]),
        ("dnf", vec!["update", "lutris"]),
        ("zypper", vec!["update", "lutris"]),
    ];

    for (pm, args) in &package_managers {
        let pm_available = Command::new("which")
            .arg(pm)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if pm_available {
            println!("🔧 Updating with {}", pm);

            let mut cmd = Command::new(pm);
            if pm != &"yay" {
                cmd.arg("sudo");
            }
            cmd.args(args);

            match cmd.status() {
                Ok(s) if s.success() => {
                    println!("✅ Lutris updated successfully");
                    return;
                }
                _ => println!("❌ Update failed with {}", pm),
            }
        }
    }

    println!("⚠️ Automatic update failed - please update manually");
}

fn check_lutris_dependencies() {
    println!("\n🔍 Checking Lutris Dependencies");
    println!("===============================");

    let deps = [
        ("wine", "Wine compatibility layer"),
        ("winetricks", "Wine helper scripts"),
        ("dxvk", "DirectX to Vulkan translation"),
        ("python3", "Python 3 runtime"),
        ("curl", "HTTP client for downloads"),
        ("cabextract", "Microsoft Cabinet extractor"),
        ("unzip", "Archive extraction"),
        ("xvfb", "Virtual framebuffer for headless operation"),
    ];

    for (dep, description) in &deps {
        let dep_available = Command::new("which")
            .arg(dep)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if dep_available {
            println!("  ✅ {}: {}", dep, description);
        } else {
            println!("  ❌ {}: {} (missing)", dep, description);
        }
    }

    println!("\n💡 Install missing dependencies for optimal Lutris performance");
}

fn lutris_game_management() {
    let options = [
        "📋 List Installed Games",
        "🎮 Install Game from Lutris Website",
        "➕ Add Local Game",
        "🗑️ Remove Game",
        "🔧 Configure Game",
        "🏃 Launch Game",
        "📊 Games Storage Analysis",
        "⬅️ Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🎮 Lutris Game Management")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => list_lutris_games(),
        1 => install_game_from_lutris(),
        2 => add_local_game(),
        3 => remove_lutris_game(),
        4 => configure_lutris_game(),
        5 => launch_lutris_game(),
        6 => lutris_games_storage(),
        _ => {}
    }
}

fn list_lutris_games() {
    println!("📋 Lutris Installed Games");
    println!("=========================\n");

    let list_result = Command::new("lutris").args(&["--list-games"]).output();

    match list_result {
        Ok(out) => {
            let games_output = String::from_utf8_lossy(&out.stdout);

            if games_output.trim().is_empty() {
                println!("📭 No games found in Lutris");
            } else {
                println!("🎮 Installed Games:");
                for line in games_output.lines() {
                    if !line.trim().is_empty() {
                        println!("  🎯 {}", line.trim());
                    }
                }
            }
        }
        Err(_) => {
            println!("❌ Failed to list games");
            println!("💡 Make sure Lutris is installed and in PATH");
        }
    }

    let games_dir = format!("{}/.local/share/lutris/games", get_home_dir());
    if Path::new(&games_dir).exists() {
        println!("\n📁 Games Directory Analysis:");
        let dir_size = get_directory_size(&games_dir);
        println!(
            "  📊 Total games storage: {} GB",
            dir_size / 1024 / 1024 / 1024
        );
    }
}

fn install_game_from_lutris() {
    println!("🎮 Install Game from Lutris Website");
    println!("===================================\n");

    println!("🌐 To install games from Lutris.net:");
    println!("1. Visit https://lutris.net/games/");
    println!("2. Search for your game");
    println!("3. Click 'Install' and choose a script");
    println!("4. The Lutris installer will launch automatically");

    println!("\n💡 Popular games available:");
    println!("  • World of Warcraft");
    println!("  • Diablo 4");
    println!("  • League of Legends");
    println!("  • Overwatch 2");
    println!("  • Steam games (as non-Steam games)");

    let Ok(launch_browser) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Open Lutris website in browser?")
        .default(true)
        .interact()
    else {
        return;
    };

    if launch_browser {
        Command::new("xdg-open")
            .arg("https://lutris.net/games/")
            .spawn()
            .ok();
    }
}

fn add_local_game() {
    println!("➕ Add Local Game to Lutris");
    println!("============================\n");

    let Ok(game_name): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Game name")
        .interact()
    else {
        return;
    };

    let Ok(game_path): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Game executable path")
        .interact()
    else {
        return;
    };

    if !Path::new(&game_path).exists() {
        println!("❌ Game executable not found: {}", game_path);
        return;
    }

    println!("🔧 Adding game to Lutris...");

    let add_result = Command::new("lutris").args(&["-i", &game_path]).status();

    match add_result {
        Ok(s) if s.success() => println!("✅ Game '{}' added to Lutris", game_name),
        _ => {
            println!("⚠️ Automatic addition failed");
            println!("💡 Manually add in Lutris GUI:");
            println!("   1. Open Lutris");
            println!("   2. Click '+' (Add Game)");
            println!("   3. Choose 'Add locally installed game'");
            println!("   4. Fill in the details");
        }
    }
}

fn remove_lutris_game() {
    println!("🗑️ Remove Game from Lutris");
    println!("============================\n");

    let list_result = Command::new("lutris").args(&["--list-games"]).output();

    match list_result {
        Ok(out) => {
            let games_output = String::from_utf8_lossy(&out.stdout);
            let games: Vec<&str> = games_output
                .lines()
                .filter(|line| !line.trim().is_empty())
                .collect();

            if games.is_empty() {
                println!("📭 No games found to remove");
                return;
            }

            let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select game to remove")
                .items(&games)
                .default(0)
                .interact()
            else {
                return;
            };

            let selected_game = games[choice];

            let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(&format!("Remove '{}'?", selected_game))
                .default(false)
                .interact()
            else {
                return;
            };

            if confirm {
                println!("💡 To remove the game:");
                println!("   1. Open Lutris GUI");
                println!("   2. Right-click on '{}'", selected_game);
                println!("   3. Select 'Remove' or 'Uninstall'");

                let Ok(open_lutris) = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Open Lutris now?")
                    .default(true)
                    .interact()
                else {
                    return;
                };

                if open_lutris {
                    Command::new("lutris").spawn().ok();
                }
            }
        }
        Err(_) => println!("❌ Failed to get games list"),
    }
}

fn configure_lutris_game() {
    println!("🔧 Configure Lutris Game");
    println!("=========================\n");

    println!("💡 Game configuration is best done through the Lutris GUI");
    println!("🔧 Available configuration options:");
    println!("   • Wine version");
    println!("   • Wine prefix settings");
    println!("   • System options (resolution, fullscreen, etc.)");
    println!("   • Game arguments");
    println!("   • Environment variables");

    let Ok(open_lutris) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Open Lutris to configure games?")
        .default(true)
        .interact()
    else {
        return;
    };

    if open_lutris {
        Command::new("lutris").spawn().ok();
        println!("✅ Lutris opened - right-click on any game to configure");
    }
}

fn launch_lutris_game() {
    println!("🏃 Launch Game via Lutris");
    println!("=========================\n");

    let list_result = Command::new("lutris").args(&["--list-games"]).output();

    match list_result {
        Ok(out) => {
            let games_output = String::from_utf8_lossy(&out.stdout);
            let games: Vec<&str> = games_output
                .lines()
                .filter(|line| !line.trim().is_empty())
                .collect();

            if games.is_empty() {
                println!("📭 No games found to launch");
                return;
            }

            let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select game to launch")
                .items(&games)
                .default(0)
                .interact()
            else {
                return;
            };

            let selected_game = games[choice];

            println!("🚀 Launching '{}'...", selected_game);

            let launch_result = Command::new("lutris")
                .args(&[
                    "lutris:rungame",
                    &selected_game.to_lowercase().replace(" ", "-"),
                ])
                .spawn();

            match launch_result {
                Ok(_) => println!("✅ Game launched"),
                Err(_) => {
                    println!("⚠️ Direct launch failed - opening Lutris GUI");
                    Command::new("lutris").spawn().ok();
                }
            }
        }
        Err(_) => println!("❌ Failed to get games list"),
    }
}

fn lutris_games_storage() {
    println!("📊 Lutris Games Storage Analysis");
    println!("=================================\n");

    let lutris_dirs = [
        (
            "Games",
            format!("{}/.local/share/lutris/games", get_home_dir()),
        ),
        (
            "Prefixes",
            format!("{}/.local/share/lutris/prefixes", get_home_dir()),
        ),
        (
            "Runners",
            format!("{}/.local/share/lutris/runners", get_home_dir()),
        ),
        ("Cache", format!("{}/.cache/lutris", get_home_dir())),
    ];

    let mut total_size = 0u64;

    for (category, path) in &lutris_dirs {
        if Path::new(path).exists() {
            let size = get_directory_size(path);
            total_size += size;

            println!("📦 {}: {} GB", category, size / 1024 / 1024 / 1024);

            if category == &"Games" && size > 0 {
                let find_result = Command::new("find")
                    .args(&[path, "-maxdepth", "1", "-type", "d"])
                    .output();

                if let Ok(out) = find_result {
                    for game_dir in String::from_utf8_lossy(&out.stdout).lines() {
                        if game_dir != *path
                            && let Some(game_name) = Path::new(game_dir).file_name()
                        {
                            let game_size = get_directory_size(game_dir);
                            if game_size > 100 * 1024 * 1024 {
                                println!(
                                    "  🎮 {}: {} MB",
                                    game_name.to_string_lossy(),
                                    game_size / 1024 / 1024
                                );
                            }
                        }
                    }
                }
            }
        } else {
            println!("📦 {}: Not found", category);
        }
    }

    println!(
        "\n📊 Total Lutris Storage: {} GB",
        total_size / 1024 / 1024 / 1024
    );

    if total_size > 50 * 1024 * 1024 * 1024 {
        println!("💡 Consider cleaning up old games and prefixes");
    }
}

fn lutris_wine_runners() {
    let options = [
        "📋 List Available Runners",
        "📥 Install Wine Runner",
        "🔄 Update Runners",
        "🗑️ Remove Old Runners",
        "🔧 Set Default Runner",
        "⬅️ Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🍷 Lutris Wine Runner Management")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => list_lutris_runners(),
        1 => install_wine_runner(),
        2 => update_lutris_runners(),
        3 => remove_old_runners(),
        4 => set_default_runner(),
        _ => {}
    }
}

fn list_lutris_runners() {
    println!("📋 Lutris Wine Runners");
    println!("======================\n");

    let runners_path = format!("{}/.local/share/lutris/runners/wine", get_home_dir());

    if !Path::new(&runners_path).exists() {
        println!("📭 No Wine runners directory found");
        fs::create_dir_all(&runners_path).ok();
        return;
    }

    let find_result = Command::new("find")
        .args(&[&runners_path, "-maxdepth", "1", "-type", "d"])
        .output();

    match find_result {
        Ok(out) => {
            let output_string = String::from_utf8_lossy(&out.stdout);
            let runners: Vec<&str> = output_string.lines().collect();

            if runners.len() <= 1 {
                println!("📭 No Wine runners installed");
            } else {
                println!("🍷 Installed Wine Runners:");

                for runner in runners {
                    if runner != runners_path
                        && let Some(name) = Path::new(runner).file_name()
                    {
                        let size = get_directory_size(runner);
                        println!("  📦 {}: {} MB", name.to_string_lossy(), size / 1024 / 1024);

                        let wine_bin = format!("{}/bin/wine", runner);
                        if Path::new(&wine_bin).exists() {
                            let version_check = Command::new(&wine_bin).arg("--version").output();

                            if let Ok(ver_out) = version_check {
                                let version = String::from_utf8_lossy(&ver_out.stdout);
                                println!("    📋 Version: {}", version.trim());
                            }
                        }
                    }
                }
            }
        }
        Err(_) => println!("❌ Failed to list runners"),
    }

    println!("\n💡 Popular Wine runners for gaming:");
    println!("   • wine-ge-8-26: GloriousEggroll's Wine with gaming patches");
    println!("   • lutris-fshack-7.2: Lutris optimized Wine with fsync");
    println!("   • wine-tkg: Community Wine with gaming optimizations");
}

fn install_wine_runner() {
    println!("📥 Install Wine Runner");
    println!("======================\n");

    let runner_options = [
        "Wine-GE (GloriousEggroll) - Best for gaming",
        "Lutris Wine-fshack - Optimized for Lutris",
        "Wine-TKG - Community gaming patches",
        "System Wine - Use system Wine",
        "Custom URL - Download from URL",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Wine runner to install")
        .items(&runner_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => install_wine_ge_lutris(),
        1 => install_lutris_fshack(),
        2 => install_wine_tkg_lutris(),
        3 => setup_system_wine(),
        4 => install_custom_runner(),
        _ => {}
    }
}

fn install_wine_ge_lutris() {
    println!("📥 Installing Wine-GE for Lutris");
    println!("=================================\n");

    let runners_path = format!("{}/.local/share/lutris/runners/wine", get_home_dir());
    fs::create_dir_all(&runners_path).ok();

    println!("📥 Fetching latest Wine-GE release...");

    let api_result = Command::new("curl")
        .args(&[
            "-s",
            "https://api.github.com/repos/GloriousEggroll/wine-ge-custom/releases/latest",
        ])
        .output();

    match api_result {
        Ok(out) => {
            let json_str = String::from_utf8_lossy(&out.stdout);

            if let Some(start) = json_str.find("browser_download_url")
                && let Some(url_start) = json_str[start..].find("https://")
                && let Some(url_end) = json_str[start + url_start..].find("\"")
            {
                let url = &json_str[start + url_start..start + url_start + url_end];

                if url.ends_with(".tar.xz") {
                    println!("📋 Found: {}", url);

                    let Ok(download) = Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Download and install this Wine-GE version?")
                        .default(true)
                        .interact()
                    else {
                        return;
                    };

                    if download {
                        download_and_install_runner(url, &runners_path);
                    }
                }
            }
        }
        Err(_) => {
            println!("❌ Failed to fetch release info");
            println!("💡 Manual installation:");
            println!("1. Visit: https://github.com/GloriousEggroll/wine-ge-custom/releases");
            println!("2. Download the latest .tar.xz file");
            println!("3. Extract to: {}", runners_path);
        }
    }
}

fn install_lutris_fshack() {
    println!("📥 Installing Lutris Wine-fshack");
    println!("================================\n");

    println!("💡 Lutris Wine-fshack runners are managed automatically");
    println!("🔧 To get Lutris runners:");
    println!("   1. Open Lutris");
    println!("   2. Go to Preferences → Runners → Wine");
    println!("   3. Click the manage versions button");
    println!("   4. Download desired versions");

    let Ok(open_lutris) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Open Lutris to manage runners?")
        .default(true)
        .interact()
    else {
        return;
    };

    if open_lutris {
        Command::new("lutris").spawn().ok();
    }
}

fn install_wine_tkg_lutris() {
    println!("📥 Installing Wine-TKG");
    println!("======================\n");

    println!("⚠️ Wine-TKG requires building from source");
    println!("🔗 Repository: https://github.com/Frogging-Family/wine-tkg-git");

    let Ok(proceed) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will clone and provide build instructions. Continue?")
        .default(false)
        .interact()
    else {
        return;
    };

    if proceed {
        let temp_dir = "/tmp/wine-tkg-lutris";

        println!("📥 Cloning Wine-TKG...");
        let clone_result = Command::new("git")
            .args(&[
                "clone",
                "https://github.com/Frogging-Family/wine-tkg-git.git",
                temp_dir,
            ])
            .status();

        match clone_result {
            Ok(s) if s.success() => {
                println!("✅ Repository cloned to {}", temp_dir);
                println!("\n🔧 To build Wine-TKG for Lutris:");
                println!("   1. cd {}/wine-tkg-git", temp_dir);
                println!("   2. Edit customization.cfg as needed");
                println!("   3. ./non-makepkg-build.sh");
                println!("   4. Copy built Wine to ~/.local/share/lutris/runners/wine/");
            }
            _ => println!("❌ Failed to clone repository"),
        }
    }
}

fn setup_system_wine() {
    println!("🍷 Setup System Wine for Lutris");
    println!("================================\n");

    let wine_check = Command::new("which").arg("wine").output();

    match wine_check {
        Ok(out) if !out.stdout.is_empty() => {
            let output_string = String::from_utf8_lossy(&out.stdout);
            let wine_path = output_string.trim().to_string();
            println!("✅ System Wine found: {}", wine_path);

            let version_check = Command::new("wine").arg("--version").output();

            if let Ok(ver_out) = version_check {
                let output_string = String::from_utf8_lossy(&ver_out.stdout);
                println!("📋 Version: {}", output_string.trim());
            }

            println!("\n💡 Lutris will automatically detect system Wine");
            println!("🔧 No additional setup required");
        }
        _ => {
            println!("❌ System Wine not found");

            let Ok(install) = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Install system Wine?")
                .default(true)
                .interact()
            else {
                return;
            };

            if install {
                install_system_wine();
            }
        }
    }
}

fn install_system_wine() {
    println!("📥 Installing System Wine");
    println!("=========================\n");

    let package_managers = [
        ("pacman", vec!["sudo", "pacman", "-S", "wine"]),
        ("apt", vec!["sudo", "apt", "install", "wine"]),
        ("dnf", vec!["sudo", "dnf", "install", "wine"]),
        ("zypper", vec!["sudo", "zypper", "install", "wine"]),
    ];

    for (pm, cmd) in &package_managers {
        let pm_available = Command::new("which")
            .arg(pm)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if pm_available {
            println!("🔧 Installing Wine with {}", pm);

            let install_result = Command::new(cmd[0]).args(&cmd[1..]).status();

            match install_result {
                Ok(s) if s.success() => {
                    println!("✅ Wine installed successfully");
                    return;
                }
                _ => println!("❌ Installation failed with {}", pm),
            }
        }
    }

    println!("⚠️ Automatic installation failed");
    println!("💡 Please install Wine manually for your distribution");
}

fn install_custom_runner() {
    println!("📥 Install Custom Wine Runner");
    println!("=============================\n");

    let Ok(url): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter download URL for Wine runner (.tar.xz)")
        .interact()
    else {
        return;
    };

    if !url.starts_with("http") || !url.ends_with(".tar.xz") {
        println!("❌ Invalid URL - must be HTTP(S) and end with .tar.xz");
        return;
    }

    let runners_path = format!("{}/.local/share/lutris/runners/wine", get_home_dir());
    fs::create_dir_all(&runners_path).ok();

    download_and_install_runner(&url, &runners_path);
}

fn download_and_install_runner(url: &str, runners_path: &str) {
    let filename = url.split('/').next_back().unwrap_or("wine-runner.tar.xz");
    let temp_path = format!("/tmp/{}", filename);

    println!("📥 Downloading {}...", filename);

    let download_result = Command::new("curl")
        .args(&["-L", "-o", &temp_path, url])
        .status();

    match download_result {
        Ok(s) if s.success() => {
            println!("✅ Download completed");

            println!("📦 Extracting...");
            let extract_result = Command::new("tar")
                .args(&["-xf", &temp_path, "-C", runners_path])
                .status();

            match extract_result {
                Ok(s) if s.success() => {
                    println!("✅ Wine runner installed successfully");
                    fs::remove_file(&temp_path).ok();

                    let find_result = Command::new("find")
                        .args(&[
                            runners_path,
                            "-maxdepth",
                            "1",
                            "-type",
                            "d",
                            "-newer",
                            "/tmp",
                        ])
                        .output();

                    if let Ok(out) = find_result {
                        for new_dir in String::from_utf8_lossy(&out.stdout).lines() {
                            if new_dir != runners_path
                                && let Some(name) = Path::new(new_dir).file_name()
                            {
                                println!("📦 Installed: {}", name.to_string_lossy());
                            }
                        }
                    }
                }
                _ => println!("❌ Extraction failed"),
            }
        }
        _ => println!("❌ Download failed"),
    }
}

fn update_lutris_runners() {
    println!("🔄 Update Lutris Runners");
    println!("========================\n");

    println!("💡 To update Lutris Wine runners:");
    println!("   1. Open Lutris");
    println!("   2. Go to Preferences → Runners → Wine");
    println!("   3. Use the manage versions button");
    println!("   4. Download newer versions");

    println!("\n🔧 For manual updates:");
    println!("   • Wine-GE: Check GitHub releases");
    println!("   • Wine-TKG: Rebuild from latest source");
    println!("   • Lutris runners: Use GUI manager");

    let Ok(open_lutris) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Open Lutris runner manager?")
        .default(true)
        .interact()
    else {
        return;
    };

    if open_lutris {
        Command::new("lutris").spawn().ok();
    }
}

fn remove_old_runners() {
    println!("🗑️ Remove Old Wine Runners");
    println!("===========================\n");

    let runners_path = format!("{}/.local/share/lutris/runners/wine", get_home_dir());

    if !Path::new(&runners_path).exists() {
        println!("📭 No runners directory found");
        return;
    }

    let find_result = Command::new("find")
        .args(&[&runners_path, "-maxdepth", "1", "-type", "d"])
        .output();

    let mut available_runners = Vec::new();

    if let Ok(out) = find_result {
        for runner in String::from_utf8_lossy(&out.stdout).lines() {
            if runner != runners_path
                && let Some(name) = Path::new(runner).file_name()
            {
                let size = get_directory_size(runner);
                available_runners.push((
                    name.to_string_lossy().to_string(),
                    runner.to_string(),
                    size,
                ));
            }
        }
    }

    if available_runners.is_empty() {
        println!("📭 No Wine runners found to remove");
        return;
    }

    println!("🍷 Available Wine Runners:");
    for (name, _, size) in &available_runners {
        println!("  📦 {}: {} MB", name, size / 1024 / 1024);
    }

    let Ok(runners_to_remove) = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select runners to remove")
        .items(
            &available_runners
                .iter()
                .map(|(name, _, size)| format!("{} ({} MB)", name, size / 1024 / 1024))
                .collect::<Vec<_>>(),
        )
        .interact()
    else {
        return;
    };

    for idx in runners_to_remove {
        let (name, path, size) = &available_runners[idx];

        let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(&format!("Remove {} ({} MB)?", name, size / 1024 / 1024))
            .default(false)
            .interact()
        else {
            continue;
        };

        if confirm {
            let result = Command::new("rm").args(&["-rf", path]).status();

            match result {
                Ok(s) if s.success() => println!("✅ {} removed", name),
                _ => println!("❌ Failed to remove {}", name),
            }
        }
    }
}

fn set_default_runner() {
    println!("🔧 Set Default Wine Runner");
    println!("==========================\n");

    println!("💡 Default Wine runner is set per-game in Lutris");
    println!("🔧 To change default runner:");
    println!("   1. Open Lutris");
    println!("   2. Right-click on a game");
    println!("   3. Configure → Runner options");
    println!("   4. Select Wine version");

    println!("\n📋 System-wide Wine runner preference:");
    println!("   • Lutris uses its own runner management");
    println!("   • Games can have individual runner settings");
    println!("   • Global preferences are in Lutris settings");

    let Ok(open_lutris) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Open Lutris to configure runners?")
        .default(true)
        .interact()
    else {
        return;
    };

    if open_lutris {
        Command::new("lutris").spawn().ok();
    }
}

fn setup_wow_complete() {
    println!("🎯 World of Warcraft Complete Setup");
    println!("===================================\n");

    println!("🎮 Setting up World of Warcraft with optimal configuration");

    println!("🔍 System Requirements Check:");
    check_wow_system_requirements();

    let Ok(proceed) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Proceed with WoW setup?")
        .default(true)
        .interact()
    else {
        return;
    };

    if !proceed {
        return;
    }

    ensure_lutris_ready_for_wow();
    install_battlenet_for_wow();
    optimize_system_for_wow();
    configure_wow_wine_prefix();
    setup_wow_graphics_layers();
    install_wow_optimizations();

    println!("\n✅ World of Warcraft setup completed!");
    println!("🎮 Next steps:");
    println!("   1. Launch Battle.net from Lutris");
    println!("   2. Log in and install World of Warcraft");
    println!("   3. Use the optimization settings we configured");
    println!("   4. Run 'ghostctl repair wine' if you encounter issues");
}

fn check_wow_system_requirements() {
    let meminfo = std::fs::read_to_string("/proc/meminfo");
    if let Ok(content) = meminfo {
        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Ok(mem_kb) = parts[1].parse::<u64>() {
                    let mem_gb = mem_kb / 1024 / 1024;
                    if mem_gb >= 8 {
                        println!("  ✅ RAM: {} GB (sufficient)", mem_gb);
                    } else {
                        println!("  ⚠️ RAM: {} GB (8GB+ recommended)", mem_gb);
                    }
                }
                break;
            }
        }
    }

    let gpu_check = Command::new("lspci").args(&["-k"]).output();
    if let Ok(out) = gpu_check {
        let output = String::from_utf8_lossy(&out.stdout);
        if output.contains("NVIDIA") {
            println!("  ✅ GPU: NVIDIA detected (excellent for WoW)");
        } else if output.contains("AMD") {
            println!("  ✅ GPU: AMD detected (good for WoW)");
        } else {
            println!("  ⚠️ GPU: Integrated graphics may struggle with WoW");
        }
    }

    let wine_check = Command::new("wine").arg("--version").output();
    match wine_check {
        Ok(out) => {
            let output_string = String::from_utf8_lossy(&out.stdout);
            println!("  ✅ Wine: {}", output_string.trim());
        }
        Err(_) => println!("  ❌ Wine: Not installed"),
    }

    let dxvk_available = Command::new("which")
        .arg("setup_dxvk")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if dxvk_available {
        println!("  ✅ DXVK: Available");
    } else {
        println!("  ⚠️ DXVK: Not found (will be installed)");
    }
}

fn ensure_lutris_ready_for_wow() {
    println!("\n🔧 Preparing Lutris for World of Warcraft");
    println!("=========================================");

    let lutris_available = Command::new("which")
        .arg("lutris")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !lutris_available {
        println!("📥 Installing Lutris...");
        install_lutris_fresh();
    } else {
        println!("✅ Lutris is installed");
    }

    let wow_deps = [
        "wine",
        "winetricks",
        "dxvk",
        "lib32-vulkan-icd-loader",
        "lib32-mesa",
        "lib32-nvidia-utils",
        "giflib",
        "lib32-giflib",
        "libpng",
        "lib32-libpng",
        "libldap",
        "lib32-libldap",
        "gnutls",
        "lib32-gnutls",
        "mpg123",
        "lib32-mpg123",
        "openal",
        "lib32-openal",
        "v4l-utils",
        "lib32-v4l-utils",
        "libpulse",
        "lib32-libpulse",
        "alsa-plugins",
        "lib32-alsa-plugins",
        "alsa-lib",
        "lib32-alsa-lib",
        "libjpeg-turbo",
        "lib32-libjpeg-turbo",
        "libxcomposite",
        "lib32-libxcomposite",
        "libxinerama",
        "lib32-libxinerama",
        "ncurses",
        "lib32-ncurses",
        "opencl-icd-loader",
        "lib32-opencl-icd-loader",
        "libxslt",
        "lib32-libxslt",
        "libva",
        "lib32-libva",
        "gtk3",
        "lib32-gtk3",
        "gst-plugins-base-libs",
        "lib32-gst-plugins-base-libs",
        "vulkan-icd-loader",
    ];

    println!("📦 Installing WoW dependencies...");
    for dep in &wow_deps {
        let install_attempts = [
            vec!["pacman", "-S", "--noconfirm", dep],
            vec!["yay", "-S", "--noconfirm", dep],
        ];

        for cmd in &install_attempts {
            let cmd_available = Command::new("which")
                .arg(cmd[0])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            if cmd_available {
                Command::new(cmd[0]).args(&cmd[1..]).status().ok();
                break;
            }
        }
    }

    println!("✅ Dependencies installation attempted");
}

fn install_battlenet_for_wow() {
    println!("\n⚔️ Installing Battle.net for World of Warcraft");
    println!("==============================================");

    let list_result = Command::new("lutris").args(&["--list-games"]).output();
    if let Ok(out) = list_result {
        let games = String::from_utf8_lossy(&out.stdout);
        if games.to_lowercase().contains("battle.net") {
            println!("✅ Battle.net already installed in Lutris");
            return;
        }
    }

    println!("📥 Setting up Battle.net installation...");

    let lutris_script = r#"
{
    "game": {
        "exe": "drive_c/Program Files (x86)/Battle.net/Battle.net Launcher.exe",
        "prefix": "$GAMEDIR"
    },
    "installer": [
        {
            "task": {
                "arch": "win64",
                "description": "Create Wine prefix",
                "name": "create_prefix",
                "prefix": "$GAMEDIR"
            }
        },
        {
            "task": {
                "app": "corefonts vcrun2017 vcrun2019",
                "description": "Install Visual C++ Redistributables and fonts",
                "name": "winetricks",
                "prefix": "$GAMEDIR"
            }
        },
        {
            "task": {
                "description": "Download Battle.net installer",
                "name": "download",
                "url": "https://www.battle.net/download/getInstallerForGame?os=win&locale=enUS&version=LIVE&gameProgram=BATTLENET_APP"
            }
        },
        {
            "task": {
                "args": "/S",
                "description": "Install Battle.net",
                "executable": "$CACHE/Battle.net-Setup.exe",
                "name": "wineexec",
                "prefix": "$GAMEDIR"
            }
        }
    ],
    "name": "Battle.net",
    "runner": "wine",
    "slug": "battlenet-wow-setup",
    "version": "WoW Optimized Setup",
    "wine": {
        "dxvk": true,
        "esync": true,
        "fsync": true,
        "Desktop": false,
        "version": "lutris-ge-8-26-x86_64"
    }
}
"#;

    let script_path = "/tmp/battlenet-wow-lutris.json";
    std::fs::write(script_path, lutris_script).ok();

    println!("🎮 Installing Battle.net via Lutris...");
    let install_result = Command::new("lutris").args(&["-i", script_path]).status();

    match install_result {
        Ok(_) => {
            println!("✅ Battle.net installation initiated");
            println!("💡 Follow the Lutris installer prompts");
        }
        Err(_) => {
            println!("⚠️ Automatic installation failed");
            println!("💡 Manual setup required:");
            println!("   1. Open Lutris");
            println!("   2. Click '+' to add a game");
            println!("   3. Search for 'Battle.net' in Lutris.net");
            println!("   4. Install the Battle.net script");
        }
    }

    fs::remove_file(script_path).ok();
}

fn optimize_system_for_wow() {
    println!("\n🚀 Optimizing System for World of Warcraft");
    println!("==========================================");

    let gamemode_available = Command::new("which")
        .arg("gamemoded")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if gamemode_available {
        println!("✅ GameMode detected");

        Command::new("systemctl")
            .args(&["--user", "enable", "gamemoded"])
            .status()
            .ok();

        Command::new("systemctl")
            .args(&["--user", "start", "gamemoded"])
            .status()
            .ok();

        println!("🎮 GameMode enabled for better performance");
    } else {
        println!("📦 Installing GameMode...");
        Command::new("sudo")
            .args(&["pacman", "-S", "--noconfirm", "gamemode"])
            .status()
            .ok();
    }

    println!("⚡ Setting CPU governor to performance mode...");
    Command::new("sudo")
        .args(&["cpupower", "frequency-set", "-g", "performance"])
        .status()
        .ok();

    println!("💾 Optimizing memory management...");
    Command::new("sudo")
        .args(&["sysctl", "vm.swappiness=10"])
        .status()
        .ok();

    println!("💿 Optimizing I/O scheduler...");
    let io_schedulers = ["mq-deadline", "kyber", "bfq"];
    for scheduler in &io_schedulers {
        let devices = ["sda", "sdb", "nvme0n1", "nvme1n1"];
        for device in &devices {
            let scheduler_path = format!("/sys/block/{}/queue/scheduler", device);
            if Path::new(&scheduler_path).exists() {
                Command::new("sudo")
                    .args(&[
                        "bash",
                        "-c",
                        &format!("echo {} > {}", scheduler, scheduler_path),
                    ])
                    .status()
                    .ok();
            }
        }
    }

    println!("✅ System optimizations applied");
}

fn configure_wow_wine_prefix() {
    println!("\n🍷 Configuring Wine Prefix for WoW");
    println!("==================================");

    let wow_prefix = format!("{}/.local/share/lutris/prefixes/battlenet", get_home_dir());

    println!("📁 WoW Wine prefix: {}", wow_prefix);

    if Path::new(&wow_prefix).exists() {
        println!("✅ Wine prefix exists");

        let wine_configs = [
            ("Windows", "win10"),
            ("Renderer", "opengl"),
            ("AudioDriver", "pulse"),
        ];

        for (setting, value) in &wine_configs {
            println!("🔧 Setting {} to {}", setting, value);

            let reg_cmd = format!(
                "WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Software\\Wine\\{}' /v Version /d {} /f",
                wow_prefix, setting, value
            );

            Command::new("bash").arg("-c").arg(&reg_cmd).status().ok();
        }

        println!("📦 Installing essential Windows components...");
        let winetricks_apps = [
            "corefonts",
            "vcrun2019",
            "vcrun2017",
            "vcrun2015",
            "d3dx9",
            "d3dx10",
            "d3dx11_43",
            "dxvk",
        ];

        for app in &winetricks_apps {
            println!("  📦 Installing {}", app);
            Command::new("env")
                .env("WINEPREFIX", &wow_prefix)
                .args(&["winetricks", "--unattended", app])
                .status()
                .ok();
        }

        println!("✅ Wine prefix configured for WoW");
    } else {
        println!("⚠️ Wine prefix not found - it will be created when Battle.net is installed");
    }
}

fn setup_wow_graphics_layers() {
    println!("\n🎨 Setting up Graphics Layers for WoW");
    println!("=====================================");

    let wow_prefix = format!("{}/.local/share/lutris/prefixes/battlenet", get_home_dir());

    println!("📦 Installing DXVK for DirectX 9/10/11 support...");

    if Path::new(&wow_prefix).exists() {
        Command::new("env")
            .env("WINEPREFIX", &wow_prefix)
            .args(&["setup_dxvk", "install"])
            .status()
            .ok();
    }

    let dxvk_config = format!(
        "{}/.local/share/lutris/prefixes/battlenet/dxvk.conf",
        get_home_dir()
    );
    let dxvk_settings = r#"
# DXVK Configuration for World of Warcraft
# Optimized for performance and stability

# Enable state cache for better performance
dxvk.enableStateCache = True

# Reduce stuttering
dxvk.numCompilerThreads = 0

# Memory optimizations for WoW
dxvk.maxDeviceMemory = 0
dxvk.maxSharedMemory = 0

# Frame rate optimizations
dxvk.maxFrameRate = 0

# HUD (disable for better performance)
dxvk.hud =

# Logging (disable for better performance)
dxvk.logLevel = none
"#;

    std::fs::write(&dxvk_config, dxvk_settings).ok();
    println!("✅ DXVK configured for WoW");

    println!("📦 Setting up VKD3D for DirectX 12 support...");

    let vkd3d_available = Command::new("which")
        .arg("setup_vkd3d")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if vkd3d_available {
        if Path::new(&wow_prefix).exists() {
            Command::new("env")
                .env("WINEPREFIX", &wow_prefix)
                .args(&["setup_vkd3d", "install"])
                .status()
                .ok();
        }
        println!("✅ VKD3D installed");
    } else {
        println!("⚠️ VKD3D not available - install manually if needed");
    }

    println!("🎨 Graphics layers configured for optimal WoW performance");
}

fn install_wow_optimizations() {
    println!("\n⚡ Installing WoW-Specific Optimizations");
    println!("=======================================");

    println!("📊 Installing MangoHud for performance monitoring...");
    Command::new("sudo")
        .args(&["pacman", "-S", "--noconfirm", "mangohud"])
        .status()
        .ok();

    let mangohud_dir = format!("{}/.config/MangoHud", get_home_dir());
    fs::create_dir_all(&mangohud_dir).ok();

    let mangohud_config = format!("{}/MangoHud.conf", mangohud_dir);
    let hud_settings = r#"
# MangoHud configuration for World of Warcraft
# Optimized for minimal performance impact

# Basic monitoring
fps
frame_timing=0
cpu_stats
gpu_stats
ram
vram

# Position and appearance
position=top-left
background_alpha=0.5
font_size=18
text_color=FFFFFF
background_color=020202

# Logging (optional)
output_folder=~/Documents/mangohud-logs
log_duration=60

# WoW-specific optimizations
vsync=1
frame_limit=144
"#;

    std::fs::write(&mangohud_config, hud_settings).ok();
    println!("✅ MangoHud configured for WoW");

    let launch_script = format!("{}/.local/bin/wow-optimized", get_home_dir());
    fs::create_dir_all(format!("{}/.local/bin", get_home_dir())).ok();

    let script_content = r#"#!/bin/bash
# World of Warcraft Optimized Launch Script

export DXVK_HUD=compiler
export DXVK_STATE_CACHE_PATH="$HOME/.cache/dxvk-state-cache"
export __GL_SHADER_DISK_CACHE=1
export __GL_SHADER_DISK_CACHE_PATH="$HOME/.cache/nvidia-shader-cache"

# CPU optimizations
export WINE_CPU_TOPOLOGY=4:2
export OMP_NUM_THREADS=4

# Memory optimizations
export MALLOC_CHECK_=0

# Launch with GameMode and MangoHud
exec gamemoderun mangohud lutris lutris:rungame/battlenet
"#;

    std::fs::write(&launch_script, script_content).ok();
    Command::new("chmod")
        .args(&["+x", &launch_script])
        .status()
        .ok();

    println!("✅ WoW launch script created: {}", launch_script);

    println!("🔧 Applying WoW-specific Wine tweaks...");

    let wow_tweaks_script = r#"#!/bin/bash
# WoW-specific Wine registry tweaks

WINEPREFIX="$HOME/.local/share/lutris/prefixes/battlenet"

# Disable Wine debugging for performance
wine reg add 'HKEY_CURRENT_USER\Software\Wine\Debug' /v 'LogLevel' /d '0' /f

# Set Windows version to Windows 10
wine reg add 'HKEY_CURRENT_USER\Software\Wine' /v 'Version' /d 'win10' /f

# Audio latency optimizations
wine reg add 'HKEY_CURRENT_USER\Software\Wine\DirectSound' /v 'DefaultBitsPerSample' /d '16' /f
wine reg add 'HKEY_CURRENT_USER\Software\Wine\DirectSound' /v 'DefaultSampleRate' /d '44100' /f

# DirectX optimizations
wine reg add 'HKEY_CURRENT_USER\Software\Wine\Direct3D' /v 'DirectDrawRenderer' /d 'opengl' /f
wine reg add 'HKEY_CURRENT_USER\Software\Wine\Direct3D' /v 'Multisampling' /d 'enabled' /f

echo "WoW Wine tweaks applied successfully"
"#;

    let tweaks_script = "/tmp/wow-wine-tweaks.sh";
    std::fs::write(tweaks_script, wow_tweaks_script).ok();
    Command::new("chmod")
        .args(&["+x", tweaks_script])
        .status()
        .ok();
    Command::new("bash").arg(tweaks_script).status().ok();
    fs::remove_file(tweaks_script).ok();

    println!("✅ All WoW optimizations installed and configured");
}

fn setup_diablo4_complete() {
    println!("⚔️ Diablo 4 Complete Setup");
    println!("===========================\n");

    println!("🎮 Setting up Diablo 4 with optimal configuration");

    println!("🔍 System Requirements Check:");
    check_diablo4_system_requirements();

    let Ok(proceed) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Proceed with Diablo 4 setup?")
        .default(true)
        .interact()
    else {
        return;
    };

    if !proceed {
        return;
    }

    ensure_battlenet_ready_for_d4();
    optimize_system_for_d4();
    configure_d4_wine_prefix();
    setup_d4_graphics_optimizations();
    setup_d4_anticheat();

    println!("\n✅ Diablo 4 setup completed!");
    println!("⚔️ Next steps:");
    println!("   1. Launch Battle.net from Lutris");
    println!("   2. Install Diablo 4 from your Battle.net library");
    println!("   3. Use our optimized launch settings");
    println!("   4. Monitor performance with MangoHud");
}

fn check_diablo4_system_requirements() {
    let meminfo = std::fs::read_to_string("/proc/meminfo");
    if let Ok(content) = meminfo {
        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Ok(mem_kb) = parts[1].parse::<u64>() {
                    let mem_gb = mem_kb / 1024 / 1024;
                    if mem_gb >= 16 {
                        println!("  ✅ RAM: {} GB (excellent)", mem_gb);
                    } else if mem_gb >= 8 {
                        println!("  ⚠️ RAM: {} GB (minimum, 16GB recommended)", mem_gb);
                    } else {
                        println!("  ❌ RAM: {} GB (insufficient for D4)", mem_gb);
                    }
                }
                break;
            }
        }
    }

    let gpu_check = Command::new("lspci").args(&["-k"]).output();
    if let Ok(out) = gpu_check {
        let output = String::from_utf8_lossy(&out.stdout);
        if output.contains("RTX") || output.contains("GTX 1060") {
            println!("  ✅ GPU: Modern NVIDIA GPU detected (excellent)");
        } else if output.contains("RX") {
            println!("  ✅ GPU: AMD GPU detected (good, ensure latest drivers)");
        } else if output.contains("NVIDIA") {
            println!("  ⚠️ GPU: Older NVIDIA GPU (may need settings adjustment)");
        } else {
            println!("  ❌ GPU: May not meet D4 requirements");
        }
    }

    let nvidia_check = Command::new("nvidia-smi").output();
    if let Ok(out) = nvidia_check {
        let output = String::from_utf8_lossy(&out.stdout);
        if output.contains("Driver Version:") {
            for line in output.lines() {
                if line.contains("Driver Version:") {
                    println!("  📋 {}", line.trim());
                    break;
                }
            }
        }
    }

    let disk_check = Command::new("df").args(&["-h", "."]).output();
    if let Ok(out) = disk_check {
        let output = String::from_utf8_lossy(&out.stdout);
        for line in output.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                println!("  💿 Available storage: {} (D4 requires ~90GB)", parts[3]);
                break;
            }
        }
    }
}

fn ensure_battlenet_ready_for_d4() {
    println!("\n⚔️ Ensuring Battle.net is ready for Diablo 4");
    println!("=============================================");

    let list_result = Command::new("lutris").args(&["--list-games"]).output();
    if let Ok(out) = list_result {
        let games = String::from_utf8_lossy(&out.stdout);
        if games.to_lowercase().contains("battle.net") {
            println!("✅ Battle.net found in Lutris");
            return;
        }
    }

    println!("📥 Battle.net not found - installing...");
    install_battlenet_for_wow();

    let d4_deps = [
        "lib32-vulkan-mesa-layers",
        "vulkan-mesa-layers",
        "lib32-opencl-mesa",
        "opencl-mesa",
        "lib32-libva-mesa-driver",
        "libva-mesa-driver",
    ];

    for dep in &d4_deps {
        Command::new("sudo")
            .args(&["pacman", "-S", "--noconfirm", dep])
            .status()
            .ok();
    }

    println!("✅ Battle.net and D4 dependencies ready");
}

fn optimize_system_for_d4() {
    println!("\n🚀 Optimizing System for Diablo 4");
    println!("=================================");

    println!("⚡ Applying high-performance settings...");

    Command::new("sudo")
        .args(&["cpupower", "frequency-set", "-g", "performance"])
        .status()
        .ok();

    let Ok(disable_mitigations) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Disable CPU security mitigations for maximum performance? (Reduces security)")
        .default(false)
        .interact()
    else {
        return;
    };

    if disable_mitigations {
        println!("⚠️ Adding mitigations=off to kernel parameters");
        println!(
            "💡 Edit /etc/default/grub and add 'mitigations=off' to GRUB_CMDLINE_LINUX_DEFAULT"
        );
        println!("💡 Run 'sudo grub-mkconfig -o /boot/grub/grub.cfg' after editing");
    }

    Command::new("sudo")
        .args(&["sysctl", "kernel.sched_migration_cost_ns=5000000"])
        .status()
        .ok();

    Command::new("sudo")
        .args(&["sysctl", "kernel.sched_autogroup_enabled=0"])
        .status()
        .ok();

    Command::new("sudo")
        .args(&["sysctl", "vm.swappiness=1"])
        .status()
        .ok();

    Command::new("sudo")
        .args(&["sysctl", "vm.vfs_cache_pressure=50"])
        .status()
        .ok();

    println!("✅ High-performance optimizations applied");
}

fn configure_d4_wine_prefix() {
    println!("\n🍷 Configuring Wine Prefix for Diablo 4");
    println!("=======================================");

    let d4_prefix = format!("{}/.local/share/lutris/prefixes/diablo4", get_home_dir());

    if !Path::new(&d4_prefix).exists() {
        println!("📁 Creating dedicated D4 Wine prefix...");
        fs::create_dir_all(&d4_prefix).ok();

        Command::new("env")
            .env("WINEPREFIX", &d4_prefix)
            .args(&["winecfg"])
            .status()
            .ok();
    }

    let d4_winetricks = [
        "vcrun2022",
        "vcrun2019",
        "dxvk",
        "vkd3d",
        "corefonts",
        "d3dx9",
        "d3dx10",
        "d3dx11_43",
        "d3dx12",
        "xinput",
        "xaudio2_9",
    ];

    for app in &d4_winetricks {
        println!("📦 Installing {} for D4", app);
        Command::new("env")
            .env("WINEPREFIX", &d4_prefix)
            .args(&["winetricks", "--unattended", app])
            .status()
            .ok();
    }

    let wine_settings = [
        ("Version", "win10"),
        ("AudioDriver", "pulse"),
        ("DirectDrawRenderer", "vulkan"),
        ("MaxVersionGL", "4.6"),
    ];

    for (key, value) in &wine_settings {
        let reg_cmd = format!(
            "WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Software\\Wine\\{}' /v Version /d {} /f",
            d4_prefix, key, value
        );

        Command::new("bash").arg("-c").arg(&reg_cmd).status().ok();
    }

    println!("✅ D4 Wine prefix configured");
}

fn setup_d4_graphics_optimizations() {
    println!("\n🎨 Setting up Diablo 4 Graphics Optimizations");
    println!("==============================================");

    let d4_prefix = format!("{}/.local/share/lutris/prefixes/diablo4", get_home_dir());

    let dxvk_config_path = format!("{}/dxvk.conf", d4_prefix);
    let dxvk_config = r#"
# DXVK Configuration for Diablo 4
# Optimized for high performance and visual quality

# Enable async pipeline compilation for smoother gameplay
dxvk.enableAsync = True

# Increase compiler thread count for faster shader compilation
dxvk.numCompilerThreads = 0

# Memory optimizations
dxvk.maxDeviceMemory = 0
dxvk.maxSharedMemory = 0

# Frame pacing
dxvk.maxFrameRate = 144

# State cache for better performance
dxvk.enableStateCache = True
dxvk.stateCacheSize = 256

# Reduce stuttering
dxvk.presentInterval = 1

# Graphics quality optimizations
dxvk.samplerAnisotropy = 16

# Disable HUD for maximum performance
dxvk.hud =

# Logging
dxvk.logLevel = none
dxvk.logPath = none
"#;

    std::fs::write(&dxvk_config_path, dxvk_config).ok();
    println!("✅ DXVK optimized for D4");

    let vkd3d_config_path = format!("{}/vkd3d.conf", d4_prefix);
    let vkd3d_config = r#"
# VKD3D Configuration for Diablo 4 DirectX 12
# Optimized for performance and compatibility

# Enable debug layer (disable in production)
VKD3D_DEBUG = none

# Memory allocation
VKD3D_VULKAN_DEVICE = 0

# Shader optimizations
VKD3D_SHADER_DEBUG = none

# Feature level
VKD3D_FEATURE_LEVEL = 12_1
"#;

    std::fs::write(&vkd3d_config_path, vkd3d_config).ok();
    println!("✅ VKD3D configured for D4 DirectX 12");

    let mangohud_dir = format!("{}/.config/MangoHud", get_home_dir());
    fs::create_dir_all(&mangohud_dir).ok();

    let d4_mangohud_config = format!("{}/Diablo4.conf", mangohud_dir);
    let hud_config = r#"
# MangoHud configuration for Diablo 4
# High performance monitoring with minimal overhead

# Core metrics
fps
frame_timing
cpu_stats
gpu_stats
cpu_temp
gpu_temp
ram
vram

# Position and style
position=top-right
background_alpha=0.4
font_size=16
text_color=FF6B35
background_color=000000

# Performance optimizations
vsync=0
fps_limit=144
frame_limit=144

# Logging for troubleshooting
log_duration=0
autostart_log=0
"#;

    std::fs::write(&d4_mangohud_config, hud_config).ok();
    println!("✅ MangoHud configured for D4");

    let d4_launch_script = format!("{}/.local/bin/diablo4-launch", get_home_dir());
    let launch_script = r#"#!/bin/bash
# Diablo 4 Optimized Launch Script

# Graphics optimizations
export DXVK_ASYNC=1
export DXVK_STATE_CACHE_PATH="$HOME/.cache/dxvk-d4"
export VKD3D_CONFIG=dxvk.conf

# NVIDIA optimizations (if applicable)
export __GL_SHADER_DISK_CACHE=1
export __GL_SHADER_DISK_CACHE_PATH="$HOME/.cache/nv-shader-d4"
export __GL_THREADED_OPTIMIZATIONS=1
export __GL_SYNC_TO_VBLANK=0

# CPU optimizations
export WINE_CPU_TOPOLOGY=8:4  # Adjust based on your CPU
export OMP_NUM_THREADS=8

# Memory optimizations
export MALLOC_CHECK_=0
export WINEDEBUG=-all

# Launch with all optimizations
exec gamemoderun mangohud --dlsym --config Diablo4 lutris lutris:rungame/diablo-iv
"#;

    std::fs::write(&d4_launch_script, launch_script).ok();
    Command::new("chmod")
        .args(&["+x", &d4_launch_script])
        .status()
        .ok();

    println!("✅ D4 launch script created: {}", d4_launch_script);
}

fn setup_d4_anticheat() {
    println!("\n🛡️ Setting up Anti-cheat Compatibility for Diablo 4");
    println!("====================================================");

    println!("⚠️ Diablo 4 uses proprietary anti-cheat systems");
    println!("🔧 Configuring Wine for anti-cheat compatibility...");

    let d4_prefix = format!("{}/.local/share/lutris/prefixes/diablo4", get_home_dir());

    let anticheat_regs = [
        (
            "HKEY_CURRENT_USER\\Software\\Wine\\DllOverrides",
            "winebus.sys",
            "disabled",
        ),
        (
            "HKEY_CURRENT_USER\\Software\\Wine\\DllOverrides",
            "winehid.sys",
            "disabled",
        ),
        (
            "HKEY_CURRENT_USER\\Software\\Wine\\DllOverrides",
            "kernel32",
            "native,builtin",
        ),
        (
            "HKEY_CURRENT_USER\\Software\\Wine\\DllOverrides",
            "ntdll",
            "native,builtin",
        ),
    ];

    for (hive, key, value) in &anticheat_regs {
        let reg_cmd = format!(
            "WINEPREFIX={} wine reg add '{}' /v '{}' /d '{}' /f",
            d4_prefix, hive, key, value
        );

        Command::new("bash").arg("-c").arg(&reg_cmd).status().ok();
    }

    println!("📦 Installing anti-cheat compatibility components...");
    let anticheat_components = ["vcrun2022", "dotnetdesktop6", "msxml3", "msxml6", "crypt32"];

    for component in &anticheat_components {
        Command::new("env")
            .env("WINEPREFIX", &d4_prefix)
            .args(&["winetricks", "--unattended", component])
            .status()
            .ok();
    }

    println!("✅ Anti-cheat compatibility configured");

    println!("\n⚠️ Important Notes for Diablo 4:");
    println!("   • Anti-cheat detection may still occur");
    println!("   • Test with offline/single-player first");
    println!("   • Keep Wine and DXVK updated");
    println!("   • Monitor Lutris forums for compatibility updates");
    println!("   • Consider using a dedicated Wine prefix for D4");
}

fn lutris_configuration() {
    let options = [
        "⚙️ General Lutris Settings",
        "🎮 Global Game Settings",
        "🍷 Default Wine Settings",
        "📁 Directory Configuration",
        "🌐 Online Services Setup",
        "⬅️ Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🔧 Lutris Configuration")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => configure_general_lutris(),
        1 => configure_global_game_settings(),
        2 => configure_default_wine(),
        3 => configure_lutris_directories(),
        4 => configure_online_services(),
        _ => {}
    }
}

fn configure_general_lutris() {
    println!("⚙️ General Lutris Configuration");
    println!("===============================\n");

    println!("💡 General configuration is best done through Lutris GUI");
    println!("🔧 Key settings to configure:");
    println!("   • Default installation directory");
    println!("   • Theme and appearance");
    println!("   • Update preferences");
    println!("   • Library management");
    println!("   • Notification settings");

    let Ok(open_lutris) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Open Lutris preferences?")
        .default(true)
        .interact()
    else {
        return;
    };

    if open_lutris {
        let pref_result = Command::new("lutris").args(&["--preferences"]).spawn();

        match pref_result {
            Ok(_) => println!("✅ Lutris preferences opened"),
            Err(_) => {
                Command::new("lutris").spawn().ok();
                println!("✅ Lutris opened - access preferences via menu");
            }
        }
    }
}

fn configure_global_game_settings() {
    println!("🎮 Global Game Settings");
    println!("======================\n");

    println!("🔧 Recommended global game settings:");
    println!("   • Enable GameMode integration");
    println!("   • Configure default Wine version");
    println!("   • Set up MangoHud integration");
    println!("   • Configure controller support");

    let lutris_config_dir = format!("{}/.config/lutris", get_home_dir());
    fs::create_dir_all(&lutris_config_dir).ok();

    let global_config = format!("{}/system.yml", lutris_config_dir);
    let config_content = r#"# Lutris Global System Configuration
# Gaming optimizations

system:
  # Performance
  gamemode: true
  mangohud: true

  # Audio
  pulse_latency: 60

  # Display
  reset_pulse: false
  use_us_layout: false

  # System
  disable_compositor: true
  disable_screen_saver: true

# Default Wine configuration
wine:
  # Performance
  gamemode: true
  mangohud: true

  # Compatibility
  version: "lutris-ge-8-26-x86_64"
  esync: true
  fsync: true

  # Graphics
  dxvk: true
  dxvk_nvapi: true
  vkd3d: true
"#;

    std::fs::write(&global_config, config_content).ok();
    println!("✅ Global configuration written to {}", global_config);

    println!("\n💡 To apply these settings:");
    println!("   1. Restart Lutris");
    println!("   2. Check Preferences → System");
    println!("   3. Verify Wine runner settings");
}

fn configure_default_wine() {
    println!("🍷 Default Wine Configuration");
    println!("=============================\n");

    let wine_config_script = r#"#!/bin/bash
# Default Wine configuration for gaming

# Create default Wine prefix if it doesn't exist
export WINEPREFIX="$HOME/.wine"

if [ ! -d "$WINEPREFIX" ]; then
    echo "Creating default Wine prefix..."
    winecfg
fi

# Apply gaming optimizations
echo "Applying Wine gaming optimizations..."

# Set Windows version to Windows 10
wine reg add 'HKEY_CURRENT_USER\Software\Wine' /v 'Version' /d 'win10' /f

# Audio optimizations
wine reg add 'HKEY_CURRENT_USER\Software\Wine\DirectSound' /v 'DefaultBitsPerSample' /d '16' /f
wine reg add 'HKEY_CURRENT_USER\Software\Wine\DirectSound' /v 'DefaultSampleRate' /d '44100' /f

# Graphics optimizations
wine reg add 'HKEY_CURRENT_USER\Software\Wine\Direct3D' /v 'DirectDrawRenderer' /d 'opengl' /f
wine reg add 'HKEY_CURRENT_USER\Software\Wine\Direct3D' /v 'Multisampling' /d 'enabled' /f
wine reg add 'HKEY_CURRENT_USER\Software\Wine\Direct3D' /v 'OffscreenRenderingMode' /d 'backbuffer' /f

# Disable Wine debugging for performance
wine reg add 'HKEY_CURRENT_USER\Software\Wine\Debug' /v 'LogLevel' /d '0' /f

# Font smoothing
wine reg add 'HKEY_CURRENT_USER\Control Panel\Desktop' /v 'FontSmoothing' /d '2' /f
wine reg add 'HKEY_CURRENT_USER\Control Panel\Desktop' /v 'FontSmoothingType' /d '2' /f

echo "Default Wine configuration completed"
"#;

    let config_script = "/tmp/configure-default-wine.sh";
    std::fs::write(config_script, wine_config_script).ok();
    Command::new("chmod")
        .args(&["+x", config_script])
        .status()
        .ok();

    let Ok(apply_config) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Apply default Wine gaming configuration?")
        .default(true)
        .interact()
    else {
        fs::remove_file(config_script).ok();
        return;
    };

    if apply_config {
        Command::new("bash").arg(config_script).status().ok();
        println!("✅ Default Wine configuration applied");
    }

    fs::remove_file(config_script).ok();
}

fn configure_lutris_directories() {
    println!("📁 Lutris Directory Configuration");
    println!("=================================\n");

    let home = get_home_dir();
    let lutris_dirs = [
        ("Games", format!("{}/Games/Lutris", home)),
        ("Prefixes", format!("{}/.local/share/lutris/prefixes", home)),
        ("Runners", format!("{}/.local/share/lutris/runners", home)),
        ("Cache", format!("{}/.cache/lutris", home)),
        ("Config", format!("{}/.config/lutris", home)),
    ];

    println!("📋 Current Lutris Directories:");
    for (name, path) in &lutris_dirs {
        let exists = Path::new(path).exists();
        let size = if exists {
            get_directory_size(path) / 1024 / 1024 / 1024
        } else {
            0
        };

        println!(
            "  📁 {}: {} ({})",
            name,
            path,
            if exists {
                format!("{} GB", size)
            } else {
                "Not found".to_string()
            }
        );
    }

    let Ok(setup_dirs) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Create missing directories?")
        .default(true)
        .interact()
    else {
        return;
    };

    if setup_dirs {
        for (_name, path) in &lutris_dirs {
            if !Path::new(path).exists() {
                fs::create_dir_all(path).ok();
                println!("✅ Created: {}", path);
            }
        }
    }

    let Ok(change_games_dir) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Change default games installation directory?")
        .default(false)
        .interact()
    else {
        return;
    };

    if change_games_dir {
        let default_dir = format!("{}/Games/Lutris", home);
        let Ok(new_games_dir): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter new games directory")
            .default(default_dir)
            .interact()
        else {
            return;
        };

        fs::create_dir_all(&new_games_dir).ok();
        println!("✅ Games directory set to: {}", new_games_dir);
        println!("💡 Update this in Lutris Preferences → System → Default installation folder");
    }
}

fn configure_online_services() {
    println!("🌐 Online Services Configuration");
    println!("================================\n");

    println!("🔐 Lutris supports integration with:");
    println!("   • Steam");
    println!("   • GOG");
    println!("   • Epic Games Store");
    println!("   • Origin/EA App");
    println!("   • Ubisoft Connect");
    println!("   • Humble Bundle");

    println!("\n💡 To configure online services:");
    println!("   1. Open Lutris");
    println!("   2. Go to Sources in the sidebar");
    println!("   3. Enable and authenticate services");
    println!("   4. Sync your game libraries");

    let service_setup_options = [
        "🔧 Configure Battle.net",
        "🎮 Configure Steam compatibility",
        "📦 Configure GOG integration",
        "🏪 Configure Epic Games",
        "🌐 Open Lutris for manual setup",
        "⬅️ Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select service to configure")
        .items(&service_setup_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => {
            println!("⚔️ Battle.net configuration:");
            println!("   • Already configured in WoW/D4 setup");
            println!("   • Access via Lutris game library");
            println!("   • Ensure latest Wine-GE for compatibility");
        }
        1 => {
            println!("🎮 Steam compatibility:");
            println!("   • Install Steam via Lutris or natively");
            println!("   • Use Proton for Windows games");
            println!("   • Enable Steam Play for all titles");
        }
        2 => {
            println!("📦 GOG integration:");
            println!("   • Connect GOG account in Lutris Sources");
            println!("   • Download games directly through Lutris");
            println!("   • Use Wine for Windows GOG games");
        }
        3 => {
            println!("🏪 Epic Games configuration:");
            println!("   • Install via Lutris script");
            println!("   • Claim free games regularly");
            println!("   • Use latest Wine-GE for compatibility");
        }
        4 => {
            Command::new("lutris").spawn().ok();
            println!("✅ Lutris opened for manual service setup");
        }
        _ => {}
    }
}

fn lutris_cleanup_maintenance() {
    let options = [
        "🧹 Clean Lutris Cache",
        "🗑️ Remove Unused Prefixes",
        "📦 Clean Old Runners",
        "🔍 Check Installation Health",
        "📊 Storage Analysis",
        "🔧 Reset Lutris Configuration",
        "⬅️ Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🧹 Lutris Cleanup & Maintenance")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => clean_lutris_cache(),
        1 => clean_unused_lutris_prefixes(),
        2 => clean_old_lutris_runners(),
        3 => check_lutris_health(),
        4 => lutris_storage_analysis(),
        5 => reset_lutris_config(),
        _ => {}
    }
}

fn clean_lutris_cache() {
    println!("🧹 Cleaning Lutris Cache");
    println!("========================\n");

    let cache_dirs = [
        ("Main Cache", format!("{}/.cache/lutris", get_home_dir())),
        ("Wine Cache", format!("{}/.cache/wine", get_home_dir())),
        (
            "DXVK Cache",
            format!("{}/.cache/dxvk-state-cache", get_home_dir()),
        ),
        ("VKD3D Cache", format!("{}/.cache/vkd3d", get_home_dir())),
        ("Temp Files", "/tmp/lutris*".to_string()),
    ];

    let mut total_cleaned = 0u64;

    for (name, path) in &cache_dirs {
        if path.contains("*") {
            let find_result = Command::new("find")
                .args(&["/tmp", "-name", "lutris*", "-type", "f"])
                .output();

            if let Ok(out) = find_result {
                for file in String::from_utf8_lossy(&out.stdout).lines() {
                    if Path::new(file).exists() {
                        let size = get_directory_size(file);
                        total_cleaned += size;
                        fs::remove_file(file).ok();
                    }
                }
                println!("🧹 Cleaned {}: Temp files", name);
            }
        } else if Path::new(path).exists() {
            let before_size = get_directory_size(path);

            Command::new("rm")
                .args(&["-rf", &format!("{}/*", path)])
                .status()
                .ok();

            let after_size = get_directory_size(path);
            let cleaned = before_size.saturating_sub(after_size);

            if cleaned > 0 {
                println!("🧹 Cleaned {}: {} MB", name, cleaned / 1024 / 1024);
                total_cleaned += cleaned;
            } else {
                println!("✅ {}: Already clean", name);
            }
        } else {
            println!("📭 {}: Not found", name);
        }
    }

    println!("\n✅ Cache cleaning completed");
    println!("📊 Total space freed: {} MB", total_cleaned / 1024 / 1024);
}

fn clean_unused_lutris_prefixes() {
    println!("🗑️ Cleaning Unused Lutris Prefixes");
    println!("===================================\n");

    let prefixes_dir = format!("{}/.local/share/lutris/prefixes", get_home_dir());

    if !Path::new(&prefixes_dir).exists() {
        println!("📭 No prefixes directory found");
        return;
    }

    let find_result = Command::new("find")
        .args(&[
            &prefixes_dir,
            "-maxdepth",
            "1",
            "-type",
            "d",
            "-atime",
            "+60",
        ])
        .output();

    let mut old_prefixes = Vec::new();

    if let Ok(out) = find_result {
        for prefix in String::from_utf8_lossy(&out.stdout).lines() {
            if prefix != prefixes_dir
                && let Some(name) = Path::new(prefix).file_name()
            {
                let size = get_directory_size(prefix);
                old_prefixes.push((name.to_string_lossy().to_string(), prefix.to_string(), size));
            }
        }
    }

    if old_prefixes.is_empty() {
        println!("✅ No unused prefixes found (older than 60 days)");
        return;
    }

    println!(
        "🔍 Found {} potentially unused prefixes:",
        old_prefixes.len()
    );
    for (name, _, size) in &old_prefixes {
        println!(
            "  📦 {}: {} MB (not accessed in 60+ days)",
            name,
            size / 1024 / 1024
        );
    }

    let Ok(selected) = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select prefixes to remove")
        .items(
            &old_prefixes
                .iter()
                .map(|(name, _, size)| format!("{} ({} MB)", name, size / 1024 / 1024))
                .collect::<Vec<_>>(),
        )
        .interact()
    else {
        return;
    };

    for idx in selected {
        let (name, path, size) = &old_prefixes[idx];

        let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(&format!(
                "Remove prefix '{}' ({} MB)?",
                name,
                size / 1024 / 1024
            ))
            .default(false)
            .interact()
        else {
            continue;
        };

        if confirm {
            let result = Command::new("rm").args(&["-rf", path]).status();

            match result {
                Ok(s) if s.success() => println!("✅ {} removed", name),
                _ => println!("❌ Failed to remove {}", name),
            }
        }
    }
}

fn clean_old_lutris_runners() {
    println!("📦 Cleaning Old Lutris Runners");
    println!("==============================\n");

    let runners_dir = format!("{}/.local/share/lutris/runners/wine", get_home_dir());

    if !Path::new(&runners_dir).exists() {
        println!("📭 No Wine runners directory found");
        return;
    }

    let find_result = Command::new("find")
        .args(&[&runners_dir, "-maxdepth", "1", "-type", "d"])
        .output();

    let mut all_runners = Vec::new();

    if let Ok(out) = find_result {
        for runner in String::from_utf8_lossy(&out.stdout).lines() {
            if runner != runners_dir
                && let Some(name) = Path::new(runner).file_name()
            {
                let size = get_directory_size(runner);
                let modified_time = get_last_access_time(runner);
                all_runners.push((
                    name.to_string_lossy().to_string(),
                    runner.to_string(),
                    size,
                    modified_time,
                ));
            }
        }
    }

    if all_runners.is_empty() {
        println!("📭 No Wine runners found");
        return;
    }

    all_runners.sort_by(|a, b| b.2.cmp(&a.2));

    println!("📋 Installed Wine Runners:");
    for (name, _, size, modified) in &all_runners {
        println!(
            "  📦 {}: {} MB (modified: {})",
            name,
            size / 1024 / 1024,
            modified
        );
    }

    let cleanup_options = [
        "🗑️ Remove selected runners",
        "🧹 Keep only latest versions",
        "📊 Show storage usage only",
        "⬅️ Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cleanup option")
        .items(&cleanup_options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => {
            let Ok(selected) = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Select runners to remove")
                .items(
                    &all_runners
                        .iter()
                        .map(|(name, _, size, _)| format!("{} ({} MB)", name, size / 1024 / 1024))
                        .collect::<Vec<_>>(),
                )
                .interact()
            else {
                return;
            };

            for idx in selected {
                let (name, path, _, _) = &all_runners[idx];

                let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(&format!("Remove runner '{}'?", name))
                    .default(false)
                    .interact()
                else {
                    continue;
                };

                if confirm {
                    let result = Command::new("rm").args(&["-rf", path]).status();

                    match result {
                        Ok(s) if s.success() => println!("✅ {} removed", name),
                        _ => println!("❌ Failed to remove {}", name),
                    }
                }
            }
        }
        1 => {
            println!("🧹 Auto-cleanup: Keeping only 2 most recent versions of each runner type");

            let mut runner_groups: std::collections::HashMap<
                String,
                Vec<&(String, String, u64, String)>,
            > = std::collections::HashMap::new();

            for runner in &all_runners {
                let base_name = if runner.0.contains("lutris") {
                    "lutris"
                } else if runner.0.contains("ge") {
                    "ge"
                } else if runner.0.contains("tkg") {
                    "tkg"
                } else {
                    "other"
                };

                runner_groups
                    .entry(base_name.to_string())
                    .or_default()
                    .push(runner);
            }

            for (group_name, mut group_runners) in runner_groups {
                if group_runners.len() > 2 {
                    group_runners.sort_by(|a, b| b.3.cmp(&a.3));

                    for runner in group_runners.iter().skip(2) {
                        println!("🗑️ Removing old {} runner: {}", group_name, runner.0);
                        Command::new("rm").args(&["-rf", &runner.1]).status().ok();
                    }
                }
            }
        }
        2 => {
            let total_size: u64 = all_runners.iter().map(|(_, _, size, _)| size).sum();
            println!(
                "📊 Total Wine runners storage: {} GB",
                total_size / 1024 / 1024 / 1024
            );
        }
        _ => {}
    }
}

fn check_lutris_health() {
    println!("🩺 Checking Lutris Installation Health");
    println!("======================================\n");

    let mut issues_found = 0;

    let lutris_check = Command::new("which").arg("lutris").output();
    match lutris_check {
        Ok(out) if !out.stdout.is_empty() => {
            println!("✅ Lutris executable: Found");

            let version_check = Command::new("lutris").arg("--version").output();
            if let Ok(ver_out) = version_check {
                let output_string = String::from_utf8_lossy(&ver_out.stdout);
                println!("  📋 Version: {}", output_string.trim());
            }
        }
        _ => {
            println!("❌ Lutris executable: Not found");
            issues_found += 1;
        }
    }

    let essential_dirs = [
        ("Config", format!("{}/.config/lutris", get_home_dir())),
        (
            "Games",
            format!("{}/.local/share/lutris/games", get_home_dir()),
        ),
        (
            "Prefixes",
            format!("{}/.local/share/lutris/prefixes", get_home_dir()),
        ),
        (
            "Runners",
            format!("{}/.local/share/lutris/runners", get_home_dir()),
        ),
    ];

    for (name, path) in &essential_dirs {
        if Path::new(path).exists() {
            println!("✅ {}: {}", name, path);
        } else {
            println!("⚠️ {}: Missing ({})", name, path);
            issues_found += 1;
        }
    }

    let wine_check = Command::new("wine").arg("--version").output();
    match wine_check {
        Ok(out) => {
            let output_string = String::from_utf8_lossy(&out.stdout);
            println!("✅ Wine: {}", output_string.trim());
        }
        Err(_) => {
            println!("❌ Wine: Not installed");
            issues_found += 1;
        }
    }

    let deps = [
        ("winetricks", "Wine helper scripts"),
        ("python3", "Python 3 runtime"),
        ("curl", "Download manager"),
        ("cabextract", "Cabinet extractor"),
    ];

    for (dep, description) in &deps {
        let dep_available = Command::new("which")
            .arg(dep)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if dep_available {
            println!("✅ {}: {}", dep, description);
        } else {
            println!("⚠️ {}: {} (missing)", dep, description);
            issues_found += 1;
        }
    }

    let runners_dir = format!("{}/.local/share/lutris/runners/wine", get_home_dir());
    if Path::new(&runners_dir).exists() {
        let find_result = Command::new("find")
            .args(&[&runners_dir, "-maxdepth", "1", "-type", "d"])
            .output();

        if let Ok(out) = find_result {
            let runner_count = String::from_utf8_lossy(&out.stdout)
                .lines()
                .count()
                .saturating_sub(1);
            if runner_count > 0 {
                println!("✅ Wine runners: {} installed", runner_count);
            } else {
                println!("⚠️ Wine runners: None installed");
                issues_found += 1;
            }
        }
    }

    let games_result = Command::new("lutris").args(&["--list-games"]).output();
    match games_result {
        Ok(out) => {
            let output_string = String::from_utf8_lossy(&out.stdout);
            let game_count = output_string
                .lines()
                .filter(|line| !line.trim().is_empty())
                .count();
            println!("📊 Games: {} installed", game_count);
        }
        Err(_) => {
            println!("⚠️ Games: Unable to list (Lutris may have issues)");
            issues_found += 1;
        }
    }

    println!("\n📊 Health Check Summary:");
    if issues_found == 0 {
        println!("✅ All systems healthy - Lutris is ready for gaming!");
    } else {
        println!(
            "⚠️ {} issues found - consider addressing them for optimal performance",
            issues_found
        );

        if issues_found > 3 {
            let Ok(reinstall) = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Many issues detected. Reinstall Lutris?")
                .default(false)
                .interact()
            else {
                return;
            };

            if reinstall {
                println!("🔄 Reinstalling Lutris...");
                install_lutris_fresh();
            }
        }
    }
}

fn lutris_storage_analysis() {
    println!("📊 Lutris Storage Analysis");
    println!("==========================\n");

    let lutris_base = format!("{}/.local/share/lutris", get_home_dir());
    let storage_categories = [
        ("Games", format!("{}/games", lutris_base)),
        ("Prefixes", format!("{}/prefixes", lutris_base)),
        ("Runners", format!("{}/runners", lutris_base)),
        ("Cache", format!("{}/.cache/lutris", get_home_dir())),
        ("Config", format!("{}/.config/lutris", get_home_dir())),
    ];

    let mut total_size = 0u64;
    let mut category_sizes = Vec::new();

    for (category, path) in &storage_categories {
        if Path::new(path).exists() {
            let size = get_directory_size(path);
            total_size += size;
            category_sizes.push((category, size));

            println!("📁 {}: {} GB", category, size / 1024 / 1024 / 1024);

            if size > 5 * 1024 * 1024 * 1024 {
                show_large_subdirectories(path, category);
            }
        } else {
            println!("📁 {}: Not found", category);
        }
    }

    category_sizes.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\n📊 Storage Summary:");
    println!(
        "  💾 Total Lutris storage: {} GB",
        total_size / 1024 / 1024 / 1024
    );

    if total_size > 100 * 1024 * 1024 * 1024 {
        println!("  ⚠️ Large storage usage detected");
        println!("  💡 Consider cleaning up old games, prefixes, or runners");

        show_cleanup_recommendations(&category_sizes);
    }

    let disk_check = Command::new("df").args(&["-h", "."]).output();
    if let Ok(out) = disk_check {
        let output = String::from_utf8_lossy(&out.stdout);
        for line in output.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                println!("  💿 Available disk space: {}", parts[3]);
                break;
            }
        }
    }
}

fn show_large_subdirectories(path: &str, category: &str) {
    let find_result = Command::new("find")
        .args(&[path, "-maxdepth", "1", "-type", "d"])
        .output();

    if let Ok(out) = find_result {
        let mut subdirs = Vec::new();

        for subdir in String::from_utf8_lossy(&out.stdout).lines() {
            if subdir != path
                && let Some(name) = Path::new(subdir).file_name()
            {
                let size = get_directory_size(subdir);
                if size > 1024 * 1024 * 1024 {
                    subdirs.push((name.to_string_lossy().to_string(), size));
                }
            }
        }

        if !subdirs.is_empty() {
            subdirs.sort_by(|a, b| b.1.cmp(&a.1));
            println!("    {} breakdown:", category);
            for (name, size) in subdirs.iter().take(5) {
                println!("      📦 {}: {} GB", name, size / 1024 / 1024 / 1024);
            }
        }
    }
}

fn show_cleanup_recommendations(category_sizes: &[(&&str, u64)]) {
    println!("\n💡 Cleanup Recommendations:");

    for (category, size) in category_sizes {
        let size_gb = *size / 1024 / 1024 / 1024;

        match **category {
            "Games" if size_gb > 50 => {
                println!(
                    "  🎮 Games ({}GB): Remove finished or unused games",
                    size_gb
                );
            }
            "Prefixes" if size_gb > 20 => {
                println!("  🍷 Prefixes ({}GB): Clean old Wine prefixes", size_gb);
            }
            "Runners" if size_gb > 10 => {
                println!(
                    "  🏃 Runners ({}GB): Remove old Wine runner versions",
                    size_gb
                );
            }
            "Cache" if size_gb > 5 => {
                println!(
                    "  💾 Cache ({}GB): Clear temporary files and caches",
                    size_gb
                );
            }
            _ => {}
        }
    }
}

fn reset_lutris_config() {
    println!("🔧 Reset Lutris Configuration");
    println!("=============================\n");

    println!("⚠️ This will reset Lutris to default settings");
    println!("📋 What will be reset:");
    println!("   • Lutris preferences");
    println!("   • Game library metadata");
    println!("   • Online service connections");
    println!("   • Custom configurations");

    println!("\n🔒 What will be preserved:");
    println!("   • Installed games");
    println!("   • Wine prefixes");
    println!("   • Wine runners");
    println!("   • Game save files");

    let Ok(confirm_reset) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Proceed with configuration reset?")
        .default(false)
        .interact()
    else {
        return;
    };

    if !confirm_reset {
        return;
    }

    let config_dir = format!("{}/.config/lutris", get_home_dir());
    let backup_dir = format!(
        "{}.backup.{}",
        config_dir,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    );

    if Path::new(&config_dir).exists() {
        println!("📦 Creating backup: {}", backup_dir);
        Command::new("cp")
            .args(&["-r", &config_dir, &backup_dir])
            .status()
            .ok();
    }

    if Path::new(&config_dir).exists() {
        println!("🗑️ Removing old configuration...");
        Command::new("rm").args(&["-rf", &config_dir]).status().ok();
    }

    let cache_dir = format!("{}/.cache/lutris", get_home_dir());
    if Path::new(&cache_dir).exists() {
        println!("🧹 Clearing cache...");
        Command::new("rm")
            .args(&["-rf", &format!("{}/*", cache_dir)])
            .status()
            .ok();
    }

    println!("✅ Lutris configuration reset completed");
    println!("📦 Backup available at: {}", backup_dir);
    println!("🔄 Restart Lutris to initialize default settings");

    let Ok(restart_lutris) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Launch Lutris now?")
        .default(true)
        .interact()
    else {
        return;
    };

    if restart_lutris {
        Command::new("lutris").spawn().ok();
    }
}
fn steam_proton_management() {
    println!("🚀 Steam/Proton Management - Feature coming soon...");
}

fn optimization_profiles() {
    println!("🔧 System Optimization Profiles - Coming soon...");
}

fn gaming_health_check() {
    println!("🩺 Gaming System Health Check - Coming soon...");
}

fn deep_cleanup() {
    println!("🧹 Deep Cleanup & Reset - Coming soon...");
}

fn game_categorization() {
    println!("🏷️ Game Categorization - Coming soon...");
}

fn symlink_management() {
    println!("🔗 Symbolic Link Management - Coming soon...");
}

fn game_database_export() {
    println!("📝 Game Database Export - Coming soon...");
}

fn wine_registry_cleanup() {
    println!("🔧 Wine Registry Cleanup - Coming soon...");
}
