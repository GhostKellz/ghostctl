//! IOMMU group detection and management
//!
//! This module provides functionality for detecting, parsing, and displaying
//! IOMMU groups and their associated PCI devices.

use super::errors::{IommuError, IommuResult};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Represents a PCI device class
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PciDeviceClass {
    /// VGA compatible controller (0300)
    VgaController,
    /// 3D controller (0302)
    Display3D,
    /// Audio device (0403)
    AudioDevice,
    /// USB controller (0c03)
    UsbController,
    /// NVMe controller (0108)
    NvmeController,
    /// SATA controller (0106)
    SataController,
    /// Ethernet controller (0200)
    EthernetController,
    /// PCI bridge (0604)
    PciBridge,
    /// ISA bridge (0601)
    IsaBridge,
    /// Host bridge (0600)
    HostBridge,
    /// SMBus controller (0c05)
    SmbusController,
    /// Serial bus controller (0c80)
    SerialBusController,
    /// Signal processing controller (1180)
    SignalProcessor,
    /// Encryption controller (1080)
    EncryptionController,
    /// Other device class
    Other(String),
}

impl PciDeviceClass {
    /// Parse a PCI class code into a PciDeviceClass
    pub fn from_code(code: &str) -> Self {
        // Handle both "0300" and "0x0300" formats
        let code = code.trim_start_matches("0x").to_lowercase();

        // Take first 4 characters for class/subclass
        let class_code = if code.len() >= 4 { &code[..4] } else { &code };

        match class_code {
            "0300" => PciDeviceClass::VgaController,
            "0302" => PciDeviceClass::Display3D,
            "0403" => PciDeviceClass::AudioDevice,
            "0c03" => PciDeviceClass::UsbController,
            "0108" => PciDeviceClass::NvmeController,
            "0106" => PciDeviceClass::SataController,
            "0200" => PciDeviceClass::EthernetController,
            "0604" => PciDeviceClass::PciBridge,
            "0601" => PciDeviceClass::IsaBridge,
            "0600" => PciDeviceClass::HostBridge,
            "0c05" => PciDeviceClass::SmbusController,
            "0c80" => PciDeviceClass::SerialBusController,
            "1180" => PciDeviceClass::SignalProcessor,
            "1080" => PciDeviceClass::EncryptionController,
            _ => PciDeviceClass::Other(code.to_string()),
        }
    }

    /// Check if this device class is a GPU
    pub fn is_gpu(&self) -> bool {
        matches!(
            self,
            PciDeviceClass::VgaController | PciDeviceClass::Display3D
        )
    }

    /// Check if this device class is a bridge
    pub fn is_bridge(&self) -> bool {
        matches!(
            self,
            PciDeviceClass::PciBridge | PciDeviceClass::IsaBridge | PciDeviceClass::HostBridge
        )
    }

    /// Get a human-readable description
    pub fn description(&self) -> &str {
        match self {
            PciDeviceClass::VgaController => "VGA Controller",
            PciDeviceClass::Display3D => "3D Controller",
            PciDeviceClass::AudioDevice => "Audio Device",
            PciDeviceClass::UsbController => "USB Controller",
            PciDeviceClass::NvmeController => "NVMe Controller",
            PciDeviceClass::SataController => "SATA Controller",
            PciDeviceClass::EthernetController => "Ethernet Controller",
            PciDeviceClass::PciBridge => "PCI Bridge",
            PciDeviceClass::IsaBridge => "ISA Bridge",
            PciDeviceClass::HostBridge => "Host Bridge",
            PciDeviceClass::SmbusController => "SMBus Controller",
            PciDeviceClass::SerialBusController => "Serial Bus Controller",
            PciDeviceClass::SignalProcessor => "Signal Processor",
            PciDeviceClass::EncryptionController => "Encryption Controller",
            PciDeviceClass::Other(code) => code.as_str(),
        }
    }
}

