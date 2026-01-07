//! Hardware offload detection and recommendations
//!
//! Provides detection of NIC hardware capabilities and recommendations
//! for optimal offload settings. Does NOT auto-apply changes - provides
//! hints and commands for manual execution.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::process::Command;

/// Known NIC chipsets with their recommended offload settings
#[derive(Debug, Clone)]
pub struct NicProfile {
    /// Chipset family name
    pub family: &'static str,
    /// Driver name pattern
    pub driver_pattern: &'static str,
    /// Recommended offload settings
    pub recommended_offloads: HashMap<&'static str, bool>,
    /// Known issues with this chipset
    pub known_issues: Vec<&'static str>,
}

/// Detected NIC information
#[derive(Debug, Clone)]
pub struct NicInfo {
    pub interface: String,
    pub driver: String,
    pub bus_info: String,
    pub firmware_version: Option<String>,
    pub supported_offloads: HashMap<String, bool>,
    pub current_offloads: HashMap<String, bool>,
}

/// Offload recommendation
#[derive(Debug, Clone)]
pub struct OffloadRecommendation {
    pub interface: String,
    pub setting: String,
    pub current: bool,
    pub recommended: bool,
    pub reason: String,
    pub command: String,
}

/// Get known NIC profiles
fn get_nic_profiles() -> Vec<NicProfile> {
    vec![
        NicProfile {
            family: "Intel i225/i226 (2.5GbE)",
            driver_pattern: "igc",
            recommended_offloads: [
                ("tx-checksum-ipv4", true),
                ("tx-checksum-ipv6", true),
                ("rx-checksum", true),
                ("tso", true),
                ("gso", true),
                ("gro", true),
                // Disable these on i225/i226 - known to cause issues
                ("tx-tcp-segmentation", true),
                ("tx-tcp6-segmentation", true),
            ]
            .into_iter()
            .collect(),
            known_issues: vec![
                "Early i225 firmware may have TSO bugs - update firmware if issues occur",
                "Some motherboard implementations have interrupt coalescing issues",
            ],
        },
        NicProfile {
            family: "Intel X550/X710 (10GbE)",
            driver_pattern: "ixgbe|i40e",
            recommended_offloads: [
                ("tx-checksum-ipv4", true),
                ("tx-checksum-ipv6", true),
                ("rx-checksum", true),
                ("tso", true),
                ("gso", true),
                ("gro", true),
                ("ntuple-filters", true),
                ("receive-hashing", true),
            ]
            .into_iter()
            .collect(),
            known_issues: vec![
                "Ensure firmware is up to date for full offload support",
                "Flow director may conflict with certain virtualization setups",
            ],
        },
        NicProfile {
            family: "Realtek RTL8125 (2.5GbE)",
            driver_pattern: "r8169|r8125",
            recommended_offloads: [
                ("tx-checksum-ipv4", true),
                ("tx-checksum-ipv6", true),
                ("rx-checksum", true),
                // Conservative on Realtek - some offloads can be buggy
                ("tso", false),
                ("gso", true),
                ("gro", true),
            ]
            .into_iter()
            .collect(),
            known_issues: vec![
                "TSO may cause packet corruption on some firmware versions",
                "Consider using r8125 driver from Realtek instead of in-kernel r8169",
                "Disable hardware timestamping if NTP issues occur",
            ],
        },
        NicProfile {
            family: "Broadcom NetXtreme",
            driver_pattern: "bnxt_en|tg3",
            recommended_offloads: [
                ("tx-checksum-ipv4", true),
                ("tx-checksum-ipv6", true),
                ("rx-checksum", true),
                ("tso", true),
                ("gso", true),
                ("gro", true),
            ]
            .into_iter()
            .collect(),
            known_issues: vec!["Some tg3 NICs have TSO issues with jumbo frames"],
        },
    ]
}

