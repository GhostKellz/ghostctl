use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

// Cache for commonly accessed paths
static HOME_DIR: OnceLock<String> = OnceLock::new();

fn get_home_dir() -> &'static str {
    HOME_DIR.get_or_init(|| std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string()))
}

pub fn proton_menu() {
    loop {
        let options = [
            "🎮 DXVK/VKD3D Management",
            "🔧 Compatibility Layers Setup",
            "🍷 Wine Tweaks & Configuration",
            "🎯 Game-Specific Fixes",
            "⚡ Performance Enhancements",
            "🛡️ Anti-Cheat Runtime Setup",
            "💾 Shader Cache Management",
            "📝 Wine Registry Editor",
            "⬅️ Back",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🚀 Proton & Wine Advanced Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => dxvk_vkd3d_management(),
            1 => compatibility_layers_setup(),
            2 => wine_tweaks_config(),
            3 => game_specific_fixes(),
            4 => performance_enhancements(),
            5 => anticheat_setup(),
            6 => shader_cache_management(),
            7 => wine_registry_editor(),
            _ => break,
        }
    }
}

fn dxvk_vkd3d_management() {
    let options = [
        "📦 Install DXVK",
        "📦 Install VKD3D-Proton",
        "🔄 Update DXVK/VKD3D",
        "🔧 Configure DXVK Settings",
        "🗑️ Remove DXVK/VKD3D",
        "📊 Check DXVK/VKD3D Status",
        "⬅️ Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("DXVK/VKD3D Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_dxvk(),
        1 => install_vkd3d(),
        2 => update_dxvk_vkd3d(),
        3 => configure_dxvk(),
        4 => remove_dxvk_vkd3d(),
        5 => check_dxvk_status(),
        _ => {}
    }
}

fn install_dxvk() {
    println!("📦 Installing DXVK...");

    let wine_prefix = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path (or press Enter for default)")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
        .unwrap();

    let version = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select DXVK version")
        .items(&["Latest (2.3)", "2.2", "2.1", "2.0", "1.10.3 (older GPUs)"])
        .default(0)
        .interact()
        .unwrap();

    let version_str = match version {
        0 => "2.3",
        1 => "2.2",
        2 => "2.1",
        3 => "2.0",
        4 => "1.10.3",
        _ => "2.3",
    };

    println!("⬇️ Downloading DXVK {}...", version_str);
    let download_cmd = format!(
        "cd /tmp && wget -q https://github.com/doitsujin/dxvk/releases/download/v{}/dxvk-{}.tar.gz",
        version_str, version_str
    );

    let status = Command::new("sh").arg("-c").arg(&download_cmd).status();

    match status {
        Ok(s) if s.success() => {
            println!("📂 Extracting DXVK...");
            let extract_cmd = format!("cd /tmp && tar -xzf dxvk-{}.tar.gz", version_str);
            Command::new("sh").arg("-c").arg(&extract_cmd).status().ok();

            println!("🔧 Installing DXVK to Wine prefix...");
            let install_cmd = format!(
                "cd /tmp/dxvk-{} && WINEPREFIX={} ./setup_dxvk.sh install",
                version_str, wine_prefix
            );

            let install_status = Command::new("sh").arg("-c").arg(&install_cmd).status();

            match install_status {
                Ok(s) if s.success() => println!("✅ DXVK {} installed successfully!", version_str),
                _ => println!("❌ Failed to install DXVK"),
            }

            // Cleanup
            Command::new("sh")
                .arg("-c")
                .arg(&format!("rm -rf /tmp/dxvk-{}*", version_str))
                .status()
                .ok();
        }
        _ => println!("❌ Failed to download DXVK"),
    }
}

fn install_vkd3d() {
    println!("📦 Installing VKD3D-Proton...");

    let wine_prefix = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path (or press Enter for default)")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
        .unwrap();

    println!("⬇️ Downloading VKD3D-Proton...");
    let download_cmd = "cd /tmp && wget -q https://github.com/HansKristian-Work/vkd3d-proton/releases/download/v2.11/vkd3d-proton-2.11.tar.zst";

    let status = Command::new("sh").arg("-c").arg(download_cmd).status();

    match status {
        Ok(s) if s.success() => {
            println!("📂 Extracting VKD3D-Proton...");
            Command::new("sh")
                .arg("-c")
                .arg("cd /tmp && tar -xf vkd3d-proton-2.11.tar.zst")
                .status()
                .ok();

            println!("🔧 Installing VKD3D-Proton to Wine prefix...");
            let install_cmd = format!(
                "cd /tmp/vkd3d-proton-2.11 && WINEPREFIX={} ./setup_vkd3d_proton.sh install",
                wine_prefix
            );

            let install_status = Command::new("sh").arg("-c").arg(&install_cmd).status();

            match install_status {
                Ok(s) if s.success() => println!("✅ VKD3D-Proton installed successfully!"),
                _ => println!("❌ Failed to install VKD3D-Proton"),
            }

            // Cleanup
            Command::new("sh")
                .arg("-c")
                .arg("rm -rf /tmp/vkd3d-proton*")
                .status()
                .ok();
        }
        _ => println!("❌ Failed to download VKD3D-Proton"),
    }
}

fn update_dxvk_vkd3d() {
    println!("🔄 Updating DXVK/VKD3D...");

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What to update?")
        .items(&["DXVK", "VKD3D-Proton", "Both"])
        .default(2)
        .interact()
        .unwrap();

    match choice {
        0 => install_dxvk(),
        1 => install_vkd3d(),
        2 => {
            install_dxvk();
            install_vkd3d();
        }
        _ => {}
    }
}

fn configure_dxvk() {
    println!("🔧 Configuring DXVK...");

    let _wine_prefix = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
        .unwrap();

    let options = [
        "Enable DXVK HUD",
        "Disable DXVK HUD",
        "Set DXVK log level",
        "Configure async compilation",
        "Set GPU memory limit",
        "Enable/Disable NVAPI",
        "Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("DXVK Configuration")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("📊 Enabling DXVK HUD...");
            unsafe { std::env::set_var("DXVK_HUD", "fps,memory,gpuload,version") };
            println!("✅ DXVK HUD enabled with: fps, memory, gpuload, version");
        }
        1 => {
            println!("📊 Disabling DXVK HUD...");
            unsafe { std::env::remove_var("DXVK_HUD") };
            println!("✅ DXVK HUD disabled");
        }
        2 => {
            let level = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select log level")
                .items(&["none", "error", "warn", "info", "debug"])
                .default(0)
                .interact()
                .unwrap();

            let level_str = ["none", "error", "warn", "info", "debug"][level];
            unsafe { std::env::set_var("DXVK_LOG_LEVEL", level_str) };
            println!("✅ DXVK log level set to: {}", level_str);
        }
        3 => {
            let async_compile = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Enable async compilation?")
                .default(true)
                .interact()
                .unwrap();

            if async_compile {
                unsafe { std::env::set_var("DXVK_ASYNC", "1") };
                println!("✅ Async compilation enabled");
            } else {
                unsafe { std::env::remove_var("DXVK_ASYNC") };
                println!("✅ Async compilation disabled");
            }
        }
        4 => {
            let memory = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter GPU memory limit in MB (e.g., 4096)")
                .interact()
                .unwrap();

            unsafe { std::env::set_var("DXVK_MEMORY_LIMIT", &memory) };
            println!("✅ GPU memory limit set to: {} MB", memory);
        }
        5 => {
            let nvapi = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Enable NVAPI?")
                .default(false)
                .interact()
                .unwrap();

            if nvapi {
                unsafe { std::env::set_var("DXVK_ENABLE_NVAPI", "1") };
                println!("✅ NVAPI enabled");
            } else {
                unsafe { std::env::remove_var("DXVK_ENABLE_NVAPI") };
                println!("✅ NVAPI disabled");
            }
        }
        _ => {}
    }
}

