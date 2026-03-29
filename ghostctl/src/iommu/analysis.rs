//! Passthrough viability analysis
//!
//! This module provides detailed analysis of devices for GPU/PCI passthrough,
//! including isolation scoring, issue detection, and recommendations.

use super::groups::{IommuGroup, PciDevice, PciDeviceClass};
use serde::{Deserialize, Serialize};

/// Passthrough viability level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PassthroughViability {
    /// 90-100: Clean isolation, no issues expected
    Excellent,
    /// 70-89: Minor issues, should work with standard setup
    Good,
    /// 50-69: Some concerns, may need workarounds
    Moderate,
    /// 30-49: Significant issues, likely problems
    Poor,
    /// 0-29: Critical issues, won't work without major changes
    NotViable,
}

impl PassthroughViability {
    /// Create from isolation score
    pub fn from_score(score: u8) -> Self {
        match score {
            90..=100 => PassthroughViability::Excellent,
            70..=89 => PassthroughViability::Good,
            50..=69 => PassthroughViability::Moderate,
            30..=49 => PassthroughViability::Poor,
            _ => PassthroughViability::NotViable,
        }
    }

    /// Get a description of this viability level
    pub fn description(&self) -> &str {
        match self {
            PassthroughViability::Excellent => {
                "This device is in a clean IOMMU group and should pass through without issues."
            }
            PassthroughViability::Good => {
                "This device should work well for passthrough with minor considerations."
            }
            PassthroughViability::Moderate => {
                "Passthrough may work but could require workarounds or ACS override."
            }
            PassthroughViability::Poor => {
                "Passthrough will likely have issues. ACS override patch recommended."
            }
            PassthroughViability::NotViable => {
                "This device cannot be passed through safely without major changes."
            }
        }
    }
}

impl std::fmt::Display for PassthroughViability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PassthroughViability::Excellent => write!(f, "Excellent"),
            PassthroughViability::Good => write!(f, "Good"),
            PassthroughViability::Moderate => write!(f, "Moderate"),
            PassthroughViability::Poor => write!(f, "Poor"),
            PassthroughViability::NotViable => write!(f, "Not Viable"),
        }
    }
}

/// Issue severity level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IssueSeverity {
    /// Critical - will prevent passthrough
    Critical,
    /// Warning - may cause issues
    Warning,
    /// Info - good to know but not a problem
    Info,
}

impl std::fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueSeverity::Critical => write!(f, "CRITICAL"),
            IssueSeverity::Warning => write!(f, "WARNING"),
            IssueSeverity::Info => write!(f, "INFO"),
        }
    }
}

/// A specific issue detected during analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassthroughIssue {
    /// Severity of the issue
    pub severity: IssueSeverity,
    /// Short code for the issue
    pub code: String,
    /// Human-readable description
    pub message: String,
    /// Suggested fix if available
    pub fix: Option<String>,
}

/// Complete passthrough analysis for a device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassthroughAnalysis {
    /// The device being analyzed
    pub device: PciDevice,
    /// The IOMMU group containing the device
    pub group: IommuGroup,
    /// Overall viability assessment
    pub viability: PassthroughViability,
    /// Isolation score (0-100)
    pub isolation_score: u8,
    /// Detected issues
    pub issues: Vec<PassthroughIssue>,
    /// Recommendations for successful passthrough
    pub recommendations: Vec<String>,
    /// Required kernel parameters
    pub required_kernel_params: Vec<String>,
    /// Device IDs to bind to VFIO (vendor:device format)
    pub vfio_device_ids: Vec<String>,
}

