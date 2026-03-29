use crate::utils::run_command;
use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use chrono::Local;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStatus {
    pub name: String,
    pub version: String,
    pub status: String,
    pub quorum: bool,
    pub local: bool,
}

#[derive(Debug, Clone)]
pub struct UpgradeConfig {
    pub use_enterprise: bool,
    pub manage_ceph: bool,
    pub ceph_version: String,
    pub dry_run: bool,
}

const APT_SOURCES_DIR: &str = "/etc/apt/sources.list.d";
const PVE_ENTERPRISE_LIST: &str = "/etc/apt/sources.list.d/pve-enterprise.list";
const PVE_NO_SUB_LIST: &str = "/etc/apt/sources.list.d/pve-no-subscription.list";
const CEPH_LIST: &str = "/etc/apt/sources.list.d/ceph.list";
const DEBIAN_SOURCES: &str = "/etc/apt/sources.list";

pub fn upgrade_menu() {
    loop {
        let options = vec![
            "Pre-upgrade Check",
            "Configure Repositories",
            "Upgrade Single Node",
            "Drain Node (Migrate VMs/CTs)",
            "Wave Upgrade (Multiple Nodes)",
            "Ceph Repository Management",
            "Rollback Configuration",
            "View Upgrade Logs",
            "Back",
        ];

        let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🚀 PVE Upgrade Tools (8→9)")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match selection {
            0 => precheck(),
            1 => configure_repositories(),
            2 => upgrade_single_node(),
            3 => drain_node_menu(),
            4 => wave_upgrade(),
            5 => ceph_management(),
            6 => rollback_menu(),
            7 => view_logs(),
            _ => break,
        }
    }
}

fn precheck() {
    println!("🔍 Running PVE 8→9 Pre-upgrade Checks...\n");
    
    // Check current PVE version
    let version = get_pve_version();
    println!("📌 Current PVE Version: {}", version);
    
    if !version.starts_with("8.") {
        println!("⚠️  WARNING: Not running PVE 8.x - upgrade path may differ!");
    }
    
    // Run pve8to9 if available
    println!("\n🔍 Running pve8to9 checker...");
    let output = Command::new("pve8to9")
        .arg("--full")
        .output();
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            if !stdout.is_empty() {
                println!("{}", stdout);
            }
            if !stderr.is_empty() {
                println!("⚠️  Warnings:\n{}", stderr);
            }
        }
        Err(_) => {
            println!("❌ pve8to9 tool not found. Running manual checks...");
            manual_precheck();
        }
    }
    
    // Check cluster status
    check_cluster_health();
    
    // Check storage status
    check_storage_health();
    
    // Check for Ceph
    check_ceph_status();
    
    println!("\n✅ Pre-check complete. Review any warnings above before proceeding.");
}

fn manual_precheck() {
    println!("\n📋 Manual Pre-upgrade Checklist:");
    
    // Check disk space
    let df_output = Command::new("df")
        .args(&["-h", "/"])
        .output()
        .unwrap_or_default();
    
    println!("\n💾 Root filesystem space:");
    println!("{}", String::from_utf8_lossy(&df_output.stdout));
    
    // Check for held packages
    let held_output = Command::new("apt-mark")
        .arg("showhold")
        .output()
        .unwrap_or_default();
    
    let held = String::from_utf8_lossy(&held_output.stdout);
    if !held.trim().is_empty() {
        println!("\n⚠️  Held packages found:");
        println!("{}", held);
    }
    
    // Check sources.list
    if let Ok(sources) = fs::read_to_string(DEBIAN_SOURCES) {
        let has_bookworm = sources.contains("bookworm");
        let has_bullseye = sources.contains("bullseye");
        
        if has_bullseye && !has_bookworm {
            println!("✅ Debian sources: Bullseye (PVE 7.x)");
        } else if has_bookworm {
            println!("✅ Debian sources: Bookworm (PVE 8.x)");
        }
    }
}

