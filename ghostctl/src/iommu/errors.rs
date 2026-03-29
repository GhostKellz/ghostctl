//! IOMMU error types for ghostctl
//!
//! This module provides structured error types for IOMMU operations.

use thiserror::Error;

/// Errors that can occur during IOMMU operations
#[derive(Error, Debug)]
pub enum IommuError {
    /// IOMMU is not enabled in kernel parameters
    #[error("IOMMU not enabled in kernel parameters")]
    NotEnabled,

    /// IOMMU groups directory is not accessible
    #[error("IOMMU groups directory not accessible: {0}")]
    GroupsNotAccessible(String),

    /// Failed to parse an IOMMU group
    #[error("Failed to parse IOMMU group {group}: {reason}")]
    GroupParseError { group: u32, reason: String },

    /// PCI device not found
    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    /// Invalid PCI address format
    #[error("Invalid PCI address format: {0}")]
    InvalidPciAddress(String),

    /// PCIe topology error
    #[error("PCIe topology error: {0}")]
    PcieTopologyError(String),

    /// ACS detection error
    #[error("ACS detection failed: {0}")]
    AcsDetectionError(String),

    /// Failed to read kernel parameters
    #[error("Failed to read kernel parameters: {0}")]
    KernelParamsError(String),

    /// Command execution failed
    #[error("Command execution failed: {0}")]
    CommandError(String),

    /// IO error wrapper
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Parse error for numeric values
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Result type alias for IOMMU operations
pub type IommuResult<T> = std::result::Result<T, IommuError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = IommuError::NotEnabled;
        assert_eq!(err.to_string(), "IOMMU not enabled in kernel parameters");

        let err = IommuError::GroupParseError {
            group: 1,
            reason: "invalid data".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Failed to parse IOMMU group 1: invalid data"
        );
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let iommu_err: IommuError = io_err.into();
        assert!(matches!(iommu_err, IommuError::IoError(_)));
    }
}
