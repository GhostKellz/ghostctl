use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn bridge_management_menu() {
    loop {
        let options = [
            "🌉 Linux Bridge Operations",
            "🔧 Bridge Configuration",
            "🖥️ VM Bridge Integration",
            "🔍 Bridge Diagnostics",
            "🛡️ Bridge Firewall Rules",
            "📊 Bridge Monitoring",
            "🚀 Advanced Bridge Features",
            "💾 Bridge Configuration Backup",
            "⬅️ Back",
        ];

        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🌉 Linux Bridge Management")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match choice {
            0 => linux_bridge_operations(),
            1 => bridge_configuration(),
            2 => vm_bridge_integration(),
            3 => bridge_diagnostics(),
            4 => bridge_firewall_rules(),
            5 => bridge_monitoring(),
            6 => advanced_bridge_features(),
            7 => bridge_configuration_backup(),
            _ => break,
        }
    }
}

fn linux_bridge_operations() {
    loop {
        let options = [
            "📋 List All Bridges",
            "➕ Create New Bridge",
            "🗑️ Delete Bridge",
            "🔌 Add Interface to Bridge",
            "❌ Remove Interface from Bridge",
            "⬆️ Bring Bridge Up",
            "⬇️ Bring Bridge Down",
            "🔧 Configure Bridge Parameters",
            "⬅️ Back",
        ];

        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🌉 Bridge Operations")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match choice {
            0 => list_all_bridges(),
            1 => create_new_bridge(),
            2 => delete_bridge(),
            3 => add_interface_to_bridge(),
            4 => remove_interface_from_bridge(),
            5 => bring_bridge_up(),
            6 => bring_bridge_down(),
            7 => configure_bridge_parameters(),
            _ => break,
        }
    }
}

fn list_all_bridges() {
    println!("📋 Linux Bridge Status");
    println!("======================");

    // Method 1: Using brctl (if available)
    let brctl_output = Command::new("brctl").arg("show").output();

    if let Ok(out) = brctl_output {
        let bridge_info = String::from_utf8_lossy(&out.stdout);
        if !bridge_info.is_empty() && !bridge_info.contains("command not found") {
            println!("\n🔧 brctl output:");
            println!("{}", bridge_info);
        }
    }

    // Method 2: Using ip command (modern approach)
    println!("\n🔧 ip bridge information:");
    let ip_bridge_output = Command::new("ip")
        .args(&["link", "show", "type", "bridge"])
        .output();

    match ip_bridge_output {
        Ok(out) => {
            let ip_bridges = String::from_utf8_lossy(&out.stdout);
            if !ip_bridges.is_empty() {
                for line in ip_bridges.lines() {
                    if line.contains(": ") && line.contains("bridge") {
                        parse_bridge_line(line);
                    }
                }
            } else {
                println!("  ❌ No bridges found");
            }
        }
        Err(_) => println!("  ❌ Failed to get bridge information"),
    }

    // Method 3: Show bridge details with ip bridge command
    let bridge_show = Command::new("ip").args(&["bridge", "show"]).output();

    if let Ok(bridge_out) = bridge_show {
        let bridge_details = String::from_utf8_lossy(&bridge_out.stdout);
        if !bridge_details.is_empty() {
            println!("\n🌉 Bridge port details:");
            println!("{}", bridge_details);
        }
    }

    // Show network namespaces if any
    let netns_output = Command::new("ip").args(&["netns", "list"]).output();

    if let Ok(ns_out) = netns_output {
        let namespaces = String::from_utf8_lossy(&ns_out.stdout);
        if !namespaces.is_empty() && !namespaces.trim().is_empty() {
            println!("\n🔒 Network Namespaces:");
            for ns in namespaces.lines() {
                if !ns.trim().is_empty() {
                    println!("  📁 {}", ns.trim());
                }
            }
        }
    }

    // Show bridge statistics
    show_bridge_statistics();
}

fn parse_bridge_line(line: &str) {
    if let Some(colon_pos) = line.find(": ") {
        let after_colon = &line[colon_pos + 2..];
        if let Some(at_pos) = after_colon.find('@') {
            let bridge_name = &after_colon[..at_pos];
            println!("  🌉 Bridge: {}", bridge_name);
        } else if let Some(space_pos) = after_colon.find(' ') {
            let bridge_name = &after_colon[..space_pos];
            println!("  🌉 Bridge: {}", bridge_name);
        }

        // Show state
        if line.contains("state UP") {
            println!("     Status: ✅ UP");
        } else if line.contains("state DOWN") {
            println!("     Status: ❌ DOWN");
        }

        // Show MAC if present
        if let Some(link_pos) = line.find("link/ether") {
            let after_link = &line[link_pos + 11..];
            if let Some(space_pos) = after_link.find(' ') {
                let mac = &after_link[..space_pos];
                println!("     MAC: {}", mac);
            }
        }
    }
}