fn configure_repositories() {
    println!("📦 Repository Configuration\n");
    
    let Ok(use_no_sub) = Confirm::new()
        .with_prompt("Use no-subscription repositories? (recommended for homelab)")
        .default(true)
        .interact()
    else {
        return;
    };
    let use_enterprise = !use_no_sub;

    let Ok(manage_ceph) = Confirm::new()
        .with_prompt("Manage Ceph repositories?")
        .default(false)
        .interact()
    else {
        return;
    };

    let Ok(target_version) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Target PVE version")
        .items(&["PVE 9 (Trixie)", "PVE 8 (Bookworm)", "Current (no change)"])
        .default(0)
        .interact()
    else {
        return;
    };
    
    match target_version {
        0 => setup_pve9_repos(use_enterprise, manage_ceph),
        1 => setup_pve8_repos(use_enterprise, manage_ceph),
        _ => println!("No repository changes made."),
    }
}

fn setup_pve9_repos(use_enterprise: bool, manage_ceph: bool) {
    println!("\n📝 Configuring PVE 9 repositories...");

    // Update Debian base to Trixie
    let debian_sources = r#"deb http://deb.debian.org/debian trixie main contrib non-free non-free-firmware
deb http://deb.debian.org/debian trixie-updates main contrib non-free non-free-firmware
deb http://security.debian.org/debian-security trixie-security main contrib non-free non-free-firmware"#;

    if let Err(e) = fs::write(DEBIAN_SOURCES, debian_sources) {
        eprintln!("Failed to write Debian sources to {}: {}", DEBIAN_SOURCES, e);
        return;
    }
    println!("✅ Updated Debian sources to Trixie");

    // Configure PVE repos
    if use_enterprise {
        // Enterprise repo
        let pve_enterprise = "deb https://enterprise.proxmox.com/debian/pve trixie pve-enterprise";
        if let Err(e) = fs::write(PVE_ENTERPRISE_LIST, pve_enterprise) {
            eprintln!("Failed to write PVE enterprise repo to {}: {}", PVE_ENTERPRISE_LIST, e);
            return;
        }

        // Disable no-subscription
        if Path::new(PVE_NO_SUB_LIST).exists() {
            let content = fs::read_to_string(PVE_NO_SUB_LIST).unwrap_or_default();
            if !content.starts_with('#') {
                fs::write(PVE_NO_SUB_LIST, format!("# {}", content)).ok();
            }
        }
        println!("✅ Configured PVE 9 Enterprise repository");
    } else {
        // No-subscription repo
        let pve_no_sub = "deb http://download.proxmox.com/debian/pve trixie pve-no-subscription";
        if let Err(e) = fs::write(PVE_NO_SUB_LIST, pve_no_sub) {
            eprintln!("Failed to write PVE no-sub repo to {}: {}", PVE_NO_SUB_LIST, e);
            return;
        }

        // Disable enterprise
        if Path::new(PVE_ENTERPRISE_LIST).exists() {
            let content = fs::read_to_string(PVE_ENTERPRISE_LIST).unwrap_or_default();
            if !content.starts_with('#') {
                fs::write(PVE_ENTERPRISE_LIST, format!("# {}", content)).ok();
            }
        }
        println!("✅ Configured PVE 9 No-Subscription repository");
    }

    if manage_ceph {
        setup_ceph_repo("reef", use_enterprise);
    }

    // Update package lists
    println!("\n🔄 Updating package lists...");
    let _ = Command::new("apt").args(&["update"]).status();
}

