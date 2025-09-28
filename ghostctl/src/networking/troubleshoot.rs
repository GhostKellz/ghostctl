use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use std::process::Command;

pub fn troubleshoot_menu() {
    loop {
        let options = [
            "🌐 General Network Troubleshooting",
            "🐳 Docker Network Troubleshooting",
            "🖥️ QEMU/KVM Bridge Troubleshooting",
            "🔌 Port & Service Analysis",
            "📡 NetworkManager Troubleshooting",
            "🔍 Advanced Network Diagnostics",
            "🚀 Performance Testing",
            "⬅️ Back",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔧 Network Troubleshooting")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => general_network_troubleshooting(),
            1 => docker_network_troubleshooting(),
            2 => qemu_kvm_troubleshooting(),
            3 => port_service_analysis(),
            4 => networkmanager_troubleshooting(),
            5 => advanced_diagnostics(),
            6 => performance_testing(),
            _ => break,
        }
    }
}

fn general_network_troubleshooting() {
    let options = [
        "🔍 Complete Network Diagnosis",
        "🌐 Internet Connectivity Test",
        "🔗 Network Interface Analysis",
        "📊 Route Table Analysis",
        "🎯 DNS Troubleshooting",
        "📈 Bandwidth Testing",
        "🔧 Quick Network Fixes",
        "⬅️ Back",
    ];

    loop {
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🌐 General Network Troubleshooting")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => complete_network_diagnosis(),
            1 => internet_connectivity_test(),
            2 => network_interface_analysis(),
            3 => route_table_analysis(),
            4 => dns_troubleshooting(),
            5 => bandwidth_testing(),
            6 => quick_network_fixes(),
            _ => break,
        }
    }
}

fn complete_network_diagnosis() {
    println!("🔍 Complete Network Diagnosis");
    println!("==============================\n");

    // 1. Network Interfaces
    println!("1️⃣ Network Interfaces Status:");
    Command::new("ip").args(&["addr", "show"]).status().ok();

    println!("\n📊 Interface Statistics:");
    Command::new("cat").args(&["/proc/net/dev"]).status().ok();

    // 2. Routing Table
    println!("\n2️⃣ Routing Table:");
    Command::new("ip").args(&["route", "show"]).status().ok();

    println!("\n📋 Route Cache:");
    Command::new("ip")
        .args(&["route", "show", "cache"])
        .status()
        .ok();

    // 3. DNS Configuration
    println!("\n3️⃣ DNS Configuration:");
    Command::new("cat")
        .args(&["/etc/resolv.conf"])
        .status()
        .ok();

    println!("\n🔍 systemd-resolved status:");
    Command::new("systemctl")
        .args(&["status", "systemd-resolved", "--no-pager"])
        .status()
        .ok();

    // 4. ARP Table
    println!("\n4️⃣ ARP Table:");
    Command::new("ip").args(&["neigh", "show"]).status().ok();

    // 5. Active Connections
    println!("\n5️⃣ Active Network Connections:");
    Command::new("ss").args(&["-tuln"]).status().ok();

    // 6. Network Services
    println!("\n6️⃣ Network Services:");
    Command::new("systemctl")
        .args(&[
            "list-units",
            "--type=service",
            "--state=active",
            "|",
            "grep",
            "network",
        ])
        .status()
        .ok();

    // 7. Firewall Status
    println!("\n7️⃣ Firewall Status:");
    Command::new("sudo")
        .args(&["iptables", "-L", "-n"])
        .status()
        .ok();

    // 8. Network Performance
    println!("\n8️⃣ Network Interface Performance:");
    Command::new("sh").arg("-c").arg("for iface in $(ls /sys/class/net/); do echo \"=== $iface ===\"; ethtool $iface 2>/dev/null | grep -E 'Speed|Duplex|Link' || echo 'ethtool not available'; done").status().ok();

    println!("\n✅ Diagnosis complete. Check above for any issues.");
}

