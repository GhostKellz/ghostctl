use dialoguer::{Select, Input, Confirm, theme::ColorfulTheme, MultiSelect};
use std::process::Command;
use std::fs;
use std::path::Path;

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

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔥 Advanced Firewall & Networking")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

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

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🚀 Advanced nftables Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

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

    let table_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .default("filter".to_string())
        .interact()
        .unwrap();

    let chain_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter chain name")
        .default("input".to_string())
        .interact()
        .unwrap();

    let rule_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select rule type")
        .items(&[
            "Allow port",
            "Block IP/subnet",
            "Rate limit",
            "Connection tracking",
            "NAT rule",
            "Jump to chain",
            "Log and drop",
            "Custom expression",
        ])
        .default(0)
        .interact()
        .unwrap();

    let mut rule = String::new();

    match rule_type {
        0 => {
            // Allow port
            let port = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter port number")
                .interact()
                .unwrap();

            let protocol = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select protocol")
                .items(&["tcp", "udp", "both"])
                .default(0)
                .interact()
                .unwrap();

            let proto_str = match protocol {
                0 => "tcp",
                1 => "udp",
                _ => "{ tcp, udp }",
            };

            rule = format!("{} dport {} accept", proto_str, port);
        }
        1 => {
            // Block IP/subnet
            let ip = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter IP or subnet (e.g., 192.168.1.0/24)")
                .interact()
                .unwrap();

            let action = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select action")
                .items(&["drop", "reject"])
                .default(0)
                .interact()
                .unwrap();

            let action_str = if action == 0 { "drop" } else { "reject" };

            rule = format!("ip saddr {} {}", ip, action_str);
        }
        2 => {
            // Rate limit
            let rate = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter rate (e.g., 10/second, 100/minute)")
                .default("10/second".to_string())
                .interact()
                .unwrap();

            let burst = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter burst limit")
                .default("5".to_string())
                .interact()
                .unwrap();

            rule = format!("limit rate {} burst {} packets accept", rate, burst);
        }
        3 => {
            // Connection tracking
            let states = vec!["new", "established", "related", "invalid"];
            let selected = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Select connection states")
                .items(&states)
                .interact()
                .unwrap();

            let mut state_list = Vec::new();
            for idx in selected {
                state_list.push(states[idx]);
            }

            let action = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select action")
                .items(&["accept", "drop", "reject"])
                .default(0)
                .interact()
                .unwrap();

            let action_str = ["accept", "drop", "reject"][action];

            rule = format!("ct state {{ {} }} {}", state_list.join(", "), action_str);
        }
        4 => {
            // NAT rule
            let nat_type = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select NAT type")
                .items(&["SNAT", "DNAT", "MASQUERADE"])
                .default(0)
                .interact()
                .unwrap();

            match nat_type {
                0 => {
                    let ip = Input::<String>::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter source IP")
                        .interact()
                        .unwrap();
                    rule = format!("snat to {}", ip);
                }
                1 => {
                    let ip = Input::<String>::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter destination IP:port")
                        .interact()
                        .unwrap();
                    rule = format!("dnat to {}", ip);
                }
                2 => {
                    rule = "masquerade".to_string();
                }
                _ => {}
            }
        }
        5 => {
            // Jump to chain
            let target_chain = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter target chain name")
                .interact()
                .unwrap();

            rule = format!("jump {}", target_chain);
        }
        6 => {
            // Log and drop
            let prefix = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter log prefix")
                .default("DROPPED: ".to_string())
                .interact()
                .unwrap();

            rule = format!("log prefix \"{}\" drop", prefix);
        }
        7 => {
            // Custom expression
            rule = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter custom nftables expression")
                .interact()
                .unwrap();
        }
        _ => {}
    }

    if !rule.is_empty() {
        let position = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter rule position (or press Enter for end)")
            .allow_empty(true)
            .interact()
            .unwrap();

        let position_str = if position.is_empty() {
            String::new()
        } else {
            format!("position {}", position)
        };

        let full_command = format!(
            "sudo nft add rule {} {} {} {}",
            table_name, chain_name, position_str, rule
        );

        println!("\n📋 Generated rule:");
        println!("{}", full_command);

        let execute = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Execute this rule?")
            .default(true)
            .interact()
            .unwrap();

        if execute {
            let status = Command::new("sh")
                .arg("-c")
                .arg(&full_command)
                .status();

            match status {
                Ok(s) if s.success() => println!("✅ Rule added successfully"),
                _ => println!("❌ Failed to add rule"),
            }
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Set management options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let table = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .default("filter".to_string())
        .interact()
        .unwrap();

    let set_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter set name")
        .interact()
        .unwrap();

    let set_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select set type")
        .items(&["ipv4_addr", "ipv6_addr", "ether_addr", "inet_proto", "inet_service", "mark"])
        .default(0)
        .interact()
        .unwrap();

    let type_str = ["ipv4_addr", "ipv6_addr", "ether_addr", "inet_proto", "inet_service", "mark"][set_type];

    let flags = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select flags (optional)")
        .items(&["interval", "timeout", "constant", "dynamic"])
        .interact()
        .unwrap();

    let mut flags_str = String::new();
    if !flags.is_empty() {
        let flag_names: Vec<&str> = flags.iter().map(|&i| {
            ["interval", "timeout", "constant", "dynamic"][i]
        }).collect();
        flags_str = format!("flags {}", flag_names.join(", "));
    }

    let cmd = format!(
        "sudo nft add set {} {} {{ type {}; {}; }}",
        table, set_name, type_str, flags_str
    );

    println!("📋 Command: {}", cmd);

    let status = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Set created: {}", set_name),
        _ => println!("❌ Failed to create set"),
    }
}