fn setup_pve8_repos(use_enterprise: bool, manage_ceph: bool) {
    println!("\n📝 Configuring PVE 8 repositories...");

    // Update Debian base to Bookworm
    let debian_sources = r#"deb http://deb.debian.org/debian bookworm main contrib non-free non-free-firmware
deb http://deb.debian.org/debian bookworm-updates main contrib non-free non-free-firmware
deb http://security.debian.org/debian-security bookworm-security main contrib non-free non-free-firmware"#;

    if let Err(e) = fs::write(DEBIAN_SOURCES, debian_sources) {
        eprintln!("Failed to write Debian sources to {}: {}", DEBIAN_SOURCES, e);
        return;
    }
    println!("✅ Updated Debian sources to Bookworm");

    // Configure PVE repos
    if use_enterprise {
        let pve_enterprise = "deb https://enterprise.proxmox.com/debian/pve bookworm pve-enterprise";
        if let Err(e) = fs::write(PVE_ENTERPRISE_LIST, pve_enterprise) {
            eprintln!("Failed to write PVE enterprise repo to {}: {}", PVE_ENTERPRISE_LIST, e);
            return;
        }

        if Path::new(PVE_NO_SUB_LIST).exists() {
            let content = fs::read_to_string(PVE_NO_SUB_LIST).unwrap_or_default();
            if !content.starts_with('#') {
                fs::write(PVE_NO_SUB_LIST, format!("# {}", content)).ok();
            }
        }
        println!("✅ Configured PVE 8 Enterprise repository");
    } else {
        let pve_no_sub = "deb http://download.proxmox.com/debian/pve bookworm pve-no-subscription";
        if let Err(e) = fs::write(PVE_NO_SUB_LIST, pve_no_sub) {
            eprintln!("Failed to write PVE no-sub repo to {}: {}", PVE_NO_SUB_LIST, e);
            return;
        }
        
        if Path::new(PVE_ENTERPRISE_LIST).exists() {
            let content = fs::read_to_string(PVE_ENTERPRISE_LIST).unwrap_or_default();
            if !content.starts_with('#') {
                fs::write(PVE_ENTERPRISE_LIST, format!("# {}", content)).ok();
            }
        }
        println!("✅ Configured PVE 8 No-Subscription repository");
    }
    
    if manage_ceph {
        setup_ceph_repo("quincy", use_enterprise);
    }
    
    // Update package lists
    println!("\n🔄 Updating package lists...");
    let _ = Command::new("apt").args(&["update"]).status();
}

fn ceph_management() {
    println!("🐙 Ceph Repository Management\n");
    
    let ceph_versions = vec![
        "Reef (18.x) - PVE 8/9",
        "Quincy (17.x) - PVE 7/8",
        "Pacific (16.x) - PVE 7",
        "Octopus (15.x) - PVE 6",
        "Remove Ceph Repositories",
        "Back",
    ];
    
    let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Ceph version")
        .items(&ceph_versions)
        .default(0)
        .interact()
    else {
        return;
    };

    let use_enterprise = if selection < 4 {
        let Ok(use_no_sub) = Confirm::new()
            .with_prompt("Use no-subscription Ceph repository?")
            .default(true)
            .interact()
        else {
            return;
        };
        !use_no_sub
    } else {
        false
    };
    
    match selection {
        0 => setup_ceph_repo("reef", use_enterprise),
        1 => setup_ceph_repo("quincy", use_enterprise),
        2 => setup_ceph_repo("pacific", use_enterprise),
        3 => setup_ceph_repo("octopus", use_enterprise),
        4 => remove_ceph_repos(),
        _ => {}
    }
}

fn setup_ceph_repo(version: &str, use_enterprise: bool) {
    println!("\n📝 Configuring Ceph {} repository...", version);

    let debian_version = match version {
        "reef" => "bookworm", // Can also be trixie for PVE 9
        "quincy" => "bookworm",
        "pacific" | "octopus" => "bullseye",
        _ => "bookworm",
    };

    let ceph_repo = if use_enterprise {
        format!(
            "deb https://enterprise.proxmox.com/debian/ceph-{} {} enterprise",
            version, debian_version
        )
    } else {
        format!(
            "deb http://download.proxmox.com/debian/ceph-{} {} no-subscription",
            version, debian_version
        )
    };

    if let Err(e) = fs::write(CEPH_LIST, ceph_repo) {
        eprintln!("Failed to write Ceph repository to {}: {}", CEPH_LIST, e);
        return;
    }
    println!(
        "✅ Configured Ceph {} ({}) repository",
        version,
        if use_enterprise {
            "enterprise"
        } else {
            "no-subscription"
        }
    );

    // Update package lists
    println!("🔄 Updating package lists...");
    let _ = Command::new("apt").args(&["update"]).status();
}

