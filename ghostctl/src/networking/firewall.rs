use crate::networking::safe_commands;
use crate::security::validation::{
    ValidatedCidr, ValidatedInterface, ValidatedIpAddress, ValidatedPort, ValidatedPortRange,
    ValidatedProtocol, ValidatedServiceName,
};
use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use std::process::Command;

/// Helper to apply an iptables rule safely without shell injection
fn apply_iptables_rule(args: &[&str]) {
    match Command::new("sudo").arg("iptables").args(args).status() {
        Ok(status) if !status.success() => {
            log::warn!("iptables rule failed: {:?}", args);
        }
        Err(e) => {
            log::warn!("Failed to run iptables: {}", e);
        }
        _ => {}
    }
}

/// Helper to apply a UFW rule safely
fn apply_ufw_rule(args: &[&str]) {
    match Command::new("sudo").arg("ufw").args(args).status() {
        Ok(status) if !status.success() => {
            log::warn!("ufw rule failed: {:?}", args);
        }
        Err(e) => {
            log::warn!("Failed to run ufw: {}", e);
        }
        _ => {}
    }
}

/// Helper to apply a firewall-cmd rule safely
fn apply_firewalld_rule(args: &[&str]) {
    match Command::new("sudo").arg("firewall-cmd").args(args).status() {
        Ok(status) if !status.success() => {
            log::warn!("firewall-cmd rule failed: {:?}", args);
        }
        Err(e) => {
            log::warn!("Failed to run firewall-cmd: {}", e);
        }
        _ => {}
    }
}

/// Helper to apply a sysctl setting safely
fn apply_sysctl(param: &str, value: &str) {
    let setting = format!("{}={}", param, value);
    match Command::new("sudo")
        .args(["sysctl", "-w", &setting])
        .status()
    {
        Ok(status) if !status.success() => {
            log::warn!("sysctl failed: {}", setting);
        }
        Err(e) => {
            log::warn!("Failed to run sysctl: {}", e);
        }
        _ => {}
    }
}

/// Validate a hostname or IP address is safe for use in ping/traceroute
fn is_valid_target(target: &str) -> bool {
    // Allow IP addresses
    if target.parse::<std::net::IpAddr>().is_ok() {
        return true;
    }
    // Allow valid hostnames (alphanumeric, dots, hyphens only)
    target
        .chars()
        .all(|c| c.is_alphanumeric() || c == '.' || c == '-')
        && !target.is_empty()
        && target.len() <= 253
}

pub fn firewall_menu() {
    loop {
        let options = [
            "🛡️ UFW Management",
            "🔥 Firewalld Management",
            "⚙️ iptables Management",
            "🚀 nftables Management",
            "🎮 Gaming Network Optimization",
            "🔍 Port Scanner & Checker",
            "🔧 Firewall Troubleshooting",
            "🌐 Network Latency Optimization",
            "📋 Firewall Status Overview",
            "⬅️ Back",
        ];

        let choice = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔥 Firewall Management")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            Ok(None) | Err(_) => break,
        };

        match choice {
            0 => ufw_management(),
            1 => firewalld_management(),
            2 => iptables_management(),
            3 => nftables_management(),
            4 => gaming_network_optimization(),
            5 => port_scanner(),
            6 => firewall_troubleshooting(),
            7 => network_latency_optimization(),
            8 => firewall_status_overview(),
            _ => break,
        }
    }
}

fn ufw_management() {
    let options = [
        "✅ Enable/Disable UFW",
        "➕ Add Rule",
        "🗑️ Delete Rule",
        "📋 List Rules",
        "🔄 Reset UFW",
        "🎯 Allow Application",
        "🚫 Deny Application",
        "📊 Status",
        "⬅️ Back",
    ];

    while let Ok(Some(choice)) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🛡️ UFW Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        match choice {
            0 => {
                let enable = match Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enable UFW?")
                    .default(true)
                    .interact_opt()
                {
                    Ok(Some(e)) => e,
                    Ok(None) | Err(_) => continue,
                };

                if enable {
                    println!("🔧 Enabling UFW...");
                    let status = Command::new("sudo").args(&["ufw", "enable"]).status();

                    match status {
                        Ok(s) if s.success() => println!("✅ UFW enabled"),
                        _ => println!("❌ Failed to enable UFW"),
                    }
                } else {
                    println!("🔧 Disabling UFW...");
                    let status = Command::new("sudo").args(&["ufw", "disable"]).status();

                    match status {
                        Ok(s) if s.success() => println!("✅ UFW disabled"),
                        _ => println!("❌ Failed to disable UFW"),
                    }
                }
            }
            1 => {
                let rule_type = match Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select rule type")
                    .items(&[
                        "Allow port",
                        "Deny port",
                        "Allow from IP",
                        "Deny from IP",
                        "Allow service",
                    ])
                    .default(0)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => continue,
                };

                match rule_type {
                    0 | 1 => {
                        let port_input: String = match Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter port number or range (e.g., 80, 8000:8080)")
                            .interact_text()
                        {
                            Ok(i) => i,
                            Err(_) => continue,
                        };

                        // Validate port/port range
                        let validated_port = match ValidatedPortRange::from_input(&port_input) {
                            Ok(p) => p.to_string(),
                            Err(e) => {
                                println!("❌ Invalid port: {}", e);
                                continue;
                            }
                        };

                        let protocol = match Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Select protocol")
                            .items(&["tcp", "udp", "both"])
                            .default(0)
                            .interact_opt()
                        {
                            Ok(Some(c)) => c,
                            Ok(None) | Err(_) => continue,
                        };

                        let action = if rule_type == 0 { "allow" } else { "deny" };

                        // Build port arg with protocol suffix
                        let port_arg = match protocol {
                            0 => format!("{}/tcp", validated_port),
                            1 => format!("{}/udp", validated_port),
                            _ => validated_port.clone(),
                        };

                        println!("🔧 Executing: sudo ufw {} {}", action, port_arg);

                        let status = Command::new("sudo")
                            .args(["ufw", action, &port_arg])
                            .status();

                        match status {
                            Ok(s) if s.success() => println!("✅ Rule added"),
                            _ => println!("❌ Failed to add rule"),
                        }
                    }
                    2 | 3 => {
                        let ip_input: String = match Input::with_theme(&ColorfulTheme::default())
                            .with_prompt(
                                "Enter IP address or subnet (e.g., 192.168.1.100, 192.168.1.0/24)",
                            )
                            .interact_text()
                        {
                            Ok(i) => i,
                            Err(_) => continue,
                        };

                        // Validate IP or CIDR
                        let validated_ip = if ip_input.contains('/') {
                            match ValidatedCidr::from_input(&ip_input) {
                                Ok(c) => c.value().to_string(),
                                Err(e) => {
                                    println!("❌ Invalid CIDR: {}", e);
                                    continue;
                                }
                            }
                        } else {
                            match ValidatedIpAddress::from_input(&ip_input) {
                                Ok(ip) => ip.value().to_string(),
                                Err(e) => {
                                    println!("❌ Invalid IP address: {}", e);
                                    continue;
                                }
                            }
                        };

                        let action = if rule_type == 2 { "allow" } else { "deny" };

                        let port_input: String = match Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter port (optional, press Enter to skip)")
                            .allow_empty(true)
                            .interact_text()
                        {
                            Ok(i) => i,
                            Err(_) => continue,
                        };

                        let status = if port_input.is_empty() {
                            println!("🔧 Executing: sudo ufw {} from {}", action, validated_ip);
                            Command::new("sudo")
                                .args(["ufw", action, "from", &validated_ip])
                                .status()
                        } else {
                            // Validate port
                            let validated_port = match ValidatedPort::from_input(&port_input) {
                                Ok(p) => p.to_string(),
                                Err(e) => {
                                    println!("❌ Invalid port: {}", e);
                                    continue;
                                }
                            };
                            println!(
                                "🔧 Executing: sudo ufw {} from {} to any port {}",
                                action, validated_ip, validated_port
                            );
                            Command::new("sudo")
                                .args([
                                    "ufw",
                                    action,
                                    "from",
                                    &validated_ip,
                                    "to",
                                    "any",
                                    "port",
                                    &validated_port,
                                ])
                                .status()
                        };

                        match status {
                            Ok(s) if s.success() => println!("✅ Rule added"),
                            _ => println!("❌ Failed to add rule"),
                        }
                    }
                    4 => {
                        let service_input: String =
                            match Input::with_theme(&ColorfulTheme::default())
                                .with_prompt("Enter service name (e.g., ssh, http, https)")
                                .interact_text()
                            {
                                Ok(i) => i,
                                Err(_) => continue,
                            };

                        // Validate service name
                        let validated_service =
                            match ValidatedServiceName::from_input(&service_input) {
                                Ok(s) => s.value().to_string(),
                                Err(e) => {
                                    println!("❌ Invalid service name: {}", e);
                                    continue;
                                }
                            };

                        println!("🔧 Executing: sudo ufw allow {}", validated_service);

                        let status = Command::new("sudo")
                            .args(["ufw", "allow", &validated_service])
                            .status();

                        match status {
                            Ok(s) if s.success() => println!("✅ Service allowed"),
                            _ => println!("❌ Failed to allow service"),
                        }
                    }
                    _ => {}
                }
            }
            2 => {
                println!("📋 Current UFW rules:");
                Command::new("sudo")
                    .args(&["ufw", "status", "numbered"])
                    .status()
                    .ok();

                let rule_num: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter rule number to delete (or 'cancel')")
                    .interact_text()
                {
                    Ok(i) => i,
                    Err(_) => continue,
                };

                if rule_num != "cancel" {
                    // Validate rule number (must be numeric)
                    if !rule_num.chars().all(|c| c.is_ascii_digit()) {
                        println!("❌ Invalid rule number: must be numeric");
                        continue;
                    }

                    let status = Command::new("sudo")
                        .args(["ufw", "delete", &rule_num])
                        .status();

                    match status {
                        Ok(s) if s.success() => println!("✅ Rule deleted"),
                        _ => println!("❌ Failed to delete rule"),
                    }
                }
            }
            3 => {
                println!("📋 UFW Rules:");
                Command::new("sudo")
                    .args(&["ufw", "status", "verbose"])
                    .status()
                    .ok();
            }
            4 => {
                let confirm = match Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("⚠️ This will reset all UFW rules. Continue?")
                    .default(false)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => continue,
                };

                if confirm {
                    Command::new("sudo")
                        .args(&["ufw", "--force", "reset"])
                        .status()
                        .ok();
                    println!("✅ UFW reset completed");
                }
            }
            5 => {
                println!("📋 Available applications:");
                Command::new("sudo")
                    .args(&["ufw", "app", "list"])
                    .status()
                    .ok();

                let app: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter application name to allow")
                    .interact_text()
                {
                    Ok(i) => i,
                    Err(_) => continue,
                };

                // Validate application name (alphanumeric with spaces allowed for app names)
                if !app
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '_')
                {
                    println!("❌ Invalid application name");
                    continue;
                }

                Command::new("sudo")
                    .args(["ufw", "allow", &app])
                    .status()
                    .ok();
            }
            6 => {
                let app: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter application name to deny")
                    .interact_text()
                {
                    Ok(i) => i,
                    Err(_) => continue,
                };

                // Validate application name
                if !app
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '_')
                {
                    println!("❌ Invalid application name");
                    continue;
                }

                Command::new("sudo")
                    .args(["ufw", "deny", &app])
                    .status()
                    .ok();
            }
            7 => {
                println!("📊 UFW Status:");
                Command::new("sudo")
                    .args(&["ufw", "status", "verbose"])
                    .status()
                    .ok();
            }
            _ => break,
        }
    }
}

fn firewalld_management() {
    let options = [
        "✅ Start/Stop Firewalld",
        "🔄 Reload Configuration",
        "📋 List Zones",
        "➕ Add Port/Service",
        "🗑️ Remove Port/Service",
        "🌐 Zone Management",
        "🛡️ Rich Rules",
        "📊 Status",
        "⬅️ Back",
    ];

    while let Ok(Some(choice)) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🔥 Firewalld Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        match choice {
            0 => {
                let action = match Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select action")
                    .items(&["Start", "Stop", "Restart", "Enable", "Disable"])
                    .default(0)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => continue,
                };

                let systemctl_action = match action {
                    0 => "start",
                    1 => "stop",
                    2 => "restart",
                    3 => "enable",
                    4 => "disable",
                    _ => "",
                };

                if !systemctl_action.is_empty() {
                    let status = Command::new("sudo")
                        .args(["systemctl", systemctl_action, "firewalld"])
                        .status();

                    match status {
                        Ok(s) if s.success() => println!("✅ Action completed"),
                        _ => println!("❌ Action failed"),
                    }
                }
            }
            1 => {
                println!("🔄 Reloading firewalld configuration...");
                Command::new("sudo")
                    .args(&["firewall-cmd", "--reload"])
                    .status()
                    .ok();
                println!("✅ Configuration reloaded");
            }
            2 => {
                println!("📋 Firewalld Zones:");
                Command::new("sudo")
                    .args(&["firewall-cmd", "--list-all-zones"])
                    .status()
                    .ok();
            }
            3 => {
                let add_type = match Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("What to add?")
                    .items(&["Port", "Service", "Source IP"])
                    .default(0)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => continue,
                };

                let permanent = match Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Make permanent?")
                    .default(true)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => continue,
                };

                let perm_flag = if permanent { "--permanent" } else { "" };

                match add_type {
                    0 => {
                        use crate::security::validation::ValidatedZone;

                        let port_input: String = match Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter port/protocol (e.g., 8080/tcp, 53/udp)")
                            .interact_text()
                        {
                            Ok(i) => i,
                            Err(_) => continue,
                        };

                        // Parse and validate port/protocol format
                        let parts: Vec<&str> = port_input.split('/').collect();
                        if parts.len() != 2 {
                            println!("❌ Invalid format. Use: port/protocol (e.g., 8080/tcp)");
                            continue;
                        }

                        let validated_port = match ValidatedPortRange::from_input(parts[0]) {
                            Ok(p) => p.to_string(),
                            Err(e) => {
                                println!("❌ Invalid port: {}", e);
                                continue;
                            }
                        };

                        let validated_protocol = match ValidatedProtocol::from_input(parts[1]) {
                            Ok(p) => p.as_str().to_string(),
                            Err(e) => {
                                println!("❌ Invalid protocol: {}", e);
                                continue;
                            }
                        };

                        let zone_input: String = match Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter zone (or press Enter for default)")
                            .allow_empty(true)
                            .interact_text()
                        {
                            Ok(i) => i,
                            Err(_) => continue,
                        };

                        let port_arg =
                            format!("--add-port={}/{}", validated_port, validated_protocol);

                        let mut args: Vec<&str> = vec!["firewall-cmd"];
                        if permanent {
                            args.push("--permanent");
                        }

                        let zone_arg;
                        if !zone_input.is_empty() {
                            let validated_zone = match ValidatedZone::from_input(&zone_input) {
                                Ok(z) => z,
                                Err(e) => {
                                    println!("❌ Invalid zone: {}", e);
                                    continue;
                                }
                            };
                            zone_arg = format!("--zone={}", validated_zone);
                            args.push(&zone_arg);
                        }
                        args.push(&port_arg);

                        println!("🔧 Executing: sudo {}", args.join(" "));
                        Command::new("sudo").args(&args).status().ok();
                    }
                    1 => {
                        let service_input: String =
                            match Input::with_theme(&ColorfulTheme::default())
                                .with_prompt("Enter service name (e.g., http, https, ssh)")
                                .interact_text()
                            {
                                Ok(i) => i,
                                Err(_) => continue,
                            };

                        let validated_service =
                            match ValidatedServiceName::from_input(&service_input) {
                                Ok(s) => s.value().to_string(),
                                Err(e) => {
                                    println!("❌ Invalid service name: {}", e);
                                    continue;
                                }
                            };

                        let service_arg = format!("--add-service={}", validated_service);

                        let mut args: Vec<&str> = vec!["firewall-cmd"];
                        if permanent {
                            args.push("--permanent");
                        }
                        args.push(&service_arg);

                        Command::new("sudo").args(&args).status().ok();
                    }
                    2 => {
                        let source_input: String =
                            match Input::with_theme(&ColorfulTheme::default())
                                .with_prompt("Enter source IP or subnet")
                                .interact_text()
                            {
                                Ok(i) => i,
                                Err(_) => continue,
                            };

                        // Validate IP or CIDR
                        let validated_source = if source_input.contains('/') {
                            match ValidatedCidr::from_input(&source_input) {
                                Ok(c) => c.value().to_string(),
                                Err(e) => {
                                    println!("❌ Invalid CIDR: {}", e);
                                    continue;
                                }
                            }
                        } else {
                            match ValidatedIpAddress::from_input(&source_input) {
                                Ok(ip) => ip.value().to_string(),
                                Err(e) => {
                                    println!("❌ Invalid IP address: {}", e);
                                    continue;
                                }
                            }
                        };

                        let source_arg = format!("--add-source={}", validated_source);

                        let mut args: Vec<&str> = vec!["firewall-cmd"];
                        if permanent {
                            args.push("--permanent");
                        }
                        args.push(&source_arg);

                        Command::new("sudo").args(&args).status().ok();
                    }
                    _ => {}
                }

                if permanent {
                    println!("🔄 Reloading to apply permanent changes...");
                    Command::new("sudo")
                        .args(&["firewall-cmd", "--reload"])
                        .status()
                        .ok();
                }
            }
            4 => {
                let remove_type = match Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("What to remove?")
                    .items(&["Port", "Service", "Source IP"])
                    .default(0)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => continue,
                };

                let permanent = match Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Remove permanently?")
                    .default(true)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => continue,
                };

                match remove_type {
                    0 => {
                        println!("📋 Current ports:");
                        Command::new("sudo")
                            .args(&["firewall-cmd", "--list-ports"])
                            .status()
                            .ok();

                        let port_input: String = match Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter port/protocol to remove")
                            .interact_text()
                        {
                            Ok(i) => i,
                            Err(_) => continue,
                        };

                        // Parse and validate port/protocol format
                        let parts: Vec<&str> = port_input.split('/').collect();
                        if parts.len() != 2 {
                            println!("❌ Invalid format. Use: port/protocol (e.g., 8080/tcp)");
                            continue;
                        }

                        let validated_port = match ValidatedPortRange::from_input(parts[0]) {
                            Ok(p) => p.to_string(),
                            Err(e) => {
                                println!("❌ Invalid port: {}", e);
                                continue;
                            }
                        };

                        let validated_protocol = match ValidatedProtocol::from_input(parts[1]) {
                            Ok(p) => p.as_str().to_string(),
                            Err(e) => {
                                println!("❌ Invalid protocol: {}", e);
                                continue;
                            }
                        };

                        let port_arg =
                            format!("--remove-port={}/{}", validated_port, validated_protocol);

                        let mut args: Vec<&str> = vec!["firewall-cmd"];
                        if permanent {
                            args.push("--permanent");
                        }
                        args.push(&port_arg);

                        Command::new("sudo").args(&args).status().ok();
                    }
                    1 => {
                        println!("📋 Current services:");
                        Command::new("sudo")
                            .args(&["firewall-cmd", "--list-services"])
                            .status()
                            .ok();

                        let service_input: String =
                            match Input::with_theme(&ColorfulTheme::default())
                                .with_prompt("Enter service to remove")
                                .interact_text()
                            {
                                Ok(i) => i,
                                Err(_) => continue,
                            };

                        let validated_service =
                            match ValidatedServiceName::from_input(&service_input) {
                                Ok(s) => s.value().to_string(),
                                Err(e) => {
                                    println!("❌ Invalid service name: {}", e);
                                    continue;
                                }
                            };

                        let service_arg = format!("--remove-service={}", validated_service);

                        let mut args: Vec<&str> = vec!["firewall-cmd"];
                        if permanent {
                            args.push("--permanent");
                        }
                        args.push(&service_arg);

                        Command::new("sudo").args(&args).status().ok();
                    }
                    2 => {
                        println!("📋 Current sources:");
                        Command::new("sudo")
                            .args(&["firewall-cmd", "--list-sources"])
                            .status()
                            .ok();

                        let source_input: String =
                            match Input::with_theme(&ColorfulTheme::default())
                                .with_prompt("Enter source to remove")
                                .interact_text()
                            {
                                Ok(i) => i,
                                Err(_) => continue,
                            };

                        // Validate IP or CIDR
                        let validated_source = if source_input.contains('/') {
                            match ValidatedCidr::from_input(&source_input) {
                                Ok(c) => c.value().to_string(),
                                Err(e) => {
                                    println!("❌ Invalid CIDR: {}", e);
                                    continue;
                                }
                            }
                        } else {
                            match ValidatedIpAddress::from_input(&source_input) {
                                Ok(ip) => ip.value().to_string(),
                                Err(e) => {
                                    println!("❌ Invalid IP address: {}", e);
                                    continue;
                                }
                            }
                        };

                        let source_arg = format!("--remove-source={}", validated_source);

                        let mut args: Vec<&str> = vec!["firewall-cmd"];
                        if permanent {
                            args.push("--permanent");
                        }
                        args.push(&source_arg);

                        Command::new("sudo").args(&args).status().ok();
                    }
                    _ => {}
                }

                if permanent {
                    Command::new("sudo")
                        .args(&["firewall-cmd", "--reload"])
                        .status()
                        .ok();
                }
            }
            5 => {
                zone_management();
            }
            6 => {
                rich_rules_management();
            }
            7 => {
                println!("📊 Firewalld Status:");
                Command::new("sudo")
                    .args(&["firewall-cmd", "--state"])
                    .status()
                    .ok();

                println!("\n🌐 Default Zone:");
                Command::new("sudo")
                    .args(&["firewall-cmd", "--get-default-zone"])
                    .status()
                    .ok();

                println!("\n📋 Active Zones:");
                Command::new("sudo")
                    .args(&["firewall-cmd", "--get-active-zones"])
                    .status()
                    .ok();

                println!("\n🔧 Current Configuration:");
                Command::new("sudo")
                    .args(&["firewall-cmd", "--list-all"])
                    .status()
                    .ok();
            }
            _ => break,
        }
    }
}