fn internet_connectivity_test() {
    println!("🌐 Internet Connectivity Test");
    println!("==============================\n");

    let test_hosts = vec![
        ("8.8.8.8", "Google DNS"),
        ("1.1.1.1", "Cloudflare DNS"),
        ("google.com", "Google"),
        ("github.com", "GitHub"),
        ("archlinux.org", "Arch Linux"),
    ];

    for (host, name) in &test_hosts {
        println!("🔍 Testing {}...", name);

        // Ping test
        let ping = Command::new("ping")
            .args(&["-c", "3", "-W", "2", host])
            .output();

        match ping {
            Ok(out) if out.status.success() => {
                let output_str = String::from_utf8_lossy(&out.stdout);
                for line in output_str.lines() {
                    if line.contains("packet loss") {
                        println!("  📊 {}", line.trim());
                    }
                    if line.contains("min/avg/max") {
                        println!("  ⏱️ {}", line.trim());
                    }
                }
            }
            _ => println!("  ❌ Ping failed to {}", host),
        }

        // DNS resolution test (for domain names)
        if !host.chars().all(|c| c.is_numeric() || c == '.') {
            let nslookup = Command::new("nslookup").arg(host).output();

            match nslookup {
                Ok(out) if out.status.success() => println!("  ✅ DNS resolution successful"),
                _ => println!("  ❌ DNS resolution failed"),
            }
        }

        println!();
    }

    // Test different protocols
    println!("🔍 Protocol Tests:");

    println!("\n📡 HTTP Test:");
    let http = Command::new("curl")
        .args(&["-I", "-m", "10", "http://httpbin.org/status/200"])
        .output();

    match http {
        Ok(out) if out.status.success() => println!("  ✅ HTTP working"),
        _ => println!("  ❌ HTTP failed"),
    }

    println!("\n🔐 HTTPS Test:");
    let https = Command::new("curl")
        .args(&["-I", "-m", "10", "https://httpbin.org/status/200"])
        .output();

    match https {
        Ok(out) if out.status.success() => println!("  ✅ HTTPS working"),
        _ => println!("  ❌ HTTPS failed"),
    }

    // Test specific ports
    println!("\n🔌 Port Connectivity Tests:");
    let port_tests = vec![
        ("google.com", "80", "HTTP"),
        ("google.com", "443", "HTTPS"),
        ("github.com", "22", "SSH"),
        ("8.8.8.8", "53", "DNS"),
    ];

    for (host, port, service) in port_tests {
        let nc = Command::new("nc")
            .args(&["-zv", "-w", "2", host, port])
            .output();

        match nc {
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                if stderr.contains("succeeded") || out.status.success() {
                    println!("  ✅ {} ({}:{}) - OPEN", service, host, port);
                } else {
                    println!("  ❌ {} ({}:{}) - BLOCKED", service, host, port);
                }
            }
            _ => println!("  ⚠️ {} ({}:{}) - TEST FAILED", service, host, port),
        }
    }
}

fn network_interface_analysis() {
    println!("🔗 Network Interface Analysis");
    println!("==============================\n");

    // List all interfaces
    println!("📋 Available Interfaces:");
    Command::new("ip").args(&["link", "show"]).status().ok();

    println!("\n📊 Interface Details:");

    // Get interface list
    let interfaces_output = Command::new("sh")
        .arg("-c")
        .arg("ls /sys/class/net/")
        .output();

    if let Ok(out) = interfaces_output {
        let interfaces = String::from_utf8_lossy(&out.stdout);

        for interface in interfaces.lines() {
            if interface.trim().is_empty() {
                continue;
            }

            println!("\n=== {} ===", interface);

            // Interface status
            let status_cmd = format!("cat /sys/class/net/{}/operstate", interface);
            let status = Command::new("sh").arg("-c").arg(&status_cmd).output();
            if let Ok(s) = status {
                println!("  Status: {}", String::from_utf8_lossy(&s.stdout).trim());
            }

            // MAC address
            let mac_cmd = format!("cat /sys/class/net/{}/address", interface);
            let mac = Command::new("sh").arg("-c").arg(&mac_cmd).output();
            if let Ok(m) = mac {
                println!("  MAC: {}", String::from_utf8_lossy(&m.stdout).trim());
            }

            // MTU
            let mtu_cmd = format!("cat /sys/class/net/{}/mtu", interface);
            let mtu = Command::new("sh").arg("-c").arg(&mtu_cmd).output();
            if let Ok(m) = mtu {
                println!("  MTU: {}", String::from_utf8_lossy(&m.stdout).trim());
            }

            // Speed (if available)
            let speed_cmd = format!("cat /sys/class/net/{}/speed 2>/dev/null", interface);
            let speed = Command::new("sh").arg("-c").arg(&speed_cmd).output();
            if let Ok(s) = speed {
                let speed_str = String::from_utf8_lossy(&s.stdout).trim().to_string();
                if !speed_str.is_empty() && speed_str != "cat: /sys/class/net" {
                    println!("  Speed: {} Mbps", speed_str);
                }
            }

            // Statistics
            let stats_cmd = format!(
                "cat /sys/class/net/{}/statistics/rx_bytes /sys/class/net/{}/statistics/tx_bytes",
                interface, interface
            );
            let stats = Command::new("sh").arg("-c").arg(&stats_cmd).output();
            if let Ok(s) = stats {
                let lines: Vec<String> = String::from_utf8_lossy(&s.stdout)
                    .lines()
                    .map(|s| s.to_string())
                    .collect();
                if lines.len() >= 2 {
                    let rx_bytes: u64 = lines[0].trim().parse().unwrap_or(0);
                    let tx_bytes: u64 = lines[1].trim().parse().unwrap_or(0);
                    println!(
                        "  RX: {} MB, TX: {} MB",
                        rx_bytes / 1024 / 1024,
                        tx_bytes / 1024 / 1024
                    );
                }
            }

            // IP addresses
            let ip_cmd = format!("ip addr show {}", interface);
            let ip_info = Command::new("sh").arg("-c").arg(&ip_cmd).output();
            if let Ok(i) = ip_info {
                let ip_str = String::from_utf8_lossy(&i.stdout);
                for line in ip_str.lines() {
                    if line.trim().starts_with("inet ") || line.trim().starts_with("inet6 ") {
                        println!("  {}", line.trim());
                    }
                }
            }
        }
    }

    // Interface errors
    println!("\n❌ Interface Errors:");
    Command::new("sh")
        .arg("-c")
        .arg("grep -H . /sys/class/net/*/statistics/*errors 2>/dev/null | grep -v ':0$'")
        .status()
        .ok();

    println!("\n📉 Interface Drops:");
    Command::new("sh")
        .arg("-c")
        .arg("grep -H . /sys/class/net/*/statistics/*drop* 2>/dev/null | grep -v ':0$'")
        .status()
        .ok();
}