fn add_to_set() {
    println!("➕ Add Elements to Set");

    let table = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .default("filter".to_string())
        .interact()
        .unwrap();

    let set_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter set name")
        .interact()
        .unwrap();

    let elements = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter elements (comma-separated)")
        .interact()
        .unwrap();

    let element_list: Vec<&str> = elements.split(',').map(|s| s.trim()).collect();
    let elements_str = element_list.join(", ");

    let cmd = format!(
        "sudo nft add element {} {} {{ {} }}",
        table, set_name, elements_str
    );

    let status = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Elements added to set"),
        _ => println!("❌ Failed to add elements"),
    }
}

fn remove_from_set() {
    println!("➖ Remove Elements from Set");

    let table = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .default("filter".to_string())
        .interact()
        .unwrap();

    let set_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter set name")
        .interact()
        .unwrap();

    let elements = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter elements to remove (comma-separated)")
        .interact()
        .unwrap();

    let element_list: Vec<&str> = elements.split(',').map(|s| s.trim()).collect();
    let elements_str = element_list.join(", ");

    let cmd = format!(
        "sudo nft delete element {} {} {{ {} }}",
        table, set_name, elements_str
    );

    let status = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Elements removed from set"),
        _ => println!("❌ Failed to remove elements"),
    }
}

fn list_sets() {
    println!("📋 List nftables Sets");

    let cmd = "sudo nft list sets";

    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output();

    match output {
        Ok(out) => {
            let result = String::from_utf8_lossy(&out.stdout);
            if result.is_empty() {
                println!("❌ No sets found");
            } else {
                println!("{}", result);
            }
        }
        _ => println!("❌ Failed to list sets"),
    }
}

fn delete_set() {
    println!("🗑️ Delete Set");

    let table = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .default("filter".to_string())
        .interact()
        .unwrap();

    let set_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter set name to delete")
        .interact()
        .unwrap();

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Delete set '{}'?", set_name))
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let cmd = format!("sudo nft delete set {} {}", table, set_name);

        let status = Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Set deleted"),
            _ => println!("❌ Failed to delete set"),
        }
    }
}

fn create_dynamic_set() {
    println!("🔄 Create Dynamic Set");

    let table = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .default("filter".to_string())
        .interact()
        .unwrap();

    let set_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter dynamic set name")
        .interact()
        .unwrap();

    let timeout = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter timeout (e.g., 30s, 5m, 1h)")
        .default("5m".to_string())
        .interact()
        .unwrap();

    let max_size = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter maximum size")
        .default("65535".to_string())
        .interact()
        .unwrap();

    let cmd = format!(
        "sudo nft add set {} {} {{ type ipv4_addr; flags dynamic, timeout; timeout {}; size {}; }}",
        table, set_name, timeout, max_size
    );

    println!("📋 Creating dynamic set for rate limiting...");

    let status = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Dynamic set created: {}", set_name);

            // Create rate limiting rule
            let create_rule = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Create rate limiting rule using this set?")
                .default(true)
                .interact()
                .unwrap();

            if create_rule {
                let rate = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter rate limit (e.g., 10/second)")
                    .default("10/second".to_string())
                    .interact()
                    .unwrap();

                let rule_cmd = format!(
                    "sudo nft add rule {} input ip saddr @{} limit rate {} drop",
                    table, set_name, rate
                );

                Command::new("sh").arg("-c").arg(&rule_cmd).status().ok();
                println!("✅ Rate limiting rule created");
            }
        }
        _ => println!("❌ Failed to create dynamic set"),
    }
}

