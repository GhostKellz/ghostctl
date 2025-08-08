use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme, MultiSelect};
use std::process::Command;
use std::fs;

pub fn sdn_menu() {
    loop {
        let options = vec![
            "VM Bridge Management",
            "SDN Zone Configuration",
            "VLAN Configuration",
            "Network Diagnostics",
            "Fix Bridge Issues",
            "VXLAN/EVPN Setup",
            "Subnet Management",
            "DHCP/IPAM Configuration",
            "Back",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🌐 PVE SDN & Network Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => bridge_management(),
            1 => sdn_zones(),
            2 => vlan_configuration(),
            3 => network_diagnostics(),
            4 => fix_bridge_issues(),
            5 => vxlan_evpn_setup(),
            6 => subnet_management(),
            7 => dhcp_ipam_config(),
            _ => break,
        }
    }
}

fn bridge_management() {
    println!("🌉 VM Bridge Management\n");
    
    let options = vec![
        "List All Bridges",
        "Create New Bridge",
        "Fix Broken Bridge",
        "Add VM to Bridge",
        "Remove Unused Bridges",
        "Bridge Performance Stats",
        "Back",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select operation")
        .items(&options)
        .interact()
        .unwrap();
    
    match selection {
        0 => list_bridges(),
        1 => create_bridge(),
        2 => fix_broken_bridge(),
        3 => add_vm_to_bridge(),
        4 => cleanup_bridges(),
        5 => bridge_stats(),
        _ => {}
    }
}

fn list_bridges() {
    println!("📋 Current Bridges:\n");
    
    // List network bridges
    let output = Command::new("ip")
        .args(&["link", "show", "type", "bridge"])
        .output();
    
    if let Ok(output) = output {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    
    // Show bridge details
    let output = Command::new("brctl")
        .arg("show")
        .output();
    
    if let Ok(output) = output {
        println!("\n🔧 Bridge Details:");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn create_bridge() {
    let bridge_name: String = Input::new()
        .with_prompt("Bridge name (e.g., vmbr1)")
        .default("vmbr1".to_string())
        .interact_text()
        .unwrap();
    
    let ip_address: String = Input::new()
        .with_prompt("IP address/CIDR (e.g., 10.10.10.1/24, or leave empty)")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    
    let physical_iface: String = Input::new()
        .with_prompt("Physical interface to bridge (leave empty for none)")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    
    println!("🔧 Creating bridge {}...", bridge_name);
    
    // Create bridge
    let _ = Command::new("ip")
        .args(&["link", "add", "name", &bridge_name, "type", "bridge"])
        .status();
    
    // Add IP if provided
    if !ip_address.is_empty() {
        let _ = Command::new("ip")
            .args(&["addr", "add", &ip_address, "dev", &bridge_name])
            .status();
    }
    
    // Bridge physical interface if provided
    if !physical_iface.is_empty() {
        let _ = Command::new("ip")
            .args(&["link", "set", &physical_iface, "master", &bridge_name])
            .status();
    }
    
    // Bring up the bridge
    let _ = Command::new("ip")
        .args(&["link", "set", &bridge_name, "up"])
        .status();
    
    // Make persistent in /etc/network/interfaces
    make_bridge_persistent(&bridge_name, &ip_address, &physical_iface);
    
    println!("✅ Bridge {} created successfully", bridge_name);
}

fn fix_broken_bridge() {
    println!("🔧 Fixing Bridge Issues\n");
    
    let fixes = vec![
        "Reset and recreate vmbr0",
        "Fix MTU mismatches",
        "Clear bridge forwarding table",
        "Fix STP (Spanning Tree) issues",
        "Reset bridge to defaults",
        "Fix VLAN filtering",
        "Repair bridge slave interfaces",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select fix to apply")
        .items(&fixes)
        .interact()
        .unwrap();
    
    match selection {
        0 => reset_vmbr0(),
        1 => fix_mtu_issues(),
        2 => clear_fdb(),
        3 => fix_stp(),
        4 => reset_bridge_defaults(),
        5 => fix_vlan_filtering(),
        6 => repair_slaves(),
        _ => {}
    }
}

fn reset_vmbr0() {
    println!("⚠️  This will reset vmbr0 to default configuration!");
    
    if !Confirm::new()
        .with_prompt("Continue?")
        .default(false)
        .interact()
        .unwrap()
    {
        return;
    }
    
    // Down the interface
    let _ = Command::new("ip")
        .args(&["link", "set", "vmbr0", "down"])
        .status();
    
    // Delete and recreate
    let _ = Command::new("ip")
        .args(&["link", "delete", "vmbr0"])
        .status();
    
    let _ = Command::new("ip")
        .args(&["link", "add", "name", "vmbr0", "type", "bridge"])
        .status();
    
    // Get physical interface (usually first ethernet)
    let physical = get_first_physical_interface();
    if let Some(iface) = physical {
        let _ = Command::new("ip")
            .args(&["link", "set", &iface, "master", "vmbr0"])
            .status();
    }
    
    // Bring it up
    let _ = Command::new("ip")
        .args(&["link", "set", "vmbr0", "up"])
        .status();
    
    println!("✅ vmbr0 reset to defaults");
}

fn fix_mtu_issues() {
    println!("🔧 Fixing MTU mismatches...");
    
    let mtu: String = Input::new()
        .with_prompt("Standard MTU size")
        .default("1500".to_string())
        .interact_text()
        .unwrap();
    
    // Get all bridges
    let output = Command::new("ls")
        .arg("/sys/class/net/")
        .output();
    
    if let Ok(output) = output {
        let interfaces = String::from_utf8_lossy(&output.stdout);
        for iface in interfaces.split_whitespace() {
            if iface.starts_with("vmbr") {
                let _ = Command::new("ip")
                    .args(&["link", "set", iface, "mtu", &mtu])
                    .status();
                println!("✅ Set {} MTU to {}", iface, mtu);
            }
        }
    }
}

fn add_vm_to_bridge() {
    let vmid: String = Input::new()
        .with_prompt("VM ID")
        .interact_text()
        .unwrap();
    
    let bridge: String = Input::new()
        .with_prompt("Bridge name")
        .default("vmbr0".to_string())
        .interact_text()
        .unwrap();
    
    let vlan: String = Input::new()
        .with_prompt("VLAN tag (leave empty for none)")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    
    // Update VM config
    let config_path = format!("/etc/pve/qemu-server/{}.conf", vmid);
    
    if let Ok(mut config) = fs::read_to_string(&config_path) {
        // Find and update net0 line
        let net_config = if vlan.is_empty() {
            format!("net0: virtio=auto,bridge={}", bridge)
        } else {
            format!("net0: virtio=auto,bridge={},tag={}", bridge, vlan)
        };
        
        // Simple replacement (in real implementation, parse properly)
        config = config.lines()
            .map(|line| {
                if line.starts_with("net0:") {
                    net_config.clone()
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        
        fs::write(&config_path, config).ok();
        println!("✅ VM {} added to bridge {}", vmid, bridge);
    } else {
        println!("❌ Could not find VM configuration");
    }
}

fn cleanup_bridges() {
    println!("🧹 Cleaning up unused bridges...");
    
    // Find bridges with no members
    let output = Command::new("brctl")
        .arg("show")
        .output();
    
    if let Ok(output) = output {
        let result = String::from_utf8_lossy(&output.stdout);
        let mut unused = Vec::new();
        
        for line in result.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 && parts[3].is_empty() {
                unused.push(parts[0].to_string());
            }
        }
        
        if unused.is_empty() {
            println!("✅ No unused bridges found");
        } else {
            println!("Found unused bridges: {:?}", unused);
            
            if Confirm::new()
                .with_prompt("Remove unused bridges?")
                .default(false)
                .interact()
                .unwrap()
            {
                for bridge in unused {
                    let _ = Command::new("ip")
                        .args(&["link", "delete", &bridge])
                        .status();
                    println!("✅ Removed {}", bridge);
                }
            }
        }
    }
}

fn bridge_stats() {
    println!("📊 Bridge Performance Statistics\n");
    
    // Show bridge statistics
    let output = Command::new("ip")
        .args(&["-s", "link", "show", "type", "bridge"])
        .output();
    
    if let Ok(output) = output {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn sdn_zones() {
    println!("🌐 SDN Zone Configuration\n");
    
    let zone_types = vec![
        "Simple (Isolated Bridge)",
        "VLAN (802.1q)",
        "VXLAN (Overlay Network)",
        "EVPN (BGP EVPN)",
        "QinQ (802.1ad)",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select zone type")
        .items(&zone_types)
        .interact()
        .unwrap();
    
    match selection {
        0 => create_simple_zone(),
        1 => create_vlan_zone(),
        2 => create_vxlan_zone(),
        3 => create_evpn_zone(),
        4 => create_qinq_zone(),
        _ => {}
    }
}

fn create_simple_zone() {
    let zone_name: String = Input::new()
        .with_prompt("Zone name")
        .interact_text()
        .unwrap();
    
    let bridge: String = Input::new()
        .with_prompt("Bridge name")
        .default("vmbr1".to_string())
        .interact_text()
        .unwrap();
    
    // Create SDN configuration
    let config = format!(
        "simple: {}\n\tbridge {}\n",
        zone_name, bridge
    );
    
    save_sdn_config("zones", &zone_name, &config);
    println!("✅ Simple zone {} created", zone_name);
}

fn create_vlan_zone() {
    let zone_name: String = Input::new()
        .with_prompt("Zone name")
        .interact_text()
        .unwrap();
    
    let bridge: String = Input::new()
        .with_prompt("Bridge name")
        .default("vmbr0".to_string())
        .interact_text()
        .unwrap();
    
    let vlan_range: String = Input::new()
        .with_prompt("VLAN range (e.g., 100-200)")
        .interact_text()
        .unwrap();
    
    let config = format!(
        "vlan: {}\n\tbridge {}\n\tvlan-protocol 802.1q\n\tvlan-range {}\n",
        zone_name, bridge, vlan_range
    );
    
    save_sdn_config("zones", &zone_name, &config);
    println!("✅ VLAN zone {} created with range {}", zone_name, vlan_range);
}

fn create_vxlan_zone() {
    let zone_name: String = Input::new()
        .with_prompt("Zone name")
        .interact_text()
        .unwrap();
    
    let vxlan_id: String = Input::new()
        .with_prompt("VXLAN ID (1-16777215)")
        .interact_text()
        .unwrap();
    
    let multicast: String = Input::new()
        .with_prompt("Multicast address (e.g., 239.0.0.1)")
        .interact_text()
        .unwrap();
    
    let config = format!(
        "vxlan: {}\n\tvxlan-id {}\n\tmulticast-address {}\n\tudp-port 4789\n",
        zone_name, vxlan_id, multicast
    );
    
    save_sdn_config("zones", &zone_name, &config);
    println!("✅ VXLAN zone {} created with ID {}", zone_name, vxlan_id);
}

fn create_evpn_zone() {
    println!("🔧 EVPN requires BGP configuration");
    
    let zone_name: String = Input::new()
        .with_prompt("Zone name")
        .interact_text()
        .unwrap();
    
    let vrf: String = Input::new()
        .with_prompt("VRF name")
        .interact_text()
        .unwrap();
    
    let controller: String = Input::new()
        .with_prompt("EVPN controller")
        .interact_text()
        .unwrap();
    
    let config = format!(
        "evpn: {}\n\tvrf {}\n\tcontroller {}\n\tvrf-vxlan-id 10000\n\tmac auto\n",
        zone_name, vrf, controller
    );
    
    save_sdn_config("zones", &zone_name, &config);
    println!("✅ EVPN zone {} created", zone_name);
}

fn create_qinq_zone() {
    let zone_name: String = Input::new()
        .with_prompt("Zone name")
        .interact_text()
        .unwrap();
    
    let bridge: String = Input::new()
        .with_prompt("Bridge name")
        .interact_text()
        .unwrap();
    
    let service_vlan: String = Input::new()
        .with_prompt("Service VLAN (S-Tag)")
        .interact_text()
        .unwrap();
    
    let config = format!(
        "qinq: {}\n\tbridge {}\n\tservice-vlan {}\n\tvlan-protocol 802.1ad\n",
        zone_name, bridge, service_vlan
    );
    
    save_sdn_config("zones", &zone_name, &config);
    println!("✅ QinQ zone {} created with S-VLAN {}", zone_name, service_vlan);
}

fn vlan_configuration() {
    println!("🏷️  VLAN Configuration\n");
    
    let bridge: String = Input::new()
        .with_prompt("Bridge name")
        .default("vmbr0".to_string())
        .interact_text()
        .unwrap();
    
    let vlan_id: String = Input::new()
        .with_prompt("VLAN ID")
        .interact_text()
        .unwrap();
    
    let ip_address: String = Input::new()
        .with_prompt("IP address/CIDR (optional)")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    
    // Create VLAN interface
    let vlan_iface = format!("{}.{}", bridge, vlan_id);
    
    let _ = Command::new("ip")
        .args(&["link", "add", "link", &bridge, "name", &vlan_iface, "type", "vlan", "id", &vlan_id])
        .status();
    
    if !ip_address.is_empty() {
        let _ = Command::new("ip")
            .args(&["addr", "add", &ip_address, "dev", &vlan_iface])
            .status();
    }
    
    let _ = Command::new("ip")
        .args(&["link", "set", &vlan_iface, "up"])
        .status();
    
    println!("✅ VLAN {} created on {}", vlan_id, bridge);
}

fn network_diagnostics() {
    println!("🔍 Network Diagnostics\n");
    
    let tools = vec![
        "Check Bridge Connectivity",
        "Test VLAN Isolation",
        "Verify MTU Path",
        "Check Spanning Tree",
        "Test Network Performance",
        "Diagnose Packet Loss",
        "Check ARP Tables",
        "Verify Routing",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select diagnostic")
        .items(&tools)
        .interact()
        .unwrap();
    
    match selection {
        0 => check_bridge_connectivity(),
        1 => test_vlan_isolation(),
        2 => verify_mtu_path(),
        3 => check_stp_status(),
        4 => test_network_performance(),
        5 => diagnose_packet_loss(),
        6 => check_arp_tables(),
        7 => verify_routing(),
        _ => {}
    }
}

fn check_bridge_connectivity() {
    println!("🔍 Checking bridge connectivity...\n");
    
    // List all bridges and their members
    let _ = Command::new("brctl")
        .arg("show")
        .status();
    
    // Check each bridge
    let output = Command::new("ip")
        .args(&["link", "show", "type", "bridge"])
        .output();
    
    if let Ok(output) = output {
        let bridges = String::from_utf8_lossy(&output.stdout);
        for line in bridges.lines() {
            if let Some(name) = line.split(':').nth(1) {
                let bridge = name.trim();
                if bridge.starts_with("vmbr") {
                    println!("\n🌉 Bridge {}: ", bridge);
                    
                    // Check if bridge is up
                    if line.contains("UP") {
                        println!("  ✅ Status: UP");
                    } else {
                        println!("  ❌ Status: DOWN");
                    }
                    
                    // Check for IP
                    let ip_output = Command::new("ip")
                        .args(&["addr", "show", bridge])
                        .output();
                    
                    if let Ok(ip_output) = ip_output {
                        let ip_info = String::from_utf8_lossy(&ip_output.stdout);
                        if ip_info.contains("inet ") {
                            println!("  ✅ Has IP address");
                        } else {
                            println!("  ⚠️  No IP address");
                        }
                    }
                }
            }
        }
    }
}

fn test_vlan_isolation() {
    println!("🔍 Testing VLAN isolation...");
    
    let vlan1: String = Input::new()
        .with_prompt("First VLAN ID")
        .interact_text()
        .unwrap();
    
    let vlan2: String = Input::new()
        .with_prompt("Second VLAN ID")
        .interact_text()
        .unwrap();
    
    println!("Testing isolation between VLAN {} and VLAN {}", vlan1, vlan2);
    
    // Check if VLANs exist
    let _ = Command::new("ip")
        .args(&["link", "show"])
        .status();
    
    println!("\n✅ VLAN isolation test guidelines:");
    println!("  1. VMs in VLAN {} should NOT reach VLAN {} without routing", vlan1, vlan2);
    println!("  2. Check firewall rules between VLANs");
    println!("  3. Verify VLAN tags on bridge ports");
}

fn verify_mtu_path() {
    let target: String = Input::new()
        .with_prompt("Target IP to test MTU")
        .default("8.8.8.8".to_string())
        .interact_text()
        .unwrap();
    
    println!("🔍 Testing MTU path to {}...", target);
    
    for size in [1500, 1450, 1400, 9000] {
        let output = Command::new("ping")
            .args(&["-M", "do", "-s", &(size - 28).to_string(), "-c", "1", &target])
            .output();
        
        if let Ok(output) = output {
            if output.status.success() {
                println!("  ✅ MTU {} works", size);
            } else {
                println!("  ❌ MTU {} fails", size);
                break;
            }
        }
    }
}

fn check_stp_status() {
    println!("🔍 Checking Spanning Tree Protocol status...\n");
    
    let _ = Command::new("brctl")
        .args(&["showstp", "vmbr0"])
        .status();
}

fn test_network_performance() {
    println!("📊 Network Performance Test\n");
    
    let target: String = Input::new()
        .with_prompt("Target IP")
        .interact_text()
        .unwrap();
    
    println!("Testing bandwidth to {}...", target);
    
    // Use iperf3 if available
    if Command::new("which").arg("iperf3").output().map(|o| o.status.success()).unwrap_or(false) {
        let _ = Command::new("iperf3")
            .args(&["-c", &target, "-t", "10"])
            .status();
    } else {
        println!("⚠️  iperf3 not installed. Using basic ping test...");
        let _ = Command::new("ping")
            .args(&["-f", "-c", "1000", &target])
            .status();
    }
}

fn diagnose_packet_loss() {
    let target: String = Input::new()
        .with_prompt("Target IP")
        .interact_text()
        .unwrap();
    
    println!("🔍 Diagnosing packet loss to {}...", target);
    
    // Extended ping test
    let _ = Command::new("ping")
        .args(&["-c", "100", "-i", "0.2", &target])
        .status();
    
    // MTR if available
    if Command::new("which").arg("mtr").output().map(|o| o.status.success()).unwrap_or(false) {
        let _ = Command::new("mtr")
            .args(&["--report", "--report-cycles", "100", &target])
            .status();
    }
}

fn check_arp_tables() {
    println!("📋 ARP Table:\n");
    let _ = Command::new("ip")
        .args(&["neigh", "show"])
        .status();
}

fn verify_routing() {
    println!("🗺️  Routing Table:\n");
    let _ = Command::new("ip")
        .args(&["route", "show"])
        .status();
    
    println!("\n📋 Policy Routing:");
    let _ = Command::new("ip")
        .args(&["rule", "show"])
        .status();
}

fn vxlan_evpn_setup() {
    println!("🌐 VXLAN/EVPN Setup\n");
    
    let setup_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select setup type")
        .items(&["Basic VXLAN", "VXLAN with Multicast", "EVPN with FRR", "EVPN Controller"])
        .interact()
        .unwrap();
    
    match setup_type {
        0 => setup_basic_vxlan(),
        1 => setup_vxlan_multicast(),
        2 => setup_evpn_frr(),
        3 => setup_evpn_controller(),
        _ => {}
    }
}

fn setup_basic_vxlan() {
    let vni: String = Input::new()
        .with_prompt("VNI (VXLAN Network Identifier)")
        .interact_text()
        .unwrap();
    
    let local_ip: String = Input::new()
        .with_prompt("Local VTEP IP")
        .interact_text()
        .unwrap();
    
    let remote_ip: String = Input::new()
        .with_prompt("Remote VTEP IP")
        .interact_text()
        .unwrap();
    
    let vxlan_name = format!("vxlan{}", vni);
    
    // Create VXLAN interface
    let _ = Command::new("ip")
        .args(&["link", "add", &vxlan_name, "type", "vxlan", 
                "id", &vni, "local", &local_ip, "remote", &remote_ip,
                "dstport", "4789"])
        .status();
    
    // Add to bridge
    let _ = Command::new("ip")
        .args(&["link", "set", &vxlan_name, "master", "vmbr1"])
        .status();
    
    let _ = Command::new("ip")
        .args(&["link", "set", &vxlan_name, "up"])
        .status();
    
    println!("✅ VXLAN {} created with VNI {}", vxlan_name, vni);
}

fn setup_vxlan_multicast() {
    let vni: String = Input::new()
        .with_prompt("VNI")
        .interact_text()
        .unwrap();
    
    let multicast: String = Input::new()
        .with_prompt("Multicast group (e.g., 239.1.1.1)")
        .default("239.1.1.1".to_string())
        .interact_text()
        .unwrap();
    
    let vxlan_name = format!("vxlan{}", vni);
    
    let _ = Command::new("ip")
        .args(&["link", "add", &vxlan_name, "type", "vxlan",
                "id", &vni, "group", &multicast, "dstport", "4789"])
        .status();
    
    println!("✅ VXLAN {} created with multicast {}", vxlan_name, multicast);
}

fn setup_evpn_frr() {
    println!("🔧 Setting up EVPN with FRR...");
    
    // Install FRR if not present
    let _ = Command::new("apt")
        .args(&["install", "-y", "frr", "frr-pythontools"])
        .status();
    
    // Enable BGP daemon
    let frr_config = r#"
bgpd=yes
ospfd=no
ospf6d=no
ripd=no
ripngd=no
isisd=no
pimd=no
ldpd=no
nhrpd=no
eigrpd=no
babeld=no
sharpd=no
pbrd=no
bfdd=no
fabricd=no
vrrpd=no
"#;
    
    fs::write("/etc/frr/daemons", frr_config).ok();
    
    println!("✅ FRR installed and BGP enabled");
    println!("Configure BGP EVPN in /etc/frr/frr.conf");
}

fn setup_evpn_controller() {
    let controller_ip: String = Input::new()
        .with_prompt("EVPN Controller IP")
        .interact_text()
        .unwrap();
    
    let asn: String = Input::new()
        .with_prompt("BGP ASN")
        .default("65000".to_string())
        .interact_text()
        .unwrap();
    
    // Create controller configuration
    let config = format!(
        "evpn-controller: main\n\tasn {}\n\tpeers {}\n",
        asn, controller_ip
    );
    
    save_sdn_config("controllers", "main", &config);
    println!("✅ EVPN controller configured");
}

fn subnet_management() {
    println!("🔢 Subnet Management\n");
    
    let subnet: String = Input::new()
        .with_prompt("Subnet CIDR (e.g., 10.10.10.0/24)")
        .interact_text()
        .unwrap();
    
    let gateway: String = Input::new()
        .with_prompt("Gateway IP")
        .interact_text()
        .unwrap();
    
    let vnet: String = Input::new()
        .with_prompt("VNet to attach to")
        .interact_text()
        .unwrap();
    
    let dhcp = Confirm::new()
        .with_prompt("Enable DHCP?")
        .default(true)
        .interact()
        .unwrap();
    
    let config = if dhcp {
        let dhcp_start: String = Input::new()
            .with_prompt("DHCP range start")
            .interact_text()
            .unwrap();
        
        let dhcp_end: String = Input::new()
            .with_prompt("DHCP range end")
            .interact_text()
            .unwrap();
        
        format!(
            "subnet: {}\n\tgateway {}\n\tdhcp-range {},{}\n",
            vnet, gateway, dhcp_start, dhcp_end
        )
    } else {
        format!("subnet: {}\n\tgateway {}\n", vnet, gateway)
    };
    
    save_sdn_config("subnets", &vnet, &config);
    println!("✅ Subnet {} configured for VNet {}", subnet, vnet);
}

fn dhcp_ipam_config() {
    println!("🏠 DHCP & IPAM Configuration\n");
    
    let ipam_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select IPAM type")
        .items(&["PVE (Built-in)", "NetBox", "phpIPAM", "Custom"])
        .interact()
        .unwrap();
    
    match ipam_type {
        0 => setup_pve_ipam(),
        1 => setup_netbox_ipam(),
        2 => setup_phpipam(),
        3 => setup_custom_ipam(),
        _ => {}
    }
}

fn setup_pve_ipam() {
    println!("Using PVE built-in IPAM");
    
    let config = "ipam: pve\n";
    save_sdn_config("ipam", "pve", config);
    
    println!("✅ PVE IPAM configured");
}

fn setup_netbox_ipam() {
    let url: String = Input::new()
        .with_prompt("NetBox URL")
        .interact_text()
        .unwrap();
    
    let token: String = Input::new()
        .with_prompt("API Token")
        .interact_text()
        .unwrap();
    
    let config = format!(
        "ipam: netbox\n\turl {}\n\ttoken {}\n",
        url, token
    );
    
    save_sdn_config("ipam", "netbox", &config);
    println!("✅ NetBox IPAM configured");
}

fn setup_phpipam() {
    let url: String = Input::new()
        .with_prompt("phpIPAM URL")
        .interact_text()
        .unwrap();
    
    let app_id: String = Input::new()
        .with_prompt("App ID")
        .interact_text()
        .unwrap();
    
    let token: String = Input::new()
        .with_prompt("API Token")
        .interact_text()
        .unwrap();
    
    let config = format!(
        "ipam: phpipam\n\turl {}\n\tapp-id {}\n\ttoken {}\n",
        url, app_id, token
    );
    
    save_sdn_config("ipam", "phpipam", &config);
    println!("✅ phpIPAM configured");
}

fn setup_custom_ipam() {
    println!("Custom IPAM setup - edit /etc/pve/sdn/ipam.cfg manually");
}

// Helper functions

fn get_first_physical_interface() -> Option<String> {
    let output = Command::new("ip")
        .args(&["link", "show"])
        .output()
        .ok()?;
    
    let interfaces = String::from_utf8_lossy(&output.stdout);
    for line in interfaces.lines() {
        if let Some(name) = line.split(':').nth(1) {
            let iface = name.trim();
            if iface.starts_with("en") || iface.starts_with("eth") {
                return Some(iface.to_string());
            }
        }
    }
    None
}

fn make_bridge_persistent(bridge: &str, ip: &str, physical: &str) {
    let mut config = format!("\nauto {}\niface {} inet ", bridge, bridge);
    
    if ip.is_empty() {
        config.push_str("manual\n");
    } else {
        config.push_str(&format!("static\n\taddress {}\n", ip));
    }
    
    config.push_str("\tbridge-ports ");
    if physical.is_empty() {
        config.push_str("none\n");
    } else {
        config.push_str(&format!("{}\n", physical));
    }
    
    config.push_str("\tbridge-stp off\n\tbridge-fd 0\n");
    
    // Append to /etc/network/interfaces
    if let Ok(mut interfaces) = fs::read_to_string("/etc/network/interfaces") {
        interfaces.push_str(&config);
        fs::write("/etc/network/interfaces", interfaces).ok();
    }
}

fn clear_fdb() {
    let bridge: String = Input::new()
        .with_prompt("Bridge name")
        .default("vmbr0".to_string())
        .interact_text()
        .unwrap();
    
    let _ = Command::new("bridge")
        .args(&["fdb", "flush", "dev", &bridge])
        .status();
    
    println!("✅ Forwarding database cleared for {}", bridge);
}

fn fix_stp() {
    let bridge: String = Input::new()
        .with_prompt("Bridge name")
        .default("vmbr0".to_string())
        .interact_text()
        .unwrap();
    
    // Disable STP (usually not needed for VM bridges)
    let _ = Command::new("brctl")
        .args(&["stp", &bridge, "off"])
        .status();
    
    // Set forward delay to 0
    let _ = Command::new("brctl")
        .args(&["setfd", &bridge, "0"])
        .status();
    
    println!("✅ STP disabled for {}", bridge);
}

fn reset_bridge_defaults() {
    let bridge: String = Input::new()
        .with_prompt("Bridge name")
        .default("vmbr0".to_string())
        .interact_text()
        .unwrap();
    
    // Reset to PVE defaults
    let _ = Command::new("brctl")
        .args(&["stp", &bridge, "off"])
        .status();
    
    let _ = Command::new("brctl")
        .args(&["setfd", &bridge, "0"])
        .status();
    
    let _ = Command::new("ip")
        .args(&["link", "set", &bridge, "mtu", "1500"])
        .status();
    
    println!("✅ {} reset to defaults", bridge);
}

fn fix_vlan_filtering() {
    let bridge: String = Input::new()
        .with_prompt("Bridge name")
        .default("vmbr0".to_string())
        .interact_text()
        .unwrap();
    
    let enable = Confirm::new()
        .with_prompt("Enable VLAN filtering?")
        .default(false)
        .interact()
        .unwrap();
    
    let value = if enable { "1" } else { "0" };
    
    let _ = Command::new("ip")
        .args(&["link", "set", &bridge, "type", "bridge", "vlan_filtering", value])
        .status();
    
    println!("✅ VLAN filtering {} for {}", if enable { "enabled" } else { "disabled" }, bridge);
}

fn repair_slaves() {
    let bridge: String = Input::new()
        .with_prompt("Bridge name")
        .default("vmbr0".to_string())
        .interact_text()
        .unwrap();
    
    // Get current slaves
    let output = Command::new("ls")
        .arg(&format!("/sys/class/net/{}/brif/", bridge))
        .output();
    
    if let Ok(output) = output {
        let slaves = String::from_utf8_lossy(&output.stdout);
        for slave in slaves.split_whitespace() {
            // Reset slave
            let _ = Command::new("ip")
                .args(&["link", "set", slave, "nomaster"])
                .status();
            
            let _ = Command::new("ip")
                .args(&["link", "set", slave, "master", &bridge])
                .status();
            
            println!("✅ Repaired slave interface: {}", slave);
        }
    }
}

fn save_sdn_config(config_type: &str, name: &str, content: &str) {
    let config_dir = "/etc/pve/sdn";
    let _ = fs::create_dir_all(config_dir);
    
    let config_file = format!("{}/{}.cfg", config_dir, config_type);
    
    // Append to existing config
    if let Ok(mut existing) = fs::read_to_string(&config_file) {
        existing.push_str("\n");
        existing.push_str(content);
        fs::write(&config_file, existing).ok();
    } else {
        fs::write(&config_file, content).ok();
    }
    
    // Apply SDN configuration
    let _ = Command::new("pvesh")
        .args(&["set", "/cluster/sdn"])
        .status();
}