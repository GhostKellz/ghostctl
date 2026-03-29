//! PCIe topology mapping
//!
//! This module provides functionality for mapping the PCIe device tree
//! and understanding device relationships for passthrough planning.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

/// PCIe device type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PcieDeviceType {
    /// Root complex / host bridge
    RootComplex,
    /// PCIe root port
    RootPort,
    /// Upstream switch port
    UpstreamPort,
    /// Downstream switch port
    DownstreamPort,
    /// PCI-to-PCI bridge
    PciBridge,
    /// Endpoint device
    Endpoint,
    /// Unknown type
    Unknown,
}

impl std::fmt::Display for PcieDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PcieDeviceType::RootComplex => write!(f, "Root Complex"),
            PcieDeviceType::RootPort => write!(f, "Root Port"),
            PcieDeviceType::UpstreamPort => write!(f, "Upstream Port"),
            PcieDeviceType::DownstreamPort => write!(f, "Downstream Port"),
            PcieDeviceType::PciBridge => write!(f, "PCI Bridge"),
            PcieDeviceType::Endpoint => write!(f, "Endpoint"),
            PcieDeviceType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// A node in the PCIe topology tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PcieNode {
    /// PCI address
    pub address: String,
    /// Device description
    pub description: String,
    /// Device type
    pub device_type: PcieDeviceType,
    /// Vendor:device ID
    pub device_id: String,
    /// IOMMU group (if any)
    pub iommu_group: Option<u32>,
    /// Current driver
    pub driver: Option<String>,
    /// Link speed (e.g., "8GT/s")
    pub link_speed: Option<String>,
    /// Link width (e.g., "x16")
    pub link_width: Option<String>,
    /// Child devices
    pub children: Vec<PcieNode>,
}

/// Full PCIe topology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PcieTopology {
    /// Root complexes (typically one per CPU socket)
    pub root_complexes: Vec<PcieNode>,
    /// Total device count
    pub total_devices: usize,
    /// Summary of devices by type
    pub device_summary: HashMap<String, usize>,
}

/// Build the PCIe topology tree
pub fn build_pcie_tree() -> Result<PcieTopology> {
    let mut device_summary: HashMap<String, usize> = HashMap::new();

    // Get all PCI devices using lspci -tv for tree view
    let tree_output = Command::new("lspci")
        .args(["-tv"])
        .output()
        .context("Failed to run lspci -tv")?;

    let tree_text = String::from_utf8_lossy(&tree_output.stdout);

    // Get detailed info for all devices
    let detail_output = Command::new("lspci")
        .args(["-vvnn"])
        .output()
        .context("Failed to run lspci -vvnn")?;

    let detail_text = String::from_utf8_lossy(&detail_output.stdout);

    // Parse the device details into a map
    let device_details = parse_device_details(&detail_text)?;

    // Parse the tree structure
    let (root_complexes, total_devices) = parse_tree_structure(&tree_text, &device_details)?;

    // Count device types
    count_device_types(&root_complexes, &mut device_summary);

    Ok(PcieTopology {
        root_complexes,
        total_devices,
        device_summary,
    })
}

/// Parse detailed device information from lspci -vvnn output
fn parse_device_details(output: &str) -> Result<HashMap<String, DeviceDetail>> {
    let mut details = HashMap::new();
    let mut current_address = String::new();
    let mut current_detail = DeviceDetail::default();

    for line in output.lines() {
        if !line.starts_with('\t') && !line.starts_with(' ') && line.contains(' ') {
            // Save previous device
            if !current_address.is_empty() {
                details.insert(current_address.clone(), current_detail.clone());
            }

            // New device line: "01:00.0 VGA compatible controller [0300]: NVIDIA..."
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() >= 2 {
                current_address = parts[0].to_string();
                current_detail = DeviceDetail::default();
                current_detail.description = parts[1].to_string();

                // Extract device class
                if let Some(class_start) = parts[1].find('[') {
                    if let Some(class_end) = parts[1][class_start..].find(']') {
                        let class_code = &parts[1][class_start + 1..class_start + class_end];
                        if class_code.len() == 4 {
                            current_detail.class_code = class_code.to_string();
                        }
                    }
                }

                // Extract vendor:device
                if let Some(id_match) = line
                    .rfind('[')
                    .and_then(|start| line[start..].find(']').map(|end| (start, start + end)))
                {
                    let id_part = &line[id_match.0 + 1..id_match.1];
                    if id_part.contains(':') && id_part.len() == 9 {
                        current_detail.device_id = id_part.to_string();
                    }
                }
            }
        } else if line.contains("LnkSta:") {
            // Link status line
            if let Some(speed_start) = line.find("Speed ") {
                if let Some(speed_end) = line[speed_start..].find(',') {
                    current_detail.link_speed =
                        Some(line[speed_start + 6..speed_start + speed_end].to_string());
                }
            }
            if let Some(width_start) = line.find("Width x") {
                let width_part = &line[width_start + 6..];
                if let Some(width_end) = width_part.find(|c: char| !c.is_numeric() && c != 'x') {
                    current_detail.link_width = Some(width_part[..width_end].to_string());
                }
            }
        } else if line.contains("DevCap:") || line.contains("DevCtl:") {
            // Device capabilities
            if line.contains("Bridge") {
                current_detail.is_bridge = true;
            }
        } else if line.contains("Kernel driver in use:") {
            current_detail.driver = line.split(':').nth(1).map(|s| s.trim().to_string());
        }
    }

    // Save last device
    if !current_address.is_empty() {
        details.insert(current_address, current_detail);
    }

    Ok(details)
}

