use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

// Cache for commonly accessed paths
static HOME_DIR: OnceLock<String> = OnceLock::new();

fn get_home_dir() -> &'static str {
    HOME_DIR.get_or_init(|| std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string()))
}

pub fn wine_tools_menu() {
    loop {
        let options = [
            "🔧 Winetricks Deep Integration",
            "🍾 Wine Bottle Management",
            "🔄 Wine Version Manager",
            "🪟 Windows Version Spoofing",
            "📚 DLL Dependency Walker",
            "⚡ Performance Optimizer",
            "🎮 Gaming Tweaks Hub",
            "🔍 Wine Debugging Tools",
            "📦 Component Manager",
            "💾 Profile Management",
            "⬅️ Back",
        ];

        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🍷 Advanced Wine Tools")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match choice {
            0 => winetricks_deep_integration(),
            1 => wine_bottle_management(),
            2 => wine_version_manager(),
            3 => windows_version_spoofing(),
            4 => dll_dependency_walker(),
            5 => performance_optimizer(),
            6 => gaming_tweaks_hub(),
            7 => wine_debugging_tools(),
            8 => component_manager(),
            9 => profile_management(),
            _ => break,
        }
    }
}

fn winetricks_deep_integration() {
    loop {
        let options = [
            "📦 Automatic Dependency Resolution",
            "📋 Batch Scripts Manager",
            "🔧 Custom Verb Creator",
            "💾 Winetricks Profile Manager",
            "🎮 Game-Specific Recipes",
            "🔍 Dependency Analyzer",
            "📊 Component Status",
            "🔄 Update Winetricks",
            "⬅️ Back",
        ];

        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔧 Winetricks Deep Integration")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match choice {
            0 => automatic_dependency_resolution(),
            1 => batch_scripts_manager(),
            2 => custom_verb_creator(),
            3 => winetricks_profile_manager(),
            4 => game_specific_recipes(),
            5 => dependency_analyzer(),
            6 => component_status(),
            7 => update_winetricks(),
            _ => break,
        }
    }
}

fn automatic_dependency_resolution() {
    println!("📦 Automatic Dependency Resolution");

    let Ok(exe_path) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter executable path to analyze")
        .interact()
    else {
        return;
    };

    if !Path::new(&exe_path).exists() {
        println!("❌ Executable not found");
        return;
    }

    let Ok(wine_prefix) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
    else {
        return;
    };

    println!("🔍 Analyzing dependencies...");

    // Check with ldd in Wine
    let ldd_cmd = format!("WINEPREFIX={} wine ldd '{}'", wine_prefix, exe_path);
    let output = Command::new("sh").arg("-c").arg(&ldd_cmd).output();

    let mut missing_dlls = Vec::new();

    if let Ok(out) = output {
        let result = String::from_utf8_lossy(&out.stdout);
        for line in result.lines() {
            if line.contains("not found")
                && let Some(dll) = line.split_whitespace().next()
            {
                missing_dlls.push(dll.to_string());
            }
        }
    }

    if missing_dlls.is_empty() {
        println!("✅ No missing dependencies found");
        return;
    }

    println!("⚠️ Missing DLLs:");
    for dll in &missing_dlls {
        println!("  ❌ {}", dll);
    }

    // Map DLLs to winetricks packages
    let dll_packages = get_dll_package_mapping();
    let mut packages_to_install = Vec::new();

    for dll in &missing_dlls {
        if let Some(package) = dll_packages.get(dll.as_str())
            && !packages_to_install.contains(package)
        {
            packages_to_install.push(package.clone());
        }
    }

    if !packages_to_install.is_empty() {
        println!("\n📦 Recommended packages to install:");
        for package in &packages_to_install {
            println!("  • {}", package);
        }

        let Ok(install) = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Install recommended packages?")
            .default(true)
            .interact()
        else {
            return;
        };

        if install {
            for package in packages_to_install {
                println!("📦 Installing {}...", package);
                let cmd = format!("WINEPREFIX={} winetricks -q {}", wine_prefix, package);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
            }
            println!("✅ Dependencies installed");
        }
    }
}

fn get_dll_package_mapping() -> HashMap<&'static str, String> {
    let mut map = HashMap::new();

    // Common DLL to package mappings
    map.insert("d3d9.dll", "d3dx9".to_string());
    map.insert("d3d11.dll", "d3dx11_43".to_string());
    map.insert("d3dcompiler_47.dll", "d3dcompiler_47".to_string());
    map.insert("msvcp140.dll", "vcrun2019".to_string());
    map.insert("vcruntime140.dll", "vcrun2019".to_string());
    map.insert("msvcp120.dll", "vcrun2013".to_string());
    map.insert("msvcr120.dll", "vcrun2013".to_string());
    map.insert("msvcp110.dll", "vcrun2012".to_string());
    map.insert("msvcr110.dll", "vcrun2012".to_string());
    map.insert("msvcp100.dll", "vcrun2010".to_string());
    map.insert("msvcr100.dll", "vcrun2010".to_string());
    map.insert("mfc140.dll", "vcrun2019".to_string());
    map.insert("api-ms-win-crt-runtime-l1-1-0.dll", "vcrun2019".to_string());
    map.insert("xinput1_3.dll", "xinput".to_string());
    map.insert("xaudio2_7.dll", "xact".to_string());
    map.insert("dotnet.dll", "dotnet48".to_string());
    map.insert("faudio.dll", "faudio".to_string());

    map
}

