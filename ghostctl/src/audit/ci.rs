//! `audit ci` - offline auditor for CI/CD workflow files.
//!
//! Scans GitHub Actions workflows (`.github/workflows/*.yml`) and GitLab CI
//! (`.gitlab-ci.yml`) for deprecated/outdated constructs against a curated,
//! deterministic table. No network and no YAML dependency: workflow files are
//! line-scanned, so results are reproducible and unit-testable. Findings are
//! advisory (heuristics flag suspicion, not proof).

use anyhow::{Result, bail};
use serde::Serialize;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};

use super::vuln::VulnSeverity;

/// A single CI/CD workflow finding.
#[derive(Debug, Clone, Serialize)]
pub struct CiFinding {
    pub file: String,
    pub line: usize,
    pub severity: VulnSeverity,
    pub rule: &'static str,
    pub message: String,
    pub remediation: String,
}

/// Curated minimum-recommended major versions for common GitHub Actions.
/// `recommended` is the current major; `eol` is the highest major considered
/// hard-deprecated (High severity); majors between `eol` and `recommended` are
/// merely outdated (Medium).
struct ActionPolicy {
    repo: &'static str,
    recommended: u32,
    eol: u32,
}

const ACTION_POLICIES: &[ActionPolicy] = &[
    ActionPolicy {
        repo: "actions/checkout",
        recommended: 4,
        eol: 2,
    },
    ActionPolicy {
        repo: "actions/setup-node",
        recommended: 4,
        eol: 2,
    },
    ActionPolicy {
        repo: "actions/setup-python",
        recommended: 5,
        eol: 3,
    },
    ActionPolicy {
        repo: "actions/setup-go",
        recommended: 5,
        eol: 3,
    },
    ActionPolicy {
        repo: "actions/setup-java",
        recommended: 4,
        eol: 2,
    },
    ActionPolicy {
        repo: "actions/setup-dotnet",
        recommended: 4,
        eol: 2,
    },
    ActionPolicy {
        repo: "actions/upload-artifact",
        recommended: 4,
        eol: 3,
    },
    ActionPolicy {
        repo: "actions/download-artifact",
        recommended: 4,
        eol: 3,
    },
    ActionPolicy {
        repo: "actions/cache",
        recommended: 4,
        eol: 2,
    },
    ActionPolicy {
        repo: "actions/github-script",
        recommended: 7,
        eol: 5,
    },
    ActionPolicy {
        repo: "actions/stale",
        recommended: 9,
        eol: 7,
    },
    ActionPolicy {
        repo: "actions/labeler",
        recommended: 5,
        eol: 3,
    },
    ActionPolicy {
        repo: "actions/configure-pages",
        recommended: 5,
        eol: 3,
    },
    ActionPolicy {
        repo: "actions/deploy-pages",
        recommended: 4,
        eol: 2,
    },
    ActionPolicy {
        repo: "actions/upload-pages-artifact",
        recommended: 3,
        eol: 1,
    },
    ActionPolicy {
        repo: "github/codeql-action",
        recommended: 3,
        eol: 1,
    },
    ActionPolicy {
        repo: "docker/build-push-action",
        recommended: 6,
        eol: 4,
    },
    ActionPolicy {
        repo: "docker/login-action",
        recommended: 3,
        eol: 1,
    },
    ActionPolicy {
        repo: "docker/setup-buildx-action",
        recommended: 3,
        eol: 1,
    },
    ActionPolicy {
        repo: "docker/setup-qemu-action",
        recommended: 3,
        eol: 1,
    },
    ActionPolicy {
        repo: "docker/metadata-action",
        recommended: 5,
        eol: 3,
    },
    ActionPolicy {
        repo: "codecov/codecov-action",
        recommended: 4,
        eol: 2,
    },
    ActionPolicy {
        repo: "softprops/action-gh-release",
        recommended: 2,
        eol: 0,
    },
    ActionPolicy {
        repo: "peter-evans/create-pull-request",
        recommended: 6,
        eol: 4,
    },
    ActionPolicy {
        repo: "Swatinem/rust-cache",
        recommended: 2,
        eol: 0,
    },
];