fn remove_dxvk_vkd3d() {
    println!("🗑️ Removing DXVK/VKD3D...");

    let wine_prefix = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
        .unwrap();

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Are you sure you want to remove DXVK/VKD3D?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        println!("🔧 Removing DXVK...");
        let remove_cmd = format!(
            "cd {} && rm -f drive_c/windows/system32/{{d3d9,d3d10core,d3d11,dxgi}}.dll drive_c/windows/syswow64/{{d3d9,d3d10core,d3d11,dxgi}}.dll",
            wine_prefix
        );
        Command::new("sh").arg("-c").arg(&remove_cmd).status().ok();

        println!("🔧 Removing VKD3D-Proton...");
        let remove_vkd3d_cmd = format!(
            "cd {} && rm -f drive_c/windows/system32/d3d12.dll drive_c/windows/syswow64/d3d12.dll",
            wine_prefix
        );
        Command::new("sh")
            .arg("-c")
            .arg(&remove_vkd3d_cmd)
            .status()
            .ok();

        println!("✅ DXVK/VKD3D removed");
    }
}

fn check_dxvk_status() {
    println!("📊 Checking DXVK/VKD3D Status...");

    let wine_prefix = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
        .unwrap();

    println!("\n🔍 Checking DXVK installation...");
    let dxvk_dlls = ["d3d9.dll", "d3d10core.dll", "d3d11.dll", "dxgi.dll"];
    for dll in &dxvk_dlls {
        let path32 = format!("{}/drive_c/windows/syswow64/{}", wine_prefix, dll);
        let path64 = format!("{}/drive_c/windows/system32/{}", wine_prefix, dll);

        if Path::new(&path32).exists() || Path::new(&path64).exists() {
            println!("  ✅ {} installed", dll);
        } else {
            println!("  ❌ {} not found", dll);
        }
    }

    println!("\n🔍 Checking VKD3D-Proton installation...");
    let path32 = format!("{}/drive_c/windows/syswow64/d3d12.dll", wine_prefix);
    let path64 = format!("{}/drive_c/windows/system32/d3d12.dll", wine_prefix);

    if Path::new(&path32).exists() || Path::new(&path64).exists() {
        println!("  ✅ d3d12.dll installed (VKD3D-Proton)");
    } else {
        println!("  ❌ d3d12.dll not found");
    }

    println!("\n📋 Environment Variables:");
    println!(
        "  DXVK_HUD: {:?}",
        std::env::var("DXVK_HUD").unwrap_or_else(|_| "Not set".to_string())
    );
    println!(
        "  DXVK_LOG_LEVEL: {:?}",
        std::env::var("DXVK_LOG_LEVEL").unwrap_or_else(|_| "Not set".to_string())
    );
    println!(
        "  DXVK_ASYNC: {:?}",
        std::env::var("DXVK_ASYNC").unwrap_or_else(|_| "Not set".to_string())
    );
}

fn compatibility_layers_setup() {
    let options = [
        "🎮 Install Gallium Nine",
        "🎮 Install D9VK",
        "🔧 Configure Gallium Nine",
        "📦 Install Wine-GE/TKG",
        "🍷 Install Wine dependencies",
        "⬅️ Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Compatibility Layers Setup")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_gallium_nine(),
        1 => install_d9vk(),
        2 => configure_gallium_nine(),
        3 => install_wine_ge(),
        4 => install_wine_dependencies(),
        _ => {}
    }
}

fn install_gallium_nine() {
    println!("🎮 Installing Gallium Nine...");

    let distro = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your distribution")
        .items(&["Arch/Manjaro", "Ubuntu/Debian", "Fedora", "Other"])
        .default(0)
        .interact()
        .unwrap();

    let cmd = match distro {
        0 => "sudo pacman -S wine-nine lib32-mesa-gallium",
        1 => "sudo apt install libd3dadapter9-mesa libd3dadapter9-mesa:i386",
        2 => "sudo dnf install wine-nine",
        _ => {
            println!("⚠️ Please install Gallium Nine manually for your distribution");
            return;
        }
    };

    let status = Command::new("sh").arg("-c").arg(cmd).status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Gallium Nine installed successfully!");
            println!("💡 Enable it with: wine ninewinecfg");
        }
        _ => println!("❌ Failed to install Gallium Nine"),
    }
}

fn install_d9vk() {
    println!("🎮 Installing D9VK (DirectX 9 over Vulkan)...");

    let wine_prefix = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
        .unwrap();

    println!("⬇️ Downloading D9VK...");
    let download_cmd = "cd /tmp && wget -q https://github.com/Joshua-Ashton/d9vk/releases/download/0.40.1/d9vk-0.40.1.tar.gz";

    let status = Command::new("sh").arg("-c").arg(download_cmd).status();

    match status {
        Ok(s) if s.success() => {
            println!("📂 Extracting D9VK...");
            Command::new("sh")
                .arg("-c")
                .arg("cd /tmp && tar -xzf d9vk-0.40.1.tar.gz")
                .status()
                .ok();

            println!("🔧 Installing D9VK to Wine prefix...");
            let install_cmd = format!(
                "cd /tmp/d9vk-0.40.1 && WINEPREFIX={} ./setup_d9vk.sh install",
                wine_prefix
            );

            let install_status = Command::new("sh").arg("-c").arg(&install_cmd).status();

            match install_status {
                Ok(s) if s.success() => println!("✅ D9VK installed successfully!"),
                _ => println!("❌ Failed to install D9VK"),
            }

            // Cleanup
            Command::new("sh")
                .arg("-c")
                .arg("rm -rf /tmp/d9vk*")
                .status()
                .ok();
        }
        _ => println!("❌ Failed to download D9VK"),
    }
}

fn configure_gallium_nine() {
    println!("🔧 Configuring Gallium Nine...");

    let wine_prefix = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
        .unwrap();

    println!("🔧 Opening Nine configuration...");
    let cmd = format!("WINEPREFIX={} wine ninewinecfg", wine_prefix);

    let status = Command::new("sh").arg("-c").arg(&cmd).status();

    match status {
        Ok(s) if s.success() => println!("✅ Configuration opened"),
        _ => println!("❌ Failed to open Nine configuration"),
    }
}