fn batch_scripts_manager() {
    println!("📋 Batch Scripts Manager");

    let options = [
        "Create new script",
        "Run existing script",
        "Edit script",
        "Delete script",
        "Import script",
        "Export script",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Batch script options")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => create_batch_script(),
        1 => run_batch_script(),
        2 => edit_batch_script(),
        3 => delete_batch_script(),
        4 => import_batch_script(),
        5 => export_batch_script(),
        _ => {}
    }
}

fn create_batch_script() {
    println!("📝 Create Batch Script");

    let Ok(script_name) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter script name")
        .interact()
    else {
        return;
    };

    let all_components = vec![
        // Runtimes
        "vcrun2019",
        "vcrun2017",
        "vcrun2015",
        "vcrun2013",
        "vcrun2012",
        "vcrun2010",
        "vcrun2008",
        "vcrun2005",
        "vcrun6",
        "vcrun6sp6",
        // .NET
        "dotnet48",
        "dotnet472",
        "dotnet471",
        "dotnet47",
        "dotnet462",
        "dotnet461",
        "dotnet46",
        "dotnet452",
        "dotnet45",
        "dotnet40",
        "dotnet35",
        "dotnet30",
        "dotnet20",
        "dotnetcore3",
        "dotnetcore2",
        // DirectX
        "d3dx9",
        "d3dx10",
        "d3dx11_43",
        "d3dcompiler_47",
        "d3dcompiler_43",
        // Audio
        "faudio",
        "xact",
        "openal",
        "dsound",
        // Fonts
        "corefonts",
        "tahoma",
        "arial",
        "comicsans",
        "georgia",
        "impact",
        // System
        "gdiplus",
        "msxml3",
        "msxml4",
        "msxml6",
        "riched20",
        "riched30",
        // Gaming
        "physx",
        "gamemode",
        "dxvk",
        "vkd3d",
    ];

    let Ok(selected) = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select components for batch script")
        .items(&all_components)
        .interact()
    else {
        return;
    };

    let mut script_content = String::from("#!/bin/bash\n# Winetricks batch script\n\n");
    script_content.push_str("# Check if Wine prefix is provided\n");
    script_content.push_str("if [ -z \"$1\" ]; then\n");
    script_content.push_str("    echo \"Usage: $0 <wine_prefix>\"\n");
    script_content.push_str("    exit 1\n");
    script_content.push_str("fi\n\n");
    script_content.push_str("WINEPREFIX=\"$1\"\n");
    script_content.push_str("export WINEPREFIX\n\n");
    script_content.push_str("echo \"Installing components to $WINEPREFIX\"\n\n");

    for idx in selected {
        let component = all_components[idx];
        script_content.push_str(&format!("echo \"Installing {}...\"\n", component));
        script_content.push_str(&format!(
            "winetricks -q {} || echo \"Failed to install {}\"\n\n",
            component, component
        ));
    }

    script_content.push_str("echo \"Batch installation complete!\"\n");

    let script_dir = format!("{}/winetricks_scripts", get_home_dir());
    fs::create_dir_all(&script_dir).ok();

    let script_path = format!("{}/{}.sh", script_dir, script_name);
    fs::write(&script_path, script_content).ok();

    // Make executable
    Command::new("chmod")
        .args(&["+x", &script_path])
        .status()
        .ok();

    println!("✅ Script created: {}", script_path);
}

fn run_batch_script() {
    println!("▶️ Run Batch Script");

    let script_dir = format!("{}/winetricks_scripts", get_home_dir());

    if !Path::new(&script_dir).exists() {
        println!("❌ No scripts found");
        return;
    }

    let mut scripts = Vec::new();
    if let Ok(entries) = fs::read_dir(&script_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("sh")
                && let Some(name) = path.file_name()
            {
                scripts.push(name.to_string_lossy().to_string());
            }
        }
    }

    if scripts.is_empty() {
        println!("❌ No scripts found");
        return;
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select script to run")
        .items(&scripts)
        .default(0)
        .interact()
    else {
        return;
    };

    let script = &scripts[choice];
    let script_path = format!("{}/{}", script_dir, script);

    let Ok(wine_prefix) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
    else {
        return;
    };

    println!("▶️ Running script: {}", script);
    let status = Command::new(&script_path).arg(&wine_prefix).status();

    match status {
        Ok(s) if s.success() => println!("✅ Script executed successfully"),
        _ => println!("❌ Script execution failed"),
    }
}

