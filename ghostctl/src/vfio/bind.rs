//! VFIO device binding and unbinding
//!
//! This module provides functionality for binding and unbinding
//! PCI devices to/from the vfio-pci driver at runtime.

use super::errors::{VfioError, VfioResult};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Status of a VFIO binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfioBindingStatus {
    /// PCI address
    pub device: String,
    /// Whether bound to vfio-pci
    pub bound_to_vfio: bool,
    /// Original driver (if known)
    pub original_driver: Option<String>,
    /// IOMMU group
    pub iommu_group: u32,
    /// Vendor:device ID
    pub device_id: String,
    /// Human-readable description
    pub description: String,
}

/// Information about VFIO modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfioModuleStatus {
    /// vfio module loaded
    pub vfio_loaded: bool,
    /// vfio_pci module loaded
    pub vfio_pci_loaded: bool,
    /// vfio_iommu_type1 module loaded
    pub vfio_iommu_type1_loaded: bool,
    /// vfio_virqfd module loaded (optional, may be built-in)
    pub vfio_virqfd_loaded: bool,
}

/// Check if VFIO modules are loaded
pub fn check_vfio_modules() -> Result<VfioModuleStatus> {
    let output = Command::new("lsmod")
        .output()
        .context("Failed to run lsmod")?;

    let lsmod = String::from_utf8_lossy(&output.stdout);

    Ok(VfioModuleStatus {
        vfio_loaded: lsmod.contains("vfio ") || lsmod.contains("vfio\t"),
        vfio_pci_loaded: lsmod.contains("vfio_pci"),
        vfio_iommu_type1_loaded: lsmod.contains("vfio_iommu_type1"),
        vfio_virqfd_loaded: lsmod.contains("vfio_virqfd"),
    })
}

/// Load VFIO kernel modules
pub fn load_vfio_modules() -> Result<()> {
    let modules = ["vfio", "vfio_iommu_type1", "vfio_pci"];

    for module in &modules {
        let status = Command::new("modprobe")
            .arg(module)
            .status()
            .with_context(|| format!("Failed to load module {}", module))?;

        if !status.success() {
            return Err(
                VfioError::ModulesNotLoaded(format!("Failed to load module {}", module)).into(),
            );
        }
    }

    Ok(())
}

/// Bind a device to vfio-pci driver
pub fn bind_device(pci_address: &str) -> Result<()> {
    let address = normalize_pci_address(pci_address)?;

    // Check if device exists
    let device_path = format!("/sys/bus/pci/devices/{}", address);
    if !Path::new(&device_path).exists() {
        return Err(VfioError::DeviceNotFound(address.clone()).into());
    }

    // Get vendor:device ID
    let vendor = fs::read_to_string(format!("{}/vendor", device_path))
        .context("Failed to read vendor ID")?
        .trim()
        .trim_start_matches("0x")
        .to_string();
    let device = fs::read_to_string(format!("{}/device", device_path))
        .context("Failed to read device ID")?
        .trim()
        .trim_start_matches("0x")
        .to_string();
    let device_id = format!("{} {}", vendor, device);

    // Get current driver
    let driver_link = format!("{}/driver", device_path);
    let current_driver = if Path::new(&driver_link).exists() {
        fs::read_link(&driver_link)
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
    } else {
        None
    };

    // If already bound to vfio-pci, we're done
    if current_driver.as_deref() == Some("vfio-pci") {
        println!("Device {} is already bound to vfio-pci", address);
        return Ok(());
    }

    // Unbind from current driver if any
    if let Some(driver) = &current_driver {
        let unbind_path = format!("/sys/bus/pci/drivers/{}/unbind", driver);
        fs::write(&unbind_path, &address).with_context(|| {
            format!(
                "Failed to unbind {} from {}. Try running as root.",
                address, driver
            )
        })?;
        println!("Unbound {} from {}", address, driver);
    }

    // Set driver override
    let override_path = format!("{}/driver_override", device_path);
    fs::write(&override_path, "vfio-pci").with_context(|| {
        format!(
            "Failed to set driver override for {}. Try running as root.",
            address
        )
    })?;

    // Register with vfio-pci (add new_id)
    let new_id_path = "/sys/bus/pci/drivers/vfio-pci/new_id";
    if let Err(e) = fs::write(new_id_path, &device_id) {
        // This may fail if already registered, which is ok
        eprintln!(
            "Note: Could not add to new_id (may already be registered): {}",
            e
        );
    }

    // Bind to vfio-pci
    let bind_path = "/sys/bus/pci/drivers/vfio-pci/bind";
    fs::write(bind_path, &address).with_context(|| {
        format!(
            "Failed to bind {} to vfio-pci. Try running as root.",
            address
        )
    })?;

    println!("Successfully bound {} to vfio-pci", address);

    // Clear driver override
    let _ = fs::write(&override_path, "");

    Ok(())
}

