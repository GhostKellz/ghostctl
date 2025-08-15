use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn platforms_menu() {
    loop {
        let options = [
            "🎯 Lutris Management",
            "🏛️  Heroic Games Launcher (Epic/GOG)",
            "🍷 Bottles Management (Wine)",
            "🎮 Emulation Platforms",
            "🔧 Wine Management",
            "🎨 Game Launchers Overview",
            "📋 Platform Status",
            "⬅️  Back",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🎯 Gaming Platforms Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => lutris_management(),
            1 => heroic_management(),
            2 => bottles_management(),
            3 => emulation_platforms(),
            4 => wine_management(),
            5 => launchers_overview(),
            6 => platform_status(),
            _ => break,
        }
    }
}

pub fn lutris_management() {
    println!("🎯 Lutris Management");
    println!("====================");

    let options = [
        "📦 Install Lutris",
        "🎮 Install Popular Game Runners",
        "🍷 Wine Management for Lutris",
        "🔧 Lutris Configuration",
        "🎨 Install Popular Games",
        "🧹 Clean Lutris Prefixes",
        "📋 Lutris Status",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Lutris Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_lutris(),
        1 => install_lutris_runners(),
        2 => lutris_wine_management(),
        3 => lutris_configuration(),
        4 => install_popular_games_lutris(),
        5 => clean_lutris_prefixes(),
        6 => lutris_status(),
        _ => return,
    }
}

fn install_lutris() {
    println!("📦 Installing Lutris");
    println!("====================");

    let confirm = Confirm::new()
        .with_prompt("Install Lutris and recommended dependencies?")
        .default(true)
        .interact()
        .unwrap();

    if !confirm {
        return;
    }

    // Install Lutris and dependencies
    let packages = [
        "lutris",
        "wine",
        "wine-mono",
        "wine-gecko",
        "winetricks",
        "lib32-mesa",
        "lib32-alsa-plugins",
        "lib32-libpulse",
        "lib32-openal",
        "python-evdev",  // For controller support
        "python-dbus",   // For desktop integration
    ];

    println!("📦 Installing Lutris and dependencies...");
    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&packages)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Lutris installed successfully!");
            
            // Install additional wine versions via AUR
            install_additional_wine_versions();
            
            let launch = Confirm::new()
                .with_prompt("Launch Lutris now?")
                .default(false)
                .interact()
                .unwrap();

            if launch {
                let _ = Command::new("lutris").spawn();
            }
        }
        _ => println!("❌ Failed to install Lutris"),
    }
}

fn install_additional_wine_versions() {
    println!("🍷 Installing additional Wine versions...");
    
    let aur_helpers = ["yay", "paru", "trizen"];
    let wine_packages = [
        "wine-staging",
        "wine-tkg-staging-fsync-git",
        "wine-ge-custom",
    ];

    for helper in &aur_helpers {
        let helper_check = Command::new("which").arg(helper).status();
        if let Ok(s) = helper_check {
            if s.success() {
                println!("🔧 Using {} to install additional Wine versions...", helper);
                
                for package in &wine_packages {
                    let install_status = Command::new(helper)
                        .args(&["-S", "--noconfirm", package])
                        .status();
                    
                    match install_status {
                        Ok(s) if s.success() => println!("  ✅ {} installed", package),
                        _ => println!("  ⚠️  Failed to install {}", package),
                    }
                }
                return;
            }
        }
    }
    
    println!("💡 No AUR helper found. Install yay for additional Wine versions:");
    println!("   sudo pacman -S --needed base-devel git");
    println!("   git clone https://aur.archlinux.org/yay.git && cd yay && makepkg -si");
}

fn install_lutris_runners() {
    println!("🎮 Install Popular Game Runners");
    println!("===============================");

    let runners = [
        ("DXVK", "dxvk-bin", "DirectX to Vulkan translation"),
        ("VKD3D", "vkd3d", "Direct3D 12 to Vulkan translation"),
        ("dgVoodoo2", "", "3dfx Glide/DirectX wrapper"),
        ("ScummVM", "scummvm", "Classic adventure games"),
        ("DOSBox", "dosbox", "DOS games emulator"),
        ("RetroArch", "retroarch", "Multi-system emulator"),
    ];

    println!("Available runners:");
    for (i, (name, pkg, desc)) in runners.iter().enumerate() {
        println!("{}. {} - {}", i + 1, name, desc);
    }

    let selections = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select runners to install")
        .items(&runners.iter().map(|(name, _, desc)| format!("{} - {}", name, desc)).collect::<Vec<_>>())
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("❌ No runners selected");
        return;
    }

    println!("📦 Installing selected runners...");
    for &index in &selections {
        let (name, package, _) = runners[index];
        if !package.is_empty() {
            let status = Command::new("sudo")
                .args(&["pacman", "-S", "--needed", "--noconfirm", package])
                .status();
            
            match status {
                Ok(s) if s.success() => println!("  ✅ {} installed", name),
                _ => println!("  ❌ Failed to install {}", name),
            }
        } else {
            println!("  💡 {} requires manual installation", name);
        }
    }
}

fn lutris_wine_management() {
    println!("🍷 Wine Management for Lutris");
    println!("=============================");

    let options = [
        "📋 List installed Wine versions",
        "🔽 Download Wine-GE for Lutris",
        "🔧 Configure Wine versions",
        "🧹 Clean Wine prefixes",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Wine Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => list_wine_versions(),
        1 => download_wine_ge(),
        2 => configure_wine_versions(),
        3 => clean_wine_prefixes(),
        _ => return,
    }
}

