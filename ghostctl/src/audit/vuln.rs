//! Dependency-vulnerability model, CVSS v3.1 scoring, and reporting.
//!
//! Shared by every language-ecosystem auditor (`audit cargo`, `audit node`,
//! `audit deps`). A `VulnFinding` is a normalized advisory hit produced from an
//! OSV record so the report looks the same whether the package came from
//! crates.io, npm, Go, or PyPI.

use serde::Serialize;

use crate::utils::is_plain_mode;

/// Normalized severity. Richer than the PKGBUILD scanner's High/Medium/Low
/// because vulnerability advisories distinguish Critical, and some carry no
/// score at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum VulnSeverity {
    // Order matters: derived Ord sorts low → high, so reports reverse it.
    Unknown,
    Low,
    Medium,
    High,
    Critical,
}

impl VulnSeverity {
    pub fn label(self) -> &'static str {
        match self {
            VulnSeverity::Critical => "CRITICAL",
            VulnSeverity::High => "HIGH",
            VulnSeverity::Medium => "MEDIUM",
            VulnSeverity::Low => "LOW",
            VulnSeverity::Unknown => "UNKNOWN",
        }
    }

    pub fn icon(self) -> &'static str {
        if is_plain_mode() {
            return "";
        }
        match self {
            VulnSeverity::Critical => "\u{1f534}", // red circle
            VulnSeverity::High => "\u{1f7e0}",     // orange circle
            VulnSeverity::Medium => "\u{1f7e1}",   // yellow circle
            VulnSeverity::Low => "\u{1f535}",      // blue circle
            VulnSeverity::Unknown => "\u{26aa}",   // white circle
        }
    }

    /// Map a textual severity (GitHub/OSV `database_specific.severity`) to a
    /// bucket. Accepts the common spellings.
    pub fn from_text(s: &str) -> Option<VulnSeverity> {
        match s.trim().to_ascii_uppercase().as_str() {
            "CRITICAL" => Some(VulnSeverity::Critical),
            "HIGH" => Some(VulnSeverity::High),
            "MODERATE" | "MEDIUM" => Some(VulnSeverity::Medium),
            "LOW" => Some(VulnSeverity::Low),
            _ => None,
        }
    }

    /// CVSS v3.x base score → severity bucket (per the CVSS v3.1 spec).
    pub fn from_cvss_score(score: f64) -> VulnSeverity {
        if score >= 9.0 {
            VulnSeverity::Critical
        } else if score >= 7.0 {
            VulnSeverity::High
        } else if score >= 4.0 {
            VulnSeverity::Medium
        } else if score > 0.0 {
            VulnSeverity::Low
        } else {
            VulnSeverity::Unknown
        }
    }
}

/// A single normalized advisory hit against a locked dependency.
#[derive(Debug, Clone, Serialize)]
pub struct VulnFinding {
    pub ecosystem: String,
    pub package: String,
    pub version: String,
    pub id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    pub severity: VulnSeverity,
    pub summary: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fixed: Vec<String>,
    pub url: String,
}

/// Count findings by severity for a summary line.
pub fn severity_counts(findings: &[VulnFinding]) -> (usize, usize, usize, usize, usize) {
    let mut crit = 0;
    let mut high = 0;
    let mut med = 0;
    let mut low = 0;
    let mut unk = 0;
    for f in findings {
        match f.severity {
            VulnSeverity::Critical => crit += 1,
            VulnSeverity::High => high += 1,
            VulnSeverity::Medium => med += 1,
            VulnSeverity::Low => low += 1,
            VulnSeverity::Unknown => unk += 1,
        }
    }
    (crit, high, med, low, unk)
}

/// True if any finding is High or Critical (used for CI exit codes).
pub fn has_high_or_critical(findings: &[VulnFinding]) -> bool {
    findings
        .iter()
        .any(|f| matches!(f.severity, VulnSeverity::High | VulnSeverity::Critical))
}

/// Sort findings most-severe first, then by ecosystem/package for stability.
pub fn sort_findings(findings: &mut [VulnFinding]) {
    findings.sort_by(|a, b| {
        b.severity
            .cmp(&a.severity)
            .then_with(|| a.ecosystem.cmp(&b.ecosystem))
            .then_with(|| a.package.cmp(&b.package))
            .then_with(|| a.id.cmp(&b.id))
    });
}