#[derive(Debug, Clone, Default)]
struct DeviceDetail {
    description: String,
    class_code: String,
    device_id: String,
    link_speed: Option<String>,
    link_width: Option<String>,
    driver: Option<String>,
    is_bridge: bool,
}

/// Parse the tree structure from lspci -tv output
fn parse_tree_structure(
    tree_output: &str,
    details: &HashMap<String, DeviceDetail>,
) -> Result<(Vec<PcieNode>, usize)> {
    let mut roots = Vec::new();
    let mut count = 0;

    // Parse lspci -tv output
    // Format is like:
    // -[0000:00]-+-00.0  AMD Device [...]
    //            +-01.0-[01]----00.0  NVIDIA Corporation [...]
    //            \-02.0  Intel Corporation [...]

    for line in tree_output.lines() {
        if line.starts_with("-[") {
            // Root complex line
            if let Some(domain_end) = line.find("]-") {
                let domain = &line[2..domain_end];

                let node = PcieNode {
                    address: format!("{}:00.0", domain),
                    description: format!("Root Complex {}", domain),
                    device_type: PcieDeviceType::RootComplex,
                    device_id: String::new(),
                    iommu_group: None,
                    driver: None,
                    link_speed: None,
                    link_width: None,
                    children: Vec::new(),
                };

                roots.push(node);
                count += 1;
            }
        } else if line.contains("00.0") || line.contains("-[") {
            // Device line - extract address
            let address = extract_address_from_tree_line(line);
            if let Some(addr) = address {
                count += 1;
            }
        }
    }

    // If parsing failed, fall back to simple enumeration
    if roots.is_empty() {
        let simple_nodes = build_simple_tree(details)?;
        count = count_nodes(&simple_nodes);
        return Ok((simple_nodes, count));
    }

    Ok((roots, count))
}

/// Extract PCI address from a tree line
fn extract_address_from_tree_line(line: &str) -> Option<String> {
    // Look for patterns like "00.0" or "01:00.0"
    let parts: Vec<&str> = line.split_whitespace().collect();
    for part in parts {
        if part.contains('.')
            && part
                .chars()
                .all(|c| c.is_ascii_hexdigit() || c == '.' || c == ':')
        {
            if part.len() >= 4 {
                return Some(part.to_string());
            }
        }
    }
    None
}

/// Build a simple tree when detailed parsing fails
fn build_simple_tree(details: &HashMap<String, DeviceDetail>) -> Result<Vec<PcieNode>> {
    let mut nodes_by_bus: HashMap<String, Vec<PcieNode>> = HashMap::new();

    // Group devices by bus
    for (address, detail) in details {
        let bus = address.split(':').next().unwrap_or("00");

        let device_type = determine_device_type(&detail.class_code, detail.is_bridge);

        let iommu_group = get_device_iommu_group(address);

        let node = PcieNode {
            address: format!("0000:{}", address),
            description: detail.description.clone(),
            device_type,
            device_id: detail.device_id.clone(),
            iommu_group,
            driver: detail.driver.clone(),
            link_speed: detail.link_speed.clone(),
            link_width: detail.link_width.clone(),
            children: Vec::new(),
        };

        nodes_by_bus.entry(bus.to_string()).or_default().push(node);
    }

    // Create root nodes for each unique root bus
    let mut roots = Vec::new();
    for (bus, devices) in nodes_by_bus {
        if bus == "00" {
            // Bus 00 devices are typically at the root
            for device in devices {
                roots.push(device);
            }
        }
    }

    // Sort by address
    roots.sort_by(|a, b| a.address.cmp(&b.address));

    Ok(roots)
}

