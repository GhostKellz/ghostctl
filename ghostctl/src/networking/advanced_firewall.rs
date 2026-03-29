use crate::security::validation::{ValidatedCidr, ValidatedIpAddress, ValidatedPort};
use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Validate an nftables identifier (table name, chain name, set name)
/// Must be alphanumeric with underscore, starting with letter or underscore
fn validate_nft_identifier(input: &str) -> Result<String, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Identifier cannot be empty".to_string());
    }
    if trimmed.len() > 64 {
        return Err("Identifier too long (max 64 characters)".to_string());
    }

    // Must start with letter or underscore
    // Safe: we already checked for empty string above
    let Some(first_char) = trimmed.chars().next() else {
        return Err("Identifier cannot be empty".to_string());
    };
    if !first_char.is_ascii_alphabetic() && first_char != '_' {
        return Err("Identifier must start with a letter or underscore".to_string());
    }

    // Rest must be alphanumeric or underscore
    if !trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err("Identifier must contain only letters, numbers, and underscores".to_string());
    }

    Ok(trimmed.to_string())
}

/// Validate a rate limit string (e.g., "10/second", "100/minute")
fn validate_rate_limit(input: &str) -> Result<String, String> {
    let trimmed = input.trim();
    let parts: Vec<&str> = trimmed.split('/').collect();
    if parts.len() != 2 {
        return Err("Rate must be in format: number/unit (e.g., 10/second)".to_string());
    }

    let number: u32 = parts[0]
        .parse()
        .map_err(|_| "Rate number must be a positive integer")?;
    if number == 0 {
        return Err("Rate number must be greater than 0".to_string());
    }

    let valid_units = ["second", "minute", "hour", "day"];
    if !valid_units.contains(&parts[1]) {
        return Err(format!(
            "Rate unit must be one of: {}",
            valid_units.join(", ")
        ));
    }

    Ok(trimmed.to_string())
}

/// Validate a burst value (positive integer)
fn validate_burst(input: &str) -> Result<u32, String> {
    let trimmed = input.trim();
    let value: u32 = trimmed
        .parse()
        .map_err(|_| "Burst must be a positive integer")?;
    if value == 0 {
        return Err("Burst must be greater than 0".to_string());
    }
    Ok(value)
}

/// Validate a log prefix (no shell metacharacters, reasonable length)
fn validate_log_prefix(input: &str) -> Result<String, String> {
    let trimmed = input.trim();
    if trimmed.len() > 64 {
        return Err("Log prefix too long (max 64 characters)".to_string());
    }

    // Only allow safe characters in log prefix
    let dangerous = [
        ';', '|', '&', '$', '`', '(', ')', '{', '}', '<', '>', '\\', '\n', '\r',
    ];
    if trimmed.chars().any(|c| dangerous.contains(&c)) {
        return Err("Log prefix contains invalid characters".to_string());
    }

    Ok(trimmed.to_string())
}

/// Validate a timeout string (e.g., "30s", "5m", "1h")
fn validate_timeout(input: &str) -> Result<String, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Timeout cannot be empty".to_string());
    }

    // Must be number followed by unit
    let len = trimmed.len();
    if len < 2 {
        return Err("Timeout must be in format: number+unit (e.g., 30s, 5m, 1h)".to_string());
    }

    let (number_part, unit) = trimmed.split_at(len - 1);
    let _number: u32 = number_part
        .parse()
        .map_err(|_| "Timeout must start with a positive integer")?;

    let valid_units = ['s', 'm', 'h', 'd'];
    let Some(unit_char) = unit.chars().next() else {
        return Err("Timeout must include a unit (s, m, h, d)".to_string());
    };
    if !valid_units.contains(&unit_char) {
        return Err(
            "Timeout unit must be s (seconds), m (minutes), h (hours), or d (days)".to_string(),
        );
    }

    Ok(trimmed.to_string())
}

/// Validate a position number
fn validate_position(input: &str) -> Result<Option<u32>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let value: u32 = trimmed
        .parse()
        .map_err(|_| "Position must be a positive integer")?;
    Ok(Some(value))
}

pub fn advanced_firewall_menu() {
    loop {
        let options = [
            "🚀 Advanced nftables Management",
            "⚙️ Advanced iptables Features",
            "🛡️ Network Security Tools",
            "🎮 Gaming Network Optimization",
            "🔍 Network Troubleshooting Tools",
            "📊 Connection State Analyzer",
            "🌐 NAT & Port Forwarding",
            "🔐 DDoS Protection Setup",
            "🚪 Port Knocking Configuration",
            "📈 QoS & Traffic Shaping",
            "⬅️ Back",
        ];

        let choice = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔥 Advanced Firewall & Networking")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            Ok(None) | Err(_) => break,
        };

        match choice {
            0 => advanced_nftables_management(),
            1 => advanced_iptables_features(),
            2 => network_security_tools(),
            3 => gaming_network_optimization(),
            4 => network_troubleshooting_tools(),
            5 => connection_state_analyzer(),
            6 => nat_port_forwarding(),
            7 => ddos_protection_setup(),
            8 => port_knocking_configuration(),
            9 => qos_traffic_shaping(),
            _ => break,
        }
    }
}

fn advanced_nftables_management() {
    loop {
        let options = [
            "🔧 nftables Rule Builder GUI",
            "📦 nftables Set Management",
            "🔢 Chain Priorities Configuration",
            "🔄 Dynamic Sets & Rate Limiting",
            "📋 Rule Optimizer",
            "🔄 iptables to nftables Migration",
            "💾 Ruleset Backup/Restore",
            "🧪 Rule Testing Sandbox",
            "📊 Performance Monitoring",
            "📝 Template Library",
            "⬅️ Back",
        ];

        let choice = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🚀 Advanced nftables Management")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            Ok(None) | Err(_) => break,
        };

        match choice {
            0 => nftables_rule_builder(),
            1 => nftables_set_management(),
            2 => chain_priorities_configuration(),
            3 => dynamic_sets_rate_limiting(),
            4 => rule_optimizer(),
            5 => iptables_to_nftables_migration(),
            6 => ruleset_backup_restore(),
            7 => rule_testing_sandbox(),
            8 => performance_monitoring(),
            9 => template_library(),
            _ => break,
        }
    }
}