/// Print a human-readable report (plain-mode aware).
pub fn print_report(findings: &[VulnFinding]) {
    if findings.is_empty() {
        if is_plain_mode() {
            println!("[OK] No known vulnerabilities found.");
        } else {
            println!("\u{2705} No known vulnerabilities found.");
        }
        return;
    }

    for f in findings {
        let icon = f.severity.icon();
        let head = format!(
            "{} {} {}@{}",
            icon,
            f.severity.label(),
            f.package,
            f.version
        );
        println!("{}", head.trim_start());
        println!("    {}  [{}]", f.id, f.ecosystem);
        if !f.summary.is_empty() {
            println!("    {}", f.summary);
        }
        if !f.fixed.is_empty() {
            println!("    fixed in: {}", f.fixed.join(", "));
        }
        if !f.url.is_empty() {
            println!("    {}", f.url);
        }
        println!();
    }

    let (crit, high, med, low, unk) = severity_counts(findings);
    println!(
        "{} vulnerabilities: {} critical, {} high, {} medium, {} low, {} unknown",
        findings.len(),
        crit,
        high,
        med,
        low,
        unk
    );
}

/// Serialize findings to a pretty JSON document.
pub fn to_json(findings: &[VulnFinding]) -> String {
    let (crit, high, med, low, unk) = severity_counts(findings);
    let doc = serde_json::json!({
        "total": findings.len(),
        "summary": {
            "critical": crit,
            "high": high,
            "medium": med,
            "low": low,
            "unknown": unk,
        },
        "findings": findings,
    });
    serde_json::to_string_pretty(&doc).unwrap_or_else(|_| "{}".to_string())
}

/// Compute the CVSS v3.1 base score from a vector string, e.g.
/// `CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H`. Returns `None` if the vector
/// is missing required metrics. Implements the formula from the CVSS v3.1 spec.
pub fn cvss31_base_score(vector: &str) -> Option<f64> {
    let mut av = None;
    let mut ac = None;
    let mut pr = None;
    let mut ui = None;
    let mut scope_changed = None;
    let mut c = None;
    let mut i = None;
    let mut a = None;

    for part in vector.split('/') {
        let (k, v) = part.split_once(':')?;
        match k {
            "AV" => {
                av = Some(match v {
                    "N" => 0.85,
                    "A" => 0.62,
                    "L" => 0.55,
                    "P" => 0.2,
                    _ => return None,
                })
            }
            "AC" => {
                ac = Some(match v {
                    "L" => 0.77,
                    "H" => 0.44,
                    _ => return None,
                })
            }
            "PR" => pr = Some(v.to_string()),
            "UI" => {
                ui = Some(match v {
                    "N" => 0.85,
                    "R" => 0.62,
                    _ => return None,
                })
            }
            "S" => {
                scope_changed = Some(match v {
                    "U" => false,
                    "C" => true,
                    _ => return None,
                })
            }
            "C" => c = Some(impact_metric(v)?),
            "I" => i = Some(impact_metric(v)?),
            "A" => a = Some(impact_metric(v)?),
            _ => {}
        }
    }

    let av = av?;
    let ac = ac?;
    let ui = ui?;
    let scope_changed = scope_changed?;
    let c = c?;
    let i = i?;
    let a = a?;

    // Privileges Required depends on Scope.
    let pr = match pr?.as_str() {
        "N" => 0.85,
        "L" => {
            if scope_changed {
                0.68
            } else {
                0.62
            }
        }
        "H" => {
            if scope_changed {
                0.5
            } else {
                0.27
            }
        }
        _ => return None,
    };

    let isc_base = 1.0 - ((1.0 - c) * (1.0 - i) * (1.0 - a));
    let impact = if scope_changed {
        7.52 * (isc_base - 0.029) - 3.25 * (isc_base - 0.02).powi(15)
    } else {
        6.42 * isc_base
    };
    let exploitability = 8.22 * av * ac * pr * ui;

    let base = if impact <= 0.0 {
        0.0
    } else if scope_changed {
        roundup((1.08 * (impact + exploitability)).min(10.0))
    } else {
        roundup((impact + exploitability).min(10.0))
    };
    Some(base)
}