fn show_bridge_statistics() {
    println!("\n📊 Bridge Statistics:");

    // Get interface statistics
    if let Ok(entries) = fs::read_dir("/sys/class/net") {
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(file_name) = path.file_name() else {
                continue;
            };
            let interface_name = file_name.to_string_lossy();

            // Check if it's a bridge
            let bridge_check = path.join("bridge");
            if bridge_check.exists() {
                println!("  🌉 {}", interface_name);

                // Get bridge forward delay
                if let Ok(forward_delay) = fs::read_to_string(path.join("bridge/forward_delay")) {
                    println!("     Forward Delay: {} centiseconds", forward_delay.trim());
                }

                // Get bridge hello time
                if let Ok(hello_time) = fs::read_to_string(path.join("bridge/hello_time")) {
                    println!("     Hello Time: {} centiseconds", hello_time.trim());
                }

                // Get bridge max age
                if let Ok(max_age) = fs::read_to_string(path.join("bridge/max_age")) {
                    println!("     Max Age: {} centiseconds", max_age.trim());
                }

                // Get STP state
                if let Ok(stp_state) = fs::read_to_string(path.join("bridge/stp_state")) {
                    let stp_enabled = stp_state.trim() == "1";
                    println!(
                        "     STP: {}",
                        if stp_enabled {
                            "✅ Enabled"
                        } else {
                            "❌ Disabled"
                        }
                    );
                }

                // Show connected interfaces
                let brif_path = path.join("brif");
                if brif_path.exists()
                    && let Ok(interfaces) = fs::read_dir(&brif_path)
                {
                    let mut connected_interfaces = Vec::new();
                    for iface in interfaces.flatten() {
                        connected_interfaces.push(iface.file_name().to_string_lossy().to_string());
                    }

                    if !connected_interfaces.is_empty() {
                        println!("     Connected: {}", connected_interfaces.join(", "));
                    } else {
                        println!("     Connected: None");
                    }
                }
            }
        }
    }
}

fn create_new_bridge() {
    println!("➕ Create New Bridge");

    let Ok(bridge_name) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter bridge name")
        .default("br0".to_string())
        .interact()
    else {
        return;
    };

    // Validate bridge name
    if bridge_name.is_empty() || bridge_name.len() > 15 {
        println!("❌ Invalid bridge name (must be 1-15 characters)");
        return;
    }

    // Check if bridge already exists
    let check_cmd = Command::new("ip")
        .args(&["link", "show", &bridge_name])
        .output();

    if let Ok(check_out) = check_cmd {
        let check_result = String::from_utf8_lossy(&check_out.stdout);
        if !check_result.is_empty() {
            println!("❌ Bridge '{}' already exists", bridge_name);
            return;
        }
    }

    println!("🔧 Creating bridge '{}'...", bridge_name);

    // Create bridge using ip command
    let create_result = Command::new("sudo")
        .args(&["ip", "link", "add", "name", &bridge_name, "type", "bridge"])
        .status();

    match create_result {
        Ok(status) if status.success() => {
            println!("✅ Bridge '{}' created successfully", bridge_name);

            // Ask about additional configuration
            let Ok(configure_now) = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Configure bridge parameters now?")
                .default(true)
                .interact()
            else {
                return;
            };

            if configure_now {
                configure_new_bridge(&bridge_name);
            }

            // Ask about bringing it up
            let Ok(bring_up) = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Bring bridge up now?")
                .default(true)
                .interact()
            else {
                return;
            };

            if bring_up {
                let up_result = Command::new("sudo")
                    .args(&["ip", "link", "set", &bridge_name, "up"])
                    .status();

                match up_result {
                    Ok(status) if status.success() => {
                        println!("✅ Bridge '{}' is now up", bridge_name);
                    }
                    _ => println!("⚠️ Failed to bring bridge up"),
                }
            }
        }
        _ => println!("❌ Failed to create bridge"),
    }
}

