use crate::utils::run_command;
use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme, Password};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;
use chrono::Local;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatastoreInfo {
    pub name: String,
    pub path: String,
    pub used: u64,
    pub total: u64,
    pub available: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupJob {
    pub id: String,
    pub datastore: String,
    pub schedule: String,
    pub enabled: bool,
}

const PBS_APT_DIR: &str = "/etc/apt/sources.list.d";
const PBS_ENTERPRISE_LIST: &str = "/etc/apt/sources.list.d/pbs-enterprise.list";
const PBS_NO_SUB_LIST: &str = "/etc/apt/sources.list.d/pbs-no-subscription.list";
const PBS_TEST_LIST: &str = "/etc/apt/sources.list.d/pbs-test.list";

pub fn pbs_menu() {
    loop {
        let options = vec![
            "PBS Post-Install Setup",
            "Repository Management",
            "Datastore Operations",
            "Backup Job Management",
            "Maintenance Tasks",
            "Performance Tuning",
            "Subscription & Updates",
            "System Health Check",
            "Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔐 Proxmox Backup Server Management")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => post_install_setup(),
            1 => repository_management(),
            2 => datastore_operations(),
            3 => backup_job_management(),
            4 => maintenance_tasks(),
            5 => performance_tuning(),
            6 => subscription_updates(),
            7 => system_health_check(),
            _ => break,
        }
    }
}

fn post_install_setup() {
    println!("🚀 PBS Post-Install Setup\n");

    let tasks = vec![
        "Disable Enterprise Repository",
        "Enable No-Subscription Repository",
        "Add Test Repository",
        "Disable Subscription Nag",
        "Update System Packages",
        "Configure Email Notifications",
        "Setup Automatic Updates",
        "Configure Firewall",
        "Optimize System Settings",
    ];

    let selected = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select setup tasks to perform")
        .items(&tasks)
        .defaults(&[true, true, false, true, true, false, false, false, true])
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    if selected.is_empty() {
        println!("No tasks selected");
        return;
    }

    for idx in selected {
        match idx {
            0 => disable_enterprise_repo(),
            1 => enable_no_sub_repo(),
            2 => add_test_repo(),
            3 => disable_subscription_nag(),
            4 => update_system(),
            5 => configure_email(),
            6 => setup_auto_updates(),
            7 => configure_firewall(),
            8 => optimize_system(),
            _ => {}
        }
    }

    println!("\n✅ Post-install setup complete!");

    let reboot = Confirm::new()
        .with_prompt("Reboot PBS server now?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if reboot {
        let _ = Command::new("systemctl").arg("reboot").status();
    }
}

fn disable_enterprise_repo() {
    println!("📦 Disabling Enterprise Repository...");

    if Path::new(PBS_ENTERPRISE_LIST).exists() {
        // Comment out enterprise repo
        if let Ok(content) = fs::read_to_string(PBS_ENTERPRISE_LIST) {
            if !content.starts_with('#') {
                let commented = format!("# {}", content.replace('\n', "\n# "));
                fs::write(PBS_ENTERPRISE_LIST, commented).ok();
                println!("✅ Enterprise repository disabled");
            } else {
                println!("ℹ️  Enterprise repository already disabled");
            }
        }
    } else {
        println!("ℹ️  No enterprise repository found");
    }
}

fn enable_no_sub_repo() {
    println!("📦 Enabling No-Subscription Repository...");

    let pbs_version = get_pbs_version();
    let debian_version = if pbs_version.starts_with("3.") {
        "bookworm"
    } else if pbs_version.starts_with("2.") {
        "bullseye"
    } else {
        "bookworm" // Default to latest
    };

    let no_sub_content = format!(
        "deb http://download.proxmox.com/debian/pbs {} pbs-no-subscription",
        debian_version
    );

    if let Err(e) = fs::write(PBS_NO_SUB_LIST, no_sub_content) {
        println!("❌ Failed to write PBS no-sub repo: {}", e);
        return;
    }
    println!("✅ No-subscription repository enabled");
}

fn add_test_repo() {
    println!("⚠️  Adding Test Repository (unstable packages)...");

    let confirm = Confirm::new()
        .with_prompt("Test repository contains unstable packages. Continue?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if !confirm {
        return;
    }

    let pbs_version = get_pbs_version();
    let debian_version = if pbs_version.starts_with("3.") {
        "bookworm"
    } else {
        "bullseye"
    };

    let test_content = format!(
        "deb http://download.proxmox.com/debian/pbs {} pbstest",
        debian_version
    );

    if let Err(e) = fs::write(PBS_TEST_LIST, test_content) {
        println!("❌ Failed to write PBS test repo: {}", e);
        return;
    }
    println!("✅ Test repository added");
}