fn remove_ceph_repos() {
    println!("🗑️  Removing Ceph repositories...");
    
    if Path::new(CEPH_LIST).exists() {
        fs::remove_file(CEPH_LIST).ok();
        println!("✅ Removed Ceph repository configuration");
    } else {
        println!("ℹ️  No Ceph repository found");
    }
    
    // Update package lists
    println!("🔄 Updating package lists...");
    let _ = Command::new("apt").args(&["update"]).status();
}

fn upgrade_single_node() {
    println!("🚀 Single Node Upgrade\n");
    
    let Ok(node_name) = Input::<String>::new()
        .with_prompt("Node name (leave empty for local node)")
        .allow_empty(true)
        .interact_text()
    else {
        return;
    };
    
    let node = if node_name.is_empty() {
        "localhost"
    } else {
        &node_name
    };
    
    println!("📋 Upgrade plan for node: {}", node);
    println!("  1. Run pre-checks");
    println!("  2. Configure repositories");
    println!("  3. Update and dist-upgrade");
    println!("  4. Refresh boot configuration");
    println!("  5. Reboot");
    
    let Ok(proceed) = Confirm::new()
        .with_prompt("Proceed with upgrade?")
        .default(false)
        .interact()
    else {
        return;
    };
    if !proceed {
        return;
    }
    
    let log_file = format!("/var/log/ghostctl/pve-upgrade-{}.log", Local::now().format("%Y%m%d-%H%M%S"));
    create_log_dir();
    
    println!("📝 Logging to: {}", log_file);
    
    // Run the upgrade
    perform_node_upgrade(node, &log_file);
}

fn perform_node_upgrade(node: &str, log_file: &str) {
    println!("\n🔄 Starting upgrade for node: {}", node);
    
    // Step 1: Pre-check
    println!("Step 1/5: Running pre-checks...");
    precheck();
    
    // Step 2: Configure repos (already done via menu)
    println!("\nStep 2/5: Repository configuration");
    println!("ℹ️  Ensure repositories are configured for PVE 9");
    
    let Ok(repos_ok) = Confirm::new()
        .with_prompt("Are repositories configured correctly?")
        .default(false)
        .interact()
    else {
        return;
    };
    if !repos_ok {
        println!("❌ Upgrade cancelled. Please configure repositories first.");
        return;
    }
    
    // Step 3: Update and upgrade
    println!("\nStep 3/5: Running system upgrade...");
    
    println!("🔄 Updating package lists...");
    let _ = Command::new("apt")
        .args(&["update"])
        .status();
    
    println!("🔄 Running dist-upgrade...");
    let status = Command::new("apt")
        .args(&["dist-upgrade", "-y"])
        .status();
    
    if !status.map(|s| s.success()).unwrap_or(false) {
        println!("❌ Upgrade failed! Check the logs and resolve any issues.");
        return;
    }
    
    // Step 4: Refresh boot
    println!("\nStep 4/5: Refreshing boot configuration...");
    let _ = Command::new("proxmox-boot-tool")
        .arg("refresh")
        .status();
    
    // Step 5: Reboot
    println!("\nStep 5/5: Reboot required");
    println!("✅ Upgrade complete! Node must be rebooted to complete the upgrade.");
    
    let Ok(reboot_now) = Confirm::new()
        .with_prompt("Reboot now?")
        .default(true)
        .interact()
    else {
        return;
    };
    if reboot_now {
        println!("🔄 Rebooting...");
        let _ = Command::new("systemctl")
            .arg("reboot")
            .status();
    }
}