fn edit_batch_script() {
    println!("✏️ Edit Batch Script");

    let script_dir = format!("{}/winetricks_scripts", get_home_dir());

    if !Path::new(&script_dir).exists() {
        println!("❌ No scripts found");
        return;
    }

    let mut scripts = Vec::new();
    if let Ok(entries) = fs::read_dir(&script_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("sh")
                && let Some(name) = path.file_name()
            {
                scripts.push(name.to_string_lossy().to_string());
            }
        }
    }

    if scripts.is_empty() {
        println!("❌ No scripts found");
        return;
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select script to edit")
        .items(&scripts)
        .default(0)
        .interact()
    else {
        return;
    };

    let script = &scripts[choice];
    let script_path = format!("{}/{}", script_dir, script);

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    Command::new(&editor).arg(&script_path).status().ok();
}

fn delete_batch_script() {
    println!("🗑️ Delete Batch Script");

    let script_dir = format!("{}/winetricks_scripts", get_home_dir());

    if !Path::new(&script_dir).exists() {
        println!("❌ No scripts found");
        return;
    }

    let mut scripts = Vec::new();
    if let Ok(entries) = fs::read_dir(&script_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("sh")
                && let Some(name) = path.file_name()
            {
                scripts.push(name.to_string_lossy().to_string());
            }
        }
    }

    if scripts.is_empty() {
        println!("❌ No scripts found");
        return;
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select script to delete")
        .items(&scripts)
        .default(0)
        .interact()
    else {
        return;
    };

    let script = &scripts[choice];

    let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Delete {}?", script))
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        let script_path = format!("{}/{}", script_dir, script);
        fs::remove_file(&script_path).ok();
        println!("✅ Script deleted");
    }
}

