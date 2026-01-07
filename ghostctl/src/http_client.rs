//! HTTP client with retry logic, exponential backoff, and fallback mirrors
//!
//! This module provides a robust HTTP client for fetching remote resources
//! with automatic retry on failure and configurable fallback mirrors.

use crate::config::GhostConfig;
use crate::logging::GhostLogger;
use anyhow::{Context, Result};
use reqwest::blocking::{Client, Response};
use reqwest::StatusCode;
use std::thread;
use std::time::Duration;

/// Default retry configuration
const DEFAULT_MAX_RETRIES: u32 = 4;
const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Retry delays in seconds (exponential backoff: 0, 1, 3, 10)
const RETRY_DELAYS: [u64; 4] = [0, 1, 3, 10];

/// User agent string for requests
const USER_AGENT: &str = "ghostctl/1.0 (+https://github.com/ghostkellz/ghostctl)";

/// Default fallback mirrors for GitHub content
const DEFAULT_FALLBACK_MIRRORS: &[&str] = &[
    "https://raw.githubusercontent.com",
    "https://cdn.jsdelivr.net/gh",
];

/// HTTP client with retry logic and fallback support
pub struct RobustHttpClient {
    client: Client,
    max_retries: u32,
    timeout: Duration,
    fallback_mirrors: Vec<String>,
}

impl RobustHttpClient {
    /// Create a new client with default settings
    pub fn new() -> Result<Self> {
        Self::with_config(None)
    }

