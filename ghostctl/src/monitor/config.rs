use serde::{Deserialize, Serialize};

/// Monitoring stack configuration stored in ghostctl config.toml under [monitor].
///
/// Targets the heimdall-style observability stack: Prometheus, Loki, Alertmanager,
/// and Grafana. All URLs default to localhost so the commands work out of the box
/// when run on the monitoring host.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonitorConfig {
    #[serde(default = "default_prometheus_url")]
    pub prometheus_url: String,

    #[serde(default = "default_loki_url")]
    pub loki_url: String,

    #[serde(default = "default_alertmanager_url")]
    pub alertmanager_url: String,

    #[serde(default = "default_grafana_url")]
    pub grafana_url: String,

    /// Grafana basic-auth user (defaults to "admin" when a token is set)
    #[serde(default)]
    pub grafana_user: Option<String>,

    /// Grafana admin password / API token for datasource health checks
    #[serde(default)]
    pub grafana_token: Option<String>,

    /// Optional node_exporter /metrics endpoint to include in health checks
    #[serde(default)]
    pub node_exporter_url: Option<String>,

    /// Optional cAdvisor endpoint to include in health checks
    #[serde(default)]
    pub cadvisor_url: Option<String>,

    /// HTTP request timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_prometheus_url() -> String {
    "http://127.0.0.1:9090".to_string()
}

fn default_loki_url() -> String {
    "http://127.0.0.1:3100".to_string()
}

fn default_alertmanager_url() -> String {
    "http://127.0.0.1:9093".to_string()
}

fn default_grafana_url() -> String {
    "http://127.0.0.1:3000".to_string()
}

fn default_timeout() -> u64 {
    10
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            prometheus_url: default_prometheus_url(),
            loki_url: default_loki_url(),
            alertmanager_url: default_alertmanager_url(),
            grafana_url: default_grafana_url(),
            grafana_user: None,
            grafana_token: None,
            node_exporter_url: None,
            cadvisor_url: None,
            timeout_secs: default_timeout(),
        }
    }
}

impl MonitorConfig {
    /// Load the monitor config from the ghostctl config, falling back to defaults.
    pub fn load() -> Self {
        crate::config::GhostConfig::load()
            .monitor
            .unwrap_or_default()
    }

    /// Trim a trailing slash from a base URL so joins are predictable.
    pub fn base(url: &str) -> &str {
        url.trim_end_matches('/')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let cfg = MonitorConfig::default();
        assert_eq!(cfg.prometheus_url, "http://127.0.0.1:9090");
        assert_eq!(cfg.loki_url, "http://127.0.0.1:3100");
        assert_eq!(cfg.alertmanager_url, "http://127.0.0.1:9093");
        assert_eq!(cfg.grafana_url, "http://127.0.0.1:3000");
        assert_eq!(cfg.timeout_secs, 10);
        assert!(cfg.grafana_token.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let cfg = MonitorConfig {
            prometheus_url: "http://10.0.0.10:9090".to_string(),
            loki_url: "http://10.0.0.10:3100".to_string(),
            alertmanager_url: "http://10.0.0.10:9093".to_string(),
            grafana_url: "http://10.0.0.10:3000".to_string(),
            grafana_user: Some("admin".to_string()),
            grafana_token: Some("secret".to_string()),
            node_exporter_url: Some("http://10.0.0.10:9100".to_string()),
            cadvisor_url: None,
            timeout_secs: 30,
        };
        let toml_str = toml::to_string_pretty(&cfg).unwrap();
        let parsed: MonitorConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.prometheus_url, cfg.prometheus_url);
        assert_eq!(parsed.grafana_user, cfg.grafana_user);
        assert_eq!(parsed.node_exporter_url, cfg.node_exporter_url);
        assert_eq!(parsed.timeout_secs, 30);
    }

    #[test]
    fn test_partial_toml_uses_defaults() {
        let parsed: MonitorConfig =
            toml::from_str("prometheus_url = \"http://host:9090\"").unwrap();
        assert_eq!(parsed.prometheus_url, "http://host:9090");
        // Unspecified fields fall back to defaults
        assert_eq!(parsed.loki_url, "http://127.0.0.1:3100");
        assert_eq!(parsed.timeout_secs, 10);
    }

    #[test]
    fn test_base_trims_slash() {
        assert_eq!(MonitorConfig::base("http://x:9090/"), "http://x:9090");
        assert_eq!(MonitorConfig::base("http://x:9090"), "http://x:9090");
    }
}
