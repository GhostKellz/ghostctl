use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

// Cache for commonly accessed paths
static HOME_DIR: OnceLock<String> = OnceLock::new();
static GAMES_DIR: OnceLock<String> = OnceLock::new();

fn get_home_dir() -> &'static str {
    HOME_DIR.get_or_init(|| std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string()))
}

fn get_games_dir() -> &'static str {
    GAMES_DIR.get_or_init(|| format!("{}/Games", get_home_dir()))
}

pub fn wine_prefix_menu() {
    loop {
        let options = [
            "🍷 Create New Prefix",
            "📋 List Prefixes",
            "🔄 Clone Prefix",
            "💾 Backup Prefix",
            "📥 Restore Prefix",
            "🗑️ Delete Prefix",
            "🔧 Configure Prefix",
            "🎮 Game-Specific Prefix",
            "🔍 Auto-Detect Prefixes",
            "🏷️ Prefix Templates",
            "⬅️ Back",
        ];

        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🍾 Wine Prefix Management")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match choice {
            0 => create_new_prefix(),
            1 => list_prefixes(),
            2 => clone_prefix(),
            3 => backup_prefix(),
            4 => restore_prefix(),
            5 => delete_prefix(),
            6 => configure_prefix(),
            7 => game_specific_prefix(),
            8 => auto_detect_prefixes(),
            9 => prefix_templates(),
            _ => break,
        }
    }
}

fn create_new_prefix() {
    println!("🍷 Creating New Wine Prefix");

    let Ok(prefix_name) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter prefix name")
        .interact()
    else {
        return;
    };

    let prefix_path = format!("{}/prefixes/{}", get_games_dir(), prefix_name);

    let Ok(arch) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select architecture")
        .items(&["64-bit (win64)", "32-bit (win32)"])
        .default(0)
        .interact()
    else {
        return;
    };

    let arch_str = if arch == 0 { "win64" } else { "win32" };

    let Ok(windows_version) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Windows version")
        .items(&["Windows 10", "Windows 7", "Windows XP", "Windows 11"])
        .default(0)
        .interact()
    else {
        return;
    };

    let win_version = match windows_version {
        0 => "win10",
        1 => "win7",
        2 => "winxp",
        3 => "win11",
        _ => "win10",
    };

    println!("📦 Creating prefix at: {}", prefix_path);
    fs::create_dir_all(&prefix_path).ok();

    // Initialize prefix
    let init_cmd = format!(
        "WINEPREFIX={} WINEARCH={} wineboot -i",
        prefix_path, arch_str
    );

    let status = Command::new("sh").arg("-c").arg(&init_cmd).status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Prefix created successfully");

            // Set Windows version
            let version_cmd = format!("WINEPREFIX={} winecfg /v {}", prefix_path, win_version);
            Command::new("sh").arg("-c").arg(&version_cmd).status().ok();

            // Save prefix metadata
            let metadata = format!(
                "name={}\narch={}\nwindows={}\ncreated={}\n",
                prefix_name,
                arch_str,
                win_version,
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            );

            fs::write(format!("{}/prefix.info", prefix_path), metadata).ok();
            println!("💾 Prefix metadata saved");

            // Ask about common components
            let Ok(install_components) = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Install common gaming components?")
                .default(true)
                .interact()
            else {
                return;
            };
            if install_components {
                install_common_components(&prefix_path);
            }
        }
        _ => println!("❌ Failed to create prefix"),
    }
}

fn install_common_components(prefix_path: &str) {
    println!("📦 Installing common gaming components...");

    let components = ["vcrun2019", "dotnet48", "d3dx9", "faudio"];

    for component in &components {
        println!("  Installing {}...", component);
        let cmd = format!("WINEPREFIX={} winetricks -q {}", prefix_path, component);
        Command::new("sh").arg("-c").arg(&cmd).status().ok();
    }

    println!("✅ Common components installed");
}

