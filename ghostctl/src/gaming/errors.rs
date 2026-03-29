//! Gaming module error types for ghostctl
//!
//! This module provides structured error types for gaming operations.

use thiserror::Error;

/// Errors that can occur during gaming operations
#[derive(Error, Debug)]
pub enum GamingError {
    /// Steam installation failed
    #[error("Steam installation failed: {0}")]
    SteamInstallError(String),

    /// Steam configuration error
    #[error("Steam configuration error: {0}")]
    SteamConfigError(String),

    /// Proton installation failed
    #[error("Proton installation failed: {0}")]
    ProtonInstallError(String),

    /// Proton prefix error
    #[error("Proton prefix error: {0}")]
    ProtonPrefixError(String),

    /// Wine installation failed
    #[error("Wine installation failed: {0}")]
    WineInstallError(String),

    /// Wine prefix error
    #[error("Wine prefix error for {prefix}: {reason}")]
    WinePrefixError { prefix: String, reason: String },

    /// Lutris error
    #[error("Lutris error: {0}")]
    LutrisError(String),

    /// Graphics driver error
    #[error("Graphics driver error: {0}")]
    GraphicsDriverError(String),

    /// Vulkan configuration error
    #[error("Vulkan configuration error: {0}")]
    VulkanError(String),

    /// DXVK installation failed
    #[error("DXVK installation failed: {0}")]
    DxvkError(String),

    /// VKD3D installation failed
    #[error("VKD3D installation failed: {0}")]
    Vkd3dError(String),

    /// Performance optimization error
    #[error("Performance optimization error: {0}")]
    PerformanceError(String),

    /// Gamemode error
    #[error("Gamemode error: {0}")]
    GamemodeError(String),

    /// MangoHud error
    #[error("MangoHud error: {0}")]
    MangoHudError(String),

    /// Game launch failed
    #[error("Game launch failed for {game}: {reason}")]
    GameLaunchError { game: String, reason: String },

    /// Controller configuration error
    #[error("Controller configuration error: {0}")]
    ControllerError(String),

    /// Environment setup error
    #[error("Environment setup error: {0}")]
    EnvironmentError(String),

    /// Platform not supported
    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),

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

    /// Download failed
    #[error("Download failed: {0}")]
    DownloadError(String),
}

/// Result type alias for gaming operations
pub type GamingResult<T> = std::result::Result<T, GamingError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = GamingError::WinePrefixError {
            prefix: "~/.wine".to_string(),
            reason: "corrupted".to_string(),
        };
        assert!(err.to_string().contains(".wine"));

        let err = GamingError::ProtonInstallError("network timeout".to_string());
        assert!(err.to_string().contains("Proton"));
    }
}