fn zone_management() {
    println!("🌐 Zone Management");

    let options = [
        "📋 List zones",
        "🔄 Change default zone",
        "➕ Add interface to zone",
        "🗑️ Remove interface from zone",
        "📝 Create custom zone",
        "Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Zone Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            Command::new("sudo")
                .args(&["firewall-cmd", "--get-zones"])
                .status()
                .ok();
        }
        1 => {
            let zone: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter new default zone")
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            Command::new("sudo")
                .args(&["firewall-cmd", "--set-default-zone", &zone])
                .status()
                .ok();
        }
        2 => {
            use crate::security::validation::ValidatedZone;

            let Ok(interface_input) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter interface name")
                .interact()
            else {
                return;
            };

            let validated_interface = match ValidatedInterface::from_input(&interface_input) {
                Ok(i) => i,
                Err(e) => {
                    println!("❌ Invalid interface: {}", e);
                    return;
                }
            };

            let Ok(zone_input) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter zone name")
                .interact()
            else {
                return;
            };

            let validated_zone = match ValidatedZone::from_input(&zone_input) {
                Ok(z) => z,
                Err(e) => {
                    println!("❌ Invalid zone: {}", e);
                    return;
                }
            };

            let zone_arg = format!("--zone={}", validated_zone);
            let interface_arg = format!("--add-interface={}", validated_interface);

            Command::new("sudo")
                .args(&["firewall-cmd", &zone_arg, &interface_arg, "--permanent"])
                .status()
                .ok();

            Command::new("sudo")
                .args(&["firewall-cmd", "--reload"])
                .status()
                .ok();
        }
        3 => {
            use crate::security::validation::ValidatedZone;

            let Ok(interface_input) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter interface name")
                .interact()
            else {
                return;
            };

            let validated_interface = match ValidatedInterface::from_input(&interface_input) {
                Ok(i) => i,
                Err(e) => {
                    println!("❌ Invalid interface: {}", e);
                    return;
                }
            };

            let Ok(zone_input) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter zone name")
                .interact()
            else {
                return;
            };

            let validated_zone = match ValidatedZone::from_input(&zone_input) {
                Ok(z) => z,
                Err(e) => {
                    println!("❌ Invalid zone: {}", e);
                    return;
                }
            };

            let zone_arg = format!("--zone={}", validated_zone);
            let interface_arg = format!("--remove-interface={}", validated_interface);

            Command::new("sudo")
                .args(&["firewall-cmd", &zone_arg, &interface_arg, "--permanent"])
                .status()
                .ok();

            Command::new("sudo")
                .args(&["firewall-cmd", "--reload"])
                .status()
                .ok();
        }
        4 => {
            use crate::security::validation::ValidatedZone;

            let Ok(zone_input) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter new zone name")
                .interact()
            else {
                return;
            };

            let validated_zone = match ValidatedZone::from_input(&zone_input) {
                Ok(z) => z,
                Err(e) => {
                    println!("❌ Invalid zone: {}", e);
                    return;
                }
            };

            let zone_arg = format!("--new-zone={}", validated_zone);

            Command::new("sudo")
                .args(&["firewall-cmd", "--permanent", &zone_arg])
                .status()
                .ok();

            Command::new("sudo")
                .args(&["firewall-cmd", "--reload"])
                .status()
                .ok();

            println!("✅ Zone '{}' created", validated_zone);
        }
        _ => {}
    }
}

fn rich_rules_management() {
    println!("🛡️ Rich Rules Management");

    let options = [
        "📋 List rich rules",
        "➕ Add rich rule",
        "🗑️ Remove rich rule",
        "📝 Add rate limiting rule",
        "🚫 Add block rule",
        "Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Rich Rules")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            Command::new("sudo")
                .args(&["firewall-cmd", "--list-rich-rules"])
                .status()
                .ok();
        }
        1 => {
            println!("📝 Rich Rule Builder");

            let rule_type = match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Rule type")
                .items(&["Accept", "Reject", "Drop"])
                .default(0)
                .interact_opt()
            {
                Ok(Some(c)) => c,
                Ok(None) | Err(_) => return,
            };

            let action = match rule_type {
                0 => "accept",
                1 => "reject",
                2 => "drop",
                _ => "accept",
            };

            let source_input: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Source address (or press Enter to skip)")
                .allow_empty(true)
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            let port_input: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Port number (or press Enter to skip)")
                .allow_empty(true)
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            let protocol = if !port_input.is_empty() {
                match Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Protocol")
                    .items(&["tcp", "udp"])
                    .default(0)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => return,
                }
            } else {
                0
            };

            let proto = if protocol == 0 { "tcp" } else { "udp" };

            let mut rule = String::from("rule ");

            if !source_input.is_empty() {
                // Validate source IP
                let validated_source = if source_input.contains('/') {
                    match ValidatedCidr::from_input(&source_input) {
                        Ok(c) => c.value().to_string(),
                        Err(e) => {
                            println!("❌ Invalid CIDR: {}", e);
                            return;
                        }
                    }
                } else {
                    match ValidatedIpAddress::from_input(&source_input) {
                        Ok(ip) => ip.value().to_string(),
                        Err(e) => {
                            println!("❌ Invalid IP address: {}", e);
                            return;
                        }
                    }
                };
                rule.push_str(&format!(
                    "family=\"ipv4\" source address=\"{}\" ",
                    validated_source
                ));
            }

            if !port_input.is_empty() {
                // Validate port
                let validated_port = match ValidatedPort::from_input(&port_input) {
                    Ok(p) => p.to_string(),
                    Err(e) => {
                        println!("❌ Invalid port: {}", e);
                        return;
                    }
                };
                rule.push_str(&format!(
                    "port port=\"{}\" protocol=\"{}\" ",
                    validated_port, proto
                ));
            }

            rule.push_str(action);

            let rule_arg = format!("--add-rich-rule={}", rule);
            println!("🔧 Executing: sudo firewall-cmd {} --permanent", rule_arg);

            Command::new("sudo")
                .args(["firewall-cmd", &rule_arg, "--permanent"])
                .status()
                .ok();

            Command::new("sudo")
                .args(&["firewall-cmd", "--reload"])
                .status()
                .ok();
        }
        2 => {
            println!("📋 Current rich rules:");
            Command::new("sudo")
                .args(&["firewall-cmd", "--list-rich-rules"])
                .status()
                .ok();

            let rule_input: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter rule to remove (copy exactly)")
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            // Rich rules have a specific format; validate no shell metacharacters
            if rule_input.contains('\'') || rule_input.contains('`') || rule_input.contains('$') {
                println!("❌ Invalid characters in rule");
                return;
            }

            let rule_arg = format!("--remove-rich-rule={}", rule_input);
            Command::new("sudo")
                .args(["firewall-cmd", &rule_arg, "--permanent"])
                .status()
                .ok();

            Command::new("sudo")
                .args(&["firewall-cmd", "--reload"])
                .status()
                .ok();
        }
        3 => {
            let service_input: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Service to rate limit (e.g., ssh)")
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            // Validate service name
            let validated_service = match ValidatedServiceName::from_input(&service_input) {
                Ok(s) => s.value().to_string(),
                Err(e) => {
                    println!("❌ Invalid service name: {}", e);
                    return;
                }
            };

            let rate_input: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Rate (e.g., 3/m for 3 per minute)")
                .default("3/m".to_string())
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            // Validate rate format (number/unit where unit is s, m, h, d)
            static RATE_REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
                regex::Regex::new(r"^\d+/[smhd]$").expect("valid regex")
            });
            if !RATE_REGEX.is_match(&rate_input) {
                println!("❌ Invalid rate format. Use: number/unit (e.g., 3/m, 10/s)");
                return;
            }

            let rule = format!(
                "rule service name=\"{}\" limit value=\"{}\" accept",
                validated_service, rate_input
            );

            let rule_arg = format!("--add-rich-rule={}", rule);
            Command::new("sudo")
                .args(["firewall-cmd", &rule_arg, "--permanent"])
                .status()
                .ok();

            Command::new("sudo")
                .args(&["firewall-cmd", "--reload"])
                .status()
                .ok();

            println!("✅ Rate limiting rule added for {}", validated_service);
        }
        4 => {
            let source_input: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("IP address to block")
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            // Validate IP address
            let validated_source = match ValidatedIpAddress::from_input(&source_input) {
                Ok(ip) => ip.value().to_string(),
                Err(e) => {
                    println!("❌ Invalid IP address: {}", e);
                    return;
                }
            };

            let rule = format!(
                "rule family=\"ipv4\" source address=\"{}\" drop",
                validated_source
            );

            let rule_arg = format!("--add-rich-rule={}", rule);
            Command::new("sudo")
                .args(["firewall-cmd", &rule_arg, "--permanent"])
                .status()
                .ok();

            Command::new("sudo")
                .args(&["firewall-cmd", "--reload"])
                .status()
                .ok();

            println!("✅ Blocked {}", validated_source);
        }
        _ => {}
    }
}

fn iptables_management() {
    let options = [
        "📋 List Rules",
        "➕ Add Rule",
        "🗑️ Delete Rule",
        "💾 Save Rules",
        "📥 Restore Rules",
        "🔄 Flush Rules",
        "🔗 Chain Management",
        "📊 Statistics",
        "⬅️ Back",
    ];

    while let Ok(Some(choice)) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("⚙️ iptables Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        match choice {
            0 => {
                let table = match Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select table")
                    .items(&["filter (default)", "nat", "mangle", "raw"])
                    .default(0)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => continue,
                };

                let mut args = vec!["iptables"];
                match table {
                    1 => args.extend(["-t", "nat"]),
                    2 => args.extend(["-t", "mangle"]),
                    3 => args.extend(["-t", "raw"]),
                    _ => {}
                }
                args.extend(["-L", "-n", "-v", "--line-numbers"]);
                Command::new("sudo").args(&args).status().ok();
            }
            1 => {
                add_iptables_rule();
            }
            2 => {
                delete_iptables_rule();
            }
            3 => {
                println!("💾 Saving iptables rules...");

                let distro = match Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select your distribution")
                    .items(&["Debian/Ubuntu", "RedHat/Fedora", "Arch", "Other"])
                    .default(0)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => continue,
                };

                let save_path = match distro {
                    0 => "/etc/iptables/rules.v4".to_string(),
                    1 => {
                        // RedHat/Fedora uses service command
                        Command::new("sudo")
                            .args(["service", "iptables", "save"])
                            .status()
                            .ok();
                        println!("Rules saved");
                        continue;
                    }
                    2 => "/etc/iptables/iptables.rules".to_string(),
                    _ => {
                        let path: String = match Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter save path")
                            .default("/etc/iptables.rules".to_string())
                            .interact_text()
                        {
                            Ok(i) => i,
                            Err(_) => continue,
                        };
                        // Validate path
                        if path.contains(';')
                            || path.contains('|')
                            || path.contains('&')
                            || path.contains('`')
                            || path.contains('$')
                        {
                            eprintln!("Invalid path: contains shell metacharacters");
                            continue;
                        }
                        path
                    }
                };

                // Use iptables-save and write to file via tee
                if let Ok(output) = Command::new("sudo").args(["iptables-save"]).output()
                    && output.status.success()
                {
                    use std::io::Write;
                    let result = std::process::Command::new("sudo")
                        .args(["tee", &save_path])
                        .stdin(std::process::Stdio::piped())
                        .stdout(std::process::Stdio::null())
                        .spawn()
                        .and_then(|mut child| {
                            if let Some(stdin) = child.stdin.as_mut() {
                                stdin.write_all(&output.stdout)?;
                            }
                            child.wait()
                        });
                    match result {
                        Ok(_) => println!("Rules saved to {}", save_path),
                        Err(e) => eprintln!("Failed to save: {}", e),
                    }
                }
            }
            4 => {
                println!("Restoring iptables rules...");

                let path: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter rules file path")
                    .default("/etc/iptables/rules.v4".to_string())
                    .interact_text()
                {
                    Ok(i) => i,
                    Err(_) => continue,
                };

                // Validate path
                if path.contains(';')
                    || path.contains('|')
                    || path.contains('&')
                    || path.contains('`')
                    || path.contains('$')
                {
                    eprintln!("Invalid path: contains shell metacharacters");
                    continue;
                }

                // Read rules file and pipe to iptables-restore
                match std::fs::read(&path) {
                    Ok(rules_content) => {
                        use std::io::Write;
                        let result = std::process::Command::new("sudo")
                            .args(["iptables-restore"])
                            .stdin(std::process::Stdio::piped())
                            .spawn()
                            .and_then(|mut child| {
                                if let Some(stdin) = child.stdin.as_mut() {
                                    stdin.write_all(&rules_content)?;
                                }
                                child.wait()
                            });
                        match result {
                            Ok(s) if s.success() => println!("Rules restored"),
                            Ok(_) => eprintln!("iptables-restore failed"),
                            Err(e) => eprintln!("Failed to restore rules: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Failed to read rules file: {}", e),
                }
            }
            5 => {
                let confirm = match Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("⚠️ This will remove ALL iptables rules. Continue?")
                    .default(false)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => continue,
                };

                if confirm {
                    Command::new("sudo").args(&["iptables", "-F"]).status().ok();
                    Command::new("sudo").args(&["iptables", "-X"]).status().ok();
                    Command::new("sudo").args(&["iptables", "-Z"]).status().ok();
                    println!("✅ All rules flushed");
                }
            }
            6 => {
                chain_management();
            }
            7 => {
                println!("📊 iptables Statistics:");
                Command::new("sudo")
                    .args(&["iptables", "-L", "-n", "-v", "-x"])
                    .status()
                    .ok();

                println!("\n📈 Packet counts by chain:");
                Command::new("sudo")
                    .args(&["iptables", "-nvL", "INPUT"])
                    .status()
                    .ok();
                Command::new("sudo")
                    .args(&["iptables", "-nvL", "OUTPUT"])
                    .status()
                    .ok();
                Command::new("sudo")
                    .args(&["iptables", "-nvL", "FORWARD"])
                    .status()
                    .ok();
            }
            _ => break,
        }
    }
}

