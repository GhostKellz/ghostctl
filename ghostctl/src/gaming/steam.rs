use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn steam_menu() {
    loop {
        let options = [
            "🔧 Install Steam (with multilib setup)",
            "🚀 Proton Management",
            "📦 ProtonUp-Qt Installation",
            "🎮 Steam Library Optimization",
            "🔄 Steam Prefix Management",
            "🛠️  Steam Troubleshooting",
            "📋 Steam Status & Info",
            "⬅️  Back",
        ];

        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🚀 Steam & Proton Management")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match choice {
            0 => install_steam(),
            1 => proton_management(),
            2 => install_protonup_qt(),
            3 => steam_library_optimization(),
            4 => steam_prefix_management(),
            5 => steam_troubleshooting(),
            6 => steam_status(),
            _ => break,
        }
    }
}

pub fn install_steam() {
    println!("🚀 Steam Installation with Multilib Setup");
    println!("=========================================");

    // Check if multilib is enabled
    let multilib_check = Command::new("grep")
        .args(&["-E", "^\\[multilib\\]", "/etc/pacman.conf"])
        .output();

    match multilib_check {
        Ok(out) if out.stdout.is_empty() => {
            println!("❌ Multilib repository is not enabled!");
            let Ok(enable_multilib) = Confirm::new()
                .with_prompt("Enable multilib repository? (Required for Steam)")
                .default(true)
                .interact()
            else {
                return;
            };

            if enable_multilib {
                enable_multilib_repo();
            } else {
                println!("❌ Cannot install Steam without multilib. Aborting.");
                return;
            }
        }
        Ok(_) => println!("✅ Multilib repository is already enabled"),
        Err(_) => println!("⚠️  Could not check multilib status"),
    }

    // Update package database
    println!("🔄 Updating package database...");
    let update_status = Command::new("sudo").args(&["pacman", "-Sy"]).status();

    match update_status {
        Ok(s) if s.success() => println!("✅ Package database updated"),
        _ => {
            println!("❌ Failed to update package database");
            return;
        }
    }

    // Install Steam and dependencies
    println!("📦 Installing Steam and dependencies...");
    let packages = [
        "steam",
        "lib32-mesa",
        "lib32-alsa-plugins",
        "lib32-libpulse",
        "lib32-openal",
        "lib32-libva",
        "lib32-libxss",
        "lib32-gst-plugins-base-libs",
    ];

    let install_status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&packages)
        .status();

    match install_status {
        Ok(s) if s.success() => {
            println!("✅ Steam and dependencies installed successfully!");
            println!(
                "💡 You can now launch Steam from your application menu or by running 'steam' in the terminal"
            );

            let Ok(launch_steam) = Confirm::new()
                .with_prompt("Launch Steam now?")
                .default(false)
                .interact()
            else {
                return;
            };

            if launch_steam {
                let _ = Command::new("steam").spawn();
                println!("🚀 Steam launched!");
            }
        }
        _ => println!("❌ Failed to install Steam"),
    }
}

fn enable_multilib_repo() {
    println!("🔧 Enabling multilib repository...");

    let backup_status = Command::new("sudo")
        .args(&["cp", "/etc/pacman.conf", "/etc/pacman.conf.backup"])
        .status();

    match backup_status {
        Ok(s) if s.success() => println!("✅ Backed up pacman.conf"),
        _ => println!("⚠️  Could not backup pacman.conf"),
    }

    // Uncomment multilib section
    let enable_status = Command::new("sudo")
        .arg("sed")
        .args(&[
            "-i",
            "/^#\\[multilib\\]/,/^#Include = \\/etc\\/pacman.d\\/mirrorlist/ s/^#//",
            "/etc/pacman.conf",
        ])
        .status();

    match enable_status {
        Ok(s) if s.success() => println!("✅ Multilib repository enabled"),
        _ => {
            println!("❌ Failed to enable multilib automatically");
            println!("💡 Please manually uncomment [multilib] section in /etc/pacman.conf");
        }
    }
}

pub fn proton_management() {
    println!("🚀 Proton Management");
    println!("===================");

    let options = [
        "📋 List Installed Proton Versions",
        "🔽 Install GloriousEggroll Proton",
        "🗑️  Remove Proton Version",
        "🎯 Set Default Proton Version",
        "🔄 Update All Proton Versions",
        "📁 Open Proton Directory",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Proton Management")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => list_proton_versions(),
        1 => install_proton_ge(),
        2 => remove_proton_version(),
        3 => set_default_proton(),
        4 => update_proton_versions(),
        5 => open_proton_directory(),
        _ => return,
    }
}

