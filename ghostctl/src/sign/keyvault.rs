use anyhow::{Context, Result, bail};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

use super::auth::{AuthMethod, AzureToken};
use super::errors::SignError;

const API_VERSION: &str = "7.4";
const MAX_RETRIES: u32 = 4;
const RETRY_DELAYS: [u64; 4] = [0, 1, 3, 10];

/// Azure Key Vault REST API client
pub struct KeyVaultClient {
    http: reqwest::blocking::Client,
    vault_url: String,
    token: AzureToken,
    auth_method: AuthMethod,
}

/// Key metadata from Key Vault
#[derive(Debug)]
pub struct KeyInfo {
    pub kid: String,
    pub key_type: String,
    pub enabled: bool,
}

/// Sign request body
#[derive(Serialize)]
struct SignRequest {
    alg: String,
    value: String,
}

/// Sign response
#[derive(Deserialize)]
pub struct SignResponse {
    kid: String,
    value: String,
}

/// Key Vault error response
#[derive(Deserialize)]
struct KvErrorResponse {
    error: KvError,
}

#[derive(Deserialize)]
struct KvError {
    code: String,
    message: String,
}

/// Key bundle from GET /keys/{name}
#[derive(Deserialize)]
struct KeyBundle {
    key: KeyProperties,
    attributes: KeyAttributes,
}

#[derive(Deserialize)]
struct KeyProperties {
    kid: String,
    kty: String,
}

#[derive(Deserialize)]
struct KeyAttributes {
    enabled: bool,
}

/// Certificate bundle from GET /certificates/{name}
#[derive(Deserialize)]
struct CertificateBundle {
    cer: String, // base64-encoded DER certificate
}

/// Certificate list response from GET /certificates
#[derive(Deserialize)]
struct CertificateListResponse {
    value: Vec<CertificateListItem>,
}

#[derive(Deserialize)]
struct CertificateListItem {
    id: String,
    #[serde(default)]
    attributes: Option<CertListAttributes>,
}

#[derive(Deserialize)]
struct CertListAttributes {
    enabled: Option<bool>,
    #[serde(rename = "exp")]
    expires: Option<u64>,
    #[serde(rename = "nbf")]
    not_before: Option<u64>,
    created: Option<u64>,
}

/// Certificate metadata from list operation
#[derive(Debug)]
pub struct CertificateItem {
    pub name: String,
    pub enabled: bool,
    pub expires: Option<u64>,
    pub not_before: Option<u64>,
    pub created: Option<u64>,
}

impl KeyVaultClient {
    /// Create a new Key Vault client
    pub fn new(vault_url: &str, auth_method: AuthMethod) -> Result<Self> {
        let token = auth_method
            .acquire_token()
            .context("Failed to acquire Azure token")?;

        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("ghostctl")
            .build()
            .context("Failed to create HTTP client")?;

        // Normalize vault URL (strip trailing slash)
        let vault_url = vault_url.trim_end_matches('/').to_string();

        Ok(Self {
            http,
            vault_url,
            token,
            auth_method,
        })
    }

    /// Ensure the token is still valid, refresh if needed
    fn ensure_token(&mut self) -> Result<()> {
        if self.token.is_expired() {
            self.token = self
                .auth_method
                .acquire_token()
                .context("Failed to refresh Azure token")?;
        }
        Ok(())
    }

    /// Sign a digest via Key Vault REST API.
    /// Returns the raw signature bytes.
    pub fn sign(
        &mut self,
        key_name: &str,
        key_version: Option<&str>,
        algorithm: &str,
        digest: &[u8],
    ) -> Result<SignResponse> {
        self.ensure_token()?;

        let version_path = match key_version {
            Some(v) if !v.is_empty() => format!("/{}", v),
            _ => String::new(),
        };

        let url = format!(
            "{}/keys/{}{}/sign?api-version={}",
            self.vault_url, key_name, version_path, API_VERSION
        );

        let body = SignRequest {
            alg: algorithm.to_string(),
            value: URL_SAFE_NO_PAD.encode(digest),
        };

        self.post_with_retry(&url, &body)
    }

    /// Get the X.509 certificate (DER-encoded) for a cert name
    pub fn get_certificate(&mut self, cert_name: &str, version: Option<&str>) -> Result<Vec<u8>> {
        self.ensure_token()?;

        let version_path = match version {
            Some(v) if !v.is_empty() => format!("/{}", v),
            _ => String::new(),
        };

        let url = format!(
            "{}/certificates/{}{}/{}?api-version={}",
            self.vault_url, cert_name, version_path, "", API_VERSION
        );

        // Clean up the double slash if version_path is empty
        let url = url
            .replace("//", "/")
            .replace("http:/", "http://")
            .replace("https:/", "https://");

        let response: CertificateBundle = self.get_with_retry(&url)?;
        let cert_der = URL_SAFE_NO_PAD
            .decode(&response.cer)
            .or_else(|_| {
                // Try standard base64 as Azure sometimes uses it for certs
                base64::engine::general_purpose::STANDARD.decode(&response.cer)
            })
            .context("Failed to decode certificate data")?;

        Ok(cert_der)
    }

    /// Check if a key exists and is enabled
    pub fn check_key(&mut self, key_name: &str) -> Result<KeyInfo> {
        self.ensure_token()?;

        let url = format!(
            "{}/keys/{}?api-version={}",
            self.vault_url, key_name, API_VERSION
        );

        let bundle: KeyBundle = self.get_with_retry(&url)?;

        Ok(KeyInfo {
            kid: bundle.key.kid,
            key_type: bundle.key.kty,
            enabled: bundle.attributes.enabled,
        })
    }

