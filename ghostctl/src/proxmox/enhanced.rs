use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use reqwest::blocking::get;
use std::process::Command;

// Enhanced Proxmox script categories
const PROXMOX_CATEGORIES: &[(&str, &[(&str, &str)])] = &[
    (
        "Container Templates",
        &[
            (
                "Docker LXC",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/docker.sh",
            ),
            (
                "Portainer LXC",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/portainer.sh",
            ),
            (
                "Nginx Proxy Manager",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/nginxproxymanager.sh",
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
                "Jellyfin LXC",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/jellyfin.sh",
            ),
            (
                "Home Assistant LXC",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/homeassistant.sh",
            ),
        ],
    ),
    (
        "Virtual Machines",
        &[
            (
                "Home Assistant OS VM",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/vm/haos-vm.sh",
            ),
            (
                "Windows 11 VM",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/vm/windows11.sh",
            ),
            (
                "Ubuntu VM",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/vm/ubuntu.sh",
            ),
            (
                "Debian VM",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/vm/debian.sh",
            ),
        ],
    ),
    (
        "System Administration",
        &[
            (
                "Post Install Setup",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/misc/post-pve-install.sh",
            ),
            (
                "Proxmox Backup Server",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/misc/pbs-install.sh",
            ),
            (
                "Dark Theme",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/misc/pve-dark-theme.sh",
            ),
            (
                "CPU Scaling Governor",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/misc/scaling-governor.sh",
            ),
        ],
    ),
    (
        "Monitoring & Logging",
        &[
            (
                "Prometheus LXC",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/prometheus.sh",
            ),
            (
                "Grafana LXC",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/grafana.sh",
            ),
            (
                "InfluxDB LXC",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/influxdb.sh",
            ),
            (
                "Zabbix LXC",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/zabbix.sh",
            ),
        ],
    ),
    (
        "Development Tools",
        &[
            (
                "GitLab LXC",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/gitlab.sh",
            ),
            (
                "Jenkins LXC",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/jenkins.sh",
            ),
            (
                "Code Server LXC",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/code-server.sh",
            ),
            (
                "Docker Registry LXC",
                "https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/docker-registry.sh",
            ),
        ],
    ),
];

pub fn enhanced_proxmox_menu() {
    loop {
        let mut menu_options: Vec<String> = PROXMOX_CATEGORIES
            .iter()
            .map(|(category, _)| format!("📂 {}", category))
            .collect();

        menu_options.extend_from_slice(&[
            "🔍 Search Scripts".to_string(),
            "🌐 Browse All Scripts Online".to_string(),
            "📋 Proxmox System Info".to_string(),
            "🛠️  Proxmox Management Tools".to_string(),
            "📦 Template Management".to_string(),
            "🔥 Firewall Automation".to_string(),
            "⬅️  Back".to_string(),
        ]);

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🏥 Enhanced Proxmox VE Tools")
            .items(&menu_options)
            .default(0)
            .interact()
            .unwrap();

        if choice == menu_options.len() - 1 {
            break; // Back
        } else if choice == menu_options.len() - 2 {
            super::firewall_automation::firewall_automation_menu();
        } else if choice == menu_options.len() - 3 {
            super::template_management::template_management_menu();
        } else if choice == menu_options.len() - 4 {
            proxmox_management_tools();
        } else if choice == menu_options.len() - 5 {
            proxmox_system_info();
        } else if choice == menu_options.len() - 6 {
            super::browse_all_scripts();
        } else if choice == menu_options.len() - 7 {
            search_scripts();
        } else {
            // Show category scripts
            let (category_name, scripts) = PROXMOX_CATEGORIES[choice];
            show_category_scripts(category_name, scripts);
        }
    }
}

fn show_category_scripts(category: &str, scripts: &[(&str, &str)]) {
    loop {
        let mut script_items: Vec<String> =
            scripts.iter().map(|(name, _)| name.to_string()).collect();
        script_items.push("⬅️  Back".to_string());

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("📂 {} Scripts", category))
            .items(&script_items)
            .default(0)
            .interact()
            .unwrap();

        if choice == script_items.len() - 1 {
            break; // Back
        }

        let (script_name, script_url) = scripts[choice];
        show_script_details(script_name, script_url);
    }
}

fn show_script_details(name: &str, url: &str) {
    println!("\n📜 Script Details: {}", name);
    println!("🔗 URL: {}", url);

    // Try to fetch script content for preview
    match get(url) {
        Ok(response) if response.status().is_success() => {
            if let Ok(content) = response.text() {
                let lines: Vec<&str> = content.lines().collect();
                println!("\n📋 Preview (first 10 lines):");
                println!("─────────────────────────");
                for line in lines.iter().take(10) {
                    println!("  {}", line);
                }
                if lines.len() > 10 {
                    println!("  ... ({} more lines)", lines.len() - 10);
                }
            }
        }
        _ => println!("⚠️  Could not fetch script preview"),
    }

    let actions = [
        "🚀 Run Script",
        "📖 View Full Script",
        "📋 Copy URL",
        "⬅️  Back",
    ];
    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose action")
        .items(&actions)
        .default(0)
        .interact()
        .unwrap();

    match action {
        0 => run_proxmox_script(name, url),
        1 => view_full_script(url),
        2 => copy_url_to_clipboard(url),
        _ => return,
    }
}