fn list_proton_versions() {
    println!("📋 Installed Proton Versions");
    println!("============================");

    let steam_dir = std::env::home_dir()
        .map(|h| h.join(".steam/steam/compatibilitytools.d"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.steam/steam/compatibilitytools.d"));

    if steam_dir.exists() {
        println!("📁 Proton installations in: {}", steam_dir.display());
        let _ = Command::new("ls")
            .args(&["-la", &steam_dir.to_string_lossy()])
            .status();
    } else {
        println!("❌ Steam compatibility tools directory not found");
        println!("💡 Steam may not be installed or never launched");
    }

    // Also check system-wide Proton
    println!("\n📦 System Proton packages:");
    let _ = Command::new("pacman").args(&["-Q"]).status();
    let _ = Command::new("bash")
        .arg("-c")
        .arg("pacman -Q | grep -i proton")
        .status();
}

fn install_proton_ge() {
    println!("🔽 Installing GloriousEggroll Proton");
    println!("===================================");

    println!("💡 This will download and install the latest Proton-GE");
    let Ok(confirm) = Confirm::new()
        .with_prompt("Continue with Proton-GE installation?")
        .default(true)
        .interact()
    else {
        return;
    };

    if !confirm {
        return;
    }

    // Check if ProtonUp-Qt is installed first
    let protonup_check = Command::new("which").arg("protonup-qt").status();
    match protonup_check {
        Ok(s) if s.success() => {
            println!("✅ ProtonUp-Qt found, launching GUI installer...");
            let _ = Command::new("protonup-qt").spawn();
        }
        _ => {
            println!("❌ ProtonUp-Qt not found. Installing via AUR...");
            install_protonup_qt_aur();
        }
    }
}

fn install_protonup_qt_aur() {
    println!("📦 Installing ProtonUp-Qt from AUR...");

    // Try different AUR helpers
    let aur_helpers = ["yay", "paru", "trizen"];
    let mut installed = false;

    for helper in &aur_helpers {
        let helper_check = Command::new("which").arg(helper).status();
        if let Ok(s) = helper_check
            && s.success()
        {
            println!("🔧 Using {} to install ProtonUp-Qt...", helper);
            let install_status = Command::new(helper)
                .args(&["-S", "--noconfirm", "protonup-qt"])
                .status();

            match install_status {
                Ok(s) if s.success() => {
                    println!("✅ ProtonUp-Qt installed successfully!");
                    installed = true;
                    break;
                }
                _ => println!("❌ Failed to install with {}", helper),
            }
        }
    }

    if !installed {
        println!("❌ No AUR helper found. Please install an AUR helper first:");
        println!("  sudo pacman -S --needed base-devel git");
        println!("  git clone https://aur.archlinux.org/yay.git && cd yay && makepkg -si");
    }
}

pub fn install_protonup_qt() {
    println!("📦 ProtonUp-Qt Installation");
    println!("===========================");

    install_protonup_qt_aur();
}

fn remove_proton_version() {
    println!("🗑️  Remove Proton Version");
    println!("========================");

    let steam_dir = std::env::home_dir()
        .map(|h| h.join(".steam/steam/compatibilitytools.d"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.steam/steam/compatibilitytools.d"));

    if !steam_dir.exists() {
        println!("❌ No Proton installations found");
        return;
    }

    println!("📁 Available Proton versions:");
    let _ = Command::new("ls")
        .args(&["-1", &steam_dir.to_string_lossy()])
        .status();

    println!("\n💡 To remove a version, delete its folder from:");
    println!("   {}", steam_dir.display());
}

fn set_default_proton() {
    println!("🎯 Set Default Proton Version");
    println!("=============================");

    println!("💡 Default Proton is set in Steam client:");
    println!("  1. Open Steam");
    println!("  2. Go to Steam > Settings > Steam Play");
    println!("  3. Enable Steam Play for supported titles");
    println!("  4. Select your preferred Proton version");
    println!("  5. Apply and restart Steam");
}

fn update_proton_versions() {
    println!("🔄 Update All Proton Versions");
    println!("=============================");

    let protonup_check = Command::new("which").arg("protonup-qt").status();
    match protonup_check {
        Ok(s) if s.success() => {
            println!("🚀 Launching ProtonUp-Qt for updates...");
            let _ = Command::new("protonup-qt").spawn();
        }
        _ => {
            println!("❌ ProtonUp-Qt not found");
            println!("💡 Install ProtonUp-Qt first for easy Proton management");
        }
    }
}

fn open_proton_directory() {
    let steam_dir = std::env::home_dir()
        .map(|h| h.join(".steam/steam/compatibilitytools.d"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.steam/steam/compatibilitytools.d"));

    println!("📁 Opening Proton directory: {}", steam_dir.display());

    let _ = Command::new("xdg-open").arg(&steam_dir).spawn();
}

pub fn steam_library_optimization() {
    println!("🎮 Steam Library Optimization");
    println!("=============================");

    let options = [
        "🚀 Enable Steam Play for all titles",
        "⚡ Optimize Steam launch options",
        "📁 Move Steam library to different drive",
        "🧹 Clear Steam download cache",
        "🔧 Repair Steam library",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Steam Library Optimization")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => enable_steam_play_all(),
        1 => optimize_launch_options(),
        2 => move_steam_library(),
        3 => clear_download_cache(),
        4 => repair_steam_library(),
        _ => return,
    }
}

fn enable_steam_play_all() {
    println!("🚀 Enable Steam Play for All Titles");
    println!("====================================");

    println!("💡 To enable Steam Play for all Windows games:");
    println!("  1. Open Steam");
    println!("  2. Steam > Settings > Steam Play");
    println!("  3. ✅ Enable Steam Play for supported titles");
    println!("  4. ✅ Enable Steam Play for all other titles");
    println!("  5. Select latest Proton version");
    println!("  6. Click OK and restart Steam");

    println!("\n🎮 This allows you to play Windows games through Proton!");
}

fn optimize_launch_options() {
    println!("⚡ Common Steam Launch Options");
    println!("=============================");

    println!("💡 Right-click game > Properties > Launch Options");
    println!("\n🎮 Gaming optimizations:");
    println!("  gamemoderun %command%           - Enable GameMode");
    println!("  mangohud %command%              - Performance overlay");
    println!("  DXVK_HUD=fps %command%          - DXVK FPS counter");
    println!("  PROTON_NO_ESYNC=1 %command%     - Disable esync (if issues)");
    println!("  PROTON_NO_FSYNC=1 %command%     - Disable fsync (if issues)");
    println!("  PROTON_USE_WINED3D=1 %command%  - Use WineD3D instead of DXVK");

    println!("\n⚡ Performance options:");
    println!("  -high                           - High CPU priority");
    println!("  -threads 4                      - Limit CPU threads");
    println!("  -refresh 144                    - Set refresh rate");

    println!("\n🔧 Combined example:");
    println!("  gamemoderun mangohud %command%");
}

fn move_steam_library() {
    println!("📁 Move Steam Library");
    println!("=====================");

    println!("💡 To move your Steam library:");
    println!("  1. Open Steam");
    println!("  2. Steam > Settings > Storage");
    println!("  3. Click dropdown arrow next to drive");
    println!("  4. Add Drive > Select new location");
    println!("  5. Move games via right-click > Properties > Local Files > Move Install Folder");

    println!("\n🔧 Or use symlinks for existing library:");
    println!("  sudo mv ~/.steam/steam/steamapps /new/location/steamapps");
    println!("  ln -s /new/location/steamapps ~/.steam/steam/steamapps");
}

fn clear_download_cache() {
    println!("🧹 Clear Steam Download Cache");
    println!("=============================");

    let Ok(confirm) = Confirm::new()
        .with_prompt("Clear Steam download cache? (Steam must be closed)")
        .default(true)
        .interact()
    else {
        return;
    };

    if confirm {
        let cache_dir = std::env::home_dir()
            .map(|h| h.join(".steam/steam/appcache"))
            .unwrap_or_else(|| std::path::PathBuf::from("~/.steam/steam/appcache"));

        if cache_dir.exists() {
            let status = Command::new("rm")
                .args(&["-rf", &cache_dir.to_string_lossy()])
                .status();

            match status {
                Ok(s) if s.success() => println!("✅ Steam cache cleared"),
                _ => println!("❌ Failed to clear cache"),
            }
        } else {
            println!("❌ Steam cache directory not found");
        }
    }
}

fn repair_steam_library() {
    println!("🔧 Repair Steam Library");
    println!("=======================");

    println!("💡 Steam library repair options:");
    println!("  1. Verify game file integrity (in Steam)");
    println!("  2. Clear download cache (above option)");
    println!("  3. Regenerate Steam shortcuts");
    println!("  4. Reset Steam configuration");

    let Ok(confirm) = Confirm::new()
        .with_prompt("Reset Steam configuration? (Will log you out)")
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        let config_file = std::env::home_dir()
            .map(|h| h.join(".steam/steam/config/config.vdf"))
            .unwrap_or_else(|| std::path::PathBuf::from("~/.steam/steam/config/config.vdf"));

        if config_file.exists() {
            let backup_name = format!("{}.backup", config_file.to_string_lossy());
            let Some(config_path) = config_file.to_str() else {
                println!("❌ Invalid config file path");
                return;
            };
            let backup_status = Command::new("cp")
                .args(&[config_path, &backup_name])
                .status();

            let _ = Command::new("rm").arg(&config_file).status();

            match backup_status {
                Ok(s) if s.success() => println!("✅ Steam config reset (backup created)"),
                _ => println!("⚠️  Config reset, no backup created"),
            }
        }
    }
}

pub fn steam_prefix_management() {
    println!("🔄 Steam Prefix Management");
    println!("==========================");

    println!("💡 Steam/Proton prefixes are located at:");
    println!("  ~/.steam/steam/steamapps/compatdata/[GAME_ID]/pfx/");

    let options = [
        "📁 List all game prefixes",
        "🧹 Clean specific game prefix",
        "🔧 Reset game prefix",
        "📋 Show prefix info",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Prefix Management")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => list_game_prefixes(),
        1 => clean_game_prefix(),
        2 => reset_game_prefix(),
        3 => show_prefix_info(),
        _ => return,
    }
}

fn list_game_prefixes() {
    let compatdata_dir = std::env::home_dir()
        .map(|h| h.join(".steam/steam/steamapps/compatdata"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.steam/steam/steamapps/compatdata"));

    if compatdata_dir.exists() {
        println!("📁 Game prefixes:");
        let _ = Command::new("ls")
            .args(&["-la", &compatdata_dir.to_string_lossy()])
            .status();
    } else {
        println!("❌ No Steam prefixes found");
    }
}

fn clean_game_prefix() {
    println!("🧹 Clean Game Prefix");
    println!("====================");
    println!("💡 This removes logs and temporary files from a game prefix");
    println!("⚠️  Enter game ID (folder name from compatdata)");
}

fn reset_game_prefix() {
    println!("🔧 Reset Game Prefix");
    println!("====================");
    println!("⚠️  This will completely reset the game's Wine prefix");
    println!("💡 Game will need to reinstall prerequisites on next launch");
    println!("📋 Find game ID in ~/.steam/steam/steamapps/compatdata/");
}

fn show_prefix_info() {
    println!("📋 Prefix Information");
    println!("=====================");
    println!("💡 Each game gets its own Wine prefix for isolation");
    println!("📁 Location: ~/.steam/steam/steamapps/compatdata/[GAME_ID]/pfx/");
    println!("🔧 Contains: Windows registry, DLLs, game data");
    println!("⚠️  Deleting a prefix forces game to recreate it");
}

pub fn steam_troubleshooting() {
    println!("🛠️  Steam Troubleshooting");
    println!("=========================");

    let options = [
        "🔧 Fix Steam runtime issues",
        "📦 Reinstall Steam Linux runtime",
        "🎮 Fix controller support",
        "🔊 Fix audio issues",
        "📱 Reset Steam to native runtime",
        "🖥️  Fix display/scaling issues",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Steam Troubleshooting")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => fix_steam_runtime(),
        1 => reinstall_steam_runtime(),
        2 => fix_controller_support(),
        3 => fix_audio_issues(),
        4 => reset_steam_native(),
        5 => fix_display_issues(),
        _ => return,
    }
}

fn fix_steam_runtime() {
    println!("🔧 Fix Steam Runtime Issues");
    println!("===========================");

    println!("🛠️  Common Steam runtime fixes:");

    let fixes = [
        (
            "Clear Steam runtime cache",
            "rm -rf ~/.steam/steam/ubuntu12_32/steam-runtime/",
        ),
        (
            "Reset Steam client",
            "rm ~/.steam/steam/clientregistry.blob",
        ),
        ("Force Steam runtime update", "steam --reset"),
    ];

    for (desc, cmd) in &fixes {
        let Ok(confirm) = Confirm::new()
            .with_prompt(&format!("Apply: {}?", desc))
            .default(false)
            .interact()
        else {
            continue;
        };

        if confirm {
            let status = Command::new("sh").arg("-c").arg(cmd).status();
            match status {
                Ok(s) if s.success() => println!("  ✅ {}", desc),
                _ => println!("  ❌ Failed: {}", desc),
            }
        }
    }
}

fn reinstall_steam_runtime() {
    println!("📦 Reinstall Steam Linux Runtime");
    println!("================================");

    let Ok(confirm) = Confirm::new()
        .with_prompt("Reinstall Steam runtime? (Steam will be closed)")
        .default(true)
        .interact()
    else {
        return;
    };

    if confirm {
        // Kill Steam
        let _ = Command::new("pkill").arg("steam").status();

        // Remove Steam runtime
        let runtime_dir = std::env::home_dir()
            .map(|h| h.join(".steam/steam/ubuntu12_32"))
            .unwrap_or_else(|| std::path::PathBuf::from("~/.steam/steam/ubuntu12_32"));

        if runtime_dir.exists() {
            let _ = Command::new("rm")
                .args(&["-rf", &runtime_dir.to_string_lossy()])
                .status();
        }

        println!("✅ Steam runtime removed. Launch Steam to reinstall it.");
    }
}

fn fix_controller_support() {
    println!("🎮 Fix Controller Support");
    println!("=========================");

    println!("🔧 Installing controller support packages...");
    let packages = ["lib32-libusb", "steam-native-runtime"];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&packages)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Controller packages installed");
            println!("💡 Enable Steam Input in Steam > Settings > Controller");
        }
        _ => println!("❌ Failed to install controller packages"),
    }
}

fn fix_audio_issues() {
    println!("🔊 Fix Audio Issues");
    println!("==================");

    println!("🔧 Installing audio libraries...");
    let packages = ["lib32-alsa-plugins", "lib32-libpulse", "lib32-openal"];

    let status = Command::new("sudo")
        .args(&["pacman", "-S", "--needed", "--noconfirm"])
        .args(&packages)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Audio libraries installed"),
        _ => println!("❌ Failed to install audio libraries"),
    }
}

fn reset_steam_native() {
    println!("📱 Reset Steam to Native Runtime");
    println!("================================");

    println!("💡 Force Steam to use native system libraries:");
    println!("  export STEAM_RUNTIME=0");
    println!("  steam");

    let Ok(confirm) = Confirm::new()
        .with_prompt("Launch Steam with native runtime now?")
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        let _ = Command::new("env")
            .args(&["STEAM_RUNTIME=0", "steam"])
            .spawn();
    }
}

fn fix_display_issues() {
    println!("🖥️  Fix Display/Scaling Issues");
    println!("==============================");

    println!("💡 Common display fixes:");
    println!("  GDK_SCALE=1 steam              - Reset Steam UI scaling");
    println!("  steam -cef-disable-gpu         - Disable GPU acceleration");
    println!("  steam -no-dwrite               - Disable DirectWrite");

    let Ok(confirm) = Confirm::new()
        .with_prompt("Launch Steam with scaling fix?")
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        let _ = Command::new("env").args(&["GDK_SCALE=1", "steam"]).spawn();
    }
}

pub fn steam_status() {
    println!("📋 Steam Status & Information");
    println!("=============================");

    // Check if Steam is installed
    let steam_check = Command::new("which").arg("steam").status();
    match steam_check {
        Ok(s) if s.success() => println!("✅ Steam is installed"),
        _ => {
            println!("❌ Steam is not installed");
            return;
        }
    }

    // Check if Steam is running
    let running_check = Command::new("pgrep").arg("steam").status();
    match running_check {
        Ok(s) if s.success() => println!("🟢 Steam is currently running"),
        _ => println!("⭕ Steam is not running"),
    }

    // Check Steam directories
    let steam_dir = std::env::home_dir()
        .map(|h| h.join(".steam"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.steam"));

    if steam_dir.exists() {
        println!("📁 Steam directory: {}", steam_dir.display());

        // Check library size
        let steamapps_dir = steam_dir.join("steam/steamapps");
        if steamapps_dir.exists() {
            println!("📦 Steam library found");
            let _ = Command::new("du")
                .args(&["-sh", &steamapps_dir.to_string_lossy()])
                .status();
        }
    }

    // Check multilib
    let multilib_check = Command::new("grep")
        .args(&["-E", "^\\[multilib\\]", "/etc/pacman.conf"])
        .output();
    match multilib_check {
        Ok(out) if !out.stdout.is_empty() => println!("✅ Multilib repository enabled"),
        _ => println!("❌ Multilib repository disabled"),
    }

    // Check Steam Play settings
    let config_file = std::env::home_dir()
        .map(|h| h.join(".steam/steam/config/config.vdf"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.steam/steam/config/config.vdf"));

    if config_file.exists() {
        let proton_check = Command::new("grep")
            .args(&["-i", "proton", &config_file.to_string_lossy()])
            .status();
        match proton_check {
            Ok(s) if s.success() => println!("✅ Steam Play/Proton configured"),
            _ => println!("⚠️  Steam Play may not be configured"),
        }
    }
}