    /// POST request with retry and re-auth on 401
    fn post_with_retry<T: for<'de> Deserialize<'de>, B: Serialize>(
        &mut self,
        url: &str,
        body: &B,
    ) -> Result<T> {
        let mut last_error: Option<anyhow::Error> = None;

        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                let delay = RETRY_DELAYS.get(attempt as usize).copied().unwrap_or(10);
                thread::sleep(Duration::from_secs(delay));
            }

            let response = self
                .http
                .post(url)
                .bearer_auth(&self.token.access_token)
                .json(body)
                .send();

            match response {
                Ok(resp) => {
                    let status = resp.status();

                    if status.is_success() {
                        let parsed = resp.json::<T>().context("Failed to parse response")?;
                        return Ok(parsed);
                    }

                    // 401: try re-auth once
                    if status.as_u16() == 401 && attempt == 0 {
                        let resp_text = resp.text().unwrap_or_default();
                        log::debug!("Got 401, attempting token refresh: {}", resp_text);
                        if let Ok(new_token) = self.auth_method.acquire_token() {
                            self.token = new_token;
                            continue;
                        }
                        bail!(SignError::Auth(
                            "Token refresh failed after 401".to_string()
                        ));
                    }

                    // 429: rate limited
                    if status.as_u16() == 429 {
                        let retry_after = resp
                            .headers()
                            .get("retry-after")
                            .and_then(|v| v.to_str().ok())
                            .and_then(|v| v.parse::<u64>().ok())
                            .unwrap_or(10);
                        thread::sleep(Duration::from_secs(retry_after));
                        continue;
                    }

                    // Other error
                    let resp_text = resp.text().unwrap_or_default();
                    if let Ok(kv_err) = serde_json::from_str::<KvErrorResponse>(&resp_text) {
                        last_error = Some(
                            SignError::KeyVault {
                                status: status.as_u16(),
                                message: format!("{}: {}", kv_err.error.code, kv_err.error.message),
                            }
                            .into(),
                        );
                    } else {
                        last_error = Some(
                            SignError::KeyVault {
                                status: status.as_u16(),
                                message: resp_text,
                            }
                            .into(),
                        );
                    }
                }
                Err(e) => {
                    last_error = Some(SignError::Http(e.to_string()).into());
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Request failed after all retries")))
    }

    /// GET request with retry and re-auth on 401
    fn get_with_retry<T: for<'de> Deserialize<'de>>(&mut self, url: &str) -> Result<T> {
        let mut last_error: Option<anyhow::Error> = None;

        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                let delay = RETRY_DELAYS.get(attempt as usize).copied().unwrap_or(10);
                thread::sleep(Duration::from_secs(delay));
            }

            let response = self
                .http
                .get(url)
                .bearer_auth(&self.token.access_token)
                .send();

            match response {
                Ok(resp) => {
                    let status = resp.status();

                    if status.is_success() {
                        let parsed = resp.json::<T>().context("Failed to parse response")?;
                        return Ok(parsed);
                    }

                    if status.as_u16() == 401 && attempt == 0 {
                        if let Ok(new_token) = self.auth_method.acquire_token() {
                            self.token = new_token;
                            continue;
                        }
                        bail!(SignError::Auth(
                            "Token refresh failed after 401".to_string()
                        ));
                    }

                    if status.as_u16() == 429 {
                        let retry_after = resp
                            .headers()
                            .get("retry-after")
                            .and_then(|v| v.to_str().ok())
                            .and_then(|v| v.parse::<u64>().ok())
                            .unwrap_or(10);
                        thread::sleep(Duration::from_secs(retry_after));
                        continue;
                    }

                    let resp_text = resp.text().unwrap_or_default();
                    if let Ok(kv_err) = serde_json::from_str::<KvErrorResponse>(&resp_text) {
                        last_error = Some(
                            SignError::KeyVault {
                                status: status.as_u16(),
                                message: format!("{}: {}", kv_err.error.code, kv_err.error.message),
                            }
                            .into(),
                        );
                    } else {
                        last_error = Some(
                            SignError::KeyVault {
                                status: status.as_u16(),
                                message: resp_text,
                            }
                            .into(),
                        );
                    }
                }
                Err(e) => {
                    last_error = Some(SignError::Http(e.to_string()).into());
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Request failed after all retries")))
    }

    /// List all certificates in the Key Vault
    pub fn list_certificates(&mut self) -> Result<Vec<CertificateItem>> {
        self.ensure_token()?;

        let url = format!(
            "{}/certificates?api-version={}",
            self.vault_url, API_VERSION
        );

        let response: CertificateListResponse = self.get_with_retry(&url)?;

        let items = response
            .value
            .into_iter()
            .map(|item| {
                // Extract name from the certificate ID URL
                // Format: https://vault.azure.net/certificates/name
                let name = item.id.rsplit('/').next().unwrap_or(&item.id).to_string();

                let attrs = item.attributes.as_ref();
                CertificateItem {
                    name,
                    enabled: attrs.and_then(|a| a.enabled).unwrap_or(true),
                    expires: attrs.and_then(|a| a.expires),
                    not_before: attrs.and_then(|a| a.not_before),
                    created: attrs.and_then(|a| a.created),
                }
            })
            .collect();

        Ok(items)
    }

    /// Get the vault URL
    pub fn vault_url(&self) -> &str {
        &self.vault_url
    }

    /// Get the key ID from the last sign response
    pub fn decode_signature(response: &SignResponse) -> Result<Vec<u8>> {
        URL_SAFE_NO_PAD
            .decode(&response.value)
            .context("Failed to decode signature from Key Vault response")
    }

    /// Get the key ID from a sign response
    pub fn key_id(response: &SignResponse) -> &str {
        &response.kid
    }
}
