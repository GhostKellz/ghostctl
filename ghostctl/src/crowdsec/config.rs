use serde::{Deserialize, Serialize};

/// CrowdSec / threat-intel configuration stored in config.toml under [crowdsec].
///
/// Covers the public threat-feed endpoint, the optional LAPI Prometheus metrics
/// endpoint, and the DNS resolvers used for posture checks.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrowdsecConfig {
    /// Public threat-feed URL (plaintext IP/CIDR list)
    #[serde(default = "default_feed_url")]
    pub threat_feed_url: String,

    /// Optional CrowdSec LAPI Prometheus metrics endpoint (e.g. http://10.0.0.23:6060/metrics)
    #[serde(default)]
    pub lapi_metrics_url: Option<String>,

    /// Primary DNS resolver for posture checks
    #[serde(default = "default_dns_primary")]
    pub dns_primary: String,

    /// Optional backup DNS resolver
    #[serde(default)]
    pub dns_backup: Option<String>,

    /// HTTP request timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_feed_url() -> String {
    "https://threat.cktechnology.io/crowdsec.txt".to_string()
}

fn default_dns_primary() -> String {
    "10.0.0.2".to_string()
}

fn default_timeout() -> u64 {
    10
}

impl Default for CrowdsecConfig {
    fn default() -> Self {
        Self {
            threat_feed_url: default_feed_url(),
            lapi_metrics_url: None,
            dns_primary: default_dns_primary(),
            dns_backup: None,
            timeout_secs: default_timeout(),
        }
    }
}

impl CrowdsecConfig {
    pub fn load() -> Self {
        crate::config::GhostConfig::load()
            .crowdsec
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let cfg = CrowdsecConfig::default();
        assert_eq!(
            cfg.threat_feed_url,
            "https://threat.cktechnology.io/crowdsec.txt"
        );
        assert_eq!(cfg.dns_primary, "10.0.0.2");
        assert_eq!(cfg.timeout_secs, 10);
        assert!(cfg.lapi_metrics_url.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let cfg = CrowdsecConfig {
            threat_feed_url: "https://example.com/feed.txt".to_string(),
            lapi_metrics_url: Some("http://10.0.0.23:6060/metrics".to_string()),
            dns_primary: "1.1.1.1".to_string(),
            dns_backup: Some("9.9.9.9".to_string()),
            timeout_secs: 20,
        };
        let toml_str = toml::to_string_pretty(&cfg).unwrap();
        let parsed: CrowdsecConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.threat_feed_url, cfg.threat_feed_url);
        assert_eq!(parsed.lapi_metrics_url, cfg.lapi_metrics_url);
        assert_eq!(parsed.dns_backup, cfg.dns_backup);
    }
}