fn add_iptables_rule() {
    println!("Add iptables Rule");

    let chain = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select chain")
        .items(&["INPUT", "OUTPUT", "FORWARD", "Custom"])
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    let chain_name = if chain == 3 {
        let name: String = match Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter custom chain name")
            .interact_text()
        {
            Ok(i) => i,
            Err(_) => return,
        };
        // Validate chain name
        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            eprintln!("Invalid chain name: must be alphanumeric with underscores/dashes");
            return;
        }
        name
    } else {
        ["INPUT", "OUTPUT", "FORWARD"][chain].to_string()
    };

    let action = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action")
        .items(&["ACCEPT", "DROP", "REJECT", "LOG"])
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    let action_str = ["ACCEPT", "DROP", "REJECT", "LOG"][action];

    // Build args vector safely
    let mut args: Vec<String> = vec!["iptables".to_string(), "-A".to_string(), chain_name.clone()];

    // Protocol
    let use_protocol = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Specify protocol?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    let mut selected_protocol: Option<usize> = None;
    if use_protocol {
        let protocol = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select protocol")
            .items(&["tcp", "udp", "icmp", "all"])
            .default(0)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            Ok(None) | Err(_) => return,
        };
        selected_protocol = Some(protocol);

        let proto = ["tcp", "udp", "icmp", "all"][protocol];
        args.push("-p".to_string());
        args.push(proto.to_string());

        // Port
        if protocol < 2 {
            let port: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter port (or press Enter to skip)")
                .allow_empty(true)
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            if !port.is_empty() {
                match ValidatedPort::from_input(&port) {
                    Ok(validated_port) => {
                        args.push("--dport".to_string());
                        args.push(validated_port.to_string().to_string());
                    }
                    Err(e) => {
                        eprintln!("Invalid port: {}", e);
                        return;
                    }
                }
            }
        }
    }

    // Source
    let source: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter source IP/CIDR (or press Enter to skip)")
        .allow_empty(true)
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    if !source.is_empty() {
        if let Ok(validated_ip) = ValidatedIpAddress::from_input(&source) {
            args.push("-s".to_string());
            args.push(validated_ip.value().to_string());
        } else if let Ok(validated_cidr) = ValidatedCidr::from_input(&source) {
            args.push("-s".to_string());
            args.push(validated_cidr.value().to_string());
        } else {
            eprintln!("Invalid source IP/CIDR: {}", source);
            return;
        }
    }

    // Destination
    let dest: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter destination IP/CIDR (or press Enter to skip)")
        .allow_empty(true)
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    if !dest.is_empty() {
        if let Ok(validated_ip) = ValidatedIpAddress::from_input(&dest) {
            args.push("-d".to_string());
            args.push(validated_ip.value().to_string());
        } else if let Ok(validated_cidr) = ValidatedCidr::from_input(&dest) {
            args.push("-d".to_string());
            args.push(validated_cidr.value().to_string());
        } else {
            eprintln!("Invalid destination IP/CIDR: {}", dest);
            return;
        }
    }

    // Interface
    if chain_name == "INPUT" {
        let interface: String = match Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter input interface (or press Enter to skip)")
            .allow_empty(true)
            .interact_text()
        {
            Ok(i) => i,
            Err(_) => return,
        };

        if !interface.is_empty() {
            match ValidatedInterface::from_input(&interface) {
                Ok(validated_iface) => {
                    args.push("-i".to_string());
                    args.push(validated_iface.value().to_string());
                }
                Err(e) => {
                    eprintln!("Invalid interface: {}", e);
                    return;
                }
            }
        }
    } else if chain_name == "OUTPUT" {
        let interface: String = match Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter output interface (or press Enter to skip)")
            .allow_empty(true)
            .interact_text()
        {
            Ok(i) => i,
            Err(_) => return,
        };

        if !interface.is_empty() {
            match ValidatedInterface::from_input(&interface) {
                Ok(validated_iface) => {
                    args.push("-o".to_string());
                    args.push(validated_iface.value().to_string());
                }
                Err(e) => {
                    eprintln!("Invalid interface: {}", e);
                    return;
                }
            }
        }
    }

    // State
    let use_state = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Use connection state?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if use_state {
        let states = vec!["NEW", "ESTABLISHED", "RELATED", "INVALID"];
        let selected = match MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select states")
            .items(&states)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => return,
        };

        if !selected.is_empty() {
            let state_list: Vec<&str> = selected.iter().map(|&i| states[i]).collect();
            args.push("-m".to_string());
            args.push("state".to_string());
            args.push("--state".to_string());
            args.push(state_list.join(","));
        }
    }

    args.push("-j".to_string());
    args.push(action_str.to_string());

    // Suppress warning for unused variable
    let _ = selected_protocol;

    println!("Executing: sudo {}", args.join(" "));
    let status = Command::new("sudo").args(&args).status();

    match status {
        Ok(s) if s.success() => println!("Rule added"),
        _ => println!("Failed to add rule"),
    }
}

fn delete_iptables_rule() {
    println!("🗑️ Delete iptables Rule");

    let chain = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select chain")
        .items(&["INPUT", "OUTPUT", "FORWARD", "Custom"])
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    let chain_name = if chain == 3 {
        match Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter custom chain name")
            .interact_text()
        {
            Ok(i) => i,
            Err(_) => return,
        }
    } else {
        ["INPUT", "OUTPUT", "FORWARD"][chain].to_string()
    };

    // Validate chain name (alphanumeric, underscore, hyphen only)
    if !chain_name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        eprintln!("Invalid chain name");
        return;
    }

    // List rules with line numbers
    Command::new("sudo")
        .args(["iptables", "-L", &chain_name, "--line-numbers", "-n"])
        .status()
        .ok();

    let rule_num: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter rule number to delete")
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    // Validate rule number (must be numeric)
    if !rule_num.chars().all(|c| c.is_ascii_digit()) {
        eprintln!("Invalid rule number: must be numeric");
        return;
    }

    let status = Command::new("sudo")
        .args(["iptables", "-D", &chain_name, &rule_num])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Rule deleted"),
        _ => println!("❌ Failed to delete rule"),
    }
}

fn chain_management() {
    println!("🔗 Chain Management");

    let options = [
        "📋 List chains",
        "➕ Create custom chain",
        "🗑️ Delete custom chain",
        "🔄 Set default policy",
        "Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Chain Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            Command::new("sudo")
                .args(&["iptables", "-L", "-n"])
                .status()
                .ok();
        }
        1 => {
            let chain_name: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter new chain name")
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            // Validate chain name
            if !chain_name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
            {
                eprintln!("Invalid chain name: must be alphanumeric with underscores/dashes");
                return;
            }

            let status = Command::new("sudo")
                .args(["iptables", "-N", &chain_name])
                .status();

            match status {
                Ok(s) if s.success() => println!("Chain '{}' created", chain_name),
                _ => println!("Failed to create chain"),
            }
        }
        2 => {
            let chain_name: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter chain name to delete")
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            // Validate chain name
            if !chain_name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
            {
                eprintln!("Invalid chain name: must be alphanumeric with underscores/dashes");
                return;
            }

            let status = Command::new("sudo")
                .args(["iptables", "-X", &chain_name])
                .status();

            match status {
                Ok(s) if s.success() => println!("Chain '{}' deleted", chain_name),
                _ => println!("Failed to delete chain"),
            }
        }
        3 => {
            let chain = match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select chain")
                .items(&["INPUT", "OUTPUT", "FORWARD"])
                .default(0)
                .interact_opt()
            {
                Ok(Some(c)) => c,
                Ok(None) | Err(_) => return,
            };

            let chain_name = ["INPUT", "OUTPUT", "FORWARD"][chain];

            let policy = match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select default policy")
                .items(&["ACCEPT", "DROP"])
                .default(0)
                .interact_opt()
            {
                Ok(Some(c)) => c,
                Ok(None) | Err(_) => return,
            };

            let policy_str = ["ACCEPT", "DROP"][policy];

            let status = Command::new("sudo")
                .args(["iptables", "-P", chain_name, policy_str])
                .status();

            match status {
                Ok(s) if s.success() => println!("Default policy set"),
                _ => println!("Failed to set policy"),
            }
        }
        _ => {}
    }
}

fn nftables_management() {
    let options = [
        "📋 List Rules",
        "➕ Add Table",
        "➕ Add Chain",
        "➕ Add Rule",
        "🗑️ Delete Rule",
        "💾 Save Configuration",
        "📥 Load Configuration",
        "📊 Status",
        "⬅️ Back",
    ];

    while let Ok(Some(choice)) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🚀 nftables Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        match choice {
            0 => {
                println!("📋 nftables Rules:");
                Command::new("sudo")
                    .args(&["nft", "list", "ruleset"])
                    .status()
                    .ok();
            }
            1 => {
                let family = match Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select address family")
                    .items(&["ip", "ip6", "inet", "bridge", "netdev"])
                    .default(2)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => continue,
                };

                let family_str = ["ip", "ip6", "inet", "bridge", "netdev"][family];

                let table_name: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter table name")
                    .interact_text()
                {
                    Ok(i) => i,
                    Err(_) => continue,
                };

                // Validate table name
                if !table_name
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
                {
                    eprintln!("Invalid table name: must be alphanumeric with underscores/dashes");
                    continue;
                }

                let status = Command::new("sudo")
                    .args(["nft", "add", "table", family_str, &table_name])
                    .status();

                match status {
                    Ok(s) if s.success() => println!("Table created"),
                    _ => println!("Failed to create table"),
                }
            }
            2 => {
                let table_name: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter table name")
                    .interact_text()
                {
                    Ok(i) => i,
                    Err(_) => continue,
                };

                // Validate table name
                if !table_name
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
                {
                    eprintln!("Invalid table name");
                    continue;
                }

                let chain_name: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter chain name")
                    .interact_text()
                {
                    Ok(i) => i,
                    Err(_) => continue,
                };

                // Validate chain name
                if !chain_name
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
                {
                    eprintln!("Invalid chain name");
                    continue;
                }

                let hook = match Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select hook")
                    .items(&["input", "output", "forward", "prerouting", "postrouting"])
                    .default(0)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => continue,
                };

                let hook_str = ["input", "output", "forward", "prerouting", "postrouting"][hook];

                let priority: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter priority (0 for filter)")
                    .default("0".to_string())
                    .interact_text()
                {
                    Ok(i) => i,
                    Err(_) => continue,
                };

                // Validate priority (must be numeric, optionally negative)
                if !priority
                    .chars()
                    .enumerate()
                    .all(|(i, c)| c.is_ascii_digit() || (i == 0 && c == '-'))
                {
                    eprintln!("Invalid priority: must be a number");
                    continue;
                }

                // Build chain spec
                let chain_spec =
                    format!("{{ type filter hook {} priority {}; }}", hook_str, priority);

                let status = Command::new("sudo")
                    .args([
                        "nft",
                        "add",
                        "chain",
                        "inet",
                        &table_name,
                        &chain_name,
                        &chain_spec,
                    ])
                    .status();

                match status {
                    Ok(s) if s.success() => println!("Chain created"),
                    _ => println!("Failed to create chain"),
                }
            }
            3 => {
                add_nftables_rule();
            }
            4 => {
                println!("Current rules:");
                Command::new("sudo")
                    .args(&["nft", "-a", "list", "ruleset"])
                    .status()
                    .ok();

                let handle: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter rule handle to delete")
                    .interact_text()
                {
                    Ok(i) => i,
                    Err(_) => continue,
                };

                // Validate handle (must be numeric)
                if !handle.chars().all(|c| c.is_ascii_digit()) {
                    eprintln!("Invalid handle: must be numeric");
                    continue;
                }

                let table: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter table name")
                    .interact_text()
                {
                    Ok(i) => i,
                    Err(_) => continue,
                };

                // Validate table name
                if !table
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
                {
                    eprintln!("Invalid table name");
                    continue;
                }

                let chain: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter chain name")
                    .interact_text()
                {
                    Ok(i) => i,
                    Err(_) => continue,
                };

                // Validate chain name
                if !chain
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
                {
                    eprintln!("Invalid chain name");
                    continue;
                }

                Command::new("sudo")
                    .args([
                        "nft", "delete", "rule", "inet", &table, &chain, "handle", &handle,
                    ])
                    .status()
                    .ok();
            }
            5 => {
                let path: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter save path")
                    .default("/etc/nftables.conf".to_string())
                    .interact_text()
                {
                    Ok(i) => i,
                    Err(_) => continue,
                };

                // Validate path (no shell metacharacters)
                if path.contains(';')
                    || path.contains('|')
                    || path.contains('&')
                    || path.contains('`')
                    || path.contains('$')
                {
                    eprintln!("Invalid path: contains shell metacharacters");
                    continue;
                }

                // Use nft output and write via tee
                if let Ok(output) = Command::new("sudo")
                    .args(["nft", "list", "ruleset"])
                    .output()
                    && output.status.success()
                {
                    use std::io::Write;
                    let result = std::process::Command::new("sudo")
                        .args(["tee", &path])
                        .stdin(std::process::Stdio::piped())
                        .stdout(std::process::Stdio::null())
                        .spawn()
                        .and_then(|mut child| {
                            if let Some(stdin) = child.stdin.as_mut() {
                                stdin.write_all(&output.stdout)?;
                            }
                            child.wait()
                        });
                    match result {
                        Ok(_) => println!("Configuration saved to {}", path),
                        Err(e) => eprintln!("Failed to save: {}", e),
                    }
                }
            }
            6 => {
                let path: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter configuration file path")
                    .default("/etc/nftables.conf".to_string())
                    .interact_text()
                {
                    Ok(i) => i,
                    Err(_) => continue,
                };

                // Validate path
                if path.contains(';')
                    || path.contains('|')
                    || path.contains('&')
                    || path.contains('`')
                    || path.contains('$')
                {
                    eprintln!("Invalid path: contains shell metacharacters");
                    continue;
                }

                let status = Command::new("sudo").args(["nft", "-f", &path]).status();

                match status {
                    Ok(s) if s.success() => println!("Configuration loaded"),
                    _ => println!("Failed to load configuration"),
                }
            }
            7 => {
                println!("📊 nftables Status:");
                Command::new("systemctl")
                    .args(&["status", "nftables"])
                    .status()
                    .ok();

                println!("\n📋 Active tables:");
                Command::new("sudo")
                    .args(&["nft", "list", "tables"])
                    .status()
                    .ok();
            }
            _ => break,
        }
    }
}

fn add_nftables_rule() {
    println!("Add nftables Rule");

    let table: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    // Validate table name
    if !table
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        eprintln!("Invalid table name");
        return;
    }

    let chain: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter chain name")
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    // Validate chain name
    if !chain
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        eprintln!("Invalid chain name");
        return;
    }

    let rule_type = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select rule type")
        .items(&["Accept", "Drop", "Reject", "Log", "Counter"])
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    // Build rule parts as separate validated tokens
    let mut rule_parts: Vec<String> = Vec::new();

    // Source
    let source: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Source IP (or press Enter to skip)")
        .allow_empty(true)
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    if !source.is_empty() {
        if let Ok(validated_ip) = ValidatedIpAddress::from_input(&source) {
            rule_parts.push("ip".to_string());
            rule_parts.push("saddr".to_string());
            rule_parts.push(validated_ip.value().to_string());
        } else if let Ok(validated_cidr) = ValidatedCidr::from_input(&source) {
            rule_parts.push("ip".to_string());
            rule_parts.push("saddr".to_string());
            rule_parts.push(validated_cidr.value().to_string());
        } else {
            eprintln!("Invalid source IP/CIDR");
            return;
        }
    }

    // Destination
    let dest: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Destination IP (or press Enter to skip)")
        .allow_empty(true)
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    if !dest.is_empty() {
        if let Ok(validated_ip) = ValidatedIpAddress::from_input(&dest) {
            rule_parts.push("ip".to_string());
            rule_parts.push("daddr".to_string());
            rule_parts.push(validated_ip.value().to_string());
        } else if let Ok(validated_cidr) = ValidatedCidr::from_input(&dest) {
            rule_parts.push("ip".to_string());
            rule_parts.push("daddr".to_string());
            rule_parts.push(validated_cidr.value().to_string());
        } else {
            eprintln!("Invalid destination IP/CIDR");
            return;
        }
    }

    // Protocol and port
    let use_port = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Specify port?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if use_port {
        let protocol = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Protocol")
            .items(&["tcp", "udp"])
            .default(0)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            Ok(None) | Err(_) => return,
        };

        let proto = ["tcp", "udp"][protocol];

        let port: String = match Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Port number")
            .interact_text()
        {
            Ok(i) => i,
            Err(_) => return,
        };

        match ValidatedPort::from_input(&port) {
            Ok(validated_port) => {
                rule_parts.push(proto.to_string());
                rule_parts.push("dport".to_string());
                rule_parts.push(validated_port.to_string().to_string());
            }
            Err(e) => {
                eprintln!("Invalid port: {}", e);
                return;
            }
        }
    }

    // Action (from fixed selection, safe)
    let action = match rule_type {
        0 => "accept",
        1 => "drop",
        2 => "reject",
        3 => "log",
        4 => "counter",
        _ => "accept",
    };

    rule_parts.push(action.to_string());

    // Build args
    let mut args = vec!["nft", "add", "rule", "inet", &table, &chain];
    let rule_parts_refs: Vec<&str> = rule_parts.iter().map(|s| s.as_str()).collect();
    args.extend(rule_parts_refs);

    println!("Executing: sudo {}", args.join(" "));

    let status = Command::new("sudo").args(&args).status();

    match status {
        Ok(s) if s.success() => println!("Rule added"),
        _ => println!("Failed to add rule"),
    }
}

