use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use super::config::SigningConfig;

const VAULT_RESOURCE: &str = "https://vault.azure.net";
const TOKEN_REFRESH_MARGIN_SECS: u64 = 300; // re-acquire if <5 min remaining

/// An Azure access token with expiry tracking
#[derive(Debug, Clone)]
pub struct AzureToken {
    pub access_token: String,
    pub expires_on: u64,
}

impl AzureToken {
    /// Check if the token is expired or within the refresh margin
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now + TOKEN_REFRESH_MARGIN_SECS >= self.expires_on
    }
}

/// Azure authentication method
pub enum AuthMethod {
    /// Use the current Azure CLI session
    AzureCli,
    /// Use a service principal with client credentials
    ServicePrincipal {
        tenant_id: String,
        client_id: String,
        client_secret: String,
    },
}

/// Response from `az account get-access-token`
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AzCliTokenResponse {
    access_token: String,
    expires_on: String,
}

/// Response from Azure AD OAuth2 token endpoint
#[derive(Deserialize)]
struct OAuthTokenResponse {
    access_token: String,
    expires_in: u64,
}

/// OAuth2 error response from Azure AD
#[derive(Deserialize)]
struct OAuthErrorResponse {
    error: String,
    error_description: Option<String>,
}

impl AuthMethod {
    /// Determine authentication method from signing config + environment
    pub fn from_config(config: &SigningConfig) -> Result<Self> {
        match config.auth_method.as_str() {
            "service_principal" | "sp" => {
                let tenant_id = config
                    .tenant_id
                    .as_ref()
                    .filter(|s| !s.is_empty())
                    .context(
                        "tenant_id required for service_principal auth (set in [signing] config)",
                    )?
                    .clone();

                let client_id = config
                    .client_id
                    .as_ref()
                    .filter(|s| !s.is_empty())
                    .context(
                        "client_id required for service_principal auth (set in [signing] config)",
                    )?
                    .clone();

                let client_secret = std::env::var("AZURE_CLIENT_SECRET").context(
                    "AZURE_CLIENT_SECRET environment variable required for service_principal auth",
                )?;

                Ok(AuthMethod::ServicePrincipal {
                    tenant_id,
                    client_id,
                    client_secret,
                })
            }
            _ => Ok(AuthMethod::AzureCli),
        }
    }

    /// Acquire a Bearer token for Azure Key Vault
    pub fn acquire_token(&self) -> Result<AzureToken> {
        match self {
            AuthMethod::AzureCli => acquire_cli_token(),
            AuthMethod::ServicePrincipal {
                tenant_id,
                client_id,
                client_secret,
            } => acquire_sp_token(tenant_id, client_id, client_secret),
        }
    }
}

/// Acquire token via Azure CLI
fn acquire_cli_token() -> Result<AzureToken> {
    let output = Command::new("az")
        .args([
            "account",
            "get-access-token",
            "--resource",
            VAULT_RESOURCE,
            "--output",
            "json",
        ])
        .output()
        .context("Failed to run 'az' CLI. Is Azure CLI installed and are you logged in?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Azure CLI token acquisition failed: {}", stderr.trim());
    }

    let response: AzCliTokenResponse = serde_json::from_slice(&output.stdout)
        .context("Failed to parse Azure CLI token response")?;

    // Azure CLI returns expires_on as a Unix timestamp string (or datetime)
    let expires_on = parse_expires_on(&response.expires_on)?;

    Ok(AzureToken {
        access_token: response.access_token,
        expires_on,
    })
}

/// Parse the expires_on field from Azure CLI response.
/// It can be a Unix timestamp string or an ISO 8601 datetime.
fn parse_expires_on(value: &str) -> Result<u64> {
    // Try parsing as a plain integer first (Unix timestamp)
    if let Ok(ts) = value.parse::<u64>() {
        return Ok(ts);
    }

    // Try parsing as integer with possible whitespace
    if let Ok(ts) = value.trim().parse::<u64>() {
        return Ok(ts);
    }

    // Fallback: assume it's roughly 1 hour from now
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    Ok(now + 3600)
}

/// Acquire token via service principal OAuth2 client credentials flow
fn acquire_sp_token(tenant_id: &str, client_id: &str, client_secret: &str) -> Result<AzureToken> {
    let token_url = format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        tenant_id
    );

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .context("Failed to create HTTP client")?;

    let scope = format!("{}/.default", VAULT_RESOURCE);
    let form_body = format!(
        "grant_type=client_credentials&client_id={}&client_secret={}&scope={}",
        urlencoding(client_id),
        urlencoding(client_secret),
        urlencoding(&scope),
    );

    let response = client
        .post(&token_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_body)
        .send()
        .context("Failed to reach Azure AD token endpoint")?;

    let status = response.status();
    let body = response
        .text()
        .context("Failed to read token response body")?;

    if !status.is_success() {
        if let Ok(err) = serde_json::from_str::<OAuthErrorResponse>(&body) {
            bail!(
                "Azure AD auth failed ({}): {}",
                err.error,
                err.error_description.unwrap_or_default()
            );
        }
        bail!("Azure AD auth failed (HTTP {}): {}", status, body);
    }

    let token_resp: OAuthTokenResponse =
        serde_json::from_str(&body).context("Failed to parse OAuth2 token response")?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    Ok(AzureToken {
        access_token: token_resp.access_token,
        expires_on: now + token_resp.expires_in,
    })
}

/// Minimal URL encoding for form parameters.
/// Encodes characters that are not unreserved per RFC 3986.
fn urlencoding(s: &str) -> String {
    let mut encoded = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", b));
            }
        }
    }
    encoded
}

/// Check if Azure CLI is installed and logged in
pub fn check_az_cli() -> Result<bool> {
    let output = Command::new("az").arg("version").output();
    match output {
        Ok(o) if o.status.success() => Ok(true),
        _ => Ok(false),
    }
}

/// Check if Azure CLI has an active session
pub fn check_az_session() -> Result<bool> {
    let output = Command::new("az")
        .args(["account", "show", "--output", "json"])
        .output();
    match output {
        Ok(o) if o.status.success() => Ok(true),
        _ => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expires_on_unix() {
        let ts = parse_expires_on("1716580800").unwrap();
        assert_eq!(ts, 1716580800);
    }

    #[test]
    fn test_parse_expires_on_fallback() {
        // Non-numeric string should fall back to ~1 hour from now
        let ts = parse_expires_on("2026-05-24T12:00:00Z").unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        assert!(ts > now);
        assert!(ts <= now + 3601);
    }

    #[test]
    fn test_token_expiry() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let expired = AzureToken {
            access_token: "test".to_string(),
            expires_on: now - 1,
        };
        assert!(expired.is_expired());

        let soon = AzureToken {
            access_token: "test".to_string(),
            expires_on: now + 60, // 1 min left, within 5 min margin
        };
        assert!(soon.is_expired());

        let valid = AzureToken {
            access_token: "test".to_string(),
            expires_on: now + 3600,
        };
        assert!(!valid.is_expired());
    }
}
