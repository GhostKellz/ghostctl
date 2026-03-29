pub mod advanced_firewall;
pub mod advanced_scanner;
pub mod bridge_management;
pub mod enterprise_networking;
pub mod errors;
pub mod export;
pub mod fingerprint;
pub mod firewall;
pub mod hw_offload;
pub mod libvirt_advanced;
pub mod nftables_enterprise;
pub mod safe_commands;
pub mod scanner;
pub mod services;
pub mod troubleshoot;
pub mod virtualization;

// Note: safe_commands provides secure command execution helpers
// Import specific functions as needed in firewall modules

use dialoguer::{Select, theme::ColorfulTheme};

pub fn networking_menu() {
    loop {
        let options = [
            "🔥 Basic Firewall Management",
            "🚀 Advanced Firewall & Networking",
            "🏢 Enterprise nftables Management",
            "🔍 Network Scanner & Discovery",
            "🌐 Enterprise Networking (VLAN/SDN)",
            "🖥️ libvirt/KVM Advanced Networking",
            "🌉 Linux Bridge Management",
            "🔧 Network Troubleshooting",
            "🖥️ Virtualization Networking",
            "⚡ Hardware Offload Settings",
            "📊 Network Status",
            "⬅️ Back",
        ];

        let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🌐 Network Management")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match choice {
            0 => firewall::firewall_menu(),
            1 => advanced_firewall::advanced_firewall_menu(),
            2 => nftables_enterprise::nftables_enterprise_menu(),
            3 => crate::network::scan::network_security_scanning(),
            4 => enterprise_networking::enterprise_networking_menu(),
            5 => libvirt_advanced::libvirt_advanced_menu(),
            6 => bridge_management::bridge_management_menu(),
            7 => troubleshoot::troubleshoot_menu(),
            8 => virtualization::virtualization_menu(),
            9 => hw_offload::offload_menu(),
            10 => network_status(),
            _ => break,
        }
    }
}

fn network_status() {
    println!("📊 Network Status");
    println!("================");

    // Check interfaces
    println!("\n🔌 Network Interfaces:");
    let status = std::process::Command::new("ip")
        .args(&["link", "show"])
        .status();

    match status {
        Ok(s) if s.success() => {}
        _ => println!("  ❌ Failed to get interface status"),
    }

    // Check connectivity
    println!("\n🌐 Internet Connectivity:");
    let ping = std::process::Command::new("ping")
        .args(&["-c", "1", "-W", "2", "8.8.8.8"])
        .output();

    match ping {
        Ok(out) if out.status.success() => println!("  ✅ Internet connection working"),
        _ => println!("  ❌ No internet connection"),
    }

    // Check DNS
    println!("\n🔍 DNS Resolution:");
    let dns = std::process::Command::new("nslookup")
        .args(&["google.com"])
        .output();

    match dns {
        Ok(out) if out.status.success() => println!("  ✅ DNS resolution working"),
        _ => println!("  ❌ DNS resolution failed"),
    }

    // Check firewall status
    println!("\n🔥 Firewall Status:");

    // Check UFW
    let ufw = std::process::Command::new("sudo")
        .args(&["ufw", "status"])
        .output();

    if let Ok(out) = ufw
        && out.status.success()
    {
        let status_str = String::from_utf8_lossy(&out.stdout);
        if status_str.contains("Status: active") {
            println!("  ✅ UFW is active");
        } else if status_str.contains("Status: inactive") {
            println!("  ⭕ UFW is inactive");
        }
    }

    // Check firewalld
    let firewalld = std::process::Command::new("systemctl")
        .args(&["is-active", "firewalld"])
        .output();

    if let Ok(out) = firewalld {
        let status_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if status_str == "active" {
            println!("  ✅ Firewalld is active");
        }
    }

    // Check iptables
    let iptables = std::process::Command::new("sudo")
        .args(&["iptables", "-L", "-n"])
        .output();

    if let Ok(out) = iptables
        && out.status.success()
    {
        let rules = String::from_utf8_lossy(&out.stdout);
        let rule_count = rules.lines().count();
        if rule_count > 10 {
            println!("  ✅ iptables has {} rules configured", rule_count);
        }
    }

    // Check NetworkManager
    println!("\n📡 NetworkManager Status:");
    let nm = std::process::Command::new("systemctl")
        .args(&["is-active", "NetworkManager"])
        .output();

    if let Ok(out) = nm {
        let status_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if status_str == "active" {
            println!("  ✅ NetworkManager is active");
        } else {
            println!("  ❌ NetworkManager is not active");
        }
    }
}