/// Deprecated GitHub Actions runner workflow commands and their replacements.
/// `set-env`/`add-path` were disabled for security (High); `set-output`/
/// `save-state` are deprecated (Medium).
const DEPRECATED_RUNNER_COMMANDS: &[(&str, VulnSeverity, &str)] = &[
    (
        "::set-output",
        VulnSeverity::Medium,
        "write to $GITHUB_OUTPUT instead",
    ),
    (
        "::save-state",
        VulnSeverity::Medium,
        "write to $GITHUB_STATE instead",
    ),
    (
        "::set-env",
        VulnSeverity::High,
        "write to $GITHUB_ENV instead (set-env was disabled)",
    ),
    (
        "::add-path",
        VulnSeverity::High,
        "write to $GITHUB_PATH instead (add-path was disabled)",
    ),
];

/// Audit CI/CD workflow files under `dir`.
pub fn audit_ci(dir: &Path, json: bool) -> Result<()> {
    let dir = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
    let mut findings = Vec::new();
    let mut scanned = 0usize;

    let wf_dir = dir.join(".github/workflows");
    if wf_dir.is_dir() {
        let mut files: Vec<PathBuf> = std::fs::read_dir(&wf_dir)
            .map(|rd| {
                rd.flatten()
                    .map(|e| e.path())
                    .filter(|p| is_yaml(p))
                    .collect()
            })
            .unwrap_or_default();
        files.sort();
        for path in files {
            if let Ok(text) = std::fs::read_to_string(&path) {
                let rel = rel_name(&dir, &path);
                findings.extend(scan_github_workflow(&rel, &text));
                scanned += 1;
            }
        }
    }

    let gitlab = dir.join(".gitlab-ci.yml");
    if gitlab.is_file()
        && let Ok(text) = std::fs::read_to_string(&gitlab)
    {
        findings.extend(scan_gitlab_ci(".gitlab-ci.yml", &text));
        scanned += 1;
    }

    if scanned == 0 {
        bail!(
            "no CI workflow files found in {} (looked for .github/workflows/*.yml and .gitlab-ci.yml)",
            dir.display()
        );
    }

    findings.sort_by(|a, b| {
        b.severity
            .cmp(&a.severity)
            .then_with(|| a.file.cmp(&b.file))
            .then_with(|| a.line.cmp(&b.line))
    });

    if json {
        print_json(&findings, scanned);
    } else {
        print_report(&findings, scanned);
    }

    let interactive = std::io::stdout().is_terminal() && !json;
    let has_high = findings
        .iter()
        .any(|f| matches!(f.severity, VulnSeverity::High | VulnSeverity::Critical));
    if has_high && !interactive {
        std::process::exit(1);
    }
    Ok(())
}

/// Scan a GitHub Actions workflow body for deprecated/outdated constructs.
pub fn scan_github_workflow(file: &str, text: &str) -> Vec<CiFinding> {
    let mut findings = Vec::new();
    for (idx, raw) in text.lines().enumerate() {
        let line = idx + 1;
        let trimmed = raw.trim_start();
        if trimmed.starts_with('#') {
            continue;
        }

        if let Some((path, gitref)) = parse_uses(trimmed) {
            findings.extend(check_action_version(file, line, path, gitref));
        }

        for (needle, severity, fix) in DEPRECATED_RUNNER_COMMANDS {
            if raw.contains(needle) {
                findings.push(CiFinding {
                    file: file.to_string(),
                    line,
                    severity: *severity,
                    rule: "deprecated-runner-command",
                    message: format!("uses deprecated workflow command `{needle}`"),
                    remediation: fix.to_string(),
                });
            }
        }
    }
    findings
}

/// Evaluate a single `uses: owner/repo@ref` reference.
fn check_action_version(file: &str, line: usize, path: &str, gitref: &str) -> Vec<CiFinding> {
    let mut out = Vec::new();
    let repo = action_repo(path);

    // Unpinned to a moving branch — a supply-chain risk.
    if gitref == "main" || gitref == "master" {
        out.push(CiFinding {
            file: file.to_string(),
            line,
            severity: VulnSeverity::Medium,
            rule: "unpinned-action",
            message: format!("`{path}` is pinned to the moving branch `@{gitref}`"),
            remediation: "pin to a released tag (e.g. @v4) or a commit SHA".to_string(),
        });
        return out;
    }

    // A 40-char hex ref is a SHA pin — good practice, nothing to flag.
    let Some(major) = major_of(gitref) else {
        return out;
    };

    if let Some(policy) = ACTION_POLICIES.iter().find(|p| p.repo == repo)
        && major < policy.recommended
    {
        let severity = if major <= policy.eol {
            VulnSeverity::High
        } else {
            VulnSeverity::Medium
        };
        out.push(CiFinding {
            file: file.to_string(),
            line,
            severity,
            rule: "outdated-action",
            message: format!(
                "`{repo}@v{major}` is outdated (current major is v{})",
                policy.recommended
            ),
            remediation: format!("update to `{repo}@v{}`", policy.recommended),
        });
    }
    out
}