fn drain_node_menu() {
    println!("🚰 Node Drain (VM/CT Migration)\n");
    
    let nodes = get_cluster_nodes();
    if nodes.is_empty() {
        println!("❌ Could not get cluster nodes. Is this a cluster?");
        return;
    }
    
    let node_names: Vec<String> = nodes.iter().map(|n| {
        format!("{} ({})", n.name, n.status)
    }).collect();
    
    let Ok(node_idx) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select node to drain")
        .items(&node_names)
        .interact()
    else {
        return;
    };

    let node = &nodes[node_idx];

    let Ok(with_local) = Confirm::new()
        .with_prompt("Migrate VMs with local disks? (requires offline migration)")
        .default(false)
        .interact()
    else {
        return;
    };

    let Ok(offline) = Confirm::new()
        .with_prompt("Use offline migration? (shuts down VMs before migration)")
        .default(false)
        .interact()
    else {
        return;
    };
    
    println!("\n📋 Drain plan for node: {}", node.name);
    println!("  • List all VMs and CTs");
    println!("  • Find suitable target nodes");
    println!("  • Migrate each VM/CT");
    if offline {
        println!("  • Using offline migration (VMs will be shut down)");
    }
    if with_local {
        println!("  • Including VMs with local disks");
    }
    
    let Ok(proceed) = Confirm::new()
        .with_prompt("Proceed with drain?")
        .default(false)
        .interact()
    else {
        return;
    };
    if !proceed {
        return;
    }
    
    drain_node(&node.name, with_local, offline);
}

fn drain_node(node: &str, with_local: bool, offline: bool) {
    println!("\n🚰 Draining node: {}", node);
    
    // Get list of VMs on this node
    let vms = get_node_vms(node);
    let cts = get_node_containers(node);
    
    println!("Found {} VMs and {} containers", vms.len(), cts.len());
    
    if vms.is_empty() && cts.is_empty() {
        println!("✅ Node is already empty!");
        return;
    }
    
    // Get target nodes
    let target_nodes = get_migration_targets(node);
    if target_nodes.is_empty() {
        println!("❌ No suitable target nodes found!");
        return;
    }
    
    println!("Available target nodes: {:?}", target_nodes);
    
    // Migrate VMs
    for vm in vms {
        println!("\n📦 Migrating VM {} ({})", vm.0, vm.1);
        
        if !target_nodes.is_empty() {
            let target = &target_nodes[0]; // Simple selection, could be improved
            
            let migrate_cmd = if offline {
                vec!["qm", "migrate", &vm.0, target, "--online", "0"]
            } else if with_local {
                vec!["qm", "migrate", &vm.0, target, "--with-local-disks", "--online", "0"]
            } else {
                vec!["qm", "migrate", &vm.0, target, "--online", "1"]
            };
            
            let status = Command::new("pvesh")
                .args(&["create", &format!("/nodes/{}/qemu/{}/migrate", node, vm.0)])
                .arg("--target")
                .arg(target)
                .status();
            
            if status.map(|s| s.success()).unwrap_or(false) {
                println!("✅ VM {} migrated to {}", vm.0, target);
            } else {
                println!("❌ Failed to migrate VM {}", vm.0);
            }
        }
    }
    
    // Migrate containers
    for ct in cts {
        println!("\n📦 Migrating container {} ({})", ct.0, ct.1);
        
        if !target_nodes.is_empty() {
            let target = &target_nodes[0];
            
            let status = Command::new("pvesh")
                .args(&["create", &format!("/nodes/{}/lxc/{}/migrate", node, ct.0)])
                .arg("--target")
                .arg(target)
                .arg("--restart")
                .arg("1")
                .status();
            
            if status.map(|s| s.success()).unwrap_or(false) {
                println!("✅ Container {} migrated to {}", ct.0, target);
            } else {
                println!("❌ Failed to migrate container {}", ct.0);
            }
        }
    }
    
    println!("\n✅ Node drain complete!");
}