/// Analyze a device for passthrough viability
pub fn analyze_device(device: &PciDevice, group: &IommuGroup) -> PassthroughAnalysis {
    let mut issues = Vec::new();
    let mut recommendations = Vec::new();
    let mut required_kernel_params = Vec::new();
    let mut vfio_device_ids = Vec::new();

    // Collect VFIO device IDs
    for dev in &group.devices {
        vfio_device_ids.push(format!("{}:{}", dev.vendor_id, dev.device_id));
    }

    // Check for isolation issues
    let non_bridge_count = group
        .devices
        .iter()
        .filter(|d| !d.class.is_bridge())
        .count();

    if non_bridge_count == 1 {
        // Perfect isolation
        issues.push(PassthroughIssue {
            severity: IssueSeverity::Info,
            code: "ISOLATED".to_string(),
            message: "Device is alone in its IOMMU group (optimal)".to_string(),
            fix: None,
        });
    } else if group.same_slot() {
        // Multiple devices but same slot (common for GPU + audio)
        issues.push(PassthroughIssue {
            severity: IssueSeverity::Info,
            code: "SAME_SLOT".to_string(),
            message: format!(
                "{} devices on the same slot will be passed through together",
                group.devices.len()
            ),
            fix: None,
        });
        recommendations
            .push("All devices on this slot should be passed through together".to_string());
    } else {
        // Multiple devices from different slots
        issues.push(PassthroughIssue {
            severity: IssueSeverity::Warning,
            code: "MIXED_GROUP".to_string(),
            message: format!(
                "Group contains {} devices from different slots",
                non_bridge_count
            ),
            fix: Some("Consider using ACS override patch to break up the group".to_string()),
        });
        recommendations.push("Install linux-zen or linux-vfio kernel for ACS override".to_string());
        required_kernel_params.push("pcie_acs_override=downstream,multifunction".to_string());
    }

    // Check for problematic device combinations
    let has_usb = group
        .devices
        .iter()
        .any(|d| matches!(d.class, PciDeviceClass::UsbController));
    let has_sata = group
        .devices
        .iter()
        .any(|d| matches!(d.class, PciDeviceClass::SataController));
    let has_nvme = group
        .devices
        .iter()
        .any(|d| matches!(d.class, PciDeviceClass::NvmeController));

    if device.is_gpu() {
        if has_usb {
            issues.push(PassthroughIssue {
                severity: IssueSeverity::Warning,
                code: "GPU_USB_SHARED".to_string(),
                message: "GPU shares IOMMU group with USB controller".to_string(),
                fix: Some(
                    "USB controller will be passed through with GPU, or use ACS override"
                        .to_string(),
                ),
            });
        }

        if has_sata {
            issues.push(PassthroughIssue {
                severity: IssueSeverity::Critical,
                code: "GPU_SATA_SHARED".to_string(),
                message: "GPU shares IOMMU group with SATA controller".to_string(),
                fix: Some(
                    "May lose access to SATA drives when VM is running. Use ACS override."
                        .to_string(),
                ),
            });
        }

        if has_nvme {
            issues.push(PassthroughIssue {
                severity: IssueSeverity::Warning,
                code: "GPU_NVME_SHARED".to_string(),
                message: "GPU shares IOMMU group with NVMe controller".to_string(),
                fix: Some(
                    "NVMe drive will be passed through with GPU, or use ACS override".to_string(),
                ),
            });
        }
    }

    // Check driver status
    if let Some(driver) = &device.current_driver {
        if driver == "vfio-pci" {
            issues.push(PassthroughIssue {
                severity: IssueSeverity::Info,
                code: "VFIO_BOUND".to_string(),
                message: "Device is already bound to vfio-pci driver".to_string(),
                fix: None,
            });
        } else if device.is_gpu()
            && (driver == "nvidia"
                || driver == "amdgpu"
                || driver == "nouveau"
                || driver == "radeon")
        {
            issues.push(PassthroughIssue {
                severity: IssueSeverity::Info,
                code: "GPU_DRIVER".to_string(),
                message: format!("GPU is currently bound to {} driver", driver),
                fix: Some("Bind to vfio-pci for passthrough".to_string()),
            });
            recommendations.push(format!(
                "Add 'vfio-pci.ids={}' to kernel parameters or use modprobe.d",
                vfio_device_ids.join(",")
            ));
        }
    }

    // Add GPU-specific recommendations
    if device.is_gpu() {
        if device.is_nvidia() {
            recommendations.push(
                "Add 'kvm.ignore_msrs=1' to kernel parameters (prevents Code 43)".to_string(),
            );
            recommendations.push("Use q35 machine type with OVMF UEFI firmware".to_string());
            recommendations.push("Consider using vfio-bind for runtime GPU switching".to_string());
        } else if device.is_amd() {
            recommendations
                .push("Check if GPU needs vendor-reset module for proper VM restart".to_string());
            recommendations.push("AMD GPUs generally work well with OVMF UEFI".to_string());
        }

        // Check for audio function
        let has_audio = group
            .devices
            .iter()
            .any(|d| matches!(d.class, PciDeviceClass::AudioDevice) && d.slot() == device.slot());
        if has_audio {
            recommendations
                .push("Include GPU audio function in passthrough for HDMI/DP audio".to_string());
        }
    }

    // Add IOMMU kernel param if not already present
    if device.is_intel() {
        required_kernel_params.insert(0, "intel_iommu=on".to_string());
    } else {
        required_kernel_params.insert(0, "amd_iommu=on".to_string());
    }
    if !required_kernel_params.contains(&"iommu=pt".to_string()) {
        required_kernel_params.insert(1, "iommu=pt".to_string());
    }

    let viability = PassthroughViability::from_score(group.isolation_score);

    PassthroughAnalysis {
        device: device.clone(),
        group: group.clone(),
        viability,
        isolation_score: group.isolation_score,
        issues,
        recommendations,
        required_kernel_params,
        vfio_device_ids,
    }
}