/// Scan a GitLab CI file for deprecated keywords and unpinned images.
pub fn scan_gitlab_ci(file: &str, text: &str) -> Vec<CiFinding> {
    let mut findings = Vec::new();
    for (idx, raw) in text.lines().enumerate() {
        let line = idx + 1;
        let trimmed = raw.trim();
        if trimmed.starts_with('#') {
            continue;
        }

        let key = trimmed.split(':').next().unwrap_or("");
        if key == "only" || key == "except" {
            findings.push(CiFinding {
                file: file.to_string(),
                line,
                severity: VulnSeverity::Medium,
                rule: "gitlab-only-except",
                message: format!("`{key}:` is deprecated in favour of `rules:`"),
                remediation: "express job conditions with `rules:`".to_string(),
            });
        }
        if key == "type" || key == "types" {
            findings.push(CiFinding {
                file: file.to_string(),
                line,
                severity: VulnSeverity::Medium,
                rule: "gitlab-type",
                message: format!("`{key}:` is deprecated; use `stage:`"),
                remediation: "rename `type:`/`types:` to `stage:`".to_string(),
            });
        }
        if key == "image"
            && let Some(value) = trimmed.split_once(':').map(|(_, v)| v.trim())
            && !value.is_empty()
            && image_is_unpinned(value)
        {
            findings.push(CiFinding {
                file: file.to_string(),
                line,
                severity: VulnSeverity::Low,
                rule: "unpinned-image",
                message: format!("image `{value}` is unpinned (no tag or `:latest`)"),
                remediation: "pin the image to an explicit version tag or digest".to_string(),
            });
        }
    }
    findings
}

/// True if a docker image reference has no explicit tag, or uses `:latest`.
fn image_is_unpinned(value: &str) -> bool {
    let value = value.trim().trim_matches('"').trim_matches('\'');
    // Block/mapping form (`image:\n  name: ...`) – not a scalar, skip.
    if value.is_empty() {
        return false;
    }
    // Tag/digest lives on the final path segment so a registry port
    // (`registry:5000/img`) is not mistaken for a tag.
    let last = value.rsplit('/').next().unwrap_or(value);
    match last.split_once('@') {
        Some(_) => false, // digest-pinned
        None => match last.split_once(':') {
            Some((_, tag)) => tag.eq_ignore_ascii_case("latest"),
            None => true, // no tag at all
        },
    }
}

/// Parse a `uses:` step into `(action_path, ref)`, skipping local and docker
/// references. Returns `None` for non-`uses` lines.
fn parse_uses(line: &str) -> Option<(&str, &str)> {
    // Tolerate the YAML list-item prefix: `- uses: ...`.
    let line = line.trim_start();
    let line = line.strip_prefix('-').map_or(line, str::trim_start);
    let rest = line.strip_prefix("uses:")?.trim();
    let rest = rest.trim_matches('"').trim_matches('\'');
    if rest.starts_with("./") || rest.starts_with("docker://") {
        return None;
    }
    let (path, gitref) = rest.split_once('@')?;
    let gitref = gitref.split_whitespace().next().unwrap_or(gitref);
    Some((path.trim(), gitref.trim()))
}

/// Reduce an action path to its `owner/repo`, dropping any subdirectory.
fn action_repo(path: &str) -> String {
    let mut parts = path.splitn(3, '/');
    match (parts.next(), parts.next()) {
        (Some(owner), Some(repo)) => format!("{owner}/{repo}"),
        _ => path.to_string(),
    }
}

/// Extract the leading major version number from a ref like `v4` or `4.1.2`.
fn major_of(gitref: &str) -> Option<u32> {
    let r = gitref.strip_prefix('v').unwrap_or(gitref);
    r.split('.').next()?.parse::<u32>().ok()
}

fn is_yaml(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("yml") | Some("yaml")
    )
}

fn rel_name(base: &Path, path: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned()
}