fn configure_new_bridge(bridge_name: &str) {
    println!("🔧 Configuring bridge '{}'", bridge_name);

    // STP configuration
    let Ok(enable_stp) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable Spanning Tree Protocol (STP)?")
        .default(false)
        .interact()
    else {
        return;
    };

    if enable_stp {
        let stp_result = Command::new("sudo")
            .args(&[
                "ip",
                "link",
                "set",
                bridge_name,
                "type",
                "bridge",
                "stp_state",
                "1",
            ])
            .status();

        match stp_result {
            Ok(status) if status.success() => println!("✅ STP enabled"),
            _ => println!("⚠️ Failed to enable STP"),
        }
    }

    // Forward delay configuration
    let Ok(forward_delay) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Forward delay (4-30 seconds)")
        .default("15".to_string())
        .interact()
    else {
        return;
    };

    if let Ok(delay_val) = forward_delay.parse::<u32>()
        && (4..=30).contains(&delay_val)
    {
        let delay_centisec = delay_val * 100;
        let delay_result = Command::new("sudo")
            .args(&[
                "ip",
                "link",
                "set",
                bridge_name,
                "type",
                "bridge",
                "forward_delay",
                &delay_centisec.to_string(),
            ])
            .status();

        match delay_result {
            Ok(status) if status.success() => {
                println!("✅ Forward delay set to {} seconds", delay_val)
            }
            _ => println!("⚠️ Failed to set forward delay"),
        }
    }

    // Hello time configuration
    let Ok(hello_time) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Hello time (1-10 seconds)")
        .default("2".to_string())
        .interact()
    else {
        return;
    };

    if let Ok(hello_val) = hello_time.parse::<u32>()
        && (1..=10).contains(&hello_val)
    {
        let hello_centisec = hello_val * 100;
        let hello_result = Command::new("sudo")
            .args(&[
                "ip",
                "link",
                "set",
                bridge_name,
                "type",
                "bridge",
                "hello_time",
                &hello_centisec.to_string(),
            ])
            .status();

        match hello_result {
            Ok(status) if status.success() => {
                println!("✅ Hello time set to {} seconds", hello_val)
            }
            _ => println!("⚠️ Failed to set hello time"),
        }
    }

    // Max age configuration
    let Ok(max_age) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Max age (6-40 seconds)")
        .default("20".to_string())
        .interact()
    else {
        return;
    };

    if let Ok(age_val) = max_age.parse::<u32>()
        && (6..=40).contains(&age_val)
    {
        let age_centisec = age_val * 100;
        let age_result = Command::new("sudo")
            .args(&[
                "ip",
                "link",
                "set",
                bridge_name,
                "type",
                "bridge",
                "max_age",
                &age_centisec.to_string(),
            ])
            .status();

        match age_result {
            Ok(status) if status.success() => println!("✅ Max age set to {} seconds", age_val),
            _ => println!("⚠️ Failed to set max age"),
        }
    }

    // IP address configuration
    let Ok(assign_ip) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Assign IP address to bridge?")
        .default(false)
        .interact()
    else {
        return;
    };

    if assign_ip {
        let Ok(ip_address) = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter IP address with CIDR (e.g., 192.168.1.1/24)")
            .interact()
        else {
            return;
        };

        let ip_result = Command::new("sudo")
            .args(&["ip", "addr", "add", &ip_address, "dev", bridge_name])
            .status();

        match ip_result {
            Ok(status) if status.success() => println!("✅ IP address {} assigned", ip_address),
            _ => println!("⚠️ Failed to assign IP address"),
        }
    }
}

fn delete_bridge() {
    println!("🗑️ Delete Bridge");

    let bridges = get_bridge_list();
    if bridges.is_empty() {
        println!("❌ No bridges found");
        return;
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bridge to delete")
        .items(&bridges)
        .default(0)
        .interact()
    else {
        return;
    };

    let bridge_name = &bridges[choice];

    // Show bridge info before deletion
    show_bridge_details(bridge_name);

    let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Delete bridge '{}'? This will disconnect all attached interfaces",
            bridge_name
        ))
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        println!("🗑️ Deleting bridge '{}'...", bridge_name);

        // Bring bridge down first
        let down_result = Command::new("sudo")
            .args(&["ip", "link", "set", bridge_name, "down"])
            .status();

        if down_result.is_err() {
            println!("⚠️ Warning: Could not bring bridge down");
        }

        // Delete the bridge
        let delete_result = Command::new("sudo")
            .args(&["ip", "link", "delete", bridge_name, "type", "bridge"])
            .status();

        match delete_result {
            Ok(status) if status.success() => {
                println!("✅ Bridge '{}' deleted successfully", bridge_name);
            }
            _ => {
                println!("❌ Failed to delete bridge");
                println!("ℹ️ Make sure all interfaces are removed from the bridge first");
            }
        }
    }
}

fn get_bridge_list() -> Vec<String> {
    let mut bridges = Vec::new();

    let output = Command::new("ip")
        .args(&["link", "show", "type", "bridge"])
        .output();

    if let Ok(out) = output {
        let bridge_output = String::from_utf8_lossy(&out.stdout);
        for line in bridge_output.lines() {
            if line.contains(": ")
                && line.contains("bridge")
                && let Some(colon_pos) = line.find(": ")
            {
                let after_colon = &line[colon_pos + 2..];
                if let Some(at_pos) = after_colon.find('@') {
                    let bridge_name = &after_colon[..at_pos];
                    bridges.push(bridge_name.to_string());
                } else if let Some(space_pos) = after_colon.find(' ') {
                    let bridge_name = &after_colon[..space_pos];
                    bridges.push(bridge_name.to_string());
                }
            }
        }
    }

    bridges
}

fn show_bridge_details(bridge_name: &str) {
    println!("\n📋 Bridge Details: {}", bridge_name);

    // Show bridge info
    let info_output = Command::new("ip")
        .args(&["link", "show", bridge_name])
        .output();

    if let Ok(info_out) = info_output {
        let info = String::from_utf8_lossy(&info_out.stdout);
        println!("  Interface Info: {}", info.lines().next().unwrap_or(""));
    }

    // Show bridge parameters using sysfs
    let bridge_path = format!("/sys/class/net/{}/bridge", bridge_name);
    if Path::new(&bridge_path).exists() {
        // STP state
        if let Ok(stp) = fs::read_to_string(format!("{}/stp_state", bridge_path)) {
            println!(
                "  STP: {}",
                if stp.trim() == "1" {
                    "Enabled"
                } else {
                    "Disabled"
                }
            );
        }

        // Forward delay
        if let Ok(forward_delay) = fs::read_to_string(format!("{}/forward_delay", bridge_path)) {
            let delay_sec = forward_delay.trim().parse::<u32>().unwrap_or(0) / 100;
            println!("  Forward Delay: {} seconds", delay_sec);
        }

        // Connected interfaces
        let brif_path = format!("/sys/class/net/{}/brif", bridge_name);
        if let Ok(interfaces) = fs::read_dir(&brif_path) {
            let mut connected = Vec::new();
            for iface in interfaces.flatten() {
                connected.push(iface.file_name().to_string_lossy().to_string());
            }
            println!(
                "  Connected Interfaces: {}",
                if connected.is_empty() {
                    "None".to_string()
                } else {
                    connected.join(", ")
                }
            );
        }
    }

    // Show IP addresses
    let addr_output = Command::new("ip")
        .args(&["addr", "show", bridge_name])
        .output();

    if let Ok(addr_out) = addr_output {
        let addr_info = String::from_utf8_lossy(&addr_out.stdout);
        for line in addr_info.lines() {
            if line.trim().starts_with("inet ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    println!("  IP Address: {}", parts[1]);
                }
            }
        }
    }
}