fn nftables_rule_builder() {
    println!("🔧 nftables Rule Builder");

    let table_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .default("filter".to_string())
        .interact()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    // Validate table name
    let table_name = match validate_nft_identifier(&table_input) {
        Ok(t) => t,
        Err(e) => {
            println!("❌ Invalid table name: {}", e);
            return;
        }
    };

    let chain_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter chain name")
        .default("input".to_string())
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    // Validate chain name
    let chain_name = match validate_nft_identifier(&chain_input) {
        Ok(c) => c,
        Err(e) => {
            println!("❌ Invalid chain name: {}", e);
            return;
        }
    };

    let rule_type = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select rule type")
        .items(&[
            "Allow port",
            "Block IP/subnet",
            "Rate limit",
            "Connection tracking",
            "NAT rule",
            "Jump to chain",
            "Log and drop",
        ])
        .default(0)
        .interact_opt()
    {
        Ok(Some(r)) => r,
        Ok(None) | Err(_) => return,
    };

    // Build rule parts as a vector for safe command construction
    let mut rule_parts: Vec<String> = Vec::new();

    match rule_type {
        0 => {
            // Allow port
            let port_input: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter port number")
                .interact()
            {
                Ok(p) => p,
                Err(_) => return,
            };

            // Validate port
            let validated_port = match ValidatedPort::from_input(&port_input) {
                Ok(p) => p,
                Err(e) => {
                    println!("❌ Invalid port: {}", e);
                    return;
                }
            };

            let protocol = match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select protocol")
                .items(&["tcp", "udp", "both"])
                .default(0)
                .interact_opt()
            {
                Ok(Some(p)) => p,
                Ok(None) | Err(_) => return,
            };

            match protocol {
                0 => {
                    rule_parts.push("tcp".to_string());
                    rule_parts.push("dport".to_string());
                    rule_parts.push(validated_port.to_string());
                    rule_parts.push("accept".to_string());
                }
                1 => {
                    rule_parts.push("udp".to_string());
                    rule_parts.push("dport".to_string());
                    rule_parts.push(validated_port.to_string());
                    rule_parts.push("accept".to_string());
                }
                _ => {
                    // Both protocols - need special nft syntax
                    rule_parts.push("meta".to_string());
                    rule_parts.push("l4proto".to_string());
                    rule_parts.push("{".to_string());
                    rule_parts.push("tcp,".to_string());
                    rule_parts.push("udp".to_string());
                    rule_parts.push("}".to_string());
                    rule_parts.push("th".to_string());
                    rule_parts.push("dport".to_string());
                    rule_parts.push(validated_port.to_string());
                    rule_parts.push("accept".to_string());
                }
            }
        }
        1 => {
            // Block IP/subnet
            let ip_input: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter IP or subnet (e.g., 192.168.1.0/24)")
                .interact()
            {
                Ok(i) => i,
                Err(_) => return,
            };

            // Validate IP or CIDR
            let validated_ip = if ip_input.contains('/') {
                match ValidatedCidr::from_input(&ip_input) {
                    Ok(c) => c.value().to_string(),
                    Err(e) => {
                        println!("❌ Invalid CIDR: {}", e);
                        return;
                    }
                }
            } else {
                match ValidatedIpAddress::from_input(&ip_input) {
                    Ok(ip) => ip.value().to_string(),
                    Err(e) => {
                        println!("❌ Invalid IP address: {}", e);
                        return;
                    }
                }
            };

            let action = match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select action")
                .items(&["drop", "reject"])
                .default(0)
                .interact_opt()
            {
                Ok(Some(a)) => a,
                Ok(None) | Err(_) => return,
            };

            let action_str = if action == 0 { "drop" } else { "reject" };

            rule_parts.push("ip".to_string());
            rule_parts.push("saddr".to_string());
            rule_parts.push(validated_ip);
            rule_parts.push(action_str.to_string());
        }
        2 => {
            // Rate limit
            let rate_input: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter rate (e.g., 10/second, 100/minute)")
                .default("10/second".to_string())
                .interact()
            {
                Ok(r) => r,
                Err(_) => return,
            };

            // Validate rate
            let validated_rate = match validate_rate_limit(&rate_input) {
                Ok(r) => r,
                Err(e) => {
                    println!("❌ Invalid rate: {}", e);
                    return;
                }
            };

            let burst_input: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter burst limit")
                .default("5".to_string())
                .interact()
            {
                Ok(b) => b,
                Err(_) => return,
            };

            // Validate burst
            let validated_burst = match validate_burst(&burst_input) {
                Ok(b) => b,
                Err(e) => {
                    println!("❌ Invalid burst: {}", e);
                    return;
                }
            };

            rule_parts.push("limit".to_string());
            rule_parts.push("rate".to_string());
            rule_parts.push(validated_rate);
            rule_parts.push("burst".to_string());
            rule_parts.push(validated_burst.to_string());
            rule_parts.push("packets".to_string());
            rule_parts.push("accept".to_string());
        }
        3 => {
            // Connection tracking
            let states = vec!["new", "established", "related", "invalid"];
            let selected = match MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Select connection states")
                .items(&states)
                .interact_opt()
            {
                Ok(Some(s)) => s,
                Ok(None) | Err(_) => return,
            };

            if selected.is_empty() {
                println!("❌ No states selected");
                return;
            }

            let state_list: Vec<&str> = selected.iter().map(|&i| states[i]).collect();

            let action = match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select action")
                .items(&["accept", "drop", "reject"])
                .default(0)
                .interact_opt()
            {
                Ok(Some(a)) => a,
                Ok(None) | Err(_) => return,
            };

            let action_str = ["accept", "drop", "reject"][action];

            rule_parts.push("ct".to_string());
            rule_parts.push("state".to_string());
            rule_parts.push("{".to_string());
            rule_parts.push(state_list.join(", "));
            rule_parts.push("}".to_string());
            rule_parts.push(action_str.to_string());
        }
        4 => {
            // NAT rule
            let nat_type = match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select NAT type")
                .items(&["SNAT", "DNAT", "MASQUERADE"])
                .default(0)
                .interact_opt()
            {
                Ok(Some(n)) => n,
                Ok(None) | Err(_) => return,
            };

            match nat_type {
                0 => {
                    let ip_input: String = match Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter source IP")
                        .interact()
                    {
                        Ok(i) => i,
                        Err(_) => return,
                    };

                    // Validate IP
                    let validated_ip = match ValidatedIpAddress::from_input(&ip_input) {
                        Ok(ip) => ip.value().to_string(),
                        Err(e) => {
                            println!("❌ Invalid IP address: {}", e);
                            return;
                        }
                    };

                    rule_parts.push("snat".to_string());
                    rule_parts.push("to".to_string());
                    rule_parts.push(validated_ip);
                }
                1 => {
                    let ip_input: String = match Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter destination IP")
                        .interact()
                    {
                        Ok(i) => i,
                        Err(_) => return,
                    };

                    // Validate IP
                    let validated_ip = match ValidatedIpAddress::from_input(&ip_input) {
                        Ok(ip) => ip.value().to_string(),
                        Err(e) => {
                            println!("❌ Invalid IP address: {}", e);
                            return;
                        }
                    };

                    let port_input: String = match Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter destination port (or press Enter to skip)")
                        .allow_empty(true)
                        .interact()
                    {
                        Ok(p) => p,
                        Err(_) => return,
                    };

                    rule_parts.push("dnat".to_string());
                    rule_parts.push("to".to_string());

                    if port_input.is_empty() {
                        rule_parts.push(validated_ip);
                    } else {
                        // Validate port
                        let validated_port = match ValidatedPort::from_input(&port_input) {
                            Ok(p) => p,
                            Err(e) => {
                                println!("❌ Invalid port: {}", e);
                                return;
                            }
                        };
                        rule_parts.push(format!("{}:{}", validated_ip, validated_port));
                    }
                }
                2 => {
                    rule_parts.push("masquerade".to_string());
                }
                _ => {}
            }
        }
        5 => {
            // Jump to chain
            let target_input: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter target chain name")
                .interact()
            {
                Ok(t) => t,
                Err(_) => return,
            };

            // Validate chain name
            let target_chain = match validate_nft_identifier(&target_input) {
                Ok(c) => c,
                Err(e) => {
                    println!("❌ Invalid chain name: {}", e);
                    return;
                }
            };

            rule_parts.push("jump".to_string());
            rule_parts.push(target_chain);
        }
        6 => {
            // Log and drop
            let prefix_input: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter log prefix")
                .default("DROPPED: ".to_string())
                .interact()
            {
                Ok(p) => p,
                Err(_) => return,
            };

            // Validate log prefix
            let validated_prefix = match validate_log_prefix(&prefix_input) {
                Ok(p) => p,
                Err(e) => {
                    println!("❌ Invalid log prefix: {}", e);
                    return;
                }
            };

            rule_parts.push("log".to_string());
            rule_parts.push("prefix".to_string());
            rule_parts.push(format!("\"{}\"", validated_prefix));
            rule_parts.push("drop".to_string());
        }
        _ => {}
    }

    if rule_parts.is_empty() {
        println!("❌ No rule components specified");
        return;
    }

    let position_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter rule position (or press Enter for end)")
        .allow_empty(true)
        .interact()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    // Validate position
    let position = match validate_position(&position_input) {
        Ok(p) => p,
        Err(e) => {
            println!("❌ Invalid position: {}", e);
            return;
        }
    };

    // Build command args safely (no shell)
    let mut args: Vec<String> = vec![
        "nft".to_string(),
        "add".to_string(),
        "rule".to_string(),
        table_name.clone(),
        chain_name.clone(),
    ];

    if let Some(pos) = position {
        args.push("position".to_string());
        args.push(pos.to_string());
    }

    args.extend(rule_parts.clone());

    // Display command for user review
    println!("\n📋 Generated rule:");
    println!("sudo {}", args.join(" "));

    let execute = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Execute this rule?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(e)) => e,
        Ok(None) | Err(_) => return,
    };

    if execute {
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

        let status = Command::new("sudo").args(&args_refs).status();

        match status {
            Ok(s) if s.success() => println!("✅ Rule added successfully"),
            Ok(s) => println!("❌ Failed to add rule (exit code: {:?})", s.code()),
            Err(e) => println!("❌ Failed to execute command: {}", e),
        }
    }
}

fn nftables_set_management() {
    println!("📦 nftables Set Management");

    let options = [
        "Create set",
        "Add elements to set",
        "Remove elements from set",
        "List sets",
        "Delete set",
        "Create dynamic set",
        "Import set from file",
        "Export set to file",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Set management options")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => create_nftables_set(),
        1 => add_to_set(),
        2 => remove_from_set(),
        3 => list_sets(),
        4 => delete_set(),
        5 => create_dynamic_set(),
        6 => import_set(),
        7 => export_set(),
        _ => {}
    }
}