fn port_scanner() {
    println!("🔍 Port Scanner & Checker");

    let options = [
        "🔍 Scan local ports",
        "🌐 Scan remote host",
        "📋 Check specific port",
        "🔊 Check listening services",
        "📊 Port usage statistics",
        "Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Port Scanner")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            println!("🔍 Scanning local ports...");

            let scan_type = match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select scan type")
                .items(&["All ports", "Common ports", "Custom range"])
                .default(1)
                .interact_opt()
            {
                Ok(Some(c)) => c,
                Ok(None) | Err(_) => return,
            };

            match scan_type {
                0 => {
                    Command::new("sudo").args(&["ss", "-tuln"]).status().ok();
                }
                1 => {
                    println!("📋 Common ports status:");
                    let common_ports = vec![
                        ("22", "SSH"),
                        ("80", "HTTP"),
                        ("443", "HTTPS"),
                        ("3306", "MySQL"),
                        ("5432", "PostgreSQL"),
                        ("6379", "Redis"),
                        ("8080", "HTTP Alt"),
                        ("9000", "PHP-FPM"),
                    ];

                    for (port, service) in common_ports {
                        let output = Command::new("ss")
                            .args(&["-tuln", "|", "grep", port])
                            .output();

                        match output {
                            Ok(out) if !out.stdout.is_empty() => {
                                println!("  ✅ Port {} ({}) - LISTENING", port, service);
                            }
                            _ => {
                                println!("  ⭕ Port {} ({}) - CLOSED", port, service);
                            }
                        }
                    }
                }
                2 => {
                    let start: String = match Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Start port")
                        .interact_text()
                    {
                        Ok(i) => i,
                        Err(_) => return,
                    };

                    let end: String = match Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("End port")
                        .interact_text()
                    {
                        Ok(i) => i,
                        Err(_) => return,
                    };

                    // Validate ports
                    let start_port: u16 = match start.parse() {
                        Ok(p) if p > 0 => p,
                        _ => {
                            eprintln!("Invalid start port");
                            return;
                        }
                    };
                    let end_port: u16 = match end.parse() {
                        Ok(p) if p > 0 && p >= start_port => p,
                        _ => {
                            eprintln!("Invalid end port");
                            return;
                        }
                    };

                    // Use nc directly with validated ports
                    println!("Scanning ports {} to {}...", start_port, end_port);
                    for port in start_port..=end_port {
                        let result = Command::new("nc")
                            .args(["-zv", "-w", "1", "localhost", &port.to_string()])
                            .output();
                        if let Ok(out) = result {
                            let stderr = String::from_utf8_lossy(&out.stderr);
                            if stderr.contains("succeeded") || out.status.success() {
                                println!("Port {} - OPEN", port);
                            }
                        }
                    }
                }
                _ => {
                    println!("Invalid scan type selected");
                }
            }
        }
        1 => {
            let host: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter host to scan")
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            let port_range: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter port range (e.g., 1-1000)")
                .default("1-1000".to_string())
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            println!("🔍 Scanning {}...", host);

            // Validate host (prevent shell injection)
            if !is_valid_target(&host) {
                println!("Invalid host format");
                return;
            }

            // Validate port range format
            let ports: Vec<&str> = port_range.split('-').collect();
            if ports.len() != 2 {
                println!("Invalid port range format. Use: start-end");
                return;
            }
            let start_port: u16 = match ports[0].parse() {
                Ok(p) => p,
                Err(_) => {
                    println!("Invalid start port");
                    return;
                }
            };
            let end_port: u16 = match ports[1].parse() {
                Ok(p) => p,
                Err(_) => {
                    println!("Invalid end port");
                    return;
                }
            };

            // Use nmap if available, otherwise nc
            let nmap_check = Command::new("which").arg("nmap").status();

            if let Ok(s) = nmap_check {
                if s.success() {
                    let port_arg = format!("{}-{}", start_port, end_port);
                    Command::new("nmap")
                        .args(["-p", &port_arg, &host])
                        .status()
                        .ok();
                } else {
                    println!("⚠️ nmap not found, using nc...");
                    for port in start_port..=end_port {
                        let port_str = port.to_string();
                        if let Ok(output) = Command::new("nc")
                            .args(["-zv", "-w", "1", &host, &port_str])
                            .output()
                        {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            if stderr.contains("succeeded") || stderr.contains("open") {
                                println!("  Port {} - OPEN", port);
                            }
                        }
                    }
                }
            }
        }
        2 => {
            let host: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter host (or localhost)")
                .default("localhost".to_string())
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            let port: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter port number")
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            // Validate inputs
            if !is_valid_target(&host) {
                println!("Invalid host format");
                return;
            }
            let port_num: u16 = match port.parse() {
                Ok(p) => p,
                Err(_) => {
                    println!("Invalid port number");
                    return;
                }
            };

            println!("🔍 Checking {}:{}...", host, port_num);

            let output = Command::new("nc")
                .args(["-zv", &host, &port_num.to_string()])
                .output();

            match output {
                Ok(out) => {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    if stderr.contains("succeeded") || out.status.success() {
                        println!("✅ Port {} is OPEN on {}", port_num, host);

                        // Try to identify service (safe: port_num is validated u16)
                        if host == "localhost" || host == "127.0.0.1" {
                            let port_arg = format!(":{}", port_num);
                            println!("\n📋 Service information:");
                            Command::new("sudo")
                                .args(["lsof", "-i", &port_arg])
                                .status()
                                .ok();
                        }
                    } else {
                        println!("❌ Port {} is CLOSED on {}", port_num, host);
                    }
                }
                _ => println!("❌ Failed to check port"),
            }
        }
        3 => {
            println!("🔊 Listening Services:");
            Command::new("sudo")
                .args(["netstat", "-tulpn"])
                .status()
                .ok();
        }
        4 => {
            println!("📊 Port Usage Statistics:");

            println!("\n📈 TCP Connections:");
            Command::new("ss").args(["-s"]).status().ok();

            // These are safe static commands with no user input
            println!("\n🔢 Port count by state:");
            Command::new("ss")
                .args(["-tan", "--no-header"])
                .output()
                .map(|out| {
                    let output = String::from_utf8_lossy(&out.stdout);
                    let mut counts: std::collections::HashMap<&str, usize> =
                        std::collections::HashMap::new();
                    for line in output.lines() {
                        if let Some(state) = line.split_whitespace().next() {
                            *counts.entry(state).or_insert(0) += 1;
                        }
                    }
                    for (state, count) in counts {
                        println!("  {} {}", count, state);
                    }
                })
                .ok();

            println!("\n🏆 Top 10 most connected ports:");
            Command::new("ss")
                .args(["-tan", "--no-header"])
                .output()
                .map(|out| {
                    let output = String::from_utf8_lossy(&out.stdout);
                    let mut port_counts: std::collections::HashMap<String, usize> =
                        std::collections::HashMap::new();
                    for line in output.lines() {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 4
                            && let Some(port) = parts[3].rsplit(':').next()
                        {
                            *port_counts.entry(port.to_string()).or_insert(0) += 1;
                        }
                    }
                    let mut sorted: Vec<_> = port_counts.iter().collect();
                    sorted.sort_by(|a, b| b.1.cmp(a.1));
                    for (port, count) in sorted.iter().take(10) {
                        println!("  {} port {}", count, port);
                    }
                })
                .ok();
        }
        _ => {}
    }
}

fn firewall_troubleshooting() {
    println!("🔧 Firewall Troubleshooting");

    let options = [
        "🔍 Diagnose connectivity issue",
        "📋 Check blocked connections",
        "🔄 Test firewall rules",
        "🚫 Find blocking rule",
        "⚡ Quick fixes",
        "📊 Firewall logs",
        "Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Troubleshooting")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            diagnose_connectivity();
        }
        1 => {
            check_blocked_connections();
        }
        2 => {
            test_firewall_rules();
        }
        3 => {
            find_blocking_rule();
        }
        4 => {
            quick_fixes();
        }
        5 => {
            view_firewall_logs();
        }
        _ => {}
    }
}

fn diagnose_connectivity() {
    println!("🔍 Diagnosing Connectivity Issue");

    let host_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter host/IP to test")
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    // Validate host
    if !is_valid_target(&host_input) {
        println!("❌ Invalid host format");
        return;
    }

    let port_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter port (or press Enter for ICMP ping)")
        .allow_empty(true)
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    // Validate port if provided
    let validated_port = if !port_input.is_empty() {
        match ValidatedPort::from_input(&port_input) {
            Ok(p) => Some(p.to_string()),
            Err(e) => {
                println!("❌ Invalid port: {}", e);
                return;
            }
        }
    } else {
        None
    };

    println!("\n🔍 Running diagnostics...");

    // 1. DNS Resolution
    println!("\n1️⃣ DNS Resolution:");
    let dns_output = Command::new("nslookup").arg(&host_input).output();

    match dns_output {
        Ok(out) if out.status.success() => {
            println!("  ✅ DNS resolution successful");
            let output_str = String::from_utf8_lossy(&out.stdout);
            for line in output_str.lines() {
                if line.contains("Address") && !line.contains("#") {
                    println!("  📍 {}", line);
                }
            }
        }
        _ => println!("  ❌ DNS resolution failed"),
    }

    // 2. Ping test
    println!("\n2️⃣ ICMP Ping Test:");
    let ping_output = Command::new("ping")
        .args(&["-c", "3", "-W", "2", &host_input])
        .output();

    match ping_output {
        Ok(out) if out.status.success() => println!("  ✅ Host is reachable via ICMP"),
        _ => println!("  ❌ Host unreachable via ICMP (may be blocked)"),
    }

    // 3. Port test if specified
    if let Some(ref port) = validated_port {
        println!("\n3️⃣ Port {} Connectivity:", port);

        let nc_output = Command::new("nc")
            .args(&["-zv", "-w", "2", &host_input, port])
            .output();

        match nc_output {
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                if stderr.contains("succeeded") || out.status.success() {
                    println!("  ✅ Port {} is open", port);
                } else {
                    println!("  ❌ Port {} is closed or filtered", port);
                }
            }
            _ => println!("  ❌ Connection test failed"),
        }

        // Traceroute to port
        println!("\n4️⃣ Traceroute to port {}:", port);
        let tcptraceroute = Command::new("which").arg("tcptraceroute").status();

        if let Ok(s) = tcptraceroute {
            if s.success() {
                Command::new("sudo")
                    .args(&["tcptraceroute", &host_input, port])
                    .status()
                    .ok();
            } else {
                println!("  ⚠️ tcptraceroute not installed, using regular traceroute");
                Command::new("traceroute")
                    .args(&["-n", "-m", "10", &host_input])
                    .status()
                    .ok();
            }
        }
    }

    // 4. Check local firewall
    println!("\n5️⃣ Local Firewall Check:");

    // Check if any firewall is blocking
    let ufw_status = Command::new("sudo").args(&["ufw", "status"]).output();

    if let Ok(out) = ufw_status {
        let status_str = String::from_utf8_lossy(&out.stdout);
        if status_str.contains("Status: active") {
            println!("  ⚠️ UFW is active - checking rules...");

            if let Some(ref port) = validated_port {
                // Use Rust string filtering instead of grep piping
                let ufw_output = Command::new("sudo").args(&["ufw", "status"]).output();

                if let Ok(out) = ufw_output {
                    let output_str = String::from_utf8_lossy(&out.stdout);
                    for line in output_str.lines() {
                        if line.contains(port) {
                            println!("  {}", line);
                        }
                    }
                }
            }
        }
    }

    // Check iptables
    if let Some(ref port) = validated_port {
        let iptables_out = Command::new("sudo")
            .args(&["iptables", "-L", "-n"])
            .output();

        if let Ok(out) = iptables_out {
            let output_str = String::from_utf8_lossy(&out.stdout);
            let matching_lines: Vec<&str> = output_str
                .lines()
                .filter(|line| line.contains(port))
                .collect();
            if !matching_lines.is_empty() {
                println!("  ⚠️ Found iptables rules for port {}", port);
                for line in matching_lines {
                    println!("    {}", line);
                }
            }
        }
    }

    println!("\n📋 Diagnosis Summary:");
    println!("  - Check if the service is running on the target host");
    println!("  - Verify firewall rules on both source and destination");
    println!("  - Check for any network ACLs or security groups");
    println!("  - Ensure routing is configured correctly");
}

fn check_blocked_connections() {
    println!("📋 Checking Blocked Connections");

    // Check dropped packets
    println!("\n🚫 Dropped Packets (iptables):");
    Command::new("sudo")
        .args(&["iptables", "-nvL", "INPUT"])
        .status()
        .ok();

    println!("\n🚫 Dropped Packets (iptables OUTPUT):");
    Command::new("sudo")
        .args(&["iptables", "-nvL", "OUTPUT"])
        .status()
        .ok();

    // Check connection tracking
    println!("\n📊 Connection Tracking:");
    Command::new("sudo")
        .args(&["conntrack", "-L"])
        .status()
        .ok();

    // Check recent blocks in logs
    println!("\n📝 Recent Blocked Connections (last 20):");
    match safe_commands::journalctl_firewall_recent(100) {
        Ok(lines) => {
            for line in lines.iter().take(20) {
                println!("{}", line);
            }
        }
        Err(e) => println!("⚠️ Failed to read logs: {}", e),
    }
}

fn test_firewall_rules() {
    println!("🔄 Test Firewall Rules");

    let test_type = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select test type")
        .items(&["Test specific rule", "Test all rules", "Simulate packet"])
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match test_type {
        0 => {
            let source_input: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Source IP (or any)")
                .default("any".to_string())
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            // Validate source if not "any"
            let validated_source = if source_input == "any" {
                "any".to_string()
            } else {
                match ValidatedIpAddress::from_input(&source_input) {
                    Ok(ip) => ip.value().to_string(),
                    Err(e) => {
                        println!("❌ Invalid source IP: {}", e);
                        return;
                    }
                }
            };

            let dest_input: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Destination IP (or any)")
                .default("any".to_string())
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            // Validate destination if not "any"
            let validated_dest = if dest_input == "any" {
                "any".to_string()
            } else {
                match ValidatedIpAddress::from_input(&dest_input) {
                    Ok(ip) => ip.value().to_string(),
                    Err(e) => {
                        println!("❌ Invalid destination IP: {}", e);
                        return;
                    }
                }
            };

            let port_input: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Port")
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            // Validate port
            let validated_port = match ValidatedPort::from_input(&port_input) {
                Ok(p) => p.to_string(),
                Err(e) => {
                    println!("❌ Invalid port: {}", e);
                    return;
                }
            };

            let protocol = match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Protocol")
                .items(&["tcp", "udp"])
                .default(0)
                .interact_opt()
            {
                Ok(Some(c)) => c,
                Ok(None) | Err(_) => return,
            };

            let proto = ["tcp", "udp"][protocol];

            println!("\n🔍 Checking rules that would match:");

            // Use Rust filtering instead of shell grep
            let iptables_out = Command::new("sudo")
                .args(&["iptables", "-L", "-n", "-v"])
                .output();

            if let Ok(out) = iptables_out {
                let output_str = String::from_utf8_lossy(&out.stdout);
                for line in output_str.lines() {
                    if (line.contains(&validated_source) && line.contains(&validated_dest))
                        || (line.contains(proto)
                            && line.contains(&format!("dpt:{}", validated_port)))
                    {
                        println!("{}", line);
                    }
                }
            }
        }
        1 => {
            println!("🔄 Testing all firewall rules...");
            println!("⚠️ This will show the packet flow through all chains");

            Command::new("sudo")
                .args(&[
                    "iptables",
                    "-t",
                    "filter",
                    "-L",
                    "-n",
                    "-v",
                    "--line-numbers",
                ])
                .status()
                .ok();

            Command::new("sudo")
                .args(&["iptables", "-t", "nat", "-L", "-n", "-v", "--line-numbers"])
                .status()
                .ok();
        }
        2 => {
            println!("📦 Simulate Packet Flow");
            println!("⚠️ This requires iptables-save format");

            let chain = match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Starting chain")
                .items(&["INPUT", "OUTPUT", "FORWARD"])
                .default(0)
                .interact_opt()
            {
                Ok(Some(c)) => c,
                Ok(None) | Err(_) => return,
            };

            let chain_name = ["INPUT", "OUTPUT", "FORWARD"][chain];

            println!("Tracing packet through {} chain:", chain_name);
            Command::new("sudo")
                .args(&["iptables", "-t", "filter", "-L", chain_name, "-n", "-v"])
                .status()
                .ok();
        }
        _ => {
            println!("Invalid test type selected");
        }
    }
}

fn find_blocking_rule() {
    println!("🚫 Find Blocking Rule");

    let port_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter port being blocked")
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    // Validate port
    let validated_port = match ValidatedPort::from_input(&port_input) {
        Ok(p) => p.to_string(),
        Err(e) => {
            println!("❌ Invalid port: {}", e);
            return;
        }
    };

    let direction = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Traffic direction")
        .items(&["Incoming (INPUT)", "Outgoing (OUTPUT)", "Both"])
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    println!("\n🔍 Searching for blocking rules...");

    // Helper function to filter and display iptables rules
    let filter_iptables_rules = |chain: &str, port: &str| {
        let output = Command::new("sudo")
            .args(&["iptables", "-L", chain, "-n", "-v", "--line-numbers"])
            .output();

        if let Ok(out) = output {
            let output_str = String::from_utf8_lossy(&out.stdout);
            for line in output_str.lines() {
                if (line.contains("DROP") || line.contains("REJECT")) && line.contains(port) {
                    println!("{}", line);
                }
            }
            // Also show the first line for policy
            if let Some(first_line) = output_str.lines().next() {
                println!("Policy: {}", first_line);
            }
        }
    };

    match direction {
        0 | 2 => {
            println!("\n📥 INPUT chain:");
            filter_iptables_rules("INPUT", &validated_port);
        }
        _ => {}
    }

    if direction == 1 || direction == 2 {
        println!("\n📤 OUTPUT chain:");
        filter_iptables_rules("OUTPUT", &validated_port);
    }

    // Check UFW - use Rust filtering
    println!("\n🛡️ UFW rules:");
    let ufw_output = Command::new("sudo")
        .args(&["ufw", "status", "numbered"])
        .output();

    if let Ok(out) = ufw_output {
        let output_str = String::from_utf8_lossy(&out.stdout);
        for line in output_str.lines() {
            if line.contains(&validated_port) {
                println!("{}", line);
            }
        }
    }

    // Check firewalld
    println!("\n🔥 Firewalld rules:");
    Command::new("sudo")
        .args(&["firewall-cmd", "--list-all"])
        .status()
        .ok();
}