fn add_interface_to_bridge() {
    println!("🔌 Add Interface to Bridge");

    let bridges = get_bridge_list();
    if bridges.is_empty() {
        println!("❌ No bridges found");
        return;
    }

    let Ok(bridge_choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bridge")
        .items(&bridges)
        .default(0)
        .interact()
    else {
        return;
    };

    let bridge_name = &bridges[bridge_choice];

    // Get available interfaces
    let available_interfaces = get_available_interfaces();
    if available_interfaces.is_empty() {
        println!("❌ No available interfaces found");
        return;
    }

    let Ok(interface_choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select interface to add")
        .items(&available_interfaces)
        .default(0)
        .interact()
    else {
        return;
    };

    let interface_name = &available_interfaces[interface_choice];

    println!(
        "🔌 Adding interface '{}' to bridge '{}'...",
        interface_name, bridge_name
    );

    // Add interface to bridge
    let add_result = Command::new("sudo")
        .args(&["ip", "link", "set", interface_name, "master", bridge_name])
        .status();

    match add_result {
        Ok(status) if status.success() => {
            println!(
                "✅ Interface '{}' added to bridge '{}'",
                interface_name, bridge_name
            );

            // Bring interface up if it's not already
            let up_result = Command::new("sudo")
                .args(&["ip", "link", "set", interface_name, "up"])
                .status();

            if up_result.is_ok() {
                println!("✅ Interface '{}' brought up", interface_name);
            }
        }
        _ => {
            println!("❌ Failed to add interface to bridge");
            println!("ℹ️ Make sure the interface is not in use and you have proper permissions");
        }
    }
}

fn get_available_interfaces() -> Vec<String> {
    let mut interfaces = Vec::new();

    let output = Command::new("ip").args(&["link", "show"]).output();

    if let Ok(out) = output {
        let link_output = String::from_utf8_lossy(&out.stdout);
        for line in link_output.lines() {
            if line.contains(": ")
                && !line.contains("lo:")
                && let Some(colon_pos) = line.find(": ")
            {
                let after_colon = &line[colon_pos + 2..];
                if let Some(at_pos) = after_colon.find('@') {
                    let iface_name = &after_colon[..at_pos];
                    // Skip bridges and other virtual interfaces
                    if !line.contains("bridge")
                        && !line.contains("vnet")
                        && !iface_name.starts_with("virbr")
                        && !iface_name.starts_with("br")
                    {
                        interfaces.push(iface_name.to_string());
                    }
                } else if let Some(space_pos) = after_colon.find(' ') {
                    let iface_name = &after_colon[..space_pos];
                    if !line.contains("bridge")
                        && !line.contains("vnet")
                        && !iface_name.starts_with("virbr")
                        && !iface_name.starts_with("br")
                    {
                        interfaces.push(iface_name.to_string());
                    }
                }
            }
        }
    }

    interfaces
}

fn remove_interface_from_bridge() {
    println!("❌ Remove Interface from Bridge");

    let bridges = get_bridge_list();
    if bridges.is_empty() {
        println!("❌ No bridges found");
        return;
    }

    let Ok(bridge_choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bridge")
        .items(&bridges)
        .default(0)
        .interact()
    else {
        return;
    };

    let bridge_name = &bridges[bridge_choice];

    // Get interfaces connected to this bridge
    let connected_interfaces = get_bridge_interfaces(bridge_name);
    if connected_interfaces.is_empty() {
        println!("❌ No interfaces connected to bridge '{}'", bridge_name);
        return;
    }

    let Ok(interface_choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select interface to remove")
        .items(&connected_interfaces)
        .default(0)
        .interact()
    else {
        return;
    };

    let interface_name = &connected_interfaces[interface_choice];

    let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Remove interface '{}' from bridge '{}'?",
            interface_name, bridge_name
        ))
        .default(true)
        .interact()
    else {
        return;
    };

    if confirm {
        println!(
            "❌ Removing interface '{}' from bridge '{}'...",
            interface_name, bridge_name
        );

        // Remove interface from bridge
        let remove_result = Command::new("sudo")
            .args(&["ip", "link", "set", interface_name, "nomaster"])
            .status();

        match remove_result {
            Ok(status) if status.success() => {
                println!(
                    "✅ Interface '{}' removed from bridge '{}'",
                    interface_name, bridge_name
                );
            }
            _ => println!("❌ Failed to remove interface from bridge"),
        }
    }
}

fn get_bridge_interfaces(bridge_name: &str) -> Vec<String> {
    let mut interfaces = Vec::new();

    let brif_path = format!("/sys/class/net/{}/brif", bridge_name);
    if let Ok(entries) = fs::read_dir(&brif_path) {
        for entry in entries.flatten() {
            interfaces.push(entry.file_name().to_string_lossy().to_string());
        }
    }

    interfaces
}

fn bring_bridge_up() {
    println!("⬆️ Bring Bridge Up");

    let bridges = get_bridge_list();
    if bridges.is_empty() {
        println!("❌ No bridges found");
        return;
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bridge to bring up")
        .items(&bridges)
        .default(0)
        .interact()
    else {
        return;
    };

    let bridge_name = &bridges[choice];

    println!("⬆️ Bringing bridge '{}' up...", bridge_name);

    let up_result = Command::new("sudo")
        .args(&["ip", "link", "set", bridge_name, "up"])
        .status();

    match up_result {
        Ok(status) if status.success() => {
            println!("✅ Bridge '{}' is now up", bridge_name);
        }
        _ => println!("❌ Failed to bring bridge up"),
    }
}

fn bring_bridge_down() {
    println!("⬇️ Bring Bridge Down");

    let bridges = get_bridge_list();
    if bridges.is_empty() {
        println!("❌ No bridges found");
        return;
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bridge to bring down")
        .items(&bridges)
        .default(0)
        .interact()
    else {
        return;
    };

    let bridge_name = &bridges[choice];

    let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Bring bridge '{}' down? This will interrupt network connectivity",
            bridge_name
        ))
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        println!("⬇️ Bringing bridge '{}' down...", bridge_name);

        let down_result = Command::new("sudo")
            .args(&["ip", "link", "set", bridge_name, "down"])
            .status();

        match down_result {
            Ok(status) if status.success() => {
                println!("✅ Bridge '{}' is now down", bridge_name);
            }
            _ => println!("❌ Failed to bring bridge down"),
        }
    }
}

fn configure_bridge_parameters() {
    println!("🔧 Configure Bridge Parameters");

    let bridges = get_bridge_list();
    if bridges.is_empty() {
        println!("❌ No bridges found");
        return;
    }

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bridge to configure")
        .items(&bridges)
        .default(0)
        .interact()
    else {
        return;
    };

    let bridge_name = &bridges[choice];

    show_bridge_details(bridge_name);

    let parameters = [
        "🔄 Toggle STP (Spanning Tree Protocol)",
        "⏱️ Set Forward Delay",
        "👋 Set Hello Time",
        "⏰ Set Max Age",
        "🏷️ Set Bridge Priority",
        "🌐 Assign/Modify IP Address",
        "📏 Set MTU",
    ];

    let Ok(param_choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select parameter to configure")
        .items(&parameters)
        .default(0)
        .interact()
    else {
        return;
    };

    match param_choice {
        0 => toggle_stp(bridge_name),
        1 => set_forward_delay(bridge_name),
        2 => set_hello_time(bridge_name),
        3 => set_max_age(bridge_name),
        4 => set_bridge_priority(bridge_name),
        5 => assign_ip_address(bridge_name),
        6 => set_mtu(bridge_name),
        _ => {}
    }
}

fn toggle_stp(bridge_name: &str) {
    println!("🔄 Toggle STP for bridge '{}'", bridge_name);

    // Get current STP state
    let stp_path = format!("/sys/class/net/{}/bridge/stp_state", bridge_name);
    let current_stp = if let Ok(stp_state) = fs::read_to_string(&stp_path) {
        stp_state.trim() == "1"
    } else {
        false
    };

    println!(
        "Current STP state: {}",
        if current_stp { "Enabled" } else { "Disabled" }
    );

    let new_state = !current_stp;
    let state_value = if new_state { "1" } else { "0" };

    let Ok(confirm) = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "{} STP?",
            if new_state { "Enable" } else { "Disable" }
        ))
        .default(true)
        .interact()
    else {
        return;
    };

    if confirm {
        let stp_result = Command::new("sudo")
            .args(&[
                "ip",
                "link",
                "set",
                bridge_name,
                "type",
                "bridge",
                "stp_state",
                state_value,
            ])
            .status();

        match stp_result {
            Ok(status) if status.success() => {
                println!(
                    "✅ STP {} for bridge '{}'",
                    if new_state { "enabled" } else { "disabled" },
                    bridge_name
                );
            }
            _ => println!("❌ Failed to modify STP state"),
        }
    }
}