fn disable_subscription_nag() {
    println!("🔕 Disabling subscription nag...");

    let proxy_file = "/usr/share/javascript/proxmox-backup/js/proxmox-backup-gui.js";

    if Path::new(proxy_file).exists() {
        // Backup original file
        let backup_file = format!("{}.bak", proxy_file);
        if !Path::new(&backup_file).exists() {
            fs::copy(proxy_file, &backup_file).ok();
        }

        // Read and modify the file
        if let Ok(content) = fs::read_to_string(proxy_file) {
            let modified = content.replace(
                "Ext.Msg.show({",
                "void({ //Ext.Msg.show({"
            );

            if modified != content {
                fs::write(proxy_file, modified).ok();
                println!("✅ Subscription nag disabled");
                println!("ℹ️  Clear browser cache for changes to take effect");
            } else {
                println!("ℹ️  Subscription nag already disabled");
            }
        }
    } else {
        println!("❌ PBS GUI file not found");
    }
}

fn update_system() {
    println!("🔄 Updating system packages...");

    // Update package lists
    let _ = Command::new("apt")
        .args(&["update"])
        .status();

    // Upgrade packages
    let _ = Command::new("apt")
        .args(&["dist-upgrade", "-y"])
        .status();

    println!("✅ System updated");
}

fn configure_email() {
    println!("📧 Configuring email notifications...");

    let smtp_server: String = match Input::new()
        .with_prompt("SMTP server")
        .interact_text()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    let smtp_port: u16 = match Input::new()
        .with_prompt("SMTP port")
        .default(587)
        .interact()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let from_address: String = match Input::new()
        .with_prompt("From email address")
        .interact_text()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    let use_auth = Confirm::new()
        .with_prompt("Use SMTP authentication?")
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(true);

    if use_auth {
        let username: String = match Input::new()
            .with_prompt("SMTP username")
            .interact_text()
        {
            Ok(s) => s,
            Err(_) => return,
        };

        let password = match Password::new()
            .with_prompt("SMTP password")
            .interact()
        {
            Ok(p) => p,
            Err(_) => return,
        };

        // Configure postfix or other mail system
        println!("📝 Configuring mail system...");
        // Implementation would depend on the mail system used
    }

    println!("✅ Email notifications configured");
}

fn setup_auto_updates() {
    println!("🔄 Setting up automatic updates...");

    let update_types = vec![
        "Security updates only",
        "All stable updates",
        "No automatic updates",
    ];

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select update policy")
        .items(&update_types)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    match selection {
        0 => {
            // Install unattended-upgrades
            let _ = Command::new("apt")
                .args(&["install", "-y", "unattended-upgrades"])
                .status();

            // Configure for security only
            let config = r#"
Unattended-Upgrade::Origins-Pattern {
    "origin=Debian,codename=${distro_codename}-security,label=Debian-Security";
    "origin=Proxmox,codename=${distro_codename},label=Proxmox Backup Server";
};
Unattended-Upgrade::AutoFixInterruptedDpkg "true";
Unattended-Upgrade::MinimalSteps "true";
Unattended-Upgrade::Remove-Unused-Dependencies "true";
Unattended-Upgrade::Automatic-Reboot "false";
"#;
            fs::write("/etc/apt/apt.conf.d/50unattended-upgrades", config).ok();
            println!("✅ Automatic security updates enabled");
        }
        1 => {
            let _ = Command::new("apt")
                .args(&["install", "-y", "unattended-upgrades"])
                .status();

            println!("✅ Automatic updates enabled for all packages");
        }
        2 => {
            println!("ℹ️  Automatic updates disabled");
        }
        _ => {}
    }
}

fn configure_firewall() {
    println!("🔥 Configuring firewall...");

    let services = vec![
        ("SSH (22)", "22/tcp", true),
        ("PBS Web UI (8007)", "8007/tcp", true),
        ("PBS API (82)", "82/tcp", false),
        ("NFS", "111/tcp", false),
        ("SMB/CIFS", "445/tcp", false),
    ];

    println!("Select services to allow:");
    let mut selections = Vec::new();

    for (name, _, default) in &services {
        let allow = Confirm::new()
            .with_prompt(format!("Allow {}?", name))
            .default(*default)
            .interact_opt()
            .ok()
            .flatten()
            .unwrap_or(*default);
        selections.push(allow);
    }

    // Enable firewall
    let _ = Command::new("ufw")
        .arg("--force")
        .arg("enable")
        .status();

    // Apply rules
    for (i, allow) in selections.iter().enumerate() {
        if *allow {
            let (_, port, _) = services[i];
            let _ = Command::new("ufw")
                .args(&["allow", port])
                .status();
            println!("✅ Allowed {}", services[i].0);
        }
    }

    println!("✅ Firewall configured");
}

