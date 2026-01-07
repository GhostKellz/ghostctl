//! NVIDIA DLSS Management Module
//!
//! Provides comprehensive DLSS (Deep Learning Super Sampling) management for Linux gaming:
//! - DLSS version detection across system, Proton, and Wine prefixes
//! - GE-Proton / Proton-cachyos DLSS configuration (PROTON_DLSS_UPGRADE, PROTON_DLSS_INDICATOR)
//! - DLSS preset recommendations and configuration
//! - DXVK-NVAPI integration for DLSS
//!
//! Note: This module helps configure DLSS but does NOT redistribute NVIDIA libraries.
//! DLSS libraries are subject to NVIDIA's proprietary license and should be obtained
//! through official channels (game installations, NVIDIA SDK, or Proton's automatic
//! download via PROTON_DLSS_UPGRADE).

use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// DLSS library information
#[derive(Debug, Clone)]
pub struct DlssLibrary {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
    pub lib_type: DlssLibType,
}

/// Types of DLSS libraries
#[derive(Debug, Clone, PartialEq)]
pub enum DlssLibType {
    SuperResolution,   // libnvidia-ngx-dlss.so / nvngx_dlss.dll
    FrameGeneration,   // libnvidia-ngx-dlssg.so / nvngx_dlssg.dll
    RayReconstruction, // libnvidia-ngx-dlssd.so / nvngx_dlssd.dll
}

impl std::fmt::Display for DlssLibType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DlssLibType::SuperResolution => write!(f, "Super Resolution (SR)"),
            DlssLibType::FrameGeneration => write!(f, "Frame Generation (FG)"),
            DlssLibType::RayReconstruction => write!(f, "Ray Reconstruction (RR)"),
        }
    }
}

/// DLSS Render Presets (from SDK 310.5.0)
#[derive(Debug, Clone, Copy)]
pub enum DlssPreset {
    Default, // Auto-select based on mode
    J,       // Less ghosting, more flickering
    K,       // Transformer-based, best quality (DLAA/Balanced/Quality default)
    L,       // Ultra Performance default
    M,       // Performance default
}

impl std::fmt::Display for DlssPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DlssPreset::Default => write!(f, "Default (Auto)"),
            DlssPreset::J => write!(f, "Preset J (Less ghosting, more flicker)"),
            DlssPreset::K => write!(f, "Preset K (Transformer, best quality)"),
            DlssPreset::L => write!(f, "Preset L (Ultra Performance default)"),
            DlssPreset::M => write!(f, "Preset M (Performance default)"),
        }
    }
}