/// Detect NIC information using ethtool
pub fn detect_nic(interface: &str) -> Result<NicInfo> {
    // Get driver info
    let driver_output = Command::new("ethtool")
        .arg("-i")
        .arg(interface)
        .output()
        .context("Failed to run ethtool -i")?;

    if !driver_output.status.success() {
        anyhow::bail!("ethtool -i failed for {}", interface);
    }

    let driver_info = String::from_utf8_lossy(&driver_output.stdout);
    let mut driver = String::new();
    let mut bus_info = String::new();
    let mut firmware_version = None;

    for line in driver_info.lines() {
        if let Some(value) = line.strip_prefix("driver: ") {
            driver = value.trim().to_string();
        } else if let Some(value) = line.strip_prefix("bus-info: ") {
            bus_info = value.trim().to_string();
        } else if let Some(value) = line.strip_prefix("firmware-version: ") {
            firmware_version = Some(value.trim().to_string());
        }
    }

    // Get offload settings
    let offload_output = Command::new("ethtool")
        .arg("-k")
        .arg(interface)
        .output()
        .context("Failed to run ethtool -k")?;

    let offload_info = String::from_utf8_lossy(&offload_output.stdout);
    let mut current_offloads = HashMap::new();
    let mut supported_offloads = HashMap::new();

    for line in offload_info.lines() {
        let line = line.trim();
        if line.contains(':') {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let setting = parts[0].trim().to_string();
                let value_part = parts[1].trim();

                // Parse value: "on", "off", "on [fixed]", "off [not requested]", etc.
                let is_on = value_part.starts_with("on");
                let is_fixed = value_part.contains("[fixed]");

                current_offloads.insert(setting.clone(), is_on);
                supported_offloads.insert(setting, !is_fixed);
            }
        }
    }

    Ok(NicInfo {
        interface: interface.to_string(),
        driver,
        bus_info,
        firmware_version,
        supported_offloads,
        current_offloads,
    })
}

/// Get list of network interfaces
pub fn list_interfaces() -> Result<Vec<String>> {
    let output = Command::new("ls")
        .arg("/sys/class/net")
        .output()
        .context("Failed to list network interfaces")?;

    let interfaces: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|iface| *iface != "lo") // Skip loopback
        .map(|s| s.to_string())
        .collect();

    Ok(interfaces)
}

/// Match a NIC to a known profile
fn match_profile(nic: &NicInfo) -> Option<NicProfile> {
    let profiles = get_nic_profiles();

    for profile in profiles {
        if regex::Regex::new(profile.driver_pattern)
            .ok()?
            .is_match(&nic.driver)
        {
            return Some(profile);
        }
    }

    None
}

/// Generate recommendations for a NIC
pub fn get_recommendations(nic: &NicInfo) -> Vec<OffloadRecommendation> {
    let mut recommendations = Vec::new();

    if let Some(profile) = match_profile(nic) {
        for (setting, recommended) in &profile.recommended_offloads {
            if let Some(&current) = nic.current_offloads.get(*setting)
                && current != *recommended {
                    let state = if *recommended { "on" } else { "off" };
                    recommendations.push(OffloadRecommendation {
                        interface: nic.interface.clone(),
                        setting: setting.to_string(),
                        current,
                        recommended: *recommended,
                        reason: format!("Recommended {} for {} chipset", state, profile.family),
                        command: format!("sudo ethtool -K {} {} {}", nic.interface, setting, state),
                    });
                }
        }
    }

    recommendations
}

