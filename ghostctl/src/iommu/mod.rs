//! IOMMU group management and analysis for ghostctl
//!
//! This module provides comprehensive IOMMU group detection, analysis, and
//! visualization capabilities for GPU passthrough and virtualization.
//!
//! # Features
//!
//! - IOMMU status detection (Intel VT-d / AMD-Vi)
//! - IOMMU group enumeration and device listing
//! - Passthrough isolation scoring
//! - Device filtering (GPUs, USB, NVMe, SATA controllers)
//! - JSON output support for scripting
//!
//! # Example
//!
//! ```no_run
//! use ghostctl::iommu;
//!
//! // Check IOMMU status
//! let status = iommu::get_iommu_status().unwrap();
//! println!("IOMMU enabled: {}", status.enabled);
//!
//! // List all groups
//! let groups = iommu::list_iommu_groups().unwrap();
//! for group in groups {
//!     println!("Group {}: {} devices", group.id, group.devices.len());
//! }
//! ```

pub mod acs;
pub mod analysis;
pub mod errors;
pub mod groups;
pub mod pcie;

// Re-export main types for convenience (public API)
#[allow(unused_imports)]
pub use errors::{IommuError, IommuResult};
#[allow(unused_imports)]
pub use groups::{
    IommuGroup, IommuMode, IommuStatus, PciDevice, PciDeviceClass, find_device_group,
    get_iommu_group, get_iommu_status, list_gpus, list_groups_cli, list_iommu_groups,
    list_nvme_controllers, list_sata_controllers, list_usb_controllers, print_groups,
    print_groups_json,
};

use anyhow::Result;
use dialoguer::{Select, theme::ColorfulTheme};

/// Interactive IOMMU management menu
pub fn iommu_menu() {
    loop {
        let options = vec![
            "Show IOMMU Status",
            "List All IOMMU Groups",
            "List GPU Groups Only",
            "List USB Controllers",
            "List NVMe Controllers",
            "List SATA Controllers",
            "Analyze Device",
            "Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("IOMMU Management")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(sel)) => sel,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => show_iommu_status(),
            1 => show_all_groups(),
            2 => show_gpu_groups(),
            3 => show_usb_controllers(),
            4 => show_nvme_controllers(),
            5 => show_sata_controllers(),
            6 => analyze_device_interactive(),
            _ => break,
        }

        println!(); // Add spacing after each operation
    }
}