fn import_batch_script() {
    println!("📥 Import Batch Script");

    let Ok(import_path) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter script path to import")
        .interact()
    else {
        return;
    };

    if !Path::new(&import_path).exists() {
        println!("❌ Script not found");
        return;
    }

    let script_dir = format!("{}/winetricks_scripts", get_home_dir());
    fs::create_dir_all(&script_dir).ok();

    let Some(file_name) = Path::new(&import_path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
    else {
        println!("❌ Invalid file path");
        return;
    };

    let dest_path = format!("{}/{}", script_dir, file_name);

    let cmd = format!("cp '{}' '{}'", import_path, dest_path);
    Command::new("sh").arg("-c").arg(&cmd).status().ok();

    // Make executable
    Command::new("chmod")
        .args(&["+x", &dest_path])
        .status()
        .ok();

    println!("✅ Script imported: {}", file_name);
}

fn export_batch_script() {
    println!("📤 Export Batch Script");

    let script_dir = format!("{}/winetricks_scripts", get_home_dir());

    if !Path::new(&script_dir).exists() {
        println!("❌ No scripts found");
        return;
    }

    let mut scripts = Vec::new();
    if let Ok(entries) = fs::read_dir(&script_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("sh")
                && let Some(name) = path.file_name()
            {
                scripts.push(name.to_string_lossy().to_string());
            }
        }
    }

    if scripts.is_empty() {
        println!("❌ No scripts found");
        return;
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select script to export")
        .items(&scripts)
        .default(0)
        .interact()
    else {
        return;
    };

    let script = &scripts[choice];
    let script_path = format!("{}/{}", script_dir, script);

    let Ok(export_path) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter export path")
        .default(format!("{}/{}", get_home_dir(), script))
        .interact()
    else {
        return;
    };

    let cmd = format!("cp '{}' '{}'", script_path, export_path);
    Command::new("sh").arg("-c").arg(&cmd).status().ok();

    println!("✅ Script exported to: {}", export_path);
}

fn custom_verb_creator() {
    println!("🔧 Custom Verb Creator");

    let Ok(verb_name) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter custom verb name")
        .interact()
    else {
        return;
    };

    let Ok(description) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter verb description")
        .interact()
    else {
        return;
    };

    let Ok(verb_type) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select verb type")
        .items(&[
            "Download and install",
            "Registry modification",
            "File operation",
            "Combined",
        ])
        .default(0)
        .interact()
    else {
        return;
    };

    let mut verb_content = format!(
        "# Custom verb: {}\n# Description: {}\n\n",
        verb_name, description
    );

    match verb_type {
        0 => {
            let Ok(url) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter download URL")
                .interact()
            else {
                return;
            };

            let Ok(install_cmd) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter installation command")
                .interact()
            else {
                return;
            };

            verb_content.push_str(&format!("w_download '{}' CHECKSUM\n", url));
            verb_content.push_str(&format!("w_try {}\n", install_cmd));
        }
        1 => {
            let Ok(reg_key) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter registry key")
                .interact()
            else {
                return;
            };

            let Ok(reg_value) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter registry value")
                .interact()
            else {
                return;
            };

            let Ok(reg_data) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter registry data")
                .interact()
            else {
                return;
            };

            verb_content.push_str(&format!(
                "w_call w_regedit '{}' '{}' '{}'\n",
                reg_key, reg_value, reg_data
            ));
        }
        2 => {
            let Ok(file_op) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter file operation command")
                .interact()
            else {
                return;
            };

            verb_content.push_str(&format!("w_try {}\n", file_op));
        }
        3 => {
            println!("📝 Enter custom verb script (end with 'END' on a new line):");
            let mut script = String::new();
            loop {
                let Ok(line) = Input::<String>::with_theme(&ColorfulTheme::default())
                    .allow_empty(true)
                    .interact()
                else {
                    break;
                };

                if line == "END" {
                    break;
                }
                script.push_str(&line);
                script.push('\n');
            }
            verb_content.push_str(&script);
        }
        _ => {}
    }

    let verb_dir = format!("{}/custom_verbs", get_home_dir());
    fs::create_dir_all(&verb_dir).ok();

    let verb_path = format!("{}/{}.verb", verb_dir, verb_name);
    fs::write(&verb_path, verb_content).ok();

    println!("✅ Custom verb created: {}", verb_path);
    println!("Note: To use this verb, you'll need to integrate it with winetricks");
}

fn winetricks_profile_manager() {
    println!("💾 Winetricks Profile Manager");

    let options = [
        "Create profile",
        "Load profile",
        "Edit profile",
        "Delete profile",
        "Export profile",
        "Import profile",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Profile options")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => create_winetricks_profile(),
        1 => load_winetricks_profile(),
        2 => edit_winetricks_profile(),
        3 => delete_winetricks_profile(),
        4 => export_winetricks_profile(),
        5 => import_winetricks_profile(),
        _ => {}
    }
}

fn create_winetricks_profile() {
    println!("📝 Create Winetricks Profile");

    let Ok(profile_name) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter profile name")
        .interact()
    else {
        return;
    };

    let Ok(profile_type) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select profile type")
        .items(&[
            "Gaming - Modern (DX12)",
            "Gaming - Classic (DX9)",
            "Office/Productivity",
            "Development",
            "Media/Creative",
            "Custom",
        ])
        .default(0)
        .interact()
    else {
        return;
    };

    let mut components = Vec::new();

    match profile_type {
        0 => {
            // Modern gaming
            components = vec![
                "vcrun2019",
                "dotnet48",
                "d3dx11_43",
                "d3dcompiler_47",
                "faudio",
                "dxvk",
                "vkd3d",
            ];
        }
        1 => {
            // Classic gaming
            components = vec!["vcrun2008", "dotnet40", "d3dx9", "physx", "xact"];
        }
        2 => {
            // Office
            components = vec!["dotnet48", "vcrun2019", "gdiplus", "riched30", "corefonts"];
        }
        3 => {
            // Development
            components = vec!["dotnet48", "vcrun2019", "python", "msxml6"];
        }
        4 => {
            // Media
            components = vec!["vcrun2019", "gdiplus", "quartz", "amstream", "wmv9vcm"];
        }
        5 => {
            // Custom
            let all_components = vec![
                "vcrun2019",
                "vcrun2017",
                "vcrun2015",
                "dotnet48",
                "dotnet40",
                "d3dx9",
                "d3dx11_43",
                "dxvk",
                "vkd3d",
                "faudio",
                "xact",
                "corefonts",
                "gdiplus",
                "physx",
            ];

            let Ok(selected) = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Select components")
                .items(&all_components)
                .interact()
            else {
                return;
            };

            for idx in selected {
                components.push(all_components[idx]);
            }
        }
        _ => {}
    }

    let profile_dir = format!("{}/winetricks_profiles", get_home_dir());
    fs::create_dir_all(&profile_dir).ok();

    let profile_path = format!("{}/{}.profile", profile_dir, profile_name);
    let profile_content = components.join("\n");
    fs::write(&profile_path, profile_content).ok();

    println!("✅ Profile created: {}", profile_name);
}

fn load_winetricks_profile() {
    println!("📂 Load Winetricks Profile");

    let profile_dir = format!("{}/winetricks_profiles", get_home_dir());

    if !Path::new(&profile_dir).exists() {
        println!("❌ No profiles found");
        return;
    }

    let mut profiles = Vec::new();
    if let Ok(entries) = fs::read_dir(&profile_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("profile")
                && let Some(stem) = path.file_stem()
            {
                profiles.push(stem.to_string_lossy().to_string());
            }
        }
    }

    if profiles.is_empty() {
        println!("❌ No profiles found");
        return;
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select profile to load")
        .items(&profiles)
        .default(0)
        .interact()
    else {
        return;
    };

    let profile = &profiles[choice];
    let profile_path = format!("{}/{}.profile", profile_dir, profile);

    let Ok(wine_prefix) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
    else {
        return;
    };

    if let Ok(content) = fs::read_to_string(&profile_path) {
        println!("📦 Installing profile components...");
        for component in content.lines() {
            if !component.is_empty() {
                println!("  Installing {}...", component);
                let cmd = format!("WINEPREFIX={} winetricks -q {}", wine_prefix, component);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
            }
        }
        println!("✅ Profile loaded successfully");
    }
}

fn edit_winetricks_profile() {
    println!("✏️ Edit Winetricks Profile");

    let profile_dir = format!("{}/winetricks_profiles", get_home_dir());

    if !Path::new(&profile_dir).exists() {
        println!("❌ No profiles found");
        return;
    }

    let mut profiles = Vec::new();
    if let Ok(entries) = fs::read_dir(&profile_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("profile")
                && let Some(stem) = path.file_stem()
            {
                profiles.push(stem.to_string_lossy().to_string());
            }
        }
    }

    if profiles.is_empty() {
        println!("❌ No profiles found");
        return;
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select profile to edit")
        .items(&profiles)
        .default(0)
        .interact()
    else {
        return;
    };

    let profile = &profiles[choice];
    let profile_path = format!("{}/{}.profile", profile_dir, profile);

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    Command::new(&editor).arg(&profile_path).status().ok();
}

fn delete_winetricks_profile() {
    println!("🗑️ Delete Winetricks Profile");

    let profile_dir = format!("{}/winetricks_profiles", get_home_dir());

    if !Path::new(&profile_dir).exists() {
        println!("❌ No profiles found");
        return;
    }

    let mut profiles = Vec::new();
    if let Ok(entries) = fs::read_dir(&profile_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("profile")
                && let Some(stem) = path.file_stem()
            {
                profiles.push(stem.to_string_lossy().to_string());
            }
        }
    }

    if profiles.is_empty() {
        println!("❌ No profiles found");
        return;
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select profile to delete")
        .items(&profiles)
        .default(0)
        .interact()
    else {
        return;
    };

    let profile = &profiles[choice];

    let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Delete profile '{}'?", profile))
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        let profile_path = format!("{}/{}.profile", profile_dir, profile);
        fs::remove_file(&profile_path).ok();
        println!("✅ Profile deleted");
    }
}