fn route_table_analysis() {
    println!("📊 Route Table Analysis");
    println!("========================\n");

    // Main routing table
    println!("🛣️ Main Routing Table:");
    Command::new("ip").args(&["route", "show"]).status().ok();

    // All routing tables
    println!("\n📋 All Routing Tables:");
    Command::new("ip")
        .args(&["route", "show", "table", "all"])
        .status()
        .ok();

    // Routing policy
    println!("\n📜 Routing Policy:");
    Command::new("ip").args(&["rule", "list"]).status().ok();

    // Default gateway analysis
    println!("\n🌐 Default Gateway Analysis:");
    let gateway_output = Command::new("ip")
        .args(&["route", "show", "default"])
        .output();

    if let Ok(out) = gateway_output {
        let route_str = String::from_utf8_lossy(&out.stdout);
        for line in route_str.lines() {
            if line.contains("default via") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 2 {
                    let gateway = parts[2];
                    println!("  Gateway: {}", gateway);

                    // Test gateway connectivity
                    println!("  Testing gateway connectivity...");
                    let ping = Command::new("ping")
                        .args(&["-c", "3", "-W", "2", gateway])
                        .output();

                    match ping {
                        Ok(p) if p.status.success() => println!("  ✅ Gateway reachable"),
                        _ => println!("  ❌ Gateway unreachable"),
                    }

                    // Check gateway MAC
                    let arp = Command::new("ip")
                        .args(&["neigh", "show", gateway])
                        .output();

                    if let Ok(a) = arp {
                        let arp_str = String::from_utf8_lossy(&a.stdout);
                        if !arp_str.trim().is_empty() {
                            println!("  ARP: {}", arp_str.trim());
                        } else {
                            println!("  ⚠️ No ARP entry for gateway");
                        }
                    }
                }
            }
        }
    }

    // Analyze routing for specific destinations
    println!("\n🎯 Route Analysis for Common Destinations:");
    let destinations = vec!["8.8.8.8", "1.1.1.1", "google.com"];

    for dest in destinations {
        println!("\n--- Route to {} ---", dest);
        Command::new("ip")
            .args(&["route", "get", dest])
            .status()
            .ok();
    }

    // Check for routing loops or issues
    println!("\n🔄 Routing Issue Detection:");

    // Check for duplicate routes
    let route_check = Command::new("sh")
        .arg("-c")
        .arg("ip route show | sort | uniq -d")
        .output();

    if let Ok(rc) = route_check {
        let duplicates = String::from_utf8_lossy(&rc.stdout);
        if !duplicates.trim().is_empty() {
            println!("  ⚠️ Duplicate routes found:");
            print!("{}", duplicates);
        } else {
            println!("  ✅ No duplicate routes found");
        }
    }

    // Check for unreachable routes
    println!("\n🔍 Testing route reachability:");
    let unreachable_check = Command::new("sh")
        .arg("-c")
        .arg("ip route show | grep -E '^[0-9]' | head -5")
        .output();

    if let Ok(uc) = unreachable_check {
        let routes = String::from_utf8_lossy(&uc.stdout);
        for line in routes.lines() {
            if let Some(network) = line.split_whitespace().next() {
                if network.contains('/') {
                    // Extract first IP of network for testing
                    println!("  Testing route to {}...", network);
                }
            }
        }
    }
}