/// Unbind a device from vfio-pci driver
pub fn unbind_device(pci_address: &str) -> Result<()> {
    let address = normalize_pci_address(pci_address)?;

    // Check if device exists
    let device_path = format!("/sys/bus/pci/devices/{}", address);
    if !Path::new(&device_path).exists() {
        return Err(VfioError::DeviceNotFound(address.clone()).into());
    }

    // Get current driver
    let driver_link = format!("{}/driver", device_path);
    let current_driver = if Path::new(&driver_link).exists() {
        fs::read_link(&driver_link)
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
    } else {
        None
    };

    // Unbind from vfio-pci
    if current_driver.as_deref() == Some("vfio-pci") {
        let unbind_path = "/sys/bus/pci/drivers/vfio-pci/unbind";
        fs::write(unbind_path, &address).with_context(|| {
            format!(
                "Failed to unbind {} from vfio-pci. Try running as root.",
                address
            )
        })?;
        println!("Unbound {} from vfio-pci", address);
    } else if let Some(driver) = current_driver {
        println!(
            "Device {} is not bound to vfio-pci (current: {})",
            address, driver
        );
        return Ok(());
    } else {
        println!("Device {} is not bound to any driver", address);
        return Ok(());
    }

    // Clear driver override
    let override_path = format!("{}/driver_override", device_path);
    let _ = fs::write(&override_path, "");

    // Trigger rescan to let kernel rebind to original driver
    let rescan_path = "/sys/bus/pci/rescan";
    fs::write(rescan_path, "1").context("Failed to trigger PCI rescan")?;
    println!("Triggered PCI rescan - device should rebind to original driver");

    Ok(())
}

/// Rebind device to its original driver
pub fn rebind_to_original(pci_address: &str, original_driver: Option<&str>) -> Result<()> {
    let address = normalize_pci_address(pci_address)?;

    // First unbind from vfio-pci
    unbind_device(&address)?;

    // If we know the original driver, explicitly bind to it
    if let Some(driver) = original_driver {
        let bind_path = format!("/sys/bus/pci/drivers/{}/bind", driver);
        if Path::new(&format!("/sys/bus/pci/drivers/{}", driver)).exists() {
            fs::write(&bind_path, &address)
                .with_context(|| format!("Failed to bind {} to {}", address, driver))?;
            println!("Bound {} to {}", address, driver);
        }
    }

    Ok(())
}

/// Get the current driver for a device
pub fn get_current_driver(pci_address: &str) -> Result<Option<String>> {
    let address = normalize_pci_address(pci_address)?;
    let driver_link = format!("/sys/bus/pci/devices/{}/driver", address);

    if Path::new(&driver_link).exists() {
        Ok(fs::read_link(&driver_link)
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string())))
    } else {
        Ok(None)
    }
}