/// Represents a PCI device
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PciDevice {
    /// PCI address (e.g., "0000:01:00.0")
    pub address: String,
    /// Vendor ID (e.g., "10de" for NVIDIA)
    pub vendor_id: String,
    /// Device ID
    pub device_id: String,
    /// Device class
    pub class: PciDeviceClass,
    /// Human-readable description
    pub description: String,
    /// Currently bound driver
    pub current_driver: Option<String>,
    /// IOMMU group this device belongs to
    pub iommu_group: u32,
    /// Subsystem vendor ID
    pub subsystem_vendor: Option<String>,
    /// Subsystem device ID
    pub subsystem_device: Option<String>,
    /// Vendor name (e.g., "NVIDIA Corporation")
    pub vendor_name: Option<String>,
}

impl PciDevice {
    /// Check if this device is a GPU
    pub fn is_gpu(&self) -> bool {
        self.class.is_gpu()
    }

    /// Check if this is an NVIDIA device
    pub fn is_nvidia(&self) -> bool {
        self.vendor_id.to_lowercase() == "10de"
    }

    /// Check if this is an AMD device
    pub fn is_amd(&self) -> bool {
        self.vendor_id.to_lowercase() == "1002"
    }

    /// Check if this is an Intel device
    pub fn is_intel(&self) -> bool {
        self.vendor_id.to_lowercase() == "8086"
    }

    /// Get the slot portion of the address (without function)
    pub fn slot(&self) -> &str {
        self.address.rsplit('.').nth(1).unwrap_or(&self.address)
    }

    /// Get the function number
    pub fn function(&self) -> Option<u8> {
        self.address.rsplit('.').next().and_then(|f| f.parse().ok())
    }
}

/// Represents an IOMMU group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IommuGroup {
    /// Group ID
    pub id: u32,
    /// Devices in this group
    pub devices: Vec<PciDevice>,
    /// Isolation score (0-100, higher is better for passthrough)
    pub isolation_score: u8,
    /// Whether the group is well-isolated for passthrough
    pub is_isolated: bool,
    /// Warnings about this group
    pub warnings: Vec<String>,
}

impl IommuGroup {
    /// Check if this group contains any GPUs
    pub fn has_gpu(&self) -> bool {
        self.devices.iter().any(|d| d.is_gpu())
    }

    /// Get all GPUs in this group
    pub fn gpus(&self) -> Vec<&PciDevice> {
        self.devices.iter().filter(|d| d.is_gpu()).collect()
    }

    /// Check if all non-bridge devices are on the same slot
    pub fn same_slot(&self) -> bool {
        let non_bridge_devices: Vec<_> = self
            .devices
            .iter()
            .filter(|d| !d.class.is_bridge())
            .collect();

        if non_bridge_devices.len() <= 1 {
            return true;
        }

        let first_slot = non_bridge_devices[0].slot();
        non_bridge_devices.iter().all(|d| d.slot() == first_slot)
    }
}

/// IOMMU status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IommuStatus {
    /// Whether IOMMU is enabled
    pub enabled: bool,
    /// IOMMU mode (Intel or AMD)
    pub mode: IommuMode,
    /// Number of IOMMU groups
    pub group_count: usize,
    /// Whether iommu=pt is present (passthrough mode)
    pub passthrough_pt: bool,
    /// Raw kernel parameters
    pub kernel_params: String,
    /// Interrupt remapping supported
    pub interrupt_remapping: bool,
}

/// IOMMU mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IommuMode {
    /// Intel VT-d
    Intel,
    /// AMD-Vi
    Amd,
    /// Unknown or not detected
    Unknown,
    /// IOMMU disabled
    Disabled,
}

impl std::fmt::Display for IommuMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IommuMode::Intel => write!(f, "Intel VT-d"),
            IommuMode::Amd => write!(f, "AMD-Vi"),
            IommuMode::Unknown => write!(f, "Unknown"),
            IommuMode::Disabled => write!(f, "Disabled"),
        }
    }
}