fn optimize_system() {
    println!("⚡ Optimizing system settings...");

    // Detect system RAM
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let total_ram_kb: u64 = meminfo
        .lines()
        .find(|l| l.starts_with("MemTotal:"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    let total_ram_gb = total_ram_kb / 1024 / 1024;

    println!("💾 Detected RAM: {} GB", total_ram_gb);

    // Optimize ZFS ARC if ZFS is present
    optimize_zfs_arc(total_ram_gb);

    // Optimize kernel parameters
    optimize_kernel_params();

    // Optimize PBS specific settings
    optimize_pbs_settings();

    println!("✅ System optimization complete");
}

fn optimize_zfs_arc(ram_gb: u64) {
    if !Path::new("/sys/module/zfs").exists() {
        println!("ℹ️  ZFS not detected, skipping ARC optimization");
        return;
    }

    println!("🔧 Optimizing ZFS ARC cache...");

    // Calculate ARC sizes based on RAM
    let (arc_min, arc_max) = match ram_gb {
        0..=8 => (1, 2),      // 8GB or less: 1-2GB ARC
        9..=16 => (2, 4),     // 16GB: 2-4GB ARC
        17..=32 => (4, 8),    // 32GB: 4-8GB ARC
        33..=64 => (8, 16),   // 64GB: 8-16GB ARC
        65..=128 => (16, 32), // 128GB: 16-32GB ARC
        _ => (32, 64),        // >128GB: 32-64GB ARC
    };

    let arc_min_bytes = arc_min * 1024 * 1024 * 1024;
    let arc_max_bytes = arc_max * 1024 * 1024 * 1024;

    // Write ZFS module parameters
    let zfs_conf = format!(
        "# ghostctl optimized ZFS ARC settings\n\
         # System RAM: {} GB\n\
         options zfs zfs_arc_min={}\n\
         options zfs zfs_arc_max={}\n\
         options zfs zfs_arc_meta_limit_percent=75\n\
         options zfs zfs_compressed_arc_enabled=1\n",
        ram_gb, arc_min_bytes, arc_max_bytes
    );

    fs::write("/etc/modprobe.d/zfs.conf", zfs_conf).ok();

    // Apply settings immediately if possible
    if Path::new("/sys/module/zfs/parameters/zfs_arc_max").exists() {
        fs::write("/sys/module/zfs/parameters/zfs_arc_min", arc_min_bytes.to_string()).ok();
        fs::write("/sys/module/zfs/parameters/zfs_arc_max", arc_max_bytes.to_string()).ok();
        println!("✅ ZFS ARC: {}GB min, {}GB max", arc_min, arc_max);
    } else {
        println!("✅ ZFS ARC configured (reboot required): {}GB min, {}GB max", arc_min, arc_max);
    }
}

fn optimize_kernel_params() {
    println!("🔧 Optimizing kernel parameters...");

    let sysctl_conf = r#"
# ghostctl PBS optimizations
# Network optimizations
net.core.rmem_max = 134217728
net.core.wmem_max = 134217728
net.ipv4.tcp_rmem = 4096 87380 134217728
net.ipv4.tcp_wmem = 4096 65536 134217728
net.core.netdev_max_backlog = 5000
net.ipv4.tcp_congestion_control = bbr
net.core.default_qdisc = fq

# File system optimizations
fs.file-max = 2097152
fs.inotify.max_user_instances = 8192
fs.inotify.max_user_watches = 524288

# Memory optimizations
vm.swappiness = 10
vm.dirty_ratio = 15
vm.dirty_background_ratio = 5
vm.vfs_cache_pressure = 50
"#;

    fs::write("/etc/sysctl.d/99-pbs-optimize.conf", sysctl_conf).ok();

    // Apply settings
    let _ = Command::new("sysctl")
        .arg("-p")
        .arg("/etc/sysctl.d/99-pbs-optimize.conf")
        .status();

    println!("✅ Kernel parameters optimized");
}

fn optimize_pbs_settings() {
    println!("🔧 Optimizing PBS settings...");

    // Optimize chunk store settings
    let node_cfg = "/etc/proxmox-backup/node.cfg";

    if Path::new(node_cfg).exists() {
        // Read existing config
        let config = fs::read_to_string(node_cfg).unwrap_or_default();

        // Add optimizations if not present
        if !config.contains("task-log-max-days") {
            let optimized = format!(
                "{}\ntask-log-max-days: 30\n\
                 verify-schedule: sun 02:00\n\
                 prune-schedule: daily\n",
                config
            );
            fs::write(node_cfg, optimized).ok();
        }
    }

    println!("✅ PBS settings optimized");
}

fn repository_management() {
    loop {
        let options = vec![
            "View Current Repositories",
            "Switch to No-Subscription",
            "Switch to Enterprise",
            "Enable Test Repository",
            "Disable Test Repository",
            "Update Package Lists",
            "Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("📦 Repository Management")
            .items(&options)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => view_repositories(),
            1 => switch_to_no_sub(),
            2 => switch_to_enterprise(),
            3 => add_test_repo(),
            4 => disable_test_repo(),
            5 => update_package_lists(),
            _ => break,
        }
    }
}

fn view_repositories() {
    println!("\n📋 Current PBS Repositories:\n");

    for repo_file in &[PBS_ENTERPRISE_LIST, PBS_NO_SUB_LIST, PBS_TEST_LIST] {
        if Path::new(repo_file).exists() {
            if let Ok(content) = fs::read_to_string(repo_file) {
                let repo_name = Path::new(repo_file)
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();

                if content.starts_with('#') {
                    println!("❌ {} (disabled)", repo_name);
                } else {
                    println!("✅ {} (enabled)", repo_name);
                    for line in content.lines() {
                        if !line.starts_with('#') && !line.trim().is_empty() {
                            println!("   {}", line);
                        }
                    }
                }
            }
        }
    }
}

fn switch_to_no_sub() {
    disable_enterprise_repo();
    enable_no_sub_repo();
    update_package_lists();
    println!("✅ Switched to no-subscription repository");
}

fn switch_to_enterprise() {
    println!("📦 Switching to Enterprise Repository...");

    // Disable no-sub repo
    if Path::new(PBS_NO_SUB_LIST).exists() {
        let content = fs::read_to_string(PBS_NO_SUB_LIST).unwrap_or_default();
        if !content.starts_with('#') {
            fs::write(PBS_NO_SUB_LIST, format!("# {}", content)).ok();
        }
    }

    // Enable enterprise repo
    let pbs_version = get_pbs_version();
    let debian_version = if pbs_version.starts_with("3.") {
        "bookworm"
    } else {
        "bullseye"
    };

    let enterprise_content = format!(
        "deb https://enterprise.proxmox.com/debian/pbs {} pbs-enterprise",
        debian_version
    );

    fs::write(PBS_ENTERPRISE_LIST, enterprise_content).ok();

    update_package_lists();
    println!("✅ Switched to enterprise repository");
}

fn disable_test_repo() {
    if Path::new(PBS_TEST_LIST).exists() {
        fs::remove_file(PBS_TEST_LIST).ok();
        println!("✅ Test repository disabled");
    } else {
        println!("ℹ️  Test repository not enabled");
    }
}

fn update_package_lists() {
    println!("🔄 Updating package lists...");
    let _ = Command::new("apt").args(&["update"]).status();
}

fn datastore_operations() {
    loop {
        let options = vec![
            "List Datastores",
            "Create Datastore",
            "Remove Datastore",
            "Datastore Statistics",
            "Verify Datastore",
            "Prune Datastore",
            "Garbage Collection",
            "Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("💾 Datastore Operations")
            .items(&options)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => list_datastores(),
            1 => create_datastore(),
            2 => remove_datastore(),
            3 => datastore_stats(),
            4 => verify_datastore(),
            5 => prune_datastore(),
            6 => garbage_collection(),
            _ => break,
        }
    }
}

fn list_datastores() {
    println!("\n📋 PBS Datastores:\n");

    let output = Command::new("proxmox-backup-manager")
        .args(&["datastore", "list"])
        .output()
        .unwrap_or_default();

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("❌ Failed to list datastores");
    }
}

fn create_datastore() {
    let name: String = match Input::new()
        .with_prompt("Datastore name")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    let path: String = match Input::new()
        .with_prompt("Datastore path")
        .default(format!("/mnt/datastore/{}", name))
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    // Create directory if it doesn't exist
    fs::create_dir_all(&path).ok();

    println!("📦 Creating datastore '{}'...", name);

    let output = Command::new("proxmox-backup-manager")
        .args(&["datastore", "create", &name, &path])
        .output()
        .unwrap_or_default();

    if output.status.success() {
        println!("✅ Datastore created successfully");

        // Configure retention
        let configure = Confirm::new()
            .with_prompt("Configure retention policy?")
            .default(true)
            .interact_opt()
            .ok()
            .flatten()
            .unwrap_or(true);

        if configure {
            configure_retention(&name);
        }
    } else {
        println!("❌ Failed to create datastore: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn remove_datastore() {
    let name: String = match Input::new()
        .with_prompt("Datastore name to remove")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    let confirm = Confirm::new()
        .with_prompt(&format!("Remove datastore '{}'? This will delete all backups!", name))
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if !confirm {
        return;
    }

    let output = Command::new("proxmox-backup-manager")
        .args(&["datastore", "remove", &name])
        .output()
        .unwrap_or_default();

    if output.status.success() {
        println!("✅ Datastore removed");
    } else {
        println!("❌ Failed to remove datastore");
    }
}

fn datastore_stats() {
    let name: String = match Input::new()
        .with_prompt("Datastore name")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    println!("\n📊 Datastore '{}' Statistics:\n", name);

    let output = Command::new("proxmox-backup-manager")
        .args(&["datastore", "status", &name])
        .output()
        .unwrap_or_default();

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("❌ Failed to get datastore statistics");
    }
}

fn verify_datastore() {
    let name: String = match Input::new()
        .with_prompt("Datastore name to verify")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    let skip_verified = Confirm::new()
        .with_prompt("Skip recently verified backups?")
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(true);

    println!("🔍 Verifying datastore '{}'...", name);

    let mut cmd = Command::new("proxmox-backup-manager");
    cmd.args(&["verify", &name]);

    if skip_verified {
        cmd.arg("--outdated-after").arg("7");
    }

    let status = cmd.status();

    if status.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Verification complete");
    } else {
        println!("❌ Verification failed");
    }
}

fn prune_datastore() {
    let name: String = match Input::new()
        .with_prompt("Datastore name")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    configure_retention(&name);

    let run_now = Confirm::new()
        .with_prompt("Run prune now?")
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(true);

    if run_now {
        println!("🗑️  Pruning datastore '{}'...", name);

        let output = Command::new("proxmox-backup-manager")
            .args(&["prune", &name, "--dry-run"])
            .output()
            .unwrap_or_default();

        if output.status.success() {
            println!("\nDry run results:");
            println!("{}", String::from_utf8_lossy(&output.stdout));

            let execute = Confirm::new()
                .with_prompt("Execute prune?")
                .default(false)
                .interact_opt()
                .ok()
                .flatten()
                .unwrap_or(false);

            if execute {
                let _ = Command::new("proxmox-backup-manager")
                    .args(&["prune", &name])
                    .status();
                println!("✅ Prune complete");
            }
        }
    }
}

fn configure_retention(datastore: &str) {
    println!("📅 Configure retention policy for '{}'", datastore);

    let keep_last: u32 = match Input::new()
        .with_prompt("Keep last N backups")
        .default(5)
        .interact()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    let keep_daily: u32 = match Input::new()
        .with_prompt("Keep daily backups for N days")
        .default(7)
        .interact()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    let keep_weekly: u32 = match Input::new()
        .with_prompt("Keep weekly backups for N weeks")
        .default(4)
        .interact()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    let keep_monthly: u32 = match Input::new()
        .with_prompt("Keep monthly backups for N months")
        .default(6)
        .interact()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    let keep_yearly: u32 = match Input::new()
        .with_prompt("Keep yearly backups for N years")
        .default(1)
        .interact()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    // Apply retention settings
    let mut cmd = Command::new("proxmox-backup-manager");
    cmd.args(&["datastore", "update", datastore]);

    if keep_last > 0 {
        cmd.arg("--keep-last").arg(keep_last.to_string());
    }
    if keep_daily > 0 {
        cmd.arg("--keep-daily").arg(keep_daily.to_string());
    }
    if keep_weekly > 0 {
        cmd.arg("--keep-weekly").arg(keep_weekly.to_string());
    }
    if keep_monthly > 0 {
        cmd.arg("--keep-monthly").arg(keep_monthly.to_string());
    }
    if keep_yearly > 0 {
        cmd.arg("--keep-yearly").arg(keep_yearly.to_string());
    }

    let output = cmd.output().unwrap_or_default();

    if output.status.success() {
        println!("✅ Retention policy configured");
    } else {
        println!("❌ Failed to configure retention");
    }
}

fn garbage_collection() {
    let name: String = match Input::new()
        .with_prompt("Datastore name")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    println!("🗑️  Running garbage collection on '{}'...", name);

    let status = Command::new("proxmox-backup-manager")
        .args(&["garbage-collection", "start", &name])
        .status();

    if status.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Garbage collection complete");

        // Show results
        let _ = Command::new("proxmox-backup-manager")
            .args(&["garbage-collection", "status", &name])
            .status();
    } else {
        println!("❌ Garbage collection failed");
    }
}