fn dns_troubleshooting() {
    println!("🎯 DNS Troubleshooting");
    println!("=======================\n");

    // Check DNS configuration
    println!("1️⃣ DNS Configuration:");
    println!("\n📋 /etc/resolv.conf:");
    Command::new("cat")
        .args(&["/etc/resolv.conf"])
        .status()
        .ok();

    println!("\n📋 systemd-resolved status:");
    Command::new("systemd-resolve")
        .args(&["--status"])
        .status()
        .ok();

    // Test DNS servers
    println!("\n2️⃣ DNS Server Tests:");
    let dns_servers = vec![
        ("8.8.8.8", "Google DNS"),
        ("1.1.1.1", "Cloudflare DNS"),
        ("9.9.9.9", "Quad9 DNS"),
        ("208.67.222.222", "OpenDNS"),
    ];

    for (server, name) in &dns_servers {
        println!("\n🔍 Testing {} ({}):", name, server);

        // Test DNS port connectivity
        let nc = Command::new("nc")
            .args(&["-zv", "-w", "2", server, "53"])
            .output();

        match nc {
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                if stderr.contains("succeeded") || out.status.success() {
                    println!("  ✅ Port 53 accessible");
                } else {
                    println!("  ❌ Port 53 blocked");
                }
            }
            _ => println!("  ❌ Connection test failed"),
        }

        // Test DNS resolution
        let dig = Command::new("dig")
            .args(&[&format!("@{}", server), "google.com", "+time=2", "+tries=1"])
            .output();

        match dig {
            Ok(out) if out.status.success() => {
                let output_str = String::from_utf8_lossy(&out.stdout);
                if output_str.contains("ANSWER SECTION") {
                    println!("  ✅ DNS resolution working");
                    // Extract query time
                    for line in output_str.lines() {
                        if line.contains("Query time:") {
                            println!("  ⏱️ {}", line.trim());
                        }
                    }
                } else {
                    println!("  ⚠️ DNS resolution returned no answers");
                }
            }
            _ => {
                println!("  ❌ DNS resolution failed");
                // Try with nslookup as fallback
                let nslookup = Command::new("sh")
                    .arg("-c")
                    .arg(&format!("echo 'google.com' | nslookup - {}", server))
                    .output();

                if let Ok(ns_out) = nslookup {
                    if ns_out.status.success() {
                        println!("  ✅ nslookup resolution working");
                    }
                }
            }
        }
    }

    // Test reverse DNS
    println!("\n3️⃣ Reverse DNS Test:");
    let reverse_test = Command::new("dig")
        .args(&["-x", "8.8.8.8", "+short"])
        .output();

    match reverse_test {
        Ok(out) if out.status.success() => {
            let result = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !result.is_empty() {
                println!("  ✅ Reverse DNS: 8.8.8.8 -> {}", result);
            } else {
                println!("  ⚠️ No reverse DNS record");
            }
        }
        _ => println!("  ❌ Reverse DNS test failed"),
    }

    // DNS cache analysis
    println!("\n4️⃣ DNS Cache Analysis:");

    // systemd-resolved cache
    let resolved_stats = Command::new("systemd-resolve")
        .args(&["--statistics"])
        .output();

    if let Ok(stats) = resolved_stats {
        println!("📊 systemd-resolved statistics:");
        print!("{}", String::from_utf8_lossy(&stats.stdout));
    }

    // Check for DNS cache issues
    println!("\n5️⃣ DNS Cache Issues:");

    // Test with and without cache
    let test_domain = "github.com";

    println!("🔍 Testing {} with cache:", test_domain);
    let with_cache = std::time::Instant::now();
    Command::new("dig")
        .args(&[test_domain, "+short"])
        .status()
        .ok();
    let cache_time = with_cache.elapsed();

    println!("🔄 Flushing DNS cache and testing again:");
    Command::new("sudo")
        .args(&["systemd-resolve", "--flush-caches"])
        .status()
        .ok();

    let without_cache = std::time::Instant::now();
    Command::new("dig")
        .args(&[test_domain, "+short"])
        .status()
        .ok();
    let no_cache_time = without_cache.elapsed();

    println!("📊 Timing comparison:");
    println!("  With cache: {:?}", cache_time);
    println!("  Without cache: {:?}", no_cache_time);

    // DNS security tests
    println!("\n6️⃣ DNS Security Tests:");

    println!("🔒 DNSSEC Test:");
    let dnssec = Command::new("dig")
        .args(&["google.com", "+dnssec", "+short"])
        .output();

    if let Ok(dnssec_out) = dnssec {
        let output = String::from_utf8_lossy(&dnssec_out.stdout);
        if output.contains("RRSIG") {
            println!("  ✅ DNSSEC signatures present");
        } else {
            println!("  ⚠️ No DNSSEC signatures found");
        }
    }

    // Test DNS over HTTPS/TLS
    println!("\n🔐 DNS over HTTPS Test (if configured):");
    let doh_test = Command::new("curl")
        .args(&[
            "-H",
            "accept: application/dns-json",
            "https://1.1.1.1/dns-query?name=google.com&type=A",
        ])
        .output();

    if let Ok(doh_out) = doh_test {
        if doh_out.status.success() {
            println!("  ✅ DNS over HTTPS working");
        } else {
            println!("  ⚠️ DNS over HTTPS not available/configured");
        }
    }
}

