use dialoguer::{Confirm, MultiSelect, Select, theme::ColorfulTheme};
use reqwest::blocking::get;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use tempfile::NamedTempFile;
use which::which;

/// Validate tool/binary name to prevent shell injection and path traversal
fn validate_tool_name(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("Tool name cannot be empty");
    }
    if name.len() > 50 {
        return Err("Tool name too long");
    }
    // Only allow alphanumeric, hyphen, underscore
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err("Tool name contains invalid characters");
    }
    if name.starts_with('-') {
        return Err("Tool name cannot start with hyphen");
    }
    Ok(())
}

/// Validate repository path (user/repo format)
fn validate_repo_path(repo: &str) -> Result<(), &'static str> {
    if repo.is_empty() {
        return Err("Repository path cannot be empty");
    }
    if repo.len() > 100 {
        return Err("Repository path too long");
    }
    // Must be in user/repo format
    let parts: Vec<&str> = repo.split('/').collect();
    if parts.len() != 2 {
        return Err("Repository must be in 'user/repo' format");
    }
    // Validate each part
    for part in parts {
        if part.is_empty() {
            return Err("Repository user/name cannot be empty");
        }
        if !part
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err("Repository contains invalid characters");
        }
        if part.starts_with('-') || part.ends_with('-') {
            return Err("Repository parts cannot start/end with hyphen");
        }
    }
    Ok(())
}

// Ghost Ecosystem Tools - Real planned tools
const GHOST_ECOSYSTEM: &[(&str, &str, &str, &str)] = &[
    // Development Tools
    (
        "zion",
        "ghostkellz/zion",
        "Zig meta-programming and build tool",
        "dev",
    ),
    (
        "oxygen",
        "ghostkellz/oxygen",
        "Advanced Rust development toolkit",
        "dev",
    ),
    (
        "sigil",
        "ghostkellz/sigil",
        "Linux development scripting framework",
        "dev",
    ),
    (
        "zmake",
        "ghostkellz/zmake",
        "Modern makepkg replacement tool",
        "dev",
    ),
    // Package Management & AUR
    (
        "reaper",
        "ghostkellz/reaper",
        "Next-gen AUR package manager (ghostbrew successor)",
        "package",
    ),
    (
        "zaur",
        "ghostkellz/zaur",
        "Zig-based AUR helper and aurutils alternative",
        "package",
    ),
    (
        "ghostview",
        "ghostkellz/ghostview",
        "GUI AUR search and package browser",
        "package",
    ),
    // System Tools
    (
        "ghostctl",
        "ghostkellz/ghostctl",
        "System administration toolkit (this tool)",
        "system",
    ),
    (
        "nvcontrol",
        "ghostkellz/nvcontrol",
        "NVIDIA control CLI and GUI suite",
        "system",
    ),
    (
        "phantomlink",
        "ghostkellz/phantomlink",
        "XLR-style audio control (Wavelink clone)",
        "system",
    ),
    // Network & Security
    // AI & Automation
    (
        "jarvis",
        "ghostkellz/jarvis",
        "AI-powered development assistant",
        "ai",
    ),
];

pub fn ghost_ecosystem_menu() {
    loop {
        let categories = [
            "All Tools",
            "Development",
            "Package Management",
            "System Tools",
            "Network & Security",
            "AI & Automation",
        ];

        let category_idx = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("👻 Ghost Ecosystem - Select Category")
            .items(&categories)
            .default(0)
            .interact()
        {
            Ok(c) => c,
            Err(_) => return,
        };

        if category_idx == 0 {
            show_all_tools();
        } else {
            let category_filter = match category_idx {
                1 => "dev",
                2 => "package",
                3 => "system",
                4 => "network",
                5 => "ai",
                _ => continue,
            };
            show_category_tools(category_filter, categories[category_idx]);
        }

        break; // Exit after selection for now
    }
}