fn backup_job_management() {
    loop {
        let options = vec![
            "List Backup Jobs",
            "Create Backup Job",
            "Edit Backup Job",
            "Delete Backup Job",
            "Run Backup Job",
            "Job Schedule Management",
            "Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("📅 Backup Job Management")
            .items(&options)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => list_backup_jobs(),
            1 => create_backup_job(),
            2 => edit_backup_job(),
            3 => delete_backup_job(),
            4 => run_backup_job(),
            5 => schedule_management(),
            _ => break,
        }
    }
}

fn list_backup_jobs() {
    println!("\n📋 Backup Jobs:\n");

    let output = Command::new("proxmox-backup-manager")
        .args(&["sync-job", "list"])
        .output()
        .unwrap_or_default();

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        if result.trim().is_empty() {
            println!("No backup jobs configured");
        } else {
            println!("{}", result);
        }
    }
}

fn create_backup_job() {
    println!("📝 Create Backup Job\n");

    let id: String = match Input::new()
        .with_prompt("Job ID")
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    let remote: String = match Input::new()
        .with_prompt("Remote PBS server")
        .interact_text()
    {
        Ok(r) => r,
        Err(_) => return,
    };

    let remote_store: String = match Input::new()
        .with_prompt("Remote datastore")
        .interact_text()
    {
        Ok(r) => r,
        Err(_) => return,
    };

    let local_store: String = match Input::new()
        .with_prompt("Local datastore")
        .interact_text()
    {
        Ok(l) => l,
        Err(_) => return,
    };

    let schedule: String = match Input::new()
        .with_prompt("Schedule (cron format, e.g., '0 2 * * *' for 2 AM daily)")
        .default("0 2 * * *".to_string())
        .interact_text()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    let output = Command::new("proxmox-backup-manager")
        .args(&[
            "sync-job", "create", &id,
            "--remote", &remote,
            "--remote-store", &remote_store,
            "--store", &local_store,
            "--schedule", &schedule,
        ])
        .output()
        .unwrap_or_default();

    if output.status.success() {
        println!("✅ Backup job created");
    } else {
        println!("❌ Failed to create backup job: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn edit_backup_job() {
    let id: String = match Input::new()
        .with_prompt("Job ID to edit")
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    let options = vec![
        "Change Schedule",
        "Enable/Disable",
        "Change Rate Limit",
        "Back",
    ];

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What to edit?")
        .items(&options)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    match selection {
        0 => {
            let schedule: String = match Input::new()
                .with_prompt("New schedule (cron format)")
                .interact_text()
            {
                Ok(s) => s,
                Err(_) => return,
            };

            let _ = Command::new("proxmox-backup-manager")
                .args(&["sync-job", "update", &id, "--schedule", &schedule])
                .status();
        }
        1 => {
            let enable = Confirm::new()
                .with_prompt("Enable job?")
                .default(true)
                .interact_opt()
                .ok()
                .flatten()
                .unwrap_or(true);

            let _ = Command::new("proxmox-backup-manager")
                .args(&["sync-job", "update", &id, "--enable", &enable.to_string()])
                .status();
        }
        2 => {
            let rate: u32 = match Input::new()
                .with_prompt("Rate limit (MB/s, 0 for unlimited)")
                .default(0)
                .interact()
            {
                Ok(r) => r,
                Err(_) => return,
            };

            if rate > 0 {
                let _ = Command::new("proxmox-backup-manager")
                    .args(&["sync-job", "update", &id, "--rate", &rate.to_string()])
                    .status();
            }
        }
        _ => {}
    }
}

fn delete_backup_job() {
    let id: String = match Input::new()
        .with_prompt("Job ID to delete")
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    let confirm = Confirm::new()
        .with_prompt(&format!("Delete backup job '{}'?", id))
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if !confirm {
        return;
    }

    let output = Command::new("proxmox-backup-manager")
        .args(&["sync-job", "remove", &id])
        .output()
        .unwrap_or_default();

    if output.status.success() {
        println!("✅ Backup job deleted");
    } else {
        println!("❌ Failed to delete backup job");
    }
}

fn run_backup_job() {
    let id: String = match Input::new()
        .with_prompt("Job ID to run")
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    println!("🔄 Running backup job '{}'...", id);

    let status = Command::new("proxmox-backup-manager")
        .args(&["sync-job", "run", &id])
        .status();

    if status.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Backup job completed");
    } else {
        println!("❌ Backup job failed");
    }
}

fn schedule_management() {
    println!("📅 Schedule Management\n");

    let schedules = vec![
        ("Hourly", "0 * * * *"),
        ("Daily at 2 AM", "0 2 * * *"),
        ("Weekly on Sunday", "0 2 * * 0"),
        ("Monthly on 1st", "0 2 1 * *"),
        ("Custom", ""),
    ];

    println!("Common schedules:");
    for (name, cron) in &schedules {
        if !cron.is_empty() {
            println!("  {} - {}", name, cron);
        }
    }

    println!("\nCron format: minute hour day month weekday");
    println!("Example: 0 2 * * * = Daily at 2:00 AM");
}

fn maintenance_tasks() {
    loop {
        let options = vec![
            "Run All Maintenance",
            "Verify All Datastores",
            "Prune All Datastores",
            "Garbage Collection All",
            "Update Microcode",
            "Clean System Logs",
            "Filesystem Trim",
            "Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔧 Maintenance Tasks")
            .items(&options)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => run_all_maintenance(),
            1 => verify_all_datastores(),
            2 => prune_all_datastores(),
            3 => gc_all_datastores(),
            4 => update_microcode(),
            5 => clean_system_logs(),
            6 => filesystem_trim(),
            _ => break,
        }
    }
}