/// Get the current IOMMU status
pub fn get_iommu_status() -> Result<IommuStatus> {
    let kernel_params = fs::read_to_string("/proc/cmdline")
        .context("Failed to read kernel command line")?
        .trim()
        .to_string();

    let has_intel_iommu = kernel_params.contains("intel_iommu=on");
    let has_amd_iommu = kernel_params.contains("amd_iommu=on");
    let passthrough_pt = kernel_params.contains("iommu=pt");

    let mode = if has_intel_iommu {
        IommuMode::Intel
    } else if has_amd_iommu {
        IommuMode::Amd
    } else {
        // Check if IOMMU groups exist even without explicit kernel params
        let iommu_path = Path::new("/sys/kernel/iommu_groups");
        if iommu_path.exists() {
            if let Ok(entries) = fs::read_dir(iommu_path) {
                if entries.count() > 0 {
                    // Detect CPU vendor to determine IOMMU type
                    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
                        if cpuinfo.contains("GenuineIntel") {
                            IommuMode::Intel
                        } else if cpuinfo.contains("AuthenticAMD") {
                            IommuMode::Amd
                        } else {
                            IommuMode::Unknown
                        }
                    } else {
                        IommuMode::Unknown
                    }
                } else {
                    IommuMode::Disabled
                }
            } else {
                IommuMode::Disabled
            }
        } else {
            IommuMode::Disabled
        }
    };

    let enabled = mode != IommuMode::Disabled;

    let group_count = if enabled {
        count_iommu_groups().unwrap_or(0)
    } else {
        0
    };

    // Check interrupt remapping
    let interrupt_remapping = check_interrupt_remapping();

    Ok(IommuStatus {
        enabled,
        mode,
        group_count,
        passthrough_pt,
        kernel_params,
        interrupt_remapping,
    })
}

/// Count the number of IOMMU groups
fn count_iommu_groups() -> Result<usize> {
    let iommu_path = Path::new("/sys/kernel/iommu_groups");
    if !iommu_path.exists() {
        return Ok(0);
    }

    let count = fs::read_dir(iommu_path)
        .context("Failed to read IOMMU groups directory")?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .count();

    Ok(count)
}

/// Check if interrupt remapping is supported
fn check_interrupt_remapping() -> bool {
    // Check dmesg for interrupt remapping
    if let Ok(output) = Command::new("dmesg").output() {
        let dmesg = String::from_utf8_lossy(&output.stdout);
        return dmesg.contains("Interrupt remapping enabled")
            || dmesg.contains("Enabled IRQ remapping");
    }
    false
}

/// List all IOMMU groups
pub fn list_iommu_groups() -> Result<Vec<IommuGroup>> {
    let iommu_path = Path::new("/sys/kernel/iommu_groups");

    if !iommu_path.exists() {
        return Err(IommuError::GroupsNotAccessible(
            "IOMMU groups directory does not exist".to_string(),
        )
        .into());
    }

    let mut groups = Vec::new();

    let entries = fs::read_dir(iommu_path).context("Failed to read IOMMU groups directory")?;

    for entry in entries.filter_map(|e| e.ok()) {
        let group_name = entry.file_name();
        let group_id: u32 = group_name
            .to_string_lossy()
            .parse()
            .map_err(|_| IommuError::ParseError("Invalid group ID".to_string()))?;

        match get_iommu_group(group_id) {
            Ok(group) => groups.push(group),
            Err(e) => {
                // Log but continue with other groups
                eprintln!("Warning: Failed to parse group {}: {}", group_id, e);
            }
        }
    }

    // Sort by group ID
    groups.sort_by_key(|g| g.id);

    Ok(groups)
}

/// Get a specific IOMMU group by ID
pub fn get_iommu_group(group_id: u32) -> Result<IommuGroup> {
    let group_path = format!("/sys/kernel/iommu_groups/{}", group_id);
    let devices_path = format!("{}/devices", group_path);

    if !Path::new(&group_path).exists() {
        return Err(IommuError::GroupParseError {
            group: group_id,
            reason: "Group does not exist".to_string(),
        }
        .into());
    }

    let mut devices = Vec::new();

    let entries = fs::read_dir(&devices_path)
        .with_context(|| format!("Failed to read devices in IOMMU group {}", group_id))?;

    for entry in entries.filter_map(|e| e.ok()) {
        let device_name = entry.file_name();
        let pci_address = device_name.to_string_lossy().to_string();

        match get_pci_device_info(&pci_address, group_id) {
            Ok(device) => devices.push(device),
            Err(e) => {
                eprintln!(
                    "Warning: Failed to get info for device {}: {}",
                    pci_address, e
                );
            }
        }
    }

    // Calculate isolation score and warnings
    let (isolation_score, warnings) = calculate_group_isolation(&devices);
    let is_isolated = isolation_score >= 70;

    Ok(IommuGroup {
        id: group_id,
        devices,
        isolation_score,
        is_isolated,
        warnings,
    })
}