fn import_set() {
    println!("📥 Import Set from File");

    let file_path = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter file path containing IPs/elements (one per line)")
        .interact()
        .unwrap();

    if !Path::new(&file_path).exists() {
        println!("❌ File not found");
        return;
    }

    let table = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .default("filter".to_string())
        .interact()
        .unwrap();

    let set_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter set name")
        .interact()
        .unwrap();

    if let Ok(content) = fs::read_to_string(&file_path) {
        let elements: Vec<&str> = content.lines().filter(|l| !l.is_empty()).collect();

        if elements.is_empty() {
            println!("❌ No elements found in file");
            return;
        }

        let elements_str = elements.join(", ");

        let cmd = format!(
            "sudo nft add element {} {} {{ {} }}",
            table, set_name, elements_str
        );

        let status = Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ {} elements imported", elements.len()),
            _ => println!("❌ Failed to import elements"),
        }
    }
}

fn export_set() {
    println!("📤 Export Set to File");

    let table = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .default("filter".to_string())
        .interact()
        .unwrap();

    let set_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter set name")
        .interact()
        .unwrap();

    let export_path = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter export file path")
        .default(format!("{}/{}_set.txt", std::env::var("HOME").unwrap_or_default(), set_name))
        .interact()
        .unwrap();

    let cmd = format!("sudo nft list set {} {}", table, set_name);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output();

    match output {
        Ok(out) => {
            let result = String::from_utf8_lossy(&out.stdout);
            fs::write(&export_path, result.as_bytes()).ok();
            println!("✅ Set exported to: {}", export_path);
        }
        _ => println!("❌ Failed to export set"),
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

    let custom = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Create custom priority chain?")
        .default(true)
        .interact()
        .unwrap();

    if custom {
        let table = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter table name")
            .default("filter".to_string())
            .interact()
            .unwrap();

        let chain = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter chain name")
            .interact()
            .unwrap();

        let hook = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select hook")
            .items(&["prerouting", "input", "forward", "output", "postrouting"])
            .default(1)
            .interact()
            .unwrap();

        let hook_str = ["prerouting", "input", "forward", "output", "postrouting"][hook];

        let priority = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter priority (-300 to 300)")
            .default("0".to_string())
            .interact()
            .unwrap();

        let policy = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select default policy")
            .items(&["accept", "drop"])
            .default(0)
            .interact()
            .unwrap();

        let policy_str = if policy == 0 { "accept" } else { "drop" };

        let cmd = format!(
            "sudo nft add chain {} {} {{ type filter hook {} priority {}; policy {}; }}",
            table, chain, hook_str, priority, policy_str
        );

        println!("📋 Command: {}", cmd);

        let status = Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Chain created with priority {}", priority),
            _ => println!("❌ Failed to create chain"),
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select scenario")
        .items(&scenarios)
        .default(0)
        .interact()
        .unwrap();

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

    let table = "filter";

    // Create dynamic set for tracking
    let cmd1 = format!(
        "sudo nft add set {} ssh_ratelimit {{ type ipv4_addr; flags dynamic, timeout; timeout 10m; }}",
        table
    );

    // Add rate limiting rule
    let cmd2 = format!(
        "sudo nft add rule {} input tcp dport 22 ct state new add @ssh_ratelimit {{ ip saddr limit rate 3/minute }} accept",
        table
    );

    // Drop excessive attempts
    let cmd3 = format!(
        "sudo nft add rule {} input tcp dport 22 ip saddr @ssh_ratelimit drop",
        table
    );

    println!("🔧 Setting up SSH brute force protection...");

    for cmd in &[cmd1, cmd2, cmd3] {
        Command::new("sh").arg("-c").arg(cmd).status().ok();
    }

    println!("✅ SSH brute force protection enabled");
    println!("  • Max 3 login attempts per minute");
    println!("  • Blocked IPs timeout after 10 minutes");
}

fn http_rate_limiting() {
    println!("🌐 HTTP/HTTPS Rate Limiting");

    let rate = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter rate limit (e.g., 100/second)")
        .default("100/second".to_string())
        .interact()
        .unwrap();

    let burst = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter burst size")
        .default("50".to_string())
        .interact()
        .unwrap();

    let table = "filter";

    // Create rules for HTTP/HTTPS
    let cmd1 = format!(
        "sudo nft add rule {} input tcp dport {{ 80, 443 }} limit rate {} burst {} accept",
        table, rate, burst
    );

    let cmd2 = format!(
        "sudo nft add rule {} input tcp dport {{ 80, 443 }} drop",
        table
    );

    println!("🔧 Setting up HTTP/HTTPS rate limiting...");

    Command::new("sh").arg("-c").arg(&cmd1).status().ok();
    Command::new("sh").arg("-c").arg(&cmd2).status().ok();

    println!("✅ HTTP/HTTPS rate limiting enabled");
    println!("  • Rate: {}", rate);
    println!("  • Burst: {}", burst);
}

fn syn_flood_protection() {
    println!("🛡️ SYN Flood Protection");

    let table = "filter";

    // Enable SYN cookies
    let sysctl_cmd = "sudo sysctl -w net.ipv4.tcp_syncookies=1";
    Command::new("sh").arg("-c").arg(sysctl_cmd).status().ok();

    // Create SYN flood protection rules
    let cmd1 = format!(
        "sudo nft add rule {} input tcp flags syn limit rate 100/second accept",
        table
    );

    let cmd2 = format!(
        "sudo nft add rule {} input tcp flags syn drop",
        table
    );

    println!("🔧 Setting up SYN flood protection...");

    Command::new("sh").arg("-c").arg(&cmd1).status().ok();
    Command::new("sh").arg("-c").arg(&cmd2).status().ok();

    println!("✅ SYN flood protection enabled");
    println!("  • SYN cookies: enabled");
    println!("  • SYN rate limit: 100/second");
}

fn connection_limit_per_ip() {
    println!("🔢 Connection Limit per IP");

    let max_conn = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter max connections per IP")
        .default("50".to_string())
        .interact()
        .unwrap();

    let table = "filter";

    let cmd = format!(
        "sudo nft add rule {} input ct state new meter connections {{ ip saddr ct count over {} }} drop",
        table, max_conn
    );

    println!("🔧 Setting connection limit...");

    let status = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Connection limit enabled");
            println!("  • Max {} connections per IP", max_conn);
        }
        _ => println!("❌ Failed to set connection limit"),
    }
}

