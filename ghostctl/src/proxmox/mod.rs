use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::process::Command;

pub mod advanced_security;
pub mod backup_rotation;
pub mod enhanced;
pub mod firewall_automation;
pub mod helper;
pub mod script_safety;
pub mod storage_migration;
pub mod template_management;
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
            2 => vfio_gpu_passthrough_guide(),
            3 => pve_upgrade_guide(),
            4 => pbs_management_guide(),
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
    println!("\n📜 Proxmox Script Execution");
    println!("═══════════════════════════");

    match script_safety::safe_run_script(name, url) {
        Ok(true) => println!("✅ Script '{}' executed successfully.", name),
        Ok(false) => println!("⏭️  Script execution was cancelled or skipped."),
        Err(e) => println!("❌ Failed to run '{}': {}", name, e),
    }
}

fn fetch_and_run_script(url: &str) -> Result<(), String> {
    // Extract script name from URL
    let name = url
        .rsplit('/')
        .next()
        .unwrap_or("script")
        .trim_end_matches(".sh");

    match script_safety::safe_run_script(name, url) {
        Ok(true) => Ok(()),
        Ok(false) => Err("Script execution cancelled".to_string()),
        Err(e) => Err(format!("Script execution failed: {}", e)),
    }
}

fn vfio_gpu_passthrough_guide() {
    println!("🎮 VFIO GPU Passthrough Guide");
    println!("==============================");
    println!();
    println!("GPU passthrough allows you to dedicate a GPU to a virtual machine");
    println!("for near-native graphics performance.");
    println!();

    let options = [
        "📋 Check IOMMU status",
        "🔍 List PCI devices for passthrough",
        "📖 View setup guide",
        "🌐 Open Proxmox VFIO Wiki",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("VFIO Options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("\n🔍 IOMMU Status:");
            println!("================");
            let _ = std::process::Command::new("dmesg")
                .args(&["|", "grep", "-i", "iommu"])
                .status();

            // Check if IOMMU is enabled
            if let Ok(cmdline) = std::fs::read_to_string("/proc/cmdline") {
                if cmdline.contains("iommu=on")
                    || cmdline.contains("intel_iommu=on")
                    || cmdline.contains("amd_iommu=on")
                {
                    println!("✅ IOMMU appears to be enabled in kernel cmdline");
                } else {
                    println!("⚠️  IOMMU may not be enabled. Add to /etc/default/grub:");
                    println!("   Intel: intel_iommu=on iommu=pt");
                    println!("   AMD: amd_iommu=on iommu=pt");
                }
            }
        }
        1 => {
            println!("\n📋 PCI Devices for Passthrough:");
            println!("================================");
            let _ = std::process::Command::new("lspci").args(&["-nn"]).status();

            println!("\n💡 Look for your GPU (usually starts with 'VGA' or 'Display')");
            println!("   Note the device IDs in format [XXXX:XXXX]");
        }
        2 => {
            println!("\n📖 VFIO GPU Passthrough Setup Guide");
            println!("====================================");
            println!();
            println!("1. Enable IOMMU in BIOS/UEFI (VT-d for Intel, AMD-Vi for AMD)");
            println!();
            println!("2. Add kernel parameters in /etc/default/grub:");
            println!("   GRUB_CMDLINE_LINUX_DEFAULT=\"quiet intel_iommu=on iommu=pt\"");
            println!("   (or amd_iommu=on for AMD)");
            println!();
            println!("3. Update GRUB: update-grub && reboot");
            println!();
            println!("4. Add vfio modules to /etc/modules:");
            println!("   vfio");
            println!("   vfio_iommu_type1");
            println!("   vfio_pci");
            println!("   vfio_virqfd");
            println!();
            println!("5. Blacklist GPU drivers in /etc/modprobe.d/blacklist.conf:");
            println!("   blacklist nvidia");
            println!("   blacklist nouveau");
            println!("   blacklist radeon");
            println!("   blacklist amdgpu");
            println!();
            println!("6. Add GPU to vfio-pci in /etc/modprobe.d/vfio.conf:");
            println!("   options vfio-pci ids=XXXX:XXXX,XXXX:XXXX");
            println!();
            println!("7. Update initramfs: update-initramfs -u -k all && reboot");
            println!();
            println!("8. Add PCI device to VM in Proxmox web interface");
        }
        3 => {
            println!("🌐 Opening Proxmox VFIO Wiki...");
            let _ = std::process::Command::new("xdg-open")
                .arg("https://pve.proxmox.com/wiki/PCI_Passthrough")
                .status();
        }
        _ => return,
    }
}

