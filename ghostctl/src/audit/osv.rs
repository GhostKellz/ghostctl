//! OSV.dev client: query locked dependencies for known vulnerabilities.
//!
//! OSV aggregates advisory databases across ecosystems — including **RustSec**
//! (`crates.io`), GitHub Advisories (`npm`, `PyPI`, ...), and the Go vuln DB —
//! behind one schema. We batch-query the locked package set, then fetch full
//! records only for the (usually few) packages that have hits, and normalize
//! each into a `VulnFinding`. No external audit binary required.

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::{Value, json};
use std::collections::BTreeMap;

use super::lockfile::Package;
use super::vuln::{VulnFinding, VulnSeverity, cvss31_base_score};

const QUERYBATCH_URL: &str = "https://api.osv.dev/v1/querybatch";
const VULN_URL: &str = "https://api.osv.dev/v1/vulns";
/// OSV caps querybatch at 1000 queries; stay well under it.
const BATCH_SIZE: usize = 500;

/// Audit a set of locked packages against OSV. Returns normalized findings.
pub fn audit_packages(client: &Client, packages: &[Package]) -> Result<Vec<VulnFinding>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }

    // 1. Batched querybatch → list of vuln ids per package (index-aligned).
    let mut ids_per_pkg: Vec<Vec<String>> = Vec::with_capacity(packages.len());
    for chunk in packages.chunks(BATCH_SIZE) {
        let body = json!({
            "queries": chunk.iter().map(|p| json!({
                "version": p.version,
                "package": { "name": p.name, "ecosystem": p.ecosystem },
            })).collect::<Vec<_>>()
        });
        let resp = client
            .post(QUERYBATCH_URL)
            .json(&body)
            .send()
            .context("OSV querybatch request failed")?;
        if !resp.status().is_success() {
            anyhow::bail!("OSV querybatch returned HTTP {}", resp.status().as_u16());
        }
        let parsed: Value = resp
            .json()
            .context("OSV querybatch returned invalid JSON")?;
        ids_per_pkg.extend(parse_querybatch_ids(&parsed, chunk.len()));
    }

    // 2. Fetch each unique vuln record once.
    let mut records: BTreeMap<String, Value> = BTreeMap::new();
    for ids in &ids_per_pkg {
        for id in ids {
            if records.contains_key(id) {
                continue;
            }
            if let Some(rec) = fetch_vuln(client, id)? {
                records.insert(id.clone(), rec);
            }
        }
    }

    // 3. Build a normalized finding per (package, vuln).
    let mut findings = Vec::new();
    for (pkg, ids) in packages.iter().zip(ids_per_pkg.iter()) {
        for id in ids {
            let Some(record) = records.get(id) else {
                continue;
            };
            findings.push(VulnFinding {
                ecosystem: pkg.ecosystem.clone(),
                package: pkg.name.clone(),
                version: pkg.version.clone(),
                id: id.clone(),
                aliases: extract_aliases(record),
                severity: derive_severity(record),
                summary: extract_summary(record),
                fixed: extract_fixed(record, &pkg.name),
                url: extract_url(record, id),
            });
        }
    }
    Ok(findings)
}

fn fetch_vuln(client: &Client, id: &str) -> Result<Option<Value>> {
    let resp = client
        .get(format!("{VULN_URL}/{id}"))
        .send()
        .with_context(|| format!("OSV vuln request failed: {id}"))?;
    if resp.status().as_u16() == 404 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        anyhow::bail!("OSV vuln {id} returned HTTP {}", resp.status().as_u16());
    }
    Ok(Some(resp.json().context("OSV vuln returned invalid JSON")?))
}