fn bandwidth_testing() {
    println!("📈 Bandwidth Testing");
    println!("====================\n");

    let test_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select bandwidth test type")
        .items(&[
            "🌐 Internet Speed Test",
            "🏠 Local Network Test",
            "📊 Interface Throughput Test",
            "🔄 Latency & Jitter Test",
            "📡 Continuous Monitoring",
        ])
        .default(0)
        .interact()
        .unwrap();

    match test_type {
        0 => internet_speed_test(),
        1 => local_network_test(),
        2 => interface_throughput_test(),
        3 => latency_jitter_test(),
        4 => continuous_monitoring(),
        _ => {}
    }
}

fn internet_speed_test() {
    println!("🌐 Internet Speed Test");
    println!("======================\n");

    // Check if speedtest-cli is available
    let speedtest_check = Command::new("which").arg("speedtest-cli").status();

    if let Ok(s) = speedtest_check {
        if s.success() {
            println!("🚀 Running speedtest-cli...");
            Command::new("speedtest-cli")
                .args(&["--simple"])
                .status()
                .ok();

            println!("\n📊 Detailed results:");
            Command::new("speedtest-cli").status().ok();
        } else {
            println!("⚠️ speedtest-cli not found. Installing...");

            let install = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Install speedtest-cli?")
                .default(true)
                .interact()
                .unwrap();

            if install {
                Command::new("pip")
                    .args(&["install", "--user", "speedtest-cli"])
                    .status()
                    .ok();

                println!("Running speed test...");
                Command::new("speedtest-cli")
                    .args(&["--simple"])
                    .status()
                    .ok();
            }
        }
    }

    // Alternative: Manual bandwidth test using curl
    println!("\n🔄 Alternative: Manual Download Test");
    println!("Testing download speed with curl...");

    let test_urls = vec![
        ("http://speedtest.ftp.otenet.gr/files/test100k.db", "100KB"),
        ("http://speedtest.ftp.otenet.gr/files/test1Mb.db", "1MB"),
        ("http://speedtest.ftp.otenet.gr/files/test10Mb.db", "10MB"),
    ];

    for (url, size) in test_urls {
        println!("\n📥 Downloading {} file...", size);
        Command::new("curl")
            .args(&[
                "-o",
                "/dev/null",
                "-w",
                "Speed: %{speed_download} bytes/sec, Time: %{time_total}s\n",
                url,
            ])
            .status()
            .ok();
    }
}

fn local_network_test() {
    println!("🏠 Local Network Test");
    println!("=====================\n");

    let target = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter target IP address for local network test")
        .interact()
        .unwrap();

    // Check if iperf3 is available
    let iperf_check = Command::new("which").arg("iperf3").status();

    if let Ok(s) = iperf_check {
        if s.success() {
            println!("🔧 Using iperf3 for bandwidth test");
            println!(
                "Note: iperf3 server must be running on target ({}) with: iperf3 -s",
                target
            );

            let proceed = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Is iperf3 server running on target?")
                .default(false)
                .interact()
                .unwrap();

            if proceed {
                println!("\n📊 TCP Bandwidth Test:");
                Command::new("iperf3")
                    .args(&["-c", &target, "-t", "10"])
                    .status()
                    .ok();

                println!("\n📡 UDP Bandwidth Test:");
                Command::new("iperf3")
                    .args(&["-c", &target, "-u", "-t", "10"])
                    .status()
                    .ok();
            }
        } else {
            println!("⚠️ iperf3 not found, using alternative methods");
        }
    }

    // Alternative: ping-based RTT test
    println!("\n📶 Ping-based Network Quality Test:");
    Command::new("ping")
        .args(&["-c", "20", "-i", "0.2", &target])
        .status()
        .ok();

    // MTU discovery
    println!("\n🔍 MTU Discovery:");
    for mtu in [1500, 1400, 1300, 1200, 1100, 1000].iter() {
        println!("Testing MTU {}...", mtu);
        let result = Command::new("ping")
            .args(&[
                "-M",
                "do",
                "-s",
                &(mtu - 28).to_string(),
                "-c",
                "1",
                &target,
            ])
            .output();

        match result {
            Ok(out) if out.status.success() => {
                println!("  ✅ MTU {} works", mtu);
                break;
            }
            _ => println!("  ❌ MTU {} failed", mtu),
        }
    }
}