/// Get detailed information about a PCI device
fn get_pci_device_info(pci_address: &str, iommu_group: u32) -> Result<PciDevice> {
    // Use lspci to get device information
    let output = Command::new("lspci")
        .args(["-nns", pci_address])
        .output()
        .context("Failed to run lspci")?;

    let lspci_line = String::from_utf8_lossy(&output.stdout);
    let lspci_line = lspci_line.trim();

    if lspci_line.is_empty() {
        return Err(IommuError::DeviceNotFound(pci_address.to_string()).into());
    }

    // Parse lspci output
    // Format: "01:00.0 VGA compatible controller [0300]: NVIDIA Corporation ... [10de:2684] (rev a1)"
    let (vendor_id, device_id, class_code, description) = parse_lspci_line(lspci_line)?;

    // Get current driver from sysfs
    let driver_link = format!("/sys/bus/pci/devices/{}/driver", pci_address);
    let current_driver = if let Ok(link) = fs::read_link(&driver_link) {
        link.file_name().map(|n| n.to_string_lossy().to_string())
    } else {
        None
    };

    // Format address with domain if needed
    let full_address = if pci_address.contains(':') && !pci_address.starts_with("0000:") {
        format!("0000:{}", pci_address)
    } else {
        pci_address.to_string()
    };

    // Get vendor name
    let vendor_name = get_vendor_name(&vendor_id);

    Ok(PciDevice {
        address: full_address,
        vendor_id,
        device_id,
        class: PciDeviceClass::from_code(&class_code),
        description,
        current_driver,
        iommu_group,
        subsystem_vendor: None,
        subsystem_device: None,
        vendor_name,
    })
}

/// Parse a line of lspci output
fn parse_lspci_line(line: &str) -> Result<(String, String, String, String)> {
    // Example: "01:00.0 VGA compatible controller [0300]: NVIDIA Corporation GA102 [GeForce RTX 3090] [10de:2204] (rev a1)"

    // Extract vendor:device IDs from the end [xxxx:yyyy]
    let vendor_device_re = regex::Regex::new(r"\[([0-9a-fA-F]{4}):([0-9a-fA-F]{4})\]")
        .map_err(|e| IommuError::ParseError(e.to_string()))?;

    let caps = vendor_device_re
        .captures_iter(line)
        .last()
        .ok_or_else(|| IommuError::ParseError("No vendor:device ID found".to_string()))?;

    let vendor_id = caps
        .get(1)
        .ok_or_else(|| IommuError::ParseError("No vendor ID found in lspci output".to_string()))?
        .as_str()
        .to_string();
    let device_id = caps
        .get(2)
        .ok_or_else(|| IommuError::ParseError("No device ID found in lspci output".to_string()))?
        .as_str()
        .to_string();

    // Extract class code [xxxx]
    let class_re = regex::Regex::new(r"\[([0-9a-fA-F]{4})\]:")
        .map_err(|e| IommuError::ParseError(e.to_string()))?;

    let class_code = class_re
        .captures(line)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| "0000".to_string());

    // Extract description - everything between class code and vendor:device
    let description = if let Some(class_match) = class_re.find(line) {
        let start = class_match.end();
        let end = line
            .rfind('[')
            .and_then(|i| if i > start { Some(i) } else { None })
            .unwrap_or(line.len());
        line[start..end].trim().to_string()
    } else {
        line.to_string()
    };

    Ok((vendor_id, device_id, class_code, description))
}