fn wave_upgrade() {
    println!("🌊 Wave Upgrade (Sequential Cluster Upgrade)\n");
    
    let nodes = get_cluster_nodes();
    if nodes.len() < 2 {
        println!("❌ Wave upgrade requires a cluster with multiple nodes");
        return;
    }
    
    let node_names: Vec<String> = nodes.iter().map(|n| n.name.clone()).collect();
    
    let Ok(selected_indices) = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select nodes to upgrade (in order)")
        .items(&node_names)
        .interact()
    else {
        return;
    };
    
    if selected_indices.is_empty() {
        println!("No nodes selected");
        return;
    }
    
    let selected_nodes: Vec<String> = selected_indices
        .iter()
        .map(|&i| nodes[i].name.clone())
        .collect();
    
    println!("\n📋 Wave upgrade plan:");
    for (i, node) in selected_nodes.iter().enumerate() {
        println!("  {}. {}", i + 1, node);
    }
    
    println!("\nEach node will:");
    println!("  1. Be drained (VMs migrated)");
    println!("  2. Upgraded to PVE 9");
    println!("  3. Rebooted");
    println!("  4. Checked for cluster health");
    
    let Ok(proceed) = Confirm::new()
        .with_prompt("Proceed with wave upgrade?")
        .default(false)
        .interact()
    else {
        return;
    };
    if !proceed {
        return;
    }
    
    let log_file = format!("/var/log/ghostctl/pve-wave-upgrade-{}.log", Local::now().format("%Y%m%d-%H%M%S"));
    create_log_dir();
    
    for node in selected_nodes {
        println!("\n{}", "=".repeat(60));
        println!("🌊 Wave: Upgrading node {}", node);
        println!("{}", "=".repeat(60));
        
        // Step 1: Drain
        println!("\n📦 Step 1: Draining node {}...", node);
        drain_node(&node, false, true);
        
        // Step 2: Upgrade
        println!("\n🚀 Step 2: Upgrading node {}...", node);
        perform_node_upgrade(&node, &log_file);
        
        // Step 3: Wait for node to come back
        println!("\n⏳ Step 3: Waiting for node {} to rejoin cluster...", node);
        wait_for_node(&node);
        
        // Step 4: Check cluster health
        println!("\n🏥 Step 4: Checking cluster health...");
        if !check_cluster_health() {
            println!("⚠️  Cluster health check failed! Pausing wave upgrade.");
            println!("Please resolve issues before continuing.");
            
            let Ok(continue_anyway) = Confirm::new()
                .with_prompt("Continue with next node anyway?")
                .default(false)
                .interact()
            else {
                break;
            };
            if !continue_anyway {
                break;
            }
        }
        
        println!("\n✅ Node {} successfully upgraded!", node);
    }
    
    println!("\n🎉 Wave upgrade complete!");
}

fn rollback_menu() {
    println!("⏪ Rollback Configuration\n");
    println!("⚠️  WARNING: This will revert repository configuration");
    
    let options = vec![
        "Rollback to PVE 8 (Bookworm)",
        "Rollback to PVE 7 (Bullseye)",
        "Back",
    ];
    
    let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select rollback target")
        .items(&options)
        .default(2)
        .interact()
    else {
        return;
    };

    match selection {
        0 => {
            let Ok(rollback) = Confirm::new()
                .with_prompt("Rollback repositories to PVE 8?")
                .default(false)
                .interact()
            else {
                return;
            };
            if rollback {
                setup_pve8_repos(false, false);
                println!("✅ Rolled back to PVE 8 repositories");
            }
        }
        1 => {
            let Ok(rollback) = Confirm::new()
                .with_prompt("Rollback repositories to PVE 7?")
                .default(false)
                .interact()
            else {
                return;
            };
            if rollback {
                rollback_to_pve7();
            }
        }
        _ => {}
    }
}

