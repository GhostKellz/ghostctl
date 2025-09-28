use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use std::path::Path;
use std::process::Command;

pub fn virtualization_menu() {
    loop {
        let options = [
            "🐳 Docker Network Troubleshooting",
            "🖥️ QEMU/KVM Integration Status",
            "🔧 Virtual Interface Management",
            "📊 Virtualization Network Status",
            "🚀 Advanced Virtual Networking",
            "ℹ️ Migration Notice",
            "⬅️ Back",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🖥️ Virtualization Networking")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => docker_network_troubleshooting(),
            1 => qemu_kvm_integration_status(),
            2 => virtual_interface_management(),
            3 => virtualization_network_status(),
            4 => advanced_virtual_networking(),
            5 => migration_notice(),
            _ => break,
        }
    }
}

fn docker_network_troubleshooting() {
    let options = [
        "📊 Docker Network Status",
        "🔍 Container Network Diagnosis",
        "🌐 Docker Network Management",
        "🔧 Fix Docker Networking Issues",
        "📡 Docker DNS Troubleshooting",
        "🔌 Port Mapping Analysis",
        "🚀 Performance Testing",
        "⬅️ Back",
    ];

    loop {
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🐳 Docker Network Troubleshooting")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => docker_network_status(),
            1 => container_network_diagnosis(),
            2 => docker_network_management(),
            3 => fix_docker_networking(),
            4 => docker_dns_troubleshooting(),
            5 => port_mapping_analysis(),
            6 => docker_performance_testing(),
            _ => break,
        }
    }
}

fn docker_network_status() {
    println!("📊 Docker Network Status");
    println!("=========================\n");

    // Check Docker daemon status
    println!("🐳 Docker Daemon Status:");
    let docker_status = Command::new("systemctl")
        .args(&["is-active", "docker"])
        .output();

    match docker_status {
        Ok(out) => {
            let status = String::from_utf8_lossy(&out.stdout)
                .trim()
                .to_string()
                .to_string();
            if status == "active" {
                println!("  ✅ Docker daemon is running");
            } else {
                println!("  ❌ Docker daemon is not running");
                return;
            }
        }
        _ => {
            println!("  ❌ Cannot check Docker daemon status");
            return;
        }
    }

    // List Docker networks
    println!("\n🌐 Docker Networks:");
    Command::new("docker")
        .args(&["network", "ls"])
        .status()
        .ok();

    // Show detailed network information
    println!("\n📋 Network Details:");
    let networks = Command::new("docker")
        .args(&["network", "ls", "--format", "{{.Name}}"])
        .output();

    if let Ok(out) = networks {
        for network in String::from_utf8_lossy(&out.stdout).lines() {
            if !network.trim().is_empty() {
                println!("\n--- {} ---", network);
                Command::new("docker")
                    .args(&["network", "inspect", network])
                    .status()
                    .ok();
            }
        }
    }

    // Check iptables rules for Docker
    println!("\n🔥 Docker iptables Rules:");
    Command::new("sudo")
        .args(&["iptables", "-t", "nat", "-L", "DOCKER", "-n"])
        .status()
        .ok();

    println!("\n🔗 Docker Bridge Information:");
    Command::new("ip")
        .args(&["addr", "show", "docker0"])
        .status()
        .ok();

    // Check Docker IP ranges
    println!("\n📊 Docker IP Ranges:");
    Command::new("docker")
        .args(&["system", "info", "--format", "{{.DefaultRuntime}}"])
        .status()
        .ok();
}

fn container_network_diagnosis() {
    println!("🔍 Container Network Diagnosis");
    println!("===============================\n");

    // List running containers
    println!("📦 Running Containers:");
    Command::new("docker")
        .args(&[
            "ps",
            "--format",
            "table {{.Names}}\t{{.Status}}\t{{.Ports}}",
        ])
        .status()
        .ok();

    let container = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter container name or ID to diagnose (or 'all' for all containers)")
        .interact()
        .unwrap();

    if container == "all" {
        diagnose_all_containers();
    } else {
        diagnose_single_container(&container);
    }
}

fn diagnose_single_container(container: &str) {
    println!("🔍 Diagnosing container: {}", container);
    println!("==========================================\n");

    // Check if container exists and is running
    let container_check = Command::new("docker")
        .args(&["inspect", container, "--format", "{{.State.Running}}"])
        .output();

    match container_check {
        Ok(out) if String::from_utf8_lossy(&out.stdout).trim().to_string() == "true" => {
            println!("✅ Container is running");
        }
        Ok(out) if String::from_utf8_lossy(&out.stdout).trim().to_string() == "false" => {
            println!("⚠️ Container exists but is not running");
            return;
        }
        _ => {
            println!("❌ Container not found");
            return;
        }
    }

    // Network configuration
    println!("\n1️⃣ Network Configuration:");
    Command::new("docker")
        .args(&[
            "inspect",
            container,
            "--format",
            "{{range .NetworkSettings.Networks}}{{.IPAddress}} {{.Gateway}} {{.MacAddress}}{{end}}",
        ])
        .status()
        .ok();

    // Network interfaces inside container
    println!("\n2️⃣ Container Network Interfaces:");
    Command::new("docker")
        .args(&["exec", container, "ip", "addr", "show"])
        .status()
        .ok();

    // Routing table inside container
    println!("\n3️⃣ Container Routing Table:");
    Command::new("docker")
        .args(&["exec", container, "ip", "route", "show"])
        .status()
        .ok();

    // DNS configuration
    println!("\n4️⃣ Container DNS Configuration:");
    Command::new("docker")
        .args(&["exec", container, "cat", "/etc/resolv.conf"])
        .status()
        .ok();

    // Test connectivity from container
    println!("\n5️⃣ Container Connectivity Tests:");

    // Test DNS
    println!("🔍 DNS Test:");
    Command::new("docker")
        .args(&["exec", container, "nslookup", "google.com"])
        .status()
        .ok();

    // Test internet connectivity
    println!("\n🌐 Internet Connectivity Test:");
    Command::new("docker")
        .args(&["exec", container, "ping", "-c", "3", "8.8.8.8"])
        .status()
        .ok();

    // Test container-to-container communication
    println!("\n🐳 Container-to-Container Test:");
    let other_containers = Command::new("docker")
        .args(&[
            "ps",
            "--format",
            "{{.Names}}",
            "--filter",
            &format!("name!={}", container),
        ])
        .output();

    if let Ok(out) = other_containers {
        let containers: Vec<String> = String::from_utf8_lossy(&out.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect();
        if !containers.is_empty() && !containers[0].trim().is_empty() {
            let target_container = containers[0].trim();
            println!("Testing connection to {}:", target_container);

            // Get target container IP
            let target_ip = Command::new("docker")
                .args(&[
                    "inspect",
                    target_container,
                    "--format",
                    "{{.NetworkSettings.IPAddress}}",
                ])
                .output();

            if let Ok(ip_out) = target_ip {
                let ip = String::from_utf8_lossy(&ip_out.stdout)
                    .trim()
                    .to_string()
                    .to_string();
                if !ip.is_empty() && ip != "<no value>" {
                    Command::new("docker")
                        .args(&["exec", container, "ping", "-c", "2", &ip])
                        .status()
                        .ok();
                }
            }
        }
    }

    // Port binding analysis
    println!("\n6️⃣ Port Bindings:");
    Command::new("docker")
        .args(&["port", container])
        .status()
        .ok();

    // Check processes listening on ports inside container
    println!("\n7️⃣ Listening Processes in Container:");
    Command::new("docker")
        .args(&["exec", container, "netstat", "-tlnp"])
        .status()
        .ok();
}

fn diagnose_all_containers() {
    println!("🔍 Diagnosing All Containers");
    println!("=============================\n");

    let containers = Command::new("docker")
        .args(&["ps", "--format", "{{.Names}}"])
        .output();

    if let Ok(out) = containers {
        for container in String::from_utf8_lossy(&out.stdout).lines() {
            if !container.trim().is_empty() {
                println!("\n{}", "=".repeat(50));
                diagnose_single_container(container.trim());
                println!("{}", "=".repeat(50));
            }
        }
    }
}

fn docker_network_management() {
    let options = [
        "➕ Create Docker Network",
        "🗑️ Remove Docker Network",
        "🔌 Connect Container to Network",
        "🔌 Disconnect Container from Network",
        "🔍 Inspect Network",
        "🧹 Prune Unused Networks",
        "⬅️ Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🌐 Docker Network Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => create_docker_network(),
        1 => remove_docker_network(),
        2 => connect_container_to_network(),
        3 => disconnect_container_from_network(),
        4 => inspect_docker_network(),
        5 => prune_docker_networks(),
        _ => {}
    }
}

fn create_docker_network() {
    println!("➕ Create Docker Network");
    println!("=========================\n");

    let name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter network name")
        .interact()
        .unwrap();

    let driver = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select network driver")
        .items(&["bridge", "overlay", "macvlan", "host", "none"])
        .default(0)
        .interact()
        .unwrap();

    let driver_name = ["bridge", "overlay", "macvlan", "host", "none"][driver];

    let mut cmd_args = vec![
        "network".to_string(),
        "create".to_string(),
        "--driver".to_string(),
        driver_name.to_string(),
    ];

    if driver == 0 {
        // bridge driver
        let subnet = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter subnet (e.g., 172.20.0.0/16) or press Enter to auto-assign")
            .allow_empty(true)
            .interact()
            .unwrap();

        if !subnet.is_empty() {
            cmd_args.push("--subnet".to_string());
            cmd_args.push(subnet);

            let gateway = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter gateway (or press Enter for auto)")
                .allow_empty(true)
                .interact()
                .unwrap();

            if !gateway.is_empty() {
                cmd_args.push("--gateway".to_string());
                cmd_args.push(gateway);
            }
        }

        let custom_bridge = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Custom bridge name (or press Enter for auto)")
            .allow_empty(true)
            .interact()
            .unwrap();

        if !custom_bridge.is_empty() {
            let bridge_opt = format!("com.docker.network.bridge.name={}", custom_bridge);
            cmd_args.push("-o".to_string());
            cmd_args.push(bridge_opt);
        }
    }

    cmd_args.push(name.clone());

    println!(
        "🔧 Creating network with command: docker {}",
        cmd_args.join(" ")
    );
    let result = Command::new("docker").args(&cmd_args).status();

    match result {
        Ok(s) if s.success() => {
            println!("✅ Network '{}' created successfully", name);

            // Show network details
            println!("\n📋 Network Details:");
            Command::new("docker")
                .args(&["network", "inspect", &name])
                .status()
                .ok();
        }
        _ => println!("❌ Failed to create network"),
    }
}

fn remove_docker_network() {
    println!("🗑️ Remove Docker Network");
    println!("=========================\n");

    // List networks
    println!("📋 Available Networks:");
    Command::new("docker")
        .args(&["network", "ls"])
        .status()
        .ok();

    let network = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter network name to remove")
        .interact()
        .unwrap();

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!("Remove network '{}'?", network))
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let result = Command::new("docker")
            .args(&["network", "rm", &network])
            .status();

        match result {
            Ok(s) if s.success() => println!("✅ Network '{}' removed", network),
            _ => println!("❌ Failed to remove network"),
        }
    }
}