/// Extract the per-query vuln-id lists from a querybatch response. Always
/// returns `expected` rows so indices stay aligned with the input chunk.
pub fn parse_querybatch_ids(resp: &Value, expected: usize) -> Vec<Vec<String>> {
    let mut out = Vec::with_capacity(expected);
    let results = resp.get("results").and_then(Value::as_array);
    for idx in 0..expected {
        let ids = results
            .and_then(|r| r.get(idx))
            .and_then(|entry| entry.get("vulns"))
            .and_then(Value::as_array)
            .map(|vulns| {
                vulns
                    .iter()
                    .filter_map(|v| v.get("id").and_then(Value::as_str))
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default();
        out.push(ids);
    }
    out
}

/// Derive a severity bucket from an OSV record, preferring an explicit textual
/// severity, then a CVSS v3 vector, else Unknown.
pub fn derive_severity(record: &Value) -> VulnSeverity {
    // Top-level database_specific.severity (GitHub Advisories set this).
    if let Some(sev) = record
        .pointer("/database_specific/severity")
        .and_then(Value::as_str)
        .and_then(VulnSeverity::from_text)
    {
        return sev;
    }
    // Any affected[].database_specific.severity.
    if let Some(affected) = record.get("affected").and_then(Value::as_array) {
        for a in affected {
            if let Some(sev) = a
                .pointer("/database_specific/severity")
                .and_then(Value::as_str)
                .and_then(VulnSeverity::from_text)
            {
                return sev;
            }
        }
    }
    // CVSS v3 vector in the severity[] array (RustSec provides this).
    if let Some(sevs) = record.get("severity").and_then(Value::as_array) {
        for s in sevs {
            let ty = s.get("type").and_then(Value::as_str).unwrap_or("");
            if ty.starts_with("CVSS_V3")
                && let Some(score) = s
                    .get("score")
                    .and_then(Value::as_str)
                    .and_then(cvss31_base_score)
            {
                return VulnSeverity::from_cvss_score(score);
            }
        }
    }
    VulnSeverity::Unknown
}

/// Collect fixed versions from the `affected` ranges for the given package.
pub fn extract_fixed(record: &Value, package: &str) -> Vec<String> {
    let mut fixed = Vec::new();
    let Some(affected) = record.get("affected").and_then(Value::as_array) else {
        return fixed;
    };
    for a in affected {
        let name = a.pointer("/package/name").and_then(Value::as_str);
        if name.is_some() && name != Some(package) {
            continue;
        }
        if let Some(ranges) = a.get("ranges").and_then(Value::as_array) {
            for r in ranges {
                if let Some(events) = r.get("events").and_then(Value::as_array) {
                    for e in events {
                        if let Some(v) = e.get("fixed").and_then(Value::as_str) {
                            fixed.push(v.to_string());
                        }
                    }
                }
            }
        }
    }
    fixed.sort();
    fixed.dedup();
    fixed
}

pub fn extract_aliases(record: &Value) -> Vec<String> {
    record
        .get("aliases")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

pub fn extract_summary(record: &Value) -> String {
    record
        .get("summary")
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_default()
}

/// Pick the best advisory URL: a reference typed ADVISORY, else the first
/// reference, else the canonical osv.dev page.
pub fn extract_url(record: &Value, id: &str) -> String {
    if let Some(refs) = record.get("references").and_then(Value::as_array) {
        if let Some(adv) = refs
            .iter()
            .find(|r| r.get("type").and_then(Value::as_str) == Some("ADVISORY"))
            && let Some(u) = adv.get("url").and_then(Value::as_str)
        {
            return u.to_string();
        }
        if let Some(u) = refs
            .first()
            .and_then(|r| r.get("url"))
            .and_then(Value::as_str)
        {
            return u.to_string();
        }
    }
    format!("https://osv.dev/vulnerability/{id}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_querybatch_ids_aligns_and_fills() {
        let resp = json!({
            "results": [
                { "vulns": [ {"id":"RUSTSEC-2021-0001","modified":"x"} ] },
                {},
                { "vulns": [ {"id":"GHSA-aaa"}, {"id":"CVE-2020-1"} ] }
            ]
        });
        let ids = parse_querybatch_ids(&resp, 3);
        assert_eq!(ids[0], vec!["RUSTSEC-2021-0001"]);
        assert!(ids[1].is_empty());
        assert_eq!(ids[2], vec!["GHSA-aaa", "CVE-2020-1"]);
    }

    #[test]
    fn test_parse_querybatch_fills_missing_rows() {
        let resp = json!({ "results": [ {} ] });
        let ids = parse_querybatch_ids(&resp, 3);
        assert_eq!(ids.len(), 3);
        assert!(ids.iter().all(Vec::is_empty));
    }

    #[test]
    fn test_derive_severity_prefers_text() {
        let rec = json!({ "database_specific": { "severity": "HIGH" } });
        assert_eq!(derive_severity(&rec), VulnSeverity::High);
    }

    #[test]
    fn test_derive_severity_from_cvss() {
        let rec = json!({
            "severity": [
                { "type": "CVSS_V3", "score": "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H" }
            ]
        });
        assert_eq!(derive_severity(&rec), VulnSeverity::Critical);
    }

    #[test]
    fn test_derive_severity_unknown_when_absent() {
        let rec = json!({ "summary": "no severity here" });
        assert_eq!(derive_severity(&rec), VulnSeverity::Unknown);
    }

    #[test]
    fn test_extract_fixed_filters_by_package() {
        let rec = json!({
            "affected": [
                {
                    "package": { "name": "foo", "ecosystem": "crates.io" },
                    "ranges": [ { "type": "SEMVER", "events": [
                        { "introduced": "0" }, { "fixed": "1.2.4" }
                    ] } ]
                },
                {
                    "package": { "name": "other", "ecosystem": "crates.io" },
                    "ranges": [ { "events": [ { "fixed": "9.9.9" } ] } ]
                }
            ]
        });
        assert_eq!(extract_fixed(&rec, "foo"), vec!["1.2.4"]);
    }

    #[test]
    fn test_extract_url_prefers_advisory() {
        let rec = json!({
            "references": [
                { "type": "WEB", "url": "https://example.com/web" },
                { "type": "ADVISORY", "url": "https://rustsec.org/advisories/x" }
            ]
        });
        assert_eq!(
            extract_url(&rec, "RUSTSEC-x"),
            "https://rustsec.org/advisories/x"
        );
    }

    #[test]
    fn test_extract_url_fallback_to_osv() {
        let rec = json!({});
        assert_eq!(
            extract_url(&rec, "RUSTSEC-x"),
            "https://osv.dev/vulnerability/RUSTSEC-x"
        );
    }
}