fn impact_metric(v: &str) -> Option<f64> {
    match v {
        "H" => Some(0.56),
        "L" => Some(0.22),
        "N" => Some(0.0),
        _ => None,
    }
}

/// CVSS "Roundup": round up to one decimal place per the spec's integer trick.
fn roundup(input: f64) -> f64 {
    let int_input = (input * 100_000.0).round() as i64;
    if int_input % 10_000 == 0 {
        int_input as f64 / 100_000.0
    } else {
        ((int_input as f64 / 10_000.0).floor() + 1.0) / 10.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_from_text() {
        assert_eq!(VulnSeverity::from_text("HIGH"), Some(VulnSeverity::High));
        assert_eq!(
            VulnSeverity::from_text("moderate"),
            Some(VulnSeverity::Medium)
        );
        assert_eq!(
            VulnSeverity::from_text("Critical"),
            Some(VulnSeverity::Critical)
        );
        assert_eq!(VulnSeverity::from_text("bogus"), None);
    }

    #[test]
    fn test_severity_ordering_critical_is_highest() {
        assert!(VulnSeverity::Critical > VulnSeverity::High);
        assert!(VulnSeverity::High > VulnSeverity::Medium);
        assert!(VulnSeverity::Low > VulnSeverity::Unknown);
    }

    #[test]
    fn test_cvss31_critical_vector() {
        // Known spec example: 9.8 (Critical).
        let s = cvss31_base_score("CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H").unwrap();
        assert!((s - 9.8).abs() < 1e-9, "got {s}");
        assert_eq!(VulnSeverity::from_cvss_score(s), VulnSeverity::Critical);
    }

    #[test]
    fn test_cvss31_scope_changed_vector() {
        // CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:C/C:H/I:H/A:H => 10.0 (Critical).
        let s = cvss31_base_score("CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:C/C:H/I:H/A:H").unwrap();
        assert!((s - 10.0).abs() < 1e-9, "got {s}");
        // UI:R lowers exploitability, dropping the same vector to 9.6.
        let s = cvss31_base_score("CVSS:3.1/AV:N/AC:L/PR:N/UI:R/S:C/C:H/I:H/A:H").unwrap();
        assert!((s - 9.6).abs() < 1e-9, "got {s}");
    }

    #[test]
    fn test_cvss31_medium_vector() {
        // CVSS:3.1/AV:L/AC:L/PR:L/UI:N/S:U/C:N/I:N/A:H => 5.5 (Medium).
        let s = cvss31_base_score("CVSS:3.1/AV:L/AC:L/PR:L/UI:N/S:U/C:N/I:N/A:H").unwrap();
        assert!((s - 5.5).abs() < 1e-9, "got {s}");
        assert_eq!(VulnSeverity::from_cvss_score(s), VulnSeverity::Medium);
    }

    #[test]
    fn test_cvss31_no_impact_is_zero() {
        let s = cvss31_base_score("CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:N/I:N/A:N").unwrap();
        assert_eq!(s, 0.0);
        assert_eq!(VulnSeverity::from_cvss_score(s), VulnSeverity::Unknown);
    }

    #[test]
    fn test_cvss31_invalid_vector_is_none() {
        assert!(cvss31_base_score("not-a-vector").is_none());
        assert!(cvss31_base_score("CVSS:3.1/AV:N").is_none());
    }

    #[test]
    fn test_sort_and_counts() {
        let mut f = vec![
            mk("a", VulnSeverity::Low),
            mk("b", VulnSeverity::Critical),
            mk("c", VulnSeverity::Medium),
        ];
        sort_findings(&mut f);
        assert_eq!(f[0].severity, VulnSeverity::Critical);
        assert_eq!(f[2].severity, VulnSeverity::Low);
        let (crit, high, med, low, unk) = severity_counts(&f);
        assert_eq!((crit, high, med, low, unk), (1, 0, 1, 1, 0));
        assert!(has_high_or_critical(&f));
    }

    fn mk(pkg: &str, sev: VulnSeverity) -> VulnFinding {
        VulnFinding {
            ecosystem: "crates.io".into(),
            package: pkg.into(),
            version: "1.0.0".into(),
            id: format!("RUSTSEC-0000-{pkg}"),
            aliases: vec![],
            severity: sev,
            summary: "test".into(),
            fixed: vec![],
            url: String::new(),
        }
    }
}