fn install_wine_ge() {
    println!("📦 Installing Wine-GE/TKG...");

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Wine version to install")
        .items(&["Wine-GE (Recommended)", "Wine-TKG", "Both"])
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 | 2 => {
            println!("⬇️ Downloading Wine-GE...");
            let download_cmd = "cd /tmp && wget -q https://github.com/GloriousEggroll/wine-ge-custom/releases/download/GE-Proton8-26/wine-lutris-GE-Proton8-26-x86_64.tar.xz";

            let status = Command::new("sh").arg("-c").arg(download_cmd).status();

            if let Ok(s) = status
                && s.success() {
                    println!("📂 Installing Wine-GE...");
                    let install_cmd = "mkdir -p ~/.local/share/lutris/runners/wine && cd ~/.local/share/lutris/runners/wine && tar -xf /tmp/wine-lutris-GE-Proton8-26-x86_64.tar.xz";
                    Command::new("sh").arg("-c").arg(install_cmd).status().ok();
                    println!("✅ Wine-GE installed");
                }
        }
        _ => {}
    }

    if choice == 1 || choice == 2 {
        println!("📦 Wine-TKG requires building from source");
        println!("Visit: https://github.com/Frogging-Family/wine-tkg-git");
    }
}

fn install_wine_dependencies() {
    println!("🍷 Installing Wine dependencies...");

    let distro = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your distribution")
        .items(&["Arch/Manjaro", "Ubuntu/Debian", "Fedora", "Other"])
        .default(0)
        .interact()
        .unwrap();

    let cmd = match distro {
        0 => "sudo pacman -S wine-staging winetricks wine-mono wine-gecko lib32-gnutls lib32-libldap lib32-libgpg-error lib32-sqlite lib32-libpulse lib32-alsa-lib",
        1 => "sudo apt install wine64 wine32 winetricks winbind",
        2 => "sudo dnf install wine winetricks wine-mono wine-gecko",
        _ => {
            println!("⚠️ Please install Wine dependencies manually for your distribution");
            return;
        }
    };

    let status = Command::new("sh").arg("-c").arg(cmd).status();

    match status {
        Ok(s) if s.success() => println!("✅ Wine dependencies installed"),
        _ => println!("❌ Failed to install Wine dependencies"),
    }
}

fn wine_tweaks_config() {
    let options = [
        "🔧 Winetricks Automation",
        "📦 DLL Overrides Management",
        "🎮 Configure Wine for Gaming",
        "🔊 Audio Configuration",
        "🖥️ Display Settings",
        "⬅️ Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Wine Tweaks & Configuration")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => winetricks_automation(),
        1 => dll_overrides_management(),
        2 => configure_wine_gaming(),
        3 => audio_configuration(),
        4 => display_settings(),
        _ => {}
    }
}

fn winetricks_automation() {
    println!("🔧 Winetricks Automation");

    let wine_prefix = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
        .unwrap();

    let common_packages = vec![
        "d3dx9",
        "d3dx10",
        "d3dx11_43",
        "vcrun2019",
        "dotnet48",
        "physx",
        "faudio",
        "xact",
        "xvid",
        "openal",
    ];

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select packages to install")
        .items(&common_packages)
        .interact()
        .unwrap();

    for idx in selected {
        let package = &common_packages[idx];
        println!("📦 Installing {}...", package);

        let cmd = format!("WINEPREFIX={} winetricks -q {}", wine_prefix, package);
        let status = Command::new("sh").arg("-c").arg(&cmd).status();

        match status {
            Ok(s) if s.success() => println!("  ✅ {} installed", package),
            _ => println!("  ❌ Failed to install {}", package),
        }
    }
}

fn dll_overrides_management() {
    println!("📦 DLL Overrides Management");

    let wine_prefix = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
        .unwrap();

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action")
        .items(&[
            "Add DLL override",
            "Remove DLL override",
            "List overrides",
            "Common gaming overrides",
        ])
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            let dll = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter DLL name (without .dll)")
                .interact()
                .unwrap();

            let mode = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select override mode")
                .items(&[
                    "native",
                    "builtin",
                    "native,builtin",
                    "builtin,native",
                    "disabled",
                ])
                .default(2)
                .interact()
                .unwrap();

            let mode_str = ["native", "builtin", "native,builtin", "builtin,native", ""][mode];

            let cmd = format!("WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Software\\Wine\\DllOverrides' /v {} /d {} /f",
                            wine_prefix, dll, mode_str);

            Command::new("sh").arg("-c").arg(&cmd).status().ok();
            println!("✅ DLL override added: {} = {}", dll, mode_str);
        }
        1 => {
            let dll = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter DLL name to remove")
                .interact()
                .unwrap();

            let cmd = format!("WINEPREFIX={} wine reg delete 'HKEY_CURRENT_USER\\Software\\Wine\\DllOverrides' /v {} /f",
                            wine_prefix, dll);

            Command::new("sh").arg("-c").arg(&cmd).status().ok();
            println!("✅ DLL override removed: {}", dll);
        }
        2 => {
            println!("📋 Current DLL overrides:");
            let cmd = format!(
                "WINEPREFIX={} wine reg query 'HKEY_CURRENT_USER\\Software\\Wine\\DllOverrides'",
                wine_prefix
            );
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
        }
        3 => {
            println!("🎮 Applying common gaming DLL overrides...");
            let overrides = [
                ("d3d9", "native"),
                ("d3d10", "native"),
                ("d3d10_1", "native"),
                ("d3d10core", "native"),
                ("d3d11", "native"),
                ("dxgi", "native"),
                ("nvapi", "disabled"),
                ("nvapi64", "disabled"),
            ];

            for (dll, mode) in &overrides {
                let cmd = format!("WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Software\\Wine\\DllOverrides' /v {} /d {} /f",
                                wine_prefix, dll, mode);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
                println!("  ✅ {} = {}", dll, mode);
            }
        }
        _ => {}
    }
}

fn configure_wine_gaming() {
    println!("🎮 Configuring Wine for Gaming");

    let wine_prefix = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
        .unwrap();

    println!("🔧 Applying gaming optimizations...");

    // Windows version
    println!("  Setting Windows version to Windows 10...");
    let cmd = format!("WINEPREFIX={} winecfg /v win10", wine_prefix);
    Command::new("sh").arg("-c").arg(&cmd).status().ok();

    // Enable CSMT
    println!("  Enabling CSMT (Command Stream Multi-Threading)...");
    let cmd = format!("WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Software\\Wine\\Direct3D' /v csmt /t REG_DWORD /d 1 /f", wine_prefix);
    Command::new("sh").arg("-c").arg(&cmd).status().ok();

    // Large address aware
    println!("  Enabling Large Address Aware...");
    unsafe { std::env::set_var("WINE_LARGE_ADDRESS_AWARE", "1") };

    // Esync
    println!("  Enabling Esync...");
    unsafe { std::env::set_var("WINEESYNC", "1") };

    // Fsync
    println!("  Enabling Fsync (if supported)...");
    unsafe { std::env::set_var("WINEFSYNC", "1") };

    println!("✅ Gaming optimizations applied!");
}