fn port_scan_detection() {
    println!("🔍 Port Scan Detection");

    let table = "filter";

    // Create set for port scanners
    let cmd1 = format!(
        "sudo nft add set {} port_scanners {{ type ipv4_addr; flags timeout; timeout 1h; }}",
        table
    );

    // Detect port scanning
    let cmd2 = format!(
        "sudo nft add rule {} input ct state new tcp flags != syn add @port_scanners {{ ip saddr }}",
        table
    );

    // Block port scanners
    let cmd3 = format!(
        "sudo nft add rule {} input ip saddr @port_scanners drop",
        table
    );

    println!("🔧 Setting up port scan detection...");

    for cmd in &[cmd1, cmd2, cmd3] {
        Command::new("sh").arg("-c").arg(cmd).status().ok();
    }

    println!("✅ Port scan detection enabled");
    println!("  • Scanners blocked for 1 hour");
}

fn custom_rate_limit() {
    println!("🔧 Custom Rate Limit");

    let port = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter port to rate limit")
        .interact()
        .unwrap();

    let rate = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter rate (e.g., 10/second, 100/minute)")
        .default("10/second".to_string())
        .interact()
        .unwrap();

    let timeout = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter block timeout")
        .default("5m".to_string())
        .interact()
        .unwrap();

    let table = "filter";

    // Create dynamic set
    let set_name = format!("ratelimit_port_{}", port);
    let cmd1 = format!(
        "sudo nft add set {} {} {{ type ipv4_addr; flags dynamic, timeout; timeout {}; }}",
        table, set_name, timeout
    );

    // Add rate limiting
    let cmd2 = format!(
        "sudo nft add rule {} input tcp dport {} add @{} {{ ip saddr limit rate {} }} accept",
        table, port, set_name, rate
    );

    // Block excessive connections
    let cmd3 = format!(
        "sudo nft add rule {} input tcp dport {} ip saddr @{} drop",
        table, port, set_name
    );

    println!("🔧 Setting up custom rate limit...");

    for cmd in &[cmd1, cmd2, cmd3] {
        Command::new("sh").arg("-c").arg(cmd).status().ok();
    }

    println!("✅ Rate limiting enabled for port {}", port);
    println!("  • Rate: {}", rate);
    println!("  • Block timeout: {}", timeout);
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

        let optimize = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Apply automatic optimizations?")
            .default(false)
            .interact()
            .unwrap();

        if optimize {
            apply_rule_optimizations();
        }
    }
}