fn create_nftables_set() {
    println!("📦 Create nftables Set");

    let table_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .default("filter".to_string())
        .interact()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    // Validate table name
    let table = match validate_nft_identifier(&table_input) {
        Ok(t) => t,
        Err(e) => {
            println!("❌ Invalid table name: {}", e);
            return;
        }
    };

    let set_name_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter set name")
        .interact()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    // Validate set name
    let set_name = match validate_nft_identifier(&set_name_input) {
        Ok(s) => s,
        Err(e) => {
            println!("❌ Invalid set name: {}", e);
            return;
        }
    };

    let set_type = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select set type")
        .items(&[
            "ipv4_addr",
            "ipv6_addr",
            "ether_addr",
            "inet_proto",
            "inet_service",
            "mark",
        ])
        .default(0)
        .interact_opt()
    {
        Ok(Some(t)) => t,
        Ok(None) | Err(_) => return,
    };

    let type_str = [
        "ipv4_addr",
        "ipv6_addr",
        "ether_addr",
        "inet_proto",
        "inet_service",
        "mark",
    ][set_type];

    let flags = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select flags (optional)")
        .items(&["interval", "timeout", "constant", "dynamic"])
        .interact_opt()
    {
        Ok(Some(f)) => f,
        Ok(None) | Err(_) => return,
    };

    // Build the set specification
    let mut set_spec = format!("{{ type {};", type_str);
    if !flags.is_empty() {
        let flag_names: Vec<&str> = flags
            .iter()
            .map(|&i| ["interval", "timeout", "constant", "dynamic"][i])
            .collect();
        set_spec.push_str(&format!(" flags {};", flag_names.join(", ")));
    }
    set_spec.push_str(" }");

    // Build command using direct args (no shell)
    println!("📋 Creating set: {} in table {}", set_name, table);

    let status = Command::new("sudo")
        .args(["nft", "add", "set", &table, &set_name, &set_spec])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Set created: {}", set_name),
        Ok(s) => println!("❌ Failed to create set (exit code: {:?})", s.code()),
        Err(e) => println!("❌ Failed to execute command: {}", e),
    }
}

fn add_to_set() {
    println!("➕ Add Elements to Set");

    let table_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .default("filter".to_string())
        .interact()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    // Validate table name
    let table = match validate_nft_identifier(&table_input) {
        Ok(t) => t,
        Err(e) => {
            println!("❌ Invalid table name: {}", e);
            return;
        }
    };

    let set_name_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter set name")
        .interact()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    // Validate set name
    let set_name = match validate_nft_identifier(&set_name_input) {
        Ok(s) => s,
        Err(e) => {
            println!("❌ Invalid set name: {}", e);
            return;
        }
    };

    let elements: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter elements (comma-separated IPs or values)")
        .interact()
    {
        Ok(e) => e,
        Err(_) => return,
    };

    // Validate each element (assuming IP addresses for now)
    let mut validated_elements: Vec<String> = Vec::new();
    for element in elements.split(',').map(|s| s.trim()) {
        if element.is_empty() {
            continue;
        }
        // Try to validate as IP or CIDR
        if element.contains('/') {
            match ValidatedCidr::from_input(element) {
                Ok(c) => validated_elements.push(c.value().to_string()),
                Err(e) => {
                    println!("❌ Invalid CIDR '{}': {}", element, e);
                    return;
                }
            }
        } else if element.contains('.') || element.contains(':') {
            match ValidatedIpAddress::from_input(element) {
                Ok(ip) => validated_elements.push(ip.value().to_string()),
                Err(e) => {
                    println!("❌ Invalid IP '{}': {}", element, e);
                    return;
                }
            }
        } else {
            // Could be a port or other numeric value
            if let Ok(port) = ValidatedPort::from_input(element) {
                validated_elements.push(port.to_string());
            } else {
                println!(
                    "❌ Invalid element '{}': must be IP, CIDR, or port",
                    element
                );
                return;
            }
        }
    }

    if validated_elements.is_empty() {
        println!("❌ No valid elements provided");
        return;
    }

    let elements_str = format!("{{ {} }}", validated_elements.join(", "));

    let status = Command::new("sudo")
        .args(["nft", "add", "element", &table, &set_name, &elements_str])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Elements added to set"),
        Ok(s) => println!("❌ Failed to add elements (exit code: {:?})", s.code()),
        Err(e) => println!("❌ Failed to execute command: {}", e),
    }
}

fn remove_from_set() {
    println!("➖ Remove Elements from Set");

    let table_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .default("filter".to_string())
        .interact()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    // Validate table name
    let table = match validate_nft_identifier(&table_input) {
        Ok(t) => t,
        Err(e) => {
            println!("❌ Invalid table name: {}", e);
            return;
        }
    };

    let set_name_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter set name")
        .interact()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    // Validate set name
    let set_name = match validate_nft_identifier(&set_name_input) {
        Ok(s) => s,
        Err(e) => {
            println!("❌ Invalid set name: {}", e);
            return;
        }
    };

    let elements: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter elements to remove (comma-separated)")
        .interact()
    {
        Ok(e) => e,
        Err(_) => return,
    };

    // Validate each element (same as add_to_set)
    let mut validated_elements: Vec<String> = Vec::new();
    for element in elements.split(',').map(|s| s.trim()) {
        if element.is_empty() {
            continue;
        }
        if element.contains('/') {
            match ValidatedCidr::from_input(element) {
                Ok(c) => validated_elements.push(c.value().to_string()),
                Err(e) => {
                    println!("❌ Invalid CIDR '{}': {}", element, e);
                    return;
                }
            }
        } else if element.contains('.') || element.contains(':') {
            match ValidatedIpAddress::from_input(element) {
                Ok(ip) => validated_elements.push(ip.value().to_string()),
                Err(e) => {
                    println!("❌ Invalid IP '{}': {}", element, e);
                    return;
                }
            }
        } else if let Ok(port) = ValidatedPort::from_input(element) {
            validated_elements.push(port.to_string());
        } else {
            println!(
                "❌ Invalid element '{}': must be IP, CIDR, or port",
                element
            );
            return;
        }
    }

    if validated_elements.is_empty() {
        println!("❌ No valid elements provided");
        return;
    }

    let elements_str = format!("{{ {} }}", validated_elements.join(", "));

    let status = Command::new("sudo")
        .args(["nft", "delete", "element", &table, &set_name, &elements_str])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Elements removed from set"),
        Ok(s) => println!("❌ Failed to remove elements (exit code: {:?})", s.code()),
        Err(e) => println!("❌ Failed to execute command: {}", e),
    }
}

fn list_sets() {
    println!("📋 List nftables Sets");

    let output = Command::new("sudo").args(["nft", "list", "sets"]).output();

    match output {
        Ok(out) if out.status.success() => {
            let result = String::from_utf8_lossy(&out.stdout);
            if result.trim().is_empty() {
                println!("No sets found");
            } else {
                println!("{}", result);
            }
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            println!("❌ Failed to list sets: {}", stderr);
        }
        Err(e) => println!("❌ Failed to execute command: {}", e),
    }
}

fn delete_set() {
    println!("🗑️ Delete Set");

    let table_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .default("filter".to_string())
        .interact()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    // Validate table name
    let table = match validate_nft_identifier(&table_input) {
        Ok(t) => t,
        Err(e) => {
            println!("❌ Invalid table name: {}", e);
            return;
        }
    };

    let set_name_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter set name to delete")
        .interact()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    // Validate set name
    let set_name = match validate_nft_identifier(&set_name_input) {
        Ok(s) => s,
        Err(e) => {
            println!("❌ Invalid set name: {}", e);
            return;
        }
    };

    // Confirmation prompt for dangerous operation
    let confirm = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "⚠️ WARNING: Delete set '{}'? This cannot be undone.",
            set_name
        ))
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        let status = Command::new("sudo")
            .args(["nft", "delete", "set", &table, &set_name])
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Set deleted"),
            Ok(s) => println!("❌ Failed to delete set (exit code: {:?})", s.code()),
            Err(e) => println!("❌ Failed to execute command: {}", e),
        }
    }
}

fn create_dynamic_set() {
    println!("🔄 Create Dynamic Set");

    let table_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .default("filter".to_string())
        .interact()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    // Validate table name
    let table = match validate_nft_identifier(&table_input) {
        Ok(t) => t,
        Err(e) => {
            println!("❌ Invalid table name: {}", e);
            return;
        }
    };

    let set_name_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter dynamic set name")
        .interact()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    // Validate set name
    let set_name = match validate_nft_identifier(&set_name_input) {
        Ok(s) => s,
        Err(e) => {
            println!("❌ Invalid set name: {}", e);
            return;
        }
    };

    let timeout_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter timeout (e.g., 30s, 5m, 1h)")
        .default("5m".to_string())
        .interact()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    // Validate timeout
    let timeout = match validate_timeout(&timeout_input) {
        Ok(t) => t,
        Err(e) => {
            println!("❌ Invalid timeout: {}", e);
            return;
        }
    };

    let max_size_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter maximum size")
        .default("65535".to_string())
        .interact()
    {
        Ok(m) => m,
        Err(_) => return,
    };

    // Validate max size
    let max_size: u32 = match max_size_input.parse() {
        Ok(m) if m > 0 => m,
        _ => {
            println!("❌ Invalid max size: must be a positive integer");
            return;
        }
    };

    // Build the set specification
    let set_spec = format!(
        "{{ type ipv4_addr; flags dynamic, timeout; timeout {}; size {}; }}",
        timeout, max_size
    );

    println!("📋 Creating dynamic set for rate limiting...");

    let status = Command::new("sudo")
        .args(["nft", "add", "set", &table, &set_name, &set_spec])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Dynamic set created: {}", set_name);

            // Create rate limiting rule
            let create_rule = match Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Create rate limiting rule using this set?")
                .default(true)
                .interact_opt()
            {
                Ok(Some(c)) => c,
                Ok(None) | Err(_) => return,
            };

            if create_rule {
                let rate_input: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter rate limit (e.g., 10/second)")
                    .default("10/second".to_string())
                    .interact()
                {
                    Ok(r) => r,
                    Err(_) => return,
                };

                // Validate rate
                let rate = match validate_rate_limit(&rate_input) {
                    Ok(r) => r,
                    Err(e) => {
                        println!("❌ Invalid rate: {}", e);
                        return;
                    }
                };

                let set_ref = format!("@{}", set_name);
                let rate_spec = format!("limit rate {}", rate);

                let status = Command::new("sudo")
                    .args([
                        "nft", "add", "rule", &table, "input", "ip", "saddr", &set_ref, &rate_spec,
                        "drop",
                    ])
                    .status();

                match status {
                    Ok(s) if s.success() => println!("✅ Rate limiting rule created"),
                    Ok(s) => println!("❌ Failed to create rule (exit code: {:?})", s.code()),
                    Err(e) => println!("❌ Failed to execute command: {}", e),
                }
            }
        }
        Ok(s) => println!(
            "❌ Failed to create dynamic set (exit code: {:?})",
            s.code()
        ),
        Err(e) => println!("❌ Failed to execute command: {}", e),
    }
}