fn show_all_tools() {
    loop {
        let mut menu_items: Vec<String> = GHOST_ECOSYSTEM
            .iter()
            .map(|(name, _, desc, category)| {
                let status = if which(name).is_ok() { "✅" } else { "📦" };
                let cat_emoji = match *category {
                    "dev" => "🛠️",
                    "package" => "📦",
                    "system" => "⚙️",
                    "network" => "🌐",
                    "ai" => "🤖",
                    _ => "🔧",
                };
                format!(
                    "{} {} {} {} - {}",
                    status,
                    cat_emoji,
                    name,
                    if which(name).is_ok() {
                        "[INSTALLED]"
                    } else {
                        "[available]"
                    },
                    desc
                )
            })
            .collect();

        menu_items.extend_from_slice(&[
            "🚀 Batch Install Available".to_string(),
            "🔄 Update All Installed".to_string(),
            "🗑️  Batch Uninstall All".to_string(),
            "📊 Show Statistics".to_string(),
            "⬅️  Back".to_string(),
        ]);

        let idx = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("👻 Ghost Ecosystem - All Tools")
            .items(&menu_items)
            .default(0)
            .interact()
        {
            Ok(i) => i,
            Err(_) => break,
        };

        if idx == menu_items.len() - 1 {
            break; // Back
        } else if idx == menu_items.len() - 2 {
            show_ecosystem_stats();
        } else if idx == menu_items.len() - 3 {
            batch_uninstall_ghost_tools();
        } else if idx == menu_items.len() - 4 {
            update_all_ghost_tools();
        } else if idx == menu_items.len() - 5 {
            batch_install_ghost_tools();
        } else {
            // Individual tool
            let (name, repo, _, _) = GHOST_ECOSYSTEM[idx];
            individual_tool_menu(name, repo);
        }
    }
}

fn show_category_tools(category_filter: &str, category_name: &str) {
    let filtered_tools: Vec<_> = GHOST_ECOSYSTEM
        .iter()
        .filter(|(_, _, _, cat)| *cat == category_filter)
        .collect();

    if filtered_tools.is_empty() {
        println!("No tools in {} category", category_name);
        return;
    }

    let mut menu_items: Vec<String> = filtered_tools
        .iter()
        .map(|(name, _, desc, _)| {
            let status = if which(name).is_ok() {
                "✅ [INSTALLED]"
            } else {
                "📦 [available]"
            };
            format!("{} {} - {}", status, name, desc)
        })
        .collect();

    menu_items.push("⬅️  Back".to_string());

    let idx = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("👻 {} Tools", category_name))
        .items(&menu_items)
        .default(0)
        .interact()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    if idx == menu_items.len() - 1 {
        return; // Back
    }

    let (name, repo, _, _) = filtered_tools[idx];
    individual_tool_menu(name, repo);
}

fn batch_install_ghost_tools() {
    let available_tools: Vec<(&str, &str)> = GHOST_ECOSYSTEM
        .iter()
        .filter_map(|(name, repo, _, _)| {
            if which(name).is_err() {
                Some((*name, *repo))
            } else {
                None
            }
        })
        .collect();

    if available_tools.is_empty() {
        println!("✅ All Ghost tools are already installed!");
        return;
    }

    let tool_names: Vec<&str> = available_tools.iter().map(|(name, _)| *name).collect();

    let selections = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select tools to install (Space to select, Enter to confirm)")
        .items(&tool_names)
        .interact()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    if selections.is_empty() {
        println!("❌ No tools selected");
        return;
    }

    println!("🚀 Installing {} selected tools...", selections.len());

    for selection in selections {
        let (tool_name, repo) = available_tools[selection];
        install_ghost_tool(tool_name, repo);
    }

    println!("✅ Batch installation complete!");
}

fn batch_uninstall_ghost_tools() {
    let installed_tools: Vec<&str> = GHOST_ECOSYSTEM
        .iter()
        .filter_map(|(name, _, _, _)| {
            if which(name).is_ok() && *name != "ghostctl" {
                // Don't uninstall ourselves
                Some(*name)
            } else {
                None
            }
        })
        .collect();

    if installed_tools.is_empty() {
        println!("📦 No Ghost tools to uninstall (ghostctl excluded)");
        return;
    }

    let selections = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("⚠️  Select tools to UNINSTALL (Space to select, Enter to confirm)")
        .items(&installed_tools)
        .interact()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    if selections.is_empty() {
        println!("❌ No tools selected for removal");
        return;
    }

    // Confirmation
    let confirm = match dialoguer::Confirm::new()
        .with_prompt(format!("⚠️  Really uninstall {} tools?", selections.len()))
        .default(false)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    if !confirm {
        println!("❌ Uninstall cancelled");
        return;
    }

    println!("🗑️  Uninstalling {} selected tools...", selections.len());

    for selection in selections {
        let tool_name = installed_tools[selection];
        uninstall_ghost_tool(tool_name);
    }

    println!("✅ Batch uninstall complete!");
}