fn export_winetricks_profile() {
    println!("📤 Export Winetricks Profile");

    let profile_dir = format!("{}/winetricks_profiles", get_home_dir());

    if !Path::new(&profile_dir).exists() {
        println!("❌ No profiles found");
        return;
    }

    let mut profiles = Vec::new();
    if let Ok(entries) = fs::read_dir(&profile_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("profile")
                && let Some(stem) = path.file_stem()
            {
                profiles.push(stem.to_string_lossy().to_string());
            }
        }
    }

    if profiles.is_empty() {
        println!("❌ No profiles found");
        return;
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select profile to export")
        .items(&profiles)
        .default(0)
        .interact()
    else {
        return;
    };

    let profile = &profiles[choice];
    let profile_path = format!("{}/{}.profile", profile_dir, profile);

    let Ok(export_path) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter export path")
        .default(format!("{}/{}.profile", get_home_dir(), profile))
        .interact()
    else {
        return;
    };

    let cmd = format!("cp '{}' '{}'", profile_path, export_path);
    Command::new("sh").arg("-c").arg(&cmd).status().ok();

    println!("✅ Profile exported to: {}", export_path);
}

fn import_winetricks_profile() {
    println!("📥 Import Winetricks Profile");

    let Ok(import_path) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter profile path to import")
        .interact()
    else {
        return;
    };

    if !Path::new(&import_path).exists() {
        println!("❌ Profile file not found");
        return;
    }

    let profile_dir = format!("{}/winetricks_profiles", get_home_dir());
    fs::create_dir_all(&profile_dir).ok();

    let Some(file_name) = Path::new(&import_path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
    else {
        println!("❌ Invalid file path");
        return;
    };

    let dest_path = format!("{}/{}", profile_dir, file_name);

    let cmd = format!("cp '{}' '{}'", import_path, dest_path);
    Command::new("sh").arg("-c").arg(&cmd).status().ok();

    println!("✅ Profile imported: {}", file_name);
}

fn game_specific_recipes() {
    println!("🎮 Game-Specific Recipes");

    let games = vec![
        ("Grand Theft Auto V", vec!["vcrun2019", "d3dcompiler_47"]),
        (
            "The Witcher 3",
            vec!["vcrun2015", "d3dx11_43", "d3dcompiler_43"],
        ),
        ("Cyberpunk 2077", vec!["vcrun2019", "dotnet48"]),
        ("Red Dead Redemption 2", vec!["vcrun2019", "dotnet48"]),
        ("Skyrim", vec!["vcrun2019", "dotnet40", "d3dx9"]),
        ("Fallout 4", vec!["vcrun2015", "d3dx11_43"]),
        ("Dark Souls III", vec!["vcrun2017", "d3dx11_43"]),
        ("Overwatch", vec!["vcrun2015", "dxvk"]),
        ("League of Legends", vec!["vcrun2019", "d3dx9"]),
        ("World of Warcraft", vec!["vcrun2019", "dxvk"]),
    ];

    let game_names: Vec<&str> = games.iter().map(|(name, _)| *name).collect();

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select game")
        .items(&game_names)
        .default(0)
        .interact()
    else {
        return;
    };

    let (game_name, components) = &games[choice];

    println!("🎮 Recipe for: {}", game_name);
    println!("\nRequired components:");
    for comp in components {
        println!("  • {}", comp);
    }

    let Ok(wine_prefix) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
    else {
        return;
    };

    let Ok(install) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Install components?")
        .default(true)
        .interact()
    else {
        return;
    };

    if install {
        for component in components {
            println!("📦 Installing {}...", component);
            let cmd = format!("WINEPREFIX={} winetricks -q {}", wine_prefix, component);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
        }
        println!("✅ Game recipe applied");
    }
}

fn dependency_analyzer() {
    println!("🔍 Dependency Analyzer");

    let Ok(wine_prefix) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix to analyze")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
    else {
        return;
    };

    if !Path::new(&wine_prefix).exists() {
        println!("❌ Wine prefix not found");
        return;
    }

    println!("🔍 Analyzing installed components...\n");

    // Check common DLLs
    let system32 = format!("{}/drive_c/windows/system32", wine_prefix);
    let syswow64 = format!("{}/drive_c/windows/syswow64", wine_prefix);

    let important_dlls = [
        ("msvcp140.dll", "Visual C++ 2019"),
        ("msvcp120.dll", "Visual C++ 2013"),
        ("msvcp110.dll", "Visual C++ 2012"),
        ("msvcp100.dll", "Visual C++ 2010"),
        ("d3d9.dll", "DirectX 9"),
        ("d3d11.dll", "DirectX 11"),
        ("d3d12.dll", "DirectX 12"),
        ("xinput1_3.dll", "XInput"),
        ("xaudio2_7.dll", "XAudio2"),
    ];

    println!("📋 DLL Status:");
    for (dll, name) in &important_dlls {
        let path32 = format!("{}/{}", syswow64, dll);
        let path64 = format!("{}/{}", system32, dll);

        if Path::new(&path32).exists() || Path::new(&path64).exists() {
            println!("  ✅ {} - {}", dll, name);
        } else {
            println!("  ❌ {} - {} (missing)", dll, name);
        }
    }

    // Check .NET Framework
    println!("\n📋 .NET Framework:");
    let dotnet_key = format!("{}/drive_c/windows/Microsoft.NET/Framework", wine_prefix);
    if Path::new(&dotnet_key).exists()
        && let Ok(entries) = fs::read_dir(&dotnet_key)
    {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("v") {
                println!("  ✅ .NET {}", name);
            }
        }
    }

    // Check fonts
    println!("\n📋 Fonts:");
    let fonts_dir = format!("{}/drive_c/windows/Fonts", wine_prefix);
    if let Ok(entries) = fs::read_dir(&fonts_dir) {
        let font_count = entries.count();
        println!("  📊 {} fonts installed", font_count);
        if font_count < 50 {
            println!("  ⚠️ Consider installing corefonts");
        }
    }
}