/// Display IOMMU status
fn show_iommu_status() {
    println!("\nIOMMU Status");
    println!("{}", "=".repeat(50));

    match get_iommu_status() {
        Ok(status) => {
            let enabled_icon = if status.enabled { "ON" } else { "OFF" };
            let pt_icon = if status.passthrough_pt { "ON" } else { "OFF" };
            let irq_icon = if status.interrupt_remapping {
                "ON"
            } else {
                "OFF"
            };

            println!("IOMMU Enabled:        {}", enabled_icon);
            println!("Mode:                 {}", status.mode);
            println!("IOMMU Groups:         {}", status.group_count);
            println!("Passthrough (iommu=pt): {}", pt_icon);
            println!("Interrupt Remapping:  {}", irq_icon);

            println!("\nKernel Parameters:");
            println!("  {}", status.kernel_params);

            if !status.enabled {
                println!("\nTo enable IOMMU, add the following to your kernel parameters:");
                println!("  Intel: intel_iommu=on iommu=pt");
                println!("  AMD:   amd_iommu=on iommu=pt");
            }

            if status.enabled && !status.passthrough_pt {
                println!("\nRecommendation: Add 'iommu=pt' for better performance");
            }

            if status.enabled && !status.interrupt_remapping {
                println!(
                    "\nWarning: Interrupt remapping not detected - may cause issues with passthrough"
                );
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to get IOMMU status: {}", e);
        }
    }
}

/// Display all IOMMU groups
fn show_all_groups() {
    println!("\nAll IOMMU Groups");
    println!("{}", "=".repeat(50));

    match list_iommu_groups() {
        Ok(groups) => {
            print_groups(&groups, false);

            // Summary
            let total_devices: usize = groups.iter().map(|g| g.devices.len()).sum();
            let gpu_count = groups
                .iter()
                .flat_map(|g| &g.devices)
                .filter(|d| d.is_gpu())
                .count();
            let isolated_count = groups.iter().filter(|g| g.is_isolated).count();

            println!("\nSummary:");
            println!("  Total groups: {}", groups.len());
            println!("  Total devices: {}", total_devices);
            println!("  GPUs found: {}", gpu_count);
            println!(
                "  Well-isolated groups: {}/{}",
                isolated_count,
                groups.len()
            );
        }
        Err(e) => {
            eprintln!("Error: Failed to list IOMMU groups: {}", e);
        }
    }
}

/// Display GPU-containing groups only
fn show_gpu_groups() {
    println!("\nGPU IOMMU Groups");
    println!("{}", "=".repeat(50));

    match list_iommu_groups() {
        Ok(groups) => {
            let gpu_groups: Vec<_> = groups.iter().filter(|g| g.has_gpu()).collect();

            if gpu_groups.is_empty() {
                println!("No GPU-containing IOMMU groups found.");
                return;
            }

            for group in &gpu_groups {
                let viability = if group.isolation_score >= 90 {
                    "Excellent for passthrough"
                } else if group.isolation_score >= 70 {
                    "Good for passthrough"
                } else if group.isolation_score >= 50 {
                    "May work with workarounds"
                } else {
                    "Poor isolation - may need ACS override"
                };

                println!(
                    "\nGroup {} - {} (Score: {}/100)",
                    group.id, viability, group.isolation_score
                );
                println!("{}", "-".repeat(40));

                for device in &group.devices {
                    let driver = device
                        .current_driver
                        .as_ref()
                        .map(|d| format!("[{}]", d))
                        .unwrap_or_default();

                    let device_type = if device.is_gpu() {
                        "GPU"
                    } else if device.class.is_bridge() {
                        "Bridge"
                    } else {
                        device.class.description()
                    };

                    println!(
                        "  {} {} {} {}",
                        device.address, device_type, device.description, driver
                    );
                }

                if !group.warnings.is_empty() {
                    println!("\n  Warnings:");
                    for warning in &group.warnings {
                        println!("    - {}", warning);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to list IOMMU groups: {}", e);
        }
    }
}

/// Display USB controllers
fn show_usb_controllers() {
    println!("\nUSB Controllers");
    println!("{}", "=".repeat(50));

    match list_usb_controllers() {
        Ok(controllers) => {
            if controllers.is_empty() {
                println!("No USB controllers found.");
                return;
            }

            for controller in controllers {
                let driver = controller
                    .current_driver
                    .as_ref()
                    .map(|d| format!("[{}]", d))
                    .unwrap_or_default();

                println!(
                    "{} - {} {} (Group {})",
                    controller.address, controller.description, driver, controller.iommu_group
                );
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to list USB controllers: {}", e);
        }
    }
}

/// Display NVMe controllers
fn show_nvme_controllers() {
    println!("\nNVMe Controllers");
    println!("{}", "=".repeat(50));

    match list_nvme_controllers() {
        Ok(controllers) => {
            if controllers.is_empty() {
                println!("No NVMe controllers found.");
                return;
            }

            for controller in controllers {
                let driver = controller
                    .current_driver
                    .as_ref()
                    .map(|d| format!("[{}]", d))
                    .unwrap_or_default();

                println!(
                    "{} - {} {} (Group {})",
                    controller.address, controller.description, driver, controller.iommu_group
                );
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to list NVMe controllers: {}", e);
        }
    }
}

/// Display SATA controllers
fn show_sata_controllers() {
    println!("\nSATA Controllers");
    println!("{}", "=".repeat(50));

    match list_sata_controllers() {
        Ok(controllers) => {
            if controllers.is_empty() {
                println!("No SATA controllers found.");
                return;
            }

            for controller in controllers {
                let driver = controller
                    .current_driver
                    .as_ref()
                    .map(|d| format!("[{}]", d))
                    .unwrap_or_default();

                println!(
                    "{} - {} {} (Group {})",
                    controller.address, controller.description, driver, controller.iommu_group
                );
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to list SATA controllers: {}", e);
        }
    }
}

/// Interactive device analysis
fn analyze_device_interactive() {
    use dialoguer::Input;

    let address: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter PCI address (e.g., 01:00.0 or 0000:01:00.0)")
        .interact_text()
    {
        Ok(addr) => addr,
        Err(_) => return,
    };

    analyze_device(&address);
}

/// Analyze a device for passthrough viability (public API)
pub fn analyze_device(pci_address: &str) {
    println!("\nDevice Analysis: {}", pci_address);
    println!("{}", "=".repeat(50));

    match find_device_group(pci_address) {
        Ok(group) => {
            // Find the specific device
            let device = group
                .devices
                .iter()
                .find(|d| d.address.ends_with(pci_address) || d.address == pci_address);

            if let Some(device) = device {
                println!("\nDevice Information:");
                println!("  Address:     {}", device.address);
                println!("  Type:        {}", device.class.description());
                println!(
                    "  Vendor:      {} ({})",
                    device.vendor_name.as_deref().unwrap_or("Unknown"),
                    device.vendor_id
                );
                println!("  Device ID:   {}", device.device_id);
                println!(
                    "  Driver:      {}",
                    device.current_driver.as_deref().unwrap_or("none")
                );
                println!("  IOMMU Group: {}", device.iommu_group);
            }

            println!("\nIOMMU Group {} Analysis:", group.id);
            println!("  Devices in group: {}", group.devices.len());
            println!("  Isolation score:  {}/100", group.isolation_score);
            println!(
                "  Well isolated:    {}",
                if group.is_isolated { "Yes" } else { "No" }
            );

            // Viability assessment
            let viability = if group.isolation_score >= 90 {
                (
                    "Excellent",
                    "This device is in a clean IOMMU group and should pass through without issues.",
                )
            } else if group.isolation_score >= 70 {
                (
                    "Good",
                    "This device should work well for passthrough with minor considerations.",
                )
            } else if group.isolation_score >= 50 {
                (
                    "Moderate",
                    "Passthrough may work but could require workarounds or ACS override.",
                )
            } else if group.isolation_score >= 30 {
                (
                    "Poor",
                    "Passthrough will likely have issues. ACS override patch recommended.",
                )
            } else {
                (
                    "Not Viable",
                    "This device cannot be passed through safely without major changes.",
                )
            };

            println!("\nPassthrough Viability: {}", viability.0);
            println!("  {}", viability.1);

            if !group.warnings.is_empty() {
                println!("\nWarnings:");
                for warning in &group.warnings {
                    println!("  - {}", warning);
                }
            }

            // Show other devices in the group
            if group.devices.len() > 1 {
                println!("\nOther devices in this group:");
                for other in &group.devices {
                    if Some(other) != device {
                        let driver = other
                            .current_driver
                            .as_ref()
                            .map(|d| format!("[{}]", d))
                            .unwrap_or_default();
                        println!("  {} {} {}", other.address, other.description, driver);
                    }
                }
            }

            // Recommendations
            println!("\nRecommendations:");
            if group.isolation_score >= 70 {
                println!("  1. Ensure vfio-pci driver is available");
                println!("  2. Add device IDs to VFIO configuration");
                if group.devices.len() > 1 && group.same_slot() {
                    println!("  3. Include all devices on the same slot in passthrough");
                }
            } else {
                println!("  1. Consider using ACS override patch");
                println!("  2. Check if your motherboard/CPU supports better IOMMU isolation");
                println!("  3. Try a different PCIe slot if available");
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to analyze device: {}", e);
            eprintln!("\nHint: Make sure IOMMU is enabled and the device address is correct.");
            eprintln!("      Use 'ghostctl iommu groups' to see available devices.");
        }
    }
}

/// Get device info by PCI address (public API)
pub fn get_device_info(pci_address: &str) -> Result<PciDevice> {
    let group = find_device_group(pci_address)?;
    group
        .devices
        .into_iter()
        .find(|d| d.address.ends_with(pci_address) || d.address == pci_address)
        .ok_or_else(|| IommuError::DeviceNotFound(pci_address.to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify that main types are accessible
        let _: fn() -> IommuResult<()> = || Ok(());
    }
}