/// Main DLSS menu
pub fn dlss_menu() {
    loop {
        println!("\n🟢 NVIDIA DLSS Management");
        println!("════════════════════════════════════════\n");

        let options = [
            "📊 DLSS Status & Version Check",
            "⬆️  Upgrade DLSS Libraries",
            "🎮 GE-Proton DLSS Configuration",
            "🎯 DLSS Preset Guide & Configuration",
            "🔧 DXVK-NVAPI DLSS Settings",
            "📋 DLSS Compatibility Check",
            "⬅️  Back",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("DLSS Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => dlss_status(),
            1 => dlss_upgrade_menu(),
            2 => ge_proton_dlss_config(),
            3 => dlss_preset_guide(),
            4 => dxvk_nvapi_dlss_settings(),
            5 => dlss_compatibility_check(),
            _ => break,
        }
    }
}

/// Check DLSS status across the system
pub fn dlss_status() {
    println!("\n📊 DLSS Status Check");
    println!("════════════════════════════════════════\n");

    // Check GPU compatibility
    println!("🎮 GPU Compatibility:");
    check_gpu_dlss_support();

    // Check system DLSS libraries
    println!("\n📦 System DLSS Libraries:");
    let system_libs = find_system_dlss_libraries();
    if system_libs.is_empty() {
        println!("  ⚠️  No system-wide DLSS libraries found");
    } else {
        for lib in &system_libs {
            println!("  ✅ {} v{}", lib.lib_type, lib.version);
            println!("     Path: {}", lib.path.display());
        }
    }

    // Check Proton DLSS
    println!("\n🚀 Proton DLSS Libraries:");
    let proton_libs = find_proton_dlss_libraries();
    if proton_libs.is_empty() {
        println!("  ⚠️  No Proton DLSS libraries found");
    } else {
        for (proton_ver, libs) in &proton_libs {
            println!("  📁 {}:", proton_ver);
            for lib in libs {
                println!("     ✅ {} v{}", lib.lib_type, lib.version);
            }
        }
    }

    // Check Wine prefixes
    println!("\n🍷 Wine Prefix DLSS Libraries:");
    let wine_libs = find_wine_prefix_dlss_libraries();
    if wine_libs.is_empty() {
        println!("  ⚠️  No Wine prefix DLSS libraries found");
    } else {
        for (prefix, libs) in &wine_libs {
            println!("  📁 {}:", prefix);
            for lib in libs {
                println!("     ✅ {} v{}", lib.lib_type, lib.version);
            }
        }
    }

    // Show latest available version
    println!("\n📌 Latest DLSS SDK Version: 310.5.0");
    println!("   Features: Gen2 Transformer Model, Presets L & M");

    println!("\nPress Enter to continue...");
    let _ = std::io::stdin().read_line(&mut String::new());
}

/// Check GPU DLSS support
fn check_gpu_dlss_support() {
    let output = Command::new("nvidia-smi")
        .args(["--query-gpu=name,compute_cap", "--format=csv,noheader"])
        .output();

    match output {
        Ok(out) => {
            let info = String::from_utf8_lossy(&out.stdout);
            for line in info.lines() {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 2 {
                    let gpu_name = parts[0].trim();
                    let compute_cap = parts[1].trim();

                    // DLSS requires Tensor Cores (RTX series, compute capability 7.5+)
                    let compute_ver: f32 = compute_cap.parse().unwrap_or(0.0);

                    if compute_ver >= 7.5 {
                        println!(
                            "  ✅ {} (Compute {}) - DLSS Supported",
                            gpu_name, compute_cap
                        );

                        // Check for DLSS 3 Frame Generation support (RTX 40 series, 8.9+)
                        if compute_ver >= 8.9 {
                            println!("     └─ DLSS 3 Frame Generation: ✅ Supported");
                        } else {
                            println!("     └─ DLSS 3 Frame Generation: ❌ RTX 40 series required");
                        }
                    } else if compute_ver >= 7.0 {
                        println!(
                            "  ⚠️  {} (Compute {}) - Limited DLSS Support",
                            gpu_name, compute_cap
                        );
                    } else {
                        println!(
                            "  ❌ {} (Compute {}) - No DLSS Support",
                            gpu_name, compute_cap
                        );
                    }
                }
            }
        }
        Err(_) => println!("  ❌ Could not detect NVIDIA GPU (nvidia-smi failed)"),
    }
}

/// Find DLSS libraries in system paths
fn find_system_dlss_libraries() -> Vec<DlssLibrary> {
    let mut libs = Vec::new();

    let search_paths = [
        "/usr/lib",
        "/usr/lib64",
        "/usr/lib/x86_64-linux-gnu",
        "/opt/nvidia",
    ];

    for path in &search_paths {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let filename = entry.file_name().to_string_lossy().to_string();
                if let Some(lib) = parse_dlss_library(&entry.path(), &filename) {
                    libs.push(lib);
                }
            }
        }
    }

    libs
}

