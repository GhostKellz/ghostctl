use dialoguer::{Select, theme::ColorfulTheme};
use reqwest::blocking::get;

// Repository URLs
const CKTECH_REPO_BASE: &str =
    "https://api.github.com/repos/GhostKellz/proxmox/contents/helper-scripts";
const COMMUNITY_REPO_BASE: &str =
    "https://api.github.com/repos/community-scripts/ProxmoxVE/contents";

pub fn cktech_helper_scripts() {
    println!("🏢 CKTech Helper Scripts");
    println!("========================");

    let script_types = ["📦 LXC Containers", "🖥️  Virtual Machines", "⬅️  Back"];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select CKTech script type")
        .items(&script_types)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => load_cktech_lxc_scripts(),
        1 => load_cktech_vm_scripts(),
        _ => return,
    }
}

pub fn community_scripts_enhanced() {
    println!("🌐 Community Proxmox Scripts");
    println!("============================");

    let script_types = [
        "📦 LXC Containers",
        "🖥️  Virtual Machines",
        "🔧 Miscellaneous Scripts",
        "🛠️  PVE Tools & Utilities",
        "🔑 TurnKey Linux Templates",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Community script type")
        .items(&script_types)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => load_community_lxc_scripts(),
        1 => load_community_vm_scripts(),
        2 => load_community_misc_scripts(),
        3 => load_pve_tools(),
        4 => load_turnkey_scripts(),
        _ => return,
    }
}

fn load_cktech_lxc_scripts() {
    println!("📦 Loading CKTech LXC Helper Scripts...");
    println!("📍 Repository: https://github.com/GhostKellz/proxmox/tree/main/helper-scripts/ct");

    let api_url = format!("{}/ct", CKTECH_REPO_BASE);

    match get(&api_url) {
        Ok(response) if response.status().is_success() => {
            if let Ok(scripts) = response.json::<serde_json::Value>() {
                if let Some(files) = scripts.as_array() {
                    let mut script_names = Vec::new();

                    for file in files {
                        if let Some(name) = file["name"].as_str()
                            && name.ends_with(".sh")
                        {
                            script_names.push(name.to_string());
                        }
                    }

                    if !script_names.is_empty() {
                        script_names.push("⬅️  Back".to_string());

                        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Select CKTech LXC script")
                            .items(&script_names)
                            .default(0)
                            .interact()
                        else {
                            return;
                        };

                        if choice < script_names.len() - 1 {
                            let script_name = &script_names[choice];
                            let script_url = format!(
                                "https://raw.githubusercontent.com/GhostKellz/proxmox/main/helper-scripts/ct/{}",
                                script_name
                            );
                            confirm_and_run_script(script_name, &script_url);
                        }
                    } else {
                        println!("No LXC scripts found in CKTech repository");
                    }
                } else {
                    println!("Invalid response format from GitHub API");
                }
            } else {
                println!("Failed to parse GitHub API response");
            }
        }
        _ => {
            println!("Failed to fetch CKTech scripts from GitHub");
            println!("Falling back to known scripts...");
            show_fallback_cktech_lxc();
        }
    }
}

fn load_cktech_vm_scripts() {
    println!("🖥️  Loading CKTech VM Helper Scripts...");
    println!("📍 Repository: https://github.com/GhostKellz/proxmox/tree/main/helper-scripts/vm");

    let api_url = format!("{}/vm", CKTECH_REPO_BASE);

    match get(&api_url) {
        Ok(response) if response.status().is_success() => {
            if let Ok(scripts) = response.json::<serde_json::Value>() {
                if let Some(files) = scripts.as_array() {
                    let mut script_names = Vec::new();

                    for file in files {
                        if let Some(name) = file["name"].as_str()
                            && name.ends_with(".sh")
                        {
                            script_names.push(name.to_string());
                        }
                    }

                    if !script_names.is_empty() {
                        script_names.push("⬅️  Back".to_string());

                        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Select CKTech VM script")
                            .items(&script_names)
                            .default(0)
                            .interact()
                        else {
                            return;
                        };

                        if choice < script_names.len() - 1 {
                            let script_name = &script_names[choice];
                            let script_url = format!(
                                "https://raw.githubusercontent.com/GhostKellz/proxmox/main/helper-scripts/vm/{}",
                                script_name
                            );
                            confirm_and_run_script(script_name, &script_url);
                        }
                    } else {
                        println!("No VM scripts found in CKTech repository");
                    }
                } else {
                    println!("Invalid response format from GitHub API");
                }
            } else {
                println!("Failed to parse GitHub API response");
            }
        }
        _ => {
            println!("Failed to fetch CKTech VM scripts from GitHub");
            println!("Visit: https://github.com/GhostKellz/proxmox/tree/main/helper-scripts/vm");
        }
    }
}

