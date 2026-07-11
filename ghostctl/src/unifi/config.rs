use serde::{Deserialize, Serialize};
use std::net::IpAddr;

/// UniFi configuration stored in config.toml under [unifi].
///
/// Targets a self-hosted **UniFi OS Server** (current, not the legacy Network
/// Application which is EOL). The controller API lives on HTTPS port 11443 and
/// authenticates with an API key (`X-API-KEY` header) generated under the
/// Network Application's Settings -> Integrations. The key is resolved from the
/// environment first (`UNIFI_API_KEY`, then `GHOSTCTL_UNIFI_API_KEY`) and only
/// falls back to the config file, so it need not be written to disk.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnifiConfig {
    /// Controller base URL. UniFi OS Server listens on HTTPS :11443.
    #[serde(default = "default_controller_url")]
    pub controller_url: String,

    /// Site name/id (UniFi's default site is literally `default`).
    #[serde(default = "default_site")]
    pub site: String,

    /// Optional API key (env vars take precedence, never logged).
    #[serde(default)]
    pub api_key: Option<String>,

    /// Verify the controller's TLS certificate. Self-hosted controllers ship a
    /// self-signed cert, so this defaults to false for local use. Set true once
    /// a trusted cert is in place.
    #[serde(default)]
    pub verify_tls: bool,

    /// Inform port devices use to reach the controller (`/inform`).
    #[serde(default = "default_inform_port")]
    pub inform_port: u16,

    /// Host devices should inform to. Defaults to the controller_url host when unset.
    #[serde(default)]
    pub inform_host: Option<String>,

    /// SSH usernames tried during remote adoption (factory devices use ui/ubnt).
    #[serde(default = "default_adopt_users")]
    pub adopt_ssh_users: Vec<String>,

    /// Optional SSH password for adoption (env `UNIFI_ADOPT_PASSWORD` preferred;
    /// passed to sshpass via the environment, never on the command line).
    #[serde(default)]
    pub adopt_ssh_password: Option<String>,

    /// Optional SSH private key path for adoption (preferred over password).
    #[serde(default)]
    pub adopt_ssh_key: Option<String>,

    /// CIDRs to exempt from CrowdSec bans (Tailscale CGNAT + mgmt subnets).
    #[serde(default = "default_exempt_cidrs")]
    pub exempt_cidrs: Vec<String>,

    /// HTTP request timeout in seconds.
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_controller_url() -> String {
    "https://127.0.0.1:11443".to_string()
}

fn default_site() -> String {
    "default".to_string()
}

fn default_inform_port() -> u16 {
    8080
}

fn default_adopt_users() -> Vec<String> {
    vec!["ui".to_string(), "ubnt".to_string()]
}

fn default_exempt_cidrs() -> Vec<String> {
    // Tailscale CGNAT range: legit management traffic that must never be banned.
    vec!["100.64.0.0/10".to_string()]
}

fn default_timeout() -> u64 {
    15
}

impl Default for UnifiConfig {
    fn default() -> Self {
        Self {
            controller_url: default_controller_url(),
            site: default_site(),
            api_key: None,
            verify_tls: false,
            inform_port: default_inform_port(),
            inform_host: None,
            adopt_ssh_users: default_adopt_users(),
            adopt_ssh_password: None,
            adopt_ssh_key: None,
            exempt_cidrs: default_exempt_cidrs(),
            timeout_secs: default_timeout(),
        }
    }
}

impl UnifiConfig {
    pub fn load() -> Self {
        crate::config::GhostConfig::load().unifi.unwrap_or_default()
    }

    /// Controller base URL with any trailing slash removed.
    pub fn base(&self) -> &str {
        self.controller_url.trim_end_matches('/')
    }

    /// Controller base URL parsed and normalized.
    pub fn parsed_base_url(&self) -> Result<reqwest::Url, String> {
        let url = reqwest::Url::parse(self.base())
            .map_err(|e| format!("invalid [unifi].controller_url: {e}"))?;
        match url.scheme() {
            "http" | "https" => {}
            scheme => {
                return Err(format!(
                    "invalid [unifi].controller_url scheme '{scheme}' (expected http or https)"
                ));
            }
        }
        if url.host_str().is_none() {
            return Err("invalid [unifi].controller_url: missing host".to_string());
        }
        Ok(url)
    }

    /// Resolve the API key: `UNIFI_API_KEY`, then `GHOSTCTL_UNIFI_API_KEY`, then
    /// the config file. Empty values are ignored.
    pub fn resolve_api_key(&self) -> Option<String> {
        for var in ["UNIFI_API_KEY", "GHOSTCTL_UNIFI_API_KEY"] {
            if let Ok(v) = std::env::var(var)
                && !v.trim().is_empty()
            {
                return Some(v);
            }
        }
        self.api_key
            .as_ref()
            .map(|k| k.trim().to_string())
            .filter(|k| !k.is_empty())
    }

    /// Resolve the adoption SSH password: `UNIFI_ADOPT_PASSWORD` env first, then
    /// config. Empty values are ignored.
    pub fn resolve_adopt_password(&self) -> Option<String> {
        if let Ok(v) = std::env::var("UNIFI_ADOPT_PASSWORD")
            && !v.is_empty()
        {
            return Some(v);
        }
        self.adopt_ssh_password
            .as_ref()
            .filter(|p| !p.is_empty())
            .cloned()
    }

    /// Host portion of the controller URL, used as the default inform host.
    pub fn controller_host(&self) -> Result<String, String> {
        let url = self.parsed_base_url()?;
        url.host_str()
            .map(normalize_url_host)
            .ok_or_else(|| "invalid [unifi].controller_url: missing host".to_string())
    }

