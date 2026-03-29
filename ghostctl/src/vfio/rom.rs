//! VBIOS ROM dumping for GPU passthrough
//!
//! This module provides functionality for dumping GPU VBIOS ROM files
//! which may be needed for GPU passthrough in some scenarios.
//!
//! # Use Cases
//!
//! - NVIDIA cards that need ROM file for passthrough
//! - Troubleshooting passthrough issues
//! - Backup of original VBIOS

use super::errors::VfioError;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

/// ROM dump result
#[derive(Debug)]
pub struct RomDumpResult {
    /// PCI address of the device
    pub pci_address: String,
    /// Path where ROM was saved
    pub output_path: String,
    /// Size of the ROM in bytes
    pub size: usize,
    /// Whether the dump was successful
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
}

/// Check if ROM is accessible for a device
pub fn check_rom_accessible(pci_address: &str) -> Result<bool> {
    let address = normalize_address(pci_address);
    let rom_path = format!("/sys/bus/pci/devices/{}/rom", address);

    Ok(Path::new(&rom_path).exists())
}

/// Enable ROM reading for a device
pub fn enable_rom(pci_address: &str) -> Result<()> {
    let address = normalize_address(pci_address);
    let rom_path = format!("/sys/bus/pci/devices/{}/rom", address);

    if !Path::new(&rom_path).exists() {
        return Err(VfioError::RomDumpError {
            device: pci_address.to_string(),
            reason: "ROM file not found in sysfs".to_string(),
        }
        .into());
    }

    // Enable ROM by writing 1
    fs::write(&rom_path, "1").with_context(|| {
        format!(
            "Failed to enable ROM for {}. Try running as root.",
            pci_address
        )
    })?;

    Ok(())
}

/// Disable ROM reading for a device
pub fn disable_rom(pci_address: &str) -> Result<()> {
    let address = normalize_address(pci_address);
    let rom_path = format!("/sys/bus/pci/devices/{}/rom", address);

    if Path::new(&rom_path).exists() {
        // Disable ROM by writing 0
        let _ = fs::write(&rom_path, "0");
    }

    Ok(())
}

/// Dump VBIOS ROM to a file
pub fn dump_rom(pci_address: &str, output_path: Option<&str>) -> Result<RomDumpResult> {
    let address = normalize_address(pci_address);
    let short_addr = address.trim_start_matches("0000:");

    // Default output path
    let output = output_path
        .map(|p| p.to_string())
        .unwrap_or_else(|| format!("gpu_{}.rom", short_addr.replace(':', "_").replace('.', "_")));

    // Check if ROM exists
    let rom_path = format!("/sys/bus/pci/devices/{}/rom", address);
    if !Path::new(&rom_path).exists() {
        return Ok(RomDumpResult {
            pci_address: pci_address.to_string(),
            output_path: output,
            size: 0,
            success: false,
            error: Some(
                "ROM file not found in sysfs. Device may not support ROM access.".to_string(),
            ),
        });
    }

    // Enable ROM
    if let Err(e) = enable_rom(&address) {
        return Ok(RomDumpResult {
            pci_address: pci_address.to_string(),
            output_path: output,
            size: 0,
            success: false,
            error: Some(format!("Failed to enable ROM: {}", e)),
        });
    }

    // Read ROM
    let rom_data = match fs::read(&rom_path) {
        Ok(data) => data,
        Err(e) => {
            let _ = disable_rom(&address);
            return Ok(RomDumpResult {
                pci_address: pci_address.to_string(),
                output_path: output,
                size: 0,
                success: false,
                error: Some(format!("Failed to read ROM: {}", e)),
            });
        }
    };

    // Disable ROM
    let _ = disable_rom(&address);

    // Check if ROM data is valid (should start with 0x55 0xAA for x86 option ROMs)
    if rom_data.len() < 2 || rom_data[0] != 0x55 || rom_data[1] != 0xAA {
        return Ok(RomDumpResult {
            pci_address: pci_address.to_string(),
            output_path: output,
            size: rom_data.len(),
            success: false,
            error: Some("ROM data appears invalid (missing 0x55AA signature)".to_string()),
        });
    }

    // Write to output file
    let size = rom_data.len();
    fs::write(&output, &rom_data).with_context(|| format!("Failed to write ROM to {}", output))?;

    Ok(RomDumpResult {
        pci_address: pci_address.to_string(),
        output_path: output,
        size,
        success: true,
        error: None,
    })
}