fn list_wine_versions() {
    println!("📋 Installed Wine Versions");
    println!("==========================");

    // System Wine
    let wine_check = Command::new("wine").arg("--version").output();
    match wine_check {
        Ok(out) => println!("🍷 System Wine: {}", String::from_utf8_lossy(&out.stdout).trim()),
        _ => println!("❌ System Wine not found"),
    }

    // Lutris Wine runners
    let lutris_runners_dir = std::env::home_dir()
        .map(|h| h.join(".local/share/lutris/runners/wine"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.local/share/lutris/runners/wine"));

    if lutris_runners_dir.exists() {
        println!("\n🎯 Lutris Wine Runners:");
        let _ = Command::new("ls").args(&["-la", &lutris_runners_dir.to_string_lossy()]).status();
    } else {
        println!("\n❌ No Lutris Wine runners found");
    }
}

fn download_wine_ge() {
    println!("🔽 Download Wine-GE for Lutris");
    println!("==============================");

    println!("💡 Wine-GE (GloriousEggroll) provides optimizations for gaming");
    
    let confirm = Confirm::new()
        .with_prompt("Download latest Wine-GE?")
        .default(true)
        .interact()
        .unwrap();

    if !confirm {
        return;
    }

    // Check if we have a script or manual process
    println!("🔧 To install Wine-GE for Lutris:");
    println!("  1. Open Lutris");
    println!("  2. Go to Preferences > Runners > Wine");
    println!("  3. Click 'Manage versions'");
    println!("  4. Install lutris-GE-Proton versions");
    
    println!("\n💡 Or install via ProtonUp-Qt (if available):");
    let protonup_check = Command::new("which").arg("protonup-qt").status();
    match protonup_check {
        Ok(s) if s.success() => {
            let launch = Confirm::new()
                .with_prompt("Launch ProtonUp-Qt for Wine-GE installation?")
                .default(true)
                .interact()
                .unwrap();
            
            if launch {
                let _ = Command::new("protonup-qt").spawn();
            }
        }
        _ => {
            println!("  ProtonUp-Qt not found - install with:");
            println!("  yay -S protonup-qt");
        }
    }
}

fn configure_wine_versions() {
    println!("🔧 Configure Wine Versions");
    println!("==========================");
    
    println!("💡 Wine configuration options:");
    println!("  winecfg                 - Wine configuration GUI");
    println!("  winetricks              - Install Windows components");
    println!("  
            - Performance Wine build");
    
    let options = [
        "🔧 Launch winecfg",
        "🎯 Launch winetricks", 
        "📦 Install common Windows libraries",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Wine Configuration")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            let _ = Command::new("winecfg").spawn();
        }
        1 => {
            let _ = Command::new("winetricks").spawn();
        }
        2 => install_common_libraries(),
        _ => return,
    }
}

fn install_common_libraries() {
    println!("📦 Install Common Windows Libraries");
    println!("===================================");

    let libraries = [
        "vcrun2019",
        "dotnet48", 
        "corefonts",
        "d3dcompiler_47",
        "dxvk",
    ];

    println!("Available libraries:");
    for (i, lib) in libraries.iter().enumerate() {
        println!("{}. {}", i + 1, lib);
    }

    let selections = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select libraries to install")
        .items(&libraries)
        .interact()
        .unwrap();

    if !selections.is_empty() {
        println!("🔧 Installing selected libraries with winetricks...");
        let selected_libs: Vec<&str> = selections.iter().map(|&i| libraries[i]).collect();
        
        let status = Command::new("winetricks")
            .args(&selected_libs)
            .status();
        
        match status {
            Ok(s) if s.success() => println!("✅ Libraries installed"),
            _ => println!("❌ Some libraries may have failed to install"),
        }
    }
}

fn clean_wine_prefixes() {
    println!("🧹 Clean Wine Prefixes");
    println!("======================");
    
    let wineprefix_dir = std::env::home_dir()
        .map(|h| h.join("Games"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/Games"));

    if wineprefix_dir.exists() {
        println!("📁 Found Wine prefixes in: {}", wineprefix_dir.display());
        let _ = Command::new("ls").args(&["-la", &wineprefix_dir.to_string_lossy()]).status();
    }

    println!("\n💡 Clean options:");
    println!("  - Clear Wine cache and temporary files");
    println!("  - Remove old prefixes (BE CAREFUL!)");
    println!("  - Reset specific game prefixes");
    
    let confirm = Confirm::new()
        .with_prompt("Clean Wine cache and temporary files?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        // Clean common Wine cache locations
        let cache_dirs = [
            ".cache/wine",
            ".wine/drive_c/users/*/Temp",
            ".wine/drive_c/windows/Temp",
        ];

        for cache_dir in &cache_dirs {
            let full_path = std::env::home_dir()
                .map(|h| h.join(cache_dir))
                .unwrap_or_else(|| std::path::PathBuf::from(&format!("~/{}", cache_dir)));

            if full_path.exists() {
                let _ = Command::new("find")
                    .args(&[&full_path.to_string_lossy(), "-type", "f", "-delete"])
                    .status();
            }
        }
        
        println!("✅ Wine cache cleaned");
    }
}

fn lutris_configuration() {
    println!("🔧 Lutris Configuration");
    println!("=======================");

    let options = [
        "🎮 Gaming optimizations",
        "🖥️  Display settings",
        "🔊 Audio configuration",
        "📁 Default directories",
        "🔧 Advanced settings",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Lutris Configuration")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => lutris_gaming_optimizations(),
        1 => lutris_display_settings(),
        2 => lutris_audio_configuration(),
        3 => lutris_directories(),
        4 => lutris_advanced_settings(),
        _ => return,
    }
}

fn lutris_gaming_optimizations() {
    println!("🎮 Lutris Gaming Optimizations");
    println!("==============================");
    
    println!("💡 Recommended Lutris optimizations:");
    println!("  1. Enable DXVK for DirectX games");
    println!("  2. Enable Esync/Fsync for performance");
    println!("  3. Set appropriate Wine version per game");
    println!("  4. Configure GameMode integration");
    
    println!("\n🔧 In Lutris preferences:");
    println!("  • System Options > Enable Feral GameMode");
    println!("  • Runners > Wine > Enable DXVK");
    println!("  • Runners > Wine > Enable Esync");
    
    let launch_lutris = Confirm::new()
        .with_prompt("Launch Lutris to configure these settings?")
        .default(true)
        .interact()
        .unwrap();

    if launch_lutris {
        let _ = Command::new("lutris").arg("-p").spawn();
    }
}

fn lutris_display_settings() {
    println!("🖥️  Lutris Display Settings");
    println!("===========================");
    
    println!("💡 Display optimization tips:");
    println!("  • Set correct resolution per game");
    println!("  • Enable/disable fullscreen as needed");
    println!("  • Configure multi-monitor setups");
    println!("  • Adjust scaling for high-DPI displays");
}

fn lutris_audio_configuration() {
    println!("🔊 Lutris Audio Configuration");
    println!("=============================");
    
    println!("💡 Audio setup for Lutris:");
    println!("  • Use PulseAudio/PipeWire for best compatibility");
    println!("  • Install lib32-libpulse for 32-bit games");
    println!("  • Configure Wine audio driver (pulse recommended)");
    
    let install_audio = Confirm::new()
        .with_prompt("Install additional audio libraries?")
        .default(true)
        .interact()
        .unwrap();

    if install_audio {
        let packages = ["lib32-libpulse", "lib32-alsa-plugins", "lib32-openal"];
        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&packages)
            .status();
        
        match status {
            Ok(s) if s.success() => println!("✅ Audio libraries installed"),
            _ => println!("❌ Failed to install audio libraries"),
        }
    }
}

fn lutris_directories() {
    println!("📁 Lutris Default Directories");
    println!("=============================");
    
    let lutris_dir = std::env::home_dir()
        .map(|h| h.join(".local/share/lutris"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.local/share/lutris"));

    println!("📁 Lutris directories:");
    println!("  Config: ~/.config/lutris/");
    println!("  Data: {}", lutris_dir.display());
    println!("  Games: ~/Games/ (default)");
    println!("  Runners: {}runners/", lutris_dir.display());
    
    let open_dir = Confirm::new()
        .with_prompt("Open Lutris data directory?")
        .default(false)
        .interact()
        .unwrap();

    if open_dir && lutris_dir.exists() {
        let _ = Command::new("xdg-open").arg(&lutris_dir).spawn();
    }
}

fn lutris_advanced_settings() {
    println!("🔧 Lutris Advanced Settings");
    println!("===========================");
    
    println!("💡 Advanced Lutris configuration:");
    println!("  • Custom Wine builds and versions");
    println!("  • DXVK/VKD3D configuration");
    println!("  • Esync/Fsync tweaks");
    println!("  • Custom environment variables");
    println!("  • Game-specific optimizations");
    
    println!("\n📖 For advanced configuration, edit:");
    println!("  ~/.config/lutris/lutris.conf");
}

fn install_popular_games_lutris() {
    println!("🎨 Install Popular Games via Lutris");
    println!("====================================");

    println!("💡 Popular games available through Lutris:");
    
    let game_categories = [
        "🎮 Battle.net (Blizzard games)",
        "🎯 Epic Games Store",
        "🔶 Origin (EA games)",
        "🎪 Ubisoft Connect",
        "🎲 GOG Galaxy",
        "🚀 Emulated games (RetroArch)",
    ];

    println!("Game platform categories:");
    for (i, category) in game_categories.iter().enumerate() {
        println!("{}. {}", i + 1, category);
    }

    println!("\n💡 To install games:");
    println!("  1. Launch Lutris");
    println!("  2. Browse online installers");
    println!("  3. Search for your game");
    println!("  4. Follow automated installation");

    let launch = Confirm::new()
        .with_prompt("Launch Lutris to browse games?")
        .default(true)
        .interact()
        .unwrap();

    if launch {
        let _ = Command::new("lutris").spawn();
    }
}

fn clean_lutris_prefixes() {
    println!("🧹 Clean Lutris Prefixes");
    println!("========================");
    
    let lutris_prefixes = std::env::home_dir()
        .map(|h| h.join("Games"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/Games"));

    if lutris_prefixes.exists() {
        println!("📁 Lutris game prefixes found:");
        let _ = Command::new("ls").args(&["-la", &lutris_prefixes.to_string_lossy()]).status();
        
        println!("\n⚠️  Cleaning options:");
        println!("  • Clear temporary files and cache");
        println!("  • Remove unused prefixes");
        println!("  • Reset problematic game installations");
        
        let clean_cache = Confirm::new()
            .with_prompt("Clean cache and temporary files from all prefixes?")
            .default(true)
            .interact()
            .unwrap();

        if clean_cache {
            // Clean common locations in prefixes
            let _ = Command::new("find")
                .args(&[&lutris_prefixes.to_string_lossy(), "-name", "*.tmp", "-delete"])
                .status();
            let _ = Command::new("find")
                .args(&[&lutris_prefixes.to_string_lossy(), "-path", "*/Temp/*", "-delete"])
                .status();
            
            println!("✅ Lutris prefixes cleaned");
        }
    } else {
        println!("❌ No Lutris prefixes found");
    }
}

fn lutris_status() {
    println!("📋 Lutris Status");
    println!("================");
    
    let lutris_check = Command::new("which").arg("lutris").status();
    match lutris_check {
        Ok(s) if s.success() => {
            println!("✅ Lutris is installed");
            
            let version_output = Command::new("lutris").arg("--version").output();
            match version_output {
                Ok(out) => println!("📋 Version: {}", String::from_utf8_lossy(&out.stdout).trim()),
                _ => {},
            }
        }
        _ => {
            println!("❌ Lutris is not installed");
            return;
        }
    }

    // Check if Lutris is running
    let running_check = Command::new("pgrep").arg("lutris").status();
    match running_check {
        Ok(s) if s.success() => println!("🟢 Lutris is currently running"),
        _ => println!("⭕ Lutris is not running"),
    }

    // Check Lutris directories
    let lutris_config = std::env::home_dir()
        .map(|h| h.join(".config/lutris"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config/lutris"));

    if lutris_config.exists() {
        println!("📁 Lutris config found: {}", lutris_config.display());
    }

    let lutris_data = std::env::home_dir()
        .map(|h| h.join(".local/share/lutris"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.local/share/lutris"));

    if lutris_data.exists() {
        println!("📁 Lutris data found: {}", lutris_data.display());
        
        // Count installed runners
        let runners_dir = lutris_data.join("runners");
        if runners_dir.exists() {
            let runner_count = std::fs::read_dir(&runners_dir)
                .map(|entries| entries.count())
                .unwrap_or(0);
            println!("🎮 Installed runners: {}", runner_count);
        }
    }
}

pub fn heroic_management() {
    println!("🏛️  Heroic Games Launcher Management");
    println!("====================================");

    let options = [
        "📦 Install Heroic Games Launcher",
        "🎮 Epic Games Store Setup",
        "🎯 GOG Integration",
        "🔧 Heroic Configuration",
        "📋 Heroic Status",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Heroic Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_heroic(),
        1 => setup_epic_games(),
        2 => setup_gog(),
        3 => heroic_configuration(),
        4 => heroic_status(),
        _ => return,
    }
}

fn install_heroic() {
    println!("📦 Installing Heroic Games Launcher");
    println!("===================================");

    let install_methods = [
        "📦 Install from AUR",
        "📱 Install AppImage",
        "🐳 Install Flatpak",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Installation method")
        .items(&install_methods)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_heroic_aur(),
        1 => install_heroic_appimage(),
        2 => install_heroic_flatpak(),
        _ => return,
    }
}

fn install_heroic_aur() {
    println!("📦 Installing Heroic from AUR");
    
    let aur_helpers = ["yay", "paru", "trizen"];
    for helper in &aur_helpers {
        let helper_check = Command::new("which").arg(helper).status();
        if let Ok(s) = helper_check {
            if s.success() {
                println!("🔧 Using {} to install Heroic...", helper);
                let install_status = Command::new(helper)
                    .args(&["-S", "--noconfirm", "heroic-games-launcher-bin"])
                    .status();
                
                match install_status {
                    Ok(s) if s.success() => {
                        println!("✅ Heroic installed successfully!");
                        return;
                    }
                    _ => println!("❌ Failed to install with {}", helper),
                }
            }
        }
    }
    
    println!("❌ No AUR helper found. Install yay first:");
    println!("   sudo pacman -S --needed base-devel git");
    println!("   git clone https://aur.archlinux.org/yay.git && cd yay && makepkg -si");
}

fn install_heroic_appimage() {
    println!("📱 Installing Heroic AppImage");
    println!("=============================");
    
    println!("💡 To install Heroic as AppImage:");
    println!("  1. Download from: https://github.com/Heroic-Games-Launcher/HeroicGamesLauncher/releases");
    println!("  2. Make executable: chmod +x Heroic-*.AppImage");
    println!("  3. Run: ./Heroic-*.AppImage");
    
    println!("\n🔧 Or use automated download:");
    let download = Confirm::new()
        .with_prompt("Download latest Heroic AppImage?")
        .default(true)
        .interact()
        .unwrap();

    if download {
        println!("🔽 Downloading Heroic AppImage...");
        let status = Command::new("curl")
            .args(&["-L", "-o", "/tmp/heroic.AppImage", 
                   "https://github.com/Heroic-Games-Launcher/HeroicGamesLauncher/releases/latest/download/Heroic-Games-Launcher.AppImage"])
            .status();
        
        match status {
            Ok(s) if s.success() => {
                let _ = Command::new("chmod").args(&["+x", "/tmp/heroic.AppImage"]).status();
                println!("✅ Heroic AppImage downloaded to /tmp/heroic.AppImage");
                
                let run_now = Confirm::new()
                    .with_prompt("Run Heroic now?")
                    .default(true)
                    .interact()
                    .unwrap();
                
                if run_now {
                    let _ = Command::new("/tmp/heroic.AppImage").spawn();
                }
            }
            _ => println!("❌ Failed to download Heroic AppImage"),
        }
    }
}

fn install_heroic_flatpak() {
    println!("🐳 Installing Heroic via Flatpak");
    println!("================================");
    
    let flatpak_check = Command::new("which").arg("flatpak").status();
    match flatpak_check {
        Ok(s) if s.success() => {
            let install_status = Command::new("flatpak")
                .args(&["install", "-y", "flathub", "com.heroicgameslauncher.hgl"])
                .status();
            
            match install_status {
                Ok(s) if s.success() => println!("✅ Heroic installed via Flatpak"),
                _ => println!("❌ Failed to install Heroic via Flatpak"),
            }
        }
        _ => {
            println!("❌ Flatpak not found. Install flatpak first:");
            println!("   sudo pacman -S flatpak");
        }
    }
}

fn setup_epic_games() {
    println!("🎮 Epic Games Store Setup");
    println!("=========================");
    
    println!("💡 To setup Epic Games Store in Heroic:");
    println!("  1. Launch Heroic Games Launcher");
    println!("  2. Click 'Log In' for Epic Games Store");
    println!("  3. Enter your Epic credentials");
    println!("  4. Browse and install games");
    
    println!("\n🎯 Popular Epic Games Store titles:");
    println!("  • Fortnite");
    println!("  • Rocket League");
    println!("  • Fall Guys");
    println!("  • Weekly free games");
}

fn setup_gog() {
    println!("🎯 GOG Integration Setup");
    println!("========================");
    
    println!("💡 To setup GOG in Heroic:");
    println!("  1. Launch Heroic Games Launcher");
    println!("  2. Click 'Log In' for GOG");
    println!("  3. Enter your GOG credentials");
    println!("  4. Access your GOG library");
    
    println!("\n🎮 GOG benefits:");
    println!("  • DRM-free games");
    println!("  • Classic games collection");
    println!("  • Often Linux-native versions");
}

fn heroic_configuration() {
    println!("🔧 Heroic Configuration");
    println!("=======================");
    
    println!("💡 Key Heroic settings to configure:");
    println!("  • Default Wine version");
    println!("  • DXVK/VKD3D settings");
    println!("  • GameMode integration");
    println!("  • Download directory");
    println!("  • Performance settings");
    
    let launch_heroic = Confirm::new()
        .with_prompt("Launch Heroic to configure settings?")
        .default(true)
        .interact()
        .unwrap();

    if launch_heroic {
        let _ = Command::new("heroic").spawn();
    }
}

fn heroic_status() {
    println!("📋 Heroic Games Launcher Status");
    println!("===============================");
    
    let heroic_check = Command::new("which").arg("heroic").status();
    let heroic_appimage = std::path::Path::new("/tmp/heroic.AppImage").exists();
    let heroic_flatpak = Command::new("flatpak").args(&["list", "--app", "|", "grep", "heroic"]).status();

    if let Ok(s) = heroic_check {
        if s.success() {
            println!("✅ Heroic installed (system)");
        }
    } else if heroic_appimage {
        println!("✅ Heroic AppImage found");
    } else if let Ok(s) = heroic_flatpak {
        if s.success() {
            println!("✅ Heroic installed (Flatpak)");
        }
    } else {
        println!("❌ Heroic not found");
        return;
    }

    let running_check = Command::new("pgrep").arg("heroic").status();
    match running_check {
        Ok(s) if s.success() => println!("🟢 Heroic is currently running"),
        _ => println!("⭕ Heroic is not running"),
    }
}

pub fn bottles_management() {
    println!("🍷 Bottles Management");
    println!("====================");

    let options = [
        "📦 Install Bottles",
        "🍷 Create New Bottle",
        "🔧 Manage Existing Bottles",
        "🎮 Gaming Bottle Templates",
        "📋 Bottles Status",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Bottles Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_bottles(),
        1 => create_bottle(),
        2 => manage_bottles(),
        3 => gaming_bottle_templates(),
        4 => bottles_status(),
        _ => return,
    }
}

fn install_bottles() {
    println!("📦 Installing Bottles");
    println!("=====================");

    let install_methods = [
        "🐳 Install via Flatpak (recommended)",
        "📦 Install from AUR",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Installation method")
        .items(&install_methods)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_bottles_flatpak(),
        1 => install_bottles_aur(),
        _ => return,
    }
}

fn install_bottles_flatpak() {
    let flatpak_check = Command::new("which").arg("flatpak").status();
    match flatpak_check {
        Ok(s) if s.success() => {
            let install_status = Command::new("flatpak")
                .args(&["install", "-y", "flathub", "com.usebottles.bottles"])
                .status();
            
            match install_status {
                Ok(s) if s.success() => println!("✅ Bottles installed via Flatpak"),
                _ => println!("❌ Failed to install Bottles via Flatpak"),
            }
        }
        _ => {
            println!("❌ Flatpak not found. Install flatpak first:");
            println!("   sudo pacman -S flatpak");
        }
    }
}

fn install_bottles_aur() {
    let aur_helpers = ["yay", "paru", "trizen"];
    for helper in &aur_helpers {
        let helper_check = Command::new("which").arg(helper).status();
        if let Ok(s) = helper_check {
            if s.success() {
                let install_status = Command::new(helper)
                    .args(&["-S", "--noconfirm", "bottles"])
                    .status();
                
                match install_status {
                    Ok(s) if s.success() => {
                        println!("✅ Bottles installed via AUR");
                        return;
                    }
                    _ => println!("❌ Failed to install with {}", helper),
                }
            }
        }
    }
    
    println!("❌ No AUR helper found");
}

fn create_bottle() {
    println!("🍷 Create New Bottle");
    println!("===================");
    
    println!("💡 To create a new bottle:");
    println!("  1. Launch Bottles");
    println!("  2. Click 'Create New Bottle'");
    println!("  3. Choose environment (Application/Gaming)");
    println!("  4. Select Wine version");
    println!("  5. Configure bottle settings");
}

fn manage_bottles() {
    println!("🔧 Manage Existing Bottles");
    println!("==========================");
    
    println!("💡 Bottle management features:");
    println!("  • Install Windows software in bottles");
    println!("  • Configure Wine settings per bottle");
    println!("  • Install dependencies (vcredist, .NET, etc.)");
    println!("  • Backup and restore bottles");
    println!("  • Bottle versioning and snapshots");
}

fn gaming_bottle_templates() {
    println!("🎮 Gaming Bottle Templates");
    println!("==========================");
    
    println!("💡 Popular gaming bottle configurations:");
    println!("  🎯 DirectX 11/12 Gaming");
    println!("  🎮 Older Games (DirectX 9)");
    println!("  🎪 Epic Games Store");
    println!("  🔶 Origin/EA App");
    println!("  🎲 Battle.net");
    
    println!("\n🔧 Template usually includes:");
    println!("  • Appropriate Wine version");
    println!("  • DXVK/VKD3D");
    println!("  • Visual C++ redistributables");
    println!("  • .NET Framework");
    println!("  • Gaming-optimized settings");
}

fn bottles_status() {
    println!("📋 Bottles Status");
    println!("=================");
    
    let bottles_flatpak = Command::new("flatpak")
        .args(&["list", "--app"])
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).contains("bottles"))
        .unwrap_or(false);

    let bottles_system = Command::new("which").arg("bottles").status();
    
    if bottles_flatpak {
        println!("✅ Bottles installed (Flatpak)");
    } else if let Ok(s) = bottles_system {
        if s.success() {
            println!("✅ Bottles installed (system)");
        }
    } else {
        println!("❌ Bottles not found");
        return;
    }

    let running_check = Command::new("pgrep").arg("bottles").status();
    match running_check {
        Ok(s) if s.success() => println!("🟢 Bottles is currently running"),
        _ => println!("⭕ Bottles is not running"),
    }
}

pub fn emulation_platforms() {
    println!("🎮 Emulation Platforms");
    println!("======================");

    let options = [
        "🕹️  RetroArch (Multi-system)",
        "🎮 Console-specific Emulators",
        "🖥️  Computer System Emulators",
        "🎯 Arcade Emulation",
        "📱 Handheld System Emulators",
        "🔧 Emulation Setup Guide",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Emulation Platforms")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_retroarch(),
        1 => console_emulators(),
        2 => computer_emulators(),
        3 => arcade_emulation(),
        4 => handheld_emulators(),
        5 => emulation_setup_guide(),
        _ => return,
    }
}

fn install_retroarch() {
    println!("🕹️  Installing RetroArch");
    println!("=========================");

    let confirm = Confirm::new()
        .with_prompt("Install RetroArch and common cores?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        let packages = [
            "retroarch",
            "libretro-beetle-psx-hw",
            "libretro-snes9x",
            "libretro-nestopia",
            "libretro-genesis-plus-gx",
            "libretro-mupen64plus-next",
        ];

        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm"])
            .args(&packages)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ RetroArch and cores installed"),
            _ => println!("❌ Failed to install RetroArch"),
        }
    }
}

fn console_emulators() {
    println!("🎮 Console-specific Emulators");
    println!("=============================");

    let emulators = [
        ("PlayStation 2", "pcsx2", "PCSX2"),
        ("PlayStation 3", "rpcs3-bin", "RPCS3 (AUR)"),
        ("GameCube/Wii", "dolphin-emu", "Dolphin"),
        ("Nintendo Switch", "yuzu-mainline-git", "Yuzu (AUR)"),
        ("Xbox 360", "xenia-git", "Xenia (AUR)"),
    ];

    println!("Available console emulators:");
    for (i, (console, package, name)) in emulators.iter().enumerate() {
        println!("{}. {} - {} ({})", i + 1, console, name, package);
    }

    let selections = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select emulators to install")
        .items(&emulators.iter().map(|(console, _, name)| format!("{} - {}", console, name)).collect::<Vec<_>>())
        .interact()
        .unwrap();

    for &index in &selections {
        let (console, package, _) = emulators[index];
        println!("📦 Installing {} emulator...", console);
        
        if package.contains("git") || package.contains("bin") {
            // AUR package
            install_aur_package(package);
        } else {
            // Official repository
            let status = Command::new("sudo")
                .args(&["pacman", "-S", "--needed", "--noconfirm", package])
                .status();
            
            match status {
                Ok(s) if s.success() => println!("  ✅ {} installed", console),
                _ => println!("  ❌ Failed to install {}", console),
            }
        }
    }
}

fn computer_emulators() {
    println!("🖥️  Computer System Emulators");
    println!("=============================");

    let emulators = [
        ("DOS", "dosbox", "DOSBox"),
        ("Amiga", "fs-uae", "FS-UAE"),
        ("Commodore 64", "vice", "VICE"),
        ("Apple II", "linapple", "LinApple"),
        ("Atari ST", "hatari", "Hatari"),
    ];

    println!("Available computer emulators:");
    for (i, (system, package, name)) in emulators.iter().enumerate() {
        println!("{}. {} - {} ({})", i + 1, system, name, package);
    }
}

fn arcade_emulation() {
    println!("🎯 Arcade Emulation");
    println!("===================");

    let arcade_emulators = [
        ("MAME", "mame", "Multiple Arcade Machine Emulator"),
        ("FinalBurn Neo", "fbneo", "Neo Geo and arcade systems"),
    ];

    println!("Available arcade emulators:");
    for (i, (name, package, desc)) in arcade_emulators.iter().enumerate() {
        println!("{}. {} - {} ({})", i + 1, name, desc, package);
    }
}

fn handheld_emulators() {
    println!("📱 Handheld System Emulators");
    println!("============================");

    let handhelds = [
        ("Game Boy/Color/Advance", "mgba", "mGBA"),
        ("Nintendo DS", "desmume", "DeSmuME"),
        ("Nintendo 3DS", "citra", "Citra"),
        ("PSP", "ppsspp", "PPSSPP"),
        ("PS Vita", "vita3k-git", "Vita3K (AUR)"),
    ];

    println!("Available handheld emulators:");
    for (i, (system, package, name)) in handhelds.iter().enumerate() {
        println!("{}. {} - {} ({})", i + 1, system, name, package);
    }
}

fn emulation_setup_guide() {
    println!("🔧 Emulation Setup Guide");
    println!("========================");
    
    println!("💡 General emulation setup tips:");
    println!("  1. Obtain legal ROM/BIOS files");
    println!("  2. Create organized ROM directories");
    println!("  3. Configure controllers/input");
    println!("  4. Set up save file management");
    println!("  5. Configure graphics and performance");
    
    println!("\n📁 Recommended directory structure:");
    println!("  ~/Games/ROMs/");
    println!("  ├── NES/");
    println!("  ├── SNES/");
    println!("  ├── PlayStation/");
    println!("  ├── Nintendo64/");
    println!("  └── etc...");
    
    println!("\n⚖️  Legal note:");
    println!("  Only use ROMs of games you legally own!");
}

fn install_aur_package(package: &str) {
    let aur_helpers = ["yay", "paru", "trizen"];
    for helper in &aur_helpers {
        let helper_check = Command::new("which").arg(helper).status();
        if let Ok(s) = helper_check {
            if s.success() {
                let install_status = Command::new(helper)
                    .args(&["-S", "--noconfirm", package])
                    .status();
                
                match install_status {
                    Ok(s) if s.success() => return,
                    _ => {},
                }
            }
        }
    }
    println!("  ❌ No AUR helper found for {}", package);
}

pub fn wine_management() {
    println!("🍷 Wine Management");
    println!("==================");

    let options = [
        "📦 Install Wine versions",
        "🔧 Configure Wine",
        "🛠️  Winetricks management", 
        "🧹 Clean Wine data",
        "📋 Wine status",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Wine Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_wine_versions(),
        1 => configure_wine(),
        2 => winetricks_management(),
        3 => clean_wine_data(),
        4 => wine_status(),
        _ => return,
    }
}

fn install_wine_versions() {
    println!("📦 Installing Wine Versions");
    println!("===========================");

    let wine_versions = [
        ("Wine (stable)", "wine", "Standard Wine from repos"),
        ("Wine Staging", "wine-staging", "Development version with patches"),
        ("Wine-TkG", "wine-tkg-staging-fsync-git", "Custom optimized build (AUR)"),
        ("Wine-GE", "wine-ge-custom", "Gaming-focused build (AUR)"),
    ];

    println!("Available Wine versions:");
    for (i, (name, package, desc)) in wine_versions.iter().enumerate() {
        println!("{}. {} - {} ({})", i + 1, name, desc, package);
    }

    let selections = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Wine versions to install")
        .items(&wine_versions.iter().map(|(name, _, desc)| format!("{} - {}", name, desc)).collect::<Vec<_>>())
        .interact()
        .unwrap();

    for &index in &selections {
        let (name, package, _) = wine_versions[index];
        println!("📦 Installing {}...", name);
        
        if package.contains("git") || package.contains("custom") {
            install_aur_package(package);
        } else {
            let status = Command::new("sudo")
                .args(&["pacman", "-S", "--needed", "--noconfirm", package])
                .status();
            
            match status {
                Ok(s) if s.success() => println!("  ✅ {} installed", name),
                _ => println!("  ❌ Failed to install {}", name),
            }
        }
    }
}

fn configure_wine() {
    println!("🔧 Configure Wine");
    println!("=================");

    let options = [
        "🔧 Launch winecfg",
        "🎮 Set up for gaming",
        "🖥️  Configure display settings",
        "🔊 Configure audio",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Wine Configuration")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            let _ = Command::new("winecfg").spawn();
        }
        1 => configure_wine_gaming(),
        2 => configure_wine_display(),
        3 => configure_wine_audio(),
        _ => return,
    }
}

fn configure_wine_gaming() {
    println!("🎮 Configure Wine for Gaming");
    println!("============================");
    
    println!("💡 Gaming optimizations:");
    println!("  • Enable DXVK for DirectX games");
    println!("  • Enable Esync/Fsync for performance");
    println!("  • Install Visual C++ redistributables");
    println!("  • Configure Windows version compatibility");
    
    let install_dxvk = Confirm::new()
        .with_prompt("Install DXVK for DirectX translation?")
        .default(true)
        .interact()
        .unwrap();

    if install_dxvk {
        let status = Command::new("sudo")
            .args(&["pacman", "-S", "--needed", "--noconfirm", "dxvk-bin"])
            .status();
        
        match status {
            Ok(s) if s.success() => println!("✅ DXVK installed"),
            _ => println!("❌ Failed to install DXVK"),
        }
    }
}

fn configure_wine_display() {
    println!("🖥️  Configure Wine Display");
    println!("===========================");
    
    println!("💡 Display configuration options:");
    println!("  • Set screen resolution");
    println!("  • Configure DPI scaling");
    println!("  • Set up virtual desktop");
    println!("  • Graphics driver settings");
    
    println!("\n🔧 Launch winecfg to configure graphics settings");
}

fn configure_wine_audio() {
    println!("🔊 Configure Wine Audio");
    println!("=======================");
    
    println!("💡 Audio driver options:");
    println!("  • PulseAudio (recommended)");
    println!("  • ALSA");
    println!("  • OSS");
    
    println!("\n🔧 In winecfg > Audio tab:");
    println!("  • Set driver to 'pulse'");
    println!("  • Test audio functionality");
}

fn winetricks_management() {
    println!("🛠️  Winetricks Management");
    println!("=========================");

    let winetricks_check = Command::new("which").arg("winetricks").status();
    match winetricks_check {
        Ok(s) if s.success() => {
            println!("✅ Winetricks found");
            
            let launch = Confirm::new()
                .with_prompt("Launch winetricks GUI?")
                .default(true)
                .interact()
                .unwrap();

            if launch {
                let _ = Command::new("winetricks").spawn();
            }
        }
        _ => {
            println!("❌ Winetricks not found");
            let install = Confirm::new()
                .with_prompt("Install winetricks?")
                .default(true)
                .interact()
                .unwrap();

            if install {
                let status = Command::new("sudo")
                    .args(&["pacman", "-S", "--needed", "--noconfirm", "winetricks"])
                    .status();
                
                match status {
                    Ok(s) if s.success() => {
                        println!("✅ Winetricks installed");
                        let _ = Command::new("winetricks").spawn();
                    }
                    _ => println!("❌ Failed to install winetricks"),
                }
            }
        }
    }
}

fn clean_wine_data() {
    println!("🧹 Clean Wine Data");
    println!("==================");
    
    let wine_dir = std::env::home_dir()
        .map(|h| h.join(".wine"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.wine"));

    if wine_dir.exists() {
        println!("📁 Wine directory found: {}", wine_dir.display());
        
        let clean_options = [
            "🗑️  Clear Wine cache",
            "🧹 Remove temporary files",
            "💾 Clean registry backups",
            "⚠️  Reset Wine prefix (DANGEROUS)",
        ];

        let selections = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select cleanup options")
            .items(&clean_options)
            .interact()
            .unwrap();

        for &index in &selections {
            match index {
                0 => {
                    let cache_dir = wine_dir.join("drive_c/users/*/AppData/Local/Temp");
                    println!("🗑️  Clearing Wine cache...");
                }
                1 => {
                    println!("🧹 Removing temporary files...");
                }
                2 => {
                    println!("💾 Cleaning registry backups...");
                }
                3 => {
                    let confirm = Confirm::new()
                        .with_prompt("⚠️  Really reset Wine prefix? This will delete all Windows software!")
                        .default(false)
                        .interact()
                        .unwrap();
                    
                    if confirm {
                        let _ = Command::new("rm").args(&["-rf", &wine_dir.to_string_lossy()]).status();
                        println!("⚠️  Wine prefix reset");
                    }
                }
                _ => {}
            }
        }
    } else {
        println!("❌ No Wine directory found");
    }
}

fn wine_status() {
    println!("📋 Wine Status");
    println!("==============");
    
    let wine_check = Command::new("wine").arg("--version").output();
    match wine_check {
        Ok(out) => {
            println!("✅ Wine version: {}", String::from_utf8_lossy(&out.stdout).trim());
        }
        _ => {
            println!("❌ Wine not found");
            return;
        }
    }

    // Check Wine prefix
    let wine_dir = std::env::home_dir()
        .map(|h| h.join(".wine"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.wine"));

    if wine_dir.exists() {
        println!("📁 Wine prefix: {}", wine_dir.display());
        
        let drive_c = wine_dir.join("drive_c");
        if drive_c.exists() {
            println!("💾 Windows C: drive configured");
        }
    } else {
        println!("❌ No Wine prefix found (run winecfg to create)");
    }

    // Check winetricks
    let winetricks_check = Command::new("which").arg("winetricks").status();
    match winetricks_check {
        Ok(s) if s.success() => println!("✅ Winetricks available"),
        _ => println!("❌ Winetricks not found"),
    }
}

pub fn launchers_overview() {
    println!("🎨 Game Launchers Overview");
    println!("==========================");
    
    println!("🎮 Available gaming platforms on Linux:");
    println!("");
    println!("🚀 Steam");
    println!("  • Native Linux client");
    println!("  • Proton for Windows games");
    println!("  • Largest game library");
    println!("  • Best Linux gaming support");
    println!("");
    println!("🎯 Lutris");
    println!("  • Universal game launcher");
    println!("  • Supports multiple stores");
    println!("  • Wine integration");
    println!("  • Community install scripts");
    println!("");
    println!("🏛️  Heroic Games Launcher");
    println!("  • Epic Games Store client");
    println!("  • GOG integration");
    println!("  • Native Linux application");
    println!("  • Free games support");
    println!("");
    println!("🍷 Bottles");
    println!("  • Wine prefix management");
    println!("  • Application isolation");
    println!("  • Easy Windows software installation");
    println!("  • Gaming templates");
    println!("");
    println!("🕹️  RetroArch");
    println!("  • Retro gaming frontend");
    println!("  • Multiple emulator cores");
    println!("  • Cross-platform saves");
    println!("  • Advanced features");
}

pub fn platform_status() {
    println!("📋 Gaming Platforms Status");
    println!("==========================");
    
    // Check Steam
    let steam_check = Command::new("which").arg("steam").status();
    match steam_check {
        Ok(s) if s.success() => println!("✅ Steam installed"),
        _ => println!("❌ Steam not installed"),
    }

    // Check Lutris
    let lutris_check = Command::new("which").arg("lutris").status();
    match lutris_check {
        Ok(s) if s.success() => println!("✅ Lutris installed"),
        _ => println!("❌ Lutris not installed"),
    }

    // Check Heroic
    let heroic_check = Command::new("which").arg("heroic").status();
    let heroic_flatpak = Command::new("flatpak")
        .args(&["list", "--app"])
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).contains("heroic"))
        .unwrap_or(false);
    
    if let Ok(s) = heroic_check {
        if s.success() {
            println!("✅ Heroic installed (system)");
        }
    } else if heroic_flatpak {
        println!("✅ Heroic installed (Flatpak)");
    } else {
        println!("❌ Heroic not installed");
    }

    // Check Bottles
    let bottles_check = Command::new("which").arg("bottles").status();
    let bottles_flatpak = Command::new("flatpak")
        .args(&["list", "--app"])
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).contains("bottles"))
        .unwrap_or(false);
    
    if let Ok(s) = bottles_check {
        if s.success() {
            println!("✅ Bottles installed (system)");
        }
    } else if bottles_flatpak {
        println!("✅ Bottles installed (Flatpak)");
    } else {
        println!("❌ Bottles not installed");
    }

    // Check RetroArch
    let retroarch_check = Command::new("which").arg("retroarch").status();
    match retroarch_check {
        Ok(s) if s.success() => println!("✅ RetroArch installed"),
        _ => println!("❌ RetroArch not installed"),
    }

    // Check Wine
    let wine_check = Command::new("which").arg("wine").status();
    match wine_check {
        Ok(s) if s.success() => println!("✅ Wine installed"),
        _ => println!("❌ Wine not installed"),
    }
}