fn connect_container_to_network() {
    println!("🔌 Connect Container to Network");
    println!("================================\n");

    // List containers
    println!("📦 Available Containers:");
    Command::new("docker")
        .args(&["ps", "--format", "table {{.Names}}\t{{.Status}}"])
        .status()
        .ok();

    let container = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter container name")
        .interact()
        .unwrap();

    // List networks
    println!("\n🌐 Available Networks:");
    Command::new("docker")
        .args(&["network", "ls"])
        .status()
        .ok();

    let network = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter network name")
        .interact()
        .unwrap();

    let ip_address = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Specific IP address (optional)")
        .allow_empty(true)
        .interact()
        .unwrap();

    let mut cmd_args = vec!["network", "connect"];

    if !ip_address.is_empty() {
        cmd_args.extend(&["--ip", &ip_address]);
    }

    cmd_args.extend(&[network.as_str(), container.as_str()]);

    let result = Command::new("docker").args(&cmd_args).status();

    match result {
        Ok(s) if s.success() => {
            println!(
                "✅ Container '{}' connected to network '{}'",
                container, network
            );

            // Show updated container network info
            println!("\n📋 Updated Container Networks:");
            Command::new("docker")
                .args(&[
                    "inspect",
                    &container,
                    "--format",
                    "{{range .NetworkSettings.Networks}}{{.IPAddress}} {{.NetworkID}}{{end}}",
                ])
                .status()
                .ok();
        }
        _ => println!("❌ Failed to connect container to network"),
    }
}

fn disconnect_container_from_network() {
    println!("🔌 Disconnect Container from Network");
    println!("=====================================\n");

    let container = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter container name")
        .interact()
        .unwrap();

    let network = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter network name")
        .interact()
        .unwrap();

    let result = Command::new("docker")
        .args(&["network", "disconnect", &network, &container])
        .status();

    match result {
        Ok(s) if s.success() => println!(
            "✅ Container '{}' disconnected from network '{}'",
            container, network
        ),
        _ => println!("❌ Failed to disconnect container from network"),
    }
}

fn inspect_docker_network() {
    println!("🔍 Inspect Docker Network");
    println!("==========================\n");

    println!("📋 Available Networks:");
    Command::new("docker")
        .args(&["network", "ls"])
        .status()
        .ok();

    let network = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter network name to inspect")
        .interact()
        .unwrap();

    println!("\n📊 Network Details:");
    Command::new("docker")
        .args(&["network", "inspect", &network])
        .status()
        .ok();

    // Show containers connected to this network
    println!("\n🐳 Connected Containers:");
    let containers_in_network = Command::new("docker")
        .args(&[
            "network",
            "inspect",
            &network,
            "--format",
            "{{range .Containers}}{{.Name}} {{.IPv4Address}}{{end}}",
        ])
        .output();

    if let Ok(out) = containers_in_network {
        let result = String::from_utf8_lossy(&out.stdout);
        if result.trim().is_empty() {
            println!("  No containers connected");
        } else {
            println!("  {}", result);
        }
    }
}

fn prune_docker_networks() {
    println!("🧹 Prune Unused Docker Networks");
    println!("=================================\n");

    println!("📋 Current Networks:");
    Command::new("docker")
        .args(&["network", "ls"])
        .status()
        .ok();

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Remove all unused networks?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        Command::new("docker")
            .args(&["network", "prune", "-f"])
            .status()
            .ok();

        println!("✅ Unused networks pruned");

        println!("\n📋 Remaining Networks:");
        Command::new("docker")
            .args(&["network", "ls"])
            .status()
            .ok();
    }
}

fn fix_docker_networking() {
    let fixes = [
        "🔄 Restart Docker daemon",
        "🌉 Recreate Docker bridge",
        "🔥 Fix iptables rules",
        "🌐 Reset Docker networks",
        "📊 Fix DNS issues",
        "🔧 Fix IP forwarding",
        "🚀 Complete Docker network reset",
        "⬅️ Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🔧 Fix Docker Networking Issues")
        .items(&fixes)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("🔄 Restarting Docker daemon...");
            Command::new("sudo")
                .args(&["systemctl", "restart", "docker"])
                .status()
                .ok();
            println!("✅ Docker daemon restarted");
        }
        1 => {
            println!("🌉 Recreating Docker bridge...");

            let confirm = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("⚠️ This will stop all containers. Continue?")
                .default(false)
                .interact()
                .unwrap();

            if confirm {
                // Stop Docker
                Command::new("sudo")
                    .args(&["systemctl", "stop", "docker"])
                    .status()
                    .ok();

                // Remove bridge
                Command::new("sudo")
                    .args(&["ip", "link", "delete", "docker0"])
                    .status()
                    .ok();

                // Start Docker (will recreate bridge)
                Command::new("sudo")
                    .args(&["systemctl", "start", "docker"])
                    .status()
                    .ok();

                println!("✅ Docker bridge recreated");
            }
        }
        2 => {
            println!("🔥 Fixing iptables rules...");

            // Add Docker chain if missing
            Command::new("sudo")
                .args(&["iptables", "-t", "nat", "-N", "DOCKER"])
                .status()
                .ok();
            Command::new("sudo")
                .args(&["iptables", "-t", "filter", "-N", "DOCKER"])
                .status()
                .ok();

            // Allow Docker bridge traffic
            Command::new("sudo")
                .args(&["iptables", "-A", "FORWARD", "-i", "docker0", "-j", "ACCEPT"])
                .status()
                .ok();
            Command::new("sudo")
                .args(&["iptables", "-A", "FORWARD", "-o", "docker0", "-j", "ACCEPT"])
                .status()
                .ok();

            // Restart Docker to rebuild rules
            Command::new("sudo")
                .args(&["systemctl", "restart", "docker"])
                .status()
                .ok();

            println!("✅ iptables rules fixed");
        }
        3 => {
            println!("🌐 Resetting Docker networks...");

            let confirm = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("⚠️ This will remove all custom networks. Continue?")
                .default(false)
                .interact()
                .unwrap();

            if confirm {
                // Stop all containers
                Command::new("docker")
                    .args(&["stop", "$(docker ps -q)"])
                    .status()
                    .ok();

                // Remove all custom networks
                Command::new("docker")
                    .args(&["network", "prune", "-f"])
                    .status()
                    .ok();

                // Restart Docker
                Command::new("sudo")
                    .args(&["systemctl", "restart", "docker"])
                    .status()
                    .ok();

                println!("✅ Docker networks reset");
            }
        }
        4 => {
            println!("📊 Fixing DNS issues...");

            // Restart systemd-resolved
            Command::new("sudo")
                .args(&["systemctl", "restart", "systemd-resolved"])
                .status()
                .ok();

            // Update Docker daemon configuration
            let config_exists = Path::new("/etc/docker/daemon.json").exists();

            if config_exists {
                println!("📝 Docker daemon config exists. Please manually check DNS settings in /etc/docker/daemon.json");
            } else {
                println!("📝 Creating Docker daemon DNS configuration...");
                let config = r#"{
    "dns": ["8.8.8.8", "8.8.4.4"]
}"#;
                std::fs::create_dir_all("/etc/docker").ok();
                std::fs::write("/etc/docker/daemon.json", config).ok();
            }

            // Restart Docker
            Command::new("sudo")
                .args(&["systemctl", "restart", "docker"])
                .status()
                .ok();

            println!("✅ DNS configuration updated");
        }
        5 => {
            println!("🔧 Fixing IP forwarding...");

            // Enable IP forwarding
            Command::new("sudo")
                .args(&["sysctl", "net.ipv4.ip_forward=1"])
                .status()
                .ok();

            // Make persistent
            let sysctl_content = "net.ipv4.ip_forward=1\n";
            std::fs::write("/etc/sysctl.d/docker.conf", sysctl_content).ok();

            println!("✅ IP forwarding enabled");
        }
        6 => {
            println!("🚀 Complete Docker network reset...");

            let confirm = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("⚠️ This will reset ALL Docker networking. Continue?")
                .default(false)
                .interact()
                .unwrap();

            if confirm {
                println!("Stopping Docker...");
                Command::new("sudo")
                    .args(&["systemctl", "stop", "docker"])
                    .status()
                    .ok();

                println!("Removing Docker bridge...");
                Command::new("sudo")
                    .args(&["ip", "link", "delete", "docker0"])
                    .status()
                    .ok();

                println!("Cleaning iptables rules...");
                Command::new("sudo")
                    .args(&["iptables", "-t", "nat", "-F", "DOCKER"])
                    .status()
                    .ok();
                Command::new("sudo")
                    .args(&["iptables", "-t", "filter", "-F", "DOCKER"])
                    .status()
                    .ok();

                println!("Restarting Docker...");
                Command::new("sudo")
                    .args(&["systemctl", "start", "docker"])
                    .status()
                    .ok();

                println!("✅ Complete Docker network reset completed");
            }
        }
        _ => {}
    }
}