/// Display offload status for an interface
pub fn show_offload_status(interface: &str) -> Result<()> {
    let nic = detect_nic(interface)?;

    println!("🔧 Hardware Offload Status: {}", interface);
    println!("═══════════════════════════════════════");
    println!();
    println!("Driver: {}", nic.driver);
    println!("Bus: {}", nic.bus_info);
    if let Some(fw) = &nic.firmware_version {
        println!("Firmware: {}", fw);
    }

    // Match to profile
    if let Some(profile) = match_profile(&nic) {
        println!();
        println!("Detected chipset: {}", profile.family);

        if !profile.known_issues.is_empty() {
            println!();
            println!("⚠️  Known issues:");
            for issue in &profile.known_issues {
                println!("  - {}", issue);
            }
        }
    } else {
        println!();
        println!("ℹ️  Unknown chipset - using generic recommendations");
    }

    println!();
    println!("Current offload settings:");
    println!("─────────────────────────");

    // Show key offloads
    let key_offloads = [
        "rx-checksumming",
        "tx-checksumming",
        "tcp-segmentation-offload",
        "generic-segmentation-offload",
        "generic-receive-offload",
        "large-receive-offload",
        "ntuple-filters",
        "receive-hashing",
    ];

    for offload in key_offloads {
        if let Some(&enabled) = nic.current_offloads.get(offload) {
            let status = if enabled { "✅ on" } else { "❌ off" };
            let modifiable = nic.supported_offloads.get(offload).copied().unwrap_or(true);
            let fixed = if !modifiable { " [fixed]" } else { "" };
            println!("  {}: {}{}", offload, status, fixed);
        }
    }

    Ok(())
}

/// Show recommendations for an interface
pub fn show_recommendations(interface: &str) -> Result<()> {
    let nic = detect_nic(interface)?;
    let recommendations = get_recommendations(&nic);

    if recommendations.is_empty() {
        println!("✅ No recommended changes for {}", interface);
        println!("Current offload settings appear optimal for detected hardware.");
        return Ok(());
    }

    println!("📋 Recommended Offload Changes for {}", interface);
    println!("═══════════════════════════════════════════════");
    println!();

    for rec in &recommendations {
        let current_state = if rec.current { "on" } else { "off" };
        let recommended_state = if rec.recommended { "on" } else { "off" };

        println!("Setting: {}", rec.setting);
        println!("  Current: {}", current_state);
        println!("  Recommended: {}", recommended_state);
        println!("  Reason: {}", rec.reason);
        println!("  Command: {}", rec.command);
        println!();
    }

    println!("─────────────────────────────────────────────────");
    println!("To apply all recommendations:");
    println!();
    for rec in &recommendations {
        println!("  {}", rec.command);
    }
    println!();
    println!("⚠️  Test thoroughly after applying changes!");
    println!("💡 To make persistent, add to /etc/networkd-dispatcher or udev rules");

    Ok(())
}

/// Interactive offload management menu
pub fn offload_menu() {
    use crate::tui;

    println!("🔧 Hardware Offload Management");
    println!("==============================");

    let interfaces = match list_interfaces() {
        Ok(i) => i,
        Err(e) => {
            println!("❌ Failed to list interfaces: {}", e);
            return;
        }
    };

    if interfaces.is_empty() {
        println!("No network interfaces found.");
        return;
    }

    let options: Vec<String> = interfaces.iter().map(|i| format!("🔌 {}", i)).collect();

    loop {
        println!();
        if let Some(choice) = tui::select_with_back(
            "Select interface",
            &options.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            0,
        ) {
            let interface = &interfaces[choice];
            interface_menu(interface);
        } else {
            break;
        }
    }
}

fn interface_menu(interface: &str) {
    use crate::tui;

    let options = [
        "📊 Show current offload status",
        "📋 Show recommendations",
        "📜 Show all offload settings",
    ];

    loop {
        println!();
        if let Some(choice) =
            tui::select_with_back(&format!("Offload settings: {}", interface), &options, 0)
        {
            match choice {
                0 => {
                    if let Err(e) = show_offload_status(interface) {
                        println!("❌ Error: {}", e);
                    }
                }
                1 => {
                    if let Err(e) = show_recommendations(interface) {
                        println!("❌ Error: {}", e);
                    }
                }
                2 => {
                    // Show raw ethtool -k output
                    let _ = Command::new("ethtool").arg("-k").arg(interface).status();
                }
                _ => {}
            }
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nic_profiles_exist() {
        let profiles = get_nic_profiles();
        assert!(!profiles.is_empty());
    }

    #[test]
    fn test_driver_pattern_valid() {
        let profiles = get_nic_profiles();
        for profile in profiles {
            assert!(
                regex::Regex::new(profile.driver_pattern).is_ok(),
                "Invalid regex for {}: {}",
                profile.family,
                profile.driver_pattern
            );
        }
    }
}