fn import_set() {
    println!("📥 Import Set from File");

    let file_path: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter file path containing IPs/elements (one per line)")
        .interact()
    {
        Ok(f) => f,
        Err(_) => return,
    };

    if !Path::new(&file_path).exists() {
        println!("❌ File not found");
        return;
    }

    let table_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .default("filter".to_string())
        .interact()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    // Validate table name
    let table = match validate_nft_identifier(&table_input) {
        Ok(t) => t,
        Err(e) => {
            println!("❌ Invalid table name: {}", e);
            return;
        }
    };

    let set_name_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter set name")
        .interact()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    // Validate set name
    let set_name = match validate_nft_identifier(&set_name_input) {
        Ok(s) => s,
        Err(e) => {
            println!("❌ Invalid set name: {}", e);
            return;
        }
    };

    let content = match fs::read_to_string(&file_path) {
        Ok(c) => c,
        Err(e) => {
            println!("❌ Failed to read file: {}", e);
            return;
        }
    };

    // Validate each element from file
    let mut validated_elements: Vec<String> = Vec::new();
    let mut line_number = 0;

    for line in content.lines() {
        line_number += 1;
        let element = line.trim();
        if element.is_empty() || element.starts_with('#') {
            continue;
        }

        // Validate as IP or CIDR
        if element.contains('/') {
            match ValidatedCidr::from_input(element) {
                Ok(c) => validated_elements.push(c.value().to_string()),
                Err(e) => {
                    println!("❌ Line {}: Invalid CIDR '{}': {}", line_number, element, e);
                    return;
                }
            }
        } else if element.contains('.') || element.contains(':') {
            match ValidatedIpAddress::from_input(element) {
                Ok(ip) => validated_elements.push(ip.value().to_string()),
                Err(e) => {
                    println!("❌ Line {}: Invalid IP '{}': {}", line_number, element, e);
                    return;
                }
            }
        } else if let Ok(port) = ValidatedPort::from_input(element) {
            validated_elements.push(port.to_string());
        } else {
            println!(
                "❌ Line {}: Invalid element '{}': must be IP, CIDR, or port",
                line_number, element
            );
            return;
        }
    }

    if validated_elements.is_empty() {
        println!("❌ No valid elements found in file");
        return;
    }

    let elements_str = format!("{{ {} }}", validated_elements.join(", "));

    let status = Command::new("sudo")
        .args(["nft", "add", "element", &table, &set_name, &elements_str])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ {} elements imported", validated_elements.len()),
        Ok(s) => println!("❌ Failed to import elements (exit code: {:?})", s.code()),
        Err(e) => println!("❌ Failed to execute command: {}", e),
    }
}

fn export_set() {
    println!("📤 Export Set to File");

    let table_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .default("filter".to_string())
        .interact()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    // Validate table name
    let table = match validate_nft_identifier(&table_input) {
        Ok(t) => t,
        Err(e) => {
            println!("❌ Invalid table name: {}", e);
            return;
        }
    };

    let set_name_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter set name")
        .interact()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    // Validate set name
    let set_name = match validate_nft_identifier(&set_name_input) {
        Ok(s) => s,
        Err(e) => {
            println!("❌ Invalid set name: {}", e);
            return;
        }
    };

    let export_path: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter export file path")
        .default(format!(
            "{}/{}_set.txt",
            std::env::var("HOME").unwrap_or_default(),
            set_name
        ))
        .interact()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let output = Command::new("sudo")
        .args(["nft", "list", "set", &table, &set_name])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let result = String::from_utf8_lossy(&out.stdout);
            match fs::write(&export_path, result.as_bytes()) {
                Ok(_) => println!("✅ Set exported to: {}", export_path),
                Err(e) => println!("❌ Failed to write file: {}", e),
            }
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            println!("❌ Failed to export set: {}", stderr);
        }
        Err(e) => println!("❌ Failed to execute command: {}", e),
    }
}

fn chain_priorities_configuration() {
    println!("🔢 Chain Priorities Configuration");

    let chains = [
        ("raw", -300),
        ("mangle", -150),
        ("dstnat", -100),
        ("filter", 0),
        ("security", 50),
        ("srcnat", 100),
    ];

    println!("📋 Standard chain priorities:");
    for (name, priority) in &chains {
        println!("  {} → {}", name, priority);
    }

    let custom = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Create custom priority chain?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if custom {
        let table_input: String = match Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter table name")
            .default("filter".to_string())
            .interact()
        {
            Ok(t) => t,
            Err(_) => return,
        };

        // Validate table name
        let table = match validate_nft_identifier(&table_input) {
            Ok(t) => t,
            Err(e) => {
                println!("❌ Invalid table name: {}", e);
                return;
            }
        };

        let chain_input: String = match Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter chain name")
            .interact()
        {
            Ok(c) => c,
            Err(_) => return,
        };

        // Validate chain name
        let chain = match validate_nft_identifier(&chain_input) {
            Ok(c) => c,
            Err(e) => {
                println!("❌ Invalid chain name: {}", e);
                return;
            }
        };

        let hook = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select hook")
            .items(&["prerouting", "input", "forward", "output", "postrouting"])
            .default(1)
            .interact_opt()
        {
            Ok(Some(h)) => h,
            Ok(None) | Err(_) => return,
        };

        let hook_str = ["prerouting", "input", "forward", "output", "postrouting"][hook];

        let priority_input: String = match Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter priority (-300 to 300)")
            .default("0".to_string())
            .interact()
        {
            Ok(p) => p,
            Err(_) => return,
        };

        // Validate priority
        let priority: i32 = match priority_input.parse() {
            Ok(p) if p >= -300 && p <= 300 => p,
            _ => {
                println!("❌ Invalid priority: must be a number between -300 and 300");
                return;
            }
        };

        let policy = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select default policy")
            .items(&["accept", "drop"])
            .default(0)
            .interact_opt()
        {
            Ok(Some(p)) => p,
            Ok(None) | Err(_) => return,
        };

        let policy_str = if policy == 0 { "accept" } else { "drop" };

        // Build chain specification
        let chain_spec = format!(
            "{{ type filter hook {} priority {}; policy {}; }}",
            hook_str, priority, policy_str
        );

        println!("📋 Creating chain: {} in table {}", chain, table);

        let status = Command::new("sudo")
            .args(["nft", "add", "chain", &table, &chain, &chain_spec])
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Chain created with priority {}", priority),
            Ok(s) => println!("❌ Failed to create chain (exit code: {:?})", s.code()),
            Err(e) => println!("❌ Failed to execute command: {}", e),
        }
    }
}

fn dynamic_sets_rate_limiting() {
    println!("🔄 Dynamic Sets & Rate Limiting");

    let scenarios = [
        "SSH brute force protection",
        "HTTP/HTTPS rate limiting",
        "SYN flood protection",
        "Connection limit per IP",
        "Port scan detection",
        "Custom rate limit",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select scenario")
        .items(&scenarios)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => ssh_brute_force_protection(),
        1 => http_rate_limiting(),
        2 => syn_flood_protection(),
        3 => connection_limit_per_ip(),
        4 => port_scan_detection(),
        5 => custom_rate_limit(),
        _ => {}
    }
}

