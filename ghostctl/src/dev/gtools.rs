use dialoguer::{Confirm, MultiSelect, Select, theme::ColorfulTheme};
use std::process::Command;
use which::which;

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
    (
        "ghostscan",
        "ghostkellz/ghostscan",
        "Network scanner and security tool",
        "network",
    ),
    (
        "zendns",
        "ghostkellz/zendns",
        "High-performance DNS server in Rust",
        "network",
    ),
    (
        "zigdns",
        "ghostkellz/zigdns",
        "Lightweight DNS server in Zig",
        "network",
    ),
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

        let category_idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("👻 Ghost Ecosystem - Select Category")
            .items(&categories)
            .default(0)
            .interact()
            .unwrap();

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

        let idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("👻 Ghost Ecosystem - All Tools")
            .items(&menu_items)
            .default(0)
            .interact()
            .unwrap();

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

    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("👻 {} Tools", category_name))
        .items(&menu_items)
        .default(0)
        .interact()
        .unwrap();

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

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select tools to install (Space to select, Enter to confirm)")
        .items(&tool_names)
        .interact()
        .unwrap();

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

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("⚠️  Select tools to UNINSTALL (Space to select, Enter to confirm)")
        .items(&installed_tools)
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("❌ No tools selected for removal");
        return;
    }

    // Confirmation
    let confirm = dialoguer::Confirm::new()
        .with_prompt(format!("⚠️  Really uninstall {} tools?", selections.len()))
        .default(false)
        .interact()
        .unwrap();

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

    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("{}: Choose action", name))
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match options[action] {
        "Install" | "Update" => install_ghost_tool(name, repo),
        "Uninstall" => uninstall_ghost_tool(name),
        "View Info" => show_tool_info(name, repo),
        _ => {}
    }
}

fn install_ghost_tool(name: &str, repo: &str) {
    println!("🚀 Installing/Updating {}...", name);

    // Special handling for different tool types
    let build_command = match name {
        "zion" | "zaur" | "zigdns" => {
            // Zig-based tools
            format!(
                r#"
                if [ -f build.zig ]; then
                    zig build -Doptimize=ReleaseFast
                    sudo install -Dm755 zig-out/bin/{name} /usr/bin/{name}
                else
                    echo "❌ No build.zig found for {name}"
                    exit 1
                fi
            "#,
                name = name
            )
        }
        "ghostview" | "phantomlink" | "nvcontrol" => {
            // GUI tools (might need additional deps)
            format!(
                r#"
                if [ -f Cargo.toml ]; then
                    cargo build --release --features gui
                    sudo install -Dm755 target/release/{name} /usr/bin/{name}
                    # Install .desktop file if present
                    if [ -f {name}.desktop ]; then
                        sudo install -Dm644 {name}.desktop /usr/share/applications/{name}.desktop
                    fi
                else
                    echo "❌ No Cargo.toml found for {name}"
                    exit 1
                fi
            "#,
                name = name
            )
        }
        _ => {
            // Standard Rust tools
            format!(
                r#"
                if [ -f Cargo.toml ]; then
                    cargo build --release
                    sudo install -Dm755 target/release/{name} /usr/bin/{name}
                else
                    echo "❌ No Cargo.toml found for {name}"
                    exit 1
                fi
            "#,
                name = name
            )
        }
    };

    let install_script = format!(
        r#"
        set -e
        cd /tmp
        rm -rf {name}
        
        if ! git clone https://github.com/{repo}.git {name}; then
            echo "❌ Repository not found or not yet created: {repo}"
            echo "💡 This tool may be in development. Check https://github.com/{repo}"
            exit 1
        fi
        
        cd {name}
        {build_command}
        
        echo "✅ {name} installed successfully!"
        echo "🔧 Run '{name} --help' to get started"
        "#,
        name = name,
        repo = repo,
        build_command = build_command
    );

    let status = Command::new("bash").arg("-c").arg(install_script).status();

    match status {
        Ok(s) if s.success() => println!("✅ {} installed successfully!", name),
        _ => {
            println!("❌ Failed to install {}", name);
            println!(
                "💡 This tool may not be available yet. Check https://github.com/{}",
                repo
            );
        }
    }
}