fn set_forward_delay(bridge_name: &str) {
    println!("⏱️ Set Forward Delay for bridge '{}'", bridge_name);

    let Ok(delay_seconds) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter forward delay in seconds (4-30)")
        .default("15".to_string())
        .interact()
    else {
        return;
    };

    if let Ok(delay_val) = delay_seconds.parse::<u32>() {
        if (4..=30).contains(&delay_val) {
            let delay_centisec = delay_val * 100;

            let result = Command::new("sudo")
                .args(&[
                    "ip",
                    "link",
                    "set",
                    bridge_name,
                    "type",
                    "bridge",
                    "forward_delay",
                    &delay_centisec.to_string(),
                ])
                .status();

            match result {
                Ok(status) if status.success() => {
                    println!("✅ Forward delay set to {} seconds", delay_val);
                }
                _ => println!("❌ Failed to set forward delay"),
            }
        } else {
            println!("❌ Forward delay must be between 4 and 30 seconds");
        }
    } else {
        println!("❌ Invalid number format");
    }
}

fn set_hello_time(bridge_name: &str) {
    println!("👋 Set Hello Time for bridge '{}'", bridge_name);

    let Ok(hello_seconds) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter hello time in seconds (1-10)")
        .default("2".to_string())
        .interact()
    else {
        return;
    };

    if let Ok(hello_val) = hello_seconds.parse::<u32>() {
        if (1..=10).contains(&hello_val) {
            let hello_centisec = hello_val * 100;

            let result = Command::new("sudo")
                .args(&[
                    "ip",
                    "link",
                    "set",
                    bridge_name,
                    "type",
                    "bridge",
                    "hello_time",
                    &hello_centisec.to_string(),
                ])
                .status();

            match result {
                Ok(status) if status.success() => {
                    println!("✅ Hello time set to {} seconds", hello_val);
                }
                _ => println!("❌ Failed to set hello time"),
            }
        } else {
            println!("❌ Hello time must be between 1 and 10 seconds");
        }
    } else {
        println!("❌ Invalid number format");
    }
}