    /// Create a new client with optional config override
    pub fn with_config(config: Option<&GhostConfig>) -> Result<Self> {
        let (timeout_secs, max_retries, fallback_mirrors) = if let Some(cfg) = config {
            (
                cfg.mirrors
                    .as_ref()
                    .map(|m| m.github_api_timeout)
                    .unwrap_or(DEFAULT_TIMEOUT_SECS),
                cfg.mirrors
                    .as_ref()
                    .map(|m| m.retry_attempts)
                    .unwrap_or(DEFAULT_MAX_RETRIES),
                cfg.mirrors
                    .as_ref()
                    .map(|m| m.fallback_mirrors.clone())
                    .unwrap_or_else(|| {
                        DEFAULT_FALLBACK_MIRRORS
                            .iter()
                            .map(|s| s.to_string())
                            .collect()
                    }),
            )
        } else {
            (
                DEFAULT_TIMEOUT_SECS,
                DEFAULT_MAX_RETRIES,
                DEFAULT_FALLBACK_MIRRORS
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            )
        };

        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .user_agent(USER_AGENT)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            max_retries,
            timeout: Duration::from_secs(timeout_secs),
            fallback_mirrors,
        })
    }

    /// Fetch a URL with automatic retry and exponential backoff
    pub fn fetch(&self, url: &str) -> Result<String> {
        let mut last_error: Option<anyhow::Error> = None;

        for attempt in 0..self.max_retries {
            // Apply delay (exponential backoff)
            if attempt > 0 {
                let delay = RETRY_DELAYS.get(attempt as usize).copied().unwrap_or(10);
                GhostLogger::log_action(
                    "http_retry",
                    true,
                    Some(&format!(
                        "attempt:{} delay:{}s url:{}",
                        attempt + 1,
                        delay,
                        url
                    )),
                );
                thread::sleep(Duration::from_secs(delay));
            }

            match self.try_fetch(url) {
                Ok(content) => {
                    if attempt > 0 {
                        GhostLogger::log_action(
                            "http_fetch",
                            true,
                            Some(&format!("url:{} attempts:{}", url, attempt + 1)),
                        );
                    }
                    return Ok(content);
                }
                Err(e) => {
                    // Check for rate limiting - skip to fallback immediately
                    if let Some(status) = extract_status_code(&e)
                        && (status == StatusCode::FORBIDDEN
                            || status == StatusCode::TOO_MANY_REQUESTS)
                        {
                            GhostLogger::log_action(
                                "http_rate_limited",
                                false,
                                Some(&format!("url:{} status:{}", url, status.as_u16())),
                            );
                            break; // Skip remaining retries, try fallback
                        }
                    last_error = Some(e);
                }
            }
        }

        // Try fallback mirrors if this is a GitHub URL
        if is_github_url(url)
            && let Some(content) = self.try_fallback_mirrors(url) {
                return Ok(content);
            }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Failed to fetch URL after all retries")))
    }

    /// Single fetch attempt
    fn try_fetch(&self, url: &str) -> Result<String> {
        let response = self
            .client
            .get(url)
            .send()
            .with_context(|| format!("HTTP request failed for {}", url))?;

        let status = response.status();
        if !status.is_success() {
            anyhow::bail!("HTTP error {} for {}", status, url);
        }

        response.text().context("Failed to read response body")
    }

    /// Attempt to fetch from fallback mirrors
    fn try_fallback_mirrors(&self, original_url: &str) -> Option<String> {
        for mirror in &self.fallback_mirrors {
            if let Some(mirror_url) = convert_to_mirror_url(original_url, mirror) {
                GhostLogger::log_action(
                    "http_fallback",
                    true,
                    Some(&format!("original:{} mirror:{}", original_url, mirror_url)),
                );

                match self.try_fetch(&mirror_url) {
                    Ok(content) => {
                        GhostLogger::log_action(
                            "http_fallback_success",
                            true,
                            Some(&format!("mirror:{}", mirror_url)),
                        );
                        return Some(content);
                    }
                    Err(e) => {
                        GhostLogger::log_action(
                            "http_fallback_failed",
                            false,
                            Some(&format!("mirror:{} error:{}", mirror_url, e)),
                        );
                        continue;
                    }
                }
            }
        }
        None
    }

    /// Fetch with response (for cases where you need headers/status)
    pub fn fetch_response(&self, url: &str) -> Result<Response> {
        let mut last_error: Option<anyhow::Error> = None;

        for attempt in 0..self.max_retries {
            if attempt > 0 {
                let delay = RETRY_DELAYS.get(attempt as usize).copied().unwrap_or(10);
                thread::sleep(Duration::from_secs(delay));
            }

            match self.client.get(url).send() {
                Ok(response) if response.status().is_success() => {
                    return Ok(response);
                }
                Ok(response) => {
                    let status = response.status();
                    if status == StatusCode::FORBIDDEN || status == StatusCode::TOO_MANY_REQUESTS {
                        break; // Rate limited, skip to fallback
                    }
                    last_error = Some(anyhow::anyhow!("HTTP error: {}", status));
                }
                Err(e) => {
                    last_error = Some(e.into());
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Failed to fetch URL")))
    }
}

impl Default for RobustHttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default HTTP client")
    }
}

/// Check if a URL is a GitHub URL
fn is_github_url(url: &str) -> bool {
    url.contains("github.com")
        || url.contains("githubusercontent.com")
        || url.contains("api.github.com")
}