fn pve_upgrade_guide() {
    println!("🚀 Proxmox VE Upgrade Guide");
    println!("===========================");
    println!();

    let options = [
        "📊 Check current PVE version",
        "🔍 Check upgrade readiness",
        "📖 View upgrade guide (8→9)",
        "🌐 Open official upgrade docs",
        "🔧 Run community upgrade script",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("PVE Upgrade Options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("\n📊 Current Proxmox VE Version:");
            let _ = std::process::Command::new("pveversion")
                .arg("--verbose")
                .status();
        }
        1 => {
            println!("\n🔍 Checking Upgrade Readiness...");
            println!("================================");

            // Check Debian version
            println!("\nOS Version:");
            let _ = std::process::Command::new("cat")
                .arg("/etc/os-release")
                .status();

            // Check disk space
            println!("\nDisk Space:");
            let _ = std::process::Command::new("df").args(&["-h", "/"]).status();

            // Check cluster status
            println!("\nCluster Status:");
            let _ = std::process::Command::new("pvecm").arg("status").status();

            // Check for running VMs
            println!("\nRunning VMs/Containers:");
            let _ = std::process::Command::new("qm").arg("list").status();
            let _ = std::process::Command::new("pct").arg("list").status();

            println!("\n⚠️  Before upgrading:");
            println!("   1. Backup all VMs and important data");
            println!("   2. Ensure all nodes in cluster are on same version");
            println!("   3. Stop non-essential VMs/containers");
            println!("   4. Review release notes for breaking changes");
        }
        2 => {
            println!("\n📖 PVE 8 to 9 Upgrade Guide");
            println!("===========================");
            println!();
            println!("1. Update current PVE 8 to latest:");
            println!("   apt update && apt full-upgrade");
            println!();
            println!("2. Run pre-upgrade checklist:");
            println!("   pve8to9 --full");
            println!();
            println!("3. Update sources.list for PVE 9:");
            println!("   sed -i 's/bookworm/trixie/g' /etc/apt/sources.list");
            println!();
            println!("4. Update Proxmox repo:");
            println!("   sed -i 's/pve-no-subscription/pve-no-subscription/g' \\");
            println!("       /etc/apt/sources.list.d/pve-no-subscription.list");
            println!();
            println!("5. Perform upgrade:");
            println!("   apt update && apt full-upgrade");
            println!();
            println!("6. Reboot the node:");
            println!("   reboot");
            println!();
            println!("⚠️  Always backup before upgrading!");
        }
        3 => {
            println!("🌐 Opening Proxmox upgrade documentation...");
            let _ = std::process::Command::new("xdg-open")
                .arg("https://pve.proxmox.com/wiki/Upgrade")
                .status();
        }
        4 => {
            println!("\n🔧 Community PVE Upgrade Script");
            println!("================================");

            let confirm = Confirm::new()
                .with_prompt("Run community-scripts PVE upgrade tool?")
                .default(false)
                .interact()
                .unwrap();

            if confirm {
                let url = "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/tools/pve/pve8-upgrade.sh";
                confirm_and_run_script("PVE Upgrade Script", url);
            }
        }
        _ => return,
    }
}

fn pbs_management_guide() {
    println!("🔐 Proxmox Backup Server Management");
    println!("====================================");
    println!();

    let options = [
        "📊 Check PBS status",
        "📋 List datastores",
        "🔧 Run PBS maintenance",
        "📖 View PBS setup guide",
        "🌐 Open PBS documentation",
        "🔧 Run community PBS script",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("PBS Options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("\n📊 PBS Service Status:");
            let _ = std::process::Command::new("systemctl")
                .args(&["status", "proxmox-backup"])
                .status();
        }
        1 => {
            println!("\n📋 PBS Datastores:");
            let _ = std::process::Command::new("proxmox-backup-manager")
                .args(&["datastore", "list"])
                .status();
        }
        2 => {
            println!("\n🔧 PBS Maintenance Tasks:");
            println!("=========================");

            let tasks = [
                "🗑️  Garbage collection",
                "✅ Verify datastore",
                "📊 Show datastore status",
                "⬅️  Back",
            ];

            let task = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select maintenance task")
                .items(&tasks)
                .default(0)
                .interact()
                .unwrap();

            match task {
                0 => {
                    let ds: String = Input::new()
                        .with_prompt("Datastore name")
                        .interact_text()
                        .unwrap();

                    let _ = std::process::Command::new("proxmox-backup-manager")
                        .args(&["garbage-collection", "start", &ds])
                        .status();
                }
                1 => {
                    let ds: String = Input::new()
                        .with_prompt("Datastore name")
                        .interact_text()
                        .unwrap();

                    let _ = std::process::Command::new("proxmox-backup-manager")
                        .args(&["verify", "start", &ds])
                        .status();
                }
                2 => {
                    let ds: String = Input::new()
                        .with_prompt("Datastore name")
                        .interact_text()
                        .unwrap();

                    let _ = std::process::Command::new("proxmox-backup-manager")
                        .args(&["datastore", "show", &ds])
                        .status();
                }
                _ => return,
            }
        }
        3 => {
            println!("\n📖 PBS Setup Guide");
            println!("==================");
            println!();
            println!("1. Install PBS on a dedicated machine/VM:");
            println!("   Download ISO from https://www.proxmox.com/en/downloads");
            println!();
            println!("2. Create a datastore:");
            println!("   proxmox-backup-manager datastore create <name> <path>");
            println!();
            println!("3. Configure PVE to use PBS:");
            println!("   - Datacenter > Storage > Add > Proxmox Backup Server");
            println!("   - Enter PBS hostname, fingerprint, and credentials");
            println!();
            println!("4. Create backup jobs in PVE:");
            println!("   - Datacenter > Backup > Add");
            println!("   - Select PBS storage and VMs to backup");
            println!();
            println!("5. Configure retention policy in PBS:");
            println!("   - PBS web UI > Datastore > Prune & GC");
        }
        4 => {
            println!("🌐 Opening PBS documentation...");
            let _ = std::process::Command::new("xdg-open")
                .arg("https://pbs.proxmox.com/docs/")
                .status();
        }
        5 => {
            println!("\n🔧 Community PBS Install Script");

            let confirm = Confirm::new()
                .with_prompt("Run community PBS install script?")
                .default(false)
                .interact()
                .unwrap();

            if confirm {
                let url = "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/misc/pbs-install.sh";
                confirm_and_run_script("PBS Install Script", url);
            }
        }
        _ => return,
    }
}