fn uninstall_ghost_tool(name: &str) {
    println!("🗑️  Uninstalling {}...", name);

    let status = Command::new("sudo")
        .args(["rm", "-f", &format!("/usr/bin/{}", name)])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ {} uninstalled successfully", name),
        _ => println!("❌ Failed to uninstall {}", name),
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

    if which(name).is_ok() {
        if let Ok(output) = Command::new(name).arg("--version").output() {
            println!(
                "📄 Version: {}",
                String::from_utf8_lossy(&output.stdout).trim()
            );
        }
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

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select tools to install")
        .items(
            &tools
                .iter()
                .map(|(name, desc, _)| format!("{} - {}", name, desc))
                .collect::<Vec<_>>(),
        )
        .interact()
        .unwrap();

    for &index in &selected {
        let (name, _, install_fn) = tools[index];
        println!("\n🚀 Installing {}...", name);
        install_fn();
    }

    println!("\n✅ Ghost tools installation completed!");
}

pub fn install_reaper() {
    println!("⚡ Installing Reaper (AUR Helper)");
    println!("=================================");

    if Command::new("which").arg("reap").status().is_ok() {
        println!("✅ Reaper is already installed");
        return;
    }

    let confirm = Confirm::new()
        .with_prompt("Install Reaper AUR helper?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("📥 Downloading and installing Reaper...");
        let status = Command::new("bash")
            .arg("-c")
            .arg("curl -sSL https://raw.githubusercontent.com/face-hh/reaper/main/release/install.sh | bash")
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Reaper installed successfully");
                println!("💡 Usage: reap <package_name>");
            }
            _ => println!("❌ Failed to install Reaper"),
        }
    }
}

pub fn install_oxygen() {
    println!("🦀 Installing Oxygen (Rust Dev Tool)");
    println!("====================================");

    if Command::new("which").arg("oxygen").status().is_ok() {
        println!("✅ Oxygen is already installed");
        return;
    }

    let confirm = Confirm::new()
        .with_prompt("Install Oxygen Rust development tool?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("📥 Downloading and installing Oxygen...");
        let status = Command::new("bash")
            .arg("-c")
            .arg("curl -sSL https://raw.githubusercontent.com/ghostkellz/oxygen/main/install.sh | bash")
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Oxygen installed successfully");
                println!("💡 Oxygen is a Rust development toolkit");
                println!("📚 Features: project management, testing, deployment");
            }
            _ => println!("❌ Failed to install Oxygen"),
        }
    }
}

pub fn install_zion() {
    println!("⚡ Installing Zion (Zig Meta Tool)");
    println!("==================================");

    if Command::new("which").arg("zion").status().is_ok() {
        println!("✅ Zion is already installed");
        return;
    }

    let confirm = Confirm::new()
        .with_prompt("Install Zion Zig meta-tool?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("📥 Downloading and installing Zion...");
        let status = Command::new("bash")
            .arg("-c")
            .arg("curl -sSL https://raw.githubusercontent.com/ghostkellz/zion/main/release/install.sh | bash")
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Zion installed successfully");
                println!("💡 Zion is a Zig meta-tool for project management");
                println!("🔗 GitHub: https://github.com/GhostKellz/zion");
            }
            _ => println!("❌ Failed to install Zion"),
        }
    }
}

fn install_nvcontrol() {
    println!("🎮 Installing NVControl (NVIDIA Control)");
    println!("=========================================");

    if Command::new("which").arg("nvcontrol").status().is_ok() {
        println!("✅ NVControl is already installed");
        return;
    }

    let confirm = Confirm::new()
        .with_prompt("Install NVControl NVIDIA management tool?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("📥 Downloading NVControl installer...");

        // Download the install script
        let status = Command::new("curl")
            .args(&[
                "-O",
                "https://raw.githubusercontent.com/GhostKellz/nvcontrol/main/install.sh",
            ])
            .status();

        if status.is_ok() && status.unwrap().success() {
            // Make it executable and run
            let _ = Command::new("chmod").args(&["+x", "install.sh"]).status();
            let install_status = Command::new("bash").arg("install.sh").status();

            match install_status {
                Ok(s) if s.success() => {
                    println!("✅ NVControl installed successfully");
                    println!("💡 NVControl provides NVIDIA GPU management");
                    println!("🔗 GitHub: https://github.com/GhostKellz/nvcontrol");
                }
                _ => println!("❌ Failed to install NVControl"),
            }

            // Cleanup
            let _ = std::fs::remove_file("install.sh");
        } else {
            println!("❌ Failed to download NVControl installer");
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

        if status.is_ok() && status.unwrap().success() {
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
    println!("🗑️  Uninstall Ghost Tools");
    println!("=========================");

    let warning = "⚠️  This will remove Ghost-branded tools from your system";
    println!("{}", warning);

    let confirm = Confirm::new()
        .with_prompt("Are you sure you want to uninstall Ghost tools?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let tools = ["reap", "oxygen", "zion", "nvcontrol"];

        for tool in &tools {
            if Command::new("which").arg(tool).status().is_ok() {
                println!("🗑️  Removing {}...", tool);

                // Try to remove from common installation paths
                let paths = [
                    format!("/usr/local/bin/{}", tool),
                    format!("/usr/bin/{}", tool),
                    format!(
                        "{}/.local/bin/{}",
                        dirs::home_dir().unwrap().display(),
                        tool
                    ),
                ];

                for path in &paths {
                    if std::path::Path::new(path).exists() {
                        let _ = Command::new("sudo").args(&["rm", "-f", path]).status();
                    }
                }
            }
        }

        println!("✅ Ghost tools uninstalled");
    }
}

// Legacy compatibility function
#[allow(dead_code)]
pub fn install_ghost_tools_menu() {
    // Redirect to new comprehensive ecosystem manager
    ghost_ecosystem_menu();
}