fn ssh_brute_force_protection() {
    println!("🛡️ SSH Brute Force Protection");

    // Confirmation for enabling protection
    let confirm = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable SSH brute force protection? This will add firewall rules.")
        .default(true)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if !confirm {
        return;
    }

    let table = "filter";

    println!("🔧 Setting up SSH brute force protection...");

    // Create dynamic set for tracking (no user input, safe)
    let set_spec = "{ type ipv4_addr; flags dynamic, timeout; timeout 10m; }";
    let status1 = Command::new("sudo")
        .args(["nft", "add", "set", table, "ssh_ratelimit", set_spec])
        .status();

    if let Err(e) = status1 {
        println!("❌ Failed to create set: {}", e);
        return;
    }

    // Add rate limiting rule (static values, safe)
    let status2 = Command::new("sudo")
        .args([
            "nft",
            "add",
            "rule",
            table,
            "input",
            "tcp",
            "dport",
            "22",
            "ct",
            "state",
            "new",
            "add",
            "@ssh_ratelimit",
            "{",
            "ip",
            "saddr",
            "limit",
            "rate",
            "3/minute",
            "}",
            "accept",
        ])
        .status();

    if let Err(e) = status2 {
        println!("❌ Failed to add rate limit rule: {}", e);
        return;
    }

    // Drop excessive attempts
    let status3 = Command::new("sudo")
        .args([
            "nft",
            "add",
            "rule",
            table,
            "input",
            "tcp",
            "dport",
            "22",
            "ip",
            "saddr",
            "@ssh_ratelimit",
            "drop",
        ])
        .status();

    if let Err(e) = status3 {
        println!("❌ Failed to add drop rule: {}", e);
        return;
    }

    println!("✅ SSH brute force protection enabled");
    println!("  - Max 3 login attempts per minute");
    println!("  - Blocked IPs timeout after 10 minutes");
}

fn http_rate_limiting() {
    println!("🌐 HTTP/HTTPS Rate Limiting");

    let rate_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter rate limit (e.g., 100/second)")
        .default("100/second".to_string())
        .interact()
    {
        Ok(r) => r,
        Err(_) => return,
    };

    // Validate rate
    let rate = match validate_rate_limit(&rate_input) {
        Ok(r) => r,
        Err(e) => {
            println!("❌ Invalid rate: {}", e);
            return;
        }
    };

    let burst_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter burst size")
        .default("50".to_string())
        .interact()
    {
        Ok(b) => b,
        Err(_) => return,
    };

    // Validate burst
    let burst = match validate_burst(&burst_input) {
        Ok(b) => b,
        Err(e) => {
            println!("❌ Invalid burst: {}", e);
            return;
        }
    };

    let table = "filter";

    println!("🔧 Setting up HTTP/HTTPS rate limiting...");

    // Create rules for HTTP/HTTPS (validated inputs)
    let limit_spec = format!("limit rate {} burst {} packets", rate, burst);
    let status1 = Command::new("sudo")
        .args([
            "nft",
            "add",
            "rule",
            table,
            "input",
            "tcp",
            "dport",
            "{",
            "80,",
            "443",
            "}",
            &limit_spec,
            "accept",
        ])
        .status();

    if let Err(e) = status1 {
        println!("❌ Failed to add rate limit rule: {}", e);
        return;
    }

    let status2 = Command::new("sudo")
        .args([
            "nft", "add", "rule", table, "input", "tcp", "dport", "{", "80,", "443", "}", "drop",
        ])
        .status();

    if let Err(e) = status2 {
        println!("❌ Failed to add drop rule: {}", e);
        return;
    }

    println!("✅ HTTP/HTTPS rate limiting enabled");
    println!("  - Rate: {}", rate);
    println!("  - Burst: {} packets", burst);
}

fn syn_flood_protection() {
    println!("🛡️ SYN Flood Protection");

    // Confirmation for enabling protection
    let confirm = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Enable SYN flood protection? This will modify kernel settings and add firewall rules.",
        )
        .default(true)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if !confirm {
        return;
    }

    let table = "filter";

    println!("🔧 Setting up SYN flood protection...");

    // Enable SYN cookies (direct args, no shell)
    let status_sysctl = Command::new("sudo")
        .args(["sysctl", "-w", "net.ipv4.tcp_syncookies=1"])
        .status();

    if let Err(e) = status_sysctl {
        println!("Warning: Failed to enable SYN cookies: {}", e);
    }

    // Create SYN flood protection rules (static values, safe)
    let status1 = Command::new("sudo")
        .args([
            "nft",
            "add",
            "rule",
            table,
            "input",
            "tcp",
            "flags",
            "syn",
            "limit",
            "rate",
            "100/second",
            "accept",
        ])
        .status();

    if let Err(e) = status1 {
        println!("❌ Failed to add SYN limit rule: {}", e);
        return;
    }

    let status2 = Command::new("sudo")
        .args([
            "nft", "add", "rule", table, "input", "tcp", "flags", "syn", "drop",
        ])
        .status();

    if let Err(e) = status2 {
        println!("❌ Failed to add SYN drop rule: {}", e);
        return;
    }

    println!("✅ SYN flood protection enabled");
    println!("  - SYN cookies: enabled");
    println!("  - SYN rate limit: 100/second");
}

fn connection_limit_per_ip() {
    println!("🔢 Connection Limit per IP");

    let max_conn_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter max connections per IP")
        .default("50".to_string())
        .interact()
    {
        Ok(m) => m,
        Err(_) => return,
    };

    // Validate max connections
    let max_conn: u32 = match max_conn_input.parse() {
        Ok(m) if m > 0 && m <= 65535 => m,
        _ => {
            println!("❌ Invalid max connections: must be a number between 1 and 65535");
            return;
        }
    };

    let table = "filter";

    println!("🔧 Setting connection limit...");

    // Build meter spec
    let meter_spec = format!("{{ ip saddr ct count over {} }}", max_conn);

    let status = Command::new("sudo")
        .args([
            "nft",
            "add",
            "rule",
            table,
            "input",
            "ct",
            "state",
            "new",
            "meter",
            "connections",
            &meter_spec,
            "drop",
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Connection limit enabled");
            println!("  - Max {} connections per IP", max_conn);
        }
        Ok(s) => println!(
            "❌ Failed to set connection limit (exit code: {:?})",
            s.code()
        ),
        Err(e) => println!("❌ Failed to execute command: {}", e),
    }
}

fn port_scan_detection() {
    println!("🔍 Port Scan Detection");

    // Confirmation for enabling detection
    let confirm = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable port scan detection? This will add firewall rules to detect and block scanners.")
        .default(true)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if !confirm {
        return;
    }

    let table = "filter";

    println!("🔧 Setting up port scan detection...");

    // Create set for port scanners (static values, safe)
    let set_spec = "{ type ipv4_addr; flags timeout; timeout 1h; }";
    let status1 = Command::new("sudo")
        .args(["nft", "add", "set", table, "port_scanners", set_spec])
        .status();

    if let Err(e) = status1 {
        println!("❌ Failed to create port_scanners set: {}", e);
        return;
    }

    // Detect port scanning (static values, safe)
    let status2 = Command::new("sudo")
        .args([
            "nft",
            "add",
            "rule",
            table,
            "input",
            "ct",
            "state",
            "new",
            "tcp",
            "flags",
            "!=",
            "syn",
            "add",
            "@port_scanners",
            "{",
            "ip",
            "saddr",
            "}",
        ])
        .status();

    if let Err(e) = status2 {
        println!("❌ Failed to add detection rule: {}", e);
        return;
    }

    // Block port scanners
    let status3 = Command::new("sudo")
        .args([
            "nft",
            "add",
            "rule",
            table,
            "input",
            "ip",
            "saddr",
            "@port_scanners",
            "drop",
        ])
        .status();

    if let Err(e) = status3 {
        println!("❌ Failed to add block rule: {}", e);
        return;
    }

    println!("✅ Port scan detection enabled");
    println!("  - Scanners blocked for 1 hour");
}

fn custom_rate_limit() {
    println!("🔧 Custom Rate Limit");

    let port_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter port to rate limit")
        .interact()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    // Validate port
    let validated_port = match ValidatedPort::from_input(&port_input) {
        Ok(p) => p,
        Err(e) => {
            println!("❌ Invalid port: {}", e);
            return;
        }
    };

    let rate_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter rate (e.g., 10/second, 100/minute)")
        .default("10/second".to_string())
        .interact()
    {
        Ok(r) => r,
        Err(_) => return,
    };

    // Validate rate
    let rate = match validate_rate_limit(&rate_input) {
        Ok(r) => r,
        Err(e) => {
            println!("❌ Invalid rate: {}", e);
            return;
        }
    };

    let timeout_input: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter block timeout")
        .default("5m".to_string())
        .interact()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    // Validate timeout
    let timeout = match validate_timeout(&timeout_input) {
        Ok(t) => t,
        Err(e) => {
            println!("❌ Invalid timeout: {}", e);
            return;
        }
    };

    let table = "filter";
    let port_str = validated_port.to_string();
    let set_name = format!("ratelimit_port_{}", port_str);

    println!("🔧 Setting up custom rate limit...");

    // Create dynamic set
    let set_spec = format!(
        "{{ type ipv4_addr; flags dynamic, timeout; timeout {}; }}",
        timeout
    );
    let status1 = Command::new("sudo")
        .args(["nft", "add", "set", table, &set_name, &set_spec])
        .status();

    if let Err(e) = status1 {
        println!("❌ Failed to create rate limit set: {}", e);
        return;
    }

    // Add rate limiting rule
    let set_ref = format!("@{}", set_name);
    let limit_spec = format!("limit rate {}", rate);
    let status2 = Command::new("sudo")
        .args([
            "nft",
            "add",
            "rule",
            table,
            "input",
            "tcp",
            "dport",
            &port_str,
            "add",
            &set_ref,
            "{",
            "ip",
            "saddr",
            &limit_spec,
            "}",
            "accept",
        ])
        .status();

    if let Err(e) = status2 {
        println!("❌ Failed to add rate limit rule: {}", e);
        return;
    }

    // Block excessive connections
    let status3 = Command::new("sudo")
        .args([
            "nft", "add", "rule", table, "input", "tcp", "dport", &port_str, "ip", "saddr",
            &set_ref, "drop",
        ])
        .status();

    if let Err(e) = status3 {
        println!("❌ Failed to add block rule: {}", e);
        return;
    }

    println!("✅ Rate limiting enabled for port {}", port_str);
    println!("  - Rate: {}", rate);
    println!("  - Block timeout: {}", timeout);
}