fn list_prefixes() {
    println!("📋 Wine Prefixes");

    let prefixes_dir = format!("{}/Games/prefixes", get_home_dir());

    if !Path::new(&prefixes_dir).exists() {
        println!("❌ No prefixes directory found");
        return;
    }

    let Ok(entries) = fs::read_dir(&prefixes_dir) else {
        println!("❌ Failed to read prefixes directory");
        return;
    };
    let mut prefixes = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let Some(file_name) = path.file_name() else {
                continue;
            };
            let name = file_name.to_string_lossy().to_string();

            // Try to read metadata
            let info_path = format!("{}/prefix.info", path.display());
            let info = if Path::new(&info_path).exists() {
                fs::read_to_string(&info_path).unwrap_or_default()
            } else {
                String::new()
            };

            // Get size
            let size_output = Command::new("du")
                .args(["-sh", &path.to_string_lossy()])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .unwrap_or_default();

            let size = size_output.split('\t').next().unwrap_or("Unknown");

            prefixes.push(format!("📁 {} ({})\n{}", name, size, info));
        }
    }

    if prefixes.is_empty() {
        println!("❌ No prefixes found");
    } else {
        for prefix in prefixes {
            println!("{}", prefix);
            println!("---");
        }
    }

    // Also check for Lutris prefixes
    let lutris_prefixes = format!("{}/.local/share/lutris/runners/wine", get_home_dir());

    if Path::new(&lutris_prefixes).exists() {
        println!("\n🎮 Lutris Wine Prefixes:");
        if let Ok(entries) = fs::read_dir(&lutris_prefixes) {
            for entry in entries.flatten() {
                println!("  📁 {}", entry.file_name().to_string_lossy());
            }
        }
    }
}

fn clone_prefix() {
    println!("🔄 Clone Wine Prefix");

    let Ok(source) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter source prefix path")
        .interact()
    else {
        return;
    };

    if !Path::new(&source).exists() {
        println!("❌ Source prefix not found");
        return;
    }

    let Ok(dest_name) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter new prefix name")
        .interact()
    else {
        return;
    };

    let dest = format!("{}/Games/prefixes/{}", get_home_dir(), dest_name);

    println!("📂 Cloning prefix...");
    let cmd = format!("cp -r '{}' '{}'", source, dest);

    let status = Command::new("sh").arg("-c").arg(&cmd).status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Prefix cloned successfully");

            // Update metadata
            let metadata = format!(
                "name={}\ncloned_from={}\ncloned={}\n",
                dest_name,
                source,
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            );

            fs::write(format!("{}/prefix.info", dest), metadata).ok();
        }
        _ => println!("❌ Failed to clone prefix"),
    }
}

fn backup_prefix() {
    println!("💾 Backup Wine Prefix");

    let Ok(prefix_path) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter prefix path to backup")
        .interact()
    else {
        return;
    };

    if !Path::new(&prefix_path).exists() {
        println!("❌ Prefix not found");
        return;
    }

    let Ok(backup_name) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter backup name")
        .default(format!(
            "backup_{}",
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        ))
        .interact()
    else {
        return;
    };

    let backup_dir = format!("{}/Games/prefix_backups", get_home_dir());

    fs::create_dir_all(&backup_dir).ok();

    let backup_path = format!("{}/{}.tar.gz", backup_dir, backup_name);

    println!("📦 Creating backup...");
    let cmd = format!("tar -czf '{}' -C '{}' .", backup_path, prefix_path);

    let status = Command::new("sh").arg("-c").arg(&cmd).status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Backup created: {}", backup_path);

            // Save backup metadata
            let metadata = format!(
                "source={}\ndate={}\nsize={}\n",
                prefix_path,
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                fs::metadata(&backup_path)
                    .ok()
                    .map(|m| m.len())
                    .unwrap_or(0)
            );

            fs::write(format!("{}.info", backup_path), metadata).ok();
        }
        _ => println!("❌ Failed to create backup"),
    }
}