fn docker_dns_troubleshooting() {
    println!("📡 Docker DNS Troubleshooting");
    println!("==============================\n");

    // Check Docker DNS configuration
    println!("1️⃣ Docker Daemon DNS Configuration:");
    let daemon_config = Path::new("/etc/docker/daemon.json");
    if daemon_config.exists() {
        Command::new("cat")
            .args(&["/etc/docker/daemon.json"])
            .status()
            .ok();
    } else {
        println!("  No custom daemon configuration found");
    }

    // Test DNS from host
    println!("\n2️⃣ Host DNS Resolution:");
    Command::new("nslookup").args(&["google.com"]).status().ok();

    // Check systemd-resolved
    println!("\n3️⃣ systemd-resolved Status:");
    Command::new("systemctl")
        .args(&["status", "systemd-resolved", "--no-pager"])
        .status()
        .ok();

    // Test DNS inside containers
    println!("\n4️⃣ Container DNS Tests:");
    let containers = Command::new("docker")
        .args(&["ps", "--format", "{{.Names}}"])
        .output();

    if let Ok(out) = containers {
        for container in String::from_utf8_lossy(&out.stdout).lines().take(3) {
            if !container.trim().is_empty() {
                println!("\n--- Container: {} ---", container);

                println!("DNS config:");
                Command::new("docker")
                    .args(&["exec", container, "cat", "/etc/resolv.conf"])
                    .status()
                    .ok();

                println!("DNS test:");
                Command::new("docker")
                    .args(&["exec", container, "nslookup", "google.com"])
                    .status()
                    .ok();
            }
        }
    }

    // Check Docker internal DNS
    println!("\n5️⃣ Docker Internal DNS Server:");
    Command::new("docker")
        .args(&["inspect", "bridge", "--format", "{{.IPAM.Config}}"])
        .status()
        .ok();

    // DNS fixes
    println!("\n🔧 DNS Fix Options:");
    let fixes = [
        "Set custom DNS in daemon.json",
        "Restart systemd-resolved",
        "Use host networking",
        "Manual DNS configuration",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select DNS fix")
        .items(&fixes)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("📝 Setting custom DNS in daemon configuration...");
            let dns1 = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Primary DNS server")
                .default("8.8.8.8".to_string())
                .interact()
                .unwrap();

            let dns2 = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Secondary DNS server")
                .default("8.8.4.4".to_string())
                .interact()
                .unwrap();

            let config = format!(
                r#"{{
    "dns": ["{}", "{}"]
}}"#,
                dns1, dns2
            );

            std::fs::create_dir_all("/etc/docker").ok();
            std::fs::write("/etc/docker/daemon.json", config).ok();

            println!("Restarting Docker...");
            Command::new("sudo")
                .args(&["systemctl", "restart", "docker"])
                .status()
                .ok();
            println!("✅ Custom DNS configured");
        }
        1 => {
            Command::new("sudo")
                .args(&["systemctl", "restart", "systemd-resolved"])
                .status()
                .ok();
            Command::new("sudo")
                .args(&["systemctl", "restart", "docker"])
                .status()
                .ok();
            println!("✅ systemd-resolved restarted");
        }
        _ => {}
    }
}

