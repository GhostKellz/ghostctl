//! Arch Security Tracker advisory parsing (pure, unit-testable).
//!
//! Source: <https://security.archlinux.org/json> - the list of Arch
//! Vulnerability Groups (AVGs). Each entry maps one or more packages to a set
//! of CVE issues, a status (Vulnerable / Testing / Fixed / Not affected), a
//! severity, and the version a fix landed in (if any).

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AvgEntry {
    /// Group id, e.g. "AVG-2701".
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub packages: Vec<String>,
    /// "Vulnerable", "Testing", "Fixed", "Not affected".
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub severity: String,
    /// Version the fix landed in, if any.
    #[serde(default)]
    pub fixed: Option<String>,
    /// Associated CVE identifiers.
    #[serde(default)]
    pub issues: Vec<String>,
}

impl AvgEntry {
    /// True if this advisory has no released fix yet.
    pub fn is_unfixed(&self) -> bool {
        self.fixed.as_deref().unwrap_or("").is_empty()
            || self.status.eq_ignore_ascii_case("Vulnerable")
    }
}

/// Parse the Arch Security Tracker `/json` array.
pub fn parse_tracker(json: &str) -> Result<Vec<AvgEntry>> {
    serde_json::from_str(json).context("failed to parse Arch Security Tracker JSON")
}

/// Numeric rank for sorting/highlighting severities (higher = worse).
pub fn severity_rank(severity: &str) -> u8 {
    match severity.to_lowercase().as_str() {
        "critical" => 4,
        "high" => 3,
        "medium" => 2,
        "low" => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"[
      {"name":"AVG-2701","packages":["chromium"],"status":"Vulnerable",
       "severity":"Critical","fixed":null,"issues":["CVE-2024-0001","CVE-2024-0002"]},
      {"name":"AVG-2702","packages":["curl","libcurl"],"status":"Fixed",
       "severity":"High","fixed":"8.7.1-1","issues":["CVE-2024-1234"]}
    ]"#;

    #[test]
    fn test_parse_tracker() {
        let entries = parse_tracker(SAMPLE).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name, "AVG-2701");
        assert_eq!(entries[0].packages, vec!["chromium"]);
        assert!(entries[0].is_unfixed());
        assert_eq!(entries[1].fixed.as_deref(), Some("8.7.1-1"));
        assert!(!entries[1].is_unfixed());
    }

    #[test]
    fn test_severity_rank() {
        assert!(severity_rank("Critical") > severity_rank("High"));
        assert!(severity_rank("High") > severity_rank("Medium"));
        assert!(severity_rank("Low") > severity_rank("Unknown"));
        assert_eq!(severity_rank("whatever"), 0);
    }
}