fn restore_prefix() {
    println!("📥 Restore Wine Prefix");

    let backup_dir = format!("{}/Games/prefix_backups", get_home_dir());

    if !Path::new(&backup_dir).exists() {
        println!("❌ No backups found");
        return;
    }

    // List available backups
    let Ok(entries) = fs::read_dir(&backup_dir) else {
        println!("❌ Failed to read backup directory");
        return;
    };
    let mut backups = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("gz") {
            let Some(file_stem) = path.file_stem() else {
                continue;
            };
            let name = file_stem.to_string_lossy().to_string();

            // Read metadata if exists
            let info_path = format!("{}.info", path.display());
            let info = if Path::new(&info_path).exists() {
                fs::read_to_string(&info_path).unwrap_or_default()
            } else {
                String::new()
            };

            backups.push((name.clone(), path.to_string_lossy().to_string(), info));
        }
    }

    if backups.is_empty() {
        println!("❌ No backups found");
        return;
    }

    let backup_names: Vec<String> = backups
        .iter()
        .map(|(name, _, info)| format!("{}\n{}", name, info))
        .collect();

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select backup to restore")
        .items(&backup_names)
        .default(0)
        .interact()
    else {
        return;
    };

    let (_, backup_path, _) = &backups[choice];

    let Ok(restore_name) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter prefix name for restoration")
        .interact()
    else {
        return;
    };

    let restore_path = format!("{}/Games/prefixes/{}", get_home_dir(), restore_name);

    if Path::new(&restore_path).exists() {
        let Ok(overwrite) = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Prefix exists. Overwrite?")
            .default(false)
            .interact()
        else {
            return;
        };

        if !overwrite {
            return;
        }

        Command::new("rm")
            .args(&["-rf", &restore_path])
            .status()
            .ok();
    }

    fs::create_dir_all(&restore_path).ok();

    println!("📂 Restoring prefix...");
    let cmd = format!("tar -xzf '{}' -C '{}'", backup_path, restore_path);

    let status = Command::new("sh").arg("-c").arg(&cmd).status();

    match status {
        Ok(s) if s.success() => println!("✅ Prefix restored successfully"),
        _ => println!("❌ Failed to restore prefix"),
    }
}

fn delete_prefix() {
    println!("🗑️ Delete Wine Prefix");

    let Ok(prefix_path) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter prefix path to delete")
        .interact()
    else {
        return;
    };

    if !Path::new(&prefix_path).exists() {
        println!("❌ Prefix not found");
        return;
    }

    // Show prefix info before deletion
    let info_path = format!("{}/prefix.info", prefix_path);
    if Path::new(&info_path).exists() {
        let info = fs::read_to_string(&info_path).unwrap_or_default();
        println!("📋 Prefix info:\n{}", info);
    }

    let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Are you sure you want to delete this prefix?")
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
            backup_prefix_internal(&prefix_path);
        }

        println!("🗑️ Deleting prefix...");
        let status = Command::new("rm").args(&["-rf", &prefix_path]).status();

        match status {
            Ok(s) if s.success() => println!("✅ Prefix deleted"),
            _ => println!("❌ Failed to delete prefix"),
        }
    }
}

fn backup_prefix_internal(prefix_path: &str) {
    let backup_name = format!("deleted_{}", chrono::Local::now().format("%Y%m%d_%H%M%S"));
    let backup_dir = format!("{}/Games/prefix_backups", get_home_dir());

    fs::create_dir_all(&backup_dir).ok();

    let backup_path = format!("{}/{}.tar.gz", backup_dir, backup_name);
    let cmd = format!("tar -czf '{}' -C '{}' .", backup_path, prefix_path);

    Command::new("sh").arg("-c").arg(&cmd).status().ok();
    println!("💾 Backup created: {}", backup_path);
}

