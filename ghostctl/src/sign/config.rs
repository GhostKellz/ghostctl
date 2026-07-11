use serde::{Deserialize, Serialize};

/// Signing configuration stored in ghostctl config.toml under [signing]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SigningConfig {
    /// Azure Key Vault URL (e.g. "https://my-vault.vault.azure.net")
    pub vault_url: String,

    /// Certificate/key name in Key Vault
    pub cert_name: String,

    /// Optional key version (uses latest if omitted)
    #[serde(default)]
    pub key_version: Option<String>,

    /// Azure tenant ID (required for service_principal auth)
    #[serde(default)]
    pub tenant_id: Option<String>,

    /// Azure client ID (required for service_principal auth)
    #[serde(default)]
    pub client_id: Option<String>,

    /// Signing algorithm
    #[serde(default = "default_algorithm")]
    pub algorithm: String,

    /// RFC 3161 timestamp authority URL
    #[serde(default = "default_tsa_url")]
    pub tsa_url: String,

    /// Stable OpenPGP public-key creation timestamp used for exported package-signing keys.
    ///
    /// OpenPGP v4 fingerprints include public-key creation time, so this must remain stable
    /// after users import the key into pacman, rpm, or gpg trust stores.
    #[serde(default)]
    pub pgp_key_created_at: Option<u32>,

    /// Authentication method: "cli" or "service_principal"
    #[serde(default = "default_auth_method")]
    pub auth_method: String,
}

fn default_algorithm() -> String {
    "RS256".to_string()
}

fn default_tsa_url() -> String {
    "http://timestamp.digicert.com".to_string()
}

fn default_auth_method() -> String {
    "cli".to_string()
}

impl Default for SigningConfig {
    fn default() -> Self {
        Self {
            vault_url: String::new(),
            cert_name: String::new(),
            key_version: None,
            tenant_id: None,
            client_id: None,
            algorithm: default_algorithm(),
            tsa_url: default_tsa_url(),
            pgp_key_created_at: None,
            auth_method: default_auth_method(),
        }
    }
}

/// Validate an Azure Key Vault key/certificate name.
/// Must be 1-127 characters, alphanumeric and hyphens only.
pub fn validate_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 127
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

/// Return the stable OpenPGP public-key creation timestamp.
///
/// Defaulting to 0 is intentional: it keeps generated OpenPGP fingerprints stable for
/// existing configs that do not yet have an explicit timestamp.
pub fn pgp_key_created_at(config: &SigningConfig) -> u32 {
    config.pgp_key_created_at.unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_name() {
        assert!(validate_name("my-signing-cert"));
        assert!(validate_name("cert123"));
        assert!(validate_name("a"));
        assert!(!validate_name(""));
        assert!(!validate_name("has spaces"));
        assert!(!validate_name("has_underscores"));
        assert!(!validate_name("has.dots"));
        assert!(!validate_name(&"a".repeat(128)));
    }

    #[test]
    fn test_default_config() {
        let cfg = SigningConfig::default();
        assert_eq!(cfg.algorithm, "RS256");
        assert_eq!(cfg.auth_method, "cli");
        assert_eq!(cfg.tsa_url, "http://timestamp.digicert.com");
    }

    #[test]
    fn test_config_roundtrip() {
        let cfg = SigningConfig {
            vault_url: "https://test.vault.azure.net".to_string(),
            cert_name: "test-cert".to_string(),
            key_version: None,
            tenant_id: Some("tenant-123".to_string()),
            client_id: Some("client-456".to_string()),
            algorithm: "RS384".to_string(),
            tsa_url: "http://timestamp.example.com".to_string(),
            pgp_key_created_at: Some(1700000000),
            auth_method: "service_principal".to_string(),
        };

        let toml_str = toml::to_string_pretty(&cfg).unwrap();
        let parsed: SigningConfig = toml::from_str(&toml_str).unwrap();

        assert_eq!(parsed.vault_url, cfg.vault_url);
        assert_eq!(parsed.cert_name, cfg.cert_name);
        assert_eq!(parsed.tenant_id, cfg.tenant_id);
        assert_eq!(parsed.algorithm, cfg.algorithm);
        assert_eq!(parsed.pgp_key_created_at, cfg.pgp_key_created_at);
        assert_eq!(parsed.auth_method, cfg.auth_method);
    }

    #[test]
    fn test_pgp_key_created_at_default_is_stable() {
        let cfg = SigningConfig::default();
        assert_eq!(pgp_key_created_at(&cfg), 0);
    }
}