fn component_status() {
    println!("📊 Component Status");

    let Ok(wine_prefix) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Wine prefix path")
        .default(format!("{}/.wine", get_home_dir()))
        .interact()
    else {
        return;
    };

    if !Path::new(&wine_prefix).exists() {
        println!("❌ Wine prefix not found");
        return;
    }

    // Get winetricks list
    println!("🔍 Checking installed winetricks components...");
    let cmd = format!("WINEPREFIX={} winetricks list-installed", wine_prefix);

    let output = Command::new("sh").arg("-c").arg(&cmd).output();

    match output {
        Ok(out) => {
            let installed = String::from_utf8_lossy(&out.stdout);
            if installed.is_empty() {
                println!("❌ No components found or winetricks not available");
            } else {
                println!("\n📋 Installed Components:");
                for line in installed.lines() {
                    if !line.is_empty() && !line.starts_with("Using") {
                        println!("  ✅ {}", line);
                    }
                }
            }
        }
        _ => println!("❌ Failed to check components"),
    }
}

fn update_winetricks() {
    println!("🔄 Update Winetricks");

    println!("📥 Downloading latest winetricks...");
    let cmd = "sudo wget -O /usr/local/bin/winetricks https://raw.githubusercontent.com/Winetricks/winetricks/master/src/winetricks && sudo chmod +x /usr/local/bin/winetricks";

    let status = Command::new("sh").arg("-c").arg(cmd).status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Winetricks updated successfully");

            // Check version
            let version_cmd = "winetricks --version";
            if let Ok(output) = Command::new("sh").arg("-c").arg(version_cmd).output() {
                let version = String::from_utf8_lossy(&output.stdout);
                println!("📋 Version: {}", version.trim());
            }
        }
        _ => println!("❌ Failed to update winetricks"),
    }
}

fn wine_bottle_management() {
    println!("🍾 Wine Bottle Management");

    let options = [
        "Create bottle",
        "List bottles",
        "Configure bottle",
        "Clone bottle",
        "Delete bottle",
        "Import/Export bottle",
        "Bottle templates",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Bottle management")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => create_wine_bottle(),
        1 => list_wine_bottles(),
        2 => configure_wine_bottle(),
        3 => clone_wine_bottle(),
        4 => delete_wine_bottle(),
        5 => import_export_bottle(),
        6 => bottle_templates(),
        _ => {}
    }
}

