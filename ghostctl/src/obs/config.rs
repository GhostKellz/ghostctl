use serde::{Deserialize, Serialize};

/// OBS helper configuration stored in config.toml under [obs].
///
/// Controls the virtual-camera defaults and an optional override for the
/// xdg-desktop-portal backend (otherwise it is auto-detected per compositor).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObsConfig {
    /// Label shown for the v4l2loopback virtual camera device.
    #[serde(default = "default_vcam_label")]
    pub vcam_label: String,

    /// Fixed /dev/videoN number for the virtual camera (stable device path).
    #[serde(default = "default_vcam_video_nr")]
    pub vcam_video_nr: u32,

    /// Force a specific xdg-desktop-portal backend package (skip auto-detect).
    #[serde(default)]
    pub portal_backend: Option<String>,
}

fn default_vcam_label() -> String {
    "OBS Virtual Camera".to_string()
}

fn default_vcam_video_nr() -> u32 {
    10
}

impl Default for ObsConfig {
    fn default() -> Self {
        Self {
            vcam_label: default_vcam_label(),
            vcam_video_nr: default_vcam_video_nr(),
            portal_backend: None,
        }
    }
}

impl ObsConfig {
    pub fn load() -> Self {
        crate::config::GhostConfig::load().obs.unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let cfg = ObsConfig::default();
        assert_eq!(cfg.vcam_label, "OBS Virtual Camera");
        assert_eq!(cfg.vcam_video_nr, 10);
        assert!(cfg.portal_backend.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let cfg = ObsConfig {
            vcam_label: "My Cam".to_string(),
            vcam_video_nr: 42,
            portal_backend: Some("xdg-desktop-portal-wlr".to_string()),
        };
        let toml_str = toml::to_string_pretty(&cfg).unwrap();
        let parsed: ObsConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.vcam_label, cfg.vcam_label);
        assert_eq!(parsed.vcam_video_nr, cfg.vcam_video_nr);
        assert_eq!(parsed.portal_backend, cfg.portal_backend);
    }
}