fn port_mapping_analysis() {
    println!("🔌 Port Mapping Analysis");
    println!("=========================\n");

    // List all port mappings
    println!("📊 All Container Port Mappings:");
    Command::new("docker")
        .args(&["ps", "--format", "table {{.Names}}\t{{.Ports}}"])
        .status()
        .ok();

    // Detailed port analysis
    println!("\n🔍 Detailed Port Analysis:");
    let containers = Command::new("docker")
        .args(&["ps", "--format", "{{.Names}}"])
        .output();

    if let Ok(out) = containers {
        for container in String::from_utf8_lossy(&out.stdout).lines() {
            if !container.trim().is_empty() {
                println!("\n--- {} ---", container);

                // Port mappings
                Command::new("docker")
                    .args(&["port", container])
                    .status()
                    .ok();

                // Check if ports are accessible
                let ports = Command::new("docker").args(&["port", container]).output();

                if let Ok(port_out) = ports {
                    for line in String::from_utf8_lossy(&port_out.stdout).lines() {
                        if line.contains("0.0.0.0:") {
                            if let Some(port_part) = line.split("0.0.0.0:").nth(1) {
                                if let Some(port) = port_part.split_whitespace().next() {
                                    println!("Testing port {}...", port);
                                    let test = Command::new("nc")
                                        .args(&["-zv", "localhost", port])
                                        .output();

                                    match test {
                                        Ok(t) if t.status.success() => {
                                            println!("  ✅ Port {} accessible", port)
                                        }
                                        _ => println!("  ❌ Port {} not accessible", port),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Check iptables NAT rules for ports
    println!("\n🔥 iptables NAT Rules for Docker:");
    Command::new("sudo")
        .args(&[
            "iptables",
            "-t",
            "nat",
            "-L",
            "DOCKER",
            "-n",
            "--line-numbers",
        ])
        .status()
        .ok();

    // Check for port conflicts
    println!("\n⚠️ Port Conflict Detection:");
    println!("Checking for conflicts between Docker and host services...");

    let host_ports = Command::new("ss").args(&["-tlnp"]).output();

    if let Ok(host_out) = host_ports {
        let docker_ports = Command::new("docker")
            .args(&["ps", "--format", "{{.Ports}}"])
            .output();

        if let Ok(docker_out) = docker_ports {
            // Simple conflict detection
            let host_lines = String::from_utf8_lossy(&host_out.stdout);
            let docker_lines = String::from_utf8_lossy(&docker_out.stdout);

            println!("Host services using common ports:");
            for line in host_lines.lines() {
                if line.contains(":80 ")
                    || line.contains(":443 ")
                    || line.contains(":3000 ")
                    || line.contains(":8080 ")
                {
                    println!("  {}", line);
                }
            }

            println!("\nDocker port mappings:");
            for line in docker_lines.lines() {
                if !line.trim().is_empty() {
                    println!("  {}", line);
                }
            }
        }
    }
}

fn docker_performance_testing() {
    println!("🚀 Docker Network Performance Testing");
    println!("======================================\n");

    let test_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select performance test")
        .items(&[
            "🌐 Container to Internet",
            "🐳 Container to Container",
            "🖥️ Container to Host",
            "📊 Bridge Performance",
            "🔧 Comprehensive Test",
        ])
        .default(0)
        .interact()
        .unwrap();

    match test_type {
        0 => test_container_to_internet(),
        1 => test_container_to_container(),
        2 => test_container_to_host(),
        3 => test_bridge_performance(),
        4 => comprehensive_docker_test(),
        _ => {}
    }
}

fn test_container_to_internet() {
    println!("🌐 Testing Container to Internet Performance");

    // Create a test container if needed
    let test_container = "nettest_container";

    println!("📦 Creating test container...");
    let create_result = Command::new("docker")
        .args(&[
            "run",
            "-d",
            "--name",
            test_container,
            "alpine:latest",
            "sleep",
            "300",
        ])
        .status();

    if create_result.is_err() {
        // Container might already exist, try to start it
        Command::new("docker")
            .args(&["start", test_container])
            .status()
            .ok();
    }

    // Install networking tools
    println!("🔧 Installing test tools...");
    Command::new("docker")
        .args(&[
            "exec",
            test_container,
            "apk",
            "add",
            "curl",
            "iperf3",
            "iputils",
        ])
        .status()
        .ok();

    // Test internet connectivity
    println!("\n📡 Testing Internet Connectivity:");
    Command::new("docker")
        .args(&["exec", test_container, "ping", "-c", "10", "8.8.8.8"])
        .status()
        .ok();

    // Test HTTP download speed
    println!("\n📥 Testing HTTP Download Speed:");
    Command::new("docker")
        .args(&[
            "exec",
            test_container,
            "curl",
            "-o",
            "/dev/null",
            "-w",
            "Speed: %{speed_download} bytes/sec\n",
            "http://speedtest.ftp.otenet.gr/files/test1Mb.db",
        ])
        .status()
        .ok();

    // Cleanup
    println!("\n🧹 Cleaning up...");
    Command::new("docker")
        .args(&["rm", "-f", test_container])
        .status()
        .ok();
}

fn test_container_to_container() {
    println!("🐳 Testing Container to Container Performance");

    let container1 = "nettest1";
    let container2 = "nettest2";

    println!("📦 Creating test containers...");

    // Create first container (server)
    Command::new("docker")
        .args(&[
            "run",
            "-d",
            "--name",
            container1,
            "alpine:latest",
            "sleep",
            "300",
        ])
        .status()
        .ok();

    // Create second container (client)
    Command::new("docker")
        .args(&[
            "run",
            "-d",
            "--name",
            container2,
            "alpine:latest",
            "sleep",
            "300",
        ])
        .status()
        .ok();

    // Install tools
    println!("🔧 Installing tools in containers...");
    for container in &[container1, container2] {
        Command::new("docker")
            .args(&["exec", *container, "apk", "add", "iperf3", "iputils"])
            .status()
            .ok();
    }

    // Get container IPs
    let ip1 = Command::new("docker")
        .args(&[
            "inspect",
            container1,
            "--format",
            "{{.NetworkSettings.IPAddress}}",
        ])
        .output();

    if let Ok(ip_out) = ip1 {
        let ip = String::from_utf8_lossy(&ip_out.stdout)
            .trim()
            .to_string()
            .to_string();
        if !ip.is_empty() {
            println!("\n📡 Testing ping between containers:");
            Command::new("docker")
                .args(&["exec", container2, "ping", "-c", "5", &ip])
                .status()
                .ok();

            println!("\n🚀 Starting iperf3 server in container1...");
            Command::new("docker")
                .args(&["exec", "-d", container1, "iperf3", "-s"])
                .status()
                .ok();

            std::thread::sleep(std::time::Duration::from_secs(2));

            println!("📊 Running bandwidth test...");
            Command::new("docker")
                .args(&["exec", container2, "iperf3", "-c", &ip, "-t", "10"])
                .status()
                .ok();
        }
    }

    // Cleanup
    println!("\n🧹 Cleaning up...");
    Command::new("docker")
        .args(&["rm", "-f", container1, container2])
        .status()
        .ok();
}

fn test_container_to_host() {
    println!("🖥️ Testing Container to Host Performance");

    let test_container = "host_test_container";

    // Create container
    println!("📦 Creating test container...");
    Command::new("docker")
        .args(&[
            "run",
            "-d",
            "--name",
            test_container,
            "alpine:latest",
            "sleep",
            "300",
        ])
        .status()
        .ok();

    // Install tools
    Command::new("docker")
        .args(&["exec", test_container, "apk", "add", "iperf3", "iputils"])
        .status()
        .ok();

    // Get docker bridge IP (host IP from container perspective)
    let bridge_ip = Command::new("docker")
        .args(&["exec", test_container, "ip", "route", "show", "default"])
        .output();

    if let Ok(route_out) = bridge_ip {
        let route_str = String::from_utf8_lossy(&route_out.stdout);
        for line in route_str.lines() {
            if line.contains("default via") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 2 {
                    let gateway = parts[2];
                    println!("\n📡 Testing connection to host via {}:", gateway);

                    // Ping test
                    Command::new("docker")
                        .args(&["exec", test_container, "ping", "-c", "5", gateway])
                        .status()
                        .ok();

                    // Check if iperf3 is available on host
                    let iperf_check = Command::new("which").arg("iperf3").status();
                    if let Ok(s) = iperf_check {
                        if s.success() {
                            println!("\n🚀 Starting iperf3 server on host...");
                            println!("Note: You need to manually start 'iperf3 -s' on the host");

                            let proceed = Confirm::with_theme(&ColorfulTheme::default())
                                .with_prompt("Is iperf3 server running on host?")
                                .default(false)
                                .interact()
                                .unwrap();

                            if proceed {
                                Command::new("docker")
                                    .args(&[
                                        "exec",
                                        test_container,
                                        "iperf3",
                                        "-c",
                                        gateway,
                                        "-t",
                                        "10",
                                    ])
                                    .status()
                                    .ok();
                            }
                        }
                    }
                }
                break;
            }
        }
    }

    // Cleanup
    println!("\n🧹 Cleaning up...");
    Command::new("docker")
        .args(&["rm", "-f", test_container])
        .status()
        .ok();
}

fn test_bridge_performance() {
    println!("📊 Testing Docker Bridge Performance");

    // Analyze bridge statistics
    println!("🌉 Docker Bridge Statistics:");
    Command::new("ip")
        .args(&["addr", "show", "docker0"])
        .status()
        .ok();

    println!("\n📈 Bridge Traffic Statistics:");
    Command::new("cat")
        .args(&["/sys/class/net/docker0/statistics/rx_bytes"])
        .status()
        .ok();
    Command::new("cat")
        .args(&["/sys/class/net/docker0/statistics/tx_bytes"])
        .status()
        .ok();

    // Monitor bridge for a short period
    println!("\n📊 Monitoring bridge for 30 seconds...");
    println!("Generate Docker network traffic to see bridge activity");

    let initial_rx = std::fs::read_to_string("/sys/class/net/docker0/statistics/rx_bytes")
        .unwrap_or_default()
        .trim()
        .parse::<u64>()
        .unwrap_or(0);

    let initial_tx = std::fs::read_to_string("/sys/class/net/docker0/statistics/tx_bytes")
        .unwrap_or_default()
        .trim()
        .parse::<u64>()
        .unwrap_or(0);

    for i in 1..=30 {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let current_rx = std::fs::read_to_string("/sys/class/net/docker0/statistics/rx_bytes")
            .unwrap_or_default()
            .trim()
            .parse::<u64>()
            .unwrap_or(0);

        let current_tx = std::fs::read_to_string("/sys/class/net/docker0/statistics/tx_bytes")
            .unwrap_or_default()
            .trim()
            .parse::<u64>()
            .unwrap_or(0);

        let rx_rate = (current_rx - initial_rx) / i;
        let tx_rate = (current_tx - initial_tx) / i;

        if i % 5 == 0 {
            println!(
                "  [{:02}s] RX: {} KB/s, TX: {} KB/s",
                i,
                rx_rate / 1024,
                tx_rate / 1024
            );
        }
    }

    println!("\n✅ Bridge monitoring complete");
}

fn comprehensive_docker_test() {
    println!("🔧 Comprehensive Docker Network Test");
    println!("=====================================\n");

    println!("🔍 Running all Docker network tests...\n");

    // 1. Network status
    println!("1️⃣ Docker Network Status:");
    docker_network_status();

    // 2. Create test containers
    println!("\n2️⃣ Creating test environment...");
    let containers = ["test1", "test2", "test3"];

    for container in &containers {
        Command::new("docker")
            .args(&[
                "run",
                "-d",
                "--name",
                container,
                "alpine:latest",
                "sleep",
                "600",
            ])
            .status()
            .ok();

        // Install tools
        Command::new("docker")
            .args(&["exec", container, "apk", "add", "curl", "iperf3", "iputils"])
            .status()
            .ok();
    }

    // 3. Test inter-container communication
    println!("\n3️⃣ Testing inter-container communication:");
    for i in 0..containers.len() {
        for j in (i + 1)..containers.len() {
            let container1 = containers[i];
            let container2 = containers[j];

            let ip = Command::new("docker")
                .args(&[
                    "inspect",
                    container2,
                    "--format",
                    "{{.NetworkSettings.IPAddress}}",
                ])
                .output();

            if let Ok(ip_out) = ip {
                let ip_str = String::from_utf8_lossy(&ip_out.stdout).trim().to_string();
                if !ip_str.is_empty() {
                    println!("  {} -> {} ({})", container1, container2, ip_str);
                    Command::new("docker")
                        .args(&["exec", container1, "ping", "-c", "2", &ip_str])
                        .status()
                        .ok();
                }
            }
        }
    }

    // 4. Test internet connectivity from containers
    println!("\n4️⃣ Testing internet connectivity:");
    for container in &containers {
        println!("  {} -> Internet:", container);
        Command::new("docker")
            .args(&["exec", container, "ping", "-c", "2", "8.8.8.8"])
            .status()
            .ok();
    }

    // 5. DNS resolution test
    println!("\n5️⃣ Testing DNS resolution:");
    for container in &containers {
        println!("  {} DNS test:", container);
        Command::new("docker")
            .args(&["exec", container, "nslookup", "google.com"])
            .status()
            .ok();
    }

    // Cleanup
    println!("\n🧹 Cleaning up test environment...");
    for container in &containers {
        Command::new("docker")
            .args(&["rm", "-f", container])
            .status()
            .ok();
    }

    println!("\n✅ Comprehensive Docker network test completed");
}

fn qemu_kvm_bridge_management() {
    let options = [
        "🌉 Create Bridge Interface",
        "🔧 Configure Bridge",
        "🔍 Bridge Status & Diagnostics",
        "🖥️ VM Network Troubleshooting",
        "🔌 TAP Interface Management",
        "📊 Bridge Performance Analysis",
        "⬅️ Back",
    ];

    loop {
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🖥️ QEMU/KVM Bridge Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => create_bridge_interface(),
            1 => configure_bridge(),
            2 => bridge_status_diagnostics(),
            3 => vm_network_troubleshooting(),
            4 => tap_interface_management(),
            5 => bridge_performance_analysis(),
            _ => break,
        }
    }
}

fn create_bridge_interface() {
    println!("🌉 Create Bridge Interface");
    println!("===========================\n");

    let bridge_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter bridge name")
        .default("br0".to_string())
        .interact()
        .unwrap();

    // Check if bridge already exists
    let bridge_check = Command::new("ip")
        .args(&["link", "show", &bridge_name])
        .output();

    if let Ok(out) = bridge_check {
        if out.status.success() {
            println!("⚠️ Bridge {} already exists", bridge_name);
            return;
        }
    }

    println!("🔧 Creating bridge {}...", bridge_name);

    // Create bridge
    let create_result = Command::new("sudo")
        .args(&["ip", "link", "add", "name", &bridge_name, "type", "bridge"])
        .status();

    match create_result {
        Ok(s) if s.success() => {
            println!("✅ Bridge {} created successfully", bridge_name);

            // Bring bridge up
            Command::new("sudo")
                .args(&["ip", "link", "set", "dev", &bridge_name, "up"])
                .status()
                .ok();

            println!("✅ Bridge {} brought up", bridge_name);

            // Configure IP if requested
            let assign_ip = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Assign IP address to bridge?")
                .default(true)
                .interact()
                .unwrap();

            if assign_ip {
                let ip_addr = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter IP address/CIDR (e.g., 192.168.100.1/24)")
                    .interact()
                    .unwrap();

                let ip_result = Command::new("sudo")
                    .args(&["ip", "addr", "add", &ip_addr, "dev", &bridge_name])
                    .status();

                match ip_result {
                    Ok(s) if s.success() => println!("✅ IP {} assigned to bridge", ip_addr),
                    _ => println!("❌ Failed to assign IP to bridge"),
                }
            }

            // Add physical interface to bridge
            let add_interface = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Add physical interface to bridge?")
                .default(false)
                .interact()
                .unwrap();

            if add_interface {
                // List available interfaces
                println!("\n📋 Available interfaces:");
                Command::new("ip").args(&["link", "show"]).status().ok();

                let interface = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter interface to add to bridge")
                    .interact()
                    .unwrap();

                println!("🔌 Adding {} to bridge {}...", interface, bridge_name);

                // Set interface down
                Command::new("sudo")
                    .args(&["ip", "link", "set", "dev", &interface, "down"])
                    .status()
                    .ok();

                // Add to bridge
                let add_result = Command::new("sudo")
                    .args(&[
                        "ip",
                        "link",
                        "set",
                        "dev",
                        &interface,
                        "master",
                        &bridge_name,
                    ])
                    .status();

                // Bring interface back up
                Command::new("sudo")
                    .args(&["ip", "link", "set", "dev", &interface, "up"])
                    .status()
                    .ok();

                match add_result {
                    Ok(s) if s.success() => println!("✅ Interface {} added to bridge", interface),
                    _ => println!("❌ Failed to add interface to bridge"),
                }
            }

            // Show final bridge configuration
            println!("\n📊 Bridge Configuration:");
            Command::new("ip")
                .args(&["addr", "show", &bridge_name])
                .status()
                .ok();

            println!("\n🌉 Bridge Members:");
            Command::new("bridge").args(&["link", "show"]).status().ok();
        }
        _ => println!("❌ Failed to create bridge"),
    }
}

fn configure_bridge() {
    println!("🔧 Configure Bridge");
    println!("====================\n");

    // List existing bridges
    println!("📋 Existing Bridges:");
    Command::new("bridge").args(&["link", "show"]).status().ok();

    let bridge_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter bridge name to configure")
        .interact()
        .unwrap();

    let config_options = [
        "🔧 Set STP (Spanning Tree Protocol)",
        "⏱️ Set Forward Delay",
        "📊 Set Hello Time",
        "🔄 Set Max Age",
        "🏷️ Set Bridge Priority",
        "🌐 Configure VLAN",
        "📝 Show Current Configuration",
        "⬅️ Back",
    ];

    loop {
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(&format!("Configure Bridge: {}", bridge_name))
            .items(&config_options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => {
                let enable_stp = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enable STP?")
                    .default(false)
                    .interact()
                    .unwrap();

                let stp_value = if enable_stp { "1" } else { "0" };
                Command::new("sudo")
                    .args(&[
                        "ip",
                        "link",
                        "set",
                        "dev",
                        &bridge_name,
                        "type",
                        "bridge",
                        "stp_state",
                        stp_value,
                    ])
                    .status()
                    .ok();

                println!(
                    "✅ STP {} for bridge",
                    if enable_stp { "enabled" } else { "disabled" }
                );
            }
            1 => {
                let delay = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter forward delay in seconds (default: 2)")
                    .default("2".to_string())
                    .interact()
                    .unwrap();

                Command::new("sudo")
                    .args(&[
                        "ip",
                        "link",
                        "set",
                        "dev",
                        &bridge_name,
                        "type",
                        "bridge",
                        "forward_delay",
                        &delay,
                    ])
                    .status()
                    .ok();

                println!("✅ Forward delay set to {} seconds", delay);
            }
            2 => {
                let hello_time = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter hello time in seconds (default: 2)")
                    .default("2".to_string())
                    .interact()
                    .unwrap();

                Command::new("sudo")
                    .args(&[
                        "ip",
                        "link",
                        "set",
                        "dev",
                        &bridge_name,
                        "type",
                        "bridge",
                        "hello_time",
                        &hello_time,
                    ])
                    .status()
                    .ok();

                println!("✅ Hello time set to {} seconds", hello_time);
            }
            3 => {
                let max_age = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter max age in seconds (default: 20)")
                    .default("20".to_string())
                    .interact()
                    .unwrap();

                Command::new("sudo")
                    .args(&[
                        "ip",
                        "link",
                        "set",
                        "dev",
                        &bridge_name,
                        "type",
                        "bridge",
                        "max_age",
                        &max_age,
                    ])
                    .status()
                    .ok();

                println!("✅ Max age set to {} seconds", max_age);
            }
            4 => {
                let priority = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter bridge priority (0-65535, default: 32768)")
                    .default("32768".to_string())
                    .interact()
                    .unwrap();

                Command::new("sudo")
                    .args(&[
                        "ip",
                        "link",
                        "set",
                        "dev",
                        &bridge_name,
                        "type",
                        "bridge",
                        "priority",
                        &priority,
                    ])
                    .status()
                    .ok();

                println!("✅ Bridge priority set to {}", priority);
            }
            5 => {
                println!("🌐 VLAN Configuration (using bridge-vlan command):");
                let vlan_id = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter VLAN ID")
                    .interact()
                    .unwrap();

                println!("Available VLAN operations:");
                println!("  bridge vlan add dev {} vid {} self", bridge_name, vlan_id);
                println!(
                    "  bridge vlan add dev {} vid {} pvid untagged",
                    bridge_name, vlan_id
                );
                println!("Run these commands manually with sudo");
            }
            6 => {
                println!("📝 Current Bridge Configuration:");
                Command::new("ip")
                    .args(&["link", "show", &bridge_name])
                    .status()
                    .ok();

                println!("\n🌉 Bridge Details:");
                Command::new("bridge")
                    .args(&["link", "show", "dev", &bridge_name])
                    .status()
                    .ok();

                println!("\n📊 Bridge FDB (Forwarding Database):");
                Command::new("bridge")
                    .args(&["fdb", "show", "br", &bridge_name])
                    .status()
                    .ok();

                println!("\n🔧 STP Information:");
                let stp_info_path = format!("/sys/class/net/{}/bridge/stp_state", bridge_name);
                if Path::new(&stp_info_path).exists() {
                    Command::new("cat").args(&[&stp_info_path]).status().ok();
                }
            }
            _ => break,
        }
    }
}

fn bridge_status_diagnostics() {
    println!("🔍 Bridge Status & Diagnostics");
    println!("===============================\n");

    // List all bridges
    println!("🌉 All Linux Bridges:");
    Command::new("bridge").args(&["link", "show"]).status().ok();

    // Select bridge for detailed analysis
    let bridge = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter bridge name for detailed diagnostics")
        .interact()
        .unwrap();

    println!("\n📊 Detailed Bridge Analysis: {}", bridge);
    println!("{}", "=".repeat(40));

    // Basic bridge information
    println!("\n1️⃣ Basic Information:");
    Command::new("ip")
        .args(&["link", "show", &bridge])
        .status()
        .ok();
    Command::new("ip")
        .args(&["addr", "show", &bridge])
        .status()
        .ok();

    // Bridge members
    println!("\n2️⃣ Bridge Members:");
    Command::new("bridge")
        .args(&["link", "show", "br", &bridge])
        .status()
        .ok();

    // MAC address learning table
    println!("\n3️⃣ MAC Address Table (FDB):");
    Command::new("bridge")
        .args(&["fdb", "show", "br", &bridge])
        .status()
        .ok();

    // STP status if available
    println!("\n4️⃣ Spanning Tree Protocol Status:");
    let stp_path = format!("/sys/class/net/{}/bridge", bridge);
    if Path::new(&stp_path).exists() {
        println!("STP State:");
        Command::new("cat")
            .args(&[&format!("{}/stp_state", stp_path)])
            .status()
            .ok();

        println!("Root ID:");
        Command::new("cat")
            .args(&[&format!("{}/root_id", stp_path)])
            .status()
            .ok();

        println!("Bridge ID:");
        Command::new("cat")
            .args(&[&format!("{}/bridge_id", stp_path)])
            .status()
            .ok();
    } else {
        println!("  Bridge sysfs information not available");
    }

    // Traffic statistics
    println!("\n5️⃣ Traffic Statistics:");
    let stats_path = format!("/sys/class/net/{}/statistics", bridge);
    if Path::new(&stats_path).exists() {
        println!("RX Bytes:");
        Command::new("cat")
            .args(&[&format!("{}/rx_bytes", stats_path)])
            .status()
            .ok();

        println!("TX Bytes:");
        Command::new("cat")
            .args(&[&format!("{}/tx_bytes", stats_path)])
            .status()
            .ok();

        println!("RX Packets:");
        Command::new("cat")
            .args(&[&format!("{}/rx_packets", stats_path)])
            .status()
            .ok();

        println!("TX Packets:");
        Command::new("cat")
            .args(&[&format!("{}/tx_packets", stats_path)])
            .status()
            .ok();
    }

    // VLAN information
    println!("\n6️⃣ VLAN Configuration:");
    Command::new("bridge")
        .args(&["vlan", "show", "br", &bridge])
        .status()
        .ok();

    // Connected devices analysis
    println!("\n7️⃣ Connected Devices Analysis:");
    let members_output = Command::new("bridge")
        .args(&["link", "show", "br", &bridge])
        .output();

    if let Ok(out) = members_output {
        let output_str = String::from_utf8_lossy(&out.stdout);
        for line in output_str.lines() {
            if line.contains("master") && line.contains(&bridge) {
                // Extract interface name
                if let Some(interface) = line.split_whitespace().nth(1) {
                    let clean_interface = interface.trim_end_matches(':');
                    println!("\n--- Member Interface: {} ---", clean_interface);

                    // Show interface details
                    Command::new("ip")
                        .args(&["link", "show", clean_interface])
                        .status()
                        .ok();

                    // Show interface statistics
                    println!("Statistics:");
                    let if_stats_path = format!("/sys/class/net/{}/statistics", clean_interface);
                    if Path::new(&if_stats_path).exists() {
                        let rx_bytes =
                            std::fs::read_to_string(&format!("{}/rx_bytes", if_stats_path))
                                .unwrap_or_default()
                                .trim()
                                .to_string();
                        let tx_bytes =
                            std::fs::read_to_string(&format!("{}/tx_bytes", if_stats_path))
                                .unwrap_or_default()
                                .trim()
                                .to_string();

                        println!("  RX: {} bytes, TX: {} bytes", rx_bytes, tx_bytes);
                    }
                }
            }
        }
    }

    // Performance analysis
    println!("\n8️⃣ Performance Metrics:");
    monitor_bridge_performance(&bridge, 10);
}

fn monitor_bridge_performance(bridge: &str, duration: u64) {
    println!(
        "📈 Monitoring bridge {} for {} seconds...",
        bridge, duration
    );

    let stats_path = format!("/sys/class/net/{}/statistics", bridge);
    if !Path::new(&stats_path).exists() {
        println!("❌ Bridge statistics not available");
        return;
    }

    let initial_rx = std::fs::read_to_string(&format!("{}/rx_bytes", stats_path))
        .unwrap_or_default()
        .trim()
        .parse::<u64>()
        .unwrap_or(0);

    let initial_tx = std::fs::read_to_string(&format!("{}/tx_bytes", stats_path))
        .unwrap_or_default()
        .trim()
        .parse::<u64>()
        .unwrap_or(0);

    for i in 1..=duration {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let current_rx = std::fs::read_to_string(&format!("{}/rx_bytes", stats_path))
            .unwrap_or_default()
            .trim()
            .parse::<u64>()
            .unwrap_or(0);

        let current_tx = std::fs::read_to_string(&format!("{}/tx_bytes", stats_path))
            .unwrap_or_default()
            .trim()
            .parse::<u64>()
            .unwrap_or(0);

        let rx_rate = (current_rx - initial_rx) / i;
        let tx_rate = (current_tx - initial_tx) / i;

        if i % 2 == 0 {
            println!(
                "  [{:02}s] RX: {} KB/s, TX: {} KB/s",
                i,
                rx_rate / 1024,
                tx_rate / 1024
            );
        }
    }

    println!("✅ Bridge monitoring complete");
}

fn vm_network_troubleshooting() {
    println!("🖥️ VM Network Troubleshooting");
    println!("==============================\n");

    let vm_options = [
        "🔍 Diagnose VM Network Issues",
        "🌉 Check Bridge Connectivity",
        "🔌 TAP Interface Issues",
        "📡 QEMU Network Configuration",
        "🚀 VM Performance Testing",
        "⬅️ Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("VM Network Troubleshooting")
        .items(&vm_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => diagnose_vm_network_issues(),
        1 => check_bridge_connectivity(),
        2 => troubleshoot_tap_interfaces(),
        3 => check_qemu_network_config(),
        4 => vm_performance_testing(),
        _ => {}
    }
}

fn diagnose_vm_network_issues() {
    println!("🔍 Diagnosing VM Network Issues");
    println!("================================\n");

    // Check if virtualization is enabled
    println!("1️⃣ Checking Virtualization Support:");
    let virt_check = Command::new("grep")
        .args(&["-E", "(vmx|svm)", "/proc/cpuinfo"])
        .output();

    match virt_check {
        Ok(out) if !out.stdout.is_empty() => println!("  ✅ Hardware virtualization supported"),
        _ => println!("  ❌ Hardware virtualization not available"),
    }

    // Check KVM module
    println!("\n2️⃣ Checking KVM Module:");
    let kvm_check = Command::new("lsmod").output();

    if let Ok(out) = kvm_check {
        let modules = String::from_utf8_lossy(&out.stdout);
        if modules.contains("kvm") {
            println!("  ✅ KVM module loaded");
        } else {
            println!("  ❌ KVM module not loaded");
        }
    }

    // Check libvirt
    println!("\n3️⃣ Checking Libvirt:");
    let libvirt_check = Command::new("systemctl")
        .args(&["is-active", "libvirtd"])
        .output();

    if let Ok(out) = libvirt_check {
        let status = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if status == "active" {
            println!("  ✅ Libvirtd is running");
        } else {
            println!("  ❌ Libvirtd is not running");
        }
    }

    // Check bridges
    println!("\n4️⃣ Checking Bridges:");
    Command::new("bridge").args(&["link", "show"]).status().ok();

    // Check TAP interfaces
    println!("\n5️⃣ Checking TAP Interfaces:");
    Command::new("ip").args(&["tuntap", "show"]).status().ok();

    // Check network namespaces
    println!("\n6️⃣ Checking Network Namespaces:");
    Command::new("ip").args(&["netns", "list"]).status().ok();

    // Check iptables rules for virtualization
    println!("\n7️⃣ Checking iptables for Virtualization:");
    Command::new("sudo")
        .args(&[
            "iptables",
            "-L",
            "-n",
            "|",
            "grep",
            "-E",
            "(FORWARD|virbr|tap)",
        ])
        .status()
        .ok();

    // VM-specific diagnostics
    println!("\n8️⃣ QEMU/Libvirt Network Configuration:");

    // Check default libvirt network
    let virsh_check = Command::new("which").arg("virsh").status();
    if let Ok(s) = virsh_check {
        if s.success() {
            println!("Libvirt networks:");
            Command::new("virsh")
                .args(&["net-list", "--all"])
                .status()
                .ok();

            println!("\nDefault network details:");
            Command::new("virsh")
                .args(&["net-dumpxml", "default"])
                .status()
                .ok();
        }
    }
}

fn check_bridge_connectivity() {
    println!("🌉 Check Bridge Connectivity");
    println!("=============================\n");

    let bridge = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter bridge name to test")
        .default("virbr0".to_string())
        .interact()
        .unwrap();

    println!("🔍 Testing bridge connectivity: {}", bridge);

    // Check if bridge exists
    let bridge_exists = Command::new("ip").args(&["link", "show", &bridge]).status();

    match bridge_exists {
        Ok(s) if s.success() => println!("  ✅ Bridge {} exists", bridge),
        _ => {
            println!("  ❌ Bridge {} does not exist", bridge);
            return;
        }
    }

    // Check bridge IP
    println!("\n📊 Bridge IP Configuration:");
    Command::new("ip")
        .args(&["addr", "show", &bridge])
        .status()
        .ok();

    // Test bridge connectivity
    let bridge_ip_output = Command::new("ip").args(&["addr", "show", &bridge]).output();

    if let Ok(out) = bridge_ip_output {
        let output_str = String::from_utf8_lossy(&out.stdout);
        for line in output_str.lines() {
            if line.contains("inet ") && !line.contains("inet6") {
                if let Some(ip_part) = line.trim().split_whitespace().nth(1) {
                    let ip = ip_part.split('/').next().unwrap_or("");
                    if !ip.is_empty() {
                        println!("\n🏓 Testing ping to bridge IP: {}", ip);
                        Command::new("ping").args(&["-c", "3", ip]).status().ok();
                    }
                }
                break;
            }
        }
    }

    // Check bridge members
    println!("\n👥 Bridge Members:");
    Command::new("bridge")
        .args(&["link", "show", "br", &bridge])
        .status()
        .ok();

    // Test member interfaces
    let members = Command::new("bridge")
        .args(&["link", "show", "br", &bridge])
        .output();

    if let Ok(out) = members {
        let output_str = String::from_utf8_lossy(&out.stdout);
        for line in output_str.lines() {
            if line.contains("master") && line.contains(&bridge) {
                if let Some(interface) = line.split_whitespace().nth(1) {
                    let clean_interface = interface.trim_end_matches(':');
                    println!("\n🔌 Testing member interface: {}", clean_interface);

                    // Check if interface is up
                    let if_status = Command::new("ip")
                        .args(&["link", "show", clean_interface])
                        .output();

                    if let Ok(if_out) = if_status {
                        let if_str = String::from_utf8_lossy(&if_out.stdout);
                        if if_str.contains("state UP") {
                            println!("  ✅ Interface {} is UP", clean_interface);
                        } else {
                            println!("  ❌ Interface {} is DOWN", clean_interface);
                        }
                    }
                }
            }
        }
    }

    // Check forwarding
    println!("\n📡 IP Forwarding Status:");
    Command::new("cat")
        .args(&["/proc/sys/net/ipv4/ip_forward"])
        .status()
        .ok();

    // Check iptables rules
    println!("\n🔥 iptables Rules for Bridge:");
    Command::new("sudo")
        .args(&["iptables", "-L", "-n", "|", "grep", &bridge])
        .status()
        .ok();
}

fn troubleshoot_tap_interfaces() {
    println!("🔌 TAP Interface Troubleshooting");
    println!("=================================\n");

    // List TAP interfaces
    println!("📋 TAP Interfaces:");
    Command::new("ip").args(&["tuntap", "show"]).status().ok();

    // Check TAP interface details
    let tap_list = Command::new("ip").args(&["tuntap", "show"]).output();

    if let Ok(out) = tap_list {
        let output_str = String::from_utf8_lossy(&out.stdout);
        for line in output_str.lines() {
            if line.contains("tap") {
                if let Some(tap_name) = line.split(':').next() {
                    println!("\n--- TAP Interface: {} ---", tap_name);

                    // Show TAP details
                    Command::new("ip")
                        .args(&["addr", "show", tap_name])
                        .status()
                        .ok();

                    // Check if attached to bridge
                    Command::new("bridge")
                        .args(&["link", "show", "dev", tap_name])
                        .status()
                        .ok();

                    // Check permissions
                    let tap_path = format!("/dev/{}", tap_name);
                    if Path::new(&tap_path).exists() {
                        Command::new("ls").args(&["-l", &tap_path]).status().ok();
                    }
                }
            }
        }
    }

    // Check TUN/TAP module
    println!("\n🔍 TUN/TAP Module Status:");
    let tun_check = Command::new("lsmod").output();

    if let Ok(out) = tun_check {
        let modules = String::from_utf8_lossy(&out.stdout);
        if modules.contains("tun") {
            println!("  ✅ TUN module loaded");
        } else {
            println!("  ❌ TUN module not loaded");
            println!("  💡 Try: sudo modprobe tun");
        }
    }

    // Check device permissions
    println!("\n🔐 Device Permissions:");
    if Path::new("/dev/net/tun").exists() {
        Command::new("ls")
            .args(&["-l", "/dev/net/tun"])
            .status()
            .ok();
    } else {
        println!("  ❌ /dev/net/tun does not exist");
    }

    // TAP creation test
    let test_tap = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Test TAP interface creation?")
        .default(false)
        .interact()
        .unwrap();

    if test_tap {
        let tap_name = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter test TAP interface name")
            .default("testtap".to_string())
            .interact()
            .unwrap();

        println!("🔧 Creating test TAP interface: {}", tap_name);

        let create_result = Command::new("sudo")
            .args(&["ip", "tuntap", "add", "dev", &tap_name, "mode", "tap"])
            .status();

        match create_result {
            Ok(s) if s.success() => {
                println!("  ✅ TAP interface created successfully");

                // Bring it up
                Command::new("sudo")
                    .args(&["ip", "link", "set", "dev", &tap_name, "up"])
                    .status()
                    .ok();

                println!("  ✅ TAP interface brought up");

                // Show details
                Command::new("ip")
                    .args(&["addr", "show", &tap_name])
                    .status()
                    .ok();

                // Cleanup
                let cleanup = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Remove test TAP interface?")
                    .default(true)
                    .interact()
                    .unwrap();

                if cleanup {
                    Command::new("sudo")
                        .args(&["ip", "link", "delete", &tap_name])
                        .status()
                        .ok();
                    println!("  🧹 Test TAP interface removed");
                }
            }
            _ => println!("  ❌ Failed to create TAP interface"),
        }
    }
}

fn check_qemu_network_config() {
    println!("📡 QEMU Network Configuration");
    println!("==============================\n");

    // Check QEMU installation
    println!("1️⃣ QEMU Installation:");
    let qemu_check = Command::new("which").arg("qemu-system-x86_64").output();

    match qemu_check {
        Ok(out) if out.status.success() => {
            println!(
                "  ✅ QEMU installed at: {}",
                String::from_utf8_lossy(&out.stdout).trim().to_string()
            );

            // Get QEMU version
            Command::new("qemu-system-x86_64")
                .args(&["--version"])
                .status()
                .ok();
        }
        _ => println!("  ❌ QEMU not found"),
    }

    // Check QEMU network backends
    println!("\n2️⃣ QEMU Network Backends:");
    Command::new("qemu-system-x86_64")
        .args(&["-netdev", "help"])
        .status()
        .ok();

    // Check libvirt networks
    println!("\n3️⃣ Libvirt Network Configuration:");
    let virsh_available = Command::new("which").arg("virsh").status();

    if let Ok(s) = virsh_available {
        if s.success() {
            println!("📋 Libvirt networks:");
            Command::new("virsh")
                .args(&["net-list", "--all"])
                .status()
                .ok();

            // Show default network config
            println!("\n📝 Default network configuration:");
            Command::new("virsh")
                .args(&["net-dumpxml", "default"])
                .status()
                .ok();

            // Check network autostart
            println!("\n🔄 Network autostart status:");
            Command::new("virsh")
                .args(&["net-autostart", "default"])
                .status()
                .ok();
        }
    }

    // Check QEMU helper scripts
    println!("\n4️⃣ QEMU Helper Scripts:");
    let qemu_scripts = [
        "/etc/qemu/bridge.conf",
        "/usr/lib/qemu/qemu-bridge-helper",
        "/etc/qemu-kvm/bridge.conf",
    ];

    for script in &qemu_scripts {
        if Path::new(script).exists() {
            println!("  ✅ Found: {}", script);
            if script.contains("bridge.conf") {
                println!("    Contents:");
                Command::new("cat").args(&[script]).status().ok();
            }
        } else {
            println!("  ❌ Not found: {}", script);
        }
    }

    // Check permissions
    println!("\n🔐 Permission Checks:");

    // Check if user is in relevant groups
    let groups_output = Command::new("groups").output();
    if let Ok(out) = groups_output {
        let groups_str = String::from_utf8_lossy(&out.stdout);
        println!("  User groups: {}", groups_str.trim());

        let relevant_groups = ["kvm", "libvirt", "qemu"];
        for group in &relevant_groups {
            if groups_str.contains(group) {
                println!("  ✅ User is in {} group", group);
            } else {
                println!("  ❌ User not in {} group", group);
            }
        }
    }

    // Sample QEMU network configurations
    println!("\n💡 Sample QEMU Network Configurations:");

    println!("\n🌉 Bridge networking:");
    println!("  -netdev bridge,id=net0,br=br0 -device virtio-net-pci,netdev=net0");

    println!("\n🔌 TAP networking:");
    println!("  -netdev tap,id=net0,ifname=tap0,script=no,downscript=no -device virtio-net-pci,netdev=net0");

    println!("\n🖥️ User networking (SLIRP):");
    println!("  -netdev user,id=net0 -device virtio-net-pci,netdev=net0");

    println!("\n🌐 Host networking:");
    println!("  -netdev socket,id=net0,listen=:1234 -device virtio-net-pci,netdev=net0");
}

fn vm_performance_testing() {
    println!("🚀 VM Network Performance Testing");
    println!("==================================\n");

    println!("⚠️ Note: This requires VMs to be running for accurate testing");

    let test_options = [
        "🏓 Ping latency test",
        "📊 Bandwidth testing with iperf3",
        "📈 Throughput analysis",
        "🔄 Bridge performance monitoring",
        "🌉 TAP interface performance",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select performance test")
        .items(&test_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            let vm_ip = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter VM IP address to test")
                .interact()
                .unwrap();

            println!("🏓 Testing ping latency to VM: {}", vm_ip);
            Command::new("ping")
                .args(&["-c", "20", "-i", "0.2", &vm_ip])
                .status()
                .ok();
        }
        1 => {
            println!("📊 Bandwidth Testing with iperf3");
            println!("Note: iperf3 server must be running in the VM");

            let vm_ip = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter VM IP address (iperf3 server)")
                .interact()
                .unwrap();

            let proceed = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Is iperf3 server running in the VM?")
                .default(false)
                .interact()
                .unwrap();

            if proceed {
                println!("Running TCP bandwidth test...");
                Command::new("iperf3")
                    .args(&["-c", &vm_ip, "-t", "30", "-P", "4"])
                    .status()
                    .ok();

                println!("Running UDP bandwidth test...");
                Command::new("iperf3")
                    .args(&["-c", &vm_ip, "-u", "-t", "10"])
                    .status()
                    .ok();
            }
        }
        2 => {
            println!("📈 Throughput Analysis");
            println!("Monitoring network interfaces for 60 seconds...");

            // Monitor all virtualization-related interfaces
            let interfaces = ["virbr0", "br0", "tap0", "vnet0"];

            for interface in &interfaces {
                let if_exists = Command::new("ip")
                    .args(&["link", "show", interface])
                    .status();

                if let Ok(s) = if_exists {
                    if s.success() {
                        println!("\n--- Monitoring {} ---", interface);
                        monitor_bridge_performance(interface, 10);
                    }
                }
            }
        }
        3 => {
            let bridge = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter bridge name to monitor")
                .default("virbr0".to_string())
                .interact()
                .unwrap();

            println!("🔄 Monitoring bridge performance: {}", bridge);
            monitor_bridge_performance(&bridge, 30);
        }
        4 => {
            println!("🌉 TAP Interface Performance");

            // List TAP interfaces
            println!("📋 Available TAP interfaces:");
            Command::new("ip").args(&["tuntap", "show"]).status().ok();

            let tap_interface = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter TAP interface to monitor")
                .interact()
                .unwrap();

            monitor_bridge_performance(&tap_interface, 20);
        }
        _ => {}
    }
}

fn qemu_kvm_integration_status() {
    println!("🖥️ QEMU/KVM Integration Status");
    println!("===============================\n");

    println!("ℹ️ For advanced QEMU/KVM networking features, please use:");
    println!("   🔧 Main Menu → Networking → libvirt/KVM Advanced Networking");
    println!();

    // Basic QEMU/KVM status
    println!("📊 Quick Status Check:");

    // Check virtualization support
    let virt_check = Command::new("grep")
        .args(&["-E", "(vmx|svm)", "/proc/cpuinfo"])
        .output();

    match virt_check {
        Ok(out) if !out.stdout.is_empty() => println!("  ✅ Hardware virtualization supported"),
        _ => println!("  ❌ Hardware virtualization not available"),
    }

    // Check KVM module
    let kvm_check = Command::new("lsmod").output();

    if let Ok(out) = kvm_check {
        let modules = String::from_utf8_lossy(&out.stdout);
        if modules.contains("kvm") {
            println!("  ✅ KVM module loaded");
        } else {
            println!("  ❌ KVM module not loaded");
        }
    }

    // Check libvirt
    let libvirt_check = Command::new("systemctl")
        .args(&["is-active", "libvirtd"])
        .output();

    if let Ok(out) = libvirt_check {
        let status = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if status == "active" {
            println!("  ✅ Libvirtd is running");
        } else {
            println!("  ❌ Libvirtd is not running");
        }
    }

    println!("\n💡 For comprehensive QEMU/KVM networking management,");
    println!("   navigate to: Main Menu → Networking → libvirt/KVM Advanced Networking");
}

fn virtual_interface_management() {
    println!("🔧 Virtual Interface Management");
    println!("===============================\n");

    println!("ℹ️ For comprehensive virtual interface management, please use:");
    println!("   🌉 Main Menu → Networking → Linux Bridge Management");
    println!("   🔧 Main Menu → Networking → libvirt/KVM Advanced Networking");
    println!();

    // Quick virtual interface overview
    println!("📊 Quick Virtual Interface Overview:");

    println!("\n🌉 Linux Bridges:");
    Command::new("bridge").args(&["link", "show"]).status().ok();

    println!("\n🔌 TAP Interfaces:");
    Command::new("ip").args(&["tuntap", "show"]).status().ok();

    println!("\n🖥️ Libvirt Networks (if available):");
    let virsh_check = Command::new("which").arg("virsh").status();
    if let Ok(s) = virsh_check {
        if s.success() {
            Command::new("virsh")
                .args(&["net-list", "--all"])
                .status()
                .ok();
        }
    } else {
        println!("  virsh not available");
    }

    println!("\n💡 For detailed interface management:");
    println!("   • Bridge creation/deletion → Linux Bridge Management");
    println!("   • VM interface attach/detach → libvirt/KVM Advanced Networking");
    println!("   • Advanced configuration → respective specialized menus");
}

fn virtualization_network_status() {
    println!("📊 Virtualization Network Status");
    println!("=================================\n");

    // Comprehensive virtualization networking status
    println!("1️⃣ Hardware & Module Status:");

    // Check virtualization support
    let virt_check = Command::new("grep")
        .args(&["-E", "(vmx|svm)", "/proc/cpuinfo"])
        .output();

    match virt_check {
        Ok(out) if !out.stdout.is_empty() => println!("  ✅ Hardware virtualization: Supported"),
        _ => println!("  ❌ Hardware virtualization: Not available"),
    }

    // Check modules
    let kvm_check = Command::new("lsmod").output();
    if let Ok(out) = kvm_check {
        let modules = String::from_utf8_lossy(&out.stdout);

        if modules.contains("kvm") {
            println!("  ✅ KVM module: Loaded");
        } else {
            println!("  ❌ KVM module: Not loaded");
        }

        if modules.contains("tun") {
            println!("  ✅ TUN/TAP module: Loaded");
        } else {
            println!("  ❌ TUN/TAP module: Not loaded");
        }

        if modules.contains("bridge") {
            println!("  ✅ Bridge module: Loaded");
        } else {
            println!("  ❌ Bridge module: Not loaded");
        }
    }

    println!("\n2️⃣ Service Status:");

    // Check libvirt
    let libvirt_status = Command::new("systemctl")
        .args(&["is-active", "libvirtd"])
        .output();

    if let Ok(out) = libvirt_status {
        let status = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if status == "active" {
            println!("  ✅ libvirtd: Running");
        } else {
            println!("  ⭕ libvirtd: Not running");
        }
    }

    // Check Docker (if relevant)
    let docker_status = Command::new("systemctl")
        .args(&["is-active", "docker"])
        .output();

    if let Ok(out) = docker_status {
        let status = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if status == "active" {
            println!("  ✅ Docker: Running");
        } else {
            println!("  ⭕ Docker: Not running");
        }
    }

    println!("\n3️⃣ Network Interfaces:");

    println!("🌉 Bridge Interfaces:");
    let bridges = Command::new("bridge").args(&["link", "show"]).output();

    if let Ok(out) = bridges {
        let output = String::from_utf8_lossy(&out.stdout);
        if output.trim().is_empty() {
            println!("  No bridge interfaces found");
        } else {
            for line in output.lines().take(5) {
                println!("  {}", line);
            }
        }
    }

    println!("\n🔌 TAP Interfaces:");
    let taps = Command::new("ip").args(&["tuntap", "show"]).output();

    if let Ok(out) = taps {
        let output = String::from_utf8_lossy(&out.stdout);
        if output.trim().is_empty() {
            println!("  No TAP interfaces found");
        } else {
            for line in output.lines().take(5) {
                println!("  {}", line);
            }
        }
    }

    println!("\n4️⃣ Libvirt Networks:");
    let virsh_check = Command::new("which").arg("virsh").status();
    if let Ok(s) = virsh_check {
        if s.success() {
            Command::new("virsh")
                .args(&["net-list", "--all"])
                .status()
                .ok();
        }
    } else {
        println!("  virsh not available");
    }

    println!("\n5️⃣ Docker Networks:");
    let docker_nets = Command::new("docker").args(&["network", "ls"]).output();

    if let Ok(out) = docker_nets {
        if out.status.success() {
            println!("  Docker networks available:");
            for line in String::from_utf8_lossy(&out.stdout).lines().take(5) {
                println!("    {}", line);
            }
        }
    } else {
        println!("  Docker not available or not running");
    }

    println!("\n6️⃣ System Configuration:");

    // Check IP forwarding
    println!("📡 IP Forwarding:");
    Command::new("cat")
        .args(&["/proc/sys/net/ipv4/ip_forward"])
        .status()
        .ok();

    // Check key paths
    println!("\n🔐 Key Configuration Paths:");
    let config_paths = [
        "/etc/qemu/bridge.conf",
        "/dev/net/tun",
        "/sys/class/net/docker0",
    ];

    for path in &config_paths {
        if Path::new(path).exists() {
            println!("  ✅ {}", path);
        } else {
            println!("  ❌ {}", path);
        }
    }
}

fn advanced_virtual_networking() {
    println!("🚀 Advanced Virtual Networking");
    println!("===============================\n");

    println!("ℹ️ Advanced virtual networking features are available in specialized modules:");
    println!();

    println!("🔧 Available Advanced Modules:");
    println!("  1. 🌉 Linux Bridge Management");
    println!("     • Bridge creation, deletion, configuration");
    println!("     • Interface management and STP configuration");
    println!("     • VLAN support and bridge diagnostics");
    println!();

    println!("  2. 🖥️ libvirt/KVM Advanced Networking");
    println!("     • VM interface management (attach/detach)");
    println!("     • Domain interface listing (domiflist)");
    println!("     • Network performance tuning");
    println!("     • Advanced virsh operations");
    println!();

    println!("  3. 🔥 Advanced Firewall & Networking");
    println!("     • nftables integration for virtualization");
    println!("     • NAT and port forwarding rules");
    println!("     • Security policies for VM traffic");
    println!();

    println!("💡 Navigation:");
    println!("   Main Menu → Networking → [Select desired module]");
    println!();

    let redirect_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Navigate to advanced module?")
        .items(&[
            "🌉 Linux Bridge Management",
            "🖥️ libvirt/KVM Advanced Networking",
            "🔥 Advanced Firewall & Networking",
            "⬅️ Stay in current menu",
        ])
        .default(3)
        .interact()
        .unwrap();

    match redirect_choice {
        0 => {
            println!("\n🔄 Redirecting to Linux Bridge Management...");
            println!("Please navigate to: Main Menu → Networking → Linux Bridge Management");
        }
        1 => {
            println!("\n🔄 Redirecting to libvirt/KVM Advanced Networking...");
            println!(
                "Please navigate to: Main Menu → Networking → libvirt/KVM Advanced Networking"
            );
        }
        2 => {
            println!("\n🔄 Redirecting to Advanced Firewall & Networking...");
            println!("Please navigate to: Main Menu → Networking → Advanced Firewall & Networking");
        }
        _ => {}
    }
}

fn migration_notice() {
    println!("ℹ️ Feature Migration Notice");
    println!("============================\n");

    println!("🔄 Advanced virtualization networking features have been moved to specialized modules for better organization and functionality:");
    println!();

    println!("🆕 New Module Locations:");
    println!();

    println!("1️⃣ 🌉 Linux Bridge Management");
    println!("   Location: Main Menu → Networking → Linux Bridge Management");
    println!("   Features:");
    println!("   • Create, configure, and delete Linux bridges");
    println!("   • Add/remove interfaces to/from bridges");
    println!("   • Configure STP, VLAN, and advanced bridge parameters");
    println!("   • Bridge diagnostics and performance monitoring");
    println!();

    println!("2️⃣ 🖥️ libvirt/KVM Advanced Networking");
    println!("   Location: Main Menu → Networking → libvirt/KVM Advanced Networking");
    println!("   Features:");
    println!("   • VM network interface management (attach/detach/modify)");
    println!("   • Domain interface listing (domiflist functionality)");
    println!("   • Network performance analysis and tuning");
    println!("   • Advanced virsh network operations");
    println!("   • Integration with bridge and TAP interfaces");
    println!();

    println!("3️⃣ 🔥 Advanced Firewall & Networking");
    println!("   Location: Main Menu → Networking → Advanced Firewall & Networking");
    println!("   Features:");
    println!("   • nftables rules for virtualization traffic");
    println!("   • NAT and port forwarding for VMs");
    println!("   • VM traffic security policies");
    println!("   • Integration with gaming and container networks");
    println!();

    println!("✨ Benefits of New Organization:");
    println!("   • 🎯 More focused and specialized functionality");
    println!("   • 🚀 Better performance and reliability");
    println!("   • 🔧 Enhanced integration with system components");
    println!("   • 📊 Improved diagnostics and monitoring");
    println!("   • 🔒 Better security and safety checks");
    println!();

    println!("📍 Current Menu Status:");
    println!("   This menu now serves as a bridge and provides basic virtualization");
    println!("   networking tools, with redirects to the specialized modules for");
    println!("   advanced functionality.");
    println!();

    let proceed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like guidance on navigating to the new modules?")
        .default(true)
        .interact()
        .unwrap();

    if proceed {
        println!("\n🧭 Navigation Guide:");
        println!("   1. Return to Main Menu (select 'Back' or press Ctrl+C)");
        println!("   2. Select 'Networking'");
        println!("   3. Choose your desired specialized module:");
        println!("      • 'Linux Bridge Management' for bridge operations");
        println!("      • 'libvirt/KVM Advanced Networking' for VM networking");
        println!("      • 'Advanced Firewall & Networking' for security rules");
        println!();
        println!("💡 Pro tip: Bookmark these locations for quick access!");
    }
}

// Missing function stubs
fn tap_interface_management() {
    println!("TAP Interface Management - Coming soon...");
}

fn bridge_performance_analysis() {
    println!("Bridge Performance Analysis - Coming soon...");
}