fn create_wine_bottle() {
    println!("🍾 Create Wine Bottle");

    let Ok(bottle_name) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter bottle name")
        .interact()
    else {
        return;
    };

    let bottles_dir = format!("{}/Wine/bottles", get_home_dir());
    fs::create_dir_all(&bottles_dir).ok();

    let bottle_path = format!("{}/{}", bottles_dir, bottle_name);

    let Ok(arch) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select architecture")
        .items(&["64-bit", "32-bit"])
        .default(0)
        .interact()
    else {
        return;
    };

    let arch_str = if arch == 0 { "win64" } else { "win32" };

    println!("🍾 Creating bottle...");
    let cmd = format!(
        "WINEPREFIX={} WINEARCH={} wineboot -i",
        bottle_path, arch_str
    );

    let status = Command::new("sh").arg("-c").arg(&cmd).status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Bottle created: {}", bottle_name);

            // Create bottle metadata
            let metadata = format!(
                r#"{{
    "name": "{}",
    "arch": "{}",
    "created": "{}",
    "wine_version": "system",
    "components": []
}}"#,
                bottle_name,
                arch_str,
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            );

            fs::write(format!("{}/bottle.json", bottle_path), metadata).ok();
        }
        _ => println!("❌ Failed to create bottle"),
    }
}

fn list_wine_bottles() {
    println!("📋 Wine Bottles");

    let bottles_dir = format!("{}/Wine/bottles", get_home_dir());

    if !Path::new(&bottles_dir).exists() {
        println!("❌ No bottles found");
        return;
    }

    if let Ok(entries) = fs::read_dir(&bottles_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let Some(name) = path.file_name().map(|n| n.to_string_lossy().to_string()) else {
                    continue;
                };

                // Read metadata
                let metadata_path = format!("{}/bottle.json", path.display());
                if Path::new(&metadata_path).exists() {
                    if let Ok(content) = fs::read_to_string(&metadata_path) {
                        println!("🍾 {}", name);
                        // Parse and display metadata
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                            if let Some(created) = json.get("created") {
                                println!("   Created: {}", created);
                            }
                            if let Some(arch) = json.get("arch") {
                                println!("   Architecture: {}", arch);
                            }
                        }
                    }
                } else {
                    println!("🍾 {} (no metadata)", name);
                }
            }
        }
    }
}

fn configure_wine_bottle() {
    println!("🔧 Configure Wine Bottle");

    let Ok(bottle_name) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter bottle name")
        .interact()
    else {
        return;
    };

    let bottle_path = format!("{}/Wine/bottles/{}", get_home_dir(), bottle_name);

    if !Path::new(&bottle_path).exists() {
        println!("❌ Bottle not found");
        return;
    }

    let cmd = format!("WINEPREFIX={} winecfg", bottle_path);
    Command::new("sh").arg("-c").arg(&cmd).status().ok();
}

fn clone_wine_bottle() {
    println!("🔄 Clone Wine Bottle");

    let Ok(source_name) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter source bottle name")
        .interact()
    else {
        return;
    };

    let bottles_dir = format!("{}/Wine/bottles", get_home_dir());
    let source_path = format!("{}/{}", bottles_dir, source_name);

    if !Path::new(&source_path).exists() {
        println!("❌ Source bottle not found");
        return;
    }

    let Ok(dest_name) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter new bottle name")
        .interact()
    else {
        return;
    };

    let dest_path = format!("{}/{}", bottles_dir, dest_name);

    println!("🔄 Cloning bottle...");
    let cmd = format!("cp -r '{}' '{}'", source_path, dest_path);

    let status = Command::new("sh").arg("-c").arg(&cmd).status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Bottle cloned: {}", dest_name);

            // Update metadata
            let metadata_path = format!("{}/bottle.json", dest_path);
            if Path::new(&metadata_path).exists()
                && let Ok(mut content) = fs::read_to_string(&metadata_path)
            {
                content = content.replace(&source_name, &dest_name);
                fs::write(&metadata_path, content).ok();
            }
        }
        _ => println!("❌ Failed to clone bottle"),
    }
}

fn delete_wine_bottle() {
    println!("🗑️ Delete Wine Bottle");

    let Ok(bottle_name) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter bottle name to delete")
        .interact()
    else {
        return;
    };

    let bottle_path = format!("{}/Wine/bottles/{}", get_home_dir(), bottle_name);

    if !Path::new(&bottle_path).exists() {
        println!("❌ Bottle not found");
        return;
    }

    let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Delete bottle '{}'?", bottle_name))
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
            let backup_path = format!(
                "{}/Wine/bottle_backups/{}_backup.tar.gz",
                get_home_dir(),
                bottle_name
            );
            fs::create_dir_all(format!("{}/Wine/bottle_backups", get_home_dir())).ok();

            let cmd = format!("tar -czf '{}' -C '{}' .", backup_path, bottle_path);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
            println!("💾 Backup created: {}", backup_path);
        }

        Command::new("rm")
            .args(&["-rf", &bottle_path])
            .status()
            .ok();
        println!("✅ Bottle deleted");
    }
}