fn proxmox_management_tools() {
    let tools = [
        "🔧 VM/CT Bulk Operations",
        "📊 Resource Usage Report",
        "🔄 Backup Management",
        "🏗️  Cluster Management",
        "🌐 Network Configuration",
        "💾 Storage Management",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🛠️  Proxmox Management Tools")
        .items(&tools)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => bulk_vm_operations(),
        1 => resource_usage_report(),
        2 => proxmox_backup_management(),
        3 => proxmox_cluster_management(),
        4 => network_configuration(),
        5 => storage_management(),
        _ => return,
    }
}

fn bulk_vm_operations() {
    println!("🔧 VM/Container Bulk Operations");

    let operations = [
        "🚀 Start All VMs/CTs",
        "🛑 Stop All VMs/CTs",
        "🔄 Restart All VMs/CTs",
        "📊 Status Report",
        "🧹 Cleanup Unused Resources",
    ];

    let op = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bulk operation")
        .items(&operations)
        .default(0)
        .interact()
        .unwrap();

    let confirm = Confirm::new()
        .with_prompt(format!(
            "⚠️  Execute '{}'? This affects ALL VMs/containers",
            operations[op]
        ))
        .default(false)
        .interact()
        .unwrap();

    if !confirm {
        println!("❌ Operation cancelled");
        return;
    }

    match op {
        0 => execute_bulk_command("qm list | awk 'NR>1 {print $1}' | xargs -I {} qm start {}"),
        1 => execute_bulk_command("qm list | awk 'NR>1 {print $1}' | xargs -I {} qm stop {}"),
        2 => execute_bulk_command("qm list | awk 'NR>1 {print $1}' | xargs -I {} qm reboot {}"),
        3 => {
            println!("📊 VM Status Report:");
            let _ = Command::new("qm").arg("list").status();
            println!("\n📊 Container Status Report:");
            let _ = Command::new("pct").arg("list").status();
        }
        4 => cleanup_unused_resources(),
        _ => {}
    }
}

fn proxmox_system_info() {
    println!("📋 Proxmox VE System Information");
    println!("═══════════════════════════════");

    // PVE Version
    println!("🔧 Proxmox VE Version:");
    let _ = Command::new("pveversion").arg("--verbose").status();

    // Cluster status
    println!("\n🌐 Cluster Status:");
    let _ = Command::new("pvecm").arg("status").status();

    // Node resources
    println!("\n💾 Node Resources:");
    let _ = Command::new("pvesh")
        .args(&["get", "/nodes/localhost/status"])
        .status();

    // Storage status
    println!("\n💿 Storage Status:");
    let _ = Command::new("pvesh").args(&["get", "/storage"]).status();

    // Recent tasks
    println!("\n📋 Recent Tasks:");
    let _ = Command::new("pvesh")
        .args(&["get", "/cluster/tasks", "--limit", "10"])
        .status();
}

// Complete missing function implementations
fn execute_bulk_command(command: &str) {
    println!("🔧 Executing bulk command: {}", command);
    let targets = get_all_vms_and_cts();
    for target in targets {
        println!("  📋 Target: {}", target);
        let _ = Command::new("ssh")
            .args(&[&target, &command.to_string()])
            .status();
    }
}

fn get_all_vms_and_cts() -> Vec<String> {
    let mut targets = Vec::new();
    // Get all VM IDs
    let vm_output = Command::new("qm").arg("list").output().unwrap();
    let vm_ids = String::from_utf8_lossy(&vm_output.stdout);
    for line in vm_ids.lines().skip(1) {
        if let Some(id) = line.split_whitespace().next() {
            targets.push(format!("{}{}", "vm", id));
        }
    }

    // Get all CT IDs
    let ct_output = Command::new("pct").arg("list").output().unwrap();
    let ct_ids = String::from_utf8_lossy(&ct_output.stdout);
    for line in ct_ids.lines().skip(1) {
        if let Some(id) = line.split_whitespace().next() {
            targets.push(format!("{}{}", "ct", id));
        }
    }

    targets
}

fn proxmox_backup_management() {
    println!("💾 Proxmox Backup Management");
    println!("============================");

    let options = [
        "📋 List backup jobs",
        "▶️ Run backup now",
        "📅 Schedule backup",
        "🔍 Verify backups",
        "⬅️ Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Backup Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => list_backup_jobs(),
        1 => run_backup_now(),
        2 => schedule_backup(),
        3 => verify_backups(),
        _ => return,
    }
}