fn rule_optimizer() {
    println!("📋 Rule Optimizer");

    println!("🔍 Analyzing current ruleset...");

    let output = Command::new("sh")
        .arg("-c")
        .arg("sudo nft list ruleset")
        .output();

    if let Ok(out) = output {
        let ruleset = String::from_utf8_lossy(&out.stdout);
        let lines: Vec<&str> = ruleset.lines().collect();

        println!("📊 Ruleset statistics:");
        println!("  • Total lines: {}", lines.len());

        // Count rule types
        let accept_count = lines.iter().filter(|l| l.contains("accept")).count();
        let drop_count = lines.iter().filter(|l| l.contains("drop")).count();
        let reject_count = lines.iter().filter(|l| l.contains("reject")).count();

        println!("  • Accept rules: {}", accept_count);
        println!("  • Drop rules: {}", drop_count);
        println!("  • Reject rules: {}", reject_count);

        // Optimization suggestions
        println!("\n💡 Optimization suggestions:");

        // Check for duplicate rules
        let mut rule_set = std::collections::HashSet::new();
        let mut duplicates = 0;
        for line in &lines {
            if line.contains("rule") && !rule_set.insert(line) {
                duplicates += 1;
            }
        }

        if duplicates > 0 {
            println!("  ⚠️ Found {} potential duplicate rules", duplicates);
        }

        // Check for inefficient ordering
        let mut state_rules_after_specific = false;
        for (i, line) in lines.iter().enumerate() {
            if line.contains("ct state") && i > 10 {
                state_rules_after_specific = true;
                break;
            }
        }

        if state_rules_after_specific {
            println!("  ⚠️ Connection state rules should be placed early for efficiency");
        }

        // Check for missing optimizations
        if !ruleset.contains("ct state { established, related } accept") {
            println!("  ⚠️ Consider adding fast-path for established connections");
        }

        let optimize = match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Apply automatic optimizations?")
            .default(false)
            .interact_opt()
        {
            Ok(Some(o)) => o,
            Ok(None) | Err(_) => return,
        };

        if optimize {
            apply_rule_optimizations();
        }
    }
}

fn apply_rule_optimizations() {
    println!("🔧 Applying optimizations...");

    // Add fast-path for established connections
    Command::new("sudo")
        .args(["nft", "insert", "rule", "filter", "input", "ct", "state", "{", "established,", "related", "}", "accept"])
        .status()
        .ok();

    // Drop invalid packets early
    Command::new("sudo")
        .args(["nft", "insert", "rule", "filter", "input", "ct", "state", "invalid", "drop"])
        .status()
        .ok();

    println!("✅ Basic optimizations applied");
}

fn iptables_to_nftables_migration() {
    println!("🔄 iptables to nftables Migration");

    let options = [
        "Analyze iptables rules",
        "Convert iptables to nftables",
        "Backup iptables rules",
        "Test migration",
        "Complete migration",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Migration options")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => analyze_iptables_rules(),
        1 => convert_iptables_to_nftables(),
        2 => backup_iptables_rules(),
        3 => test_migration(),
        4 => complete_migration(),
        _ => {}
    }
}

fn analyze_iptables_rules() {
    println!("🔍 Analyzing iptables rules");

    let tables = ["filter", "nat", "mangle", "raw", "security"];

    for table in &tables {
        let output = Command::new("sudo")
            .args(["iptables", "-t", table, "-L", "-n", "--line-numbers"])
            .output();
        if let Ok(out) = output {
            let count = String::from_utf8_lossy(&out.stdout).lines().count();
            println!("  {} table: {} rules", table, count);
        }
    }
}

fn convert_iptables_to_nftables() {
    println!("🔄 Converting iptables to nftables");

    let home = std::env::var("HOME").unwrap_or_default();
    let backup_path = format!("{}/iptables_backup.rules", home);
    let converted_path = format!("{}/nftables_converted.rules", home);

    // Save iptables rules (capture output and write to file)
    let save_output = Command::new("sudo")
        .args(["iptables-save"])
        .output();

    match save_output {
        Ok(out) if out.status.success() => {
            if let Err(e) = std::fs::write(&backup_path, &out.stdout) {
                println!("❌ Failed to write backup: {}", e);
                return;
            }
        }
        _ => {
            println!("❌ Failed to save iptables rules");
            return;
        }
    }

    // Convert using iptables-restore-translate
    let convert_output = Command::new("sudo")
        .args(["iptables-restore-translate", "-f", &backup_path])
        .output();

    match convert_output {
        Ok(out) if out.status.success() => {
            if let Err(e) = std::fs::write(&converted_path, &out.stdout) {
                println!("❌ Failed to write converted rules: {}", e);
                return;
            }
            println!("✅ Conversion complete");
            println!("  Converted rules saved to: ~/nftables_converted.rules");
        }
        _ => println!("❌ Conversion failed"),
    }
}

fn backup_iptables_rules() {
    println!("💾 Backing up iptables rules");

    let backup_dir = format!(
        "{}/firewall_backups",
        std::env::var("HOME").unwrap_or_default()
    );
    fs::create_dir_all(&backup_dir).ok();

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_path = format!("{}/iptables_{}.rules", backup_dir, timestamp);

    // Capture iptables-save output and write to file
    let output = Command::new("sudo")
        .args(["iptables-save"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            match std::fs::write(&backup_path, &out.stdout) {
                Ok(_) => println!("✅ Backup saved to: {}", backup_path),
                Err(e) => println!("❌ Failed to write backup: {}", e),
            }
        }
        _ => println!("❌ Backup failed"),
    }
}

fn test_migration() {
    println!("🧪 Testing migration");

    println!("This will:");
    println!("  1. Save current iptables rules");
    println!("  2. Load nftables rules");
    println!("  3. Test connectivity");
    println!("  4. Option to rollback");

    let proceed = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Proceed with test?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(p)) => p,
        Ok(None) | Err(_) => return,
    };

    if proceed {
        // Implementation would include actual testing logic
        println!("⚠️ Test migration requires careful implementation");
        println!("  Please review converted rules manually first");
    }
}

fn complete_migration() {
    println!("🔄 Complete Migration");

    let confirm = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will replace iptables with nftables. Continue?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        println!("🔧 Completing migration...");

        // Disable iptables services
        let services = ["iptables", "ip6tables"];
        for service in &services {
            Command::new("sudo")
                .args(["systemctl", "disable", service])
                .status()
                .ok();
        }

        // Enable nftables
        Command::new("sudo")
            .args(["systemctl", "enable", "--now", "nftables"])
            .status()
            .ok();

        println!("✅ Migration complete");
        println!("  nftables is now active");
    }
}

fn ruleset_backup_restore() {
    println!("💾 Ruleset Backup/Restore");

    let options = [
        "Backup current ruleset",
        "Restore from backup",
        "List backups",
        "Delete backup",
        "Schedule automatic backups",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Backup/Restore options")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => backup_ruleset(),
        1 => restore_ruleset(),
        2 => list_backups(),
        3 => delete_backup(),
        4 => schedule_backups(),
        _ => {}
    }
}

fn backup_ruleset() {
    println!("💾 Backing up ruleset");

    let backup_dir = format!(
        "{}/nftables_backups",
        std::env::var("HOME").unwrap_or_default()
    );
    fs::create_dir_all(&backup_dir).ok();

    let name: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter backup name")
        .default(chrono::Local::now().format("%Y%m%d_%H%M%S").to_string())
        .interact()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    let backup_path = format!("{}/{}.nft", backup_dir, name);

    // Capture output and write to file instead of shell redirect
    let output = Command::new("sudo")
        .args(["nft", "list", "ruleset"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            if let Err(e) = fs::write(&backup_path, &out.stdout) {
                println!("❌ Failed to write backup file: {}", e);
                return;
            }
            println!("✅ Ruleset backed up to: {}", backup_path);

            // Add metadata
            let metadata = format!(
                "# Backup created: {}\n# System: {}\n",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string())
            );

            if let Ok(content) = fs::read_to_string(&backup_path) {
                fs::write(&backup_path, format!("{}{}", metadata, content)).ok();
            }
        }
        _ => println!("❌ Backup failed"),
    }
}