fn quick_fixes() {
    println!("⚡ Quick Firewall Fixes");

    let options = [
        "Allow SSH (port 22)",
        "Allow HTTP/HTTPS (80/443)",
        "Allow common development ports",
        "Disable firewall temporarily",
        "Reset to default rules",
        "Allow ping (ICMP)",
        "Fix Docker networking",
        "Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Quick Fix")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            println!("🔧 Allowing SSH...");
            Command::new("sudo")
                .args(&["ufw", "allow", "22/tcp"])
                .status()
                .ok();
            Command::new("sudo")
                .args(&[
                    "iptables", "-A", "INPUT", "-p", "tcp", "--dport", "22", "-j", "ACCEPT",
                ])
                .status()
                .ok();
            println!("✅ SSH access allowed");
        }
        1 => {
            println!("🔧 Allowing HTTP/HTTPS...");
            Command::new("sudo")
                .args(&["ufw", "allow", "80/tcp"])
                .status()
                .ok();
            Command::new("sudo")
                .args(&["ufw", "allow", "443/tcp"])
                .status()
                .ok();
            Command::new("sudo")
                .args(&[
                    "iptables", "-A", "INPUT", "-p", "tcp", "--dport", "80", "-j", "ACCEPT",
                ])
                .status()
                .ok();
            Command::new("sudo")
                .args(&[
                    "iptables", "-A", "INPUT", "-p", "tcp", "--dport", "443", "-j", "ACCEPT",
                ])
                .status()
                .ok();
            println!("✅ HTTP/HTTPS access allowed");
        }
        2 => {
            println!("🔧 Allowing development ports...");
            let dev_ports: Vec<&str> = vec![
                "3000", "3001", "8000", "8080", "8081", "5000", "5001", "4200", "9000",
            ];

            for port in dev_ports {
                let port_arg = format!("{}/tcp", port);
                Command::new("sudo")
                    .args(["ufw", "allow", &port_arg])
                    .status()
                    .ok();
            }
            println!("✅ Common development ports allowed");
        }
        3 => {
            let duration_input: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Disable for how many minutes? (0 for permanent)")
                .default("5".to_string())
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            // Validate duration is numeric
            let duration: u32 = match duration_input.parse() {
                Ok(d) => d,
                Err(_) => {
                    println!("❌ Invalid duration: must be a number");
                    return;
                }
            };

            // Require confirmation for dangerous operation
            let confirm_msg = if duration == 0 {
                "⚠️ WARNING: This will PERMANENTLY disable the firewall. Continue?"
            } else {
                "Disable firewall temporarily?"
            };

            let confirmed = match Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(confirm_msg)
                .default(false)
                .interact_opt()
            {
                Ok(Some(c)) => c,
                Ok(None) | Err(_) => return,
            };

            if !confirmed {
                println!("Operation cancelled");
                return;
            }

            if duration == 0 {
                let status = Command::new("sudo").args(&["ufw", "disable"]).status();
                if let Err(e) = status {
                    println!("Failed to disable ufw: {}", e);
                }
                let status = Command::new("sudo")
                    .args(&["systemctl", "stop", "firewalld"])
                    .status();
                if let Err(e) = status {
                    println!("Failed to stop firewalld: {}", e);
                }
                println!("⚠️ Firewall disabled permanently");
            } else {
                let status = Command::new("sudo").args(&["ufw", "disable"]).status();
                if let Err(e) = status {
                    println!("Failed to disable ufw: {}", e);
                }
                let status = Command::new("sudo")
                    .args(&["systemctl", "stop", "firewalld"])
                    .status();
                if let Err(e) = status {
                    println!("Failed to stop firewalld: {}", e);
                }

                println!("⚠️ Firewall disabled for {} minutes", duration);
                println!("⏰ Will re-enable automatically");

                // Spawn background thread to re-enable firewall after duration
                let duration_mins = duration;
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(duration_mins as u64 * 60));
                    // Re-enable firewalls
                    let _ = Command::new("sudo").args(["ufw", "enable"]).status();
                    let _ = Command::new("sudo")
                        .args(["systemctl", "start", "firewalld"])
                        .status();
                    log::info!("Firewall re-enabled after {} minute timeout", duration_mins);
                });
            }
        }
        4 => {
            println!("⚠️ WARNING: This will flush ALL firewall rules and reset to defaults.");
            println!("   This may temporarily expose your system to network attacks.");

            let confirm = match Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Reset firewall to default rules? This cannot be undone.")
                .default(false)
                .interact_opt()
            {
                Ok(Some(c)) => c,
                Ok(None) | Err(_) => return,
            };

            if confirm {
                println!("🔄 Resetting firewall...");

                // Reset UFW
                Command::new("sudo")
                    .args(&["ufw", "--force", "reset"])
                    .status()
                    .ok();
                Command::new("sudo")
                    .args(&["ufw", "default", "deny", "incoming"])
                    .status()
                    .ok();
                Command::new("sudo")
                    .args(&["ufw", "default", "allow", "outgoing"])
                    .status()
                    .ok();
                Command::new("sudo")
                    .args(&["ufw", "allow", "ssh"])
                    .status()
                    .ok();
                Command::new("sudo").args(&["ufw", "enable"]).status().ok();

                // Reset iptables
                Command::new("sudo").args(&["iptables", "-F"]).status().ok();
                Command::new("sudo").args(&["iptables", "-X"]).status().ok();
                Command::new("sudo")
                    .args(&["iptables", "-t", "nat", "-F"])
                    .status()
                    .ok();
                Command::new("sudo")
                    .args(&["iptables", "-t", "nat", "-X"])
                    .status()
                    .ok();
                Command::new("sudo")
                    .args(&["iptables", "-t", "mangle", "-F"])
                    .status()
                    .ok();
                Command::new("sudo")
                    .args(&["iptables", "-t", "mangle", "-X"])
                    .status()
                    .ok();
                Command::new("sudo")
                    .args(&["iptables", "-P", "INPUT", "ACCEPT"])
                    .status()
                    .ok();
                Command::new("sudo")
                    .args(&["iptables", "-P", "FORWARD", "ACCEPT"])
                    .status()
                    .ok();
                Command::new("sudo")
                    .args(&["iptables", "-P", "OUTPUT", "ACCEPT"])
                    .status()
                    .ok();

                println!("✅ Firewall reset to defaults");
            }
        }
        5 => {
            println!("🔧 Allowing ICMP (ping)...");
            Command::new("sudo")
                .args(&["iptables", "-A", "INPUT", "-p", "icmp", "-j", "ACCEPT"])
                .status()
                .ok();
            Command::new("sudo")
                .args(&["iptables", "-A", "OUTPUT", "-p", "icmp", "-j", "ACCEPT"])
                .status()
                .ok();
            println!("✅ ICMP/ping allowed");
        }
        6 => {
            println!("🐳 Fixing Docker networking...");

            // Restart Docker
            Command::new("sudo")
                .args(&["systemctl", "restart", "docker"])
                .status()
                .ok();

            // Allow Docker bridge
            Command::new("sudo")
                .args(&["iptables", "-A", "FORWARD", "-i", "docker0", "-j", "ACCEPT"])
                .status()
                .ok();
            Command::new("sudo")
                .args(&["iptables", "-A", "FORWARD", "-o", "docker0", "-j", "ACCEPT"])
                .status()
                .ok();

            // Fix Docker DNS
            Command::new("sudo")
                .args(&[
                    "iptables", "-A", "INPUT", "-i", "docker0", "-p", "udp", "--dport", "53", "-j",
                    "ACCEPT",
                ])
                .status()
                .ok();

            println!("✅ Docker networking rules applied");
        }
        _ => {}
    }
}

fn view_firewall_logs() {
    println!("📊 Firewall Logs");

    let log_type = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select log source")
        .items(&[
            "UFW logs",
            "iptables logs",
            "Firewalld logs",
            "All firewall logs",
            "Live monitoring",
        ])
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match log_type {
        0 => {
            println!("📋 UFW Logs:");
            Command::new("sudo")
                .args(&["grep", "UFW", "/var/log/syslog"])
                .status()
                .ok();
        }
        1 => {
            println!("📋 iptables Logs:");
            Command::new("sudo")
                .args(&["journalctl", "-xe", "|", "grep", "iptables"])
                .status()
                .ok();
        }
        2 => {
            println!("📋 Firewalld Logs:");
            Command::new("sudo")
                .args(&["journalctl", "-u", "firewalld", "-n", "50"])
                .status()
                .ok();
        }
        3 => {
            println!("📋 All Firewall Logs (last 100):");
            match safe_commands::journalctl_firewall_recent(100) {
                Ok(lines) => {
                    for line in lines {
                        println!("{}", line);
                    }
                }
                Err(e) => println!("⚠️ Failed to read logs: {}", e),
            }
        }
        4 => {
            println!("👁️ Live Firewall Monitoring (Ctrl+C to stop):");
            // Use direct tail command with proper arguments
            Command::new("sudo")
                .args([
                    "journalctl",
                    "-f",
                    "-u",
                    "ufw",
                    "-u",
                    "firewalld",
                    "-u",
                    "iptables",
                ])
                .status()
                .ok();
        }
        _ => {}
    }
}

fn firewall_status_overview() {
    println!("📋 Firewall Status Overview");
    println!("============================\n");

    // Check UFW
    println!("🛡️ UFW Status:");
    let ufw_status = Command::new("sudo").args(&["ufw", "status"]).output();

    if let Ok(out) = ufw_status {
        let status_str = String::from_utf8_lossy(&out.stdout);
        if status_str.contains("Status: active") {
            println!("  ✅ UFW is ACTIVE");

            // Count rules
            let rule_count = status_str.lines().count() - 4;
            println!("  📊 {} rules configured", rule_count);
        } else if status_str.contains("inactive") {
            println!("  ⭕ UFW is INACTIVE");
        } else {
            println!("  ❌ UFW not installed");
        }
    }

    // Check firewalld
    println!("\n🔥 Firewalld Status:");
    let firewalld_status = Command::new("systemctl")
        .args(&["is-active", "firewalld"])
        .output();

    if let Ok(out) = firewalld_status {
        let status_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if status_str == "active" {
            println!("  ✅ Firewalld is ACTIVE");

            // Get default zone
            let zone = Command::new("sudo")
                .args(&["firewall-cmd", "--get-default-zone"])
                .output();

            if let Ok(z) = zone {
                println!(
                    "  🌐 Default zone: {}",
                    String::from_utf8_lossy(&z.stdout).trim()
                );
            }
        } else {
            println!("  ⭕ Firewalld is INACTIVE");
        }
    }

    // Check iptables
    println!("\n⚙️ iptables Status:");
    match safe_commands::iptables_rule_count() {
        Ok(count) => {
            if count > 10 {
                println!("  ✅ iptables has {} rules configured", count);

                // Check default policies
                println!("  📋 Default policies:");
                match safe_commands::iptables_get_policies() {
                    Ok(policies) => {
                        for policy in policies {
                            println!("    {}", policy);
                        }
                    }
                    Err(e) => println!("    ⚠️ Failed to get policies: {}", e),
                }
            } else {
                println!("  ⭕ iptables has minimal rules");
            }
        }
        Err(e) => println!("  ⚠️ Failed to count rules: {}", e),
    }

    // Check nftables
    println!("\n🚀 nftables Status:");
    let nft_check = Command::new("systemctl")
        .args(&["is-active", "nftables"])
        .output();

    if let Ok(out) = nft_check {
        let status_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if status_str == "active" {
            println!("  ✅ nftables is ACTIVE");

            let tables = Command::new("sudo")
                .args(&["nft", "list", "tables"])
                .output();

            if let Ok(t) = tables {
                let table_list = String::from_utf8_lossy(&t.stdout);
                let table_count = table_list.lines().count();
                println!("  📊 {} tables configured", table_count);
            }
        } else {
            println!("  ⭕ nftables is INACTIVE");
        }
    }

    // Summary
    println!("\n📊 Quick Stats:");
    match safe_commands::ss_listening_count() {
        Ok(count) => println!("  🔌 Open ports: {}", count),
        Err(e) => println!("  🔌 Open ports: (error: {})", e),
    }

    match safe_commands::ss_established_count() {
        Ok(count) => println!("  🌐 Active connections: {}", count),
        Err(e) => println!("  🌐 Active connections: (error: {})", e),
    }

    println!("\n💡 Tip: Use the specific management options to configure each firewall");
}

fn gaming_network_optimization() {
    println!("🎮 Gaming Network Optimization");
    println!("==============================\n");

    let options = [
        "⚡ Gaming Port Optimization",
        "🎯 Anti-cheat Firewall Rules",
        "🚀 Low Latency Configuration",
        "📡 Gaming Service Ports",
        "🔧 QoS Gaming Priority",
        "🌐 Gaming DNS Optimization",
        "📊 Gaming Network Analysis",
        "⬅️ Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🎮 Gaming Network Optimization")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => gaming_port_optimization(),
        1 => anticheat_firewall_rules(),
        2 => low_latency_configuration(),
        3 => gaming_service_ports(),
        4 => qos_gaming_priority(),
        5 => gaming_dns_optimization(),
        6 => gaming_network_analysis(),
        _ => {}
    }
}

fn gaming_port_optimization() {
    println!("⚡ Gaming Port Optimization");
    println!("==========================\n");

    let games = [
        "🎮 All Popular Games (Recommended)",
        "⚔️ World of Warcraft",
        "🔥 Diablo 4",
        "🔫 Counter-Strike 2",
        "⚡ League of Legends",
        "🚀 Rocket League",
        "👑 Fortnite",
        "🎯 Valorant",
        "🎲 Discord Gaming",
        "🖥️ Steam Gaming",
        "🎮 Custom Game Ports",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select gaming platform to optimize")
        .items(&games)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => optimize_all_gaming_ports(),
        1 => optimize_wow_ports(),
        2 => optimize_diablo4_ports(),
        3 => optimize_cs2_ports(),
        4 => optimize_lol_ports(),
        5 => optimize_rocket_league_ports(),
        6 => optimize_fortnite_ports(),
        7 => optimize_valorant_ports(),
        8 => optimize_discord_gaming(),
        9 => optimize_steam_gaming(),
        10 => optimize_custom_game_ports(),
        _ => {}
    }
}

fn optimize_all_gaming_ports() {
    println!("🎮 Optimizing All Popular Gaming Ports");
    println!("=====================================\n");

    let gaming_ports: Vec<(&str, &str, &str)> = vec![
        // Battle.net games
        ("1119", "tcp", "Battle.net"),
        ("3724", "tcp", "World of Warcraft"),
        ("6113", "tcp", "Battle.net"),
        ("6881:6999", "tcp", "Blizzard Downloader"),
        // Steam
        ("27000:27100", "udp", "Steam Client"),
        ("27015:27030", "tcp", "Steam"),
        ("27015:27030", "udp", "Steam"),
        // Discord
        ("50000:65535", "udp", "Discord Voice"),
        // Popular games
        ("7777:7784", "tcp", "Unreal Tournament"),
        ("27015", "tcp", "Source Games"),
        ("25565", "tcp", "Minecraft"),
        ("19132", "udp", "Minecraft Bedrock"),
        // Console gaming
        ("53", "udp", "Console DNS"),
        ("80", "tcp", "Console Updates"),
        ("443", "tcp", "Console Services"),
        ("3478:3480", "udp", "PlayStation/Xbox"),
    ];

    println!("🔧 Configuring firewall rules for optimal gaming...");

    for (port, protocol, service) in &gaming_ports {
        println!("  ⚡ Optimizing {} - {} ({})", service, port, protocol);

        // UFW rules - port ranges already use : format
        let port_arg = format!("{}/{}", port, protocol);
        Command::new("sudo")
            .args(["ufw", "allow", &port_arg])
            .status()
            .ok();

        // iptables rules with priority - use direct args
        let comment = format!("Gaming: {}", service);
        Command::new("sudo")
            .args([
                "iptables",
                "-A",
                "INPUT",
                "-p",
                protocol,
                "--dport",
                port,
                "-j",
                "ACCEPT",
                "-m",
                "comment",
                "--comment",
                &comment,
            ])
            .status()
            .ok();

        // Firewalld rules
        let firewalld_port = format!("--add-port={}/{}", port, protocol);
        Command::new("sudo")
            .args(["firewall-cmd", "--permanent", &firewalld_port])
            .status()
            .ok();
    }

    // Reload firewalld
    Command::new("sudo")
        .args(&["firewall-cmd", "--reload"])
        .status()
        .ok();

    println!("\n✅ Gaming port optimization completed!");
    println!("🎮 Optimized ports for:");
    println!("  - Battle.net games (WoW, Diablo, etc.)");
    println!("  - Steam platform");
    println!("  - Discord gaming");
    println!("  - Popular multiplayer games");
    println!("  - Console gaming services");
}

fn optimize_wow_ports() {
    println!("⚔️ World of Warcraft Port Optimization");
    println!("======================================\n");

    let wow_ports: Vec<(&str, &str, &str)> = vec![
        ("1119", "tcp", "Battle.net Authentication"),
        ("3724", "tcp", "WoW Game Connection"),
        ("6112", "tcp", "Battle.net"),
        ("6113", "tcp", "Battle.net"),
        ("6881:6999", "tcp", "Blizzard Downloader"),
        ("80", "tcp", "Battle.net Web"),
        ("443", "tcp", "Battle.net HTTPS"),
    ];

    println!("⚔️ Configuring optimal firewall rules for World of Warcraft...");

    for (port, protocol, service) in &wow_ports {
        println!("  ⚡ Configuring {} - {}", service, port);

        // Priority iptables rules for WoW
        let comment = format!("WoW {}", service);
        Command::new("sudo")
            .args([
                "iptables",
                "-I",
                "INPUT",
                "1",
                "-p",
                protocol,
                "--dport",
                port,
                "-j",
                "ACCEPT",
                "-m",
                "comment",
                "--comment",
                &comment,
            ])
            .status()
            .ok();

        // UFW allow
        let port_arg = format!("{}/{}", port, protocol);
        Command::new("sudo")
            .args(["ufw", "allow", &port_arg])
            .status()
            .ok();

        // Firewalld
        let firewalld_arg = format!("--add-port={}/{}", port, protocol);
        Command::new("sudo")
            .args(["firewall-cmd", "--permanent", &firewalld_arg])
            .status()
            .ok();
    }

    // WoW-specific optimizations
    println!("\n🚀 Applying WoW-specific network optimizations...");

    // Prioritize WoW traffic using direct commands
    Command::new("sudo")
        .args([
            "iptables",
            "-t",
            "mangle",
            "-A",
            "OUTPUT",
            "-p",
            "tcp",
            "--dport",
            "3724",
            "-j",
            "DSCP",
            "--set-dscp-class",
            "EF",
        ])
        .status()
        .ok();

    Command::new("sudo")
        .args([
            "iptables",
            "-t",
            "mangle",
            "-A",
            "OUTPUT",
            "-p",
            "tcp",
            "--dport",
            "1119",
            "-j",
            "DSCP",
            "--set-dscp-class",
            "AF41",
        ])
        .status()
        .ok();

    Command::new("sudo")
        .args(&["firewall-cmd", "--reload"])
        .status()
        .ok();

    println!("✅ World of Warcraft network optimization completed!");
    println!("⚔️ Configured priority traffic handling for WoW connections");
}

fn optimize_diablo4_ports() {
    println!("🔥 Diablo 4 Port Optimization");
    println!("=============================\n");

    let d4_ports: Vec<(&str, &str, &str)> = vec![
        ("1119", "tcp", "Battle.net Authentication"),
        ("6112:6119", "tcp", "Battle.net Services"),
        ("80", "tcp", "Battle.net Web Services"),
        ("443", "tcp", "Battle.net HTTPS"),
        ("27000:27050", "tcp", "Diablo 4 Game Servers"),
        ("3478:3480", "udp", "Voice Chat"),
        ("6881:6999", "tcp", "Blizzard Downloader"),
    ];

    println!("🔥 Configuring optimal firewall rules for Diablo 4...");

    for (port, protocol, service) in &d4_ports {
        println!("  ⚡ Configuring {} - {}", service, port);

        // High-priority rules for Diablo 4
        let comment = format!("D4 {}", service);
        Command::new("sudo")
            .args([
                "iptables",
                "-I",
                "INPUT",
                "1",
                "-p",
                protocol,
                "--dport",
                port,
                "-j",
                "ACCEPT",
                "-m",
                "comment",
                "--comment",
                &comment,
            ])
            .status()
            .ok();

        let port_arg = format!("{}/{}", port, protocol);
        Command::new("sudo")
            .args(["ufw", "allow", &port_arg])
            .status()
            .ok();

        let firewalld_arg = format!("--add-port={}/{}", port, protocol);
        Command::new("sudo")
            .args(["firewall-cmd", "--permanent", &firewalld_arg])
            .status()
            .ok();
    }

    // D4-specific optimizations for anti-cheat
    println!("\n🛡️ Configuring anti-cheat friendly rules...");

    // Allow Diablo 4 anti-cheat communication
    Command::new("sudo")
        .args([
            "iptables",
            "-A",
            "INPUT",
            "-m",
            "state",
            "--state",
            "ESTABLISHED,RELATED",
            "-j",
            "ACCEPT",
        ])
        .status()
        .ok();

    Command::new("sudo")
        .args([
            "iptables",
            "-A",
            "OUTPUT",
            "-m",
            "state",
            "--state",
            "NEW,ESTABLISHED",
            "-j",
            "ACCEPT",
        ])
        .status()
        .ok();

    Command::new("sudo")
        .args([
            "iptables",
            "-t",
            "mangle",
            "-A",
            "OUTPUT",
            "-p",
            "tcp",
            "--dport",
            "27000:27050",
            "-j",
            "DSCP",
            "--set-dscp-class",
            "EF",
        ])
        .status()
        .ok();

    Command::new("sudo")
        .args(&["firewall-cmd", "--reload"])
        .status()
        .ok();

    println!("✅ Diablo 4 network optimization completed!");
    println!("🔥 Configured for optimal D4 performance and anti-cheat compatibility");
}