/// Get VFIO binding status for all VFIO-bound devices
pub fn get_vfio_status() -> Result<Vec<VfioBindingStatus>> {
    let mut statuses = Vec::new();

    // Check /sys/bus/pci/drivers/vfio-pci for bound devices
    let vfio_driver_path = "/sys/bus/pci/drivers/vfio-pci";
    if !Path::new(vfio_driver_path).exists() {
        return Ok(statuses);
    }

    if let Ok(entries) = fs::read_dir(vfio_driver_path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let name = entry.file_name().to_string_lossy().to_string();

            // Skip non-device entries
            if !name.contains(':') {
                continue;
            }

            let address = name.clone();
            let device_path = format!("/sys/bus/pci/devices/{}", address);

            // Get device info
            let vendor = fs::read_to_string(format!("{}/vendor", device_path))
                .unwrap_or_default()
                .trim()
                .trim_start_matches("0x")
                .to_string();
            let device = fs::read_to_string(format!("{}/device", device_path))
                .unwrap_or_default()
                .trim()
                .trim_start_matches("0x")
                .to_string();

            // Get IOMMU group
            let iommu_group = fs::read_link(format!("{}/iommu_group", device_path))
                .ok()
                .and_then(|p| {
                    p.file_name()
                        .and_then(|n| n.to_str())
                        .and_then(|s| s.parse().ok())
                })
                .unwrap_or(0);

            // Get description from lspci
            let description = get_device_description(&address).unwrap_or_default();

            statuses.push(VfioBindingStatus {
                device: address,
                bound_to_vfio: true,
                original_driver: None, // We don't track this currently
                iommu_group,
                device_id: format!("{}:{}", vendor, device),
                description,
            });
        }
    }

    Ok(statuses)
}

/// Get device description from lspci
fn get_device_description(pci_address: &str) -> Result<String> {
    let short_addr = pci_address.trim_start_matches("0000:");

    let output = Command::new("lspci")
        .args(["-s", short_addr])
        .output()
        .context("Failed to run lspci")?;

    let line = String::from_utf8_lossy(&output.stdout);
    let line = line.trim();

    // Parse: "01:00.0 VGA compatible controller: NVIDIA Corporation..."
    if let Some(pos) = line.find(": ") {
        Ok(line[pos + 2..].to_string())
    } else {
        Ok(line.to_string())
    }
}

/// Normalize PCI address to full format (0000:xx:yy.z)
fn normalize_pci_address(address: &str) -> VfioResult<String> {
    let address = address.trim();

    // Already full format
    if address.len() == 12 && address.chars().filter(|c| *c == ':').count() == 2 {
        return Ok(address.to_string());
    }

    // Short format (xx:yy.z)
    if address.len() == 7 && address.chars().filter(|c| *c == ':').count() == 1 {
        return Ok(format!("0000:{}", address));
    }

    Err(VfioError::InvalidPciAddress(address.to_string()))
}

/// Print VFIO status to stdout
pub fn print_status(json_output: bool) {
    println!("\nVFIO Status");
    println!("{}", "=".repeat(50));

    // Check modules
    match check_vfio_modules() {
        Ok(modules) => {
            println!("\nVFIO Modules:");
            println!(
                "  vfio:            {}",
                if modules.vfio_loaded {
                    "Loaded"
                } else {
                    "Not loaded"
                }
            );
            println!(
                "  vfio_pci:        {}",
                if modules.vfio_pci_loaded {
                    "Loaded"
                } else {
                    "Not loaded"
                }
            );
            println!(
                "  vfio_iommu_type1: {}",
                if modules.vfio_iommu_type1_loaded {
                    "Loaded"
                } else {
                    "Not loaded"
                }
            );
        }
        Err(e) => {
            eprintln!("Error checking modules: {}", e);
        }
    }

    // Check bound devices
    match get_vfio_status() {
        Ok(statuses) => {
            if json_output {
                if let Ok(json) = serde_json::to_string_pretty(&statuses) {
                    println!("\n{}", json);
                }
            } else {
                println!("\nVFIO-Bound Devices:");
                if statuses.is_empty() {
                    println!("  No devices currently bound to vfio-pci");
                } else {
                    for status in &statuses {
                        println!(
                            "  {} [{}] {} (Group {})",
                            status.device, status.device_id, status.description, status.iommu_group
                        );
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error checking VFIO status: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
