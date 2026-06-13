//! Thin blocking HTTP helpers for the monitoring stack.
//!
//! These wrap `reqwest::blocking` with a shared timeout and consistent error
//! mapping. Response parsing lives in `parse.rs` so it can be unit-tested
//! without touching the network.

use anyhow::{Context, Result, bail};
use reqwest::blocking::Client;
use std::time::Duration;

pub struct MonitorClient {
    client: Client,
}

impl MonitorClient {
    pub fn new(timeout_secs: u64) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .user_agent("ghostctl")
            .build()
            .context("failed to build HTTP client")?;
        Ok(Self { client })
    }

    /// GET a URL and return the body as text on 2xx, else error with status.
    pub fn get_text(&self, url: &str) -> Result<String> {
        let resp = self
            .client
            .get(url)
            .send()
            .with_context(|| format!("request failed: {url}"))?;
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        if !status.is_success() {
            bail!("HTTP {} from {}", status.as_u16(), url);
        }
        Ok(body)
    }

    /// GET with URL-encoded query parameters (used for Loki / Prometheus queries).
    pub fn get_text_query(&self, url: &str, params: &[(&str, &str)]) -> Result<String> {
        let full = reqwest::Url::parse_with_params(url, params.iter().copied())
            .with_context(|| format!("invalid URL: {url}"))?;
        let resp = self
            .client
            .get(full)
            .send()
            .with_context(|| format!("request failed: {url}"))?;
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        if !status.is_success() {
            bail!("HTTP {} from {}: {}", status.as_u16(), url, body.trim());
        }
        Ok(body)
    }

    /// GET with optional basic auth (used for Grafana datasource health).
    pub fn get_text_auth(
        &self,
        url: &str,
        user: Option<&str>,
        pass: Option<&str>,
    ) -> Result<String> {
        let mut req = self.client.get(url);
        if let Some(p) = pass {
            req = req.basic_auth(user.unwrap_or("admin"), Some(p));
        }
        let resp = req
            .send()
            .with_context(|| format!("request failed: {url}"))?;
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        if !status.is_success() {
            bail!("HTTP {} from {}", status.as_u16(), url);
        }
        Ok(body)
    }

    /// POST with an empty body (used for /-/reload endpoints).
    pub fn post_empty(&self, url: &str) -> Result<()> {
        let resp = self
            .client
            .post(url)
            .send()
            .with_context(|| format!("request failed: {url}"))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().unwrap_or_default();
            bail!("HTTP {} from {}: {}", status.as_u16(), url, body.trim());
        }
        Ok(())
    }

    /// Lightweight liveness probe: true if the URL returns any 2xx response.
    pub fn is_up(&self, url: &str) -> bool {
        self.client
            .get(url)
            .send()
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }
}