fn optimize_cs2_ports() {
    println!("🔫 Counter-Strike 2 Port Optimization");
    println!("=====================================\n");

    let cs2_ports: Vec<(&str, &str, &str)> = vec![
        ("27015", "tcp", "CS2 Game Server"),
        ("27015", "udp", "CS2 Game Server"),
        ("27005", "tcp", "Steam Client Service"),
        ("27000:27100", "udp", "Steam Client"),
        ("4380", "tcp", "Steam Local"),
        ("26900", "tcp", "Steam Networking"),
        ("26900", "udp", "Steam Networking"),
    ];

    for (port, protocol, service) in &cs2_ports {
        let comment = format!("CS2 {}", service);
        Command::new("sudo")
            .args([
                "iptables",
                "-I",
                "INPUT",
                "1",
                "-p",
                protocol,
                "--dport",
                port,
                "-j",
                "ACCEPT",
                "-m",
                "comment",
                "--comment",
                &comment,
            ])
            .status()
            .ok();

        let port_arg = format!("{}/{}", port, protocol);
        Command::new("sudo")
            .args(["ufw", "allow", &port_arg])
            .status()
            .ok();
    }

    // CS2-specific low-latency optimizations
    Command::new("sudo")
        .args([
            "iptables",
            "-t",
            "mangle",
            "-A",
            "OUTPUT",
            "-p",
            "udp",
            "--dport",
            "27015",
            "-j",
            "DSCP",
            "--set-dscp-class",
            "EF",
        ])
        .status()
        .ok();

    Command::new("sudo")
        .args([
            "iptables",
            "-t",
            "mangle",
            "-A",
            "OUTPUT",
            "-p",
            "tcp",
            "--dport",
            "27015",
            "-j",
            "DSCP",
            "--set-dscp-class",
            "EF",
        ])
        .status()
        .ok();

    println!("✅ Counter-Strike 2 optimization completed!");
}

fn optimize_lol_ports() {
    println!("⚡ League of Legends Port Optimization");
    println!("=====================================\n");

    let lol_ports: Vec<(&str, &str, &str)> = vec![
        ("2099", "tcp", "Riot Services"),
        ("5223", "tcp", "Riot Chat"),
        ("8393:8400", "tcp", "Riot Patcher"),
        ("80", "tcp", "HTTP Updates"),
        ("443", "tcp", "HTTPS Services"),
        ("5000:5500", "udp", "Game Traffic"),
    ];

    for (port, protocol, service) in &lol_ports {
        let comment = format!("LoL {}", service);
        Command::new("sudo")
            .args([
                "iptables",
                "-I",
                "INPUT",
                "1",
                "-p",
                protocol,
                "--dport",
                port,
                "-j",
                "ACCEPT",
                "-m",
                "comment",
                "--comment",
                &comment,
            ])
            .status()
            .ok();
    }

    println!("✅ League of Legends optimization completed!");
}

fn optimize_rocket_league_ports() {
    println!("🚀 Rocket League Port Optimization");
    println!("==================================\n");

    let rl_ports: Vec<(&str, &str, &str)> = vec![
        ("7000:9000", "tcp", "Rocket League Servers"),
        ("7000:9000", "udp", "Rocket League Game Traffic"),
        ("80", "tcp", "HTTP Services"),
        ("443", "tcp", "HTTPS Services"),
    ];

    for (port, protocol, service) in &rl_ports {
        let comment = format!("RL {}", service);
        Command::new("sudo")
            .args([
                "iptables",
                "-I",
                "INPUT",
                "1",
                "-p",
                protocol,
                "--dport",
                port,
                "-j",
                "ACCEPT",
                "-m",
                "comment",
                "--comment",
                &comment,
            ])
            .status()
            .ok();
    }

    println!("✅ Rocket League optimization completed!");
}

fn optimize_fortnite_ports() {
    println!("👑 Fortnite Port Optimization");
    println!("=============================\n");

    let fortnite_ports: Vec<(&str, &str, &str)> = vec![
        ("80", "tcp", "HTTP Services"),
        ("443", "tcp", "HTTPS Services"),
        ("3478:3479", "udp", "Game Traffic"),
        ("5222", "tcp", "Epic Services"),
        ("13000:13050", "udp", "Game Servers"),
    ];

    for (port, protocol, service) in &fortnite_ports {
        let comment = format!("Fortnite {}", service);
        Command::new("sudo")
            .args([
                "iptables",
                "-I",
                "INPUT",
                "1",
                "-p",
                protocol,
                "--dport",
                port,
                "-j",
                "ACCEPT",
                "-m",
                "comment",
                "--comment",
                &comment,
            ])
            .status()
            .ok();
    }

    println!("✅ Fortnite optimization completed!");
}

fn optimize_valorant_ports() {
    println!("🎯 Valorant Port Optimization");
    println!("=============================\n");

    let valorant_ports: Vec<(&str, &str, &str)> = vec![
        ("80", "tcp", "HTTP Services"),
        ("443", "tcp", "HTTPS Services"),
        ("8080:8090", "tcp", "Riot Services"),
        ("2099", "tcp", "Riot Client"),
        ("5223", "tcp", "Riot Chat"),
        ("7000:8000", "udp", "Game Traffic"),
    ];

    for (port, protocol, service) in &valorant_ports {
        let comment = format!("Valorant {}", service);
        Command::new("sudo")
            .args([
                "iptables",
                "-I",
                "INPUT",
                "1",
                "-p",
                protocol,
                "--dport",
                port,
                "-j",
                "ACCEPT",
                "-m",
                "comment",
                "--comment",
                &comment,
            ])
            .status()
            .ok();
    }

    // Valorant anti-cheat specific rules
    Command::new("sudo")
        .args([
            "iptables",
            "-A",
            "INPUT",
            "-m",
            "state",
            "--state",
            "ESTABLISHED,RELATED",
            "-j",
            "ACCEPT",
        ])
        .status()
        .ok();

    Command::new("sudo")
        .args([
            "iptables",
            "-A",
            "OUTPUT",
            "-m",
            "state",
            "--state",
            "NEW,ESTABLISHED",
            "-j",
            "ACCEPT",
        ])
        .status()
        .ok();

    println!("✅ Valorant optimization completed!");
    println!("🛡️ Anti-cheat compatibility rules applied");
}

fn optimize_discord_gaming() {
    println!("🎲 Discord Gaming Optimization");
    println!("==============================\n");

    let discord_ports: Vec<(&str, &str, &str)> = vec![
        ("443", "tcp", "Discord HTTPS"),
        ("80", "tcp", "Discord HTTP"),
        ("50000:65535", "udp", "Discord Voice"),
        ("3478:3479", "udp", "Discord Voice backup"),
    ];

    for (port, protocol, service) in &discord_ports {
        let comment = format!("Discord {}", service);
        Command::new("sudo")
            .args([
                "iptables",
                "-I",
                "INPUT",
                "1",
                "-p",
                protocol,
                "--dport",
                port,
                "-j",
                "ACCEPT",
                "-m",
                "comment",
                "--comment",
                &comment,
            ])
            .status()
            .ok();
    }

    // Prioritize Discord voice traffic
    Command::new("sudo")
        .args([
            "iptables",
            "-t",
            "mangle",
            "-A",
            "OUTPUT",
            "-p",
            "udp",
            "--dport",
            "50000:65535",
            "-j",
            "DSCP",
            "--set-dscp-class",
            "EF",
        ])
        .status()
        .ok();

    Command::new("sudo")
        .args([
            "iptables",
            "-t",
            "mangle",
            "-A",
            "INPUT",
            "-p",
            "udp",
            "--sport",
            "50000:65535",
            "-j",
            "DSCP",
            "--set-dscp-class",
            "EF",
        ])
        .status()
        .ok();

    println!("✅ Discord gaming optimization completed!");
    println!("🎤 Voice traffic prioritized for low latency");
}

fn optimize_steam_gaming() {
    println!("🖥️ Steam Gaming Platform Optimization");
    println!("=====================================\n");

    let steam_ports: Vec<(&str, &str, &str)> = vec![
        ("27000:27100", "udp", "Steam Client"),
        ("27015:27030", "tcp", "Steam Downloads"),
        ("27015:27030", "udp", "Steam Servers"),
        ("4380", "tcp", "Steam Client Service"),
        ("26900", "tcp", "Steam Networking"),
        ("26900", "udp", "Steam Networking"),
        ("80", "tcp", "Steam Store"),
        ("443", "tcp", "Steam HTTPS"),
    ];

    for (port, protocol, service) in &steam_ports {
        let comment = format!("Steam {}", service);
        Command::new("sudo")
            .args([
                "iptables",
                "-I",
                "INPUT",
                "1",
                "-p",
                protocol,
                "--dport",
                port,
                "-j",
                "ACCEPT",
                "-m",
                "comment",
                "--comment",
                &comment,
            ])
            .status()
            .ok();
    }

    // Steam-specific optimizations
    Command::new("sudo")
        .args([
            "iptables",
            "-t",
            "mangle",
            "-A",
            "OUTPUT",
            "-p",
            "tcp",
            "--dport",
            "27015:27030",
            "-j",
            "DSCP",
            "--set-dscp-class",
            "AF41",
        ])
        .status()
        .ok();

    Command::new("sudo")
        .args([
            "iptables",
            "-t",
            "mangle",
            "-A",
            "OUTPUT",
            "-p",
            "udp",
            "--dport",
            "27000:27100",
            "-j",
            "DSCP",
            "--set-dscp-class",
            "AF41",
        ])
        .status()
        .ok();

    println!("✅ Steam gaming platform optimization completed!");
}

fn optimize_custom_game_ports() {
    println!("🎮 Custom Game Port Configuration");
    println!("================================\n");

    let port_range_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter port range (e.g., 7777:7784 or single port 25565)")
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    // Validate port range input
    let validated_port_range =
        match ValidatedPortRange::from_input(&port_range_input.replace('-', ":")) {
            Ok(p) => p.to_string(),
            Err(e) => {
                println!("❌ Invalid port range: {}", e);
                return;
            }
        };

    let protocol = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select protocol")
        .items(&["tcp", "udp", "both"])
        .default(2)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    let game_name_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter game name for comments")
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    // Validate game name (alphanumeric with spaces)
    if !game_name_input
        .chars()
        .all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '_')
    {
        println!("❌ Invalid game name: must be alphanumeric");
        return;
    }

    let priority = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Apply high priority QoS markings?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    println!("\n🔧 Configuring custom ports for {}...", game_name_input);

    match protocol {
        0 | 2 => {
            // TCP rules
            Command::new("sudo")
                .args([
                    "iptables",
                    "-I",
                    "INPUT",
                    "1",
                    "-p",
                    "tcp",
                    "--dport",
                    &validated_port_range,
                    "-j",
                    "ACCEPT",
                    "-m",
                    "comment",
                    "--comment",
                    &game_name_input,
                ])
                .status()
                .ok();

            if priority {
                Command::new("sudo")
                    .args([
                        "iptables",
                        "-t",
                        "mangle",
                        "-A",
                        "OUTPUT",
                        "-p",
                        "tcp",
                        "--dport",
                        &validated_port_range,
                        "-j",
                        "DSCP",
                        "--set-dscp-class",
                        "EF",
                    ])
                    .status()
                    .ok();
            }
        }
        _ => {}
    }

    if protocol == 1 || protocol == 2 {
        // UDP rules
        Command::new("sudo")
            .args([
                "iptables",
                "-I",
                "INPUT",
                "1",
                "-p",
                "udp",
                "--dport",
                &validated_port_range,
                "-j",
                "ACCEPT",
                "-m",
                "comment",
                "--comment",
                &game_name_input,
            ])
            .status()
            .ok();

        if priority {
            Command::new("sudo")
                .args([
                    "iptables",
                    "-t",
                    "mangle",
                    "-A",
                    "OUTPUT",
                    "-p",
                    "udp",
                    "--dport",
                    &validated_port_range,
                    "-j",
                    "DSCP",
                    "--set-dscp-class",
                    "EF",
                ])
                .status()
                .ok();
        }
    }

    println!("✅ Custom game port configuration completed!");
    if priority {
        println!("🚀 High-priority QoS markings applied");
    }
}

fn anticheat_firewall_rules() {
    println!("🎯 Anti-cheat Firewall Rules");
    println!("============================\n");

    let anticheat_systems = [
        "🛡️ All Anti-cheat Systems (Recommended)",
        "⚔️ EasyAntiCheat (EAC)",
        "🛡️ BattlEye",
        "🔒 Vanguard (Valorant)",
        "⚡ FairFight",
        "🚀 VAC (Steam)",
        "🎮 Custom Anti-cheat Rules",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select anti-cheat system to configure")
        .items(&anticheat_systems)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => configure_all_anticheat_rules(),
        1 => configure_eac_rules(),
        2 => configure_battleye_rules(),
        3 => configure_vanguard_rules(),
        4 => configure_fairfight_rules(),
        5 => configure_vac_rules(),
        6 => configure_custom_anticheat(),
        _ => {}
    }
}

fn configure_all_anticheat_rules() {
    println!("🛡️ Configuring Universal Anti-cheat Rules");
    println!("==========================================\n");

    println!("🔧 Applying anti-cheat friendly firewall configuration...");

    use super::safe_commands::{
        firewalld_add_port_simple, firewalld_add_service_simple, firewalld_reload,
        iptables_allow_established, iptables_allow_input, iptables_allow_input_sport,
        iptables_allow_loopback, iptables_allow_output_conntrack, iptables_allow_output_port,
        ufw_allow_in, ufw_allow_out,
    };

    // Allow established connections
    println!("  ⚡ Applying conntrack rules...");
    if let Err(e) = iptables_allow_established() {
        eprintln!("    Failed: {}", e);
    }
    if let Err(e) = iptables_allow_output_conntrack() {
        eprintln!("    Failed: {}", e);
    }

    // Allow loopback (essential for anti-cheat)
    println!("  ⚡ Applying loopback rules...");
    if let Err(e) = iptables_allow_loopback() {
        eprintln!("    Failed: {}", e);
    }

    // Anti-cheat communication ports
    let input_rules = [
        (80, "tcp", "Anti-cheat HTTP"),
        (443, "tcp", "Anti-cheat HTTPS"),
        (6672, "tcp", "Anti-cheat Services"),
    ];
    for (port, proto, comment) in &input_rules {
        println!("  ⚡ Applying rule: {} port {}", comment, port);
        if let Err(e) = iptables_allow_input(*port, proto, comment) {
            eprintln!("    Failed: {}", e);
        }
    }

    // DNS for anti-cheat lookups
    println!("  ⚡ Applying DNS rules...");
    for proto in ["udp", "tcp"] {
        if let Err(e) = iptables_allow_output_port(53, proto) {
            eprintln!("    Failed output {}: {}", proto, e);
        }
        if let Err(e) = iptables_allow_input_sport(53, proto) {
            eprintln!("    Failed input {}: {}", proto, e);
        }
    }

    // NTP for time synchronization (critical for anti-cheat)
    println!("  ⚡ Applying NTP rules...");
    if let Err(e) = iptables_allow_output_port(123, "udp") {
        eprintln!("    Failed: {}", e);
    }
    if let Err(e) = iptables_allow_input_sport(123, "udp") {
        eprintln!("    Failed: {}", e);
    }

    // UFW rules for anti-cheat
    let ufw_out_rules = [
        (80, Some("tcp"), "Anti-cheat HTTP"),
        (443, Some("tcp"), "Anti-cheat HTTPS"),
        (53, None, "Anti-cheat DNS"),
        (123, Some("udp"), "Anti-cheat NTP"),
    ];
    for (port, proto, comment) in &ufw_out_rules {
        if let Err(e) = ufw_allow_out(*port, *proto, comment) {
            // UFW might not be installed, don't spam errors
            log::debug!("UFW rule failed: {}", e);
        }
    }
    if let Err(e) = ufw_allow_in(6672, Some("tcp"), "Anti-cheat Services") {
        log::debug!("UFW rule failed: {}", e);
    }

    // Firewalld anti-cheat configuration
    for service in ["http", "https", "dns", "ntp"] {
        if let Err(e) = firewalld_add_service_simple(service) {
            log::debug!("firewalld service failed: {}", e);
        }
    }
    if let Err(e) = firewalld_add_port_simple(6672, "tcp") {
        log::debug!("firewalld port failed: {}", e);
    }
    let _ = firewalld_reload();

    println!("\n✅ Universal anti-cheat firewall rules configured!");
    println!("🛡️ Compatible with: EAC, BattlEye, Vanguard, VAC, FairFight");
    println!("⚠️ Note: Some anti-cheat systems may require additional game-specific rules");
}

fn configure_eac_rules() {
    println!("⚔️ EasyAntiCheat (EAC) Firewall Configuration");
    println!("=============================================\n");

    use super::safe_commands::{iptables_allow_established, iptables_allow_output};

    let rules = [
        (6672, "tcp", "EAC Service"),
        (443, "tcp", "EAC HTTPS"),
        (80, "tcp", "EAC HTTP"),
    ];

    for (port, proto, comment) in &rules {
        if let Err(e) = iptables_allow_output(*port, proto, comment) {
            eprintln!("Failed to add rule for {}: {}", comment, e);
        }
    }

    if let Err(e) = iptables_allow_established() {
        eprintln!("Failed to add conntrack rule: {}", e);
    }

    println!("✅ EasyAntiCheat firewall rules configured!");
}

fn configure_battleye_rules() {
    println!("🛡️ BattlEye Firewall Configuration");
    println!("=================================\n");

    use super::safe_commands::{iptables_allow_established, iptables_allow_output};

    let rules = [
        (80, "tcp", "BattlEye HTTP"),
        (443, "tcp", "BattlEye HTTPS"),
        (2344, "tcp", "BattlEye Service"),
    ];

    for (port, proto, comment) in &rules {
        if let Err(e) = iptables_allow_output(*port, proto, comment) {
            eprintln!("Failed to add rule for {}: {}", comment, e);
        }
    }

    if let Err(e) = iptables_allow_established() {
        eprintln!("Failed to add conntrack rule: {}", e);
    }

    println!("✅ BattlEye firewall rules configured!");
}

fn configure_vanguard_rules() {
    println!("🔒 Vanguard (Valorant) Firewall Configuration");
    println!("=============================================\n");

    use super::safe_commands::{
        iptables_allow_established, iptables_allow_output, iptables_drop_invalid,
    };

    let rules = [
        (443, "tcp", "Vanguard HTTPS"),
        (80, "tcp", "Vanguard HTTP"),
        (2099, "tcp", "Riot Services"),
    ];

    for (port, proto, comment) in &rules {
        if let Err(e) = iptables_allow_output(*port, proto, comment) {
            eprintln!("Failed to add rule for {}: {}", comment, e);
        }
    }

    if let Err(e) = iptables_allow_established() {
        eprintln!("Failed to add conntrack rule: {}", e);
    }

    // Vanguard requires very strict connection tracking
    if let Err(e) = iptables_drop_invalid() {
        eprintln!("Failed to add drop invalid rule: {}", e);
    }

    println!("✅ Vanguard anti-cheat firewall rules configured!");
    println!(
        "⚠️ Note: Vanguard requires kernel-level access and may conflict with some firewall configurations"
    );
}