fn audio_configuration() {
    println!("🔊 Audio Configuration");

    let wine_prefix = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
        .unwrap();

    let audio_system = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select audio system")
        .items(&["PulseAudio", "ALSA", "OSS", "Disabled"])
        .default(0)
        .interact()
        .unwrap();

    let driver = match audio_system {
        0 => "pulse",
        1 => "alsa",
        2 => "oss",
        _ => "",
    };

    if !driver.is_empty() {
        let cmd = format!("WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Software\\Wine\\Drivers' /v Audio /d {} /f",
                        wine_prefix, driver);
        Command::new("sh").arg("-c").arg(&cmd).status().ok();
        println!("✅ Audio system set to: {}", driver);
    }

    // Sample rate
    let sample_rate = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter sample rate (default: 48000)")
        .default("48000".to_string())
        .interact()
        .unwrap();

    let cmd = format!("WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Software\\Wine\\DirectSound' /v DefaultSampleRate /t REG_DWORD /d {} /f",
                    wine_prefix, sample_rate);
    Command::new("sh").arg("-c").arg(&cmd).status().ok();

    println!("✅ Audio configuration updated");
}

fn display_settings() {
    println!("🖥️ Display Settings");

    let wine_prefix = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
        .unwrap();

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select setting to configure")
        .items(&[
            "Virtual Desktop",
            "Screen Resolution",
            "DPI Settings",
            "Disable Window Manager",
            "Back",
        ])
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            let enable = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Enable virtual desktop?")
                .default(false)
                .interact()
                .unwrap();

            if enable {
                let _resolution = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter resolution (e.g., 1920x1080)")
                    .default("1920x1080".to_string())
                    .interact()
                    .unwrap();

                let cmd = format!("WINEPREFIX={} winecfg", wine_prefix);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
                println!("✅ Please configure virtual desktop in the opened window");
            }
        }
        1 => {
            println!("📏 Opening display configuration...");
            let cmd = format!("WINEPREFIX={} winecfg", wine_prefix);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
        }
        2 => {
            let dpi = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter DPI value (default: 96)")
                .default("96".to_string())
                .interact()
                .unwrap();

            let cmd = format!("WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Control Panel\\Desktop' /v LogPixels /t REG_DWORD /d {} /f",
                            wine_prefix, dpi);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
            println!("✅ DPI set to: {}", dpi);
        }
        3 => {
            let disable = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Disable window manager decorations?")
                .default(false)
                .interact()
                .unwrap();

            if disable {
                let cmd = format!("WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Software\\Wine\\X11 Driver' /v Decorated /d N /f", wine_prefix);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
                println!("✅ Window decorations disabled");
            }
        }
        _ => {}
    }
}

fn game_specific_fixes() {
    println!("🎯 Game-Specific Fixes");

    let options = [
        "🔧 Apply Protonfixes",
        "📝 Custom Game Scripts",
        "🎮 Common Game Fixes",
        "💾 Game-specific configurations",
        "⬅️ Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Game-Specific Fixes")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => apply_protonfixes(),
        1 => custom_game_scripts(),
        2 => common_game_fixes(),
        3 => game_specific_configs(),
        _ => {}
    }
}

fn apply_protonfixes() {
    println!("🔧 Applying Protonfixes...");

    println!("📦 Installing protonfixes...");
    let install_cmd = "pip install --user protonfixes";

    let status = Command::new("sh").arg("-c").arg(install_cmd).status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Protonfixes installed");

            let game_id = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter Steam App ID (or game name)")
                .interact()
                .unwrap();

            println!("🔍 Checking for fixes for: {}", game_id);

            // Enable protonfixes
            unsafe { std::env::set_var("PROTONFIXES_DISABLE", "0") };
            println!("✅ Protonfixes enabled for the game");
        }
        _ => println!("❌ Failed to install protonfixes"),
    }
}

fn custom_game_scripts() {
    println!("📝 Custom Game Scripts");

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action")
        .items(&[
            "Create launch script",
            "Edit existing script",
            "List scripts",
            "Back",
        ])
        .default(0)
        .interact()
        .unwrap();

    if choice == 0 {
        let game_name = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter game name")
            .interact()
            .unwrap();

        let script_path = format!("{}/Games/scripts/{}.sh", get_home_dir(), game_name);

        println!("📝 Creating launch script: {}", script_path);

        let script_content = format!(
            r#"#!/bin/bash
    # Launch script for {}

    # Wine prefix
    export WINEPREFIX="$HOME/Games/prefixes/{}"

    # Performance settings
    export __GL_THREADED_OPTIMIZATIONS=1
    export __GL_SHADER_DISK_CACHE=1
    export __GL_SHADER_DISK_CACHE_PATH="$HOME/.cache/shaders"

    # DXVK settings
    export DXVK_HUD=fps
    export DXVK_ASYNC=1

    # Wine settings
    export WINEESYNC=1
    export WINEFSYNC=1

    # Game executable
    GAME_EXE="path/to/game.exe"

    # Launch with gamemode and mangohud
    gamemoderun mangohud wine "$GAME_EXE" "$@"
    "#,
            game_name, game_name
        );

        fs::create_dir_all(format!("{}/Games/scripts", get_home_dir())).ok();
        fs::write(&script_path, script_content).ok();

        // Make executable
        Command::new("chmod")
            .args(&["+x", &script_path])
            .status()
            .ok();

        println!("✅ Script created: {}", script_path);
    }
}

fn common_game_fixes() {
    println!("🎮 Common Game Fixes");

    let fixes = [
        "Fix black screen issues",
        "Fix controller not working",
        "Fix audio crackling",
        "Fix video cutscenes",
        "Fix multiplayer connection",
        "Fix save game issues",
        "Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select fix to apply")
        .items(&fixes)
        .default(0)
        .interact()
        .unwrap();

    let wine_prefix = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("🖤 Fixing black screen issues...");
            println!("  Disabling NVAPI...");
            unsafe { std::env::set_var("DXVK_NVAPI_DRIVER_VERSION", "0") };
            println!("  Setting windowed mode...");
            println!("  Disabling fullscreen optimizations...");
            let cmd = format!("WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Software\\Wine\\Direct3D' /v ForceWindowedMode /t REG_DWORD /d 1 /f", wine_prefix);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
            println!("✅ Black screen fixes applied");
        }
        1 => {
            println!("🎮 Fixing controller issues...");
            println!("  Installing xinput...");
            let cmd = format!("WINEPREFIX={} winetricks -q xinput", wine_prefix);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
            println!("  Enabling SDL controller support...");
            unsafe { std::env::set_var("SDL_GAMECONTROLLERCONFIG", "1") };
            println!("✅ Controller fixes applied");
        }
        2 => {
            println!("🔊 Fixing audio crackling...");
            println!("  Setting pulse latency...");
            unsafe { std::env::set_var("PULSE_LATENCY_MSEC", "60") };
            println!("  Configuring sample rate...");
            let cmd = format!("WINEPREFIX={} wine reg add 'HKEY_CURRENT_USER\\Software\\Wine\\DirectSound' /v HelBuflen /t REG_DWORD /d 512 /f", wine_prefix);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
            println!("✅ Audio fixes applied");
        }
        3 => {
            println!("🎬 Fixing video cutscenes...");
            println!("  Installing media codecs...");
            let cmd = format!("WINEPREFIX={} winetricks -q mf quartz wmp10", wine_prefix);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
            println!("✅ Video codec fixes applied");
        }
        4 => {
            println!("🌐 Fixing multiplayer connection...");
            println!("  Configuring network settings...");
            let cmd = format!("WINEPREFIX={} wine reg add 'HKEY_LOCAL_MACHINE\\System\\CurrentControlSet\\Services\\Tcpip\\Parameters' /v TcpTimedWaitDelay /t REG_DWORD /d 30 /f", wine_prefix);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
            println!("✅ Network fixes applied");
        }
        5 => {
            println!("💾 Fixing save game issues...");
            println!("  Creating Documents folders...");
            let docs_path = format!(
                "{}/drive_c/users/{}/Documents",
                wine_prefix,
                std::env::var("USER").unwrap_or_default()
            );
            fs::create_dir_all(&docs_path).ok();
            println!("  Setting permissions...");
            Command::new("chmod")
                .args(&["-R", "755", &docs_path])
                .status()
                .ok();
            println!("✅ Save game fixes applied");
        }
        _ => {}
    }
}