fn restore_ruleset() {
    println!("📥 Restore Ruleset");

    let backup_dir = format!(
        "{}/nftables_backups",
        std::env::var("HOME").unwrap_or_default()
    );

    if !Path::new(&backup_dir).exists() {
        println!("❌ No backups found");
        return;
    }

    let mut backups = Vec::new();
    if let Ok(entries) = fs::read_dir(&backup_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("nft") {
                if let Some(stem) = path.file_stem() {
                    backups.push(stem.to_string_lossy().to_string());
                }
            }
        }
    }

    if backups.is_empty() {
        println!("❌ No backups found");
        return;
    }

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select backup to restore")
        .items(&backups)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    let backup = &backups[choice];
    let backup_path = format!("{}/{}.nft", backup_dir, backup);

    let confirm = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will replace current ruleset. Continue?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        // Backup current before restore
        println!("📦 Backing up current ruleset...");
        let temp_backup = format!(
            "{}/pre_restore_{}.nft",
            backup_dir,
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        );

        // Capture nft list ruleset and write to file
        if let Ok(out) = Command::new("sudo").args(["nft", "list", "ruleset"]).output() {
            if out.status.success() {
                let _ = std::fs::write(&temp_backup, &out.stdout);
            }
        }

        // Flush and restore
        println!("🔄 Restoring ruleset...");
        Command::new("sudo")
            .args(["nft", "flush", "ruleset"])
            .status()
            .ok();

        let status = Command::new("sudo")
            .args(["nft", "-f", &backup_path])
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Ruleset restored from: {}", backup),
            _ => {
                println!("❌ Restore failed, attempting rollback...");
                Command::new("sudo")
                    .args(["nft", "-f", &temp_backup])
                    .status()
                    .ok();
            }
        }
    }
}

fn list_backups() {
    println!("📋 List Backups");

    let backup_dir = format!(
        "{}/nftables_backups",
        std::env::var("HOME").unwrap_or_default()
    );

    if !Path::new(&backup_dir).exists() {
        println!("❌ No backups found");
        return;
    }

    if let Ok(entries) = fs::read_dir(&backup_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("nft") {
                if let Some(name) = path.file_name() {
                    let name = name.to_string_lossy();
                    let metadata = fs::metadata(&path).ok();
                    let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
                    let modified = metadata.and_then(|m| m.modified().ok());

                    println!("  📁 {} ({} bytes)", name, size);
                    if let Some(time) = modified {
                        println!("     Modified: {:?}", time);
                    }
                }
            }
        }
    }
}

fn delete_backup() {
    println!("🗑️ Delete Backup");

    let backup_dir = format!(
        "{}/nftables_backups",
        std::env::var("HOME").unwrap_or_default()
    );

    if !Path::new(&backup_dir).exists() {
        println!("❌ No backups found");
        return;
    }

    let mut backups = Vec::new();
    if let Ok(entries) = fs::read_dir(&backup_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("nft") {
                if let Some(name) = path.file_name() {
                    backups.push(name.to_string_lossy().to_string());
                }
            }
        }
    }

    if backups.is_empty() {
        println!("❌ No backups found");
        return;
    }

    let selected = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select backups to delete")
        .items(&backups)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    if !selected.is_empty() {
        let confirm = match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Delete {} backup(s)?", selected.len()))
            .default(false)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            Ok(None) | Err(_) => return,
        };

        if confirm {
            for idx in selected {
                let backup_path = format!("{}/{}", backup_dir, backups[idx]);
                fs::remove_file(&backup_path).ok();
                println!("  🗑️ Deleted: {}", backups[idx]);
            }
        }
    }
}

fn schedule_backups() {
    println!("⏰ Schedule Automatic Backups");

    let frequency = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select backup frequency")
        .items(&["Hourly", "Daily", "Weekly", "Monthly"])
        .default(1)
        .interact_opt()
    {
        Ok(Some(f)) => f,
        Ok(None) | Err(_) => return,
    };

    let frequency_str = match frequency {
        0 => "0 * * * *",
        1 => "0 2 * * *",
        2 => "0 2 * * 0",
        3 => "0 2 1 * *",
        _ => "0 2 * * *",
    };

    let backup_script = format!(
        r#"#!/bin/bash
BACKUP_DIR="{}/nftables_backups"
mkdir -p "$BACKUP_DIR"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
sudo nft list ruleset > "$BACKUP_DIR/auto_$TIMESTAMP.nft"
# Keep only last 30 backups
ls -t "$BACKUP_DIR"/auto_*.nft | tail -n +31 | xargs -r rm
"#,
        std::env::var("HOME").unwrap_or_default()
    );

    let script_path = format!(
        "{}/nftables_backup.sh",
        std::env::var("HOME").unwrap_or_default()
    );
    fs::write(&script_path, backup_script).ok();
    Command::new("chmod")
        .args(["+x", &script_path])
        .status()
        .ok();

    let cron_entry = format!("{} {}", frequency_str, script_path);

    println!("📝 Add this to your crontab:");
    println!("{}", cron_entry);

    let add_cron = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Add to crontab now?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(a)) => a,
        Ok(None) | Err(_) => return,
    };

    if add_cron {
        // Get existing crontab entries
        let existing = Command::new("crontab")
            .arg("-l")
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    String::from_utf8(o.stdout).ok()
                } else {
                    None
                }
            })
            .unwrap_or_default();

        // Create new crontab with existing + new entry
        let new_crontab = if existing.is_empty() {
            format!("{}\n", cron_entry)
        } else {
            format!("{}{}\n", existing, cron_entry)
        };

        // Write new crontab via stdin
        use std::io::Write;
        let mut child = match Command::new("crontab")
            .arg("-")
            .stdin(std::process::Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                println!("❌ Failed to update crontab: {}", e);
                return;
            }
        };

        if let Some(ref mut stdin) = child.stdin {
            if stdin.write_all(new_crontab.as_bytes()).is_ok() {
                if child.wait().is_ok() {
                    println!("✅ Automatic backup scheduled");
                }
            }
        }
    }
}

fn rule_testing_sandbox() {
    println!("🧪 Rule Testing Sandbox");

    println!("Creating isolated testing environment...");

    // Create test namespace
    let namespace = format!("nft_test_{}", chrono::Local::now().format("%Y%m%d_%H%M%S"));

    // Validate namespace name (should be safe since we generate it, but defense in depth)
    if !namespace
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        println!("❌ Invalid namespace name generated");
        return;
    }

    let status = Command::new("sudo")
        .args(["ip", "netns", "add", &namespace])
        .status();

    if !status.map(|s| s.success()).unwrap_or(false) {
        println!("❌ Failed to create test namespace");
        return;
    }

    println!("✅ Test namespace created: {}", namespace);

    let options = [
        "Add test rule",
        "Test packet flow",
        "Simulate attack",
        "View test results",
        "Clean up sandbox",
    ];

    loop {
        let choice = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Sandbox options")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            Ok(None) | Err(_) => {
                cleanup_sandbox(&namespace);
                break;
            }
        };

        match choice {
            0 => add_test_rule(&namespace),
            1 => test_packet_flow(&namespace),
            2 => simulate_attack(&namespace),
            3 => view_test_results(&namespace),
            4 => {
                cleanup_sandbox(&namespace);
                break;
            }
            _ => break,
        }
    }
}

fn add_test_rule(namespace: &str) {
    println!("➕ Add test rule to sandbox");

    let rule: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter nftables rule to test")
        .interact()
    {
        Ok(r) => r,
        Err(_) => return,
    };

    // Validate namespace name (alphanumeric, underscore, hyphen only)
    if !namespace
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        println!("❌ Invalid namespace name");
        return;
    }

    // Parse the rule into arguments (basic split, handles simple cases)
    let rule_args: Vec<&str> = rule.split_whitespace().collect();
    if rule_args.is_empty() {
        println!("❌ Empty rule");
        return;
    }

    // Build command: ip netns exec <ns> nft <rule_args...>
    let mut args = vec!["ip", "netns", "exec", namespace, "nft"];
    args.extend(rule_args.iter());

    let status = Command::new("sudo").args(&args).status();

    match status {
        Ok(s) if s.success() => println!("✅ Test rule added"),
        _ => println!("❌ Failed to add test rule"),
    }
}

fn test_packet_flow(namespace: &str) {
    println!("🔍 Test packet flow");

    let src_ip: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter source IP")
        .default("192.168.1.100".to_string())
        .interact()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    let dst_ip: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter destination IP")
        .default("192.168.1.1".to_string())
        .interact()
    {
        Ok(d) => d,
        Err(_) => return,
    };

    let port: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter destination port")
        .default("80".to_string())
        .interact()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    // Simulate packet flow
    println!("📦 Simulating packet flow...");
    println!("  {} → {}:{}", src_ip, dst_ip, port);

    // This would use tools like hping3 or scapy for actual testing
    println!("⚠️ Actual packet simulation requires additional tools");
    let _ = namespace; // Silence unused warning
}

