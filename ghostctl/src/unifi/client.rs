//! Blocking HTTP client for a self-hosted UniFi OS Server controller.
//!
//! Auth is via the `X-API-KEY` header against the documented Network
//! Integration API (base `/proxy/network/integration/v1`). A few deeper
//! diagnostics need the *private* stats API (`/proxy/network/api/s/{site}`);
//! those are clearly marked and callers must treat them as best-effort since
//! Ubiquiti does not guarantee that surface across UOS upgrades.

use anyhow::{Context, Result, bail};
use reqwest::blocking::Client;
use serde_json::Value;
use std::time::Duration;

use super::config::UnifiConfig;

pub struct UnifiClient {
    client: Client,
    base: String,
    site: String,
}

impl UnifiClient {
    pub fn new(cfg: &UnifiConfig) -> Result<Self> {
        let api_key = cfg.resolve_api_key().context(
            "no UniFi API key configured (set UNIFI_API_KEY or [unifi].api_key). \
             Generate one in the Network app under Settings -> Integrations.",
        )?;

        let mut headers = reqwest::header::HeaderMap::new();
        let mut key_val = reqwest::header::HeaderValue::from_str(&api_key)
            .context("API key contains invalid header characters")?;
        key_val.set_sensitive(true);
        headers.insert("X-API-KEY", key_val);
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let client = Client::builder()
            .timeout(Duration::from_secs(cfg.timeout_secs))
            .user_agent("ghostctl")
            .default_headers(headers)
            // Self-hosted controllers ship a self-signed cert; only verify when
            // the operator has opted in via [unifi].verify_tls = true.
            .danger_accept_invalid_certs(!cfg.verify_tls)
            .build()
            .context("failed to build UniFi HTTP client")?;

        let base = cfg
            .parsed_base_url()
            .map_err(anyhow::Error::msg)?
            .to_string()
            .trim_end_matches('/')
            .to_string();

        Ok(Self {
            client,
            base,
            site: cfg.site.clone(),
        })
    }

    fn integration(&self, path: &str) -> String {
        format!("{}/proxy/network/integration/v1{}", self.base, path)
    }

    fn get_json(&self, url: &str) -> Result<Value> {
        let resp = self
            .client
            .get(url)
            .send()
            .with_context(|| format!("request failed: {url}"))?;
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        if status.as_u16() == 401 || status.as_u16() == 403 {
            bail!(
                "HTTP {} from {} — API key rejected. Check the key and that it \
                 was created on this controller (Settings -> Integrations).",
                status.as_u16(),
                url
            );
        }
        if !status.is_success() {
            bail!("HTTP {} from {}: {}", status.as_u16(), url, body.trim());
        }
        serde_json::from_str(&body).with_context(|| format!("invalid JSON from {url}"))
    }

    /// List sites via the documented integration API. Doubles as an auth/liveness probe.
    pub fn list_sites(&self) -> Result<Value> {
        self.get_json(&self.integration("/sites"))
    }

    /// Resolve the internal site id for the configured site name.
    pub fn resolve_site_id(&self) -> Result<String> {
        let sites = self.list_sites()?;
        let items = sites
            .get("data")
            .and_then(|d| d.as_array())
            .context("unexpected /sites response shape")?;
        // Match on the human name; fall back to the first site if only one exists.
        for s in items {
            let name = s.get("name").and_then(|n| n.as_str()).unwrap_or("");
            if name.eq_ignore_ascii_case(&self.site)
                && let Some(id) = s.get("id").and_then(|i| i.as_str())
            {
                return Ok(id.to_string());
            }
        }
        if items.len() == 1
            && let Some(id) = items[0].get("id").and_then(|i| i.as_str())
        {
            return Ok(id.to_string());
        }
        bail!("site '{}' not found on controller", self.site)
    }

    /// List devices for a site via the documented integration API.
    pub fn list_devices(&self, site_id: &str) -> Result<Value> {
        self.get_json(&self.integration(&format!("/sites/{site_id}/devices")))
    }

    /// Deep per-device stats including `port_table` (STP state, errors, uplinks).
    ///
    /// PRIVATE API — not guaranteed stable across UOS upgrades. Callers should
    /// degrade gracefully if this errors.
    pub fn stat_device(&self) -> Result<Value> {
        let url = format!(
            "{}/proxy/network/api/s/{}/stat/device",
            self.base, self.site
        );
        self.get_json(&url)
    }
}
