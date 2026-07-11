use anyhow::{Context, Result, bail};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use reqwest::Url;
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

        let vault_url = normalize_vault_url(vault_url)?;

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

        let mut segments = vec!["keys", key_name];
        if let Some(version) = key_version.filter(|v| !v.is_empty()) {
            segments.push(version);
        }
        segments.push("sign");
        let url = self.api_url(&segments)?;

        let body = SignRequest {
            alg: algorithm.to_string(),
            value: URL_SAFE_NO_PAD.encode(digest),
        };

        self.post_with_retry(url, &body)
    }

    /// Get the X.509 certificate (DER-encoded) for a cert name
    pub fn get_certificate(&mut self, cert_name: &str, version: Option<&str>) -> Result<Vec<u8>> {
        self.ensure_token()?;

        let mut segments = vec!["certificates", cert_name];
        if let Some(version) = version.filter(|v| !v.is_empty()) {
            segments.push(version);
        }
        let url = self.api_url(&segments)?;

        let response: CertificateBundle = self.get_with_retry(url)?;
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

        let url = self.api_url(&["keys", key_name])?;

        let bundle: KeyBundle = self.get_with_retry(url)?;

        Ok(KeyInfo {
            kid: bundle.key.kid,
            key_type: bundle.key.kty,
            enabled: bundle.attributes.enabled,
        })
    }

    /// POST request with retry and re-auth on 401
    fn post_with_retry<T: for<'de> Deserialize<'de>, B: Serialize>(
        &mut self,
        url: Url,
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
                .post(url.clone())
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
    fn get_with_retry<T: for<'de> Deserialize<'de>>(&mut self, url: Url) -> Result<T> {
        let mut last_error: Option<anyhow::Error> = None;

        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                let delay = RETRY_DELAYS.get(attempt as usize).copied().unwrap_or(10);
                thread::sleep(Duration::from_secs(delay));
            }

            let response = self
                .http
                .get(url.clone())
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

        let url = self.api_url(&["certificates"])?;

        let response: CertificateListResponse = self.get_with_retry(url)?;

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

    fn api_url(&self, segments: &[&str]) -> Result<Url> {
        let mut url = Url::parse(&self.vault_url).context("Invalid Key Vault URL")?;
        {
            let mut path = url
                .path_segments_mut()
                .map_err(|_| anyhow::anyhow!("Key Vault URL cannot be used as a base URL"))?;
            path.clear();
            for segment in segments {
                path.push(segment);
            }
        }
        url.query_pairs_mut()
            .clear()
            .append_pair("api-version", API_VERSION);
        Ok(url)
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

fn normalize_vault_url(vault_url: &str) -> Result<String> {
    let mut url = Url::parse(vault_url.trim()).context("Invalid Azure Key Vault URL")?;

    if url.scheme() != "https" {
        bail!("Azure Key Vault URL must use https");
    }

    if url.username() != "" || url.password().is_some() {
        bail!("Azure Key Vault URL must not include credentials");
    }

    let host = url
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("Azure Key Vault URL must include a host"))?;

    let allowed_suffixes = [
        ".vault.azure.net",
        ".vault.azure.cn",
        ".vault.usgovcloudapi.net",
        ".managedhsm.azure.net",
        ".managedhsm.azure.cn",
        ".managedhsm.usgovcloudapi.net",
    ];

    if !allowed_suffixes.iter().any(|suffix| host.ends_with(suffix)) {
        bail!("Azure Key Vault URL host is not a recognized Azure vault endpoint");
    }

    url.set_path("");
    url.set_query(None);
    url.set_fragment(None);
    Ok(url.as_str().trim_end_matches('/').to_string())
}

#[cfg(test)]
mod tests {
    use super::normalize_vault_url;

    #[test]
    fn normalizes_azure_vault_url() {
        let url = normalize_vault_url("https://example.vault.azure.net/path?x=1#frag").unwrap();
        assert_eq!(url, "https://example.vault.azure.net");
    }

    #[test]
    fn rejects_non_https_vault_url() {
        assert!(normalize_vault_url("http://example.vault.azure.net").is_err());
    }

    #[test]
    fn rejects_non_azure_vault_url() {
        assert!(normalize_vault_url("https://example.com").is_err());
    }
}