/// Get vendor name from vendor ID
fn get_vendor_name(vendor_id: &str) -> Option<String> {
    let vendor_names: HashMap<&str, &str> = [
        ("10de", "NVIDIA Corporation"),
        ("1002", "AMD/ATI"),
        ("8086", "Intel Corporation"),
        ("1022", "AMD"),
        ("10ec", "Realtek"),
        ("14e4", "Broadcom"),
        ("168c", "Qualcomm Atheros"),
        ("1b4b", "Marvell"),
        ("1b21", "ASMedia"),
        ("144d", "Samsung"),
    ]
    .into_iter()
    .collect();

    vendor_names
        .get(vendor_id.to_lowercase().as_str())
        .map(|s| s.to_string())
}

/// Calculate isolation score and warnings for a group
fn calculate_group_isolation(devices: &[PciDevice]) -> (u8, Vec<String>) {
    let mut warnings = Vec::new();
    let mut score: i32 = 100;

    // Count device types
    let non_bridge_devices: Vec<_> = devices.iter().filter(|d| !d.class.is_bridge()).collect();

    let gpu_count = devices.iter().filter(|d| d.is_gpu()).count();
    let bridge_count = devices.iter().filter(|d| d.class.is_bridge()).count();

    // Single device (excluding bridges) is perfect
    if non_bridge_devices.len() == 1 {
        return (100, warnings);
    }

    // Multiple devices on the same slot (e.g., GPU + audio) is acceptable
    let slots: std::collections::HashSet<_> = non_bridge_devices.iter().map(|d| d.slot()).collect();

    if slots.len() == 1 {
        // All on same slot - likely GPU with audio function
        if gpu_count == 1 && non_bridge_devices.len() == 2 {
            // GPU + audio is common and fine
            return (95, warnings);
        }
        score = 85;
    } else {
        // Different slots means isolation issues
        score -= 30;
        warnings.push(format!(
            "Group contains {} devices on {} different slots",
            non_bridge_devices.len(),
            slots.len()
        ));
    }

    // Check for problematic device combinations
    let has_usb = devices
        .iter()
        .any(|d| matches!(d.class, PciDeviceClass::UsbController));
    let has_sata = devices
        .iter()
        .any(|d| matches!(d.class, PciDeviceClass::SataController));
    let has_ethernet = devices
        .iter()
        .any(|d| matches!(d.class, PciDeviceClass::EthernetController));

    if gpu_count > 0 {
        if has_usb {
            score -= 20;
            warnings.push("Group contains both GPU and USB controller".to_string());
        }
        if has_sata {
            score -= 20;
            warnings.push("Group contains both GPU and SATA controller".to_string());
        }
        if has_ethernet {
            score -= 15;
            warnings.push("Group contains both GPU and Ethernet controller".to_string());
        }
    }

    // Multiple GPUs in same group is problematic
    if gpu_count > 1 {
        score -= 25;
        warnings.push(format!(
            "Group contains {} GPUs - cannot pass through individually",
            gpu_count
        ));
    }

    // Bridges themselves don't hurt much
    if bridge_count > 0 && non_bridge_devices.len() > 1 {
        score -= 5;
    }

    // ACS issue warning
    if !slots.is_empty() && slots.len() > 1 {
        warnings
            .push("Consider ACS override patch if you need to isolate these devices".to_string());
    }

    (score.max(0) as u8, warnings)
}

/// Find which IOMMU group a device belongs to
pub fn find_device_group(pci_address: &str) -> Result<IommuGroup> {
    let normalized_address = normalize_pci_address(pci_address)?;

    let iommu_link = format!("/sys/bus/pci/devices/{}/iommu_group", normalized_address);

    let group_path = fs::read_link(&iommu_link).with_context(|| {
        format!(
            "Device {} not found or has no IOMMU group",
            normalized_address
        )
    })?;

    let group_id: u32 = group_path
        .file_name()
        .and_then(|n| n.to_str())
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| IommuError::ParseError("Invalid IOMMU group path".to_string()))?;

    get_iommu_group(group_id)
}