fn set_max_age(bridge_name: &str) {
    println!("⏰ Set Max Age for bridge '{}'", bridge_name);

    let Ok(max_seconds) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter max age in seconds (6-40)")
        .default("20".to_string())
        .interact()
    else {
        return;
    };

    if let Ok(max_val) = max_seconds.parse::<u32>() {
        if (6..=40).contains(&max_val) {
            let max_centisec = max_val * 100;

            let result = Command::new("sudo")
                .args(&[
                    "ip",
                    "link",
                    "set",
                    bridge_name,
                    "type",
                    "bridge",
                    "max_age",
                    &max_centisec.to_string(),
                ])
                .status();

            match result {
                Ok(status) if status.success() => {
                    println!("✅ Max age set to {} seconds", max_val);
                }
                _ => println!("❌ Failed to set max age"),
            }
        } else {
            println!("❌ Max age must be between 6 and 40 seconds");
        }
    } else {
        println!("❌ Invalid number format");
    }
}

fn set_bridge_priority(bridge_name: &str) {
    println!("🏷️ Set Bridge Priority for bridge '{}'", bridge_name);

    let Ok(priority) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter bridge priority (0-65535, lower = higher priority)")
        .default("32768".to_string())
        .interact()
    else {
        return;
    };

    if let Ok(priority_val) = priority.parse::<u32>() {
        if priority_val <= 65535 {
            let result = Command::new("sudo")
                .args(&[
                    "ip",
                    "link",
                    "set",
                    bridge_name,
                    "type",
                    "bridge",
                    "priority",
                    &priority_val.to_string(),
                ])
                .status();

            match result {
                Ok(status) if status.success() => {
                    println!("✅ Bridge priority set to {}", priority_val);
                }
                _ => println!("❌ Failed to set bridge priority"),
            }
        } else {
            println!("❌ Priority must be between 0 and 65535");
        }
    } else {
        println!("❌ Invalid number format");
    }
}