fn update_all_ghost_tools() {
    let installed_tools: Vec<(&str, &str)> = GHOST_ECOSYSTEM
        .iter()
        .filter_map(|(name, repo, _, _)| {
            if which(name).is_ok() {
                Some((*name, *repo))
            } else {
                None
            }
        })
        .collect();

    if installed_tools.is_empty() {
        println!("� No Ghost tools are currently installed");
        return;
    }

    println!(
        "�🔄 Updating {} installed Ghost tools...",
        installed_tools.len()
    );

    for (name, repo) in installed_tools {
        println!("🔄 Updating {}...", name);
        install_ghost_tool(name, repo);
    }

    println!("✅ All Ghost tools updated!");
}

fn show_ecosystem_stats() {
    let total_tools = GHOST_ECOSYSTEM.len();
    let installed_count = GHOST_ECOSYSTEM
        .iter()
        .filter(|(name, _, _, _)| which(name).is_ok())
        .count();

    let mut category_stats = std::collections::HashMap::new();
    let mut category_installed = std::collections::HashMap::new();

    for (name, _, _, category) in GHOST_ECOSYSTEM {
        *category_stats.entry(*category).or_insert(0) += 1;
        if which(name).is_ok() {
            *category_installed.entry(*category).or_insert(0) += 1;
        }
    }

    println!("\n📊 Ghost Ecosystem Statistics");
    println!("═══════════════════════════════");
    println!("📦 Total Tools: {}", total_tools);
    println!("✅ Installed: {}", installed_count);
    println!("📥 Available: {}", total_tools - installed_count);
    println!(
        "📈 Coverage: {:.1}%",
        (installed_count as f32 / total_tools as f32) * 100.0
    );

    println!("\n📂 By Category:");
    println!("───────────────");
    for (category, total) in category_stats {
        let installed = category_installed.get(category).unwrap_or(&0);
        let cat_name = match category {
            "dev" => "🛠️  Development",
            "package" => "📦 Package Mgmt",
            "system" => "⚙️  System Tools",
            "network" => "🌐 Network & Sec",
            "ai" => "🤖 AI & Automation",
            _ => "🔧 Other",
        };
        println!("  {}: {}/{}", cat_name, installed, total);
    }
}

fn individual_tool_menu(name: &str, repo: &str) {
    let is_installed = which(name).is_ok();
    let options = if is_installed {
        vec!["Update", "Uninstall", "View Info", "Back"]
    } else {
        vec!["Install", "View Info", "Back"]
    };

    let action = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("{}: Choose action", name))
        .items(&options)
        .default(0)
        .interact()
    {
        Ok(a) => a,
        Err(_) => return,
    };

    match options[action] {
        "Install" | "Update" => install_ghost_tool(name, repo),
        "Uninstall" => uninstall_ghost_tool(name),
        "View Info" => show_tool_info(name, repo),
        _ => {}
    }
}