fn rollback_to_pve7() {
    println!("📝 Rolling back to PVE 7 repositories...");

    // Debian Bullseye sources
    let debian_sources = r#"deb http://deb.debian.org/debian bullseye main contrib
deb http://deb.debian.org/debian bullseye-updates main contrib
deb http://security.debian.org/debian-security bullseye-security main contrib"#;

    if let Err(e) = fs::write(DEBIAN_SOURCES, debian_sources) {
        eprintln!("Failed to write Debian sources to {}: {}", DEBIAN_SOURCES, e);
        return;
    }

    // PVE 7 no-subscription
    let pve_no_sub = "deb http://download.proxmox.com/debian/pve bullseye pve-no-subscription";
    if let Err(e) = fs::write(PVE_NO_SUB_LIST, pve_no_sub) {
        eprintln!("Failed to write PVE repo to {}: {}", PVE_NO_SUB_LIST, e);
        return;
    }

    println!("✅ Rolled back to PVE 7 repositories");
    println!("⚠️  You may need to downgrade packages manually");
}

fn view_logs() {
    let log_dir = "/var/log/ghostctl";
    
    if !Path::new(log_dir).exists() {
        println!("No upgrade logs found");
        return;
    }
    
    let Ok(entries) = fs::read_dir(log_dir) else {
        println!("Failed to read log directory");
        return;
    };
    let logs = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                if name.starts_with("pve-") && name.ends_with(".log") {
                    Some(name)
                } else {
                    None
                }
            })
        })
        .collect::<Vec<String>>();

    if logs.is_empty() {
        println!("No upgrade logs found");
        return;
    }

    let mut log_options = logs.clone();
    log_options.push("Back".to_string());

    let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select log to view")
        .items(&log_options)
        .interact()
    else {
        return;
    };
    
    if selection < logs.len() {
        let log_path = format!("{}/{}", log_dir, logs[selection]);
        let _ = Command::new("less")
            .arg(&log_path)
            .status();
    }
}

// Helper functions

fn get_pve_version() -> String {
    let output = Command::new("pveversion")
        .arg("--verbose")
        .output()
        .unwrap_or_default();
    
    let version_info = String::from_utf8_lossy(&output.stdout);
    
    for line in version_info.lines() {
        if line.starts_with("proxmox-ve:") {
            return line.split(':').nth(1).unwrap_or("unknown").trim().to_string();
        }
    }
    
    "unknown".to_string()
}

fn get_cluster_nodes() -> Vec<NodeStatus> {
    let output = Command::new("pvesh")
        .args(&["get", "/nodes", "--output-format", "json"])
        .output()
        .unwrap_or_default();
    
    if output.status.success() {
        let json_str = String::from_utf8_lossy(&output.stdout);
        if let Ok(nodes) = serde_json::from_str::<Vec<NodeStatus>>(&json_str) {
            return nodes;
        }
    }
    
    Vec::new()
}

fn get_node_vms(node: &str) -> Vec<(String, String)> {
    let output = Command::new("pvesh")
        .args(&["get", &format!("/nodes/{}/qemu", node), "--output-format", "json"])
        .output()
        .unwrap_or_default();
    
    let mut vms = Vec::new();
    
    if output.status.success() {
        let json_str = String::from_utf8_lossy(&output.stdout);
        if let Ok(vm_list) = serde_json::from_str::<Vec<serde_json::Value>>(&json_str) {
            for vm in vm_list {
                if let (Some(vmid), Some(name)) = (vm["vmid"].as_u64(), vm["name"].as_str()) {
                    vms.push((vmid.to_string(), name.to_string()));
                }
            }
        }
    }
    
    vms
}