/// Dump ROM using nvidia-smi (for NVIDIA cards)
pub fn dump_rom_nvidia(pci_address: &str, output_path: Option<&str>) -> Result<RomDumpResult> {
    let short_addr = pci_address.trim_start_matches("0000:").to_uppercase();

    let output = output_path
        .map(|p| p.to_string())
        .unwrap_or_else(|| format!("gpu_{}.rom", short_addr.replace(':', "_").replace('.', "_")));

    // Check if nvidia-smi exists
    let nvidia_smi = Command::new("which")
        .arg("nvidia-smi")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !nvidia_smi {
        return Ok(RomDumpResult {
            pci_address: pci_address.to_string(),
            output_path: output,
            size: 0,
            success: false,
            error: Some("nvidia-smi not found".to_string()),
        });
    }

    // Try to use nvidia-smi to get VBIOS version (doesn't dump, but verifies access)
    let status = Command::new("nvidia-smi")
        .args([
            "--query-gpu=vbios_version",
            "--format=csv,noheader",
            "-i",
            &short_addr,
        ])
        .output();

    match status {
        Ok(output_cmd) if output_cmd.status.success() => {
            let vbios_version = String::from_utf8_lossy(&output_cmd.stdout);
            println!("VBIOS Version: {}", vbios_version.trim());

            // Fall back to sysfs method for actual dump
            dump_rom(pci_address, Some(&output))
        }
        _ => {
            // Fall back to sysfs method
            dump_rom(pci_address, Some(&output))
        }
    }
}

/// List devices that may have dumpable ROMs
pub fn list_rom_devices() -> Result<Vec<RomDeviceInfo>> {
    let mut devices = Vec::new();

    // Get all GPUs
    if let Ok(gpus) = crate::iommu::list_gpus() {
        for gpu in gpus {
            let address = &gpu.address;
            let rom_path = format!("/sys/bus/pci/devices/{}/rom", address);
            let has_rom = Path::new(&rom_path).exists();

            devices.push(RomDeviceInfo {
                pci_address: address.clone(),
                description: gpu.description.clone(),
                vendor_id: gpu.vendor_id.clone(),
                device_id: gpu.device_id.clone(),
                has_rom,
                current_driver: gpu.current_driver.clone(),
            });
        }
    }

    Ok(devices)
}

/// Information about a device that may have a ROM
#[derive(Debug)]
pub struct RomDeviceInfo {
    /// PCI address
    pub pci_address: String,
    /// Device description
    pub description: String,
    /// Vendor ID
    pub vendor_id: String,
    /// Device ID
    pub device_id: String,
    /// Whether ROM file exists in sysfs
    pub has_rom: bool,
    /// Current driver
    pub current_driver: Option<String>,
}

/// Print ROM device information
pub fn print_rom_devices() {
    println!("\nGPU ROM Information");
    println!("{}", "=".repeat(50));

    match list_rom_devices() {
        Ok(devices) => {
            if devices.is_empty() {
                println!("No GPUs found.");
                return;
            }

            for device in devices {
                let driver = device
                    .current_driver
                    .as_ref()
                    .map(|d| format!(" [{}]", d))
                    .unwrap_or_default();

                let rom_status = if device.has_rom {
                    "ROM available"
                } else {
                    "No ROM access"
                };

                println!(
                    "\n{} - {}{}",
                    device.pci_address, device.description, driver
                );
                println!("  Vendor:Device: {}:{}", device.vendor_id, device.device_id);
                println!("  ROM Status:    {}", rom_status);
            }
        }
        Err(e) => {
            eprintln!("Error listing devices: {}", e);
        }
    }
}

/// Normalize PCI address to full format
fn normalize_address(address: &str) -> String {
    let address = address.trim();

    // Already full format
    if address.starts_with("0000:") {
        return address.to_string();
    }

    // Short format
    format!("0000:{}", address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_address() {
        assert_eq!(normalize_address("01:00.0"), "0000:01:00.0");
        assert_eq!(normalize_address("0000:01:00.0"), "0000:01:00.0");
    }
}
