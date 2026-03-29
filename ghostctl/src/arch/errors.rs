//! Arch Linux module error types for ghostctl
//!
//! This module provides structured error types for Arch Linux operations.

use thiserror::Error;

/// Errors that can occur during Arch Linux operations
#[derive(Error, Debug)]
pub enum ArchError {
    /// Package installation failed
    #[error("Failed to install package {package}: {reason}")]
    PackageInstallFailed { package: String, reason: String },

    /// Package removal failed
    #[error("Failed to remove package {package}: {reason}")]
    PackageRemoveFailed { package: String, reason: String },

    /// Package query failed
    #[error("Failed to query packages: {0}")]
    PackageQueryFailed(String),

    /// AUR helper not found
    #[error("AUR helper not found: {0}")]
    AurHelperNotFound(String),

    /// AUR operation failed
    #[error("AUR operation failed: {0}")]
    AurOperationFailed(String),

    /// Mirror list update failed
    #[error("Failed to update mirror list: {0}")]
    MirrorUpdateFailed(String),

    /// Keyring operation failed
    #[error("Keyring operation failed: {0}")]
    KeyringError(String),

    /// Service operation failed
    #[error("Service operation failed for {service}: {reason}")]
    ServiceError { service: String, reason: String },

    /// Boot configuration error
    #[error("Boot configuration error: {0}")]
    BootConfigError(String),

    /// Kernel operation failed
    #[error("Kernel operation failed: {0}")]
    KernelError(String),

    /// Hardware detection failed
    #[error("Hardware detection failed: {0}")]
    HardwareDetectionError(String),

    /// Driver installation failed
    #[error("Driver installation failed for {driver}: {reason}")]
    DriverInstallError { driver: String, reason: String },

    /// Swap configuration error
    #[error("Swap configuration error: {0}")]
    SwapError(String),

    /// Zram configuration error
    #[error("Zram configuration error: {0}")]
    ZramError(String),

    /// Performance tuning error
    #[error("Performance tuning error: {0}")]
    PerfTuningError(String),

    /// Recovery operation failed
    #[error("Recovery operation failed: {0}")]
    RecoveryError(String),

    /// Diagnostics failed
    #[error("Diagnostics failed: {0}")]
    DiagnosticsError(String),

    /// Configuration file error
    #[error("Configuration file error: {path}: {reason}")]
    ConfigError { path: String, reason: String },

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Command execution failed
    #[error("Command execution failed: {0}")]
    CommandError(String),

    /// IO error wrapper
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Parse error
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Result type alias for Arch operations
pub type ArchResult<T> = std::result::Result<T, ArchError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ArchError::PackageInstallFailed {
            package: "nvidia".to_string(),
            reason: "dependency conflict".to_string(),
        };
        assert!(err.to_string().contains("nvidia"));

        let err = ArchError::AurHelperNotFound("yay".to_string());
        assert!(err.to_string().contains("yay"));
    }
}
