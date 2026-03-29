//! Proxmox module error types for ghostctl
//!
//! This module provides structured error types for Proxmox operations.

use thiserror::Error;

/// Errors that can occur during Proxmox operations
#[derive(Error, Debug)]
pub enum ProxmoxError {
    /// API request failed
    #[error("API request failed: {0}")]
    ApiError(String),

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthError(String),

    /// VM operation failed
    #[error("VM operation failed for VMID {vmid}: {reason}")]
    VmOperationError { vmid: u32, reason: String },

    /// Container operation failed
    #[error("Container operation failed for CTID {ctid}: {reason}")]
    ContainerOperationError { ctid: u32, reason: String },

    /// Template management error
    #[error("Template management error: {0}")]
    TemplateError(String),

    /// Storage operation failed
    #[error("Storage operation failed for {storage}: {reason}")]
    StorageError { storage: String, reason: String },

    /// Storage migration failed
    #[error("Storage migration failed: {0}")]
    MigrationError(String),

    /// Backup operation failed
    #[error("Backup operation failed: {0}")]
    BackupError(String),

    /// Backup rotation error
    #[error("Backup rotation error: {0}")]
    RotationError(String),

    /// PBS (Proxmox Backup Server) error
    #[error("PBS error: {0}")]
    PbsError(String),

    /// Firewall configuration error
    #[error("Firewall configuration error: {0}")]
    FirewallError(String),

    /// SDN (Software Defined Networking) error
    #[error("SDN error: {0}")]
    SdnError(String),

    /// Cluster operation failed
    #[error("Cluster operation failed: {0}")]
    ClusterError(String),

    /// Node operation failed
    #[error("Node operation failed for {node}: {reason}")]
    NodeError { node: String, reason: String },

    /// Upgrade operation failed
    #[error("Upgrade operation failed: {0}")]
    UpgradeError(String),

    /// VFIO passthrough error
    #[error("VFIO passthrough error: {0}")]
    VfioError(String),

    /// Script execution error
    #[error("Script execution error: {0}")]
    ScriptError(String),

    /// Script safety violation
    #[error("Script safety violation: {0}")]
    ScriptSafetyError(String),

    /// Configuration file error
    #[error("Configuration file error: {path}: {reason}")]
    ConfigError { path: String, reason: String },

    /// Configuration write error
    #[error("Failed to write configuration to {path}: {reason}")]
    ConfigWriteError { path: String, reason: String },

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Command execution failed
    #[error("Command execution failed: {0}")]
    CommandError(String),

    /// IO error wrapper
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON parsing error
    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// Result type alias for Proxmox operations
pub type ProxmoxResult<T> = std::result::Result<T, ProxmoxError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ProxmoxError::VmOperationError {
            vmid: 100,
            reason: "locked".to_string(),
        };
        assert!(err.to_string().contains("100"));

        let err = ProxmoxError::StorageError {
            storage: "local-lvm".to_string(),
            reason: "not found".to_string(),
        };
        assert!(err.to_string().contains("local-lvm"));
    }
}