/// Normalize a PCI address to full format (0000:xx:yy.z)
fn normalize_pci_address(address: &str) -> IommuResult<String> {
    let address = address.trim();

    // Already full format
    if address.len() == 12 && address.chars().filter(|c| *c == ':').count() == 2 {
        return Ok(address.to_string());
    }

    // Short format (xx:yy.z)
    if address.len() == 7 && address.chars().filter(|c| *c == ':').count() == 1 {
        return Ok(format!("0000:{}", address));
    }

    Err(IommuError::InvalidPciAddress(address.to_string()))
}

/// List all GPU devices
pub fn list_gpus() -> Result<Vec<PciDevice>> {
    let groups = list_iommu_groups()?;
    let gpus: Vec<PciDevice> = groups
        .into_iter()
        .flat_map(|g| g.devices)
        .filter(|d| d.is_gpu())
        .collect();
    Ok(gpus)
}

/// List all USB controllers
pub fn list_usb_controllers() -> Result<Vec<PciDevice>> {
    let groups = list_iommu_groups()?;
    let controllers: Vec<PciDevice> = groups
        .into_iter()
        .flat_map(|g| g.devices)
        .filter(|d| matches!(d.class, PciDeviceClass::UsbController))
        .collect();
    Ok(controllers)
}

/// List all NVMe controllers
pub fn list_nvme_controllers() -> Result<Vec<PciDevice>> {
    let groups = list_iommu_groups()?;
    let controllers: Vec<PciDevice> = groups
        .into_iter()
        .flat_map(|g| g.devices)
        .filter(|d| matches!(d.class, PciDeviceClass::NvmeController))
        .collect();
    Ok(controllers)
}

/// List all SATA controllers
pub fn list_sata_controllers() -> Result<Vec<PciDevice>> {
    let groups = list_iommu_groups()?;
    let controllers: Vec<PciDevice> = groups
        .into_iter()
        .flat_map(|g| g.devices)
        .filter(|d| matches!(d.class, PciDeviceClass::SataController))
        .collect();
    Ok(controllers)
}

/// Print IOMMU groups to stdout (for CLI)
pub fn print_groups(groups: &[IommuGroup], gpu_only: bool) {
    let filtered: Vec<_> = if gpu_only {
        groups.iter().filter(|g| g.has_gpu()).collect()
    } else {
        groups.iter().collect()
    };

    if filtered.is_empty() {
        if gpu_only {
            println!("No IOMMU groups containing GPUs found.");
        } else {
            println!("No IOMMU groups found. Is IOMMU enabled?");
        }
        return;
    }

    for group in filtered {
        let isolation_indicator = if group.isolation_score >= 90 {
            "excellent"
        } else if group.isolation_score >= 70 {
            "good"
        } else if group.isolation_score >= 50 {
            "moderate"
        } else {
            "poor"
        };

        println!(
            "\nIOMMU Group {} [score: {}/100 - {}]",
            group.id, group.isolation_score, isolation_indicator
        );
        println!("{}", "-".repeat(50));

        for device in &group.devices {
            let driver_info = device
                .current_driver
                .as_ref()
                .map(|d| format!(" [{}]", d))
                .unwrap_or_default();

            let gpu_marker = if device.is_gpu() { " (GPU)" } else { "" };

            println!(
                "  {} {}:{} {}{}{}",
                device.address,
                device.vendor_id,
                device.device_id,
                device.description,
                driver_info,
                gpu_marker
            );
        }

        for warning in &group.warnings {
            println!("  Warning: {}", warning);
        }
    }
}

/// Print groups as JSON
pub fn print_groups_json(groups: &[IommuGroup], gpu_only: bool) -> Result<()> {
    let filtered: Vec<_> = if gpu_only {
        groups.iter().filter(|g| g.has_gpu()).cloned().collect()
    } else {
        groups.to_vec()
    };

    let json = serde_json::to_string_pretty(&filtered).context("Failed to serialize to JSON")?;
    println!("{}", json);
    Ok(())
}