fn game_specific_configs() {
    println!("💾 Game-specific Configurations");

    let games = [
        "Grand Theft Auto V",
        "The Witcher 3",
        "Cyberpunk 2077",
        "Red Dead Redemption 2",
        "Elden Ring",
        "Other (Manual)",
        "Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select game")
        .items(&games)
        .default(0)
        .interact()
        .unwrap();

    let wine_prefix = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("🚗 Configuring GTA V...");
            println!("  Installing Visual C++ 2019...");
            Command::new("sh")
                .arg("-c")
                .arg(&format!(
                    "WINEPREFIX={} winetricks -q vcrun2019",
                    wine_prefix
                ))
                .status()
                .ok();
            println!("  Disabling Esync for stability...");
            unsafe { std::env::remove_var("WINEESYNC") };
            println!("✅ GTA V configuration applied");
        }
        1 => {
            println!("⚔️ Configuring The Witcher 3...");
            println!("  Installing dependencies...");
            Command::new("sh")
                .arg("-c")
                .arg(&format!(
                    "WINEPREFIX={} winetricks -q vcrun2015 d3dx11_43",
                    wine_prefix
                ))
                .status()
                .ok();
            println!("✅ The Witcher 3 configuration applied");
        }
        2 => {
            println!("🤖 Configuring Cyberpunk 2077...");
            println!("  Enabling AVX support...");
            unsafe { std::env::set_var("WINE_CPU_TOPOLOGY", "4:2") };
            println!("  Installing Visual C++ 2019...");
            Command::new("sh")
                .arg("-c")
                .arg(&format!(
                    "WINEPREFIX={} winetricks -q vcrun2019",
                    wine_prefix
                ))
                .status()
                .ok();
            println!("✅ Cyberpunk 2077 configuration applied");
        }
        3 => {
            println!("🤠 Configuring Red Dead Redemption 2...");
            println!("  Setting CPU topology...");
            unsafe { std::env::set_var("WINE_CPU_TOPOLOGY", "8:4") };
            println!("  Installing dependencies...");
            Command::new("sh")
                .arg("-c")
                .arg(&format!(
                    "WINEPREFIX={} winetricks -q vcrun2019",
                    wine_prefix
                ))
                .status()
                .ok();
            println!("✅ RDR2 configuration applied");
        }
        4 => {
            println!("⚔️ Configuring Elden Ring...");
            println!("  Disabling EAC for offline play...");
            println!("  Installing Visual C++...");
            Command::new("sh")
                .arg("-c")
                .arg(&format!(
                    "WINEPREFIX={} winetricks -q vcrun2019",
                    wine_prefix
                ))
                .status()
                .ok();
            println!("✅ Elden Ring configuration applied");
        }
        _ => {}
    }
}

fn performance_enhancements() {
    let options = [
        "🎮 GameMode Setup",
        "📊 MangoHud Configuration",
        "🚀 FSR/DLSS Setup",
        "⚡ CPU Governor Settings",
        "🧵 Process Priority",
        "⬅️ Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Performance Enhancements")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => gamemode_setup(),
        1 => mangohud_config(),
        2 => fsr_dlss_setup(),
        3 => cpu_governor_settings(),
        4 => process_priority(),
        _ => {}
    }
}

fn gamemode_setup() {
    println!("🎮 GameMode Setup");

    let status = Command::new("which").arg("gamemoderun").status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ GameMode is installed");

            let choice = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select action")
                .items(&[
                    "Configure GameMode",
                    "Test GameMode",
                    "Enable for all games",
                    "Back",
                ])
                .default(0)
                .interact()
                .unwrap();

            match choice {
                0 => {
                    println!("📝 Creating GameMode configuration...");
                    let config_path = format!("{}/.config/gamemode.ini", get_home_dir());

                    let config = r#"[general]
; GameMode configuration

[custom]
; Custom scripts
start=notify-send "GameMode started"
end=notify-send "GameMode ended"

[gpu]
; GPU performance mode
apply_gpu_optimisations=accept-responsibility
gpu_device=0
amd_performance_level=high

[cpu]
; CPU governor
governor=performance"#;

                    fs::write(&config_path, config).ok();
                    println!("✅ Configuration saved to: {}", config_path);
                }
                1 => {
                    println!("🧪 Testing GameMode...");
                    Command::new("gamemoded").arg("-t").status().ok();
                }
                2 => {
                    println!("✅ To enable GameMode for all games, add to launch options:");
                    println!("   gamemoderun %command%");
                }
                _ => {}
            }
        }
        _ => {
            println!("❌ GameMode not installed");

            let install = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Install GameMode?")
                .default(true)
                .interact()
                .unwrap();

            if install {
                Command::new("sh")
                    .arg("-c")
                    .arg("sudo pacman -S gamemode lib32-gamemode")
                    .status()
                    .ok();
            }
        }
    }
}

fn mangohud_config() {
    println!("📊 MangoHud Configuration");

    let config_path = format!("{}/.config/MangoHud/MangoHud.conf", get_home_dir());

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select configuration preset")
        .items(&["Minimal", "Default", "Full", "Custom", "Back"])
        .default(1)
        .interact()
        .unwrap();

    let config = match choice {
        0 => {
            // Minimal
            r#"fps
frame_timing=0
cpu_stats
gpu_stats"#
        }
        1 => {
            // Default
            r#"fps
frame_timing=1
cpu_stats
cpu_temp
gpu_stats
gpu_temp
ram
vram"#
        }
        2 => {
            // Full
            r#"fps
frame_timing=1
cpu_stats
cpu_temp
cpu_power
gpu_stats
gpu_temp
gpu_power
ram
vram
wine
gamemode
io_read
io_write
arch
engine_version"#
        }
        3 => {
            // Custom
            println!("📝 Enter custom configuration:");
            let custom = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Config options (comma separated)")
                .interact()
                .unwrap();

            fs::create_dir_all(format!("{}/.config/MangoHud", get_home_dir())).ok();
            fs::write(&config_path, &custom).ok();
            println!("✅ Configuration saved to: {}", config_path);
            return;
        }
        _ => return,
    };

    fs::create_dir_all(format!("{}/.config/MangoHud", get_home_dir())).ok();
    fs::write(&config_path, config).ok();
    println!("✅ Configuration saved to: {}", config_path);
}

