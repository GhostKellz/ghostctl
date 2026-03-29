//! ACS (Access Control Services) override detection
//!
//! This module detects whether the kernel has ACS override patches
//! and whether ACS override is enabled, which is necessary for
//! breaking up IOMMU groups on some systems.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

/// ACS override status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcsStatus {
    /// Whether the kernel appears to have ACS patches
    pub kernel_patched: bool,
    /// Whether ACS override is currently enabled in kernel params
    pub acs_override_enabled: bool,
    /// The ACS override mode if enabled
    pub override_mode: Option<String>,
    /// Devices that might benefit from ACS override
    pub affected_devices: Vec<String>,
    /// Recommendation based on current status
    pub recommendation: String,
}

/// Check ACS override status
pub fn check_acs_status() -> Result<AcsStatus> {
    let kernel_params = fs::read_to_string("/proc/cmdline")
        .context("Failed to read kernel command line")?
        .trim()
        .to_string();

    // Check if ACS override is in kernel params
    let acs_override_enabled = kernel_params.contains("pcie_acs_override=");
    let override_mode = if acs_override_enabled {
        // Extract the mode (downstream, multifunction, id:xxxx:xxxx, etc.)
        kernel_params
            .split_whitespace()
            .find(|p| p.starts_with("pcie_acs_override="))
            .map(|p| p.trim_start_matches("pcie_acs_override=").to_string())
    } else {
        None
    };

    // Check if kernel has ACS override patch
    let kernel_patched = check_kernel_acs_patch();

    // Find devices that might benefit from ACS override
    let affected_devices = find_acs_affected_devices().unwrap_or_default();

    let recommendation = generate_acs_recommendation(
        kernel_patched,
        acs_override_enabled,
        &override_mode,
        &affected_devices,
    );

    Ok(AcsStatus {
        kernel_patched,
        acs_override_enabled,
        override_mode,
        affected_devices,
        recommendation,
    })
}

/// Check if the kernel has ACS override patch
fn check_kernel_acs_patch() -> bool {
    // Check kernel config for ACS override option
    let uname = Command::new("uname").arg("-r").output().ok();

    if let Some(output) = uname {
        let kernel_version = String::from_utf8_lossy(&output.stdout).trim().to_string();

        // Check kernel config
        let config_paths = [
            format!("/boot/config-{}", kernel_version),
            "/proc/config.gz".to_string(),
            format!("/lib/modules/{}/build/.config", kernel_version),
        ];

        for path in &config_paths {
            if let Ok(content) = fs::read_to_string(path) {
                // Look for ACS override config option
                if content.contains("CONFIG_PCI_QUIRKS=y") || content.contains("PCIE_ACS_OVERRIDE")
                {
                    return true;
                }
            }
        }

        // Check if it's a known patched kernel
        let patched_kernels = [
            "linux-vfio",
            "linux-zen",
            "linux-lqx",
            "linux-xanmod",
            "linux-tkg",
            "linux-cachyos",
        ];

        for patched in &patched_kernels {
            if kernel_version.to_lowercase().contains(patched) {
                return true;
            }
        }

        // Check dmesg for ACS override messages
        if let Ok(dmesg_output) = Command::new("dmesg").output() {
            let dmesg = String::from_utf8_lossy(&dmesg_output.stdout);
            if dmesg.contains("pcie_acs_override") || dmesg.contains("ACS override") {
                return true;
            }
        }
    }

    false
}

/// Find devices that might benefit from ACS override
fn find_acs_affected_devices() -> Result<Vec<String>> {
    let mut affected = Vec::new();

    // Look for PCIe bridges without proper ACS support
    let output = Command::new("lspci")
        .args(["-vvv"])
        .output()
        .context("Failed to run lspci")?;

    let lspci_output = String::from_utf8_lossy(&output.stdout);
    let mut current_device = String::new();
    let mut in_caps_section = false;

    for line in lspci_output.lines() {
        if !line.starts_with('\t') && !line.starts_with(' ') {
            // New device
            current_device = line.split_whitespace().next().unwrap_or("").to_string();
            in_caps_section = false;
        } else if line.contains("Capabilities:") {
            in_caps_section = true;
        } else if in_caps_section && line.contains("ACSCap:") {
            // Found ACS capabilities
            if line.contains("SrcValid-") || line.contains("ReqRedir-") {
                affected.push(format!(
                    "{} - Limited ACS support (may need override)",
                    current_device
                ));
            }
        }
    }

    Ok(affected)
}

/// Generate recommendation based on ACS status
fn generate_acs_recommendation(
    kernel_patched: bool,
    acs_override_enabled: bool,
    override_mode: &Option<String>,
    affected_devices: &[String],
) -> String {
    if acs_override_enabled {
        let mode = override_mode
            .as_ref()
            .map(|m| m.as_str())
            .unwrap_or("unknown");
        format!(
            "ACS override is enabled with mode '{}'. \
             This allows breaking up IOMMU groups but may reduce security isolation.",
            mode
        )
    } else if !kernel_patched {
        "Your kernel does not appear to have ACS override patches. \
         If you need to break up IOMMU groups, consider using a patched kernel \
         like linux-zen, linux-vfio, linux-cachyos, or linux-xanmod."
            .to_string()
    } else if affected_devices.is_empty() {
        "Your kernel supports ACS override but it's not currently enabled. \
         Your IOMMU groups appear to be well-isolated already."
            .to_string()
    } else {
        format!(
            "Your kernel supports ACS override. {} devices may benefit from it. \
             Add 'pcie_acs_override=downstream,multifunction' to kernel params if needed.",
            affected_devices.len()
        )
    }
}

/// Print ACS status to stdout
pub fn check_and_print_acs_status() {
    println!("\nACS Override Status");
    println!("{}", "=".repeat(50));

    match check_acs_status() {
        Ok(status) => {
            println!(
                "Kernel Patched:      {}",
                if status.kernel_patched { "Yes" } else { "No" }
            );
            println!(
                "ACS Override:        {}",
                if status.acs_override_enabled {
                    "Enabled"
                } else {
                    "Disabled"
                }
            );

            if let Some(mode) = &status.override_mode {
                println!("Override Mode:       {}", mode);
            }

            if !status.affected_devices.is_empty() {
                println!("\nDevices with Limited ACS:");
                for device in &status.affected_devices {
                    println!("  {}", device);
                }
            }

            println!("\nRecommendation:");
            println!("  {}", status.recommendation);

            if !status.acs_override_enabled && status.kernel_patched {
                println!("\nTo enable ACS override, add to kernel parameters:");
                println!("  pcie_acs_override=downstream,multifunction");
                println!("\nAvailable override modes:");
                println!("  downstream    - Override on downstream ports");
                println!("  multifunction - Override on multifunction devices");
                println!("  id:xxxx:xxxx  - Override on specific device ID");
            }
        }
        Err(e) => {
            eprintln!("Error checking ACS status: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_recommendation_enabled() {
        let recommendation = generate_acs_recommendation(
            true,
            true,
            &Some("downstream,multifunction".to_string()),
            &[],
        );
        assert!(recommendation.contains("enabled"));
    }

    #[test]
    fn test_generate_recommendation_not_patched() {
        let recommendation = generate_acs_recommendation(false, false, &None, &[]);
        assert!(recommendation.contains("does not appear to have ACS override patches"));
    }

    #[test]
    fn test_generate_recommendation_patched_not_needed() {
        let recommendation = generate_acs_recommendation(true, false, &None, &[]);
        assert!(recommendation.contains("well-isolated"));
    }
}