/// CLI helper for listing groups
pub fn list_groups_cli(gpu_only: bool, json_output: bool) {
    match list_iommu_groups() {
        Ok(groups) => {
            if json_output {
                if let Err(e) = print_groups_json(&groups, gpu_only) {
                    eprintln!("Error: {}", e);
                }
            } else {
                print_groups(&groups, gpu_only);
            }
        }
        Err(e) => {
            eprintln!("Error listing IOMMU groups: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pci_device_class_from_code() {
        assert_eq!(
            PciDeviceClass::from_code("0300"),
            PciDeviceClass::VgaController
        );
        assert_eq!(PciDeviceClass::from_code("0302"), PciDeviceClass::Display3D);
        assert_eq!(
            PciDeviceClass::from_code("0403"),
            PciDeviceClass::AudioDevice
        );
        assert_eq!(
            PciDeviceClass::from_code("0c03"),
            PciDeviceClass::UsbController
        );
        assert_eq!(
            PciDeviceClass::from_code("0x0300"),
            PciDeviceClass::VgaController
        );
    }

    #[test]
    fn test_pci_device_class_is_gpu() {
        assert!(PciDeviceClass::VgaController.is_gpu());
        assert!(PciDeviceClass::Display3D.is_gpu());
        assert!(!PciDeviceClass::AudioDevice.is_gpu());
        assert!(!PciDeviceClass::UsbController.is_gpu());
    }

    #[test]
    fn test_pci_device_class_is_bridge() {
        assert!(PciDeviceClass::PciBridge.is_bridge());
        assert!(PciDeviceClass::IsaBridge.is_bridge());
        assert!(PciDeviceClass::HostBridge.is_bridge());
        assert!(!PciDeviceClass::VgaController.is_bridge());
    }

    #[test]
    fn test_normalize_pci_address() {
        assert_eq!(
            normalize_pci_address("01:00.0").unwrap(),
            "0000:01:00.0".to_string()
        );
        assert_eq!(
            normalize_pci_address("0000:01:00.0").unwrap(),
            "0000:01:00.0".to_string()
        );
        assert!(normalize_pci_address("invalid").is_err());
    }

    #[test]
    fn test_isolation_score_single_device() {
        let devices = vec![PciDevice {
            address: "0000:01:00.0".to_string(),
            vendor_id: "10de".to_string(),
            device_id: "2684".to_string(),
            class: PciDeviceClass::VgaController,
            description: "Test GPU".to_string(),
            current_driver: Some("nvidia".to_string()),
            iommu_group: 1,
            subsystem_vendor: None,
            subsystem_device: None,
            vendor_name: Some("NVIDIA Corporation".to_string()),
        }];

        let (score, warnings) = calculate_group_isolation(&devices);
        assert_eq!(score, 100);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_isolation_score_gpu_with_audio() {
        let devices = vec![
            PciDevice {
                address: "0000:01:00.0".to_string(),
                vendor_id: "10de".to_string(),
                device_id: "2684".to_string(),
                class: PciDeviceClass::VgaController,
                description: "Test GPU".to_string(),
                current_driver: Some("nvidia".to_string()),
                iommu_group: 1,
                subsystem_vendor: None,
                subsystem_device: None,
                vendor_name: Some("NVIDIA Corporation".to_string()),
            },
            PciDevice {
                address: "0000:01:00.1".to_string(),
                vendor_id: "10de".to_string(),
                device_id: "22ba".to_string(),
                class: PciDeviceClass::AudioDevice,
                description: "Test Audio".to_string(),
                current_driver: Some("snd_hda_intel".to_string()),
                iommu_group: 1,
                subsystem_vendor: None,
                subsystem_device: None,
                vendor_name: Some("NVIDIA Corporation".to_string()),
            },
        ];

        let (score, _warnings) = calculate_group_isolation(&devices);
        assert!(score >= 90); // GPU + audio on same slot should be high score
    }

    #[test]
    fn test_get_vendor_name() {
        assert_eq!(
            get_vendor_name("10de"),
            Some("NVIDIA Corporation".to_string())
        );
        assert_eq!(get_vendor_name("1002"), Some("AMD/ATI".to_string()));
        assert_eq!(
            get_vendor_name("8086"),
            Some("Intel Corporation".to_string())
        );
        assert_eq!(get_vendor_name("ffff"), None);
    }
}