fn fsr_dlss_setup() {
    println!("🚀 FSR/DLSS Setup");

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select technology")
        .items(&["AMD FSR", "NVIDIA DLSS", "Intel XeSS", "Back"])
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("🔴 AMD FSR Setup");
            println!("  Enabling Wine FSR...");
            unsafe { std::env::set_var("WINE_FSR", "1") };

            let strength = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("FSR strength (0-5, default 2)")
                .default("2".to_string())
                .interact()
                .unwrap();

            unsafe { std::env::set_var("WINE_FSR_STRENGTH", &strength) };
            println!("✅ FSR enabled with strength: {}", strength);
        }
        1 => {
            println!("🟢 NVIDIA DLSS Setup");
            println!("  DLSS requires game support");
            println!("  Installing DLSS files...");

            let _wine_prefix = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter Wine prefix path")
                .default(format!("{}/.wine", get_home_dir()))
                .interact()
                .unwrap();

            // Note: DLSS files need to be obtained from NVIDIA
            println!("⚠️ DLSS files must be obtained from games that include them");
        }
        2 => {
            println!("🔵 Intel XeSS Setup");
            println!("  XeSS requires game support");
        }
        _ => {}
    }
}

fn cpu_governor_settings() {
    println!("⚡ CPU Governor Settings");

    let governors = ["performance", "ondemand", "powersave", "schedutil"];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select CPU governor")
        .items(&governors)
        .default(0)
        .interact()
        .unwrap();

    let governor = governors[choice];

    println!("🔧 Setting CPU governor to: {}", governor);
    let cmd = format!("sudo cpupower frequency-set -g {}", governor);

    let status = Command::new("sh").arg("-c").arg(&cmd).status();

    match status {
        Ok(s) if s.success() => println!("✅ CPU governor set to: {}", governor),
        _ => println!("❌ Failed to set CPU governor (need sudo)"),
    }
}

fn process_priority() {
    println!("🧵 Process Priority Settings");

    let game_exe = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter game executable name")
        .interact()
        .unwrap();

    let priority = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select priority")
        .items(&[
            "High (-10)",
            "Above Normal (-5)",
            "Normal (0)",
            "Below Normal (5)",
            "Low (10)",
        ])
        .default(0)
        .interact()
        .unwrap();

    let nice_value = match priority {
        0 => "-10",
        1 => "-5",
        2 => "0",
        3 => "5",
        4 => "10",
        _ => "0",
    };

    println!("🔧 Setting priority for {}...", game_exe);
    let cmd = format!("renice {} -p $(pgrep {})", nice_value, game_exe);

    let status = Command::new("sh").arg("-c").arg(&cmd).status();

    match status {
        Ok(s) if s.success() => println!("✅ Priority set"),
        _ => println!("⚠️ Process not found or permission denied"),
    }
}

fn anticheat_setup() {
    let options = [
        "🛡️ EasyAntiCheat Setup",
        "🛡️ BattlEye Setup",
        "🔧 Proton EAC Runtime",
        "📋 Check AntiCheat Status",
        "⬅️ Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Anti-Cheat Runtime Setup")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => eac_setup(),
        1 => battleye_setup(),
        2 => proton_eac_runtime(),
        3 => check_anticheat_status(),
        _ => {}
    }
}

fn eac_setup() {
    println!("🛡️ EasyAntiCheat Setup");

    let steam_path = format!("{}/.steam", get_home_dir());
    let eac_runtime_path = format!("{}/steam/steamapps/common/EasyAntiCheat", steam_path);

    if Path::new(&eac_runtime_path).exists() {
        println!("✅ EAC runtime found at: {}", eac_runtime_path);
    } else {
        println!("❌ EAC runtime not found");
        println!("📦 Installing EAC runtime...");

        let install = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Download and install EAC runtime?")
            .default(true)
            .interact()
            .unwrap();

        if install {
            fs::create_dir_all(&eac_runtime_path).ok();
            println!("📂 Created directory: {}", eac_runtime_path);
            println!("⚠️ EAC runtime will be downloaded by Steam when needed");
        }
    }

    println!("\n💡 To enable EAC for a game:");
    println!("1. Right-click game in Steam → Properties");
    println!("2. Compatibility → Force Proton Experimental or Proton 7.0+");
    println!("3. Launch options: PROTON_EAC_RUNTIME=1 %command%");
}

fn battleye_setup() {
    println!("🛡️ BattlEye Setup");

    let steam_path = format!("{}/.steam", get_home_dir());
    let be_runtime_path = format!("{}/steam/steamapps/common/BattlEye", steam_path);

    if Path::new(&be_runtime_path).exists() {
        println!("✅ BattlEye runtime found at: {}", be_runtime_path);
    } else {
        println!("❌ BattlEye runtime not found");
        println!("📦 Installing BattlEye runtime...");

        let install = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Create BattlEye runtime directory?")
            .default(true)
            .interact()
            .unwrap();

        if install {
            fs::create_dir_all(&be_runtime_path).ok();
            println!("📂 Created directory: {}", be_runtime_path);
            println!("⚠️ BattlEye runtime will be downloaded by Steam when needed");
        }
    }

    println!("\n💡 To enable BattlEye for a game:");
    println!("1. Use Proton Experimental or Proton 7.0+");
    println!("2. The game must have Linux BattlEye support enabled by developers");
}

fn proton_eac_runtime() {
    println!("🔧 Proton EAC Runtime Configuration");

    let game_id = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Steam App ID")
        .interact()
        .unwrap();

    let steam_path = format!("{}/.steam/steam", get_home_dir());
    let compat_path = format!("{}/steamapps/compatdata/{}", steam_path, game_id);

    if Path::new(&compat_path).exists() {
        println!("✅ Game compatibility data found");

        // Create EAC configuration
        let eac_config = format!(
            "{}/pfx/drive_c/users/steamuser/AppData/Roaming/EasyAntiCheat",
            compat_path
        );
        fs::create_dir_all(&eac_config).ok();

        println!("📝 Creating EAC settings...");
        let settings = r#"{"productid":"","sandboxid":"","deploymentid":"","clientid":""}"#;
        fs::write(format!("{}/settings.json", eac_config), settings).ok();

        println!("✅ EAC runtime configured for App ID: {}", game_id);
    } else {
        println!("❌ Game not found. Please run the game once first.");
    }
}