fn load_community_lxc_scripts() {
    println!("📦 Loading Community LXC Scripts...");
    println!("📍 Repository: https://github.com/community-scripts/ProxmoxVE/tree/main/ct");

    let api_url = format!("{}/ct", COMMUNITY_REPO_BASE);

    match get(&api_url) {
        Ok(response) if response.status().is_success() => {
            if let Ok(scripts) = response.json::<serde_json::Value>() {
                if let Some(files) = scripts.as_array() {
                    let mut script_names = Vec::new();

                    for file in files {
                        if let Some(name) = file["name"].as_str()
                            && name.ends_with(".sh")
                        {
                            // Remove .sh extension for display
                            let display_name = name.trim_end_matches(".sh");
                            script_names.push((display_name.to_string(), name.to_string()));
                        }
                    }

                    if !script_names.is_empty() {
                        script_names.sort_by(|a, b| a.0.cmp(&b.0));
                        let mut display_names: Vec<String> = script_names
                            .iter()
                            .map(|(display, _)| display.clone())
                            .collect();
                        display_names.push("⬅️  Back".to_string());

                        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Select Community LXC script")
                            .items(&display_names)
                            .default(0)
                            .interact()
                        else {
                            return;
                        };

                        if choice < script_names.len() {
                            let (display_name, file_name) = &script_names[choice];
                            let script_url = format!(
                                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/{}",
                                file_name
                            );
                            confirm_and_run_script(display_name, &script_url);
                        }
                    } else {
                        println!("No LXC scripts found in Community repository");
                    }
                } else {
                    println!("Invalid response format from GitHub API");
                }
            } else {
                println!("Failed to parse GitHub API response");
            }
        }
        _ => {
            println!("Failed to fetch Community LXC scripts from GitHub");
            println!("Visit: https://github.com/community-scripts/ProxmoxVE/tree/main/ct");
        }
    }
}

fn load_community_vm_scripts() {
    println!("🖥️  Loading Community VM Scripts...");
    println!("📍 Repository: https://github.com/community-scripts/ProxmoxVE/tree/main/vm");

    let api_url = format!("{}/vm", COMMUNITY_REPO_BASE);

    match get(&api_url) {
        Ok(response) if response.status().is_success() => {
            if let Ok(scripts) = response.json::<serde_json::Value>() {
                if let Some(files) = scripts.as_array() {
                    let mut script_names = Vec::new();

                    for file in files {
                        if let Some(name) = file["name"].as_str()
                            && name.ends_with(".sh")
                        {
                            let display_name = name.trim_end_matches(".sh");
                            script_names.push((display_name.to_string(), name.to_string()));
                        }
                    }

                    if !script_names.is_empty() {
                        script_names.sort_by(|a, b| a.0.cmp(&b.0));
                        let mut display_names: Vec<String> = script_names
                            .iter()
                            .map(|(display, _)| display.clone())
                            .collect();
                        display_names.push("⬅️  Back".to_string());

                        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Select Community VM script")
                            .items(&display_names)
                            .default(0)
                            .interact()
                        else {
                            return;
                        };

                        if choice < script_names.len() {
                            let (display_name, file_name) = &script_names[choice];
                            let script_url = format!(
                                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/vm/{}",
                                file_name
                            );
                            confirm_and_run_script(display_name, &script_url);
                        }
                    } else {
                        println!("No VM scripts found in Community repository");
                    }
                } else {
                    println!("Invalid response format from GitHub API");
                }
            } else {
                println!("Failed to parse GitHub API response");
            }
        }
        _ => {
            println!("Failed to fetch Community VM scripts from GitHub");
            println!("Visit: https://github.com/community-scripts/ProxmoxVE/tree/main/vm");
        }
    }
}

fn load_community_misc_scripts() {
    println!("🔧 Loading Community Miscellaneous Scripts...");
    println!("📍 Repository: https://github.com/community-scripts/ProxmoxVE/tree/main/misc");

    let api_url = format!("{}/misc", COMMUNITY_REPO_BASE);

    match get(&api_url) {
        Ok(response) if response.status().is_success() => {
            if let Ok(scripts) = response.json::<serde_json::Value>() {
                if let Some(files) = scripts.as_array() {
                    let mut script_names = Vec::new();

                    for file in files {
                        if let Some(name) = file["name"].as_str()
                            && name.ends_with(".sh")
                        {
                            let display_name = name.trim_end_matches(".sh");
                            script_names.push((display_name.to_string(), name.to_string()));
                        }
                    }

                    if !script_names.is_empty() {
                        script_names.sort_by(|a, b| a.0.cmp(&b.0));
                        let mut display_names: Vec<String> = script_names
                            .iter()
                            .map(|(display, _)| display.clone())
                            .collect();
                        display_names.push("⬅️  Back".to_string());

                        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Select Community Misc script")
                            .items(&display_names)
                            .default(0)
                            .interact()
                        else {
                            return;
                        };

                        if choice < script_names.len() {
                            let (display_name, file_name) = &script_names[choice];
                            let script_url = format!(
                                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/misc/{}",
                                file_name
                            );
                            confirm_and_run_script(display_name, &script_url);
                        }
                    } else {
                        println!("No misc scripts found in Community repository");
                    }
                } else {
                    println!("Invalid response format from GitHub API");
                }
            } else {
                println!("Failed to parse GitHub API response");
            }
        }
        _ => {
            println!("Failed to fetch Community misc scripts from GitHub");
            println!("Visit: https://github.com/community-scripts/ProxmoxVE/tree/main/misc");
        }
    }
}

