//! Networking error types for ghostctl
//!
//! This module provides structured error types for networking operations.

use thiserror::Error;

/// Errors that can occur during networking operations
#[derive(Error, Debug)]
pub enum NetworkingError {
    /// User cancelled the operation
    #[error("Operation cancelled by user")]
    UserCancelled,

    /// Invalid port number or range
    #[error("Invalid port: {0}")]
    InvalidPort(String),

    /// Invalid IP address format
    #[error("Invalid IP address: {0}")]
    InvalidIpAddress(String),

    /// Invalid network interface
    #[error("Invalid network interface: {0}")]
    InvalidInterface(String),

    /// Firewall command failed
    #[error("Firewall command failed: {0}")]
    FirewallError(String),

    /// Network configuration error
    #[error("Network configuration error: {0}")]
    ConfigError(String),

    /// Bridge operation failed
    #[error("Bridge operation failed: {0}")]
    BridgeError(String),

    /// Command execution failed
    #[error("Command execution failed: {0}")]
    CommandError(String),

    /// IO error wrapper
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Parse error
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Dialoguer interaction error
    #[error("Interactive input error: {0}")]
    InteractionError(String),
}

/// Result type alias for networking operations
pub type NetworkingResult<T> = std::result::Result<T, NetworkingError>;

impl From<dialoguer::Error> for NetworkingError {
    fn from(err: dialoguer::Error) -> Self {
        NetworkingError::InteractionError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = NetworkingError::UserCancelled;
        assert_eq!(err.to_string(), "Operation cancelled by user");

        let err = NetworkingError::InvalidPort("99999".to_string());
        assert_eq!(err.to_string(), "Invalid port: 99999");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let net_err: NetworkingError = io_err.into();
        assert!(matches!(net_err, NetworkingError::IoError(_)));
    }
}