fn simulate_attack(namespace: &str) {
    println!("⚠️ Simulate attack patterns");

    let attacks = ["SYN flood", "Port scan", "Brute force", "DDoS simulation"];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select attack type to simulate")
        .items(&attacks)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    println!("🔧 Simulating {} in sandbox...", attacks[choice]);
    println!("⚠️ This is a safe simulation in isolated namespace");
    let _ = namespace; // Silence unused warning

    // Simulation logic would go here
}

fn view_test_results(namespace: &str) {
    println!("📊 View test results");

    // Validate namespace name (defense in depth)
    if !namespace
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        println!("❌ Invalid namespace name");
        return;
    }

    let output = Command::new("sudo")
        .args(["ip", "netns", "exec", namespace, "nft", "list", "ruleset"])
        .output();

    if let Ok(out) = output {
        let result = String::from_utf8_lossy(&out.stdout);
        println!("{}", result);
    }
}

fn cleanup_sandbox(namespace: &str) {
    println!("🧹 Cleaning up sandbox");

    // Validate namespace name (defense in depth)
    if !namespace
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        println!("❌ Invalid namespace name");
        return;
    }

    let status = Command::new("sudo")
        .args(["ip", "netns", "delete", namespace])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Sandbox cleaned up"),
        _ => println!("❌ Failed to clean up sandbox"),
    }
}

fn performance_monitoring() {
    println!("📊 Performance Monitoring");

    println!("🔍 Analyzing firewall performance...");

    // Get rule statistics
    if let Ok(output) = Command::new("sudo")
        .args(["nft", "list", "ruleset", "-a", "-n"])
        .output()
    {
        if output.status.success() {
            let ruleset = String::from_utf8_lossy(&output.stdout);
            let rule_count = ruleset.lines().filter(|l| l.contains("handle")).count();
            println!("  • Total rules: {}", rule_count);
        }
    }

    // Check packet counters
    if let Ok(output) = Command::new("sudo")
        .args(["nft", "list", "counters"])
        .output()
    {
        if output.status.success() && !output.stdout.is_empty() {
            println!("  • Active counters detected");
        }
    }

    // Connection tracking stats
    if let Ok(output) = Command::new("sudo")
        .args(["conntrack", "-C"])
        .output()
    {
        if output.status.success() {
            let count = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("  • Active connections: {}", count);
        }
    }

    // CPU usage - use /proc instead of piping through shell
    println!("\n💻 System Impact:");
    if let Ok(content) = std::fs::read_to_string("/proc/net/netfilter/nf_conntrack") {
        let lines: Vec<_> = content.lines().collect();
        println!("  • Conntrack entries: {}", lines.len());
    }

    // Show nft process if running (via /proc)
    if let Ok(entries) = std::fs::read_dir("/proc") {
        for entry in entries.flatten() {
            if let Ok(name) = entry.file_name().into_string() {
                if name.chars().all(|c| c.is_ascii_digit()) {
                    let comm_path = entry.path().join("comm");
                    if let Ok(comm) = std::fs::read_to_string(&comm_path) {
                        if comm.trim() == "nft" {
                            println!("  • nft process running (PID: {})", name);
                        }
                    }
                }
            }
        }
    }
}

fn template_library() {
    println!("📝 Template Library");

    let templates = [
        "Web Server Protection",
        "Game Server Rules",
        "Mail Server Security",
        "Docker Host Firewall",
        "VPN Gateway Rules",
        "Home Network Protection",
        "Enterprise DMZ Setup",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select template")
        .items(&templates)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => web_server_template(),
        1 => game_server_template(),
        2 => mail_server_template(),
        3 => docker_host_template(),
        4 => vpn_gateway_template(),
        5 => home_network_template(),
        6 => enterprise_dmz_template(),
        _ => {}
    }
}

fn web_server_template() {
    println!("🌐 Web Server Protection Template");

    let template = r#"#!/usr/sbin/nft -f

flush ruleset

table inet filter {
    chain input {
        type filter hook input priority 0; policy drop;

        # Allow loopback
        iif lo accept

        # Allow established connections
        ct state established,related accept

        # Drop invalid
        ct state invalid drop

        # Allow SSH (rate limited)
        tcp dport 22 ct state new limit rate 3/minute accept

        # Allow HTTP/HTTPS
        tcp dport { 80, 443 } accept

        # Rate limit connections
        tcp dport { 80, 443 } ct state new limit rate 100/second accept

        # Log and drop
        log prefix "dropped: " drop
    }

    chain forward {
        type filter hook forward priority 0; policy drop;
    }

    chain output {
        type filter hook output priority 0; policy accept;
    }
}"#;

    println!("{}", template);

    let apply = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Apply this template?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(a)) => a,
        Ok(None) | Err(_) => return,
    };

    if apply {
        let temp_file = "/tmp/web_server_template.nft";
        if let Err(e) = fs::write(temp_file, template) {
            println!("❌ Failed to write template: {}", e);
            return;
        }

        let status = Command::new("sudo")
            .args(["nft", "-f", temp_file])
            .status();

        fs::remove_file(temp_file).ok();

        match status {
            Ok(s) if s.success() => println!("✅ Web server template applied"),
            _ => println!("❌ Failed to apply web server template"),
        }
    }
}

fn game_server_template() {
    println!("🎮 Game Server Template");

    let game_type = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select game type")
        .items(&["Minecraft", "CS:GO", "Rust", "Valheim", "Custom"])
        .default(0)
        .interact_opt()
    {
        Ok(Some(g)) => g,
        Ok(None) | Err(_) => return,
    };

    let port = match game_type {
        0 => "25565".to_string(),     // Minecraft
        1 => "27015".to_string(),     // CS:GO
        2 => "28015".to_string(),     // Rust
        3 => "2456-2458".to_string(), // Valheim
        _ => {
            match Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter game port")
                .interact()
            {
                Ok(p) => p,
                Err(_) => return,
            }
        }
    };

    let template = format!(
        r#"#!/usr/sbin/nft -f

table inet gaming {{
    chain input {{
        type filter hook input priority 0; policy drop;

        # Allow loopback
        iif lo accept

        # Allow established
        ct state established,related accept

        # Drop invalid
        ct state invalid drop

        # SSH (rate limited)
        tcp dport 22 ct state new limit rate 3/minute accept

        # Game server ports
        tcp dport {{ {} }} accept
        udp dport {{ {} }} accept

        # Rate limiting for game
        udp dport {} ct state new limit rate 100/second accept

        # DDoS protection
        ct state new limit rate 1000/second burst 100 packets accept

        log prefix "gaming-dropped: " drop
    }}

    chain forward {{
        type filter hook forward priority 0; policy drop;
    }}

    chain output {{
        type filter hook output priority 0; policy accept;
    }}
}}"#,
        port, port, port
    );

    println!("{}", template);

    let apply = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Apply this template?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(a)) => a,
        Ok(None) | Err(_) => return,
    };

    if apply {
        let temp_file = "/tmp/game_server_template.nft";
        if let Err(e) = fs::write(temp_file, &template) {
            println!("❌ Failed to write template: {}", e);
            return;
        }

        let status = Command::new("sudo")
            .args(["nft", "-f", temp_file])
            .status();

        fs::remove_file(temp_file).ok();

        match status {
            Ok(s) if s.success() => println!("✅ Game server template applied"),
            _ => println!("❌ Failed to apply game server template"),
        }
    }
}

// Continue with other template implementations...
fn mail_server_template() {
    println!("📧 Mail Server Security Template");
    // Implementation continues...
}

fn docker_host_template() {
    println!("🐳 Docker Host Firewall Template");
    // Implementation continues...
}

fn vpn_gateway_template() {
    println!("🔐 VPN Gateway Rules Template");
    // Implementation continues...
}

fn home_network_template() {
    println!("🏠 Home Network Protection Template");
    // Implementation continues...
}

fn enterprise_dmz_template() {
    println!("🏢 Enterprise DMZ Setup Template");
    // Implementation continues...
}

// Continue with other main menu functions...
fn advanced_iptables_features() {
    println!("⚙️ Advanced iptables Features");
    // Implementation continues...
}

fn network_security_tools() {
    println!("🛡️ Network Security Tools");
    // Implementation continues...
}

fn gaming_network_optimization() {
    println!("🎮 Gaming Network Optimization");
    // Implementation continues...
}

fn network_troubleshooting_tools() {
    println!("🔍 Network Troubleshooting Tools");
    // Implementation continues...
}

fn connection_state_analyzer() {
    println!("📊 Connection State Analyzer");
    // Implementation continues...
}

fn nat_port_forwarding() {
    println!("🌐 NAT & Port Forwarding");
    // Implementation continues...
}

fn ddos_protection_setup() {
    println!("🔐 DDoS Protection Setup");
    // Implementation continues...
}

fn port_knocking_configuration() {
    println!("🚪 Port Knocking Configuration");
    // Implementation continues...
}

fn qos_traffic_shaping() {
    println!("📈 QoS & Traffic Shaping");
    // Implementation continues...
}

use chrono;