/// Determine device type from class code
fn determine_device_type(class_code: &str, is_bridge: bool) -> PcieDeviceType {
    let code = class_code.to_lowercase();

    if code.starts_with("06") {
        // Bridge class
        match code.as_str() {
            "0600" => PcieDeviceType::RootComplex, // Host bridge
            "0604" => {
                if is_bridge {
                    PcieDeviceType::RootPort
                } else {
                    PcieDeviceType::PciBridge
                }
            }
            _ => PcieDeviceType::PciBridge,
        }
    } else {
        PcieDeviceType::Endpoint
    }
}

/// Get IOMMU group for a device
fn get_device_iommu_group(address: &str) -> Option<u32> {
    let full_addr = if address.contains(':') && !address.starts_with("0000:") {
        format!("0000:{}", address)
    } else {
        address.to_string()
    };

    let link_path = format!("/sys/bus/pci/devices/{}/iommu_group", full_addr);
    if let Ok(link) = fs::read_link(&link_path) {
        link.file_name()
            .and_then(|n| n.to_str())
            .and_then(|s| s.parse().ok())
    } else {
        None
    }
}

/// Count nodes in tree
fn count_nodes(nodes: &[PcieNode]) -> usize {
    let mut count = nodes.len();
    for node in nodes {
        count += count_nodes(&node.children);
    }
    count
}

/// Count device types in tree
fn count_device_types(nodes: &[PcieNode], summary: &mut HashMap<String, usize>) {
    for node in nodes {
        let type_name = format!("{}", node.device_type);
        *summary.entry(type_name).or_insert(0) += 1;
        count_device_types(&node.children, summary);
    }
}

/// Print PCIe topology tree
pub fn print_pcie_tree() -> Result<()> {
    println!("\nPCIe Topology Tree");
    println!("{}", "=".repeat(60));

    // Use lspci -tv for a clean tree view
    let output = Command::new("lspci")
        .args(["-tv"])
        .output()
        .context("Failed to run lspci -tv")?;

    let tree = String::from_utf8_lossy(&output.stdout);

    if tree.is_empty() {
        println!("No PCIe devices found or lspci not available.");
        return Ok(());
    }

    // Print the tree with IOMMU group annotations
    let groups_map = build_iommu_groups_map()?;

    for line in tree.lines() {
        // Try to extract address and add IOMMU group info
        if let Some(addr) = extract_address_from_tree_line(line) {
            let group_info = groups_map
                .get(&addr)
                .map(|g| format!(" [IOMMU:{}]", g))
                .unwrap_or_default();

            println!("{}{}", line, group_info);
        } else {
            println!("{}", line);
        }
    }

    // Print summary
    println!("\nIOMMU Group Summary:");
    let mut group_counts: HashMap<u32, usize> = HashMap::new();
    for group in groups_map.values() {
        *group_counts.entry(*group).or_insert(0) += 1;
    }

    let mut groups: Vec<_> = group_counts.into_iter().collect();
    groups.sort_by_key(|(g, _)| *g);

    for (group, count) in groups {
        println!("  Group {}: {} devices", group, count);
    }

    Ok(())
}

/// Build a map of PCI address to IOMMU group
fn build_iommu_groups_map() -> Result<HashMap<String, u32>> {
    let mut map = HashMap::new();

    let iommu_path = Path::new("/sys/kernel/iommu_groups");
    if !iommu_path.exists() {
        return Ok(map);
    }

    let entries = fs::read_dir(iommu_path).context("Failed to read IOMMU groups directory")?;

    for entry in entries.filter_map(|e| e.ok()) {
        let group_id: u32 = entry.file_name().to_string_lossy().parse().unwrap_or(0);

        let devices_path = entry.path().join("devices");
        if let Ok(devices) = fs::read_dir(devices_path) {
            for device in devices.filter_map(|d| d.ok()) {
                let addr = device.file_name().to_string_lossy().to_string();
                // Store both full and short addresses
                map.insert(addr.clone(), group_id);
                if addr.starts_with("0000:") {
                    map.insert(addr[5..].to_string(), group_id);
                }
            }
        }
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_device_type_host_bridge() {
        assert_eq!(
            determine_device_type("0600", false),
            PcieDeviceType::RootComplex
        );
    }

    #[test]
    fn test_determine_device_type_pci_bridge() {
        assert_eq!(
            determine_device_type("0604", false),
            PcieDeviceType::PciBridge
        );
    }

    #[test]
    fn test_determine_device_type_endpoint() {
        assert_eq!(
            determine_device_type("0300", false),
            PcieDeviceType::Endpoint
        );
    }

    #[test]
    fn test_extract_address() {
        // Function extracts whitespace-separated addresses
        assert_eq!(
            extract_address_from_tree_line("01:00.0 NVIDIA Corporation"),
            Some("01:00.0".to_string())
        );
    }
}