fn assign_ip_address(bridge_name: &str) {
    println!("🌐 Assign IP Address to bridge '{}'", bridge_name);

    // Show current IP addresses
    let addr_output = Command::new("ip")
        .args(&["addr", "show", bridge_name])
        .output();

    if let Ok(addr_out) = addr_output {
        let addr_info = String::from_utf8_lossy(&addr_out.stdout);
        println!("Current IP addresses:");
        for line in addr_info.lines() {
            if line.trim().starts_with("inet ") {
                println!("  {}", line.trim());
            }
        }
    }

    let actions = [
        "➕ Add IP address",
        "❌ Remove IP address",
        "🔄 Replace IP address",
    ];

    let Ok(action_choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action")
        .items(&actions)
        .default(0)
        .interact()
    else {
        return;
    };

    match action_choice {
        0 => {
            let Ok(ip_address) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter IP address with CIDR (e.g., 192.168.1.1/24)")
                .interact()
            else {
                return;
            };

            let result = Command::new("sudo")
                .args(&["ip", "addr", "add", &ip_address, "dev", bridge_name])
                .status();

            match result {
                Ok(status) if status.success() => {
                    println!("✅ IP address {} added", ip_address);
                }
                _ => println!("❌ Failed to add IP address"),
            }
        }
        1 => {
            let Ok(ip_address) = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter IP address to remove (with CIDR)")
                .interact()
            else {
                return;
            };

            let result = Command::new("sudo")
                .args(&["ip", "addr", "del", &ip_address, "dev", bridge_name])
                .status();

            match result {
                Ok(status) if status.success() => {
                    println!("✅ IP address {} removed", ip_address);
                }
                _ => println!("❌ Failed to remove IP address"),
            }
        }
        2 => {
            println!("🔄 To replace an IP address, first remove the old one, then add the new one");
        }
        _ => {}
    }
}

fn set_mtu(bridge_name: &str) {
    println!("📏 Set MTU for bridge '{}'", bridge_name);

    let Ok(mtu) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter MTU size (68-65536)")
        .default("1500".to_string())
        .interact()
    else {
        return;
    };

    if let Ok(mtu_val) = mtu.parse::<u32>() {
        if (68..=65536).contains(&mtu_val) {
            let result = Command::new("sudo")
                .args(&[
                    "ip",
                    "link",
                    "set",
                    bridge_name,
                    "mtu",
                    &mtu_val.to_string(),
                ])
                .status();

            match result {
                Ok(status) if status.success() => {
                    println!("✅ MTU set to {} bytes", mtu_val);
                }
                _ => println!("❌ Failed to set MTU"),
            }
        } else {
            println!("❌ MTU must be between 68 and 65536");
        }
    } else {
        println!("❌ Invalid number format");
    }
}

// Placeholder functions for other menu items
fn bridge_configuration() {
    println!("🔧 Bridge Configuration");
    configure_bridge_parameters();
}

fn vm_bridge_integration() {
    println!("🖥️ VM Bridge Integration");
    println!("ℹ️ This will show VM-specific bridge operations");

    // Check if libvirt is available
    if Command::new("which").arg("virsh").status().is_ok() {
        println!("✅ libvirt detected - VM bridge integration available");

        // Show VMs and their bridge connections
        show_vm_bridge_connections();
    } else {
        println!("⚠️ libvirt not available - install libvirt for VM integration");
    }
}

fn show_vm_bridge_connections() {
    println!("\n🖥️ VM Bridge Connections:");

    let vm_output = Command::new("virsh").args(&["list", "--all"]).output();

    if let Ok(vm_out) = vm_output {
        let vm_list = String::from_utf8_lossy(&vm_out.stdout);

        for line in vm_list.lines().skip(2) {
            if !line.trim().is_empty() && !line.contains("---") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let vm_name = parts[1];
                    println!("\n  VM: {}", vm_name);

                    // Get network interfaces for this VM
                    let iface_output = Command::new("virsh").args(&["domiflist", vm_name]).output();

                    if let Ok(iface_out) = iface_output {
                        let interfaces = String::from_utf8_lossy(&iface_out.stdout);

                        for iface_line in interfaces.lines().skip(2) {
                            if !iface_line.trim().is_empty() && !iface_line.contains("---") {
                                let iface_parts: Vec<&str> =
                                    iface_line.split_whitespace().collect();
                                if iface_parts.len() >= 3 {
                                    println!(
                                        "     🔌 Interface: {} -> Bridge: {}",
                                        iface_parts[0], iface_parts[2]
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn bridge_diagnostics() {
    println!("Bridge Diagnostics - Coming soon...");
}

fn bridge_firewall_rules() {
    println!("Bridge Firewall Rules - Coming soon...");
}

fn bridge_monitoring() {
    println!("Bridge Monitoring - Coming soon...");
}

fn advanced_bridge_features() {
    println!("Advanced Bridge Features - Coming soon...");
}

fn bridge_configuration_backup() {
    println!("Bridge Configuration Backup - Coming soon...");
}

/// Validates a bridge name according to Linux conventions
/// Bridge names must be 1-15 characters long and contain only alphanumeric and underscore
pub fn is_valid_bridge_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 15 {
        return false;
    }
    name.chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}

/// Parse MAC address from a string
pub fn parse_mac_address(mac_str: &str) -> Option<[u8; 6]> {
    let parts: Vec<&str> = mac_str.split(':').collect();
    if parts.len() != 6 {
        return None;
    }
    let mut mac = [0u8; 6];
    for (i, part) in parts.iter().enumerate() {
        match u8::from_str_radix(part, 16) {
            Ok(byte) => mac[i] = byte,
            Err(_) => return None,
        }
    }
    Some(mac)
}

/// Format MAC address bytes to string
pub fn format_mac_address(mac: &[u8; 6]) -> String {
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}

/// Validate STP forward delay value (4-30 seconds)
pub fn is_valid_forward_delay(delay_seconds: u32) -> bool {
    (4..=30).contains(&delay_seconds)
}

/// Validate STP hello time value (1-10 seconds)
pub fn is_valid_hello_time(hello_seconds: u32) -> bool {
    (1..=10).contains(&hello_seconds)
}

/// Validate STP max age value (6-40 seconds)
pub fn is_valid_max_age(age_seconds: u32) -> bool {
    (6..=40).contains(&age_seconds)
}

/// Validate MTU value (68-65536)
pub fn is_valid_mtu(mtu: u32) -> bool {
    (68..=65536).contains(&mtu)
}

/// Validate bridge priority (0-65535)
pub fn is_valid_bridge_priority(priority: u32) -> bool {
    priority <= 65535
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Bridge Name Validation Tests ====================

    #[test]
    fn test_is_valid_bridge_name_normal() {
        assert!(is_valid_bridge_name("br0"));
        assert!(is_valid_bridge_name("bridge1"));
        assert!(is_valid_bridge_name("my_bridge"));
    }

    #[test]
    fn test_is_valid_bridge_name_with_dash() {
        assert!(is_valid_bridge_name("br-lan"));
        assert!(is_valid_bridge_name("vm-bridge"));
    }

    #[test]
    fn test_is_valid_bridge_name_empty() {
        assert!(!is_valid_bridge_name(""));
    }

    #[test]
    fn test_is_valid_bridge_name_too_long() {
        assert!(!is_valid_bridge_name("this_name_is_way_too_long"));
        assert!(!is_valid_bridge_name("1234567890123456")); // 16 chars
    }

    #[test]
    fn test_is_valid_bridge_name_max_length() {
        assert!(is_valid_bridge_name("123456789012345")); // 15 chars - max allowed
    }

    #[test]
    fn test_is_valid_bridge_name_invalid_chars() {
        assert!(!is_valid_bridge_name("br.0"));
        assert!(!is_valid_bridge_name("br/0"));
        assert!(!is_valid_bridge_name("br@0"));
    }

    // ==================== MAC Address Tests ====================

    #[test]
    fn test_parse_mac_address_valid() {
        let mac = parse_mac_address("aa:bb:cc:dd:ee:ff");
        assert!(mac.is_some());
        assert_eq!(mac.unwrap(), [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);
    }

    #[test]
    fn test_parse_mac_address_uppercase() {
        let mac = parse_mac_address("AA:BB:CC:DD:EE:FF");
        assert!(mac.is_some());
        assert_eq!(mac.unwrap(), [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);
    }

    #[test]
    fn test_parse_mac_address_zeros() {
        let mac = parse_mac_address("00:00:00:00:00:00");
        assert!(mac.is_some());
        assert_eq!(mac.unwrap(), [0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_parse_mac_address_invalid_format() {
        assert!(parse_mac_address("aa:bb:cc:dd:ee").is_none());
        assert!(parse_mac_address("aa:bb:cc:dd:ee:ff:00").is_none());
        assert!(parse_mac_address("aabbccddeeff").is_none());
    }

    #[test]
    fn test_parse_mac_address_invalid_hex() {
        assert!(parse_mac_address("gg:bb:cc:dd:ee:ff").is_none());
        assert!(parse_mac_address("aa:zz:cc:dd:ee:ff").is_none());
    }

    #[test]
    fn test_format_mac_address() {
        let mac = [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
        assert_eq!(format_mac_address(&mac), "aa:bb:cc:dd:ee:ff");
    }

    #[test]
    fn test_format_mac_address_zeros() {
        let mac = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(format_mac_address(&mac), "00:00:00:00:00:00");
    }

    #[test]
    fn test_mac_roundtrip() {
        let original = "ab:cd:ef:12:34:56";
        let parsed = parse_mac_address(original).unwrap();
        let formatted = format_mac_address(&parsed);
        assert_eq!(formatted, original);
    }

    // ==================== STP Parameter Validation Tests ====================

    #[test]
    fn test_is_valid_forward_delay() {
        assert!(is_valid_forward_delay(4));
        assert!(is_valid_forward_delay(15));
        assert!(is_valid_forward_delay(30));
    }

    #[test]
    fn test_is_valid_forward_delay_out_of_range() {
        assert!(!is_valid_forward_delay(3));
        assert!(!is_valid_forward_delay(31));
        assert!(!is_valid_forward_delay(0));
    }

    #[test]
    fn test_is_valid_hello_time() {
        assert!(is_valid_hello_time(1));
        assert!(is_valid_hello_time(2));
        assert!(is_valid_hello_time(10));
    }

    #[test]
    fn test_is_valid_hello_time_out_of_range() {
        assert!(!is_valid_hello_time(0));
        assert!(!is_valid_hello_time(11));
    }

    #[test]
    fn test_is_valid_max_age() {
        assert!(is_valid_max_age(6));
        assert!(is_valid_max_age(20));
        assert!(is_valid_max_age(40));
    }

    #[test]
    fn test_is_valid_max_age_out_of_range() {
        assert!(!is_valid_max_age(5));
        assert!(!is_valid_max_age(41));
    }

    // ==================== MTU Validation Tests ====================

    #[test]
    fn test_is_valid_mtu_normal() {
        assert!(is_valid_mtu(1500));
        assert!(is_valid_mtu(9000)); // Jumbo frames
    }

    #[test]
    fn test_is_valid_mtu_boundaries() {
        assert!(is_valid_mtu(68));
        assert!(is_valid_mtu(65536));
    }

    #[test]
    fn test_is_valid_mtu_out_of_range() {
        assert!(!is_valid_mtu(67));
        assert!(!is_valid_mtu(65537));
        assert!(!is_valid_mtu(0));
    }

    // ==================== Bridge Priority Tests ====================

    #[test]
    fn test_is_valid_bridge_priority() {
        assert!(is_valid_bridge_priority(0));
        assert!(is_valid_bridge_priority(32768)); // Default
        assert!(is_valid_bridge_priority(65535));
    }

    #[test]
    fn test_is_valid_bridge_priority_out_of_range() {
        assert!(!is_valid_bridge_priority(65536));
        assert!(!is_valid_bridge_priority(100000));
    }
}