fn configure_fairfight_rules() {
    println!("⚡ FairFight Firewall Configuration");
    println!("=================================\n");

    use super::safe_commands::{iptables_allow_established, iptables_allow_output};

    let rules = [
        (443, "tcp", "FairFight HTTPS"),
        (80, "tcp", "FairFight HTTP"),
    ];

    for (port, proto, comment) in &rules {
        if let Err(e) = iptables_allow_output(*port, proto, comment) {
            eprintln!("Failed to add rule for {}: {}", comment, e);
        }
    }

    if let Err(e) = iptables_allow_established() {
        eprintln!("Failed to add conntrack rule: {}", e);
    }

    println!("✅ FairFight firewall rules configured!");
}

fn configure_vac_rules() {
    println!("🚀 VAC (Steam) Firewall Configuration");
    println!("====================================\n");

    use super::safe_commands::{iptables_allow_established, iptables_allow_output};

    let rules = [
        (27030, "tcp", "VAC Steam"),
        (443, "tcp", "VAC HTTPS"),
        (80, "tcp", "VAC HTTP"),
    ];

    for (port, proto, comment) in &rules {
        if let Err(e) = iptables_allow_output(*port, proto, comment) {
            eprintln!("Failed to add rule for {}: {}", comment, e);
        }
    }

    if let Err(e) = iptables_allow_established() {
        eprintln!("Failed to add conntrack rule: {}", e);
    }

    println!("✅ VAC (Steam) firewall rules configured!");
}

fn configure_custom_anticheat() {
    println!("🎮 Custom Anti-cheat Configuration");
    println!("=================================\n");

    use super::safe_commands::iptables_allow_output;

    let service_name: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter anti-cheat service name")
        .interact()
    {
        Ok(name) => name,
        Err(_) => return,
    };

    // Validate service name (alphanumeric, spaces, hyphens only)
    if !service_name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == ' ' || c == '-' || c == '_')
    {
        println!("❌ Invalid service name. Use alphanumeric characters, spaces, hyphens only.");
        return;
    }

    let ports: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter ports (comma-separated, e.g., 80,443,6672)")
        .interact()
    {
        Ok(ports) => ports,
        Err(_) => return,
    };

    let protocols = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select protocols")
        .items(&["TCP", "UDP"])
        .interact()
    {
        Ok(protos) => protos,
        Err(_) => return,
    };

    println!(
        "\n🔧 Configuring custom anti-cheat rules for {}...",
        service_name
    );

    for port_str in ports.split(',') {
        let port_str = port_str.trim();

        // Parse and validate port
        let port: u16 = match port_str.parse() {
            Ok(p) if p > 0 => p,
            _ => {
                eprintln!("  ❌ Invalid port: {}", port_str);
                continue;
            }
        };

        for &protocol_idx in &protocols {
            let protocol = if protocol_idx == 0 { "tcp" } else { "udp" };
            let comment = format!("{} {}", service_name, protocol.to_uppercase());

            println!(
                "  ⚡ Adding rule for {} port {}",
                protocol.to_uppercase(),
                port
            );

            if let Err(e) = iptables_allow_output(port, protocol, &comment) {
                eprintln!("    Failed: {}", e);
            }
        }
    }

    println!("✅ Custom anti-cheat rules configured for {}", service_name);
}

fn network_latency_optimization() {
    println!("🌐 Network Latency Optimization");
    println!("===============================\n");

    let options = [
        "⚡ TCP/UDP Optimization",
        "🚀 Kernel Network Tuning",
        "🎯 Gaming QoS Configuration",
        "📡 DNS Optimization",
        "🔧 Network Interface Tuning",
        "📊 Latency Testing & Analysis",
        "⬅️ Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🌐 Network Latency Optimization")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => tcp_udp_optimization(),
        1 => kernel_network_tuning(),
        2 => gaming_qos_configuration(),
        3 => dns_optimization(),
        4 => network_interface_tuning(),
        5 => latency_testing_analysis(),
        _ => {}
    }
}

fn tcp_udp_optimization() {
    println!("⚡ TCP/UDP Optimization for Gaming");
    println!("=================================\n");

    println!("🔧 Applying TCP optimizations for reduced latency...");

    use super::safe_commands::{sudo_write_file, sysctl_set};

    let tcp_optimizations = vec![
        ("net.ipv4.tcp_timestamps", "1"),
        ("net.ipv4.tcp_window_scaling", "1"),
        ("net.ipv4.tcp_sack", "1"),
        ("net.ipv4.tcp_fack", "1"),
        ("net.ipv4.tcp_low_latency", "1"),
        ("net.ipv4.tcp_congestion_control", "bbr"),
        ("net.core.default_qdisc", "fq"),
        ("net.ipv4.tcp_fastopen", "3"),
        ("net.ipv4.tcp_slow_start_after_idle", "0"),
        ("net.ipv4.tcp_tw_reuse", "1"),
        ("net.ipv4.tcp_fin_timeout", "15"),
        ("net.ipv4.ip_local_port_range", "1024 65535"),
        ("net.core.rmem_default", "262144"),
        ("net.core.rmem_max", "16777216"),
        ("net.core.wmem_default", "262144"),
        ("net.core.wmem_max", "16777216"),
        ("net.core.netdev_max_backlog", "5000"),
        ("net.ipv4.tcp_rmem", "4096 65536 16777216"),
        ("net.ipv4.tcp_wmem", "4096 65536 16777216"),
    ];

    for (parameter, value) in &tcp_optimizations {
        println!("  ⚡ Setting {} = {}", parameter, value);
        if let Err(e) = sysctl_set(parameter, value) {
            eprintln!("    Failed: {}", e);
        }
    }

    // Make changes persistent
    let sysctl_config = "/etc/sysctl.d/99-gaming-network.conf";
    let mut config_content = String::from("# Gaming Network Optimizations\n");

    for (parameter, value) in &tcp_optimizations {
        config_content.push_str(&format!("{}={}\n", parameter, value));
    }

    if let Err(e) = sudo_write_file(sysctl_config, &config_content) {
        eprintln!("Failed to write config: {}", e);
    }

    println!("\n🚀 UDP optimizations for real-time gaming...");

    let udp_optimizations = vec![
        ("net.core.rmem_default", "262144"),
        ("net.core.rmem_max", "16777216"),
        ("net.core.wmem_default", "262144"),
        ("net.core.wmem_max", "16777216"),
        ("net.ipv4.udp_mem", "102400 873800 16777216"),
        ("net.ipv4.udp_rmem_min", "8192"),
        ("net.ipv4.udp_wmem_min", "8192"),
    ];

    for (parameter, value) in &udp_optimizations {
        println!("  ⚡ Setting {} = {}", parameter, value);
        if let Err(e) = sysctl_set(parameter, value) {
            eprintln!("    Failed: {}", e);
        }
    }

    println!("\n✅ TCP/UDP optimization completed!");
    println!("🎮 Network stack optimized for gaming performance");
    println!("💾 Settings saved to {}", sysctl_config);
}

fn kernel_network_tuning() {
    println!("🚀 Kernel Network Tuning for Gaming");
    println!("===================================\n");

    println!("🔧 Applying kernel-level network optimizations...");

    let kernel_optimizations = vec![
        // Network buffer optimizations
        ("net.core.netdev_max_backlog", "30000"),
        ("net.core.netdev_budget", "600"),
        ("net.core.netdev_budget_usecs", "5000"),
        // Interrupt handling
        ("net.core.dev_weight", "64"),
        // Memory pressure handling
        ("vm.min_free_kbytes", "65536"),
        ("vm.swappiness", "1"),
        // Network stack tuning
        ("net.ipv4.neigh.default.gc_thresh1", "1024"),
        ("net.ipv4.neigh.default.gc_thresh2", "2048"),
        ("net.ipv4.neigh.default.gc_thresh3", "4096"),
        ("net.netfilter.nf_conntrack_max", "1000000"),
        ("net.netfilter.nf_conntrack_tcp_timeout_established", "1800"),
        // Gaming-specific optimizations
        ("net.ipv4.tcp_mtu_probing", "1"),
        ("net.ipv4.tcp_base_mss", "1024"),
        ("net.ipv4.route.flush", "1"),
    ];

    use super::safe_commands::{sudo_write_file, sysctl_set};

    for (parameter, value) in &kernel_optimizations {
        println!("  🔧 {}: {}", parameter, value);
        if let Err(e) = sysctl_set(parameter, value) {
            eprintln!("    Failed: {}", e);
        }
    }

    // IRQ affinity optimization
    println!("\n⚡ Optimizing IRQ affinity for network interfaces...");

    if let Ok(interrupts) = std::fs::read_to_string("/proc/interrupts") {
        for line in interrupts.lines() {
            if (line.contains("eth") || line.contains("enp") || line.contains("wlan"))
                && let Some(irq) = line.split_whitespace().next()
                && let Ok(irq_num) = irq.replace(":", "").parse::<u32>()
            {
                // Set IRQ affinity to CPU 0 for consistent latency
                let affinity_path = format!("/proc/irq/{}/smp_affinity", irq_num);
                if let Err(e) = sudo_write_file(&affinity_path, "1") {
                    eprintln!("  Failed to set IRQ {} affinity: {}", irq_num, e);
                } else {
                    println!("  📍 Set IRQ {} affinity to CPU 0", irq_num);
                }
            }
        }
    }

    println!("\n✅ Kernel network tuning completed!");
    println!("🎮 System optimized for minimum network latency");
}

fn gaming_qos_configuration() {
    println!("🎯 Gaming QoS Configuration");
    println!("===========================\n");

    println!("🔧 Setting up Quality of Service for gaming traffic...");

    // Check if tc (traffic control) is available
    let tc_check = Command::new("which").arg("tc").status();

    if tc_check.is_err() || !tc_check.as_ref().map(|s| s.success()).unwrap_or(false) {
        println!("⚠️ tc (traffic control) not found. Installing...");
        Command::new("sudo")
            .args(["apt", "install", "iproute2"])
            .status()
            .ok();
        Command::new("sudo")
            .args(["pacman", "-S", "iproute2"])
            .status()
            .ok();
        Command::new("sudo")
            .args(["dnf", "install", "iproute"])
            .status()
            .ok();
    }

    // Get primary network interface
    let interface_result = Command::new("ip")
        .args(["route", "get", "8.8.8.8"])
        .output();

    let interface = if let Ok(out) = interface_result {
        let output = String::from_utf8_lossy(&out.stdout);
        let mut found_interface = "eth0".to_string();
        for word in output.split_whitespace() {
            if word.starts_with("dev") {
                continue;
            }
            if word.contains("eth") || word.contains("enp") || word.contains("wlan") {
                found_interface = word.to_string();
                break;
            }
        }
        found_interface
    } else {
        "eth0".to_string()
    };

    // Validate interface name
    if safe_commands::validate_interface_name(&interface).is_err() {
        println!("❌ Invalid interface name detected");
        return;
    }

    println!("🌐 Configuring QoS for interface: {}", interface);

    // Remove existing qdisc (ignore errors)
    let _ = safe_commands::tc_del_qdisc(&interface, "root");

    // Add root qdisc with HTB
    if let Err(e) = safe_commands::tc_add_qdisc(
        &interface,
        &["root", "handle", "1:", "htb", "default", "30"],
    ) {
        println!("⚠️ Failed to add root qdisc: {}", e);
    }

    // Create classes for different traffic types
    // Root class
    if let Err(e) = safe_commands::tc_add_class(
        &interface,
        &["parent", "1:", "classid", "1:1", "htb", "rate", "1000mbit"],
    ) {
        println!("⚠️ Failed to add root class: {}", e);
    }

    // Gaming class (high priority)
    if let Err(e) = safe_commands::tc_add_class(
        &interface,
        &[
            "parent", "1:1", "classid", "1:10", "htb", "rate", "800mbit", "ceil", "1000mbit",
            "prio", "1",
        ],
    ) {
        println!("⚠️ Failed to add gaming class: {}", e);
    }

    // Voice class (medium priority)
    if let Err(e) = safe_commands::tc_add_class(
        &interface,
        &[
            "parent", "1:1", "classid", "1:20", "htb", "rate", "150mbit", "ceil", "300mbit",
            "prio", "2",
        ],
    ) {
        println!("⚠️ Failed to add voice class: {}", e);
    }

    // Default class (low priority)
    if let Err(e) = safe_commands::tc_add_class(
        &interface,
        &[
            "parent", "1:1", "classid", "1:30", "htb", "rate", "50mbit", "ceil", "200mbit", "prio",
            "3",
        ],
    ) {
        println!("⚠️ Failed to add default class: {}", e);
    }

    // Add SFQ qdiscs for fairness
    let _ = safe_commands::tc_add_qdisc(
        &interface,
        &["parent", "1:10", "handle", "10:", "sfq", "perturb", "10"],
    );
    let _ = safe_commands::tc_add_qdisc(
        &interface,
        &["parent", "1:20", "handle", "20:", "sfq", "perturb", "10"],
    );
    let _ = safe_commands::tc_add_qdisc(
        &interface,
        &["parent", "1:30", "handle", "30:", "sfq", "perturb", "10"],
    );

    // Add filters for gaming traffic
    // WoW ports
    let _ = safe_commands::tc_add_filter(
        &interface,
        &[
            "protocol", "ip", "parent", "1:0", "prio", "1", "u32", "match", "ip", "dport", "3724",
            "0xffff", "flowid", "1:10",
        ],
    );
    let _ = safe_commands::tc_add_filter(
        &interface,
        &[
            "protocol", "ip", "parent", "1:0", "prio", "1", "u32", "match", "ip", "dport", "1119",
            "0xffff", "flowid", "1:10",
        ],
    );

    // Steam ports
    let _ = safe_commands::tc_add_filter(
        &interface,
        &[
            "protocol", "ip", "parent", "1:0", "prio", "1", "u32", "match", "ip", "dport", "27015",
            "0xffff", "flowid", "1:10",
        ],
    );
    let _ = safe_commands::tc_add_filter(
        &interface,
        &[
            "protocol", "ip", "parent", "1:0", "prio", "1", "u32", "match", "ip", "dport", "27030",
            "0xffff", "flowid", "1:10",
        ],
    );

    // Discord voice (high priority)
    let _ = safe_commands::tc_add_filter(
        &interface,
        &[
            "protocol", "ip", "parent", "1:0", "prio", "1", "u32", "match", "ip", "sport", "50000",
            "0xc000", "flowid", "1:20",
        ],
    );

    // DSCP marking: EF for gaming
    let _ = safe_commands::tc_add_filter(
        &interface,
        &[
            "protocol", "ip", "parent", "1:0", "prio", "1", "u32", "match", "ip", "tos", "0xb8",
            "0xfc", "flowid", "1:10",
        ],
    );

    // DSCP marking: AF41 for voice
    let _ = safe_commands::tc_add_filter(
        &interface,
        &[
            "protocol", "ip", "parent", "1:0", "prio", "2", "u32", "match", "ip", "tos", "0x88",
            "0xfc", "flowid", "1:20",
        ],
    );

    println!("\n🎮 Gaming traffic prioritization configured!");
    println!("📊 QoS classes created:");
    println!("  🎯 Class 1:10 - Gaming traffic (high priority)");
    println!("  🎤 Class 1:20 - Voice traffic (medium priority)");
    println!("  📡 Class 1:30 - Default traffic (low priority)");

    // Show QoS status
    println!("\n📋 Current QoS configuration:");
    match safe_commands::tc_show_class(&interface) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("⚠️ Failed to show QoS status: {}", e),
    }
}

fn dns_optimization() {
    println!("📡 DNS Optimization for Gaming");
    println!("==============================\n");

    let dns_options = [
        "🚀 Configure Gaming DNS Servers",
        "🔧 Setup DNS Caching",
        "📊 DNS Performance Test",
        "🛠️ Custom DNS Configuration",
        "⬅️ Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("DNS Optimization Options")
        .items(&dns_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => configure_gaming_dns(),
        1 => setup_dns_caching(),
        2 => dns_performance_test(),
        3 => custom_dns_configuration(),
        _ => {}
    }
}

fn configure_gaming_dns() {
    println!("🚀 Configuring Gaming DNS Servers");
    println!("=================================\n");

    let dns_providers = [
        "🎮 Cloudflare Gaming (1.1.1.1)",
        "🚀 Quad9 (9.9.9.9)",
        "🔧 Google DNS (8.8.8.8)",
        "📡 OpenDNS (208.67.222.222)",
        "⚡ Custom DNS Servers",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select DNS provider")
        .items(&dns_providers)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    let (primary_dns, secondary_dns): (String, String) = match choice {
        0 => ("1.1.1.1".to_string(), "1.0.0.1".to_string()),
        1 => ("9.9.9.9".to_string(), "149.112.112.112".to_string()),
        2 => ("8.8.8.8".to_string(), "8.8.4.4".to_string()),
        3 => ("208.67.222.222".to_string(), "208.67.220.220".to_string()),
        4 => {
            let primary: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter primary DNS server")
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };
            let secondary: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter secondary DNS server")
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };
            (primary, secondary)
        }
        _ => ("1.1.1.1".to_string(), "1.0.0.1".to_string()),
    };

    println!(
        "\n🔧 Configuring DNS servers: {} and {}",
        primary_dns, secondary_dns
    );

    // Update resolv.conf
    let resolv_content = format!(
        "# Gaming DNS Configuration\nnameserver {}\nnameserver {}\noptions timeout:1\noptions attempts:2\noptions rotate\n",
        primary_dns, secondary_dns
    );

    if let Err(e) = safe_commands::sudo_write_file("/etc/resolv.conf", &resolv_content) {
        println!("⚠️ Failed to update resolv.conf: {}", e);
    }

    // NetworkManager configuration
    let nm_conf = format!(
        "[main]\ndns=none\n\n[global-dns-domain-*]\nservers={},{}\n",
        primary_dns, secondary_dns
    );

    if let Err(e) =
        safe_commands::sudo_write_file("/etc/NetworkManager/conf.d/gaming-dns.conf", &nm_conf)
    {
        println!("⚠️ Failed to update NetworkManager config: {}", e);
    }

    // Restart NetworkManager
    Command::new("sudo")
        .args(["systemctl", "restart", "NetworkManager"])
        .status()
        .ok();

    println!("✅ Gaming DNS servers configured!");
    println!("🎮 Optimized for low-latency DNS resolution");
}