/// Convert a GitHub URL to a mirror URL
fn convert_to_mirror_url(original: &str, mirror_base: &str) -> Option<String> {
    // Handle raw.githubusercontent.com URLs
    // Example: https://raw.githubusercontent.com/user/repo/branch/path
    if original.contains("raw.githubusercontent.com") {
        if mirror_base.contains("jsdelivr") {
            // Convert to jsDelivr format: https://cdn.jsdelivr.net/gh/user/repo@branch/path
            let parts: Vec<&str> = original.split('/').collect();
            if parts.len() >= 7 {
                let user = parts[3];
                let repo = parts[4];
                let branch = parts[5];
                let path = parts[6..].join("/");
                return Some(format!(
                    "{}/{}/{}@{}/{}",
                    mirror_base, user, repo, branch, path
                ));
            }
        } else if mirror_base.contains("raw.githubusercontent.com") {
            // Already raw, just return original
            return Some(original.to_string());
        }
    }

    // Handle api.github.com URLs - convert to raw
    if original.contains("api.github.com") && mirror_base.contains("raw.githubusercontent.com") {
        // Example: https://api.github.com/repos/user/repo/contents/path?ref=branch
        // Convert to: https://raw.githubusercontent.com/user/repo/branch/path
        if let Some(captures) = parse_github_api_url(original) {
            return Some(format!(
                "{}/{}/{}/{}/{}",
                mirror_base, captures.user, captures.repo, captures.branch, captures.path
            ));
        }
    }

    // Handle github.com blob URLs
    if original.contains("github.com") && original.contains("/blob/") {
        // Example: https://github.com/user/repo/blob/branch/path
        // Convert to: https://raw.githubusercontent.com/user/repo/branch/path
        let converted = original
            .replace("github.com", "raw.githubusercontent.com")
            .replace("/blob/", "/");

        if mirror_base.contains("raw.githubusercontent.com") {
            return Some(converted);
        } else if mirror_base.contains("jsdelivr") {
            // Further convert to jsDelivr
            return convert_to_mirror_url(&converted, mirror_base);
        }
    }

    None
}

/// Parsed GitHub API URL components
struct GitHubApiParts {
    user: String,
    repo: String,
    branch: String,
    path: String,
}

/// Parse a GitHub API URL into components
fn parse_github_api_url(url: &str) -> Option<GitHubApiParts> {
    // Pattern: https://api.github.com/repos/{user}/{repo}/contents/{path}?ref={branch}
    let url_without_query = url.split('?').next()?;
    let parts: Vec<&str> = url_without_query.split('/').collect();

    // Find repos index
    let repos_idx = parts.iter().position(|&p| p == "repos")?;

    if parts.len() > repos_idx + 4 {
        let user = parts[repos_idx + 1].to_string();
        let repo = parts[repos_idx + 2].to_string();
        // Skip "contents"
        let path = parts[repos_idx + 4..].join("/");

        // Extract branch from query string
        let branch = url
            .split("ref=")
            .nth(1)
            .map(|s| s.split('&').next().unwrap_or("main"))
            .unwrap_or("main")
            .to_string();

        return Some(GitHubApiParts {
            user,
            repo,
            branch,
            path,
        });
    }

    None
}

/// Extract status code from an error (if present)
fn extract_status_code(err: &anyhow::Error) -> Option<StatusCode> {
    // Try to extract status code from error message
    let msg = err.to_string();
    if msg.contains("403") {
        Some(StatusCode::FORBIDDEN)
    } else if msg.contains("429") {
        Some(StatusCode::TOO_MANY_REQUESTS)
    } else if msg.contains("404") {
        Some(StatusCode::NOT_FOUND)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_github_url() {
        assert!(is_github_url("https://github.com/user/repo"));
        assert!(is_github_url(
            "https://raw.githubusercontent.com/user/repo/main/file"
        ));
        assert!(is_github_url("https://api.github.com/repos/user/repo"));
        assert!(!is_github_url("https://example.com/file"));
    }

    #[test]
    fn test_convert_raw_to_jsdelivr() {
        let original = "https://raw.githubusercontent.com/user/repo/main/path/to/file.sh";
        let mirror = "https://cdn.jsdelivr.net/gh";
        let result = convert_to_mirror_url(original, mirror);
        assert_eq!(
            result,
            Some("https://cdn.jsdelivr.net/gh/user/repo@main/path/to/file.sh".to_string())
        );
    }

    #[test]
    fn test_convert_blob_to_raw() {
        let original = "https://github.com/user/repo/blob/main/file.sh";
        let mirror = "https://raw.githubusercontent.com";
        let result = convert_to_mirror_url(original, mirror);
        assert_eq!(
            result,
            Some("https://raw.githubusercontent.com/user/repo/main/file.sh".to_string())
        );
    }
}
