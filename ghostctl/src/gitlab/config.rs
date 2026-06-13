use serde::{Deserialize, Serialize};

/// GitLab configuration stored in config.toml under [gitlab].
///
/// Targets self-hosted instances (e.g. `https://git.cktechx.com`) as well as
/// gitlab.com. The access token is resolved from the environment first
/// (`GITLAB_TOKEN`, then `GHOSTCTL_GITLAB_TOKEN`) and only falls back to the
/// config file, so tokens need not be written to disk.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitlabConfig {
    /// Base instance URL, without a trailing `/api/v4`.
    #[serde(default = "default_url")]
    pub url: String,

    /// Optional personal/project access token (env vars take precedence).
    #[serde(default)]
    pub token: Option<String>,

    /// Default project: numeric id or `group/subgroup/repo` path.
    #[serde(default)]
    pub project: Option<String>,

    /// HTTP request timeout in seconds.
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_url() -> String {
    "https://gitlab.com".to_string()
}

fn default_timeout() -> u64 {
    15
}

impl Default for GitlabConfig {
    fn default() -> Self {
        Self {
            url: default_url(),
            token: None,
            project: None,
            timeout_secs: default_timeout(),
        }
    }
}

impl GitlabConfig {
    pub fn load() -> Self {
        crate::config::GhostConfig::load()
            .gitlab
            .unwrap_or_default()
    }

    /// Resolve the access token: `GITLAB_TOKEN`, then `GHOSTCTL_GITLAB_TOKEN`,
    /// then the config file. Empty values are ignored.
    pub fn resolve_token(&self) -> Option<String> {
        for var in ["GITLAB_TOKEN", "GHOSTCTL_GITLAB_TOKEN"] {
            if let Ok(v) = std::env::var(var)
                && !v.trim().is_empty()
            {
                return Some(v);
            }
        }
        self.token
            .as_ref()
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
    }

    /// Instance base URL with any trailing slash removed.
    pub fn base(&self) -> &str {
        self.url.trim_end_matches('/')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let cfg = GitlabConfig::default();
        assert_eq!(cfg.url, "https://gitlab.com");
        assert!(cfg.token.is_none());
        assert!(cfg.project.is_none());
        assert_eq!(cfg.timeout_secs, 15);
    }

    #[test]
    fn test_base_strips_trailing_slash() {
        let cfg = GitlabConfig {
            url: "https://git.cktechx.com/".to_string(),
            ..Default::default()
        };
        assert_eq!(cfg.base(), "https://git.cktechx.com");
    }

    #[test]
    fn test_roundtrip() {
        let cfg = GitlabConfig {
            url: "https://git.cktechx.com".to_string(),
            token: Some("secret".to_string()),
            project: Some("group/repo".to_string()),
            timeout_secs: 10,
        };
        let toml_str = toml::to_string_pretty(&cfg).unwrap();
        let parsed: GitlabConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.url, cfg.url);
        assert_eq!(parsed.project, cfg.project);
        assert_eq!(parsed.timeout_secs, cfg.timeout_secs);
    }

    #[test]
    fn test_resolve_token_prefers_config_when_no_env() {
        let cfg = GitlabConfig {
            token: Some("cfg-token".to_string()),
            ..Default::default()
        };
        // This test must not set process env (global); with no GITLAB_TOKEN in
        // the test environment, the config token is used.
        if std::env::var("GITLAB_TOKEN").is_err() && std::env::var("GHOSTCTL_GITLAB_TOKEN").is_err()
        {
            assert_eq!(cfg.resolve_token().as_deref(), Some("cfg-token"));
        }
    }
}