fn setup_dns_caching() {
    println!("🔧 Setting up DNS Caching");
    println!("=========================\n");

    // Check if systemd-resolved is available
    let resolved_check = Command::new("systemctl")
        .args(&["is-active", "systemd-resolved"])
        .status();

    if resolved_check
        .as_ref()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        println!("📡 Configuring systemd-resolved for DNS caching...");

        let resolved_conf = r#"[Resolve]
DNS=1.1.1.1 1.0.0.1
FallbackDNS=8.8.8.8 8.8.4.4
Domains=~.
DNSSEC=yes
DNSOverTLS=opportunistic
Cache=yes
DNSStubListener=yes
ReadEtcHosts=yes
"#;

        if let Err(e) = safe_commands::sudo_write_file("/etc/systemd/resolved.conf", resolved_conf)
        {
            println!("⚠️ Failed to update resolved.conf: {}", e);
        }

        Command::new("sudo")
            .args(&["systemctl", "restart", "systemd-resolved"])
            .status()
            .ok();

        println!("✅ systemd-resolved DNS caching configured!");
    } else {
        // Install and configure dnsmasq
        println!("📦 Installing dnsmasq for DNS caching...");

        Command::new("sudo")
            .args(&["apt", "install", "dnsmasq"])
            .status()
            .ok();
        Command::new("sudo")
            .args(&["pacman", "-S", "dnsmasq"])
            .status()
            .ok();
        Command::new("sudo")
            .args(&["dnf", "install", "dnsmasq"])
            .status()
            .ok();

        let dnsmasq_conf = r#"# Gaming DNS Cache Configuration
cache-size=10000
no-resolv
server=1.1.1.1
server=1.0.0.1
server=8.8.8.8
server=8.8.4.4
listen-address=127.0.0.1
bind-interfaces
no-poll
no-negcache
dns-forward-max=1000
"#;

        if let Err(e) = safe_commands::sudo_write_file("/etc/dnsmasq.conf", dnsmasq_conf) {
            println!("⚠️ Failed to update dnsmasq.conf: {}", e);
        }

        Command::new("sudo")
            .args(&["systemctl", "enable", "dnsmasq"])
            .status()
            .ok();
        Command::new("sudo")
            .args(&["systemctl", "start", "dnsmasq"])
            .status()
            .ok();

        println!("✅ dnsmasq DNS caching configured!");
    }

    println!("🚀 DNS caching enabled for faster game server resolution");
}

fn dns_performance_test() {
    println!("📊 DNS Performance Test");
    println!("=======================\n");

    let test_domains = vec![
        "worldofwarcraft.com",
        "battle.net",
        "steampowered.com",
        "discordapp.com",
        "epicgames.com",
        "riotgames.com",
        "ea.com",
        "ubisoft.com",
    ];

    let dns_servers = vec![
        ("Cloudflare", "1.1.1.1"),
        ("Google", "8.8.8.8"),
        ("Quad9", "9.9.9.9"),
        ("OpenDNS", "208.67.222.222"),
    ];

    println!("🔍 Testing DNS resolution speed for gaming domains...\n");

    for (provider, server) in &dns_servers {
        println!("📡 Testing {} ({}):", provider, server);

        let mut total_time = 0.0;
        let mut successful_queries = 0;

        for domain in &test_domains {
            let start_time = std::time::Instant::now();

            let result = Command::new("dig")
                .args(&[
                    &format!("@{}", server),
                    &**domain,
                    "+short",
                    "+time=2",
                    "+tries=1",
                ])
                .output();

            let elapsed = start_time.elapsed().as_millis() as f64;

            match result {
                Ok(out) if out.status.success() && !out.stdout.is_empty() => {
                    println!("  ✅ {}: {:.0}ms", domain, elapsed);
                    total_time += elapsed;
                    successful_queries += 1;
                }
                _ => {
                    println!("  ❌ {}: timeout/error", domain);
                }
            }
        }

        if successful_queries > 0 {
            let avg_time = total_time / successful_queries as f64;
            println!(
                "  📊 Average: {:.1}ms ({}/{} successful)\n",
                avg_time,
                successful_queries,
                test_domains.len()
            );
        } else {
            println!("  📊 Average: N/A (no successful queries)\n");
        }
    }

    println!("💡 Recommendation: Choose the DNS provider with the lowest average latency");
}

fn custom_dns_configuration() {
    println!("🛠️ Custom DNS Configuration");
    println!("===========================\n");

    println!("🔧 Advanced DNS settings for gaming optimization:");

    let enable_ipv6 = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable IPv6 DNS resolution?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    let enable_dnssec = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable DNSSEC validation?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    let cache_size: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("DNS cache size (entries)")
        .default("10000".to_string())
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    let timeout: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("DNS query timeout (seconds)")
        .default("2".to_string())
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    println!("\n🔧 Applying custom DNS configuration...");

    // Apply custom systemd-resolved settings
    let mut resolved_conf = String::from("[Resolve]\n");
    resolved_conf.push_str("DNS=1.1.1.1 1.0.0.1\n");

    if enable_ipv6 {
        resolved_conf.push_str("DNS=2606:4700:4700::1111 2606:4700:4700::1001\n");
    }

    resolved_conf.push_str(&format!(
        "DNSSEC={}\n",
        if enable_dnssec { "yes" } else { "no" }
    ));
    resolved_conf.push_str("Cache=yes\n");
    resolved_conf.push_str("DNSStubListener=yes\n");

    if let Err(e) = safe_commands::sudo_write_file("/etc/systemd/resolved.conf", &resolved_conf) {
        println!("⚠️ Failed to update resolved.conf: {}", e);
    }

    // Apply kernel DNS settings using safe sysctl helper
    let dns_sysctls = [
        ("net.ipv4.ip_local_reserved_ports", "53"),
        ("net.core.busy_poll", "50"),
        ("net.core.busy_read", "50"),
    ];

    for (param, value) in &dns_sysctls {
        if let Err(e) = safe_commands::sysctl_set(param, value) {
            println!("⚠️ Failed to set {}: {}", param, e);
        }
    }

    Command::new("sudo")
        .args(["systemctl", "restart", "systemd-resolved"])
        .status()
        .ok();

    println!("✅ Custom DNS configuration applied!");
    println!("🎮 Optimized for gaming performance");
}

fn network_interface_tuning() {
    println!("🔧 Network Interface Tuning");
    println!("============================\n");

    // Get network interfaces
    let interfaces_result = Command::new("ls").arg("/sys/class/net").output();

    let interfaces = if let Ok(out) = interfaces_result {
        String::from_utf8_lossy(&out.stdout)
            .lines()
            .filter(|line| !line.contains("lo") && !line.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    } else {
        vec!["eth0".to_string()]
    };

    if interfaces.is_empty() {
        println!("❌ No network interfaces found");
        return;
    }

    let interface = if interfaces.len() == 1 {
        interfaces[0].to_string()
    } else {
        let choice = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select network interface to optimize")
            .items(&interfaces)
            .default(0)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            Ok(None) | Err(_) => return,
        };
        interfaces[choice].to_string()
    };

    println!("🌐 Optimizing network interface: {}", interface);

    // Validate interface name
    if safe_commands::validate_interface_name(&interface).is_err() {
        println!("❌ Invalid interface name");
        return;
    }

    println!("🔧 Applying interface optimizations...");

    // Buffer sizes
    if let Err(e) = safe_commands::ethtool_set_ring(&interface, 4096, 4096) {
        log::debug!("Ring buffer adjustment: {}", e);
    }

    // Interrupt coalescing for gaming
    if let Err(e) = safe_commands::ethtool_set_coalesce(&interface, 10, 10) {
        log::debug!("Coalescing adjustment: {}", e);
    }

    // Disable features that add latency
    if let Err(e) = safe_commands::ethtool_disable_offloads(&interface) {
        log::debug!("Offload disable: {}", e);
    }

    // Enable features that improve performance
    if let Err(e) = safe_commands::ethtool_enable_features(&interface) {
        log::debug!("Feature enable: {}", e);
    }

    // CPU affinity for network interrupts
    println!("📍 Setting CPU affinity for network interrupts...");

    match safe_commands::get_interface_irqs(&interface) {
        Ok(irqs) => {
            for irq in irqs {
                if let Err(e) = safe_commands::set_irq_affinity(irq, "2") {
                    log::debug!("IRQ {} affinity: {}", irq, e);
                } else {
                    println!("  ⚡ Set IRQ {} to CPU 1", irq);
                }
            }
        }
        Err(e) => {
            log::debug!("Failed to get IRQs: {}", e);
        }
    }

    // Interface queue optimizations using sysfs
    println!("📊 Optimizing interface queues...");

    // Note: Queue optimization for multiple queues requires glob expansion
    // which is filesystem-level, not shell. For safety, we attempt known paths.
    for i in 0..8 {
        let xps_path = format!("/sys/class/net/{}/queues/tx-{}/xps_cpus", interface, i);
        let rps_path = format!("/sys/class/net/{}/queues/rx-{}/rps_cpus", interface, i);

        // These may fail if queue doesn't exist - that's OK
        let _ = safe_commands::sysfs_write(&xps_path, "ff");
        let _ = safe_commands::sysfs_write(&rps_path, "2");
    }

    println!("\n✅ Network interface optimization completed!");
    println!(
        "🎮 Interface {} optimized for gaming performance",
        interface
    );

    // Show current settings
    println!("\n📋 Current interface settings:");
    match safe_commands::ethtool_show(&interface) {
        Ok(output) => {
            for line in output.lines().take(20) {
                println!("{}", line);
            }
        }
        Err(e) => println!("⚠️ Failed to show settings: {}", e),
    }
}

fn latency_testing_analysis() {
    println!("📊 Latency Testing & Analysis");
    println!("=============================\n");

    let test_options = [
        "🎯 Gaming Server Latency Test",
        "🌐 DNS Resolution Speed Test",
        "📡 Network Interface Analysis",
        "⚡ Real-time Latency Monitoring",
        "🔍 Packet Loss Analysis",
        "⬅️ Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select latency test")
        .items(&test_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => gaming_server_latency_test(),
        1 => dns_resolution_speed_test(),
        2 => network_interface_analysis(),
        3 => realtime_latency_monitoring(),
        4 => packet_loss_analysis(),
        _ => {}
    }
}

fn gaming_server_latency_test() {
    println!("🎯 Gaming Server Latency Test");
    println!("=============================\n");

    let gaming_servers = vec![
        ("WoW US East", "us.battle.net", "3724"),
        ("WoW US West", "us.battle.net", "3724"),
        ("WoW EU", "eu.battle.net", "3724"),
        ("Steam US", "steamcommunity.com", "27015"),
        ("Discord", "discord.com", "443"),
        ("Riot Games", "riot-geo.pas.si.riotgames.com", "443"),
        ("Epic Games", "epicgames.com", "443"),
        ("Valve", "valvesoftware.com", "27015"),
    ];

    println!("🔍 Testing latency to popular gaming servers...\n");

    for (server_name, hostname, port) in &gaming_servers {
        println!("📡 Testing {} ({}):", server_name, hostname);

        // ICMP ping test
        let ping_result = Command::new("ping")
            .args(&["-c", "4", "-W", "2", hostname])
            .output();

        if let Ok(out) = ping_result {
            let output = String::from_utf8_lossy(&out.stdout);
            for line in output.lines() {
                if line.contains("min/avg/max") {
                    println!(
                        "  🏓 ICMP: {}",
                        line.split('=').next_back().unwrap_or("N/A").trim()
                    );
                    break;
                }
            }
        } else {
            println!("  ❌ ICMP: timeout/blocked");
        }

        // TCP connection test
        let tcp_start = std::time::Instant::now();
        let tcp_result = Command::new("nc")
            .args(&["-zv", "-w", "2", hostname, port])
            .output();

        let tcp_time = tcp_start.elapsed().as_millis();

        match tcp_result {
            Ok(out) if out.status.success() => {
                println!("  ⚡ TCP {}: {}ms", port, tcp_time);
            }
            _ => {
                println!("  ❌ TCP {}: timeout/closed", port);
            }
        }

        println!();
    }

    println!("💡 Lower latency values indicate better gaming performance");
}

fn dns_resolution_speed_test() {
    println!("🌐 DNS Resolution Speed Test");
    println!("============================\n");

    let gaming_domains = vec![
        "worldofwarcraft.com",
        "battle.net",
        "steampowered.com",
        "discordapp.com",
        "riotgames.com",
        "epicgames.com",
        "ea.com",
        "ubisoft.com",
        "activision.com",
        "blizzard.com",
    ];

    println!("🔍 Testing DNS resolution speed for gaming domains...\n");

    let mut total_time = 0.0;
    let mut successful_queries = 0;

    for domain in &gaming_domains {
        let start_time = std::time::Instant::now();

        let result = Command::new("nslookup").arg(domain).output();

        let elapsed = start_time.elapsed().as_millis() as f64;

        match result {
            Ok(out) if out.status.success() => {
                println!("  ✅ {}: {:.0}ms", domain, elapsed);
                total_time += elapsed;
                successful_queries += 1;
            }
            _ => {
                println!("  ❌ {}: resolution failed", domain);
            }
        }
    }

    if successful_queries > 0 {
        let average = total_time / successful_queries as f64;
        println!("\n📊 DNS Resolution Summary:");
        println!("  📈 Average resolution time: {:.1}ms", average);
        println!(
            "  ✅ Successful queries: {}/{}",
            successful_queries,
            gaming_domains.len()
        );

        if average < 20.0 {
            println!("  🎮 Status: Excellent for gaming");
        } else if average < 50.0 {
            println!("  ⚡ Status: Good for gaming");
        } else {
            println!("  ⚠️ Status: May impact gaming performance");
        }
    }
}

fn network_interface_analysis() {
    println!("📡 Network Interface Analysis");
    println!("=============================\n");

    // Get all network interfaces
    let interfaces_result = Command::new("ls").arg("/sys/class/net").output();

    let interfaces = if let Ok(out) = interfaces_result {
        String::from_utf8_lossy(&out.stdout)
            .lines()
            .filter(|line| !line.contains("lo") && !line.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    } else {
        vec![]
    };

    for interface in &interfaces {
        println!("🌐 Interface: {}", interface);

        // Interface statistics
        let stats_path = format!("/sys/class/net/{}/statistics", interface);

        let rx_bytes_result = std::fs::read_to_string(format!("{}/rx_bytes", stats_path));
        let tx_bytes_result = std::fs::read_to_string(format!("{}/tx_bytes", stats_path));
        let rx_errors_result = std::fs::read_to_string(format!("{}/rx_errors", stats_path));
        let tx_errors_result = std::fs::read_to_string(format!("{}/tx_errors", stats_path));

        if let (Ok(rx_bytes), Ok(tx_bytes)) = (rx_bytes_result, tx_bytes_result) {
            let rx_mb = rx_bytes.trim().parse::<u64>().unwrap_or(0) / 1024 / 1024;
            let tx_mb = tx_bytes.trim().parse::<u64>().unwrap_or(0) / 1024 / 1024;
            println!(
                "  📊 Traffic: {} MB received, {} MB transmitted",
                rx_mb, tx_mb
            );
        }

        if let (Ok(rx_errors), Ok(tx_errors)) = (rx_errors_result, tx_errors_result) {
            let rx_err = rx_errors.trim().parse::<u64>().unwrap_or(0);
            let tx_err = tx_errors.trim().parse::<u64>().unwrap_or(0);
            if rx_err > 0 || tx_err > 0 {
                println!("  ⚠️ Errors: {} RX, {} TX", rx_err, tx_err);
            } else {
                println!("  ✅ No errors detected");
            }
        }

        // Interface speed
        let speed_result = std::fs::read_to_string(format!("/sys/class/net/{}/speed", interface));
        if let Ok(speed) = speed_result
            && let Ok(speed_mbps) = speed.trim().parse::<u32>()
        {
            println!("  ⚡ Link speed: {} Mbps", speed_mbps);
        }

        // Driver info
        let ethtool_result = Command::new("ethtool").args(&["-i", interface]).output();

        if let Ok(out) = ethtool_result {
            let output = String::from_utf8_lossy(&out.stdout);
            for line in output.lines() {
                if line.starts_with("driver:") {
                    println!("  🔧 {}", line);
                }
            }
        }

        println!();
    }
}

fn realtime_latency_monitoring() {
    println!("⚡ Real-time Latency Monitoring");
    println!("==============================\n");

    let target: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter target hostname or IP to monitor")
        .default("8.8.8.8".to_string())
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    let duration: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Monitor duration in seconds")
        .default("30".to_string())
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    println!("\n🔍 Starting real-time latency monitoring...");
    println!(
        "Target: {} | Duration: {}s | Press Ctrl+C to stop\n",
        target, duration
    );

    // Validate target (basic check for alphanumeric + dots/hyphens for hostnames/IPs)
    if !target
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == ':')
    {
        println!("❌ Invalid target hostname/IP");
        return;
    }

    // Parse duration
    let timeout_secs: u64 = duration.parse().unwrap_or(30);

    // Use ping directly with timeout
    let status = Command::new("timeout")
        .args([
            &format!("{}s", timeout_secs),
            "ping",
            "-i",
            "0.2",
            "-D", // Print timestamps (on supported systems)
            &target,
        ])
        .status();

    if let Err(e) = status {
        println!("⚠️ Ping failed: {}", e);
    }

    println!("\n📊 Monitoring completed");
    println!("💡 Look for consistent latency patterns - spikes may indicate network issues");
}

fn packet_loss_analysis() {
    println!("🔍 Packet Loss Analysis");
    println!("========================\n");

    let targets = vec![
        ("Google DNS", "8.8.8.8"),
        ("Cloudflare", "1.1.1.1"),
        ("Battle.net", "us.battle.net"),
        ("Steam", "steamcommunity.com"),
        ("Discord", "discord.com"),
    ];

    println!("📊 Testing packet loss to multiple targets...\n");

    for (name, target) in &targets {
        println!("🎯 Testing {}: {}", name, target);

        let ping_result = Command::new("ping")
            .args(&["-c", "100", "-i", "0.1", "-W", "2", target])
            .output();

        match ping_result {
            Ok(out) => {
                let output = String::from_utf8_lossy(&out.stdout);

                // Extract packet loss statistics
                for line in output.lines() {
                    if line.contains("packet loss") {
                        println!("  📊 {}", line.trim());
                    }
                    if line.contains("min/avg/max") {
                        println!("  ⚡ {}", line.trim());
                    }
                }
            }
            _ => {
                println!("  ❌ Test failed - host unreachable");
            }
        }

        println!();
    }

    println!("💡 Packet loss analysis:");
    println!("  • 0% loss: Perfect connection");
    println!("  • 1-3% loss: Acceptable for gaming");
    println!("  • 4-10% loss: May cause gaming issues");
    println!("  • >10% loss: Poor connection, investigate network");
}

// Missing function stubs for gaming optimization
fn low_latency_configuration() {
    println!("Low Latency Configuration - Coming soon...");
}

fn gaming_service_ports() {
    println!("Gaming Service Ports - Coming soon...");
}

fn qos_gaming_priority() {
    println!("QoS Gaming Priority - Coming soon...");
}

fn gaming_dns_optimization() {
    println!("Gaming DNS Optimization - Coming soon...");
}

fn gaming_network_analysis() {
    println!("Gaming Network Analysis - Coming soon...");
}