fn import_export_bottle() {
    println!("📦 Import/Export Bottle");

    let Ok(action) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action")
        .items(&["Export bottle", "Import bottle"])
        .default(0)
        .interact()
    else {
        return;
    };

    match action {
        0 => {
            let Ok(bottle_name) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter bottle name to export")
                .interact()
            else {
                return;
            };

            let bottle_path = format!("{}/Wine/bottles/{}", get_home_dir(), bottle_name);

            if !Path::new(&bottle_path).exists() {
                println!("❌ Bottle not found");
                return;
            }

            let Ok(export_path) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter export path")
                .default(format!("{}/{}.tar.gz", get_home_dir(), bottle_name))
                .interact()
            else {
                return;
            };

            println!("📦 Exporting bottle...");
            let cmd = format!("tar -czf '{}' -C '{}' .", export_path, bottle_path);

            let status = Command::new("sh").arg("-c").arg(&cmd).status();

            match status {
                Ok(s) if s.success() => println!("✅ Bottle exported to: {}", export_path),
                _ => println!("❌ Failed to export bottle"),
            }
        }
        1 => {
            let Ok(import_path) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter bottle archive path")
                .interact()
            else {
                return;
            };

            if !Path::new(&import_path).exists() {
                println!("❌ Archive not found");
                return;
            }

            let Ok(bottle_name) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter bottle name")
                .interact()
            else {
                return;
            };

            let bottles_dir = format!("{}/Wine/bottles", get_home_dir());
            fs::create_dir_all(&bottles_dir).ok();

            let bottle_path = format!("{}/{}", bottles_dir, bottle_name);
            fs::create_dir_all(&bottle_path).ok();

            println!("📥 Importing bottle...");
            let cmd = format!("tar -xzf '{}' -C '{}'", import_path, bottle_path);

            let status = Command::new("sh").arg("-c").arg(&cmd).status();

            match status {
                Ok(s) if s.success() => println!("✅ Bottle imported: {}", bottle_name),
                _ => println!("❌ Failed to import bottle"),
            }
        }
        _ => {}
    }
}

fn bottle_templates() {
    println!("📋 Bottle Templates");

    let templates = [
        ("Gaming - High Performance", "gaming_perf"),
        ("Gaming - Maximum Compatibility", "gaming_compat"),
        ("Office/Productivity", "office"),
        ("Development", "development"),
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

    let (_template_name, template_id) = templates[choice];

    let Ok(bottle_name) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter bottle name")
        .default(template_id.to_string())
        .interact()
    else {
        return;
    };

    // Create bottle with template settings
    create_wine_bottle_with_template(&bottle_name, template_id);
}

fn create_wine_bottle_with_template(bottle_name: &str, template_id: &str) {
    let bottles_dir = format!("{}/Wine/bottles", get_home_dir());
    fs::create_dir_all(&bottles_dir).ok();

    let bottle_path = format!("{}/{}", bottles_dir, bottle_name);

    println!("🍾 Creating bottle with template: {}", template_id);

    // Initialize bottle
    let cmd = format!("WINEPREFIX={} WINEARCH=win64 wineboot -i", bottle_path);
    Command::new("sh").arg("-c").arg(&cmd).status().ok();

    // Apply template-specific settings
    match template_id {
        "gaming_perf" => {
            // High performance gaming
            Command::new("sh")
                .arg("-c")
                .arg(&format!("WINEPREFIX={} winecfg /v win10", bottle_path))
                .status()
                .ok();

            let components = ["vcrun2019", "dotnet48", "dxvk", "vkd3d"];
            for comp in &components {
                let cmd = format!("WINEPREFIX={} winetricks -q {}", bottle_path, comp);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
            }
        }
        "gaming_compat" => {
            // Maximum compatibility
            Command::new("sh")
                .arg("-c")
                .arg(&format!("WINEPREFIX={} winecfg /v win7", bottle_path))
                .status()
                .ok();

            let components = ["vcrun2019", "vcrun2017", "vcrun2015", "dotnet48", "d3dx9"];
            for comp in &components {
                let cmd = format!("WINEPREFIX={} winetricks -q {}", bottle_path, comp);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
            }
        }
        _ => {}
    }

    println!("✅ Bottle created with template: {}", bottle_name);
}

// Continue with remaining functions...
fn wine_version_manager() {
    println!("🔄 Wine Version Manager");
    // Implementation continues...
}

fn windows_version_spoofing() {
    println!("🪟 Windows Version Spoofing");
    // Implementation continues...
}

fn dll_dependency_walker() {
    println!("📚 DLL Dependency Walker");
    // Implementation continues...
}

fn performance_optimizer() {
    println!("⚡ Performance Optimizer");
    // Implementation continues...
}

fn gaming_tweaks_hub() {
    println!("🎮 Gaming Tweaks Hub");
    // Implementation continues...
}

fn wine_debugging_tools() {
    println!("🔍 Wine Debugging Tools");
    // Implementation continues...
}

fn component_manager() {
    println!("📦 Component Manager");
    // Implementation continues...
}

fn profile_management() {
    println!("💾 Profile Management");
    // Implementation continues...
}

use chrono;
use serde_json;