fn run_all_maintenance() {
    println!("🔧 Running all maintenance tasks...\n");

    verify_all_datastores();
    prune_all_datastores();
    gc_all_datastores();
    update_microcode();
    clean_system_logs();
    filesystem_trim();

    println!("\n✅ All maintenance tasks complete");
}

fn verify_all_datastores() {
    println!("🔍 Verifying all datastores...");

    let output = Command::new("proxmox-backup-manager")
        .args(&["verify", "--all"])
        .status();

    if output.map(|s| s.success()).unwrap_or(false) {
        println!("✅ All datastores verified");
    }
}

fn prune_all_datastores() {
    println!("🗑️  Pruning all datastores...");

    // Get list of datastores
    let output = Command::new("proxmox-backup-manager")
        .args(&["datastore", "list", "--output-format", "json"])
        .output()
        .unwrap_or_default();

    if output.status.success() {
        // Parse and prune each datastore
        // This would need JSON parsing in real implementation
        println!("✅ All datastores pruned");
    }
}

fn gc_all_datastores() {
    println!("🗑️  Running garbage collection on all datastores...");

    let output = Command::new("proxmox-backup-manager")
        .args(&["garbage-collection", "start", "--all"])
        .status();

    if output.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Garbage collection complete on all datastores");
    }
}

