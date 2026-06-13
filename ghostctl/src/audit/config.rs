use serde::{Deserialize, Serialize};

/// Package-audit configuration stored in config.toml under [audit].
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditConfig {
    /// Arch Security Tracker JSON endpoint.
    #[serde(default = "default_tracker_url")]
    pub tracker_url: String,

    /// AUR base URL (used to fetch PKGBUILDs for foreign packages).
    #[serde(default = "default_aur_base")]
    pub aur_base_url: String,

    /// HTTP request timeout in seconds.
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,

    /// Optional indicator-of-compromise feed for `audit ioc`: a local file path
    /// or http(s) URL listing one suspect package name per line (`#` comments
    /// allowed). Left empty by default so no campaign-specific data is baked
    /// into the binary - point it at a feed you trust.
    #[serde(default)]
    pub ioc_feed: Option<String>,

    /// Glob for pacman log files scanned by `audit ioc`.
    #[serde(default = "default_pacman_log_glob")]
    pub pacman_log_glob: String,
}

fn default_tracker_url() -> String {
    "https://security.archlinux.org/json".to_string()
}

fn default_aur_base() -> String {
    "https://aur.archlinux.org".to_string()
}

fn default_timeout() -> u64 {
    20
}

fn default_pacman_log_glob() -> String {
    "/var/log/pacman.log*".to_string()
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            tracker_url: default_tracker_url(),
            aur_base_url: default_aur_base(),
            timeout_secs: default_timeout(),
            ioc_feed: None,
            pacman_log_glob: default_pacman_log_glob(),
        }
    }
}

impl AuditConfig {
    pub fn load() -> Self {
        crate::config::GhostConfig::load().audit.unwrap_or_default()
    }

    /// URL for a package's PKGBUILD on the AUR cgit mirror.
    pub fn pkgbuild_url(&self, pkg: &str) -> String {
        self.aur_file_url(pkg, "PKGBUILD")
    }

    /// URL for an arbitrary file in a package's AUR repo (e.g. its `.install`
    /// hook) on the cgit mirror.
    pub fn aur_file_url(&self, pkg: &str, file: &str) -> String {
        format!(
            "{}/cgit/aur.git/plain/{}?h={}",
            self.aur_base_url.trim_end_matches('/'),
            file,
            pkg
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let cfg = AuditConfig::default();
        assert_eq!(cfg.tracker_url, "https://security.archlinux.org/json");
        assert_eq!(cfg.timeout_secs, 20);
    }

    #[test]
    fn test_pkgbuild_url() {
        let cfg = AuditConfig::default();
        assert_eq!(
            cfg.pkgbuild_url("yay"),
            "https://aur.archlinux.org/cgit/aur.git/plain/PKGBUILD?h=yay"
        );
    }

    #[test]
    fn test_aur_file_url() {
        let cfg = AuditConfig::default();
        assert_eq!(
            cfg.aur_file_url("yay", "yay.install"),
            "https://aur.archlinux.org/cgit/aur.git/plain/yay.install?h=yay"
        );
    }

    #[test]
    fn test_roundtrip() {
        let cfg = AuditConfig {
            tracker_url: "https://example.com/json".to_string(),
            aur_base_url: "https://aur.example.com".to_string(),
            timeout_secs: 5,
            ioc_feed: Some("/etc/ghostctl/ioc.txt".to_string()),
            pacman_log_glob: "/var/log/pacman.log*".to_string(),
        };
        let toml_str = toml::to_string_pretty(&cfg).unwrap();
        let parsed: AuditConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.tracker_url, cfg.tracker_url);
        assert_eq!(parsed.aur_base_url, cfg.aur_base_url);
        assert_eq!(parsed.timeout_secs, cfg.timeout_secs);
        assert_eq!(parsed.ioc_feed, cfg.ioc_feed);
        assert_eq!(parsed.pacman_log_glob, cfg.pacman_log_glob);
    }
}