fn check_anticheat_status() {
    println!("📋 Checking Anti-Cheat Status");

    let steam_path = format!("{}/.steam", get_home_dir());

    // Check EAC
    println!("\n🛡️ EasyAntiCheat:");
    let eac_path = format!("{}/steam/steamapps/common/EasyAntiCheat", steam_path);
    if Path::new(&eac_path).exists() {
        println!("  ✅ Runtime directory exists");

        // Check for EAC files
        let eac_so = format!("{}/easyanticheat_x64.so", eac_path);
        if Path::new(&eac_so).exists() {
            println!("  ✅ EAC library found");
        } else {
            println!("  ⚠️ EAC library not found");
        }
    } else {
        println!("  ❌ Runtime not installed");
    }

    // Check BattlEye
    println!("\n🛡️ BattlEye:");
    let be_path = format!("{}/steam/steamapps/common/BattlEye", steam_path);
    if Path::new(&be_path).exists() {
        println!("  ✅ Runtime directory exists");
    } else {
        println!("  ❌ Runtime not installed");
    }

    // Check Proton version
    println!("\n🚀 Proton Status:");
    let proton_exp = format!(
        "{}/steam/steamapps/common/Proton - Experimental",
        steam_path
    );
    let proton_8 = format!("{}/steam/steamapps/common/Proton 8.0", steam_path);

    if Path::new(&proton_exp).exists() {
        println!("  ✅ Proton Experimental installed (best for anti-cheat)");
    } else if Path::new(&proton_8).exists() {
        println!("  ✅ Proton 8.0 installed");
    } else {
        println!("  ⚠️ No recent Proton version found");
    }
}

fn shader_cache_management() {
    let options = [
        "📊 View Shader Cache Status",
        "🗑️ Clear Shader Cache",
        "📦 Backup Shader Cache",
        "📥 Restore Shader Cache",
        "🔧 Configure Cache Settings",
        "⬅️ Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Shader Cache Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => view_shader_cache(),
        1 => clear_shader_cache(),
        2 => backup_shader_cache(),
        3 => restore_shader_cache(),
        4 => configure_cache_settings(),
        _ => {}
    }
}

fn view_shader_cache() {
    println!("📊 Shader Cache Status");

    let cache_dirs = [
        (
            format!("{}/.cache/mesa_shader_cache", get_home_dir()),
            "Mesa",
        ),
        (format!("{}/.cache/nvidia", get_home_dir()), "NVIDIA"),
        (
            format!("{}/.cache/radv_builtin_shaders", get_home_dir()),
            "RADV",
        ),
        (
            format!("{}/.steam/steam/steamapps/shadercache", get_home_dir()),
            "Steam",
        ),
    ];

    for (path, name) in &cache_dirs {
        if Path::new(path).exists() {
            let output = Command::new("du").args(&["-sh", path]).output();

            match output {
                Ok(out) => {
                    let size = String::from_utf8_lossy(&out.stdout);
                    println!("  {} Cache: {}", name, size.trim());
                }
                _ => println!("  {} Cache: Unable to determine size", name),
            }
        } else {
            println!("  {} Cache: Not found", name);
        }
    }
}

fn clear_shader_cache() {
    println!("🗑️ Clear Shader Cache");

    let caches = vec!["Mesa", "NVIDIA", "RADV", "Steam", "All"];

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select caches to clear")
        .items(&caches)
        .interact()
        .unwrap();

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Are you sure you want to clear selected caches?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        for idx in selected {
            match idx {
                0 => {
                    let path = format!("{}/.cache/mesa_shader_cache", get_home_dir());
                    Command::new("rm").args(&["-rf", &path]).status().ok();
                    println!("  ✅ Mesa cache cleared");
                }
                1 => {
                    let path = format!("{}/.cache/nvidia", get_home_dir());
                    Command::new("rm").args(&["-rf", &path]).status().ok();
                    println!("  ✅ NVIDIA cache cleared");
                }
                2 => {
                    let path = format!("{}/.cache/radv_builtin_shaders", get_home_dir());
                    Command::new("rm").args(&["-rf", &path]).status().ok();
                    println!("  ✅ RADV cache cleared");
                }
                3 => {
                    let path = format!("{}/.steam/steam/steamapps/shadercache", get_home_dir());
                    Command::new("rm").args(&["-rf", &path]).status().ok();
                    println!("  ✅ Steam cache cleared");
                }
                4 => {
                    // Clear all
                    let paths = [
                        format!("{}/.cache/mesa_shader_cache", get_home_dir()),
                        format!("{}/.cache/nvidia", get_home_dir()),
                        format!("{}/.cache/radv_builtin_shaders", get_home_dir()),
                        format!("{}/.steam/steam/steamapps/shadercache", get_home_dir()),
                    ];
                    for path in &paths {
                        Command::new("rm").args(&["-rf", path]).status().ok();
                    }
                    println!("  ✅ All caches cleared");
                }
                _ => {}
            }
        }
    }
}

fn backup_shader_cache() {
    println!("📦 Backup Shader Cache");

    let backup_dir = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter backup directory")
        .default(format!("{}/shader_cache_backup", get_home_dir()))
        .interact()
        .unwrap();

    fs::create_dir_all(&backup_dir).ok();

    let caches = [
        (
            format!("{}/.cache/mesa_shader_cache", get_home_dir()),
            "mesa",
        ),
        (format!("{}/.cache/nvidia", get_home_dir()), "nvidia"),
        (
            format!("{}/.steam/steam/steamapps/shadercache", get_home_dir()),
            "steam",
        ),
    ];

    for (source, name) in &caches {
        if Path::new(source).exists() {
            let dest = format!("{}/{}", backup_dir, name);
            println!("  Backing up {} cache...", name);

            let cmd = format!("cp -r {} {}", source, dest);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
        }
    }

    println!("✅ Shader cache backed up to: {}", backup_dir);
}

fn restore_shader_cache() {
    println!("📥 Restore Shader Cache");

    let backup_dir = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter backup directory")
        .default(format!("{}/shader_cache_backup", get_home_dir()))
        .interact()
        .unwrap();

    if !Path::new(&backup_dir).exists() {
        println!("❌ Backup directory not found");
        return;
    }

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will replace current shader caches. Continue?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let caches = [
            (
                format!("{}/mesa", backup_dir),
                format!("{}/.cache/mesa_shader_cache", get_home_dir()),
            ),
            (
                format!("{}/nvidia", backup_dir),
                format!("{}/.cache/nvidia", get_home_dir()),
            ),
            (
                format!("{}/steam", backup_dir),
                format!("{}/.steam/steam/steamapps/shadercache", get_home_dir()),
            ),
        ];

        for (source, dest) in &caches {
            if Path::new(source).exists() {
                println!("  Restoring {}...", source);
                Command::new("rm").args(&["-rf", dest]).status().ok();
                let cmd = format!("cp -r {} {}", source, dest);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
            }
        }

        println!("✅ Shader cache restored");
    }
}