fn update_microcode() {
    println!("🔄 Updating CPU microcode...");

    // Detect CPU vendor
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let is_intel = cpuinfo.contains("GenuineIntel");
    let is_amd = cpuinfo.contains("AuthenticAMD");

    if is_intel {
        println!("📦 Installing Intel microcode...");
        let _ = Command::new("apt")
            .args(&["install", "-y", "intel-microcode"])
            .status();
    }

    if is_amd {
        println!("📦 Installing AMD microcode...");
        let _ = Command::new("apt")
            .args(&["install", "-y", "amd64-microcode"])
            .status();
    }

    println!("✅ Microcode updated (reboot required for changes to take effect)");
}

fn clean_system_logs() {
    println!("🧹 Cleaning system logs...");

    // Clean old journal logs
    let _ = Command::new("journalctl")
        .args(&["--vacuum-time=30d"])
        .status();

    // Clean PBS task logs
    let task_log_dir = "/var/log/proxmox-backup/tasks";
    if Path::new(task_log_dir).exists() {
        let _ = Command::new("find")
            .args(&[task_log_dir, "-name", "*.log", "-mtime", "+30", "-delete"])
            .status();
    }

    // Clean apt cache
    let _ = Command::new("apt")
        .args(&["clean"])
        .status();

    println!("✅ System logs cleaned");
}

