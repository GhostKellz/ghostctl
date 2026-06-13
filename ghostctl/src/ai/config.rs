use serde::{Deserialize, Serialize};

/// Local AI configuration stored in ghostctl config.toml under [ai].
///
/// Targets a local Ollama server plus an optional Hermes agent CLI. Defaults
/// match the standard Ollama install (localhost:11434).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiConfig {
    #[serde(default = "default_ollama_url")]
    pub ollama_url: String,

    /// Default model for `ai run` / `ai ctx-check` when none is given
    #[serde(default)]
    pub default_model: Option<String>,

    /// Minimum context window (tokens) expected for agent use. Hermes requires >= 64k.
    #[serde(default = "default_min_context")]
    pub min_context: u64,

    /// Name/path of the Hermes agent CLI binary
    #[serde(default = "default_hermes_bin")]
    pub hermes_bin: String,

    /// HTTP request timeout in seconds (model loads can be slow)
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_ollama_url() -> String {
    "http://127.0.0.1:11434".to_string()
}

fn default_min_context() -> u64 {
    65536
}

fn default_hermes_bin() -> String {
    "hermes".to_string()
}

fn default_timeout() -> u64 {
    30
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            ollama_url: default_ollama_url(),
            default_model: None,
            min_context: default_min_context(),
            hermes_bin: default_hermes_bin(),
            timeout_secs: default_timeout(),
        }
    }
}

impl AiConfig {
    pub fn load() -> Self {
        crate::config::GhostConfig::load().ai.unwrap_or_default()
    }

    pub fn base(&self) -> &str {
        self.ollama_url.trim_end_matches('/')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let cfg = AiConfig::default();
        assert_eq!(cfg.ollama_url, "http://127.0.0.1:11434");
        assert_eq!(cfg.min_context, 65536);
        assert_eq!(cfg.hermes_bin, "hermes");
        assert_eq!(cfg.timeout_secs, 30);
        assert!(cfg.default_model.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let cfg = AiConfig {
            ollama_url: "http://10.0.0.5:11434".to_string(),
            default_model: Some("qwen3-coder:30b".to_string()),
            min_context: 131072,
            hermes_bin: "/usr/local/bin/hermes".to_string(),
            timeout_secs: 60,
        };
        let toml_str = toml::to_string_pretty(&cfg).unwrap();
        let parsed: AiConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.ollama_url, cfg.ollama_url);
        assert_eq!(parsed.default_model, cfg.default_model);
        assert_eq!(parsed.min_context, 131072);
    }

    #[test]
    fn test_base_trims_slash() {
        let cfg = AiConfig {
            ollama_url: "http://h:11434/".to_string(),
            ..Default::default()
        };
        assert_eq!(cfg.base(), "http://h:11434");
    }
}
