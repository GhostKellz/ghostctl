//! NVIDIA module error types for ghostctl
//!
//! This module provides structured error types for NVIDIA operations.

use thiserror::Error;

/// Errors that can occur during NVIDIA operations
#[derive(Error, Debug)]
pub enum NvidiaError {
    /// Driver installation failed
    #[error("Driver installation failed: {0}")]
    DriverInstallFailed(String),

    /// Driver not found
    #[error("NVIDIA driver not found: {0}")]
    DriverNotFound(String),

    /// CUDA installation failed
    #[error("CUDA installation failed: {0}")]
    CudaInstallFailed(String),

    /// Container toolkit error
    #[error("Container toolkit error: {0}")]
    ContainerToolkitError(String),

    /// Docker runtime configuration failed
    #[error("Docker runtime configuration failed: {0}")]
    DockerConfigError(String),

    /// GPU not detected
    #[error("No NVIDIA GPU detected")]
    NoGpuDetected,

    /// GPU passthrough error
    #[error("GPU passthrough error: {0}")]
    PassthroughError(String),

    /// VFIO binding failed
    #[error("VFIO binding failed for {device}: {reason}")]
    VfioBindError { device: String, reason: String },

    /// DKMS error
    #[error("DKMS error: {0}")]
    DkmsError(String),

    /// Module compilation failed
    #[error("Module compilation failed: {0}")]
    ModuleCompileFailed(String),

    /// DLSS operation failed
    #[error("DLSS operation failed: {0}")]
    DlssError(String),

    /// Wayland configuration error
    #[error("Wayland configuration error: {0}")]
    WaylandError(String),

    /// X11 configuration error
    #[error("X11 configuration error: {0}")]
    X11Error(String),

    /// Power management error
    #[error("Power management error: {0}")]
    PowerManagementError(String),

    /// Persistence mode error
    #[error("Persistence mode error: {0}")]
    PersistenceModeError(String),

    /// nvidia-smi error
    #[error("nvidia-smi error: {0}")]
    NvidiaSmiError(String),

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
}

/// Result type alias for NVIDIA operations
pub type NvidiaResult<T> = std::result::Result<T, NvidiaError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = NvidiaError::DriverInstallFailed("kernel mismatch".to_string());
        assert!(err.to_string().contains("kernel mismatch"));

        let err = NvidiaError::VfioBindError {
            device: "0000:01:00.0".to_string(),
            reason: "device in use".to_string(),
        };
        assert!(err.to_string().contains("01:00.0"));
    }
}