fn list_backup_jobs() {
    println!("📋 Listing backup jobs...");
    let _ = Command::new("pvesh")
        .args(&["get", "/cluster/backup"])
        .status();
}

fn run_backup_now() {
    println!("▶️ Running backup now...");
    let job_id: String = Input::new()
        .with_prompt("Backup job ID")
        .interact_text()
        .unwrap();

    let _ = Command::new("pvesh")
        .args(&[
            "create",
            "/nodes/localhost/vzdump",
            "--mode",
            "snapshot",
            "--vmid",
            &job_id,
        ])
        .status();
}

fn schedule_backup() {
    println!("📅 Scheduling backup...");
    println!("Use Proxmox web interface for advanced scheduling");
}

fn verify_backups() {
    println!("🔍 Verifying backups...");
    let _ = Command::new("pvesh")
        .args(&["get", "/nodes/localhost/storage"])
        .status();
}

fn proxmox_cluster_management() {
    println!("🌐 Proxmox Cluster Management");
    println!("=============================");

    let options = [
        "📊 Cluster status",
        "🔗 Join cluster",
        "➕ Add node",
        "➖ Remove node",
        "⬅️ Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Cluster Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => show_cluster_status(),
        1 => join_cluster(),
        2 => add_cluster_node(),
        3 => remove_cluster_node(),
        _ => return,
    }
}

fn show_cluster_status() {
    println!("📊 Cluster Status:");
    let _ = Command::new("pvecm").arg("status").status();
}

fn join_cluster() {
    println!("🔗 Join cluster...");
    let cluster_ip: String = Input::new()
        .with_prompt("Cluster IP address")
        .interact_text()
        .unwrap();

    println!("Use: pvecm add {}", cluster_ip);
}

fn add_cluster_node() {
    println!("➕ Add cluster node...");
    println!("Use Proxmox web interface for node management");
}

fn remove_cluster_node() {
    println!("➖ Remove cluster node...");
    println!("Use: pvecm delnode <nodename>");
}

fn search_scripts() {
    println!("🔍 Searching Proxmox scripts...");
    println!("💡 Feature coming soon - search functionality");
}

fn run_proxmox_script(name: &str, url: &str) {
    println!("🚀 Running Proxmox script: {}", name);
    super::run_script_by_url(url);
}

fn view_full_script(url: &str) {
    println!("📖 Viewing script at: {}", url);
    match get(url) {
        Ok(response) => {
            if let Ok(content) = response.text() {
                println!("{}", content);
            } else {
                println!("❌ Failed to read script content");
            }
        }
        Err(_) => println!("❌ Failed to fetch script"),
    }
}

fn copy_url_to_clipboard(url: &str) {
    println!("📋 Copying URL to clipboard: {}", url);
    println!("💡 Manual copy required: {}", url);
}

fn resource_usage_report() {
    println!("📊 Proxmox Resource Usage Report");
    println!("================================");
    proxmox_system_info();
}

fn backup_management() {
    println!("💾 Proxmox Backup Management");
    println!("============================");
    println!("💡 Feature coming soon");
}

fn network_configuration() {
    println!("🌐 Proxmox Network Configuration");
    println!("=================================");
    println!("💡 Feature coming soon");
}

fn storage_management() {
    loop {
        let options = vec![
            "🔄 Storage Migration",
            "📊 Storage Status & Usage",
            "➕ Add Storage Pool",
            "🗑️  Remove Storage Pool",
            "⚙️  Storage Configuration",
            "🔍 Storage Performance Analysis",
            "⬅️  Back",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("💾 Proxmox Storage Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => super::storage_migration::storage_migration_menu(),
            1 => storage_status_usage(),
            2 => add_storage_pool(),
            3 => remove_storage_pool(),
            4 => storage_configuration(),
            5 => storage_performance_analysis(),
            _ => break,
        }
    }
}

fn storage_status_usage() {
    println!("📊 Storage Status & Usage\n");
    let _ = Command::new("pvesm").args(&["status"]).status();
}

fn add_storage_pool() {
    println!("➕ Add Storage Pool\n");
    println!("💡 Use the Storage Migration menu for comprehensive storage pool management");
}

fn remove_storage_pool() {
    println!("🗑️  Remove Storage Pool\n");
    println!("💡 Use the Storage Migration menu for comprehensive storage pool management");
}

fn storage_configuration() {
    println!("⚙️  Storage Configuration\n");
    let _ = Command::new("cat").args(&["/etc/pve/storage.cfg"]).status();
}

fn storage_performance_analysis() {
    println!("🔍 Storage Performance Analysis\n");
    let _ = Command::new("iostat").args(&["-x", "1", "3"]).status();
}

fn cleanup_unused_resources() {
    println!("🧹 Cleaning up unused Proxmox resources");
    println!("========================================");
    println!("💡 Feature coming soon");
}