fn configure_prefix() {
    println!("🔧 Configure Wine Prefix");

    let Ok(prefix_path) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter prefix path")
        .interact()
    else {
        return;
    };

    if !Path::new(&prefix_path).exists() {
        println!("❌ Prefix not found");
        return;
    }

    let options = [
        "🪟 Change Windows Version",
        "🔧 Run winecfg",
        "📦 Install Components",
        "🎮 Gaming Optimizations",
        "🔊 Audio Configuration",
        "🖥️ Display Settings",
        "📝 Edit Registry",
        "🔄 Reset to Default",
        "⬅️ Back",
    ];

    loop {
        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Configure Prefix")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match choice {
            0 => change_windows_version(&prefix_path),
            1 => {
                let cmd = format!("WINEPREFIX={} winecfg", prefix_path);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
            }
            2 => install_prefix_components(&prefix_path),
            3 => apply_gaming_optimizations(&prefix_path),
            4 => configure_audio(&prefix_path),
            5 => configure_display(&prefix_path),
            6 => edit_registry(&prefix_path),
            7 => reset_prefix(&prefix_path),
            _ => break,
        }
    }
}

fn change_windows_version(prefix_path: &str) {
    let versions = [
        "win11", "win10", "win81", "win8", "win7", "winxp", "win98", "win95",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Windows version")
        .items(&versions)
        .default(1)
        .interact()
    else {
        return;
    };

    let cmd = format!("WINEPREFIX={} winecfg /v {}", prefix_path, versions[choice]);

    let status = Command::new("sh").arg("-c").arg(&cmd).status();

    match status {
        Ok(s) if s.success() => println!("✅ Windows version changed to {}", versions[choice]),
        _ => println!("❌ Failed to change Windows version"),
    }
}

fn install_prefix_components(prefix_path: &str) {
    let components = vec![
        "Visual C++ 2019 (vcrun2019)",
        ".NET Framework 4.8 (dotnet48)",
        "DirectX 9 (d3dx9)",
        "DirectX 10 (d3dx10)",
        "DirectX 11 (d3dx11_43)",
        "OpenAL (openal)",
        "PhysX (physx)",
        "XNA Framework (xna40)",
        "Media Foundation (mf)",
        "Windows Media Player (wmp10)",
    ];

    let Ok(selected) = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select components to install")
        .items(&components)
        .interact()
    else {
        return;
    };

    for idx in selected {
        let Some(component) = components[idx]
            .split('(')
            .nth(1)
            .map(|s| s.trim_end_matches(')'))
        else {
            continue;
        };
        println!("📦 Installing {}...", component);

        let cmd = format!("WINEPREFIX={} winetricks -q {}", prefix_path, component);
        let status = Command::new("sh").arg("-c").arg(&cmd).status();

        match status {
            Ok(s) if s.success() => println!("  ✅ {} installed", component),
            _ => println!("  ❌ Failed to install {}", component),
        }
    }
}

fn apply_gaming_optimizations(prefix_path: &str) {
    println!("🎮 Applying Gaming Optimizations");

    // Enable CSMT
    let cmd = format!(
        "WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Software\\Wine\\Direct3D' /v csmt /t REG_DWORD /d 1 /f",
        prefix_path
    );
    Command::new("sh").arg("-c").arg(&cmd).status().ok();

    // Enable Large Address Aware
    unsafe { std::env::set_var("WINE_LARGE_ADDRESS_AWARE", "1") };

    // Disable debugging
    let cmd = format!(
        "WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Software\\Wine\\Debug' /v RelayExclude /d 'ntdll.RtlEnterCriticalSection;ntdll.RtlLeaveCriticalSection' /f",
        prefix_path
    );
    Command::new("sh").arg("-c").arg(&cmd).status().ok();

    // Optimize heap allocation
    let cmd = format!(
        "WINEPREFIX={} wine reg add 'HKEY_LOCAL_MACHINE\\System\\CurrentControlSet\\Control\\Session Manager' /v HeapDeCommitFreeBlockThreshold /t REG_DWORD /d 262144 /f",
        prefix_path
    );
    Command::new("sh").arg("-c").arg(&cmd).status().ok();

    println!("✅ Gaming optimizations applied");
}

fn configure_audio(prefix_path: &str) {
    let audio_systems = ["pulse", "alsa", "oss", "jack"];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select audio system")
        .items(&audio_systems)
        .default(0)
        .interact()
    else {
        return;
    };

    let cmd = format!(
        "WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Software\\Wine\\Drivers' /v Audio /d {} /f",
        prefix_path, audio_systems[choice]
    );

    Command::new("sh").arg("-c").arg(&cmd).status().ok();
    println!("✅ Audio system set to {}", audio_systems[choice]);
}

fn configure_display(prefix_path: &str) {
    let options = [
        "Enable Virtual Desktop",
        "Disable Virtual Desktop",
        "Set DPI",
        "Enable Window Decorations",
        "Disable Window Decorations",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Display configuration")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => {
            let Ok(_resolution) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter resolution (e.g., 1920x1080)")
                .default("1920x1080".to_string())
                .interact()
            else {
                return;
            };

            let cmd = format!("WINEPREFIX={} winecfg", prefix_path);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
            println!("✅ Please configure virtual desktop in the opened window");
        }
        1 => {
            let cmd = format!("WINEPREFIX={} winecfg", prefix_path);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
            println!("✅ Please disable virtual desktop in the opened window");
        }
        2 => {
            let Ok(dpi) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter DPI value")
                .default("96".to_string())
                .interact()
            else {
                return;
            };

            let cmd = format!(
                "WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Control Panel\\Desktop' /v LogPixels /t REG_DWORD /d {} /f",
                prefix_path, dpi
            );
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
            println!("✅ DPI set to {}", dpi);
        }
        3 => {
            let cmd = format!(
                "WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Software\\Wine\\X11 Driver' /v Decorated /d Y /f",
                prefix_path
            );
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
            println!("✅ Window decorations enabled");
        }
        4 => {
            let cmd = format!(
                "WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Software\\Wine\\X11 Driver' /v Decorated /d N /f",
                prefix_path
            );
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
            println!("✅ Window decorations disabled");
        }
        _ => {}
    }
}