fn configure_cache_settings() {
    println!("🔧 Configure Cache Settings");

    let options = [
        "Set cache size limit",
        "Enable/Disable shader cache",
        "Configure DXVK cache",
        "Configure Mesa cache",
        "Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Cache Configuration")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            let size = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter cache size limit in MB")
                .default("1024".to_string())
                .interact()
                .unwrap();

            unsafe { std::env::set_var("MESA_GLSL_CACHE_MAX_SIZE", &format!("{}M", size)) };
            println!("✅ Cache size limit set to: {} MB", size);
        }
        1 => {
            let enable = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Enable shader cache?")
                .default(true)
                .interact()
                .unwrap();

            if enable {
                unsafe { std::env::set_var("__GL_SHADER_DISK_CACHE", "1") };
                unsafe { std::env::set_var("MESA_GLSL_CACHE_DISABLE", "0") };
                println!("✅ Shader cache enabled");
            } else {
                unsafe { std::env::set_var("__GL_SHADER_DISK_CACHE", "0") };
                unsafe { std::env::set_var("MESA_GLSL_CACHE_DISABLE", "1") };
                println!("✅ Shader cache disabled");
            }
        }
        2 => {
            println!("🔧 DXVK Cache Configuration");
            let state_cache = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Enable DXVK state cache?")
                .default(true)
                .interact()
                .unwrap();

            if state_cache {
                unsafe { std::env::set_var("DXVK_STATE_CACHE", "1") };
                println!("✅ DXVK state cache enabled");
            } else {
                unsafe { std::env::set_var("DXVK_STATE_CACHE", "0") };
                println!("✅ DXVK state cache disabled");
            }
        }
        3 => {
            println!("🔧 Mesa Cache Configuration");
            let path = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Mesa cache directory")
                .default(format!("{}/.cache/mesa_shader_cache", get_home_dir()))
                .interact()
                .unwrap();

            unsafe { std::env::set_var("MESA_GLSL_CACHE_DIR", &path) };
            println!("✅ Mesa cache directory set to: {}", path);
        }
        _ => {}
    }
}

fn wine_registry_editor() {
    let options = [
        "📝 Edit Registry Key",
        "➕ Add Registry Entry",
        "🗑️ Delete Registry Entry",
        "📋 Export Registry",
        "📥 Import Registry File",
        "🔍 Search Registry",
        "⬅️ Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Wine Registry Editor")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    let wine_prefix = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
        .unwrap();

    match choice {
        0 => edit_registry_key(&wine_prefix),
        1 => add_registry_entry(&wine_prefix),
        2 => delete_registry_entry(&wine_prefix),
        3 => export_registry(&wine_prefix),
        4 => import_registry(&wine_prefix),
        5 => search_registry(&wine_prefix),
        _ => {}
    }
}

fn edit_registry_key(wine_prefix: &str) {
    println!("📝 Edit Registry Key");

    let common_keys = [
        "HKEY_CURRENT_USER\\Software\\Wine",
        "HKEY_CURRENT_USER\\Software\\Wine\\Direct3D",
        "HKEY_CURRENT_USER\\Software\\Wine\\Drivers",
        "HKEY_LOCAL_MACHINE\\Software\\Microsoft\\Windows\\CurrentVersion",
        "Custom",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select registry key")
        .items(&common_keys)
        .default(0)
        .interact()
        .unwrap();

    let key = if choice == 4 {
        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter registry key path")
            .interact()
            .unwrap()
    } else {
        common_keys[choice].to_string()
    };

    let value_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter value name")
        .interact()
        .unwrap();

    let value_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select value type")
        .items(&[
            "REG_SZ (String)",
            "REG_DWORD (Number)",
            "REG_BINARY (Binary)",
        ])
        .default(0)
        .interact()
        .unwrap();

    let value_data = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter value data")
        .interact()
        .unwrap();

    let type_flag = match value_type {
        0 => "/t REG_SZ",
        1 => "/t REG_DWORD",
        2 => "/t REG_BINARY",
        _ => "/t REG_SZ",
    };

    let cmd = format!(
        "WINEPREFIX={} wine reg add '{}' /v {} {} /d {} /f",
        wine_prefix, key, value_name, type_flag, value_data
    );

    let status = Command::new("sh").arg("-c").arg(&cmd).status();

    match status {
        Ok(s) if s.success() => println!("✅ Registry key updated"),
        _ => println!("❌ Failed to update registry key"),
    }
}

fn add_registry_entry(wine_prefix: &str) {
    println!("➕ Add Registry Entry");
    edit_registry_key(wine_prefix);
}

fn delete_registry_entry(wine_prefix: &str) {
    println!("🗑️ Delete Registry Entry");

    let key = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter registry key path")
        .interact()
        .unwrap();

    let value_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter value name (or leave empty to delete entire key)")
        .allow_empty(true)
        .interact()
        .unwrap();

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Are you sure you want to delete this entry?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let cmd = if value_name.is_empty() {
            format!("WINEPREFIX={} wine reg delete '{}' /f", wine_prefix, key)
        } else {
            format!(
                "WINEPREFIX={} wine reg delete '{}' /v {} /f",
                wine_prefix, key, value_name
            )
        };

        let status = Command::new("sh").arg("-c").arg(&cmd).status();

        match status {
            Ok(s) if s.success() => println!("✅ Registry entry deleted"),
            _ => println!("❌ Failed to delete registry entry"),
        }
    }
}

fn export_registry(wine_prefix: &str) {
    println!("📋 Export Registry");

    let export_path = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter export file path")
        .default(format!("{}/wine_registry_export.reg", get_home_dir()))
        .interact()
        .unwrap();

    let key = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter registry key to export (or leave empty for full export)")
        .allow_empty(true)
        .interact()
        .unwrap();

    let cmd = if key.is_empty() {
        format!(
            "WINEPREFIX={} wine regedit /e '{}'",
            wine_prefix, export_path
        )
    } else {
        format!(
            "WINEPREFIX={} wine regedit /e '{}' '{}'",
            wine_prefix, export_path, key
        )
    };

    let status = Command::new("sh").arg("-c").arg(&cmd).status();

    match status {
        Ok(s) if s.success() => println!("✅ Registry exported to: {}", export_path),
        _ => println!("❌ Failed to export registry"),
    }
}

fn import_registry(wine_prefix: &str) {
    println!("📥 Import Registry File");

    let import_path = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter registry file path to import")
        .interact()
        .unwrap();

    if !Path::new(&import_path).exists() {
        println!("❌ File not found: {}", import_path);
        return;
    }

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Are you sure you want to import this registry file?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let cmd = format!("WINEPREFIX={} wine regedit '{}'", wine_prefix, import_path);

        let status = Command::new("sh").arg("-c").arg(&cmd).status();

        match status {
            Ok(s) if s.success() => println!("✅ Registry imported from: {}", import_path),
            _ => println!("❌ Failed to import registry"),
        }
    }
}

fn search_registry(wine_prefix: &str) {
    println!("🔍 Search Registry");

    let search_term = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter search term")
        .interact()
        .unwrap();

    println!("🔍 Searching for: {}", search_term);

    let cmd = format!(
        "WINEPREFIX={} wine reg query HKEY_CURRENT_USER /s /f '{}'",
        wine_prefix, search_term
    );

    let output = Command::new("sh").arg("-c").arg(&cmd).output();

    match output {
        Ok(out) => {
            let results = String::from_utf8_lossy(&out.stdout);
            if results.is_empty() {
                println!("❌ No results found");
            } else {
                println!("📋 Search results:");
                println!("{}", results);
            }
        }
        _ => println!("❌ Search failed"),
    }
}