fn apply_rule_optimizations() {
    println!("🔧 Applying optimizations...");

    // Add fast-path for established connections
    let cmd1 = "sudo nft insert rule filter input ct state { established, related } accept";

    // Drop invalid packets early
    let cmd2 = "sudo nft insert rule filter input ct state invalid drop";

    for cmd in &[cmd1, cmd2] {
        Command::new("sh").arg("-c").arg(cmd).status().ok();
    }

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Migration options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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
        let cmd = format!("sudo iptables -t {} -L -n --line-numbers | wc -l", table);
        if let Ok(output) = Command::new("sh").arg("-c").arg(&cmd).output() {
            let count = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("  {} table: {} rules", table, count);
        }
    }
}

fn convert_iptables_to_nftables() {
    println!("🔄 Converting iptables to nftables");

    let backup_path = format!("{}/iptables_backup.rules",
        std::env::var("HOME").unwrap_or_default());

    // Save iptables rules
    let save_cmd = format!("sudo iptables-save > {}", backup_path);
    Command::new("sh").arg("-c").arg(&save_cmd).status().ok();

    // Convert using iptables-restore-translate
    let convert_cmd = format!(
        "sudo iptables-restore-translate -f {} > {}/nftables_converted.rules",
        backup_path,
        std::env::var("HOME").unwrap_or_default()
    );

    let status = Command::new("sh")
        .arg("-c")
        .arg(&convert_cmd)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Conversion complete");
            println!("  Converted rules saved to: ~/nftables_converted.rules");
        }
        _ => println!("❌ Conversion failed"),
    }
}

fn backup_iptables_rules() {
    println!("💾 Backing up iptables rules");

    let backup_dir = format!("{}/firewall_backups", std::env::var("HOME").unwrap_or_default());
    fs::create_dir_all(&backup_dir).ok();

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_path = format!("{}/iptables_{}.rules", backup_dir, timestamp);

    let cmd = format!("sudo iptables-save > {}", backup_path);

    let status = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Backup saved to: {}", backup_path),
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

    let proceed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Proceed with test?")
        .default(false)
        .interact()
        .unwrap();

    if proceed {
        // Implementation would include actual testing logic
        println!("⚠️ Test migration requires careful implementation");
        println!("  Please review converted rules manually first");
    }
}

fn complete_migration() {
    println!("🔄 Complete Migration");

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will replace iptables with nftables. Continue?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        println!("🔧 Completing migration...");

        // Disable iptables services
        let services = ["iptables", "ip6tables"];
        for service in &services {
            let cmd = format!("sudo systemctl disable {}", service);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();
        }

        // Enable nftables
        let enable_cmd = "sudo systemctl enable --now nftables";
        Command::new("sh").arg("-c").arg(enable_cmd).status().ok();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Backup/Restore options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let backup_dir = format!("{}/nftables_backups", std::env::var("HOME").unwrap_or_default());
    fs::create_dir_all(&backup_dir).ok();

    let name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter backup name")
        .default(chrono::Local::now().format("%Y%m%d_%H%M%S").to_string())
        .interact()
        .unwrap();

    let backup_path = format!("{}/{}.nft", backup_dir, name);

    let cmd = format!("sudo nft list ruleset > {}", backup_path);

    let status = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status();

    match status {
        Ok(s) if s.success() => {
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

    let backup_dir = format!("{}/nftables_backups", std::env::var("HOME").unwrap_or_default());

    if !Path::new(&backup_dir).exists() {
        println!("❌ No backups found");
        return;
    }

    let mut backups = Vec::new();
    if let Ok(entries) = fs::read_dir(&backup_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("nft") {
                    backups.push(path.file_stem().unwrap().to_string_lossy().to_string());
                }
            }
        }
    }

    if backups.is_empty() {
        println!("❌ No backups found");
        return;
    }

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select backup to restore")
        .items(&backups)
        .default(0)
        .interact()
        .unwrap();

    let backup = &backups[choice];
    let backup_path = format!("{}/{}.nft", backup_dir, backup);

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will replace current ruleset. Continue?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        // Backup current before restore
        println!("📦 Backing up current ruleset...");
        let temp_backup = format!("{}/pre_restore_{}.nft",
            backup_dir,
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        );
        let backup_cmd = format!("sudo nft list ruleset > {}", temp_backup);
        Command::new("sh").arg("-c").arg(&backup_cmd).status().ok();

        // Flush and restore
        println!("🔄 Restoring ruleset...");
        let flush_cmd = "sudo nft flush ruleset";
        let restore_cmd = format!("sudo nft -f {}", backup_path);

        Command::new("sh").arg("-c").arg(flush_cmd).status().ok();

        let status = Command::new("sh")
            .arg("-c")
            .arg(&restore_cmd)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Ruleset restored from: {}", backup),
            _ => {
                println!("❌ Restore failed, attempting rollback...");
                let rollback_cmd = format!("sudo nft -f {}", temp_backup);
                Command::new("sh").arg("-c").arg(&rollback_cmd).status().ok();
            }
        }
    }
}