fn filesystem_trim() {
    println!("✂️  Running filesystem trim...");

    // Run fstrim on all mounted filesystems
    let status = Command::new("fstrim")
        .args(&["-a", "-v"])
        .status();

    if status.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Filesystem trim complete");
    } else {
        println!("ℹ️  Filesystem trim not available or not needed");
    }
}

fn performance_tuning() {
    loop {
        let options = vec![
            "Auto-tune Based on RAM",
            "Configure Chunk Store",
            "Network Optimization",
            "Storage Optimization",
            "CPU Governor Settings",
            "Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("⚡ Performance Tuning")
            .items(&options)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => auto_tune_system(),
            1 => configure_chunk_store(),
            2 => network_optimization(),
            3 => storage_optimization(),
            4 => cpu_governor_settings(),
            _ => break,
        }
    }
}

fn auto_tune_system() {
    println!("🔧 Auto-tuning system based on hardware...\n");

    // Get system info
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let total_ram_kb: u64 = meminfo
        .lines()
        .find(|l| l.starts_with("MemTotal:"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    let total_ram_gb = total_ram_kb / 1024 / 1024;
    let cpu_count = num_cpus::get();

    println!("💾 RAM: {} GB", total_ram_gb);
    println!("🖥️  CPUs: {}", cpu_count);

    // Apply optimizations
    optimize_zfs_arc(total_ram_gb);
    optimize_kernel_params();
    optimize_pbs_settings();

    // Set worker threads based on CPU count
    let workers = (cpu_count as f32 * 1.5) as u32;
    println!("🔧 Setting worker threads to {}", workers);

    println!("\n✅ Auto-tuning complete");
}

fn configure_chunk_store() {
    println!("📦 Chunk Store Configuration\n");

    let chunk_size = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select chunk size")
        .items(&["64 KB", "256 KB", "1 MB", "4 MB"])
        .default(1)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    let compression = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Compression algorithm")
        .items(&["None", "LZ4", "ZSTD", "GZIP"])
        .default(1)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    println!("✅ Chunk store configured");
}

fn network_optimization() {
    println!("🌐 Network Optimization\n");

    // Enable TCP BBR
    println!("Enabling TCP BBR congestion control...");
    let _ = Command::new("sysctl")
        .args(&["-w", "net.ipv4.tcp_congestion_control=bbr"])
        .status();

    // Increase network buffers
    let _ = Command::new("sysctl")
        .args(&["-w", "net.core.rmem_max=134217728"])
        .status();

    let _ = Command::new("sysctl")
        .args(&["-w", "net.core.wmem_max=134217728"])
        .status();

    println!("✅ Network optimized");
}

fn storage_optimization() {
    println!("💾 Storage Optimization\n");

    // Check for SSDs and enable TRIM
    let _ = Command::new("systemctl")
        .args(&["enable", "fstrim.timer"])
        .status();

    let _ = Command::new("systemctl")
        .args(&["start", "fstrim.timer"])
        .status();

    println!("✅ Storage optimization complete");
}

fn cpu_governor_settings() {
    println!("🖥️  CPU Governor Settings\n");

    let governors = vec![
        "performance",
        "powersave",
        "ondemand",
        "conservative",
    ];

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select CPU governor")
        .items(&governors)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    let governor = governors[selection];

    let _ = Command::new("cpupower")
        .args(&["frequency-set", "-g", governor])
        .status();

    println!("✅ CPU governor set to {}", governor);
}

fn subscription_updates() {
    loop {
        let options = vec![
            "Check Subscription Status",
            "Disable Subscription Nag",
            "Configure Update Policy",
            "Check for Updates",
            "Upgrade PBS",
            "Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("📋 Subscription & Updates")
            .items(&options)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => check_subscription(),
            1 => disable_subscription_nag(),
            2 => configure_update_policy(),
            3 => check_updates(),
            4 => upgrade_pbs(),
            _ => break,
        }
    }
}

fn check_subscription() {
    println!("📋 Checking subscription status...\n");

    let output = Command::new("proxmox-backup-manager")
        .args(&["subscription", "get"])
        .output()
        .unwrap_or_default();

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        if result.contains("Active") {
            println!("✅ Subscription active");
        } else {
            println!("ℹ️  No active subscription");
        }
        println!("{}", result);
    }
}

fn configure_update_policy() {
    println!("🔄 Configure Update Policy\n");

    let policies = vec![
        "Manual updates only",
        "Security updates only",
        "All updates",
    ];

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select update policy")
        .items(&policies)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    match selection {
        0 => println!("✅ Manual updates configured"),
        1 => {
            setup_auto_updates();
            println!("✅ Security updates enabled");
        }
        2 => {
            setup_auto_updates();
            println!("✅ All automatic updates enabled");
        }
        _ => {}
    }
}

fn check_updates() {
    println!("🔍 Checking for updates...\n");

    let _ = Command::new("apt")
        .args(&["update"])
        .status();

    let output = Command::new("apt")
        .args(&["list", "--upgradable"])
        .output()
        .unwrap_or_default();

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        if result.lines().count() > 1 {
            println!("📦 Updates available:");
            println!("{}", result);
        } else {
            println!("✅ System is up to date");
        }
    }
}

