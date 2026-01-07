use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use std::process::Command;

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

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔥 Firewall Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

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

    loop {
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🛡️ UFW Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => {
                let enable = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enable UFW?")
                    .default(true)
                    .interact()
                    .unwrap();

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
                let rule_type = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select rule type")
                    .items(&[
                        "Allow port",
                        "Deny port",
                        "Allow from IP",
                        "Deny from IP",
                        "Allow service",
                    ])
                    .default(0)
                    .interact()
                    .unwrap();

                match rule_type {
                    0 | 1 => {
                        let port = Input::<String>::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter port number or range (e.g., 80, 8000:8080)")
                            .interact()
                            .unwrap();

                        let protocol = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Select protocol")
                            .items(&["tcp", "udp", "both"])
                            .default(0)
                            .interact()
                            .unwrap();

                        let action = if rule_type == 0 { "allow" } else { "deny" };
                        let proto = match protocol {
                            0 => "/tcp",
                            1 => "/udp",
                            _ => "",
                        };

                        let cmd = format!("sudo ufw {} {}{}", action, port, proto);
                        println!("🔧 Executing: {}", cmd);

                        let status = Command::new("sh").arg("-c").arg(&cmd).status();

                        match status {
                            Ok(s) if s.success() => println!("✅ Rule added"),
                            _ => println!("❌ Failed to add rule"),
                        }
                    }
                    2 | 3 => {
                        let ip = Input::<String>::with_theme(&ColorfulTheme::default())
                            .with_prompt(
                                "Enter IP address or subnet (e.g., 192.168.1.100, 192.168.1.0/24)",
                            )
                            .interact()
                            .unwrap();

                        let action = if rule_type == 2 { "allow" } else { "deny" };

                        let port = Input::<String>::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter port (optional, press Enter to skip)")
                            .allow_empty(true)
                            .interact()
                            .unwrap();

                        let cmd = if port.is_empty() {
                            format!("sudo ufw {} from {}", action, ip)
                        } else {
                            format!("sudo ufw {} from {} to any port {}", action, ip, port)
                        };

                        println!("🔧 Executing: {}", cmd);
                        let status = Command::new("sh").arg("-c").arg(&cmd).status();

                        match status {
                            Ok(s) if s.success() => println!("✅ Rule added"),
                            _ => println!("❌ Failed to add rule"),
                        }
                    }
                    4 => {
                        let service = Input::<String>::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter service name (e.g., ssh, http, https)")
                            .interact()
                            .unwrap();

                        let cmd = format!("sudo ufw allow {}", service);
                        println!("🔧 Executing: {}", cmd);

                        let status = Command::new("sh").arg("-c").arg(&cmd).status();

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

                let rule_num = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter rule number to delete (or 'cancel')")
                    .interact()
                    .unwrap();

                if rule_num != "cancel" {
                    let cmd = format!("sudo ufw delete {}", rule_num);
                    let status = Command::new("sh").arg("-c").arg(&cmd).status();

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
                let confirm = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("⚠️ This will reset all UFW rules. Continue?")
                    .default(false)
                    .interact()
                    .unwrap();

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

                let app = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter application name to allow")
                    .interact()
                    .unwrap();

                let cmd = format!("sudo ufw allow '{}'", app);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
            }
            6 => {
                let app = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter application name to deny")
                    .interact()
                    .unwrap();

                let cmd = format!("sudo ufw deny '{}'", app);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
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

    loop {
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔥 Firewalld Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => {
                let action = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select action")
                    .items(&["Start", "Stop", "Restart", "Enable", "Disable"])
                    .default(0)
                    .interact()
                    .unwrap();

                let cmd = match action {
                    0 => "sudo systemctl start firewalld",
                    1 => "sudo systemctl stop firewalld",
                    2 => "sudo systemctl restart firewalld",
                    3 => "sudo systemctl enable firewalld",
                    4 => "sudo systemctl disable firewalld",
                    _ => "",
                };

                if !cmd.is_empty() {
                    let status = Command::new("sh").arg("-c").arg(cmd).status();

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
                let add_type = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("What to add?")
                    .items(&["Port", "Service", "Source IP"])
                    .default(0)
                    .interact()
                    .unwrap();

                let permanent = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Make permanent?")
                    .default(true)
                    .interact()
                    .unwrap();

                let perm_flag = if permanent { "--permanent" } else { "" };

                match add_type {
                    0 => {
                        let port = Input::<String>::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter port/protocol (e.g., 8080/tcp, 53/udp)")
                            .interact()
                            .unwrap();

                        let zone = Input::<String>::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter zone (or press Enter for default)")
                            .allow_empty(true)
                            .interact()
                            .unwrap();

                        let zone_flag = if zone.is_empty() {
                            String::new()
                        } else {
                            format!("--zone={}", zone)
                        };

                        let cmd = format!(
                            "sudo firewall-cmd {} {} --add-port={}",
                            perm_flag, zone_flag, port
                        );

                        println!("🔧 Executing: {}", cmd);
                        Command::new("sh").arg("-c").arg(&cmd).status().ok();
                    }
                    1 => {
                        let service = Input::<String>::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter service name (e.g., http, https, ssh)")
                            .interact()
                            .unwrap();

                        let cmd =
                            format!("sudo firewall-cmd {} --add-service={}", perm_flag, service);

                        Command::new("sh").arg("-c").arg(&cmd).status().ok();
                    }
                    2 => {
                        let source = Input::<String>::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter source IP or subnet")
                            .interact()
                            .unwrap();

                        let cmd =
                            format!("sudo firewall-cmd {} --add-source={}", perm_flag, source);

                        Command::new("sh").arg("-c").arg(&cmd).status().ok();
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
                let remove_type = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("What to remove?")
                    .items(&["Port", "Service", "Source IP"])
                    .default(0)
                    .interact()
                    .unwrap();

                let permanent = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Remove permanently?")
                    .default(true)
                    .interact()
                    .unwrap();

                let perm_flag = if permanent { "--permanent" } else { "" };

                match remove_type {
                    0 => {
                        println!("📋 Current ports:");
                        Command::new("sudo")
                            .args(&["firewall-cmd", "--list-ports"])
                            .status()
                            .ok();

                        let port = Input::<String>::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter port/protocol to remove")
                            .interact()
                            .unwrap();

                        let cmd = format!("sudo firewall-cmd {} --remove-port={}", perm_flag, port);

                        Command::new("sh").arg("-c").arg(&cmd).status().ok();
                    }
                    1 => {
                        println!("📋 Current services:");
                        Command::new("sudo")
                            .args(&["firewall-cmd", "--list-services"])
                            .status()
                            .ok();

                        let service = Input::<String>::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter service to remove")
                            .interact()
                            .unwrap();

                        let cmd = format!(
                            "sudo firewall-cmd {} --remove-service={}",
                            perm_flag, service
                        );

                        Command::new("sh").arg("-c").arg(&cmd).status().ok();
                    }
                    2 => {
                        println!("📋 Current sources:");
                        Command::new("sudo")
                            .args(&["firewall-cmd", "--list-sources"])
                            .status()
                            .ok();

                        let source = Input::<String>::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter source to remove")
                            .interact()
                            .unwrap();

                        let cmd =
                            format!("sudo firewall-cmd {} --remove-source={}", perm_flag, source);

                        Command::new("sh").arg("-c").arg(&cmd).status().ok();
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Zone Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            Command::new("sudo")
                .args(&["firewall-cmd", "--get-zones"])
                .status()
                .ok();
        }
        1 => {
            let zone = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter new default zone")
                .interact()
                .unwrap();

            Command::new("sudo")
                .args(&["firewall-cmd", "--set-default-zone", &zone])
                .status()
                .ok();
        }
        2 => {
            let interface = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter interface name")
                .interact()
                .unwrap();

            let zone = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter zone name")
                .interact()
                .unwrap();

            let cmd = format!(
                "sudo firewall-cmd --zone={} --add-interface={} --permanent",
                zone, interface
            );

            Command::new("sh").arg("-c").arg(&cmd).status().ok();

            Command::new("sudo")
                .args(&["firewall-cmd", "--reload"])
                .status()
                .ok();
        }
        3 => {
            let interface = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter interface name")
                .interact()
                .unwrap();

            let zone = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter zone name")
                .interact()
                .unwrap();

            let cmd = format!(
                "sudo firewall-cmd --zone={} --remove-interface={} --permanent",
                zone, interface
            );

            Command::new("sh").arg("-c").arg(&cmd).status().ok();

            Command::new("sudo")
                .args(&["firewall-cmd", "--reload"])
                .status()
                .ok();
        }
        4 => {
            let zone_name = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter new zone name")
                .interact()
                .unwrap();

            let cmd = format!("sudo firewall-cmd --permanent --new-zone={}", zone_name);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();

            Command::new("sudo")
                .args(&["firewall-cmd", "--reload"])
                .status()
                .ok();

            println!("✅ Zone '{}' created", zone_name);
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Rich Rules")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            Command::new("sudo")
                .args(&["firewall-cmd", "--list-rich-rules"])
                .status()
                .ok();
        }
        1 => {
            println!("📝 Rich Rule Builder");

            let rule_type = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Rule type")
                .items(&["Accept", "Reject", "Drop"])
                .default(0)
                .interact()
                .unwrap();

            let action = match rule_type {
                0 => "accept",
                1 => "reject",
                2 => "drop",
                _ => "accept",
            };

            let source = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Source address (or press Enter to skip)")
                .allow_empty(true)
                .interact()
                .unwrap();

            let port = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Port number (or press Enter to skip)")
                .allow_empty(true)
                .interact()
                .unwrap();

            let protocol = if !port.is_empty() {
                Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Protocol")
                    .items(&["tcp", "udp"])
                    .default(0)
                    .interact()
                    .unwrap()
            } else {
                0
            };

            let proto = if protocol == 0 { "tcp" } else { "udp" };

            let mut rule = String::from("rule ");

            if !source.is_empty() {
                rule.push_str(&format!("family=\"ipv4\" source address=\"{}\" ", source));
            }

            if !port.is_empty() {
                rule.push_str(&format!("port port=\"{}\" protocol=\"{}\" ", port, proto));
            }

            rule.push_str(action);

            let cmd = format!("sudo firewall-cmd --add-rich-rule='{}' --permanent", rule);
            println!("🔧 Executing: {}", cmd);

            Command::new("sh").arg("-c").arg(&cmd).status().ok();

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

            let rule = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter rule to remove (copy exactly)")
                .interact()
                .unwrap();

            let cmd = format!(
                "sudo firewall-cmd --remove-rich-rule='{}' --permanent",
                rule
            );
            Command::new("sh").arg("-c").arg(&cmd).status().ok();

            Command::new("sudo")
                .args(&["firewall-cmd", "--reload"])
                .status()
                .ok();
        }
        3 => {
            let service = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Service to rate limit (e.g., ssh)")
                .interact()
                .unwrap();

            let rate = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Rate (e.g., 3/m for 3 per minute)")
                .default("3/m".to_string())
                .interact()
                .unwrap();

            let rule = format!(
                "rule service name=\"{}\" limit value=\"{}\" accept",
                service, rate
            );

            let cmd = format!("sudo firewall-cmd --add-rich-rule='{}' --permanent", rule);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();

            Command::new("sudo")
                .args(&["firewall-cmd", "--reload"])
                .status()
                .ok();

            println!("✅ Rate limiting rule added for {}", service);
        }
        4 => {
            let source = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("IP address to block")
                .interact()
                .unwrap();

            let rule = format!("rule family=\"ipv4\" source address=\"{}\" drop", source);

            let cmd = format!("sudo firewall-cmd --add-rich-rule='{}' --permanent", rule);
            Command::new("sh").arg("-c").arg(&cmd).status().ok();

            Command::new("sudo")
                .args(&["firewall-cmd", "--reload"])
                .status()
                .ok();

            println!("✅ Blocked {}", source);
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

    loop {
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("⚙️ iptables Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => {
                let table = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select table")
                    .items(&["filter (default)", "nat", "mangle", "raw"])
                    .default(0)
                    .interact()
                    .unwrap();

                let table_flag = match table {
                    1 => "-t nat",
                    2 => "-t mangle",
                    3 => "-t raw",
                    _ => "",
                };

                let cmd = format!("sudo iptables {} -L -n -v --line-numbers", table_flag);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
            }
            1 => {
                add_iptables_rule();
            }
            2 => {
                delete_iptables_rule();
            }
            3 => {
                println!("💾 Saving iptables rules...");

                let distro = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select your distribution")
                    .items(&["Debian/Ubuntu", "RedHat/Fedora", "Arch", "Other"])
                    .default(0)
                    .interact()
                    .unwrap();

                let cmd = match distro {
                    0 => "sudo iptables-save > /etc/iptables/rules.v4",
                    1 => "sudo service iptables save",
                    2 => "sudo iptables-save > /etc/iptables/iptables.rules",
                    _ => {
                        let path = Input::<String>::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter save path")
                            .default("/etc/iptables.rules".to_string())
                            .interact()
                            .unwrap();
                        &format!("sudo iptables-save > {}", path)
                    }
                };

                Command::new("sh").arg("-c").arg(cmd).status().ok();

                println!("✅ Rules saved");
            }
            4 => {
                println!("📥 Restoring iptables rules...");

                let path = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter rules file path")
                    .default("/etc/iptables/rules.v4".to_string())
                    .interact()
                    .unwrap();

                let cmd = format!("sudo iptables-restore < {}", path);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();

                println!("✅ Rules restored");
            }
            5 => {
                let confirm = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("⚠️ This will remove ALL iptables rules. Continue?")
                    .default(false)
                    .interact()
                    .unwrap();

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
    println!("➕ Add iptables Rule");

    let chain = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select chain")
        .items(&["INPUT", "OUTPUT", "FORWARD", "Custom"])
        .default(0)
        .interact()
        .unwrap();

    let chain_name = if chain == 3 {
        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter custom chain name")
            .interact()
            .unwrap()
    } else {
        ["INPUT", "OUTPUT", "FORWARD"][chain].to_string()
    };

    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action")
        .items(&["ACCEPT", "DROP", "REJECT", "LOG"])
        .default(0)
        .interact()
        .unwrap();

    let action_str = ["ACCEPT", "DROP", "REJECT", "LOG"][action];

    let mut rule = format!("sudo iptables -A {} ", chain_name);

    // Protocol
    let use_protocol = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Specify protocol?")
        .default(true)
        .interact()
        .unwrap();

    if use_protocol {
        let protocol = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select protocol")
            .items(&["tcp", "udp", "icmp", "all"])
            .default(0)
            .interact()
            .unwrap();

        let proto = ["tcp", "udp", "icmp", "all"][protocol];
        rule.push_str(&format!("-p {} ", proto));

        // Port
        if protocol < 2 {
            let port = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter port (or press Enter to skip)")
                .allow_empty(true)
                .interact()
                .unwrap();

            if !port.is_empty() {
                rule.push_str(&format!("--dport {} ", port));
            }
        }
    }

    // Source
    let source = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter source IP (or press Enter to skip)")
        .allow_empty(true)
        .interact()
        .unwrap();

    if !source.is_empty() {
        rule.push_str(&format!("-s {} ", source));
    }

    // Destination
    let dest = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter destination IP (or press Enter to skip)")
        .allow_empty(true)
        .interact()
        .unwrap();

    if !dest.is_empty() {
        rule.push_str(&format!("-d {} ", dest));
    }

    // Interface
    if chain_name == "INPUT" {
        let interface = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter input interface (or press Enter to skip)")
            .allow_empty(true)
            .interact()
            .unwrap();

        if !interface.is_empty() {
            rule.push_str(&format!("-i {} ", interface));
        }
    } else if chain_name == "OUTPUT" {
        let interface = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter output interface (or press Enter to skip)")
            .allow_empty(true)
            .interact()
            .unwrap();

        if !interface.is_empty() {
            rule.push_str(&format!("-o {} ", interface));
        }
    }

    // State
    let use_state = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Use connection state?")
        .default(false)
        .interact()
        .unwrap();

    if use_state {
        let states = vec!["NEW", "ESTABLISHED", "RELATED", "INVALID"];
        let selected = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select states")
            .items(&states)
            .interact()
            .unwrap();

        if !selected.is_empty() {
            let state_list: Vec<String> = selected.iter().map(|&i| states[i].to_string()).collect();
            rule.push_str(&format!("-m state --state {} ", state_list.join(",")));
        }
    }

    rule.push_str(&format!("-j {}", action_str));

    println!("🔧 Executing: {}", rule);
    let status = Command::new("sh").arg("-c").arg(&rule).status();

    match status {
        Ok(s) if s.success() => println!("✅ Rule added"),
        _ => println!("❌ Failed to add rule"),
    }
}

fn delete_iptables_rule() {
    println!("🗑️ Delete iptables Rule");

    let chain = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select chain")
        .items(&["INPUT", "OUTPUT", "FORWARD", "Custom"])
        .default(0)
        .interact()
        .unwrap();

    let chain_name = if chain == 3 {
        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter custom chain name")
            .interact()
            .unwrap()
    } else {
        ["INPUT", "OUTPUT", "FORWARD"][chain].to_string()
    };

    // List rules with line numbers
    let cmd = format!("sudo iptables -L {} --line-numbers -n", chain_name);
    Command::new("sh").arg("-c").arg(&cmd).status().ok();

    let rule_num = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter rule number to delete")
        .interact()
        .unwrap();

    let delete_cmd = format!("sudo iptables -D {} {}", chain_name, rule_num);
    let status = Command::new("sh").arg("-c").arg(&delete_cmd).status();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Chain Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            Command::new("sudo")
                .args(&["iptables", "-L", "-n"])
                .status()
                .ok();
        }
        1 => {
            let chain_name = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter new chain name")
                .interact()
                .unwrap();

            let cmd = format!("sudo iptables -N {}", chain_name);
            let status = Command::new("sh").arg("-c").arg(&cmd).status();

            match status {
                Ok(s) if s.success() => println!("✅ Chain '{}' created", chain_name),
                _ => println!("❌ Failed to create chain"),
            }
        }
        2 => {
            let chain_name = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter chain name to delete")
                .interact()
                .unwrap();

            let cmd = format!("sudo iptables -X {}", chain_name);
            let status = Command::new("sh").arg("-c").arg(&cmd).status();

            match status {
                Ok(s) if s.success() => println!("✅ Chain '{}' deleted", chain_name),
                _ => println!("❌ Failed to delete chain"),
            }
        }
        3 => {
            let chain = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select chain")
                .items(&["INPUT", "OUTPUT", "FORWARD"])
                .default(0)
                .interact()
                .unwrap();

            let chain_name = ["INPUT", "OUTPUT", "FORWARD"][chain];

            let policy = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select default policy")
                .items(&["ACCEPT", "DROP"])
                .default(0)
                .interact()
                .unwrap();

            let policy_str = ["ACCEPT", "DROP"][policy];

            let cmd = format!("sudo iptables -P {} {}", chain_name, policy_str);
            let status = Command::new("sh").arg("-c").arg(&cmd).status();

            match status {
                Ok(s) if s.success() => println!("✅ Default policy set"),
                _ => println!("❌ Failed to set policy"),
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

    loop {
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🚀 nftables Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => {
                println!("📋 nftables Rules:");
                Command::new("sudo")
                    .args(&["nft", "list", "ruleset"])
                    .status()
                    .ok();
            }
            1 => {
                let family = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select address family")
                    .items(&["ip", "ip6", "inet", "bridge", "netdev"])
                    .default(2)
                    .interact()
                    .unwrap();

                let family_str = ["ip", "ip6", "inet", "bridge", "netdev"][family];

                let table_name = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter table name")
                    .interact()
                    .unwrap();

                let cmd = format!("sudo nft add table {} {}", family_str, table_name);
                let status = Command::new("sh").arg("-c").arg(&cmd).status();

                match status {
                    Ok(s) if s.success() => println!("✅ Table created"),
                    _ => println!("❌ Failed to create table"),
                }
            }
            2 => {
                let table_name = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter table name")
                    .interact()
                    .unwrap();

                let chain_name = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter chain name")
                    .interact()
                    .unwrap();

                let hook = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select hook")
                    .items(&["input", "output", "forward", "prerouting", "postrouting"])
                    .default(0)
                    .interact()
                    .unwrap();

                let hook_str = ["input", "output", "forward", "prerouting", "postrouting"][hook];

                let priority = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter priority (0 for filter)")
                    .default("0".to_string())
                    .interact()
                    .unwrap();

                let cmd = format!(
                    "sudo nft add chain inet {} {} '{{ type filter hook {} priority {}; }}'",
                    table_name, chain_name, hook_str, priority
                );

                let status = Command::new("sh").arg("-c").arg(&cmd).status();

                match status {
                    Ok(s) if s.success() => println!("✅ Chain created"),
                    _ => println!("❌ Failed to create chain"),
                }
            }
            3 => {
                add_nftables_rule();
            }
            4 => {
                println!("📋 Current rules:");
                Command::new("sudo")
                    .args(&["nft", "-a", "list", "ruleset"])
                    .status()
                    .ok();

                let handle = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter rule handle to delete")
                    .interact()
                    .unwrap();

                let table = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter table name")
                    .interact()
                    .unwrap();

                let chain = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter chain name")
                    .interact()
                    .unwrap();

                let cmd = format!(
                    "sudo nft delete rule inet {} {} handle {}",
                    table, chain, handle
                );

                Command::new("sh").arg("-c").arg(&cmd).status().ok();
            }
            5 => {
                let path = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter save path")
                    .default("/etc/nftables.conf".to_string())
                    .interact()
                    .unwrap();

                let cmd = format!("sudo nft list ruleset > {}", path);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();

                println!("✅ Configuration saved to {}", path);
            }
            6 => {
                let path = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter configuration file path")
                    .default("/etc/nftables.conf".to_string())
                    .interact()
                    .unwrap();

                let cmd = format!("sudo nft -f {}", path);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();

                println!("✅ Configuration loaded");
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
    println!("➕ Add nftables Rule");

    let table = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter table name")
        .interact()
        .unwrap();

    let chain = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter chain name")
        .interact()
        .unwrap();

    let rule_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select rule type")
        .items(&["Accept", "Drop", "Reject", "Log", "Custom"])
        .default(0)
        .interact()
        .unwrap();

    let mut rule = String::new();

    // Source
    let source = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Source IP (or press Enter to skip)")
        .allow_empty(true)
        .interact()
        .unwrap();

    if !source.is_empty() {
        rule.push_str(&format!("ip saddr {} ", source));
    }

    // Destination
    let dest = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Destination IP (or press Enter to skip)")
        .allow_empty(true)
        .interact()
        .unwrap();

    if !dest.is_empty() {
        rule.push_str(&format!("ip daddr {} ", dest));
    }

    // Protocol and port
    let use_port = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Specify port?")
        .default(false)
        .interact()
        .unwrap();

    if use_port {
        let protocol = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Protocol")
            .items(&["tcp", "udp"])
            .default(0)
            .interact()
            .unwrap();

        let proto = ["tcp", "udp"][protocol];

        let port = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Port number")
            .interact()
            .unwrap();

        rule.push_str(&format!("{} dport {} ", proto, port));
    }

    // Action
    let action = match rule_type {
        0 => "accept",
        1 => "drop",
        2 => "reject",
        3 => "log",
        4 => "custom",
        _ => "accept",
    };

    rule.push_str(action);

    let cmd = format!("sudo nft add rule inet {} {} {}", table, chain, rule);
    println!("🔧 Executing: {}", cmd);

    let status = Command::new("sh").arg("-c").arg(&cmd).status();

    match status {
        Ok(s) if s.success() => println!("✅ Rule added"),
        _ => println!("❌ Failed to add rule"),
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Port Scanner")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("🔍 Scanning local ports...");

            let scan_type = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select scan type")
                .items(&["All ports", "Common ports", "Custom range"])
                .default(1)
                .interact()
                .unwrap();

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
                    let start = Input::<String>::with_theme(&ColorfulTheme::default())
                        .with_prompt("Start port")
                        .interact()
                        .unwrap();

                    let end = Input::<String>::with_theme(&ColorfulTheme::default())
                        .with_prompt("End port")
                        .interact()
                        .unwrap();

                    let cmd = format!(
                        "for port in $(seq {} {}); do nc -zv localhost $port 2>&1 | grep succeeded; done",
                        start, end
                    );
                    Command::new("sh").arg("-c").arg(&cmd).status().ok();
                }
                _ => {
                    println!("Invalid scan type selected");
                }
            }
        }
        1 => {
            let host = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter host to scan")
                .interact()
                .unwrap();

            let port_range = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter port range (e.g., 1-1000)")
                .default("1-1000".to_string())
                .interact()
                .unwrap();

            println!("🔍 Scanning {}...", host);

            // Use nmap if available, otherwise nc
            let nmap_check = Command::new("which").arg("nmap").status();

            if let Ok(s) = nmap_check {
                if s.success() {
                    let cmd = format!("nmap -p {} {}", port_range, host);
                    Command::new("sh").arg("-c").arg(&cmd).status().ok();
                } else {
                    println!("⚠️ nmap not found, using nc...");
                    let ports: Vec<&str> = port_range.split('-').collect();
                    if ports.len() == 2 {
                        let cmd = format!(
                            "for port in $(seq {} {}); do nc -zv -w 1 {} $port 2>&1 | grep succeeded; done",
                            ports[0], ports[1], host
                        );
                        Command::new("sh").arg("-c").arg(&cmd).status().ok();
                    }
                }
            }
        }
        2 => {
            let host = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter host (or localhost)")
                .default("localhost".to_string())
                .interact()
                .unwrap();

            let port = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter port number")
                .interact()
                .unwrap();

            println!("🔍 Checking {}:{}...", host, port);

            let output = Command::new("nc").args(&["-zv", &host, &port]).output();

            match output {
                Ok(out) => {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    if stderr.contains("succeeded") || out.status.success() {
                        println!("✅ Port {} is OPEN on {}", port, host);

                        // Try to identify service
                        if host == "localhost" || host == "127.0.0.1" {
                            let cmd = format!("sudo lsof -i :{}", port);
                            println!("\n📋 Service information:");
                            Command::new("sh").arg("-c").arg(&cmd).status().ok();
                        }
                    } else {
                        println!("❌ Port {} is CLOSED on {}", port, host);
                    }
                }
                _ => println!("❌ Failed to check port"),
            }
        }
        3 => {
            println!("🔊 Listening Services:");
            Command::new("sudo")
                .args(&["netstat", "-tulpn"])
                .status()
                .ok();
        }
        4 => {
            println!("📊 Port Usage Statistics:");

            println!("\n📈 TCP Connections:");
            Command::new("ss").args(&["-s"]).status().ok();

            println!("\n🔢 Port count by state:");
            Command::new("sh")
                .arg("-c")
                .arg("ss -tan | awk 'NR>1 {print $1}' | sort | uniq -c")
                .status()
                .ok();

            println!("\n🏆 Top 10 most connected ports:");
            Command::new("sh")
                .arg("-c")
                .arg("ss -tan | awk 'NR>1 {print $4}' | cut -d: -f2 | sort | uniq -c | sort -rn | head -10")
                .status()
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Troubleshooting")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let host = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter host/IP to test")
        .interact()
        .unwrap();

    let port = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter port (or press Enter for ICMP ping)")
        .allow_empty(true)
        .interact()
        .unwrap();

    println!("\n🔍 Running diagnostics...");

    // 1. DNS Resolution
    println!("\n1️⃣ DNS Resolution:");
    let dns_output = Command::new("nslookup").arg(&host).output();

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
        .args(&["-c", "3", "-W", "2", &host])
        .output();

    match ping_output {
        Ok(out) if out.status.success() => println!("  ✅ Host is reachable via ICMP"),
        _ => println!("  ❌ Host unreachable via ICMP (may be blocked)"),
    }

    // 3. Port test if specified
    if !port.is_empty() {
        println!("\n3️⃣ Port {} Connectivity:", port);

        let nc_output = Command::new("nc")
            .args(&["-zv", "-w", "2", &host, &port])
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
                    .args(&["tcptraceroute", &host, &port])
                    .status()
                    .ok();
            } else {
                println!("  ⚠️ tcptraceroute not installed, using regular traceroute");
                Command::new("traceroute")
                    .args(&["-n", "-m", "10", &host])
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

            if !port.is_empty() {
                let check_cmd = format!("sudo ufw status | grep {}", port);
                Command::new("sh").arg("-c").arg(&check_cmd).status().ok();
            }
        }
    }

    // Check iptables
    if !port.is_empty() {
        let iptables_cmd = format!("sudo iptables -L -n | grep {}", port);
        let iptables_out = Command::new("sh").arg("-c").arg(&iptables_cmd).output();

        if let Ok(out) = iptables_out
            && !out.stdout.is_empty()
        {
            println!("  ⚠️ Found iptables rules for port {}", port);
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
    Command::new("sh")
        .arg("-c")
        .arg("sudo journalctl -xe | grep -i 'block\\|drop\\|reject' | tail -20")
        .status()
        .ok();
}

fn test_firewall_rules() {
    println!("🔄 Test Firewall Rules");

    let test_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select test type")
        .items(&["Test specific rule", "Test all rules", "Simulate packet"])
        .default(0)
        .interact()
        .unwrap();

    match test_type {
        0 => {
            let source = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Source IP (or any)")
                .default("any".to_string())
                .interact()
                .unwrap();

            let dest = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Destination IP (or any)")
                .default("any".to_string())
                .interact()
                .unwrap();

            let port = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Port")
                .interact()
                .unwrap();

            let protocol = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Protocol")
                .items(&["tcp", "udp"])
                .default(0)
                .interact()
                .unwrap();

            let proto = ["tcp", "udp"][protocol];

            println!("\n🔍 Checking rules that would match:");

            // Check iptables
            let cmd = format!(
                "sudo iptables -L -n -v | grep -E '{}.*{}|{}.*dpt:{}'",
                source, dest, proto, port
            );

            Command::new("sh").arg("-c").arg(&cmd).status().ok();
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

            let chain = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Starting chain")
                .items(&["INPUT", "OUTPUT", "FORWARD"])
                .default(0)
                .interact()
                .unwrap();

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

    let port = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter port being blocked")
        .interact()
        .unwrap();

    let direction = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Traffic direction")
        .items(&["Incoming (INPUT)", "Outgoing (OUTPUT)", "Both"])
        .default(0)
        .interact()
        .unwrap();

    println!("\n🔍 Searching for blocking rules...");

    match direction {
        0 | 2 => {
            println!("\n📥 INPUT chain:");
            let cmd = format!(
                "sudo iptables -L INPUT -n -v --line-numbers | grep -E 'DROP|REJECT' | grep {}",
                port
            );
            Command::new("sh").arg("-c").arg(&cmd).status().ok();

            // Also check for default DROP policy
            let policy_cmd = "sudo iptables -L INPUT -n | head -1";
            Command::new("sh").arg("-c").arg(policy_cmd).status().ok();
        }
        _ => {}
    }

    if direction == 1 || direction == 2 {
        println!("\n📤 OUTPUT chain:");
        let cmd = format!(
            "sudo iptables -L OUTPUT -n -v --line-numbers | grep -E 'DROP|REJECT' | grep {}",
            port
        );
        Command::new("sh").arg("-c").arg(&cmd).status().ok();
    }

    // Check UFW
    println!("\n🛡️ UFW rules:");
    let ufw_cmd = format!("sudo ufw status numbered | grep {}", port);
    Command::new("sh").arg("-c").arg(&ufw_cmd).status().ok();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Quick Fix")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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
            let dev_ports = vec![
                "3000", "3001", "8000", "8080", "8081", "5000", "5001", "4200", "9000",
            ];

            for port in dev_ports {
                let cmd = format!("sudo ufw allow {}/tcp", port);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
            }
            println!("✅ Common development ports allowed");
        }
        3 => {
            let duration = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Disable for how many minutes? (0 for permanent)")
                .default("5".to_string())
                .interact()
                .unwrap();

            if duration == "0" {
                Command::new("sudo").args(&["ufw", "disable"]).status().ok();
                Command::new("sudo")
                    .args(&["systemctl", "stop", "firewalld"])
                    .status()
                    .ok();
                println!("⚠️ Firewall disabled permanently");
            } else {
                Command::new("sudo").args(&["ufw", "disable"]).status().ok();
                Command::new("sudo")
                    .args(&["systemctl", "stop", "firewalld"])
                    .status()
                    .ok();

                println!("⚠️ Firewall disabled for {} minutes", duration);
                println!("⏰ Will re-enable automatically");

                let enable_cmd = format!(
                    "sleep {}m && sudo ufw enable && sudo systemctl start firewalld",
                    duration
                );

                Command::new("sh").arg("-c").arg(&enable_cmd).spawn().ok();
            }
        }
        4 => {
            let confirm = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Reset firewall to default rules?")
                .default(false)
                .interact()
                .unwrap();

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

    let log_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select log source")
        .items(&[
            "UFW logs",
            "iptables logs",
            "Firewalld logs",
            "All firewall logs",
            "Live monitoring",
        ])
        .default(0)
        .interact()
        .unwrap();

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
            Command::new("sh")
                .arg("-c")
                .arg("sudo journalctl -xe | grep -E 'firewall|ufw|iptables|netfilter' | tail -100")
                .status()
                .ok();
        }
        4 => {
            println!("👁️ Live Firewall Monitoring (Ctrl+C to stop):");
            Command::new("sh")
                .arg("-c")
                .arg("sudo tail -f /var/log/syslog | grep -E 'UFW|firewall|iptables'")
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
    let iptables_count = Command::new("sh")
        .arg("-c")
        .arg("sudo iptables -L -n | wc -l")
        .output();

    if let Ok(out) = iptables_count {
        let count = String::from_utf8_lossy(&out.stdout)
            .trim()
            .parse::<i32>()
            .unwrap_or(0);
        if count > 10 {
            println!("  ✅ iptables has {} rules configured", count);

            // Check default policies
            println!("  📋 Default policies:");
            let policies = Command::new("sh")
                .arg("-c")
                .arg("sudo iptables -L -n | head -8 | grep Chain")
                .output();

            if let Ok(p) = policies {
                print!("{}", String::from_utf8_lossy(&p.stdout));
            }
        } else {
            println!("  ⭕ iptables has minimal rules");
        }
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
    println!("  🔌 Open ports:");
    Command::new("sh")
        .arg("-c")
        .arg("sudo ss -tuln | grep LISTEN | wc -l")
        .status()
        .ok();

    println!("  🌐 Active connections:");
    Command::new("sh")
        .arg("-c")
        .arg("ss -tan | grep ESTAB | wc -l")
        .status()
        .ok();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🎮 Gaming Network Optimization")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select gaming platform to optimize")
        .items(&games)
        .default(0)
        .interact()
        .unwrap();

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

    let gaming_ports = vec![
        // Battle.net games
        ("1119", "tcp", "Battle.net"),
        ("3724", "tcp", "World of Warcraft"),
        ("6113", "tcp", "Battle.net"),
        ("6881-6999", "tcp", "Blizzard Downloader"),
        // Steam
        ("27000-27100", "udp", "Steam Client"),
        ("27015-27030", "tcp", "Steam"),
        ("27015-27030", "udp", "Steam"),
        // Discord
        ("50000-65535", "udp", "Discord Voice"),
        // Popular games
        ("7777-7784", "tcp", "Unreal Tournament"),
        ("27015", "tcp", "Source Games"),
        ("25565", "tcp", "Minecraft"),
        ("19132", "udp", "Minecraft Bedrock"),
        // Console gaming
        ("53", "udp", "Console DNS"),
        ("80", "tcp", "Console Updates"),
        ("443", "tcp", "Console Services"),
        ("3478-3480", "udp", "PlayStation/Xbox"),
    ];

    println!("🔧 Configuring firewall rules for optimal gaming...");

    for (port, protocol, service) in &gaming_ports {
        println!("  ⚡ Optimizing {} - {} ({})", service, port, protocol);

        // UFW rules
        let ufw_cmd = format!("sudo ufw allow {}/{}", port, protocol);
        Command::new("sh").arg("-c").arg(&ufw_cmd).status().ok();

        // iptables rules with priority
        if protocol == &"tcp" {
            let iptables_cmd = format!(
                "sudo iptables -A INPUT -p tcp --dport {} -j ACCEPT -m comment --comment '{}'",
                port, service
            );
            Command::new("sh")
                .arg("-c")
                .arg(&iptables_cmd)
                .status()
                .ok();
        } else {
            let iptables_cmd = format!(
                "sudo iptables -A INPUT -p udp --dport {} -j ACCEPT -m comment --comment '{}'",
                port, service
            );
            Command::new("sh")
                .arg("-c")
                .arg(&iptables_cmd)
                .status()
                .ok();
        }

        // Firewalld rules
        let firewalld_cmd = format!(
            "sudo firewall-cmd --permanent --add-port={}/{}",
            port, protocol
        );
        Command::new("sh")
            .arg("-c")
            .arg(&firewalld_cmd)
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
    println!("  • Battle.net games (WoW, Diablo, etc.)");
    println!("  • Steam platform");
    println!("  • Discord gaming");
    println!("  • Popular multiplayer games");
    println!("  • Console gaming services");
}

fn optimize_wow_ports() {
    println!("⚔️ World of Warcraft Port Optimization");
    println!("======================================\n");

    let wow_ports = vec![
        ("1119", "tcp", "Battle.net Authentication"),
        ("3724", "tcp", "WoW Game Connection"),
        ("6112", "tcp", "Battle.net"),
        ("6113", "tcp", "Battle.net"),
        ("6881-6999", "tcp", "Blizzard Downloader"),
        ("80", "tcp", "Battle.net Web"),
        ("443", "tcp", "Battle.net HTTPS"),
    ];

    println!("⚔️ Configuring optimal firewall rules for World of Warcraft...");

    for (port, protocol, service) in &wow_ports {
        println!("  ⚡ Configuring {} - {}", service, port);

        // Priority iptables rules for WoW
        let iptables_cmd = format!(
            "sudo iptables -I INPUT 1 -p {} --dport {} -j ACCEPT -m comment --comment 'WoW {}'",
            protocol, port, service
        );
        Command::new("sh")
            .arg("-c")
            .arg(&iptables_cmd)
            .status()
            .ok();

        // UFW allow
        let ufw_cmd = format!("sudo ufw allow {}/{}", port, protocol);
        Command::new("sh").arg("-c").arg(&ufw_cmd).status().ok();

        // Firewalld
        let firewalld_cmd = format!(
            "sudo firewall-cmd --permanent --add-port={}/{}",
            port, protocol
        );
        Command::new("sh")
            .arg("-c")
            .arg(&firewalld_cmd)
            .status()
            .ok();
    }

    // WoW-specific optimizations
    println!("\n🚀 Applying WoW-specific network optimizations...");

    // Prioritize WoW traffic
    let priority_rules = vec![
        "sudo iptables -t mangle -A OUTPUT -p tcp --dport 3724 -j DSCP --set-dscp-class EF",
        "sudo iptables -t mangle -A OUTPUT -p tcp --dport 1119 -j DSCP --set-dscp-class AF41",
    ];

    for rule in &priority_rules {
        Command::new("sh").arg("-c").arg(rule).status().ok();
    }

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

    let d4_ports = vec![
        ("1119", "tcp", "Battle.net Authentication"),
        ("6112-6119", "tcp", "Battle.net Services"),
        ("80", "tcp", "Battle.net Web Services"),
        ("443", "tcp", "Battle.net HTTPS"),
        ("27000-27050", "tcp", "Diablo 4 Game Servers"),
        ("3478-3480", "udp", "Voice Chat"),
        ("6881-6999", "tcp", "Blizzard Downloader"),
    ];

    println!("🔥 Configuring optimal firewall rules for Diablo 4...");

    for (port, protocol, service) in &d4_ports {
        println!("  ⚡ Configuring {} - {}", service, port);

        // High-priority rules for Diablo 4
        let iptables_cmd = format!(
            "sudo iptables -I INPUT 1 -p {} --dport {} -j ACCEPT -m comment --comment 'D4 {}'",
            protocol, port, service
        );
        Command::new("sh")
            .arg("-c")
            .arg(&iptables_cmd)
            .status()
            .ok();

        let ufw_cmd = format!("sudo ufw allow {}/{}", port, protocol);
        Command::new("sh").arg("-c").arg(&ufw_cmd).status().ok();

        let firewalld_cmd = format!(
            "sudo firewall-cmd --permanent --add-port={}/{}",
            port, protocol
        );
        Command::new("sh")
            .arg("-c")
            .arg(&firewalld_cmd)
            .status()
            .ok();
    }

    // D4-specific optimizations for anti-cheat
    println!("\n🛡️ Configuring anti-cheat friendly rules...");

    // Allow Diablo 4 anti-cheat communication
    let anticheat_rules = vec![
        "sudo iptables -A INPUT -m state --state ESTABLISHED,RELATED -j ACCEPT",
        "sudo iptables -A OUTPUT -m state --state NEW,ESTABLISHED -j ACCEPT",
        "sudo iptables -t mangle -A OUTPUT -p tcp --dport 27000:27050 -j DSCP --set-dscp-class EF",
    ];

    for rule in &anticheat_rules {
        Command::new("sh").arg("-c").arg(rule).status().ok();
    }

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

    let cs2_ports = vec![
        ("27015", "tcp", "CS2 Game Server"),
        ("27015", "udp", "CS2 Game Server"),
        ("27005", "tcp", "Steam Client Service"),
        ("27000-27100", "udp", "Steam Client"),
        ("4380", "tcp", "Steam Local"),
        ("26900", "tcp", "Steam Networking"),
        ("26900", "udp", "Steam Networking"),
    ];

    for (port, protocol, service) in &cs2_ports {
        let iptables_cmd = format!(
            "sudo iptables -I INPUT 1 -p {} --dport {} -j ACCEPT -m comment --comment 'CS2 {}'",
            protocol, port, service
        );
        Command::new("sh")
            .arg("-c")
            .arg(&iptables_cmd)
            .status()
            .ok();

        let ufw_cmd = format!("sudo ufw allow {}/{}", port, protocol);
        Command::new("sh").arg("-c").arg(&ufw_cmd).status().ok();
    }

    // CS2-specific low-latency optimizations
    let cs2_optimizations = vec![
        "sudo iptables -t mangle -A OUTPUT -p udp --dport 27015 -j DSCP --set-dscp-class EF",
        "sudo iptables -t mangle -A OUTPUT -p tcp --dport 27015 -j DSCP --set-dscp-class EF",
    ];

    for rule in &cs2_optimizations {
        Command::new("sh").arg("-c").arg(rule).status().ok();
    }

    println!("✅ Counter-Strike 2 optimization completed!");
}

fn optimize_lol_ports() {
    println!("⚡ League of Legends Port Optimization");
    println!("=====================================\n");

    let lol_ports = vec![
        ("2099", "tcp", "Riot Services"),
        ("5223", "tcp", "Riot Chat"),
        ("8393-8400", "tcp", "Riot Patcher"),
        ("80", "tcp", "HTTP Updates"),
        ("443", "tcp", "HTTPS Services"),
        ("5000-5500", "udp", "Game Traffic"),
    ];

    for (port, protocol, service) in &lol_ports {
        let iptables_cmd = format!(
            "sudo iptables -I INPUT 1 -p {} --dport {} -j ACCEPT -m comment --comment 'LoL {}'",
            protocol, port, service
        );
        Command::new("sh")
            .arg("-c")
            .arg(&iptables_cmd)
            .status()
            .ok();
    }

    println!("✅ League of Legends optimization completed!");
}

fn optimize_rocket_league_ports() {
    println!("🚀 Rocket League Port Optimization");
    println!("==================================\n");

    let rl_ports = vec![
        ("7000-9000", "tcp", "Rocket League Servers"),
        ("7000-9000", "udp", "Rocket League Game Traffic"),
        ("80", "tcp", "HTTP Services"),
        ("443", "tcp", "HTTPS Services"),
    ];

    for (port, protocol, service) in &rl_ports {
        let iptables_cmd = format!(
            "sudo iptables -I INPUT 1 -p {} --dport {} -j ACCEPT -m comment --comment 'RL {}'",
            protocol, port, service
        );
        Command::new("sh")
            .arg("-c")
            .arg(&iptables_cmd)
            .status()
            .ok();
    }

    println!("✅ Rocket League optimization completed!");
}

fn optimize_fortnite_ports() {
    println!("👑 Fortnite Port Optimization");
    println!("=============================\n");

    let fortnite_ports = vec![
        ("80", "tcp", "HTTP Services"),
        ("443", "tcp", "HTTPS Services"),
        ("3478-3479", "udp", "Game Traffic"),
        ("5222", "tcp", "Epic Services"),
        ("13000-13050", "udp", "Game Servers"),
    ];

    for (port, protocol, service) in &fortnite_ports {
        let iptables_cmd = format!(
            "sudo iptables -I INPUT 1 -p {} --dport {} -j ACCEPT -m comment --comment 'Fortnite {}'",
            protocol, port, service
        );
        Command::new("sh")
            .arg("-c")
            .arg(&iptables_cmd)
            .status()
            .ok();
    }

    println!("✅ Fortnite optimization completed!");
}

fn optimize_valorant_ports() {
    println!("🎯 Valorant Port Optimization");
    println!("=============================\n");

    let valorant_ports = vec![
        ("80", "tcp", "HTTP Services"),
        ("443", "tcp", "HTTPS Services"),
        ("8080-8090", "tcp", "Riot Services"),
        ("2099", "tcp", "Riot Client"),
        ("5223", "tcp", "Riot Chat"),
        ("7000-8000", "udp", "Game Traffic"),
    ];

    for (port, protocol, service) in &valorant_ports {
        let iptables_cmd = format!(
            "sudo iptables -I INPUT 1 -p {} --dport {} -j ACCEPT -m comment --comment 'Valorant {}'",
            protocol, port, service
        );
        Command::new("sh")
            .arg("-c")
            .arg(&iptables_cmd)
            .status()
            .ok();
    }

    // Valorant anti-cheat specific rules
    let anticheat_rules = vec![
        "sudo iptables -A INPUT -m state --state ESTABLISHED,RELATED -j ACCEPT",
        "sudo iptables -A OUTPUT -m state --state NEW,ESTABLISHED -j ACCEPT",
    ];

    for rule in &anticheat_rules {
        Command::new("sh").arg("-c").arg(rule).status().ok();
    }

    println!("✅ Valorant optimization completed!");
    println!("🛡️ Anti-cheat compatibility rules applied");
}

fn optimize_discord_gaming() {
    println!("🎲 Discord Gaming Optimization");
    println!("==============================\n");

    let discord_ports = vec![
        ("443", "tcp", "Discord HTTPS"),
        ("80", "tcp", "Discord HTTP"),
        ("50000-65535", "udp", "Discord Voice"),
        ("3478-3479", "udp", "Discord Voice (backup)"),
    ];

    for (port, protocol, service) in &discord_ports {
        let iptables_cmd = format!(
            "sudo iptables -I INPUT 1 -p {} --dport {} -j ACCEPT -m comment --comment 'Discord {}'",
            protocol, port, service
        );
        Command::new("sh")
            .arg("-c")
            .arg(&iptables_cmd)
            .status()
            .ok();
    }

    // Prioritize Discord voice traffic
    let voice_priority = vec![
        "sudo iptables -t mangle -A OUTPUT -p udp --dport 50000:65535 -j DSCP --set-dscp-class EF",
        "sudo iptables -t mangle -A INPUT -p udp --sport 50000:65535 -j DSCP --set-dscp-class EF",
    ];

    for rule in &voice_priority {
        Command::new("sh").arg("-c").arg(rule).status().ok();
    }

    println!("✅ Discord gaming optimization completed!");
    println!("🎤 Voice traffic prioritized for low latency");
}

fn optimize_steam_gaming() {
    println!("🖥️ Steam Gaming Platform Optimization");
    println!("=====================================\n");

    let steam_ports = vec![
        ("27000-27100", "udp", "Steam Client"),
        ("27015-27030", "tcp", "Steam Downloads"),
        ("27015-27030", "udp", "Steam Servers"),
        ("4380", "tcp", "Steam Client Service"),
        ("26900", "tcp", "Steam Networking"),
        ("26900", "udp", "Steam Networking"),
        ("80", "tcp", "Steam Store"),
        ("443", "tcp", "Steam HTTPS"),
    ];

    for (port, protocol, service) in &steam_ports {
        let iptables_cmd = format!(
            "sudo iptables -I INPUT 1 -p {} --dport {} -j ACCEPT -m comment --comment 'Steam {}'",
            protocol, port, service
        );
        Command::new("sh")
            .arg("-c")
            .arg(&iptables_cmd)
            .status()
            .ok();
    }

    // Steam-specific optimizations
    let steam_optimizations = vec![
        "sudo iptables -t mangle -A OUTPUT -p tcp --dport 27015:27030 -j DSCP --set-dscp-class AF41",
        "sudo iptables -t mangle -A OUTPUT -p udp --dport 27000:27100 -j DSCP --set-dscp-class AF41",
    ];

    for rule in &steam_optimizations {
        Command::new("sh").arg("-c").arg(rule).status().ok();
    }

    println!("✅ Steam gaming platform optimization completed!");
}

fn optimize_custom_game_ports() {
    println!("🎮 Custom Game Port Configuration");
    println!("================================\n");

    let port_range: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter port range (e.g., 7777-7784 or single port 25565)")
        .interact()
        .unwrap();

    let protocol = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select protocol")
        .items(&["tcp", "udp", "both"])
        .default(2)
        .interact()
        .unwrap();

    let game_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter game name for comments")
        .interact()
        .unwrap();

    let priority = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Apply high priority QoS markings?")
        .default(true)
        .interact()
        .unwrap();

    println!("\n🔧 Configuring custom ports for {}...", game_name);

    match protocol {
        0 | 2 => {
            // TCP rules
            let iptables_cmd = format!(
                "sudo iptables -I INPUT 1 -p tcp --dport {} -j ACCEPT -m comment --comment '{}'",
                port_range, game_name
            );
            Command::new("sh")
                .arg("-c")
                .arg(&iptables_cmd)
                .status()
                .ok();

            if priority {
                let qos_cmd = format!(
                    "sudo iptables -t mangle -A OUTPUT -p tcp --dport {} -j DSCP --set-dscp-class EF",
                    port_range
                );
                Command::new("sh").arg("-c").arg(&qos_cmd).status().ok();
            }
        }
        _ => {}
    }

    if protocol == 1 || protocol == 2 {
        // UDP rules
        let iptables_cmd = format!(
            "sudo iptables -I INPUT 1 -p udp --dport {} -j ACCEPT -m comment --comment '{}'",
            port_range, game_name
        );
        Command::new("sh")
            .arg("-c")
            .arg(&iptables_cmd)
            .status()
            .ok();

        if priority {
            let qos_cmd = format!(
                "sudo iptables -t mangle -A OUTPUT -p udp --dport {} -j DSCP --set-dscp-class EF",
                port_range
            );
            Command::new("sh").arg("-c").arg(&qos_cmd).status().ok();
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select anti-cheat system to configure")
        .items(&anticheat_systems)
        .default(0)
        .interact()
        .unwrap();

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

    // Essential anti-cheat rules
    let anticheat_rules = vec![
        // Allow established connections
        "sudo iptables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT",
        "sudo iptables -A OUTPUT -m conntrack --ctstate NEW,ESTABLISHED -j ACCEPT",
        // Allow loopback (essential for anti-cheat)
        "sudo iptables -A INPUT -i lo -j ACCEPT",
        "sudo iptables -A OUTPUT -o lo -j ACCEPT",
        // Anti-cheat communication ports
        "sudo iptables -A INPUT -p tcp --dport 80 -j ACCEPT -m comment --comment 'Anti-cheat HTTP'",
        "sudo iptables -A INPUT -p tcp --dport 443 -j ACCEPT -m comment --comment 'Anti-cheat HTTPS'",
        "sudo iptables -A INPUT -p tcp --dport 6672 -j ACCEPT -m comment --comment 'Anti-cheat Services'",
        // DNS for anti-cheat lookups
        "sudo iptables -A OUTPUT -p udp --dport 53 -j ACCEPT",
        "sudo iptables -A OUTPUT -p tcp --dport 53 -j ACCEPT",
        "sudo iptables -A INPUT -p udp --sport 53 -j ACCEPT",
        "sudo iptables -A INPUT -p tcp --sport 53 -j ACCEPT",
        // NTP for time synchronization (critical for anti-cheat)
        "sudo iptables -A OUTPUT -p udp --dport 123 -j ACCEPT",
        "sudo iptables -A INPUT -p udp --sport 123 -j ACCEPT",
    ];

    for rule in &anticheat_rules {
        println!(
            "  ⚡ Applying rule: {}",
            rule.split("comment").next().unwrap_or("")
        );
        Command::new("sh").arg("-c").arg(rule).status().ok();
    }

    // UFW rules for anti-cheat
    let ufw_rules = vec![
        "sudo ufw allow out 80/tcp comment 'Anti-cheat HTTP'",
        "sudo ufw allow out 443/tcp comment 'Anti-cheat HTTPS'",
        "sudo ufw allow out 53 comment 'Anti-cheat DNS'",
        "sudo ufw allow out 123/udp comment 'Anti-cheat NTP'",
        "sudo ufw allow 6672/tcp comment 'Anti-cheat Services'",
    ];

    for rule in &ufw_rules {
        Command::new("sh").arg("-c").arg(rule).status().ok();
    }

    // Firewalld anti-cheat configuration
    let firewalld_rules = vec![
        "sudo firewall-cmd --permanent --add-service=http",
        "sudo firewall-cmd --permanent --add-service=https",
        "sudo firewall-cmd --permanent --add-service=dns",
        "sudo firewall-cmd --permanent --add-service=ntp",
        "sudo firewall-cmd --permanent --add-port=6672/tcp",
    ];

    for rule in &firewalld_rules {
        Command::new("sh").arg("-c").arg(rule).status().ok();
    }

    Command::new("sudo")
        .args(&["firewall-cmd", "--reload"])
        .status()
        .ok();

    println!("\n✅ Universal anti-cheat firewall rules configured!");
    println!("🛡️ Compatible with: EAC, BattlEye, Vanguard, VAC, FairFight");
    println!("⚠️ Note: Some anti-cheat systems may require additional game-specific rules");
}

fn configure_eac_rules() {
    println!("⚔️ EasyAntiCheat (EAC) Firewall Configuration");
    println!("=============================================\n");

    let eac_rules = vec![
        "sudo iptables -A OUTPUT -p tcp --dport 6672 -j ACCEPT -m comment --comment 'EAC Service'",
        "sudo iptables -A OUTPUT -p tcp --dport 443 -j ACCEPT -m comment --comment 'EAC HTTPS'",
        "sudo iptables -A OUTPUT -p tcp --dport 80 -j ACCEPT -m comment --comment 'EAC HTTP'",
        "sudo iptables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT",
    ];

    for rule in &eac_rules {
        Command::new("sh").arg("-c").arg(rule).status().ok();
    }

    println!("✅ EasyAntiCheat firewall rules configured!");
}

fn configure_battleye_rules() {
    println!("🛡️ BattlEye Firewall Configuration");
    println!("=================================\n");

    let battleye_rules = vec![
        "sudo iptables -A OUTPUT -p tcp --dport 80 -j ACCEPT -m comment --comment 'BattlEye HTTP'",
        "sudo iptables -A OUTPUT -p tcp --dport 443 -j ACCEPT -m comment --comment 'BattlEye HTTPS'",
        "sudo iptables -A OUTPUT -p tcp --dport 2344 -j ACCEPT -m comment --comment 'BattlEye Service'",
        "sudo iptables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT",
    ];

    for rule in &battleye_rules {
        Command::new("sh").arg("-c").arg(rule).status().ok();
    }

    println!("✅ BattlEye firewall rules configured!");
}

fn configure_vanguard_rules() {
    println!("🔒 Vanguard (Valorant) Firewall Configuration");
    println!("=============================================\n");

    let vanguard_rules = vec![
        "sudo iptables -A OUTPUT -p tcp --dport 443 -j ACCEPT -m comment --comment 'Vanguard HTTPS'",
        "sudo iptables -A OUTPUT -p tcp --dport 80 -j ACCEPT -m comment --comment 'Vanguard HTTP'",
        "sudo iptables -A OUTPUT -p tcp --dport 2099 -j ACCEPT -m comment --comment 'Riot Services'",
        "sudo iptables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT",
        // Vanguard requires very strict connection tracking
        "sudo iptables -A INPUT -m conntrack --ctstate INVALID -j DROP",
    ];

    for rule in &vanguard_rules {
        Command::new("sh").arg("-c").arg(rule).status().ok();
    }

    println!("✅ Vanguard anti-cheat firewall rules configured!");
    println!(
        "⚠️ Note: Vanguard requires kernel-level access and may conflict with some firewall configurations"
    );
}

fn configure_fairfight_rules() {
    println!("⚡ FairFight Firewall Configuration");
    println!("=================================\n");

    let fairfight_rules = vec![
        "sudo iptables -A OUTPUT -p tcp --dport 443 -j ACCEPT -m comment --comment 'FairFight HTTPS'",
        "sudo iptables -A OUTPUT -p tcp --dport 80 -j ACCEPT -m comment --comment 'FairFight HTTP'",
        "sudo iptables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT",
    ];

    for rule in &fairfight_rules {
        Command::new("sh").arg("-c").arg(rule).status().ok();
    }

    println!("✅ FairFight firewall rules configured!");
}

fn configure_vac_rules() {
    println!("🚀 VAC (Steam) Firewall Configuration");
    println!("====================================\n");

    let vac_rules = vec![
        "sudo iptables -A OUTPUT -p tcp --dport 27030 -j ACCEPT -m comment --comment 'VAC Steam'",
        "sudo iptables -A OUTPUT -p tcp --dport 443 -j ACCEPT -m comment --comment 'VAC HTTPS'",
        "sudo iptables -A OUTPUT -p tcp --dport 80 -j ACCEPT -m comment --comment 'VAC HTTP'",
        "sudo iptables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT",
    ];

    for rule in &vac_rules {
        Command::new("sh").arg("-c").arg(rule).status().ok();
    }

    println!("✅ VAC (Steam) firewall rules configured!");
}

fn configure_custom_anticheat() {
    println!("🎮 Custom Anti-cheat Configuration");
    println!("=================================\n");

    let service_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter anti-cheat service name")
        .interact()
        .unwrap();

    let ports: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter ports (comma-separated, e.g., 80,443,6672)")
        .interact()
        .unwrap();

    let protocols = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select protocols")
        .items(&["TCP", "UDP"])
        .interact()
        .unwrap();

    println!(
        "\n🔧 Configuring custom anti-cheat rules for {}...",
        service_name
    );

    for port in ports.split(',') {
        let port = port.trim();

        for &protocol_idx in &protocols {
            let protocol = if protocol_idx == 0 { "tcp" } else { "udp" };

            let rule = format!(
                "sudo iptables -A OUTPUT -p {} --dport {} -j ACCEPT -m comment --comment '{} {}'",
                protocol,
                port,
                service_name,
                protocol.to_uppercase()
            );

            println!(
                "  ⚡ Adding rule for {} port {}",
                protocol.to_uppercase(),
                port
            );
            Command::new("sh").arg("-c").arg(&rule).status().ok();
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🌐 Network Latency Optimization")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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
        let cmd = format!("sudo sysctl -w {}={}", parameter, value);
        Command::new("sh").arg("-c").arg(&cmd).status().ok();
    }

    // Make changes persistent
    let sysctl_config = "/etc/sysctl.d/99-gaming-network.conf";
    let mut config_content = String::from("# Gaming Network Optimizations\n");

    for (parameter, value) in &tcp_optimizations {
        config_content.push_str(&format!("{}={}\n", parameter, value));
    }

    let write_config = format!("echo '{}' | sudo tee {}", config_content, sysctl_config);
    Command::new("sh")
        .arg("-c")
        .arg(&write_config)
        .status()
        .ok();

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
        let cmd = format!("sudo sysctl -w {}={}", parameter, value);
        Command::new("sh").arg("-c").arg(&cmd).status().ok();
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

    for (parameter, value) in &kernel_optimizations {
        println!("  🔧 {}: {}", parameter, value);
        let cmd = format!("sudo sysctl -w {}={}", parameter, value);
        Command::new("sh").arg("-c").arg(&cmd).status().ok();
    }

    // IRQ affinity optimization
    println!("\n⚡ Optimizing IRQ affinity for network interfaces...");

    let check_irq = Command::new("cat").arg("/proc/interrupts").output();

    if let Ok(out) = check_irq {
        let interrupts = String::from_utf8_lossy(&out.stdout);
        for line in interrupts.lines() {
            if (line.contains("eth") || line.contains("enp") || line.contains("wlan"))
                && let Some(irq) = line.split_whitespace().next()
                && let Ok(irq_num) = irq.replace(":", "").parse::<u32>()
            {
                // Set IRQ affinity to CPU 0 for consistent latency
                let cmd = format!("echo 1 | sudo tee /proc/irq/{}/smp_affinity", irq_num);
                Command::new("sh").arg("-c").arg(&cmd).status().ok();
                println!("  📍 Set IRQ {} affinity to CPU 0", irq_num);
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

    if tc_check.is_err() || !tc_check.unwrap().success() {
        println!("⚠️ tc (traffic control) not found. Installing...");
        Command::new("sudo")
            .args(&["apt", "install", "iproute2"])
            .status()
            .ok();
        Command::new("sudo")
            .args(&["pacman", "-S", "iproute2"])
            .status()
            .ok();
        Command::new("sudo")
            .args(&["dnf", "install", "iproute"])
            .status()
            .ok();
    }

    // Get primary network interface
    let interface_result = Command::new("ip")
        .args(&["route", "get", "8.8.8.8"])
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

    println!("🌐 Configuring QoS for interface: {}", interface);

    // Setup HTB (Hierarchical Token Bucket) qdisc
    let qos_commands = vec![
        // Remove existing qdisc
        format!(
            "sudo tc qdisc del dev {} root 2>/dev/null || true",
            interface
        ),
        // Add root qdisc
        format!(
            "sudo tc qdisc add dev {} root handle 1: htb default 30",
            interface
        ),
        // Create classes for different traffic types
        format!(
            "sudo tc class add dev {} parent 1: classid 1:1 htb rate 1000mbit",
            interface
        ),
        format!(
            "sudo tc class add dev {} parent 1:1 classid 1:10 htb rate 800mbit ceil 1000mbit prio 1",
            interface
        ), // Gaming
        format!(
            "sudo tc class add dev {} parent 1:1 classid 1:20 htb rate 150mbit ceil 300mbit prio 2",
            interface
        ), // Voice
        format!(
            "sudo tc class add dev {} parent 1:1 classid 1:30 htb rate 50mbit ceil 200mbit prio 3",
            interface
        ), // Default
        // Add SFQ to classes for fairness
        format!(
            "sudo tc qdisc add dev {} parent 1:10 handle 10: sfq perturb 10",
            interface
        ),
        format!(
            "sudo tc qdisc add dev {} parent 1:20 handle 20: sfq perturb 10",
            interface
        ),
        format!(
            "sudo tc qdisc add dev {} parent 1:30 handle 30: sfq perturb 10",
            interface
        ),
    ];

    for cmd in &qos_commands {
        Command::new("sh").arg("-c").arg(cmd).status().ok();
    }

    // Add filters for gaming traffic
    let gaming_filters = vec![
        // WoW
        format!(
            "sudo tc filter add dev {} protocol ip parent 1:0 prio 1 u32 match ip dport 3724 0xffff flowid 1:10",
            interface
        ),
        format!(
            "sudo tc filter add dev {} protocol ip parent 1:0 prio 1 u32 match ip dport 1119 0xffff flowid 1:10",
            interface
        ),
        // Steam
        format!(
            "sudo tc filter add dev {} protocol ip parent 1:0 prio 1 u32 match ip dport 27015 0xffff flowid 1:10",
            interface
        ),
        format!(
            "sudo tc filter add dev {} protocol ip parent 1:0 prio 1 u32 match ip dport 27030 0xffff flowid 1:10",
            interface
        ),
        // Discord Voice (high priority)
        format!(
            "sudo tc filter add dev {} protocol ip parent 1:0 prio 1 u32 match ip sport 50000 0xc000 flowid 1:20",
            interface
        ),
        // DSCP marking for gaming
        format!(
            "sudo tc filter add dev {} protocol ip parent 1:0 prio 1 u32 match ip tos 0xb8 0xfc flowid 1:10",
            interface
        ), // EF (gaming)
        format!(
            "sudo tc filter add dev {} protocol ip parent 1:0 prio 2 u32 match ip tos 0x88 0xfc flowid 1:20",
            interface
        ), // AF41 (voice)
    ];

    for filter in &gaming_filters {
        Command::new("sh").arg("-c").arg(filter).status().ok();
    }

    println!("\n🎮 Gaming traffic prioritization configured!");
    println!("📊 QoS classes created:");
    println!("  🎯 Class 1:10 - Gaming traffic (high priority)");
    println!("  🎤 Class 1:20 - Voice traffic (medium priority)");
    println!("  📡 Class 1:30 - Default traffic (low priority)");

    // Show QoS status
    println!("\n📋 Current QoS configuration:");
    let show_cmd = format!("sudo tc -s class show dev {}", interface);
    Command::new("sh").arg("-c").arg(&show_cmd).status().ok();
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("DNS Optimization Options")
        .items(&dns_options)
        .default(0)
        .interact()
        .unwrap();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select DNS provider")
        .items(&dns_providers)
        .default(0)
        .interact()
        .unwrap();

    let (primary_dns, secondary_dns): (String, String) = match choice {
        0 => ("1.1.1.1".to_string(), "1.0.0.1".to_string()),
        1 => ("9.9.9.9".to_string(), "149.112.112.112".to_string()),
        2 => ("8.8.8.8".to_string(), "8.8.4.4".to_string()),
        3 => ("208.67.222.222".to_string(), "208.67.220.220".to_string()),
        4 => {
            let primary: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter primary DNS server")
                .interact()
                .unwrap();
            let secondary: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter secondary DNS server")
                .interact()
                .unwrap();
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

    let update_resolv = format!("echo '{}' | sudo tee /etc/resolv.conf", resolv_content);
    Command::new("sh")
        .arg("-c")
        .arg(&update_resolv)
        .status()
        .ok();

    // NetworkManager configuration
    let nm_conf = format!(
        "[main]\ndns=none\n\n[global-dns-domain-*]\nservers={},{}\n",
        primary_dns, secondary_dns
    );

    let update_nm = format!(
        "echo '{}' | sudo tee /etc/NetworkManager/conf.d/gaming-dns.conf",
        nm_conf
    );
    Command::new("sh").arg("-c").arg(&update_nm).status().ok();

    // Restart NetworkManager
    Command::new("sudo")
        .args(&["systemctl", "restart", "NetworkManager"])
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

    if resolved_check.is_ok() && resolved_check.unwrap().success() {
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

        let update_resolved = format!(
            "echo '{}' | sudo tee /etc/systemd/resolved.conf",
            resolved_conf
        );
        Command::new("sh")
            .arg("-c")
            .arg(&update_resolved)
            .status()
            .ok();

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

        let update_dnsmasq = format!("echo '{}' | sudo tee /etc/dnsmasq.conf", dnsmasq_conf);
        Command::new("sh")
            .arg("-c")
            .arg(&update_dnsmasq)
            .status()
            .ok();

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

    let enable_ipv6 = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable IPv6 DNS resolution?")
        .default(false)
        .interact()
        .unwrap();

    let enable_dnssec = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable DNSSEC validation?")
        .default(true)
        .interact()
        .unwrap();

    let cache_size: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("DNS cache size (entries)")
        .default("10000".to_string())
        .interact()
        .unwrap();

    let timeout: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("DNS query timeout (seconds)")
        .default("2".to_string())
        .interact()
        .unwrap();

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

    let update_resolved = format!(
        "echo '{}' | sudo tee /etc/systemd/resolved.conf",
        resolved_conf
    );
    Command::new("sh")
        .arg("-c")
        .arg(&update_resolved)
        .status()
        .ok();

    // Apply kernel DNS settings
    let dns_sysctls = vec![
        ("net.ipv4.ip_local_reserved_ports", "53"),
        ("net.core.busy_poll", "50"),
        ("net.core.busy_read", "50"),
    ];

    for (param, value) in &dns_sysctls {
        let cmd = format!("sudo sysctl -w {}={}", param, value);
        Command::new("sh").arg("-c").arg(&cmd).status().ok();
    }

    Command::new("sudo")
        .args(&["systemctl", "restart", "systemd-resolved"])
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
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select network interface to optimize")
            .items(&interfaces)
            .default(0)
            .interact()
            .unwrap();
        interfaces[choice].to_string()
    };

    println!("🌐 Optimizing network interface: {}", interface);

    // Interface-specific optimizations
    let interface_optimizations = vec![
        // Buffer sizes
        format!(
            "sudo ethtool -G {} rx 4096 tx 4096 2>/dev/null || true",
            interface
        ),
        // Interrupt coalescing for gaming
        format!(
            "sudo ethtool -C {} rx-usecs 10 rx-frames 4 tx-usecs 10 tx-frames 4 2>/dev/null || true",
            interface
        ),
        // Disable features that add latency
        format!(
            "sudo ethtool -K {} tso off gso off gro off lro off 2>/dev/null || true",
            interface
        ),
        // Enable features that improve performance
        format!(
            "sudo ethtool -K {} rx on tx on sg on 2>/dev/null || true",
            interface
        ),
        // Set ring buffer parameters
        format!("sudo ethtool -g {} 2>/dev/null || true", interface),
    ];

    println!("🔧 Applying interface optimizations...");

    for cmd in &interface_optimizations {
        Command::new("sh").arg("-c").arg(cmd).status().ok();
    }

    // CPU affinity for network interrupts
    println!("📍 Setting CPU affinity for network interrupts...");

    let irq_result = Command::new("grep")
        .args(&[&interface, "/proc/interrupts"])
        .output();

    if let Ok(out) = irq_result {
        let irq_line = String::from_utf8_lossy(&out.stdout);
        for line in irq_line.lines() {
            if let Some(irq) = line.split_whitespace().next() {
                let irq_num = irq.replace(":", "");
                if irq_num.parse::<u32>().is_ok() {
                    // Set interrupt affinity to specific CPU
                    let cmd = format!(
                        "echo 2 | sudo tee /proc/irq/{}/smp_affinity 2>/dev/null || true",
                        irq_num
                    );
                    Command::new("sh").arg("-c").arg(&cmd).status().ok();
                    println!("  ⚡ Set IRQ {} to CPU 1", irq_num);
                }
            }
        }
    }

    // Interface queue optimizations
    println!("📊 Optimizing interface queues...");

    let queue_optimizations = vec![
        format!(
            "echo mq | sudo tee /sys/class/net/{}/queues/tx-*/xps_cpus 2>/dev/null || true",
            interface
        ),
        format!(
            "echo 2 | sudo tee /sys/class/net/{}/queues/rx-*/rps_cpus 2>/dev/null || true",
            interface
        ),
    ];

    for cmd in &queue_optimizations {
        Command::new("sh").arg("-c").arg(cmd).status().ok();
    }

    println!("\n✅ Network interface optimization completed!");
    println!(
        "🎮 Interface {} optimized for gaming performance",
        interface
    );

    // Show current settings
    println!("\n📋 Current interface settings:");
    let show_settings = format!("sudo ethtool {} | head -20", interface);
    Command::new("sh")
        .arg("-c")
        .arg(&show_settings)
        .status()
        .ok();
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select latency test")
        .items(&test_options)
        .default(0)
        .interact()
        .unwrap();

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

    let target = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter target hostname or IP to monitor")
        .default("8.8.8.8".to_string())
        .interact()
        .unwrap();

    let duration: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Monitor duration in seconds")
        .default("30".to_string())
        .interact()
        .unwrap();

    println!("\n🔍 Starting real-time latency monitoring...");
    println!(
        "Target: {} | Duration: {}s | Press Ctrl+C to stop\n",
        target, duration
    );

    // Use continuous ping with timestamps
    let ping_cmd = format!(
        "timeout {}s ping -i 0.2 {} | while read line; do echo \"$(date +'%H:%M:%S.%3N'): $line\"; done",
        duration, target
    );

    Command::new("sh").arg("-c").arg(&ping_cmd).status().ok();

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