fn load_pve_tools() {
    println!("🛠️  Loading PVE Tools & Utilities...");
    println!("📍 Repository: https://github.com/community-scripts/ProxmoxVE/tree/main/tools/pve");

    let api_url = format!("{}/tools/pve", COMMUNITY_REPO_BASE);

    match get(&api_url) {
        Ok(response) if response.status().is_success() => {
            if let Ok(scripts) = response.json::<serde_json::Value>() {
                if let Some(files) = scripts.as_array() {
                    let mut script_names = Vec::new();

                    for file in files {
                        if let Some(name) = file["name"].as_str()
                            && name.ends_with(".sh")
                        {
                            let display_name = name.trim_end_matches(".sh");
                            script_names.push((display_name.to_string(), name.to_string()));
                        }
                    }

                    if !script_names.is_empty() {
                        script_names.sort_by(|a, b| a.0.cmp(&b.0));
                        let mut display_names: Vec<String> = script_names
                            .iter()
                            .map(|(display, _)| display.clone())
                            .collect();
                        display_names.push("⬅️  Back".to_string());

                        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Select PVE Tool")
                            .items(&display_names)
                            .default(0)
                            .interact()
                        else {
                            return;
                        };

                        if choice < script_names.len() {
                            let (display_name, file_name) = &script_names[choice];
                            let script_url = format!(
                                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/tools/pve/{}",
                                file_name
                            );

                            // Show additional info for PVE tools
                            println!("\n🛠️  PVE Tool: {}", display_name);
                            match display_name.as_str() {
                                "clean" => println!(
                                    "📝 Description: Cleanup and optimization tool for Proxmox VE"
                                ),
                                "pbs3-upgrade" => println!(
                                    "📝 Description: Proxmox Backup Server upgrade utility"
                                ),
                                "pve8-upgrade" => {
                                    println!("📝 Description: Proxmox VE 8.x upgrade utility")
                                }
                                _ => println!("📝 Description: Proxmox VE utility tool"),
                            }

                            confirm_and_run_script(display_name, &script_url);
                        }
                    } else {
                        println!("No PVE tools found in Community repository");
                        show_fallback_pve_tools();
                    }
                } else {
                    println!("Invalid response format from GitHub API");
                }
            } else {
                println!("Failed to parse GitHub API response");
            }
        }
        _ => {
            println!("Failed to fetch PVE tools from GitHub");
            println!("Visit: https://github.com/community-scripts/ProxmoxVE/tree/main/tools/pve");
            show_fallback_pve_tools();
        }
    }
}

fn load_turnkey_scripts() {
    println!("🔑 TurnKey Linux Templates");
    println!("==========================");
    println!("📍 Repository: https://github.com/community-scripts/ProxmoxVE/tree/main/turnkey");

    let options = [
        "🔑 Run TurnKey Template Installer",
        "📋 View TurnKey Information",
        "🌐 Browse TurnKey Templates Online",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("TurnKey Linux Options")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => {
            let script_url = "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/turnkey/turnkey.sh";
            confirm_and_run_script("TurnKey Linux Template Installer", script_url);
        }
        1 => {
            println!("\n🔑 TurnKey Linux Templates");
            println!("===========================");
            println!("TurnKey Linux is a collection of pre-configured virtual appliances");
            println!("that include popular web applications and system tools.");
            println!("\nFeatures:");
            println!("• 180+ ready-to-use appliances");
            println!("• Security hardened and optimized");
            println!("• Web-based administration");
            println!("• Automatic security updates");
            println!("\nUse the installer to browse and deploy TurnKey templates.");
        }
        2 => {
            println!("🌐 Opening TurnKey Linux website...");
            if let Err(e) = std::process::Command::new("xdg-open")
                .arg("https://www.turnkeylinux.org/")
                .status()
            {
                println!("Could not open browser: {}", e);
                println!("Visit: https://www.turnkeylinux.org/");
            }
        }
        _ => return,
    }
}

fn show_fallback_pve_tools() {
    println!("\n🛠️  Popular PVE Tools:");
    println!("• clean.sh - System cleanup and optimization");
    println!("• pbs3-upgrade.sh - Proxmox Backup Server upgrade");
    println!("• pve8-upgrade.sh - Proxmox VE 8.x upgrade");
    println!("\nVisit the repository for the complete list and manual execution.");
}

fn show_fallback_cktech_lxc() {
    println!("🏢 CKTech LXC Scripts (Fallback)");
    println!("Note: Visit https://github.com/GhostKellz/proxmox/tree/main/helper-scripts/ct");
    println!("for the latest scripts.");
}

fn confirm_and_run_script(name: &str, url: &str) {
    println!("\n📜 Proxmox Script Execution");
    println!("═══════════════════════════");

    match super::script_safety::safe_run_script(name, url) {
        Ok(true) => println!("✅ Script '{}' executed successfully.", name),
        Ok(false) => println!("⏭️  Script execution was cancelled or skipped."),
        Err(e) => println!("❌ Failed to run '{}': {}", name, e),
    }
}