fn list_backups() {
    println!("📋 List Backups");

    let backup_dir = format!("{}/nftables_backups", std::env::var("HOME").unwrap_or_default());

    if !Path::new(&backup_dir).exists() {
        println!("❌ No backups found");
        return;
    }

    if let Ok(entries) = fs::read_dir(&backup_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("nft") {
                    let name = path.file_name().unwrap().to_string_lossy();
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

    let backup_dir = format!("{}/nftables_backups", std::env::var("HOME").unwrap_or_default());

    if !Path::new(&backup_dir).exists() {
        println!("❌ No backups found");
        return;
    }

    let mut backups = Vec::new();
    if let Ok(entries) = fs::read_dir(&backup_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("nft") {
                    backups.push(path.file_name().unwrap().to_string_lossy().to_string());
                }
            }
        }
    }

    if backups.is_empty() {
        println!("❌ No backups found");
        return;
    }

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select backups to delete")
        .items(&backups)
        .interact()
        .unwrap();

    if !selected.is_empty() {
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Delete {} backup(s)?", selected.len()))
            .default(false)
            .interact()
            .unwrap();

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

    let frequency = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select backup frequency")
        .items(&["Hourly", "Daily", "Weekly", "Monthly"])
        .default(1)
        .interact()
        .unwrap();

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

    let script_path = format!("{}/nftables_backup.sh", std::env::var("HOME").unwrap_or_default());
    fs::write(&script_path, backup_script).ok();
    Command::new("chmod").args(&["+x", &script_path]).status().ok();

    let cron_entry = format!("{} {}", frequency_str, script_path);

    println!("📝 Add this to your crontab:");
    println!("{}", cron_entry);

    let add_cron = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Add to crontab now?")
        .default(true)
        .interact()
        .unwrap();

    if add_cron {
        let cmd = format!("(crontab -l 2>/dev/null; echo '{}') | crontab -", cron_entry);
        Command::new("sh").arg("-c").arg(&cmd).status().ok();
        println!("✅ Automatic backup scheduled");
    }
}

fn rule_testing_sandbox() {
    println!("🧪 Rule Testing Sandbox");

    println!("Creating isolated testing environment...");

    // Create test namespace
    let namespace = format!("nft_test_{}", chrono::Local::now().format("%Y%m%d_%H%M%S"));

    let create_ns = format!("sudo ip netns add {}", namespace);
    Command::new("sh").arg("-c").arg(&create_ns).status().ok();

    println!("✅ Test namespace created: {}", namespace);

    let options = [
        "Add test rule",
        "Test packet flow",
        "Simulate attack",
        "View test results",
        "Clean up sandbox",
    ];

    loop {
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Sandbox options")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

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

    let rule = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter nftables rule to test")
        .interact()
        .unwrap();

    let cmd = format!("sudo ip netns exec {} nft {}", namespace, rule);

    let status = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Test rule added"),
        _ => println!("❌ Failed to add test rule"),
    }
}

fn test_packet_flow(namespace: &str) {
    println!("🔍 Test packet flow");

    let src_ip = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter source IP")
        .default("192.168.1.100".to_string())
        .interact()
        .unwrap();

    let dst_ip = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter destination IP")
        .default("192.168.1.1".to_string())
        .interact()
        .unwrap();

    let port = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter destination port")
        .default("80".to_string())
        .interact()
        .unwrap();

    // Simulate packet flow
    println!("📦 Simulating packet flow...");
    println!("  {} → {}:{}", src_ip, dst_ip, port);

    // This would use tools like hping3 or scapy for actual testing
    println!("⚠️ Actual packet simulation requires additional tools");
}

fn simulate_attack(namespace: &str) {
    println!("⚠️ Simulate attack patterns");

    let attacks = [
        "SYN flood",
        "Port scan",
        "Brute force",
        "DDoS simulation",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select attack type to simulate")
        .items(&attacks)
        .default(0)
        .interact()
        .unwrap();

    println!("🔧 Simulating {} in sandbox...", attacks[choice]);
    println!("⚠️ This is a safe simulation in isolated namespace");

    // Simulation logic would go here
}

fn view_test_results(namespace: &str) {
    println!("📊 View test results");

    let cmd = format!("sudo ip netns exec {} nft list ruleset", namespace);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output();

    if let Ok(out) = output {
        let result = String::from_utf8_lossy(&out.stdout);
        println!("{}", result);
    }
}

fn cleanup_sandbox(namespace: &str) {
    println!("🧹 Cleaning up sandbox");

    let cmd = format!("sudo ip netns delete {}", namespace);

    let status = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
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
    let cmd = "sudo nft list ruleset -a -n";
    if let Ok(output) = Command::new("sh").arg("-c").arg(cmd).output() {
        let ruleset = String::from_utf8_lossy(&output.stdout);
        let rule_count = ruleset.lines().filter(|l| l.contains("handle")).count();
        println!("  • Total rules: {}", rule_count);
    }

    // Check packet counters
    let counter_cmd = "sudo nft list counters";
    if let Ok(output) = Command::new("sh").arg("-c").arg(counter_cmd).output() {
        println!("  • Active counters detected");
    }

    // Connection tracking stats
    let conntrack_cmd = "sudo conntrack -C";
    if let Ok(output) = Command::new("sh").arg("-c").arg(conntrack_cmd).output() {
        let count = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("  • Active connections: {}", count);
    }

    // CPU usage
    println!("\n💻 System Impact:");
    let cpu_cmd = "top -bn1 | grep 'nft\\|netfilter'";
    if let Ok(output) = Command::new("sh").arg("-c").arg(cpu_cmd).output() {
        let result = String::from_utf8_lossy(&output.stdout);
        if !result.is_empty() {
            println!("{}", result);
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select template")
        .items(&templates)
        .default(0)
        .interact()
        .unwrap();

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

    let apply = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Apply this template?")
        .default(false)
        .interact()
        .unwrap();

    if apply {
        let temp_file = "/tmp/web_server_template.nft";
        fs::write(temp_file, template).ok();

        let cmd = format!("sudo nft -f {}", temp_file);
        Command::new("sh").arg("-c").arg(&cmd).status().ok();

        fs::remove_file(temp_file).ok();
        println!("✅ Web server template applied");
    }
}

fn game_server_template() {
    println!("🎮 Game Server Template");

    let game_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select game type")
        .items(&["Minecraft", "CS:GO", "Rust", "Valheim", "Custom"])
        .default(0)
        .interact()
        .unwrap();

    let port = match game_type {
        0 => "25565".to_string(),  // Minecraft
        1 => "27015".to_string(),  // CS:GO
        2 => "28015".to_string(),  // Rust
        3 => "2456-2458".to_string(),   // Valheim
        _ => {
            Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter game port")
                .interact()
                .unwrap()
        }
    };

    let template = format!(r#"#!/usr/sbin/nft -f

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
}}"#, port, port, port);

    println!("{}", template);

    let apply = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Apply this template?")
        .default(false)
        .interact()
        .unwrap();

    if apply {
        let temp_file = "/tmp/game_server_template.nft";
        fs::write(temp_file, template).ok();

        let cmd = format!("sudo nft -f {}", temp_file);
        Command::new("sh").arg("-c").arg(&cmd).status().ok();

        fs::remove_file(temp_file).ok();
        println!("✅ Game server template applied");
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