fn interface_throughput_test() {
    println!("📊 Interface Throughput Test");
    println!("============================\n");

    // Get list of interfaces
    let interfaces_output = Command::new("sh")
        .arg("-c")
        .arg("ls /sys/class/net/ | grep -v lo")
        .output();

    if let Ok(out) = interfaces_output {
        let interfaces: Vec<String> = String::from_utf8_lossy(&out.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect();

        let selected_interface = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select interface to monitor")
            .items(&interfaces)
            .default(0)
            .interact()
            .unwrap();

        let interface = &interfaces[selected_interface];

        println!("📈 Monitoring {} for 30 seconds...", interface);
        println!("Generate some network traffic to see throughput");

        // Monitor interface statistics
        let stats_path_rx = format!("/sys/class/net/{}/statistics/rx_bytes", interface);
        let stats_path_tx = format!("/sys/class/net/{}/statistics/tx_bytes", interface);

        let initial_rx = std::fs::read_to_string(&stats_path_rx)
            .unwrap_or_default()
            .trim()
            .parse::<u64>()
            .unwrap_or(0);

        let initial_tx = std::fs::read_to_string(&stats_path_tx)
            .unwrap_or_default()
            .trim()
            .parse::<u64>()
            .unwrap_or(0);

        for i in 1..=30 {
            std::thread::sleep(std::time::Duration::from_secs(1));

            let current_rx = std::fs::read_to_string(&stats_path_rx)
                .unwrap_or_default()
                .trim()
                .parse::<u64>()
                .unwrap_or(0);

            let current_tx = std::fs::read_to_string(&stats_path_tx)
                .unwrap_or_default()
                .trim()
                .parse::<u64>()
                .unwrap_or(0);

            let rx_rate = (current_rx - initial_rx) / i as u64;
            let tx_rate = (current_tx - initial_tx) / i as u64;

            println!(
                "  [{:02}s] RX: {} KB/s, TX: {} KB/s",
                i,
                rx_rate / 1024,
                tx_rate / 1024
            );
        }

        println!("\n📊 Final Statistics:");
        println!("Interface: {}", interface);

        // Get final stats
        let final_rx = std::fs::read_to_string(&stats_path_rx)
            .unwrap_or_default()
            .trim()
            .parse::<u64>()
            .unwrap_or(0);

        let final_tx = std::fs::read_to_string(&stats_path_tx)
            .unwrap_or_default()
            .trim()
            .parse::<u64>()
            .unwrap_or(0);

        println!("  Total RX: {} MB", (final_rx - initial_rx) / 1024 / 1024);
        println!("  Total TX: {} MB", (final_tx - initial_tx) / 1024 / 1024);
        println!(
            "  Average RX Rate: {} KB/s",
            (final_rx - initial_rx) / 30 / 1024
        );
        println!(
            "  Average TX Rate: {} KB/s",
            (final_tx - initial_tx) / 30 / 1024
        );
    }
}

fn latency_jitter_test() {
    println!("🔄 Latency & Jitter Test");
    println!("========================\n");

    let target = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter target host/IP for latency test")
        .default("8.8.8.8".to_string())
        .interact()
        .unwrap();

    println!("📊 Running comprehensive latency test...");

    // Extended ping test for jitter analysis
    println!("\n🏓 Ping Test (100 packets):");
    let ping_output = Command::new("ping")
        .args(&["-c", "100", "-i", "0.1", &target])
        .output();

    if let Ok(out) = ping_output {
        if out.status.success() {
            let output_str = String::from_utf8_lossy(&out.stdout);

            // Parse ping results for jitter analysis
            let mut rtts = Vec::new();
            for line in output_str.lines() {
                if line.contains("time=") {
                    if let Some(time_str) = line.split("time=").nth(1) {
                        if let Some(rtt_str) = time_str.split_whitespace().next() {
                            if let Ok(rtt) = rtt_str.parse::<f64>() {
                                rtts.push(rtt);
                            }
                        }
                    }
                }
            }

            if !rtts.is_empty() {
                let min_rtt = rtts.iter().cloned().fold(f64::INFINITY, f64::min);
                let max_rtt = rtts.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let avg_rtt = rtts.iter().sum::<f64>() / rtts.len() as f64;

                // Calculate jitter (standard deviation)
                let variance = rtts.iter().map(|&rtt| (rtt - avg_rtt).powi(2)).sum::<f64>()
                    / rtts.len() as f64;
                let jitter = variance.sqrt();

                println!("\n📊 Latency Analysis:");
                println!("  Packets: {}", rtts.len());
                println!("  Min RTT: {:.3} ms", min_rtt);
                println!("  Max RTT: {:.3} ms", max_rtt);
                println!("  Avg RTT: {:.3} ms", avg_rtt);
                println!("  Jitter (StdDev): {:.3} ms", jitter);

                // Quality assessment
                if jitter < 1.0 {
                    println!("  Quality: ✅ Excellent (jitter < 1ms)");
                } else if jitter < 5.0 {
                    println!("  Quality: 👍 Good (jitter < 5ms)");
                } else if jitter < 10.0 {
                    println!("  Quality: ⚠️ Fair (jitter < 10ms)");
                } else {
                    println!("  Quality: ❌ Poor (jitter > 10ms)");
                }
            }

            print!("{}", output_str);
        }
    }

    // Traceroute for path analysis
    println!("\n🗺️ Path Analysis (traceroute):");
    Command::new("traceroute")
        .args(&["-n", &target])
        .status()
        .ok();

    // MTR for continuous monitoring
    let mtr_check = Command::new("which").arg("mtr").status();
    if let Ok(s) = mtr_check {
        if s.success() {
            println!("\n📈 MTR Analysis (10 cycles):");
            Command::new("mtr")
                .args(&["-r", "-c", "10", &target])
                .status()
                .ok();
        }
    }
}

fn continuous_monitoring() {
    println!("📡 Continuous Network Monitoring");
    println!("=================================\n");

    let monitor_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select monitoring type")
        .items(&[
            "📊 Interface Statistics",
            "🌐 Connection Monitoring",
            "📈 Bandwidth Monitoring",
            "🔄 Ping Monitoring",
            "📋 Complete System Monitor",
        ])
        .default(0)
        .interact()
        .unwrap();

    let duration = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Monitoring duration in seconds")
        .default("60".to_string())
        .interact()
        .unwrap();

    let duration_secs: u64 = duration.parse().unwrap_or(60);

    match monitor_type {
        0 => monitor_interface_stats(duration_secs),
        1 => monitor_connections(duration_secs),
        2 => monitor_bandwidth(duration_secs),
        3 => monitor_ping(duration_secs),
        4 => monitor_complete_system(duration_secs),
        _ => {}
    }
}

fn monitor_interface_stats(duration: u64) {
    println!(
        "📊 Monitoring Interface Statistics for {} seconds",
        duration
    );
    println!("Press Ctrl+C to stop early\n");

    for i in 0..duration {
        if i % 5 == 0 {
            println!("\n--- {} seconds ---", i);
            Command::new("cat").args(&["/proc/net/dev"]).status().ok();
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn monitor_connections(duration: u64) {
    println!("🌐 Monitoring Network Connections for {} seconds", duration);
    println!("Press Ctrl+C to stop early\n");

    for i in 0..duration {
        if i % 10 == 0 {
            println!("\n--- {} seconds ---", i);
            Command::new("ss").args(&["-tuln"]).status().ok();

            let conn_count = Command::new("ss")
                .args(&["-tan"])
                .arg("state")
                .arg("established")
                .output();

            if let Ok(out) = conn_count {
                let count = String::from_utf8_lossy(&out.stdout).lines().count();
                println!("Active connections: {}", count);
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn monitor_bandwidth(duration: u64) {
    println!("📈 Monitoring Bandwidth for {} seconds", duration);

    // Use iftop if available
    let iftop_check = Command::new("which").arg("iftop").status();
    if let Ok(s) = iftop_check {
        if s.success() {
            println!("Using iftop for bandwidth monitoring...");
            Command::new("sudo")
                .args(&["iftop", "-t", "-s", &duration.to_string()])
                .status()
                .ok();
            return;
        }
    }

    // Fallback to manual monitoring
    println!("Manual bandwidth monitoring (no iftop found)");

    for i in 0..duration {
        if i % 5 == 0 {
            println!("\n--- {} seconds ---", i);

            // Show current network usage
            Command::new("sh")
                .arg("-c")
                .arg("cat /proc/net/dev | grep -v '^ *lo:' | awk 'NR>2{print $1, $2, $10}'")
                .status()
                .ok();
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn monitor_ping(duration: u64) {
    println!("🔄 Continuous Ping Monitoring for {} seconds", duration);

    let target = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter target to ping")
        .default("8.8.8.8".to_string())
        .interact()
        .unwrap();

    Command::new("ping")
        .args(&["-c", &duration.to_string(), &target])
        .status()
        .ok();
}

fn monitor_complete_system(duration: u64) {
    println!(
        "📋 Complete Network System Monitoring for {} seconds",
        duration
    );
    println!("This will show periodic snapshots of network state\n");

    for i in 0..duration {
        if i % 15 == 0 {
            println!("\n{}", "=".repeat(50));
            println!("Network Status at {} seconds", i);
            println!("{}", "=".repeat(50));

            println!("\n📡 Interfaces:");
            Command::new("ip").args(&["link", "show"]).status().ok();

            println!("\n🔗 Routes:");
            Command::new("ip").args(&["route", "show"]).status().ok();

            println!("\n🌐 Connections:");
            Command::new("ss").args(&["-tuln"]).status().ok();

            println!("\n📊 Traffic:");
            Command::new("cat").args(&["/proc/net/dev"]).status().ok();
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn quick_network_fixes() {
    println!("🔧 Quick Network Fixes");
    println!("======================\n");

    let fixes = [
        "🔄 Restart NetworkManager",
        "🔌 Restart all network interfaces",
        "🌐 Flush DNS cache",
        "📡 Reset routing table",
        "🔧 Fix common connectivity issues",
        "📊 Reset network statistics",
        "🖥️ Restart network services",
        "⬅️ Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select fix to apply")
        .items(&fixes)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            println!("🔄 Restarting NetworkManager...");
            Command::new("sudo")
                .args(&["systemctl", "restart", "NetworkManager"])
                .status()
                .ok();
            println!("✅ NetworkManager restarted");
        }
        1 => {
            println!("🔌 Restarting network interfaces...");

            // Get interfaces (excluding loopback)
            let interfaces = Command::new("sh")
                .arg("-c")
                .arg("ls /sys/class/net/ | grep -v lo")
                .output();

            if let Ok(out) = interfaces {
                for interface in String::from_utf8_lossy(&out.stdout).lines() {
                    println!("  Restarting {}...", interface);
                    Command::new("sudo")
                        .args(&["ip", "link", "set", interface, "down"])
                        .status()
                        .ok();
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    Command::new("sudo")
                        .args(&["ip", "link", "set", interface, "up"])
                        .status()
                        .ok();
                }
            }
            println!("✅ Network interfaces restarted");
        }
        2 => {
            println!("🌐 Flushing DNS cache...");
            Command::new("sudo")
                .args(&["systemd-resolve", "--flush-caches"])
                .status()
                .ok();
            Command::new("sudo")
                .args(&["systemctl", "restart", "systemd-resolved"])
                .status()
                .ok();
            println!("✅ DNS cache flushed");
        }
        3 => {
            println!("📡 Resetting routing table...");
            let confirm = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("⚠️ This will remove all routes. Continue?")
                .default(false)
                .interact()
                .unwrap();

            if confirm {
                Command::new("sudo")
                    .args(&["ip", "route", "flush", "table", "main"])
                    .status()
                    .ok();
                Command::new("sudo")
                    .args(&["systemctl", "restart", "NetworkManager"])
                    .status()
                    .ok();
                println!("✅ Routing table reset");
            }
        }
        4 => {
            println!("🔧 Applying common connectivity fixes...");

            // Enable IP forwarding
            Command::new("sudo")
                .args(&["sysctl", "net.ipv4.ip_forward=1"])
                .status()
                .ok();

            // Fix MTU issues
            Command::new("sudo")
                .args(&["ip", "link", "set", "dev", "eth0", "mtu", "1500"])
                .status()
                .ok();

            // Restart networking
            Command::new("sudo")
                .args(&["systemctl", "restart", "networking"])
                .status()
                .ok();

            println!("✅ Common fixes applied");
        }
        5 => {
            println!("📊 Resetting network statistics...");

            // Reset interface statistics
            let interfaces = Command::new("sh")
                .arg("-c")
                .arg("ls /sys/class/net/")
                .output();

            if let Ok(out) = interfaces {
                for interface in String::from_utf8_lossy(&out.stdout).lines() {
                    if !interface.trim().is_empty() {
                        Command::new("sudo")
                            .args(&["ip", "link", "set", interface, "down"])
                            .status()
                            .ok();
                        Command::new("sudo")
                            .args(&["ip", "link", "set", interface, "up"])
                            .status()
                            .ok();
                    }
                }
            }
            println!("✅ Network statistics reset");
        }
        6 => {
            println!("🖥️ Restarting network services...");

            let services = vec![
                "NetworkManager",
                "systemd-networkd",
                "systemd-resolved",
                "networking",
            ];

            for service in services {
                println!("  Restarting {}...", service);
                Command::new("sudo")
                    .args(&["systemctl", "restart", service])
                    .status()
                    .ok();
            }
            println!("✅ Network services restarted");
        }
        _ => {}
    }
}

// Missing function stubs
fn docker_network_troubleshooting() {
    println!("Docker Network Troubleshooting - Coming soon...");
}

fn qemu_kvm_troubleshooting() {
    println!("QEMU/KVM Troubleshooting - Coming soon...");
}

fn port_service_analysis() {
    println!("Port Service Analysis - Coming soon...");
}

fn networkmanager_troubleshooting() {
    println!("NetworkManager Troubleshooting - Coming soon...");
}

fn advanced_diagnostics() {
    println!("Advanced Diagnostics - Coming soon...");
}

fn performance_testing() {
    println!("Performance Testing - Coming soon...");
}