/// Find DLSS libraries in Proton installations
fn find_proton_dlss_libraries() -> HashMap<String, Vec<DlssLibrary>> {
    let mut proton_libs = HashMap::new();
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());

    let proton_paths = [
        format!("{}/.steam/steam/steamapps/common", home),
        format!("{}/.local/share/Steam/steamapps/common", home),
    ];

    for base_path in &proton_paths {
        if let Ok(entries) = fs::read_dir(base_path) {
            for entry in entries.flatten() {
                let dirname = entry.file_name().to_string_lossy().to_string();
                if dirname.contains("Proton") || dirname.contains("GE-Proton") {
                    let files_path = entry.path().join("files/lib64/wine/nvngx");
                    let files_path2 = entry.path().join("files/lib/wine/nvngx");

                    let mut libs = Vec::new();
                    for nvngx_path in [&files_path, &files_path2] {
                        if let Ok(files) = fs::read_dir(nvngx_path) {
                            for file in files.flatten() {
                                let filename = file.file_name().to_string_lossy().to_string();
                                if let Some(lib) = parse_dlss_library(&file.path(), &filename) {
                                    libs.push(lib);
                                }
                            }
                        }
                    }

                    if !libs.is_empty() {
                        proton_libs.insert(dirname, libs);
                    }
                }
            }
        }
    }

    proton_libs
}

/// Find DLSS libraries in Wine prefixes
fn find_wine_prefix_dlss_libraries() -> HashMap<String, Vec<DlssLibrary>> {
    let mut prefix_libs = HashMap::new();
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());

    // Check common Wine prefix locations
    let prefix_locations = [
        format!("{}/.wine", home),
        format!("{}/.local/share/lutris/runners/wine", home),
        format!("{}/.steam/steam/steamapps/compatdata", home),
    ];

    for location in &prefix_locations {
        if Path::new(location).exists() {
            // Search for nvngx_dlss.dll in the prefix
            let output = Command::new("find")
                .args([location, "-name", "nvngx_dlss*.dll", "-type", "f"])
                .output();

            if let Ok(out) = output {
                let paths = String::from_utf8_lossy(&out.stdout);
                for path_str in paths.lines() {
                    let path = PathBuf::from(path_str);
                    let filename = path
                        .file_name()
                        .map(|f| f.to_string_lossy().to_string())
                        .unwrap_or_default();

                    if let Some(lib) = parse_dlss_library(&path, &filename) {
                        let prefix_name = path
                            .ancestors()
                            .find(|p| p.ends_with("drive_c"))
                            .and_then(|p| p.parent())
                            .map(|p| {
                                p.file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string()
                            })
                            .unwrap_or_else(|| "Unknown".to_string());

                        prefix_libs
                            .entry(prefix_name)
                            .or_insert_with(Vec::new)
                            .push(lib);
                    }
                }
            }
        }
    }

    prefix_libs
}

/// Parse DLSS library from filename
fn parse_dlss_library(path: &Path, filename: &str) -> Option<DlssLibrary> {
    let lib_type = if filename.contains("dlssg") {
        Some(DlssLibType::FrameGeneration)
    } else if filename.contains("dlssd") {
        Some(DlssLibType::RayReconstruction)
    } else if filename.contains("dlss") {
        Some(DlssLibType::SuperResolution)
    } else {
        None
    };

    lib_type.map(|lt| {
        // Extract version from filename (e.g., libnvidia-ngx-dlss.so.310.5.0 or nvngx_dlss.dll)
        let version = extract_dlss_version(path, filename);

        DlssLibrary {
            name: filename.to_string(),
            version,
            path: path.to_path_buf(),
            lib_type: lt,
        }
    })
}

/// Extract version from DLSS library
fn extract_dlss_version(path: &Path, filename: &str) -> String {
    // Try to extract from filename first
    if filename.contains(".so.") {
        // Linux: libnvidia-ngx-dlss.so.310.5.0
        if let Some(version) = filename.split(".so.").nth(1) {
            return version.to_string();
        }
    }

    // Try to get version from file properties or PE header for DLLs
    if filename.ends_with(".dll") {
        // Use exiftool or similar if available
        let output = Command::new("exiftool")
            .args(["-ProductVersion", path.to_str().unwrap_or("")])
            .output();

        if let Ok(out) = output {
            let info = String::from_utf8_lossy(&out.stdout);
            if let Some(version) = info.lines().next()
                && let Some(v) = version.split(':').nth(1)
            {
                return v.trim().to_string();
            }
        }
    }

    "Unknown".to_string()
}