    /// The host devices should be pointed at for `/inform`.
    pub fn effective_inform_host(&self) -> Result<String, String> {
        let host = self
            .inform_host
            .as_ref()
            .filter(|h| !h.trim().is_empty())
            .map(|h| h.trim().to_string())
            .map(Ok)
            .unwrap_or_else(|| self.controller_host())?;
        validate_host(&host)?;
        Ok(host)
    }

    /// Validate and normalize an operator-supplied inform host override.
    pub fn validate_inform_host(host: &str) -> Result<String, String> {
        let trimmed = host.trim();
        validate_host(trimmed)?;
        Ok(trimmed.to_string())
    }
}

fn normalize_url_host(host: &str) -> String {
    host.strip_prefix('[')
        .and_then(|h| h.strip_suffix(']'))
        .unwrap_or(host)
        .to_string()
}

fn validate_host(host: &str) -> Result<(), String> {
    if host.is_empty() {
        return Err("host must not be empty".to_string());
    }
    if host.len() > 253 {
        return Err(format!("host '{host}' is too long"));
    }
    if host.parse::<IpAddr>().is_ok() {
        return Ok(());
    }
    for label in host.trim_end_matches('.').split('.') {
        if label.is_empty() || label.len() > 63 {
            return Err(format!("invalid host '{host}'"));
        }
        let bytes = label.as_bytes();
        if bytes.first() == Some(&b'-') || bytes.last() == Some(&b'-') {
            return Err(format!("invalid host '{host}'"));
        }
        if !bytes
            .iter()
            .all(|b| b.is_ascii_alphanumeric() || *b == b'-')
        {
            return Err(format!("invalid host '{host}'"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let cfg = UnifiConfig::default();
        assert_eq!(cfg.controller_url, "https://127.0.0.1:11443");
        assert_eq!(cfg.site, "default");
        assert!(cfg.api_key.is_none());
        assert!(!cfg.verify_tls);
        assert_eq!(cfg.inform_port, 8080);
        assert_eq!(cfg.adopt_ssh_users, vec!["ui", "ubnt"]);
        assert_eq!(cfg.exempt_cidrs, vec!["100.64.0.0/10"]);
        assert_eq!(cfg.timeout_secs, 15);
    }

    #[test]
    fn test_base_strips_trailing_slash() {
        let cfg = UnifiConfig {
            controller_url: "https://unifi.cktechx.com:11443/".to_string(),
            ..Default::default()
        };
        assert_eq!(cfg.base(), "https://unifi.cktechx.com:11443");
    }

    #[test]
    fn test_controller_host_and_inform() {
        let cfg = UnifiConfig {
            controller_url: "https://unifi.cktechx.com:11443".to_string(),
            ..Default::default()
        };
        assert_eq!(cfg.controller_host().unwrap(), "unifi.cktechx.com");
        // With no explicit inform_host, it falls back to the controller host.
        assert_eq!(cfg.effective_inform_host().unwrap(), "unifi.cktechx.com");

        let cfg2 = UnifiConfig {
            controller_url: "https://10.0.0.1:11443".to_string(),
            inform_host: Some("69.169.98.98".to_string()),
            ..Default::default()
        };
        assert_eq!(cfg2.effective_inform_host().unwrap(), "69.169.98.98");
    }

    #[test]
    fn test_controller_host_handles_ipv6_url() {
        let cfg = UnifiConfig {
            controller_url: "https://[fd00::1]:11443".to_string(),
            ..Default::default()
        };
        assert_eq!(cfg.controller_host().unwrap(), "fd00::1");
        assert_eq!(cfg.effective_inform_host().unwrap(), "fd00::1");
    }

    #[test]
    fn test_inform_host_rejects_shell_metacharacters() {
        assert!(UnifiConfig::validate_inform_host("unifi.example.com").is_ok());
        assert!(UnifiConfig::validate_inform_host("192.168.1.2").is_ok());
        assert!(UnifiConfig::validate_inform_host("unifi.example.com;reboot").is_err());
        assert!(UnifiConfig::validate_inform_host("$(id)").is_err());
        assert!(UnifiConfig::validate_inform_host("bad_host").is_err());
    }

    #[test]
    fn test_roundtrip() {
        let cfg = UnifiConfig {
            controller_url: "https://unifi.cktechx.com:11443".to_string(),
            site: "default".to_string(),
            api_key: Some("secret".to_string()),
            verify_tls: true,
            inform_port: 8080,
            inform_host: Some("69.169.98.98".to_string()),
            adopt_ssh_users: vec!["ui".to_string()],
            adopt_ssh_password: None,
            adopt_ssh_key: Some("/home/chris/.ssh/id_ed25519".to_string()),
            exempt_cidrs: vec!["100.64.0.0/10".to_string(), "10.0.0.0/24".to_string()],
            timeout_secs: 20,
        };
        let toml_str = toml::to_string_pretty(&cfg).unwrap();
        let parsed: UnifiConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.controller_url, cfg.controller_url);
        assert_eq!(parsed.inform_host, cfg.inform_host);
        assert_eq!(parsed.exempt_cidrs, cfg.exempt_cidrs);
        assert_eq!(parsed.timeout_secs, 20);
    }

    #[test]
    fn test_partial_toml_uses_defaults() {
        let parsed: UnifiConfig =
            toml::from_str("controller_url = \"https://host:11443\"").unwrap();
        assert_eq!(parsed.controller_url, "https://host:11443");
        assert_eq!(parsed.site, "default");
        assert_eq!(parsed.inform_port, 8080);
        assert_eq!(parsed.timeout_secs, 15);
    }
}
