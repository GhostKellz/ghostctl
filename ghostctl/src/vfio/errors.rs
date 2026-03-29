//! VFIO error types for ghostctl
//!
//! This module provides structured error types for VFIO operations.

use thiserror::Error;

/// Errors that can occur during VFIO operations
#[derive(Error, Debug)]
pub enum VfioError {
    /// VFIO modules are not loaded
    #[error("VFIO modules not loaded: {0}")]
    ModulesNotLoaded(String),

    /// Failed to bind a device
    #[error("Failed to bind device {device} to {driver}: {reason}")]
    BindError {
        device: String,
        driver: String,
        reason: String,
    },

    /// Failed to unbind a device
    #[error("Failed to unbind device {device}: {reason}")]
    UnbindError { device: String, reason: String },

    /// Configuration file error
    #[error("Configuration file error: {path}: {reason}")]
    ConfigError { path: String, reason: String },

    /// ROM dump failed
    #[error("ROM dump failed for {device}: {reason}")]
    RomDumpError { device: String, reason: String },

    /// Single GPU passthrough error
    #[error("Single GPU passthrough error: {0}")]
    SingleGpuError(String),

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Device is in use
    #[error("Device is in use: {0}")]
    DeviceInUse(String),

    /// Device not found
    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    /// Invalid PCI address
    #[error("Invalid PCI address: {0}")]
    InvalidPciAddress(String),

    /// IOMMU group error
    #[error("IOMMU group error: {0}")]
    IommuGroupError(String),

    /// Script generation error
    #[error("Script generation error: {0}")]
    ScriptError(String),

    /// Display manager error
    #[error("Display manager error: {0}")]
    DisplayManagerError(String),

    /// IO error wrapper
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// IOMMU error wrapper
    #[error("IOMMU error: {0}")]
    IommuError(#[from] crate::iommu::errors::IommuError),

    /// Command execution failed
    #[error("Command execution failed: {0}")]
    CommandError(String),
}

/// Result type alias for VFIO operations
pub type VfioResult<T> = std::result::Result<T, VfioError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = VfioError::ModulesNotLoaded("vfio_pci".to_string());
        assert!(err.to_string().contains("vfio_pci"));

        let err = VfioError::BindError {
            device: "0000:01:00.0".to_string(),
            driver: "vfio-pci".to_string(),
            reason: "device in use".to_string(),
        };
        assert!(err.to_string().contains("01:00.0"));
    }
}