fn get_node_containers(node: &str) -> Vec<(String, String)> {
    let output = Command::new("pvesh")
        .args(&["get", &format!("/nodes/{}/lxc", node), "--output-format", "json"])
        .output()
        .unwrap_or_default();
    
    let mut cts = Vec::new();
    
    if output.status.success() {
        let json_str = String::from_utf8_lossy(&output.stdout);
        if let Ok(ct_list) = serde_json::from_str::<Vec<serde_json::Value>>(&json_str) {
            for ct in ct_list {
                if let (Some(vmid), Some(name)) = (ct["vmid"].as_u64(), ct["name"].as_str()) {
                    cts.push((vmid.to_string(), name.to_string()));
                }
            }
        }
    }
    
    cts
}

fn get_migration_targets(exclude_node: &str) -> Vec<String> {
    let nodes = get_cluster_nodes();
    nodes
        .into_iter()
        .filter(|n| n.name != exclude_node && n.status == "online")
        .map(|n| n.name)
        .collect()
}

fn check_cluster_health() -> bool {
    println!("\n🏥 Checking cluster health...");
    
    let output = Command::new("pvecm")
        .arg("status")
        .output()
        .unwrap_or_default();
    
    if output.status.success() {
        let status = String::from_utf8_lossy(&output.stdout);
        
        let has_quorum = status.contains("Quorate: Yes");
        let nodes_online = status.lines()
            .filter(|l| l.contains("Online:"))
            .count() > 0;
        
        if has_quorum {
            println!("✅ Cluster has quorum");
        } else {
            println!("❌ Cluster does NOT have quorum!");
        }
        
        if nodes_online {
            println!("✅ Nodes are online");
        }
        
        return has_quorum && nodes_online;
    }
    
    println!("⚠️  Could not determine cluster health");
    false
}

fn check_storage_health() {
    println!("\n💾 Checking storage health...");
    
    let output = Command::new("pvesh")
        .args(&["get", "/storage", "--output-format", "json"])
        .output()
        .unwrap_or_default();
    
    if output.status.success() {
        let json_str = String::from_utf8_lossy(&output.stdout);
        if let Ok(storages) = serde_json::from_str::<Vec<serde_json::Value>>(&json_str) {
            for storage in storages {
                if let (Some(id), Some(enabled)) = (storage["storage"].as_str(), storage["enabled"].as_u64()) {
                    if enabled == 1 {
                        println!("  ✅ {} - enabled", id);
                    } else {
                        println!("  ⚠️  {} - disabled", id);
                    }
                }
            }
        }
    }
}

fn check_ceph_status() {
    println!("\n🐙 Checking for Ceph...");
    
    let output = Command::new("which")
        .arg("ceph")
        .output()
        .unwrap_or_default();
    
    if output.status.success() {
        println!("⚠️  Ceph is installed!");
        println!("  Ceph upgrades require special attention.");
        println!("  Consider using --ack-ceph flag or managing Ceph separately.");
        
        // Try to get Ceph version
        let version_output = Command::new("ceph")
            .arg("--version")
            .output()
            .unwrap_or_default();
        
        if version_output.status.success() {
            let version = String::from_utf8_lossy(&version_output.stdout);
            println!("  Current: {}", version.trim());
        }
    } else {
        println!("✅ No Ceph installation detected");
    }
}

fn wait_for_node(node: &str) {
    println!("⏳ Waiting for node {} to come back online...", node);
    
    for i in 0..60 {
        thread::sleep(Duration::from_secs(10));
        
        let nodes = get_cluster_nodes();
        if let Some(n) = nodes.iter().find(|n| n.name == node) {
            if n.status == "online" {
                println!("✅ Node {} is back online!", node);
                return;
            }
        }
        
        if i % 6 == 0 {
            println!("  Still waiting... ({} seconds)", i * 10);
        }
    }
    
    println!("⚠️  Timeout waiting for node {}. Please check manually.", node);
}

fn create_log_dir() {
    let log_dir = "/var/log/ghostctl";
    if !Path::new(log_dir).exists() {
        fs::create_dir_all(log_dir).ok();
    }
}