fn print_report(findings: &[CiFinding], scanned: usize) {
    if findings.is_empty() {
        println!("✅ Scanned {scanned} workflow file(s): no deprecated CI constructs found.");
        return;
    }
    for f in findings {
        println!(
            "{} {} {}:{}",
            f.severity.icon(),
            f.severity.label(),
            f.file,
            f.line
        );
        println!("    {} [{}]", f.message, f.rule);
        println!("    fix: {}", f.remediation);
        println!();
    }
    let high = findings
        .iter()
        .filter(|f| matches!(f.severity, VulnSeverity::High | VulnSeverity::Critical))
        .count();
    println!(
        "{} finding(s) across {scanned} workflow file(s) ({high} high).",
        findings.len()
    );
}

fn print_json(findings: &[CiFinding], scanned: usize) {
    let doc = serde_json::json!({
        "scanned": scanned,
        "total": findings.len(),
        "findings": findings,
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&doc).unwrap_or_else(|_| "{}".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uses_and_major() {
        assert_eq!(
            parse_uses("uses: actions/checkout@v3"),
            Some(("actions/checkout", "v3"))
        );
        assert_eq!(
            parse_uses("uses: github/codeql-action/init@v2"),
            Some(("github/codeql-action/init", "v2"))
        );
        assert_eq!(parse_uses("uses: ./local/action"), None);
        assert_eq!(parse_uses("name: build"), None);
        assert_eq!(major_of("v4"), Some(4));
        assert_eq!(major_of("4.1.2"), Some(4));
        assert_eq!(major_of("main"), None);
    }

    #[test]
    fn test_action_repo_drops_subdir() {
        assert_eq!(
            action_repo("github/codeql-action/init"),
            "github/codeql-action"
        );
        assert_eq!(action_repo("actions/checkout"), "actions/checkout");
    }

    #[test]
    fn test_outdated_and_eol_actions() {
        let wf = "\
jobs:
  build:
    steps:
      - uses: actions/checkout@v2
      - uses: actions/setup-python@v4
      - uses: actions/checkout@v4
";
        let f = scan_github_workflow("ci.yml", wf);
        // checkout@v2 is EOL (High), setup-python@v4 is outdated (Medium),
        // checkout@v4 is current (no finding).
        assert_eq!(f.len(), 2);
        let checkout = f.iter().find(|x| x.message.contains("checkout")).unwrap();
        assert_eq!(checkout.severity, VulnSeverity::High);
        let py = f
            .iter()
            .find(|x| x.message.contains("setup-python"))
            .unwrap();
        assert_eq!(py.severity, VulnSeverity::Medium);
    }

    #[test]
    fn test_sha_pinned_action_is_clean() {
        let wf = "      - uses: actions/checkout@8f4b7f84864484a7bf31766abe9204da3cbe65b3\n";
        assert!(scan_github_workflow("ci.yml", wf).is_empty());
    }

    #[test]
    fn test_unpinned_branch_ref() {
        let wf = "      - uses: some/action@main\n";
        let f = scan_github_workflow("ci.yml", wf);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].rule, "unpinned-action");
    }

    #[test]
    fn test_deprecated_runner_commands() {
        let wf = "        run: echo \"::set-output name=x::1\"\n";
        let f = scan_github_workflow("ci.yml", wf);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].rule, "deprecated-runner-command");
        assert_eq!(f[0].severity, VulnSeverity::Medium);

        let wf2 = "        run: echo \"::add-path::/opt/bin\"\n";
        let f2 = scan_github_workflow("ci.yml", wf2);
        assert_eq!(f2[0].severity, VulnSeverity::High);
    }

    #[test]
    fn test_gitlab_only_except_and_image() {
        let yml = "\
build:
  image: node:latest
  only:
    - main
  script:
    - echo hi

test:
  image: rust:1.90
  script: cargo test
";
        let f = scan_gitlab_ci(".gitlab-ci.yml", yml);
        assert!(f.iter().any(|x| x.rule == "gitlab-only-except"));
        assert!(f.iter().any(|x| x.rule == "unpinned-image"));
        // rust:1.90 is pinned, must not be flagged.
        assert_eq!(f.iter().filter(|x| x.rule == "unpinned-image").count(), 1);
    }

    #[test]
    fn test_image_pinning_rules() {
        assert!(image_is_unpinned("node"));
        assert!(image_is_unpinned("node:latest"));
        assert!(!image_is_unpinned("node:20"));
        assert!(!image_is_unpinned("registry:5000/img:1.2"));
        assert!(image_is_unpinned("registry:5000/img"));
        assert!(!image_is_unpinned("img@sha256:abcd"));
    }
}