/// DLSS upgrade menu
fn dlss_upgrade_menu() {
    println!("\n⬆️  DLSS Upgrade Options");
    println!("════════════════════════════════════════\n");

    let options = [
        "🚀 Configure PROTON_DLSS_UPGRADE (Recommended)",
        "📋 DLSS Library Installation Guide",
        "📊 Check Available DLSS Versions",
        "🔍 Detect Installed DLSS Versions",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("DLSS Upgrade Options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => configure_proton_dlss_upgrade(),
        1 => install_dlss_to_prefix(),
        2 => show_available_dlss_versions(),
        3 => dlss_status(),
        _ => {}
    }
}

/// Configure PROTON_DLSS_UPGRADE for automatic DLSS updates
fn configure_proton_dlss_upgrade() {
    println!("\n🚀 DLSS Upgrade Configuration");
    println!("════════════════════════════════════════\n");

    println!("PROTON_DLSS_UPGRADE automatically downloads and installs newer DLSS DLLs.");
    println!("Works with: GE-Proton, Proton-cachyos, and similar custom builds.\n");

    println!("💡 TIP: Use PROTON_DLSS_UPGRADE=1 to force the latest SDK (currently 310.5.0)");
    println!("   This enables the new Gen2 Transformer model on ANY game with DLSS support!\n");

    let options = [
        "1 - Force Latest (Recommended - currently 310.5.0)",
        "310.5.0 - Explicit latest version",
        "310.2.1 - GE-Proton previous default",
        "Custom version",
        "Disable DLSS upgrade",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select DLSS upgrade mode")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    let version = match choice {
        0 => "1".to_string(), // Force latest
        1 => "310.5.0".to_string(),
        2 => "310.2.1".to_string(),
        3 => Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter DLSS version (e.g., 310.5.0)")
            .interact()
            .unwrap(),
        _ => {
            println!("✅ DLSS upgrade disabled");
            return;
        }
    };

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║  DLSS Upgrade Configuration                                   ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("📝 Add to your Steam launch options:");
    println!("   PROTON_DLSS_UPGRADE={} %command%\n", version);

    println!("🔍 To verify the upgrade is working, also add the indicator:");
    println!(
        "   PROTON_DLSS_UPGRADE={} PROTON_DLSS_INDICATOR=1 %command%\n",
        version
    );

    println!("   The indicator will show version 310.5.0 in the bottom-left corner.\n");

    println!("🐧 Supported Proton builds:");
    println!("   • GE-Proton 10-26+");
    println!("   • Proton-cachyos");
    println!("   • Other custom Proton builds with DLSS support");

    // Ask if user wants to create an environment file
    let create_env = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Create/update ~/.config/environment.d/dlss.conf?")
        .default(false)
        .interact()
        .unwrap();

    if create_env {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
        let env_dir = format!("{}/.config/environment.d", home);
        let env_file = format!("{}/dlss.conf", env_dir);

        fs::create_dir_all(&env_dir).ok();

        let content = format!(
            "# DLSS Configuration for GE-Proton\n\
             PROTON_DLSS_UPGRADE=\"{}\"\n\
             PROTON_DLSS_INDICATOR=1\n",
            version
        );

        match fs::write(&env_file, &content) {
            Ok(_) => println!("✅ Environment file created: {}", env_file),
            Err(e) => println!("❌ Failed to create environment file: {}", e),
        }
    }
}

/// Guide for getting DLSS libraries properly
fn install_dlss_to_prefix() {
    println!("\n📦 DLSS Library Installation Guide");
    println!("════════════════════════════════════════\n");

    println!("⚠️  DLSS libraries are subject to NVIDIA's proprietary license.");
    println!("   This tool helps configure DLSS but does not redistribute libraries.\n");

    println!("✅ Recommended Methods to Get DLSS:\n");

    println!("1️⃣  Use PROTON_DLSS_UPGRADE (Easiest - Recommended)");
    println!("   ─────────────────────────────────────────────────");
    println!("   Set: PROTON_DLSS_UPGRADE=1");
    println!("   Proton will automatically download the latest DLSS from NVIDIA.\n");

    println!("2️⃣  Copy from a Game Installation");
    println!("   ─────────────────────────────────────────────────");
    println!("   Many games include DLSS libraries in their installation.");
    println!("   Look for: nvngx_dlss.dll in the game's directory.\n");

    println!("3️⃣  NVIDIA NGX SDK (For Developers)");
    println!("   ─────────────────────────────────────────────────");
    println!("   Download from: https://developer.nvidia.com/rtx/dlss");
    println!("   SDK includes libraries for development and testing.\n");

    println!("📍 Common DLSS Library Locations:\n");
    println!("   • Wine prefix: <prefix>/drive_c/windows/system32/nvngx_dlss.dll");
    println!("   • Game folder: <game>/nvngx_dlss.dll");
    println!("   • Proton: ~/.steam/steam/steamapps/common/Proton*/files/lib64/wine/nvngx/\n");

    println!("🔧 DLSS Library Types:");
    println!("   • nvngx_dlss.dll   - Super Resolution (upscaling)");
    println!("   • nvngx_dlssg.dll  - Frame Generation (RTX 40+ only)");
    println!("   • nvngx_dlssd.dll  - Ray Reconstruction");

    println!("\nPress Enter to continue...");
    let _ = std::io::stdin().read_line(&mut String::new());
}

/// Show available DLSS versions
fn show_available_dlss_versions() {
    println!("\n📋 Available DLSS Versions");
    println!("════════════════════════════════════════\n");

    println!("🆕 Latest Versions (as of 2025):");
    println!("   • 310.5.0 - Gen2 Transformer Model");
    println!("     └─ New Presets L (Ultra Perf) and M (Perf)");
    println!("     └─ Improved quality for all modes");
    println!();
    println!("   • 310.2.1 - GE-Proton default");
    println!("   • 310.1.0 - Stable release");
    println!("   • 3.7.20  - Legacy (pre-310 versioning)");
    println!();
    println!("📦 DLSS Components:");
    println!("   • DLSS Super Resolution (SR) - Upscaling");
    println!("   • DLSS Frame Generation (FG) - RTX 40+ only");
    println!("   • DLSS Ray Reconstruction (RR) - Enhanced ray tracing");
    println!();
    println!("🔗 Download SDK: https://developer.nvidia.com/rtx/dlss");
}

/// GE-Proton DLSS configuration
fn ge_proton_dlss_config() {
    println!("\n🎮 GE-Proton DLSS Configuration");
    println!("════════════════════════════════════════\n");

    let options = [
        "🔄 Configure PROTON_DLSS_UPGRADE",
        "👁️  Enable DLSS Indicator Overlay",
        "🎯 Configure DLSS Presets (DRS Settings)",
        "📝 Generate Steam Launch Options",
        "📋 Show Current Configuration",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("GE-Proton DLSS Options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => configure_proton_dlss_upgrade(),
        1 => configure_dlss_indicator(),
        2 => configure_dlss_drs_settings(),
        3 => generate_steam_launch_options(),
        4 => show_current_dlss_config(),
        _ => {}
    }
}

/// Configure DLSS indicator overlay
fn configure_dlss_indicator() {
    println!("\n👁️  DLSS Indicator Configuration");
    println!("════════════════════════════════════════\n");

    println!("PROTON_DLSS_INDICATOR shows a DLSS overlay in the bottom-left corner.");
    println!("Useful for verifying DLSS is active in games.\n");

    let enable = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable DLSS indicator overlay?")
        .default(true)
        .interact()
        .unwrap();

    if enable {
        println!("\n📝 Add to your environment or Steam launch options:");
        println!("   PROTON_DLSS_INDICATOR=1");
        println!("\n   For Steam: PROTON_DLSS_INDICATOR=1 %command%");
    }
}

/// Configure DLSS DRS (Driver Settings)
fn configure_dlss_drs_settings() {
    println!("\n🎯 DLSS DRS Settings Configuration");
    println!("════════════════════════════════════════\n");

    println!("DXVK_NVAPI_DRS_SETTINGS configures DLSS presets via driver settings.");
    println!("PROTON_DLSS_UPGRADE automatically applies latest presets.\n");

    let presets = [
        "Auto (let PROTON_DLSS_UPGRADE handle it)",
        "Preset K - Best Quality (Transformer, DLAA/Balanced/Quality)",
        "Preset J - Less Ghosting (more flicker)",
        "Preset L - Ultra Performance Mode",
        "Preset M - Performance Mode",
        "Custom DRS string",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select DLSS preset configuration")
        .items(&presets)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("\n✅ PROTON_DLSS_UPGRADE will automatically configure optimal presets.");
        }
        1..=4 => {
            let preset_id = match choice {
                1 => "0x00707011", // Preset K
                2 => "0x00707010", // Preset J
                3 => "0x00707012", // Preset L
                4 => "0x00707013", // Preset M
                _ => "0x00707011",
            };
            println!("\n📝 Add to environment:");
            println!(
                "   DXVK_NVAPI_DRS_SETTINGS=\"DLSSHintRenderPreset={}\"",
                preset_id
            );
        }
        5 => {
            let custom = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter custom DRS settings string")
                .interact()
                .unwrap();
            println!("\n📝 Add to environment:");
            println!("   DXVK_NVAPI_DRS_SETTINGS=\"{}\"", custom);
        }
        _ => {}
    }
}

/// Generate Steam launch options for DLSS
fn generate_steam_launch_options() {
    println!("\n📝 Steam Launch Options Generator");
    println!("════════════════════════════════════════\n");

    let mut options = Vec::new();

    // DLSS Upgrade
    let enable_upgrade = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable DLSS auto-upgrade to latest (310.5.0)?")
        .default(true)
        .interact()
        .unwrap();

    if enable_upgrade {
        options.push("PROTON_DLSS_UPGRADE=1".to_string());
    }

    // DLSS Indicator
    let enable_indicator = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable DLSS indicator overlay?")
        .default(false)
        .interact()
        .unwrap();

    if enable_indicator {
        options.push("PROTON_DLSS_INDICATOR=1".to_string());
    }

    // GameMode
    let enable_gamemode = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable GameMode?")
        .default(true)
        .interact()
        .unwrap();

    if enable_gamemode {
        options.push("gamemoderun".to_string());
    }

    // MangoHud
    let enable_mangohud = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable MangoHud?")
        .default(false)
        .interact()
        .unwrap();

    if enable_mangohud {
        options.push("mangohud".to_string());
    }

    // Generate final string
    let launch_options = if options.is_empty() {
        "%command%".to_string()
    } else {
        format!("{} %command%", options.join(" "))
    };

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║ Steam Launch Options:                                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("\n{}\n", launch_options);
    println!("📋 Copy this to Steam → Game → Properties → Launch Options");
}

/// Show current DLSS configuration
fn show_current_dlss_config() {
    println!("\n📋 Current DLSS Configuration");
    println!("════════════════════════════════════════\n");

    let env_vars = [
        ("PROTON_DLSS_UPGRADE", "DLSS auto-upgrade version"),
        ("PROTON_DLSS_INDICATOR", "DLSS indicator overlay"),
        ("DXVK_NVAPI_DRS_SETTINGS", "DLSS DRS settings"),
        ("DXVK_ENABLE_NVAPI", "NVAPI enabled"),
        ("PROTON_ENABLE_NVAPI", "Proton NVAPI"),
    ];

    for (var, desc) in &env_vars {
        match std::env::var(var) {
            Ok(val) => println!("  ✅ {}: {} ({})", var, val, desc),
            Err(_) => println!("  ⚠️  {}: Not set ({})", var, desc),
        }
    }

    // Check environment.d files
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
    let env_file = format!("{}/.config/environment.d/dlss.conf", home);

    if Path::new(&env_file).exists() {
        println!("\n📄 Environment file: {}", env_file);
        if let Ok(content) = fs::read_to_string(&env_file) {
            for line in content.lines() {
                if !line.starts_with('#') && !line.is_empty() {
                    println!("   {}", line);
                }
            }
        }
    }
}

/// DLSS preset guide and recommendations
fn dlss_preset_guide() {
    println!("\n🎯 DLSS Preset Guide (SDK 310.5.0)");
    println!("════════════════════════════════════════\n");

    println!("📊 DLSS Render Presets:\n");

    println!("┌─────────────┬────────────────────────────────────────────────────┐");
    println!("│ Preset      │ Description                                        │");
    println!("├─────────────┼────────────────────────────────────────────────────┤");
    println!("│ Default     │ Auto-selects based on DLSS mode                    │");
    println!("│ Preset K    │ Gen2 Transformer - Best quality, higher perf cost  │");
    println!("│             │ Default for DLAA/Balanced/Quality modes            │");
    println!("│ Preset J    │ Less ghosting but more flickering than K           │");
    println!("│ Preset L    │ NEW - Optimized for Ultra Performance mode         │");
    println!("│ Preset M    │ NEW - Optimized for Performance mode               │");
    println!("└─────────────┴────────────────────────────────────────────────────┘");

    println!("\n🎮 Recommendations by Game Type:\n");

    println!("  • Competitive/Fast-paced: Preset J or M");
    println!("    └─ Less ghosting, better motion clarity");
    println!();
    println!("  • Single-player/Cinematic: Preset K");
    println!("    └─ Best image quality, transformer AI");
    println!();
    println!("  • 4K Ultra Performance: Preset L");
    println!("    └─ Optimized for high upscale ratios");
    println!();
    println!("  • 1440p Performance: Preset M");
    println!("    └─ Balanced for medium upscale ratios");

    println!("\n💡 Tips:");
    println!("  • PROTON_DLSS_UPGRADE=\"310.5.0\" automatically applies optimal presets");
    println!("  • Gen2 Transformer model (Preset K/L/M) requires DLSS 310+");
    println!("  • Older presets (A-F) are deprecated, use J/K/L/M instead");

    println!("\nPress Enter to continue...");
    let _ = std::io::stdin().read_line(&mut String::new());
}

/// DXVK-NVAPI DLSS settings
fn dxvk_nvapi_dlss_settings() {
    println!("\n🔧 DXVK-NVAPI DLSS Settings");
    println!("════════════════════════════════════════\n");

    let options = [
        "Enable NVAPI for DLSS",
        "Disable NVAPI",
        "Configure NVAPI driver version",
        "Set NVAPI log level",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("DXVK-NVAPI Options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("\n📝 To enable NVAPI for DLSS, add to environment:");
            println!("   DXVK_ENABLE_NVAPI=1");
            println!("   PROTON_ENABLE_NVAPI=1");
            println!("\n   For Steam: DXVK_ENABLE_NVAPI=1 PROTON_ENABLE_NVAPI=1 %command%");
        }
        1 => {
            println!("\n📝 To disable NVAPI:");
            println!("   DXVK_ENABLE_NVAPI=0");
            println!("   PROTON_ENABLE_NVAPI=0");
        }
        2 => {
            let version = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter driver version to spoof (e.g., 56538)")
                .default("56538".to_string())
                .interact()
                .unwrap();
            println!("\n📝 Add to environment:");
            println!("   DXVK_NVAPI_DRIVER_VERSION={}", version);
        }
        3 => {
            let levels = ["none", "error", "warn", "info", "debug"];
            let level = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select log level")
                .items(&levels)
                .default(0)
                .interact()
                .unwrap();
            println!("\n📝 Add to environment:");
            println!("   DXVK_NVAPI_LOG_LEVEL={}", levels[level]);
        }
        _ => {}
    }
}

/// DLSS compatibility check
fn dlss_compatibility_check() {
    println!("\n📋 DLSS Compatibility Check");
    println!("════════════════════════════════════════\n");

    // Check GPU
    println!("🎮 Checking GPU compatibility...");
    check_gpu_dlss_support();

    // Check driver version
    println!("\n🔧 Checking driver version...");
    let output = Command::new("nvidia-smi")
        .args(["--query-gpu=driver_version", "--format=csv,noheader"])
        .output();

    match output {
        Ok(out) => {
            let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let major: u32 = version
                .split('.')
                .next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);

            if major >= 545 {
                println!("  ✅ Driver {} - Full DLSS 3.5+ support", version);
            } else if major >= 525 {
                println!("  ✅ Driver {} - DLSS 3 support", version);
            } else if major >= 470 {
                println!("  ⚠️  Driver {} - DLSS 2 support only", version);
            } else {
                println!("  ❌ Driver {} - Update recommended for DLSS", version);
            }
        }
        Err(_) => println!("  ❌ Could not detect driver version"),
    }

    // Check Vulkan
    println!("\n🌋 Checking Vulkan support...");
    let vulkan = Command::new("vulkaninfo").args(["--summary"]).output();
    match vulkan {
        Ok(_) => println!("  ✅ Vulkan available"),
        Err(_) => println!("  ⚠️  Vulkan not detected"),
    }

    // Check DXVK
    println!("\n📦 Checking DXVK-NVAPI...");
    let home = std::env::var("HOME").unwrap_or_default();
    let dxvk_nvapi_paths = [
        format!(
            "{}/.local/share/Steam/steamapps/common/Proton - Experimental/files/lib64/wine/nvapi",
            home
        ),
        "/usr/share/dxvk-nvapi".to_string(),
    ];

    let mut found_nvapi = false;
    for path in &dxvk_nvapi_paths {
        if Path::new(path).exists() {
            println!("  ✅ DXVK-NVAPI found at: {}", path);
            found_nvapi = true;
            break;
        }
    }
    if !found_nvapi {
        println!("  ⚠️  DXVK-NVAPI not found in common locations");
    }

    // Check GE-Proton
    println!("\n🚀 Checking GE-Proton...");
    let ge_proton_path = format!("{}/.steam/steam/compatibilitytools.d", home);
    if let Ok(entries) = fs::read_dir(&ge_proton_path) {
        let ge_versions: Vec<String> = entries
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|n| n.contains("GE-Proton"))
            .collect();

        if ge_versions.is_empty() {
            println!("  ⚠️  No GE-Proton found (recommended for DLSS)");
        } else {
            for ver in &ge_versions {
                println!("  ✅ {}", ver);
            }
        }
    }

    println!("\n✅ Compatibility check complete!");
    println!("\nPress Enter to continue...");
    let _ = std::io::stdin().read_line(&mut String::new());
}

/// Quick DLSS status for integration
pub fn quick_dlss_status() -> String {
    let system_libs = find_system_dlss_libraries();
    let proton_libs = find_proton_dlss_libraries();

    let mut status = String::new();

    if !system_libs.is_empty()
        && let Some(lib) = system_libs.first()
    {
        status.push_str(&format!("System: v{}", lib.version));
    }

    if !proton_libs.is_empty() {
        if !status.is_empty() {
            status.push_str(" | ");
        }
        status.push_str(&format!("Proton: {} installs", proton_libs.len()));
    }

    if status.is_empty() {
        "Not detected".to_string()
    } else {
        status
    }
}