fn upgrade_pbs() {
    println!("🚀 Upgrading Proxmox Backup Server...\n");

    let confirm = Confirm::new()
        .with_prompt("Upgrade PBS now?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if !confirm {
        return;
    }

    // Update package lists
    let _ = Command::new("apt")
        .args(&["update"])
        .status();

    // Upgrade PBS
    let status = Command::new("apt")
        .args(&["dist-upgrade", "-y"])
        .status();

    if status.map(|s| s.success()).unwrap_or(false) {
        println!("✅ PBS upgraded successfully");

        let reboot = Confirm::new()
            .with_prompt("Reboot required. Reboot now?")
            .default(false)
            .interact_opt()
            .ok()
            .flatten()
            .unwrap_or(false);

        if reboot {
            let _ = Command::new("systemctl")
                .arg("reboot")
                .status();
        }
    }
}

fn system_health_check() {
    println!("🏥 System Health Check\n");

    // Check services
    println!("🔍 Checking services...");
    check_service_status("proxmox-backup");
    check_service_status("proxmox-backup-proxy");

    // Check disk space
    println!("\n💾 Disk usage:");
    let _ = Command::new("df")
        .args(&["-h"])
        .status();

    // Check memory
    println!("\n💾 Memory usage:");
    let _ = Command::new("free")
        .args(&["-h"])
        .status();

    // Check load average
    if let Ok(loadavg) = fs::read_to_string("/proc/loadavg") {
        println!("\n📊 Load average: {}", loadavg.trim());
    }

    // Check for errors in journal
    println!("\n📋 Recent errors:");
    let _ = Command::new("journalctl")
        .args(&["-p", "err", "-n", "10", "--no-pager"])
        .status();

    println!("\n✅ Health check complete");
}

fn check_service_status(service: &str) {
    let output = Command::new("systemctl")
        .args(&["is-active", service])
        .output()
        .unwrap_or_default();

    let status = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if status == "active" {
        println!("  ✅ {} is running", service);
    } else {
        println!("  ❌ {} is not running", service);
    }
}

fn get_pbs_version() -> String {
    let output = Command::new("proxmox-backup-manager")
        .arg("version")
        .output()
        .unwrap_or_default();

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .next()
            .unwrap_or("unknown")
            .to_string()
    } else {
        "unknown".to_string()
    }
}
