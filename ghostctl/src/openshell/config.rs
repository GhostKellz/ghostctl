use serde::{Deserialize, Serialize};

/// OpenShell configuration stored in config.toml under [openshell].
///
/// OpenShell provides sandboxed, policy-governed runtimes for autonomous AI
/// agents. ghostctl does not reimplement its CLI; it runs readiness checks and
/// passes commands through to the `openshell` binary. The gateway URL defaults
/// to the standalone Docker dev gateway bound on localhost.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenshellConfig {
    /// Name/path of the OpenShell CLI binary
    #[serde(default = "default_bin")]
    pub bin: String,

    /// Gateway HTTP endpoint used for the reachability check
    #[serde(default = "default_gateway_url")]
    pub gateway_url: String,

    /// HTTP request timeout in seconds for the reachability check
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_bin() -> String {
    "openshell".to_string()
}

fn default_gateway_url() -> String {
    "http://127.0.0.1:18080".to_string()
}

fn default_timeout() -> u64 {
    10
}

impl Default for OpenshellConfig {
    fn default() -> Self {
        Self {
            bin: default_bin(),
            gateway_url: default_gateway_url(),
            timeout_secs: default_timeout(),
        }
    }
}

impl OpenshellConfig {
    pub fn load() -> Self {
        crate::config::GhostConfig::load()
            .openshell
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let cfg = OpenshellConfig::default();
        assert_eq!(cfg.bin, "openshell");
        assert_eq!(cfg.gateway_url, "http://127.0.0.1:18080");
        assert_eq!(cfg.timeout_secs, 10);
    }

    #[test]
    fn test_roundtrip() {
        let cfg = OpenshellConfig {
            bin: "/usr/local/bin/openshell".to_string(),
            gateway_url: "http://127.0.0.1:19080".to_string(),
            timeout_secs: 5,
        };
        let toml_str = toml::to_string_pretty(&cfg).unwrap();
        let parsed: OpenshellConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.bin, cfg.bin);
        assert_eq!(parsed.gateway_url, cfg.gateway_url);
        assert_eq!(parsed.timeout_secs, cfg.timeout_secs);
    }
}
