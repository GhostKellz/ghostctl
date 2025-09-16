use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use reqwest::blocking::get;
use std::process::Command;

pub mod enhanced;
pub mod helper;
pub mod storage_migration;
pub mod template_management;
pub mod backup_rotation;
pub mod firewall_automation;
pub mod advanced_security;
// pub mod vfio;
// pub mod upgrade;
// pub mod pbs;

// Popular scripts from the community-scripts repo
const POPULAR_COMMUNITY_SCRIPTS: &[(&str, &str)] = &[
    (
        "Post Install",
        "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/misc/post-pve-install.sh",
    ),
    (
        "Docker LXC",
        "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/docker.sh",
    ),
    (
        "Home Assistant OS VM",
        "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/vm/haos-vm.sh",
    ),
    (
        "Nginx Proxy Manager LXC",
        "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/nginxproxymanager.sh",
    ),
    (
        "Portainer LXC",
        "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/portainer.sh",
    ),
    (
        "Pi-hole LXC",
        "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/pihole.sh",
    ),
    (
        "Nextcloud LXC",
        "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/nextcloud.sh",
    ),
    (
        "Plex LXC",
        "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/plex.sh",
    ),
];

// Repository configurations
#[allow(dead_code)]
const COMMUNITY_SCRIPTS_REPO: &str =
    "https://api.github.com/repos/community-scripts/ProxmoxVE/contents";
#[allow(dead_code)]
const CKTECH_REPO: &str = "https://api.github.com/repos/GhostKellz/proxmox/contents/helper-scripts";

pub fn proxmox_menu() {
    loop {
        let menu_type = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🏥 Proxmox VE Tools")
            .items(&[
                "🚀 Quick Access (Popular Scripts)",
                "📂 Enhanced Categories & Management",
                "🎮 VFIO GPU Passthrough",
                "🚀 PVE Upgrade (8→9)",
                "🔐 Proxmox Backup Server (PBS)",
                "🏢 CKTech Helper Scripts", 
                "🌐 All Community Scripts",
                "⬅️  Back",
            ])
            .default(1)
            .interact()
            .unwrap();

        match menu_type {
            0 => quick_access_menu(),
            1 => enhanced::enhanced_proxmox_menu(),
            2 => {
                println!("🎮 VFIO GPU Passthrough - Coming in next build!");
                println!("Features: GPU passthrough, vendor reset, diagnostics");
            }
            3 => {
                println!("🚀 PVE Upgrade (8→9) - Coming in next build!");  
                println!("Features: Cluster upgrades, repo management, node draining");
            }
            4 => {
                println!("🔐 PBS Management - Coming in next build!");
                println!("Features: Datastore management, maintenance tasks, tuning");
            }
            5 => helper::cktech_helper_scripts(),
            6 => helper::community_scripts_enhanced(),
            _ => break,
        }
    }
}

fn quick_access_menu() {
    loop {
        let mut menu_items: Vec<String> = POPULAR_COMMUNITY_SCRIPTS
            .iter()
            .map(|(name, _)| name.to_string())
            .collect();

        menu_items.extend_from_slice(&[
            "Browse All Scripts".to_string(),
            "Custom Script URL".to_string(),
            "Back".to_string(),
        ]);

        let idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Proxmox Helper Scripts")
            .items(&menu_items)
            .default(0)
            .interact()
            .unwrap();

        if idx == menu_items.len() - 1 {
            break; // Back
        } else if idx == menu_items.len() - 2 {
            // Custom Script URL
            custom_script_url();
        } else if idx == menu_items.len() - 3 {
            // Browse All Scripts
            browse_all_scripts();
        } else {
            // Popular Script
            let (name, url) = POPULAR_COMMUNITY_SCRIPTS[idx];
            confirm_and_run_script(name, url);
        }
    }
}

pub fn run_script_by_url(url: &str) {
    println!("Fetching and running script from: {}", url);
    match fetch_and_run_script(url) {
        Ok(_) => println!("✅ Script executed successfully."),
        Err(e) => println!("❌ Error: {}", e),
    }
}

#[allow(dead_code)]
pub fn list_popular_scripts() {
    println!("📋 Popular Proxmox Helper Scripts:");
    for (i, (name, url)) in POPULAR_COMMUNITY_SCRIPTS.iter().enumerate() {
        println!("  {}. {} - {}", i + 1, name, url);
    }
}

fn custom_script_url() {
    let url: String = Input::new()
        .with_prompt("Enter the script URL")
        .interact_text()
        .unwrap();

    if url.trim().is_empty() {
        println!("❌ No URL provided.");
        return;
    }

    confirm_and_run_script("Custom Script", &url);
}

pub fn browse_all_scripts() {
    println!("🌐 Opening Proxmox VE Community Scripts repository...");
    let _ = Command::new("xdg-open")
        .arg("https://community-scripts.github.io/ProxmoxVE/")
        .status();
}

fn confirm_and_run_script(name: &str, url: &str) {
    println!("\n📜 Script: {}", name);
    println!("🔗 URL: {}", url);

    let confirm = Confirm::new()
        .with_prompt("Do you want to run this script?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        match fetch_and_run_script(url) {
            Ok(_) => println!("✅ Script '{}' executed successfully.", name),
            Err(e) => println!("❌ Failed to run '{}': {}", name, e),
        }
    } else {
        println!("❌ Script execution cancelled.");
    }
}

fn fetch_and_run_script(url: &str) -> Result<(), String> {
    println!("📥 Fetching script...");

    let response = get(url).map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    let script = response
        .text()
        .map_err(|e| format!("Failed to read response: {}", e))?;

    if script.trim().is_empty() {
        return Err("Script is empty".to_string());
    }

    println!("🚀 Running script...");

    let status = Command::new("bash")
        .arg("-c")
        .arg(&script)
        .status()
        .map_err(|e| format!("Failed to execute script: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Script failed with exit code: {:?}", status.code()))
    }
}