fn install_ghost_tool(name: &str, repo: &str) {
    // Validate inputs to prevent shell injection
    if let Err(e) = validate_tool_name(name) {
        eprintln!("Invalid tool name '{}': {}", name, e);
        return;
    }
    if let Err(e) = validate_repo_path(repo) {
        eprintln!("Invalid repository '{}': {}", repo, e);
        return;
    }

    println!("Installing/Updating {}...", name);

    let tmp_dir = format!("/tmp/ghostctl-install-{}", name);
    let repo_url = format!("https://github.com/{}.git", repo);
    let install_path = format!("/usr/bin/{}", name);

    // Clean up any previous attempt
    if std::path::Path::new(&tmp_dir).exists()
        && let Err(e) = std::fs::remove_dir_all(&tmp_dir)
    {
        eprintln!("Warning: Failed to clean up {}: {}", tmp_dir, e);
    }

    // Clone repository - using direct command args, no shell interpolation
    println!("Cloning repository...");
    match Command::new("git")
        .args(["clone", "--depth", "1", &repo_url, &tmp_dir])
        .status()
    {
        Ok(status) if status.success() => {
            println!("Repository cloned successfully");
        }
        Ok(_) => {
            eprintln!("Repository not found or not yet created: {}", repo);
            eprintln!(
                "This tool may be in development. Check https://github.com/{}",
                repo
            );
            return;
        }
        Err(e) => {
            eprintln!("Failed to clone repository: {}", e);
            return;
        }
    }

    // Determine build type based on tool name (from trusted GHOST_ECOSYSTEM list)
    let is_zig_tool = matches!(name, "zion" | "zaur" | "zigdns");
    let is_gui_tool = matches!(name, "ghostview" | "phantomlink" | "nvcontrol");

    // Build the tool
    println!("Building {}...", name);
    let build_success = if is_zig_tool {
        build_zig_tool(&tmp_dir, name)
    } else if is_gui_tool {
        build_rust_tool(&tmp_dir, name, true)
    } else {
        build_rust_tool(&tmp_dir, name, false)
    };

    if !build_success {
        eprintln!("Build failed for {}", name);
        return;
    }

    // Install the binary
    let binary_path = if is_zig_tool {
        format!("{}/zig-out/bin/{}", tmp_dir, name)
    } else {
        format!("{}/target/release/{}", tmp_dir, name)
    };

    println!("Installing binary...");
    match Command::new("sudo")
        .args(["install", "-Dm755", &binary_path, &install_path])
        .status()
    {
        Ok(status) if status.success() => {
            println!("{} installed successfully!", name);
            println!("Run '{} --help' to get started", name);
        }
        Ok(status) => {
            eprintln!("Failed to install binary (exit code: {:?})", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run install command: {}", e);
        }
    }

    // Install .desktop file for GUI tools
    if is_gui_tool {
        let desktop_file = format!("{}/{}.desktop", tmp_dir, name);
        if std::path::Path::new(&desktop_file).exists() {
            let desktop_dest = format!("/usr/share/applications/{}.desktop", name);
            if let Err(e) = Command::new("sudo")
                .args(["install", "-Dm644", &desktop_file, &desktop_dest])
                .status()
            {
                eprintln!("Warning: Failed to install .desktop file: {}", e);
            }
        }
    }

    // Clean up
    if let Err(e) = std::fs::remove_dir_all(&tmp_dir) {
        eprintln!("Warning: Failed to clean up {}: {}", tmp_dir, e);
    }
}

fn build_zig_tool(dir: &str, name: &str) -> bool {
    let build_zig = format!("{}/build.zig", dir);
    if !std::path::Path::new(&build_zig).exists() {
        eprintln!("No build.zig found for {}", name);
        return false;
    }

    match Command::new("zig")
        .args(["build", "-Doptimize=ReleaseFast"])
        .current_dir(dir)
        .status()
    {
        Ok(status) if status.success() => true,
        Ok(status) => {
            eprintln!("Zig build failed with code: {:?}", status.code());
            false
        }
        Err(e) => {
            eprintln!("Failed to run zig build: {}", e);
            false
        }
    }
}

fn build_rust_tool(dir: &str, name: &str, gui: bool) -> bool {
    let cargo_toml = format!("{}/Cargo.toml", dir);
    if !std::path::Path::new(&cargo_toml).exists() {
        eprintln!("No Cargo.toml found for {}", name);
        return false;
    }

    let mut args = vec!["build", "--release"];
    if gui {
        args.push("--features");
        args.push("gui");
    }

    match Command::new("cargo").args(&args).current_dir(dir).status() {
        Ok(status) if status.success() => true,
        Ok(status) => {
            eprintln!("Cargo build failed with code: {:?}", status.code());
            false
        }
        Err(e) => {
            eprintln!("Failed to run cargo build: {}", e);
            false
        }
    }
}

fn uninstall_ghost_tool(name: &str) {
    // Validate tool name to prevent path traversal
    if let Err(e) = validate_tool_name(name) {
        eprintln!("Invalid tool name '{}': {}", name, e);
        return;
    }

    println!("Uninstalling {}...", name);

    let bin_path = format!("/usr/bin/{}", name);

    // Check if file exists before trying to remove
    if !std::path::Path::new(&bin_path).exists() {
        println!("{} is not installed at {}", name, bin_path);
        return;
    }

    match Command::new("sudo").args(["rm", "-f", &bin_path]).status() {
        Ok(status) if status.success() => {
            println!("{} uninstalled successfully", name);
        }
        Ok(status) => {
            eprintln!(
                "Failed to uninstall {} (exit code: {:?})",
                name,
                status.code()
            );
        }
        Err(e) => {
            eprintln!("Failed to run uninstall command: {}", e);
        }
    }
}

fn show_tool_info(name: &str, repo: &str) {
    println!("\n📋 {} Information:", name);
    println!("🔗 Repository: https://github.com/{}", repo);
    println!(
        "📦 Installation status: {}",
        if which(name).is_ok() {
            "INSTALLED"
        } else {
            "not installed"
        }
    );

    if which(name).is_ok()
        && let Ok(output) = Command::new(name).arg("--version").output()
    {
        println!(
            "📄 Version: {}",
            String::from_utf8_lossy(&output.stdout).trim()
        );
    }

    println!("🌐 More info: https://github.com/{}", repo);
}

// Ghost Tools Installation Functions
pub fn install_all_ghost_tools() {
    println!("🏗️  Installing All Ghost Tools");
    println!("==============================");

    let tools = [
        ("Reaper", "AUR Helper", install_reaper as fn()),
        ("Oxygen", "Rust Dev Tool", install_oxygen as fn()),
        ("Zion", "Zig Meta Tool", install_zion as fn()),
        ("NVControl", "NVIDIA Control", install_nvcontrol as fn()),
    ];

    let selected = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select tools to install")
        .items(
            &tools
                .iter()
                .map(|(name, desc, _)| format!("{} - {}", name, desc))
                .collect::<Vec<_>>(),
        )
        .interact()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    for &index in &selected {
        let (name, _, install_fn) = tools[index];
        println!("\n🚀 Installing {}...", name);
        install_fn();
    }

    println!("\n✅ Ghost tools installation completed!");
}

/// Securely download and execute a script with verification
/// Downloads to temp file, shows hash for verification, requires confirmation
fn secure_script_install(url: &str, tool_name: &str) -> Result<(), String> {
    println!("📥 Downloading {} install script...", tool_name);
    println!("   URL: {}", url);

    // Download script content
    let response = get(url).map_err(|e| format!("Download failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    let content = response
        .text()
        .map_err(|e| format!("Failed to read response: {}", e))?;

    // Calculate SHA256 hash
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let hash = format!("{:x}", hasher.finalize());

    println!("\n📝 Script SHA256: {}", hash);
    println!("\n📄 Script preview (first 500 chars):");
    println!("{}", "─".repeat(50));
    let preview: String = content.chars().take(500).collect();
    println!("{}", preview);
    if content.len() > 500 {
        println!("... (truncated, {} total chars)", content.len());
    }
    println!("{}", "─".repeat(50));

    // Require explicit confirmation
    let confirm = Confirm::new()
        .with_prompt(format!(
            "⚠️  Execute this script to install {}? Review the content above.",
            tool_name
        ))
        .default(false)
        .interact()
        .unwrap_or(false);

    if !confirm {
        return Err("Installation cancelled by user".to_string());
    }

    // Create secure temp file
    let mut temp_file =
        NamedTempFile::new().map_err(|e| format!("Failed to create temp file: {}", e))?;

    // Write content
    temp_file
        .write_all(content.as_bytes())
        .map_err(|e| format!("Failed to write script: {}", e))?;

    // Set executable permissions (owner only)
    let path = temp_file.path();
    fs::set_permissions(path, fs::Permissions::from_mode(0o700))
        .map_err(|e| format!("Failed to set permissions: {}", e))?;

    // Execute
    println!("🚀 Executing install script...");
    let status = Command::new("bash").arg(path).status();

    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(format!("Script exited with code: {:?}", s.code())),
        Err(e) => Err(format!("Failed to execute: {}", e)),
    }
    // temp_file is automatically deleted when dropped
}

pub fn install_reaper() {
    println!("⚡ Installing Reaper (AUR Helper)");
    println!("=================================");

    if Command::new("which").arg("reap").status().is_ok() {
        println!("✅ Reaper is already installed");
        return;
    }

    match secure_script_install(
        "https://raw.githubusercontent.com/face-hh/reaper/main/release/install.sh",
        "Reaper",
    ) {
        Ok(()) => {
            println!("✅ Reaper installed successfully");
            println!("💡 Usage: reap <package_name>");
        }
        Err(e) => println!("❌ Failed to install Reaper: {}", e),
    }
}

pub fn install_oxygen() {
    println!("🦀 Installing Oxygen (Rust Dev Tool)");
    println!("====================================");

    if Command::new("which").arg("oxygen").status().is_ok() {
        println!("✅ Oxygen is already installed");
        return;
    }

    match secure_script_install(
        "https://raw.githubusercontent.com/ghostkellz/oxygen/main/install.sh",
        "Oxygen",
    ) {
        Ok(()) => {
            println!("✅ Oxygen installed successfully");
            println!("💡 Oxygen is a Rust development toolkit");
            println!("📚 Features: project management, testing, deployment");
        }
        Err(e) => println!("❌ Failed to install Oxygen: {}", e),
    }
}

pub fn install_zion() {
    println!("⚡ Installing Zion (Zig Meta Tool)");
    println!("==================================");

    if Command::new("which").arg("zion").status().is_ok() {
        println!("✅ Zion is already installed");
        return;
    }

    match secure_script_install(
        "https://raw.githubusercontent.com/ghostkellz/zion/main/release/install.sh",
        "Zion",
    ) {
        Ok(()) => {
            println!("✅ Zion installed successfully");
            println!("💡 Zion is a Zig meta-tool for project management");
            println!("🔗 GitHub: https://github.com/GhostKellz/zion");
        }
        Err(e) => println!("❌ Failed to install Zion: {}", e),
    }
}

fn install_nvcontrol() {
    println!("Installing NVControl (NVIDIA Control)");
    println!("=========================================");

    if Command::new("which")
        .arg("nvcontrol")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("NVControl is already installed");
        return;
    }

    let confirm = match Confirm::new()
        .with_prompt("Install NVControl NVIDIA management tool?")
        .default(true)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    if confirm {
        // Use the secure script installer
        match secure_script_install(
            "https://raw.githubusercontent.com/GhostKellz/nvcontrol/main/install.sh",
            "NVControl",
        ) {
            Ok(()) => {
                println!("NVControl installed successfully");
                println!("NVControl provides NVIDIA GPU management");
                println!("GitHub: https://github.com/GhostKellz/nvcontrol");
            }
            Err(e) => eprintln!("Failed to install NVControl: {}", e),
        }
    }
}

pub fn check_tool_status() {
    println!("📊 Ghost Tools Status Check");
    println!("===========================");

    let tools = [
        ("reap", "Reaper AUR Helper"),
        ("oxygen", "Oxygen Rust Tool"),
        ("zion", "Zion Zig Tool"),
        ("nvcontrol", "NVControl NVIDIA Tool"),
    ];

    for (cmd, name) in &tools {
        let status = Command::new("which").arg(cmd).status();

        if status.map(|s| s.success()).unwrap_or(false) {
            println!("  ✅ {} - Installed", name);

            // Try to get version info
            if let Ok(output) = Command::new(cmd).arg("--version").output() {
                let version = String::from_utf8_lossy(&output.stdout);
                if !version.trim().is_empty() {
                    println!("     📋 Version: {}", version.trim());
                }
            }
        } else {
            println!("  ❌ {} - Not installed", name);
        }
    }

    println!("\n💡 Use 'Install All Ghost Tools' to install missing tools");
}

#[allow(dead_code)]
fn uninstall_ghost_tools() {
    println!("Uninstall Ghost Tools");
    println!("=========================");

    let warning = "Warning: This will remove Ghost-branded tools from your system";
    println!("{}", warning);

    let confirm = match Confirm::new()
        .with_prompt("Are you sure you want to uninstall Ghost tools?")
        .default(false)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    if confirm {
        // Trusted tool names from the ecosystem
        let tools = ["reap", "oxygen", "zion", "nvcontrol"];

        for tool in &tools {
            // Validate tool name even though these are hardcoded - defense in depth
            if validate_tool_name(tool).is_err() {
                continue;
            }

            if Command::new("which")
                .arg(tool)
                .output()
                .is_ok_and(|o| o.status.success())
            {
                println!("Removing {}...", tool);

                // Try to remove from common installation paths
                // Paths are constructed safely with validated tool names
                let home_local_bin =
                    dirs::home_dir().map(|h| format!("{}/.local/bin/{}", h.display(), tool));

                let paths: Vec<String> = [
                    Some(format!("/usr/local/bin/{}", tool)),
                    Some(format!("/usr/bin/{}", tool)),
                    home_local_bin,
                ]
                .into_iter()
                .flatten()
                .collect();

                for path in &paths {
                    if std::path::Path::new(path).exists() {
                        match Command::new("sudo").args(["rm", "-f", path]).status() {
                            Ok(status) if status.success() => {
                                println!("Removed: {}", path);
                            }
                            Ok(_) => {
                                eprintln!("Warning: Failed to remove {}", path);
                            }
                            Err(e) => {
                                eprintln!("Warning: Error removing {}: {}", path, e);
                            }
                        }
                    }
                }
            }
        }

        println!("Ghost tools uninstalled");
    }
}

// Legacy compatibility function
#[allow(dead_code)]
pub fn install_ghost_tools_menu() {
    // Redirect to new comprehensive ecosystem manager
    ghost_ecosystem_menu();
}