/// Print analysis to stdout
pub fn print_analysis(analysis: &PassthroughAnalysis) {
    println!("\nPassthrough Analysis Report");
    println!("{}", "=".repeat(60));

    println!(
        "\nDevice: {} ({})",
        analysis.device.address, analysis.device.description
    );
    println!("IOMMU Group: {}", analysis.group.id);
    println!("Isolation Score: {}/100", analysis.isolation_score);
    println!("Viability: {}", analysis.viability);
    println!("  {}", analysis.viability.description());

    if !analysis.issues.is_empty() {
        println!("\nIssues Detected:");
        for issue in &analysis.issues {
            let icon = match issue.severity {
                IssueSeverity::Critical => "!!",
                IssueSeverity::Warning => "!",
                IssueSeverity::Info => "i",
            };
            println!("  [{}] {}: {}", icon, issue.code, issue.message);
            if let Some(fix) = &issue.fix {
                println!("      Fix: {}", fix);
            }
        }
    }

    if !analysis.recommendations.is_empty() {
        println!("\nRecommendations:");
        for (i, rec) in analysis.recommendations.iter().enumerate() {
            println!("  {}. {}", i + 1, rec);
        }
    }

    if !analysis.required_kernel_params.is_empty() {
        println!("\nRequired Kernel Parameters:");
        println!("  {}", analysis.required_kernel_params.join(" "));
    }

    if !analysis.vfio_device_ids.is_empty() {
        println!("\nVFIO Device IDs:");
        println!("  vfio-pci.ids={}", analysis.vfio_device_ids.join(","));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viability_from_score() {
        assert_eq!(
            PassthroughViability::from_score(100),
            PassthroughViability::Excellent
        );
        assert_eq!(
            PassthroughViability::from_score(90),
            PassthroughViability::Excellent
        );
        assert_eq!(
            PassthroughViability::from_score(75),
            PassthroughViability::Good
        );
        assert_eq!(
            PassthroughViability::from_score(60),
            PassthroughViability::Moderate
        );
        assert_eq!(
            PassthroughViability::from_score(40),
            PassthroughViability::Poor
        );
        assert_eq!(
            PassthroughViability::from_score(20),
            PassthroughViability::NotViable
        );
    }

    #[test]
    fn test_issue_severity_display() {
        assert_eq!(format!("{}", IssueSeverity::Critical), "CRITICAL");
        assert_eq!(format!("{}", IssueSeverity::Warning), "WARNING");
        assert_eq!(format!("{}", IssueSeverity::Info), "INFO");
    }
}