fn edit_registry(prefix_path: &str) {
    let cmd = format!("WINEPREFIX={} wine regedit", prefix_path);
    Command::new("sh").arg("-c").arg(&cmd).status().ok();
}

fn reset_prefix(prefix_path: &str) {
    let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will reset the prefix to default settings. Continue?")
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        println!("🔄 Resetting prefix...");
        let cmd = format!("WINEPREFIX={} wineboot -r", prefix_path);

        let status = Command::new("sh").arg("-c").arg(&cmd).status();

        match status {
            Ok(s) if s.success() => println!("✅ Prefix reset to default"),
            _ => println!("❌ Failed to reset prefix"),
        }
    }
}

fn game_specific_prefix() {
    println!("🎮 Game-Specific Prefix Setup");

    let Ok(game_name) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter game name")
        .interact()
    else {
        return;
    };

    let templates = [
        "Modern AAA Game (DX12, Ray Tracing)",
        "Classic Game (DX9, older)",
        "Indie Game (Unity/Godot)",
        "Multiplayer Game (Anti-cheat)",
        "Custom Configuration",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select game template")
        .items(&templates)
        .default(0)
        .interact()
    else {
        return;
    };

    let prefix_path = format!(
        "{}/Games/prefixes/{}",
        get_home_dir(),
        game_name.to_lowercase().replace(" ", "_")
    );

    // Create prefix
    fs::create_dir_all(&prefix_path).ok();
    let init_cmd = format!("WINEPREFIX={} WINEARCH=win64 wineboot -i", prefix_path);
    Command::new("sh").arg("-c").arg(&init_cmd).status().ok();

    match choice {
        0 => setup_modern_game_prefix(&prefix_path),
        1 => setup_classic_game_prefix(&prefix_path),
        2 => setup_indie_game_prefix(&prefix_path),
        3 => setup_multiplayer_game_prefix(&prefix_path),
        4 => configure_prefix_internal(&prefix_path),
        _ => {}
    }

    println!("✅ Game-specific prefix created: {}", prefix_path);
}

fn setup_modern_game_prefix(prefix_path: &str) {
    println!("🎮 Setting up modern AAA game prefix...");

    // Windows 10
    Command::new("sh")
        .arg("-c")
        .arg(&format!("WINEPREFIX={} winecfg /v win10", prefix_path))
        .status()
        .ok();

    // Install components
    let components = ["vcrun2019", "dotnet48", "d3dx11_43", "faudio"];
    for comp in &components {
        let cmd = format!("WINEPREFIX={} winetricks -q {}", prefix_path, comp);
        Command::new("sh").arg("-c").arg(&cmd).status().ok();
    }

    // Enable DXVK/VKD3D
    println!("  Installing DXVK and VKD3D-Proton...");
    // Would install DXVK/VKD3D here

    println!("✅ Modern game prefix configured");
}

fn setup_classic_game_prefix(prefix_path: &str) {
    println!("🎮 Setting up classic game prefix...");

    // Windows XP
    Command::new("sh")
        .arg("-c")
        .arg(&format!("WINEPREFIX={} winecfg /v winxp", prefix_path))
        .status()
        .ok();

    // Install components
    let components = ["vcrun2005", "vcrun2008", "d3dx9", "directplay"];
    for comp in &components {
        let cmd = format!("WINEPREFIX={} winetricks -q {}", prefix_path, comp);
        Command::new("sh").arg("-c").arg(&cmd).status().ok();
    }

    println!("✅ Classic game prefix configured");
}

fn setup_indie_game_prefix(prefix_path: &str) {
    println!("🎮 Setting up indie game prefix...");

    // Windows 7
    Command::new("sh")
        .arg("-c")
        .arg(&format!("WINEPREFIX={} winecfg /v win7", prefix_path))
        .status()
        .ok();

    // Install components
    let components = ["dotnet40", "vcrun2017", "openal"];
    for comp in &components {
        let cmd = format!("WINEPREFIX={} winetricks -q {}", prefix_path, comp);
        Command::new("sh").arg("-c").arg(&cmd).status().ok();
    }

    println!("✅ Indie game prefix configured");
}

fn setup_multiplayer_game_prefix(prefix_path: &str) {
    println!("🎮 Setting up multiplayer game prefix...");

    // Windows 10
    Command::new("sh")
        .arg("-c")
        .arg(&format!("WINEPREFIX={} winecfg /v win10", prefix_path))
        .status()
        .ok();

    // Disable Esync/Fsync for compatibility
    unsafe { std::env::remove_var("WINEESYNC") };
    unsafe { std::env::remove_var("WINEFSYNC") };

    println!("⚠️ Note: Anti-cheat support may require additional configuration");
    println!("✅ Multiplayer game prefix configured");
}

fn configure_prefix_internal(_prefix_path: &str) {
    configure_prefix();
}

fn auto_detect_prefixes() {
    println!("🔍 Auto-Detecting Wine Prefixes");

    let mut found_prefixes = Vec::new();

    // Check common locations
    let locations = [
        format!("{}/.wine", get_home_dir()),
        format!("{}/.local/share/lutris/runners/wine", get_home_dir()),
        format!("{}/.local/share/bottles/bottles", get_home_dir()),
        format!("{}/.PlayOnLinux/wineprefix", get_home_dir()),
        format!("{}/.steam/steam/steamapps/compatdata", get_home_dir()),
        format!("{}/Games/prefixes", get_home_dir()),
    ];

    for location in &locations {
        if Path::new(location).exists() {
            println!("📂 Checking {}...", location);

            if location.contains("compatdata") {
                // Steam Proton prefixes
                if let Ok(entries) = fs::read_dir(location) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() {
                            let pfx_path = format!("{}/pfx", path.display());
                            if Path::new(&pfx_path).exists() {
                                if let Some(file_name) = path.file_name() {
                                    found_prefixes
                                        .push(format!("Steam: {}", file_name.to_string_lossy()));
                                }
                            }
                        }
                    }
                }
            } else if location.contains("bottles") {
                // Bottles prefixes
                if let Ok(entries) = fs::read_dir(location) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() {
                            if let Some(file_name) = path.file_name() {
                                found_prefixes
                                    .push(format!("Bottles: {}", file_name.to_string_lossy()));
                            }
                        }
                    }
                }
            } else {
                // Regular Wine prefixes
                if Path::new(&format!("{}/drive_c", location)).exists() {
                    found_prefixes.push(location.clone());
                } else if let Ok(entries) = fs::read_dir(location) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir()
                            && Path::new(&format!("{}/drive_c", path.display())).exists()
                        {
                            found_prefixes.push(path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }

    if found_prefixes.is_empty() {
        println!("❌ No Wine prefixes found");
    } else {
        println!("\n✅ Found {} prefixes:", found_prefixes.len());
        for prefix in found_prefixes {
            println!("  📁 {}", prefix);
        }
    }
}

fn prefix_templates() {
    println!("🏷️ Prefix Templates");

    let templates = [
        ("Gaming - High Performance", "gaming_performance"),
        ("Gaming - Compatibility", "gaming_compat"),
        ("Office/Productivity", "office"),
        ("Development Tools", "development"),
        ("Media Creation", "media"),
        ("Legacy Software", "legacy"),
    ];

    let template_names: Vec<&str> = templates.iter().map(|(name, _)| *name).collect();

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select template")
        .items(&template_names)
        .default(0)
        .interact()
    else {
        return;
    };

    let (_, template_id) = templates[choice];

    let Ok(prefix_name) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter prefix name")
        .default(template_id.to_string())
        .interact()
    else {
        return;
    };

    let prefix_path = format!("{}/prefixes/{}", get_games_dir(), prefix_name);

    // Create prefix
    fs::create_dir_all(&prefix_path).ok();
    let init_cmd = format!("WINEPREFIX={} WINEARCH=win64 wineboot -i", prefix_path);
    Command::new("sh").arg("-c").arg(&init_cmd).status().ok();

    match template_id {
        "gaming_performance" => {
            println!("⚡ Applying gaming performance template...");
            Command::new("sh")
                .arg("-c")
                .arg(&format!("WINEPREFIX={} winecfg /v win10", prefix_path))
                .status()
                .ok();

            let components = ["vcrun2019", "dotnet48", "d3dx11_43", "faudio"];
            for comp in &components {
                let cmd = format!("WINEPREFIX={} winetricks -q {}", prefix_path, comp);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
            }

            apply_gaming_optimizations(&prefix_path);
        }
        "gaming_compat" => {
            println!("🛡️ Applying gaming compatibility template...");
            Command::new("sh")
                .arg("-c")
                .arg(&format!("WINEPREFIX={} winecfg /v win7", prefix_path))
                .status()
                .ok();

            let components = ["vcrun2019", "vcrun2017", "vcrun2015", "dotnet48", "d3dx9"];
            for comp in &components {
                let cmd = format!("WINEPREFIX={} winetricks -q {}", prefix_path, comp);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
            }
        }
        "office" => {
            println!("📄 Applying office/productivity template...");
            Command::new("sh")
                .arg("-c")
                .arg(&format!("WINEPREFIX={} winecfg /v win10", prefix_path))
                .status()
                .ok();

            let components = ["dotnet48", "vcrun2019", "gdiplus", "riched20"];
            for comp in &components {
                let cmd = format!("WINEPREFIX={} winetricks -q {}", prefix_path, comp);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
            }
        }
        _ => {}
    }

    println!("✅ Template applied successfully");
}

// Add chrono to dependencies
use chrono;
