//! Native lockfile parsing for dependency auditing.
//!
//! Turns the locked dependency graph of a project into a flat set of
//! `Package { ecosystem, name, version }` records that the OSV client can query.
//! Everything here is pure text/JSON/TOML parsing — no package-manager binary is
//! invoked, so `audit cargo`/`audit node` work offline up to the OSV request.
//!
//! Supported lockfiles:
//!   * `Cargo.lock`         → crates.io   (TOML `[[package]]`)
//!   * `package-lock.json`  → npm         (lockfileVersion 1/2/3)
//!   * `yarn.lock`          → npm         (classic and berry)
//!   * `pnpm-lock.yaml`     → npm         (v5 slash and v6+ `@` key forms)
//!   * `bun.lock`           → npm         (JSONC text format)

use anyhow::{Context, Result};
use serde_json::Value;
use std::path::{Path, PathBuf};

/// A locked dependency expressed in OSV ecosystem terms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub ecosystem: String,
    pub name: String,
    pub version: String,
}

/// Node package managers, in lockfile-detection priority order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodePackageManager {
    Bun,
    Pnpm,
    Yarn,
    Npm,
}

impl NodePackageManager {
    pub fn label(self) -> &'static str {
        match self {
            NodePackageManager::Bun => "bun",
            NodePackageManager::Pnpm => "pnpm",
            NodePackageManager::Yarn => "yarn",
            NodePackageManager::Npm => "npm",
        }
    }
}

/// Find the Node lockfile in `dir`, if any. Detection order mirrors what a
/// developer would expect: a `bun.lock` wins over `package-lock.json`.
pub fn detect_node_lockfile(dir: &Path) -> Option<(NodePackageManager, PathBuf)> {
    const CANDIDATES: &[(&str, NodePackageManager)] = &[
        ("bun.lock", NodePackageManager::Bun),
        ("pnpm-lock.yaml", NodePackageManager::Pnpm),
        ("yarn.lock", NodePackageManager::Yarn),
        ("package-lock.json", NodePackageManager::Npm),
        ("npm-shrinkwrap.json", NodePackageManager::Npm),
    ];
    for (file, pm) in CANDIDATES {
        let p = dir.join(file);
        if p.exists() {
            return Some((*pm, p));
        }
    }
    None
}

/// Parse a Node lockfile body according to its package manager.
pub fn parse_node_lockfile(pm: NodePackageManager, text: &str) -> Result<Vec<Package>> {
    match pm {
        NodePackageManager::Bun => parse_bun_lock(text),
        NodePackageManager::Pnpm => parse_pnpm_lock(text),
        NodePackageManager::Yarn => parse_yarn_lock(text),
        NodePackageManager::Npm => parse_npm_lock(text),
    }
}

// ---- Cargo.lock ----

/// Parse `Cargo.lock` into crates.io packages.
pub fn parse_cargo_lock(text: &str) -> Result<Vec<Package>> {
    let value: toml::Value = toml::from_str(text).context("invalid Cargo.lock TOML")?;
    let mut out = Vec::new();
    if let Some(pkgs) = value.get("package").and_then(|v| v.as_array()) {
        for p in pkgs {
            let (Some(name), Some(version)) = (
                p.get("name").and_then(|v| v.as_str()),
                p.get("version").and_then(|v| v.as_str()),
            ) else {
                continue;
            };
            out.push(Package {
                ecosystem: "crates.io".into(),
                name: name.to_string(),
                version: version.to_string(),
            });
        }
    }
    dedup(&mut out);
    Ok(out)
}

// ---- package-lock.json ----

/// Parse `package-lock.json` / `npm-shrinkwrap.json` into npm packages.
/// Handles lockfileVersion 2/3 (`packages` map) and the legacy v1
/// (`dependencies` tree).
pub fn parse_npm_lock(text: &str) -> Result<Vec<Package>> {
    let v: Value = serde_json::from_str(text).context("invalid package-lock.json")?;
    let mut out = Vec::new();
    if let Some(packages) = v.get("packages").and_then(Value::as_object) {
        for (path, meta) in packages {
            // The root project is keyed by an empty path; skip it.
            let Some(name) = npm_name_from_path(path) else {
                continue;
            };
            let Some(version) = meta.get("version").and_then(Value::as_str) else {
                continue;
            };
            out.push(Package {
                ecosystem: "npm".into(),
                name,
                version: version.to_string(),
            });
        }
    } else if let Some(deps) = v.get("dependencies").and_then(Value::as_object) {
        collect_npm_v1(deps, &mut out);
    }
    dedup(&mut out);
    Ok(out)
}

/// Extract the package name from a v2/v3 `packages` key like
/// `node_modules/foo`, `node_modules/@scope/bar`, or a nested
/// `node_modules/a/node_modules/b`. The deepest `node_modules/` segment wins.
fn npm_name_from_path(path: &str) -> Option<String> {
    let marker = "node_modules/";
    let idx = path.rfind(marker)?;
    let rest = &path[idx + marker.len()..];
    if rest.is_empty() {
        return None;
    }
    if let Some(scoped) = rest.strip_prefix('@') {
        let mut parts = scoped.splitn(2, '/');
        let scope = parts.next()?;
        let name = parts.next()?.split('/').next()?;
        if scope.is_empty() || name.is_empty() {
            return None;
        }
        Some(format!("@{scope}/{name}"))
    } else {
        Some(rest.split('/').next()?.to_string())
    }
}

/// Recursively collect packages from a v1 `dependencies` tree.
fn collect_npm_v1(deps: &serde_json::Map<String, Value>, out: &mut Vec<Package>) {
    for (name, meta) in deps {
        if let Some(version) = meta.get("version").and_then(Value::as_str) {
            out.push(Package {
                ecosystem: "npm".into(),
                name: name.clone(),
                version: version.to_string(),
            });
        }
        if let Some(nested) = meta.get("dependencies").and_then(Value::as_object) {
            collect_npm_v1(nested, out);
        }
    }
}

// ---- yarn.lock ----

/// Parse `yarn.lock` (both the classic v1 format and berry's YAML-ish format).
pub fn parse_yarn_lock(text: &str) -> Result<Vec<Package>> {
    let mut out = Vec::new();
    let mut current_name: Option<String> = None;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let indented = line.starts_with(' ') || line.starts_with('\t');
        if !indented && line.trim_end().ends_with(':') {
            // Block header: one or more comma-separated specifiers.
            let header = line.trim_end().trim_end_matches(':');
            let first = header.split(',').next().unwrap_or("").trim();
            current_name = yarn_name_from_spec(first);
        } else if indented && let Some(rest) = trimmed.strip_prefix("version") {
            // classic: `version "1.2.3"`  berry: `version: 1.2.3`
            let ver = rest.trim().trim_start_matches(':').trim().trim_matches('"');
            if let (Some(name), false) = (current_name.clone(), ver.is_empty()) {
                out.push(Package {
                    ecosystem: "npm".into(),
                    name,
                    version: ver.to_string(),
                });
            }
        }
    }
    dedup(&mut out);
    Ok(out)
}

/// Extract a package name from a yarn specifier such as `foo@^1.0.0`,
/// `@scope/name@^1.0.0`, or berry's `@scope/name@npm:^1.0.0`.
fn yarn_name_from_spec(spec: &str) -> Option<String> {
    let spec = spec.trim().trim_matches('"');
    let at = if let Some(rest) = spec.strip_prefix('@') {
        rest.rfind('@').map(|i| i + 1)
    } else {
        spec.rfind('@')
    }?;
    let name = &spec[..at];
    (!name.is_empty()).then(|| name.to_string())
}

// ---- pnpm-lock.yaml ----

/// Parse `pnpm-lock.yaml`'s `packages:` section. Handles the v5 slash form
/// (`/foo/1.2.3`), the v6+ `@` form (`/foo@1.2.3`), v9's leading-slash-less
/// keys (`foo@1.2.3`), and peer-dependency suffixes (`foo@1.2.3(bar@2.0.0)`).
pub fn parse_pnpm_lock(text: &str) -> Result<Vec<Package>> {
    let mut out = Vec::new();
    let mut in_packages = false;
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        if indent == 0 {
            in_packages = line.trim_start().starts_with("packages:");
            continue;
        }
        // Package entry keys sit one level (2 spaces) under `packages:`.
        if in_packages
            && indent == 2
            && line.trim_end().ends_with(':')
            && let Some(p) = pnpm_parse_key(line)
        {
            out.push(p);
        }
    }
    dedup(&mut out);
    Ok(out)
}

fn pnpm_parse_key(line: &str) -> Option<Package> {
    let key = line.trim().trim_end_matches(':').trim();
    // Drop peer-dependency parenthetical: `foo@1.2.3(react@18.0.0)`.
    let key = key.split('(').next().unwrap_or(key);
    let key = key.strip_prefix('/').unwrap_or(key);
    // Prefer the `name@version` form.
    if let Some(p) = split_name_version_at(key) {
        return Some(p);
    }
    // Legacy `name/version` form (version is the final slash segment).
    if let Some(idx) = key.rfind('/') {
        let (name, version) = key.split_at(idx);
        let version = &version[1..];
        if !name.is_empty() && version.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            return Some(Package {
                ecosystem: "npm".into(),
                name: name.to_string(),
                version: version.to_string(),
            });
        }
    }
    None
}

// ---- bun.lock ----

/// Parse `bun.lock` (the text JSONC lockfile; the binary `bun.lockb` is not
/// supported). Each `packages` entry's first array element is a `name@version`
/// descriptor.
pub fn parse_bun_lock(text: &str) -> Result<Vec<Package>> {
    let cleaned = strip_jsonc(text);
    let v: Value = serde_json::from_str(&cleaned).context("invalid bun.lock")?;
    let mut out = Vec::new();
    if let Some(pkgs) = v.get("packages").and_then(Value::as_object) {
        for val in pkgs.values() {
            let descriptor = match val {
                Value::Array(a) => a.first().and_then(Value::as_str),
                Value::String(s) => Some(s.as_str()),
                _ => None,
            };
            if let Some(p) = descriptor.and_then(split_name_version_at) {
                out.push(p);
            }
        }
    }
    dedup(&mut out);
    Ok(out)
}

/// Strip `//` and `/* */` comments and trailing commas so a JSONC document
/// parses as strict JSON. String contents are preserved verbatim.
fn strip_jsonc(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    let mut in_string = false;
    let mut escaped = false;
    while let Some(c) = chars.next() {
        if in_string {
            out.push(c);
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }
        match c {
            '"' => {
                in_string = true;
                out.push(c);
            }
            '/' if chars.peek() == Some(&'/') => {
                for nc in chars.by_ref() {
                    if nc == '\n' {
                        out.push('\n');
                        break;
                    }
                }
            }
            '/' if chars.peek() == Some(&'*') => {
                chars.next();
                let mut prev = '\0';
                for nc in chars.by_ref() {
                    if prev == '*' && nc == '/' {
                        break;
                    }
                    prev = nc;
                }
            }
            _ => out.push(c),
        }
    }
    remove_trailing_commas(&out)
}

/// Remove commas that directly precede a closing `}` or `]` (ignoring
/// whitespace), respecting string literals.
fn remove_trailing_commas(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut in_string = false;
    let mut escaped = false;
    for (i, &c) in chars.iter().enumerate() {
        if in_string {
            out.push(c);
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }
        if c == '"' {
            in_string = true;
            out.push(c);
            continue;
        }
        if c == ',' {
            let mut j = i + 1;
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            if j < chars.len() && (chars[j] == '}' || chars[j] == ']') {
                continue;
            }
        }
        out.push(c);
    }
    out
}

// ---- shared helpers ----

/// Split a `name@version` descriptor into a package, validating that the
/// version starts with a digit (filters out git/file/alias specs we can't
/// audit). Scoped names keep their leading `@`.
fn split_name_version_at(s: &str) -> Option<Package> {
    let s = s.trim();
    let at = if let Some(rest) = s.strip_prefix('@') {
        rest.rfind('@').map(|i| i + 1)
    } else {
        s.rfind('@')
    }?;
    let name = &s[..at];
    let version = &s[at + 1..];
    if name.is_empty() || !version.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        return None;
    }
    Some(Package {
        ecosystem: "npm".into(),
        name: name.to_string(),
        version: version.to_string(),
    })
}

/// Sort and de-duplicate a package list so OSV queries stay minimal and stable.
fn dedup(pkgs: &mut Vec<Package>) {
    pkgs.sort_by(|a, b| a.name.cmp(&b.name).then_with(|| a.version.cmp(&b.version)));
    pkgs.dedup();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(pkgs: &[Package]) -> Vec<(&str, &str)> {
        pkgs.iter()
            .map(|p| (p.name.as_str(), p.version.as_str()))
            .collect()
    }

    #[test]
    fn test_parse_cargo_lock() {
        let text = r#"
version = 3

[[package]]
name = "anyhow"
version = "1.0.86"

[[package]]
name = "serde"
version = "1.0.203"
source = "registry+https://github.com/rust-lang/crates.io-index"
"#;
        let pkgs = parse_cargo_lock(text).unwrap();
        assert_eq!(
            names(&pkgs),
            vec![("anyhow", "1.0.86"), ("serde", "1.0.203")]
        );
        assert!(pkgs.iter().all(|p| p.ecosystem == "crates.io"));
    }

    #[test]
    fn test_parse_npm_lock_v3() {
        let text = r#"{
            "name": "app",
            "lockfileVersion": 3,
            "packages": {
                "": { "name": "app", "version": "1.0.0" },
                "node_modules/lodash": { "version": "4.17.21" },
                "node_modules/@babel/core": { "version": "7.24.0" },
                "node_modules/a/node_modules/b": { "version": "2.0.0" }
            }
        }"#;
        let pkgs = parse_npm_lock(text).unwrap();
        assert_eq!(
            names(&pkgs),
            vec![
                ("@babel/core", "7.24.0"),
                ("b", "2.0.0"),
                ("lodash", "4.17.21")
            ]
        );
    }

    #[test]
    fn test_parse_npm_lock_v1() {
        let text = r#"{
            "name": "app",
            "lockfileVersion": 1,
            "dependencies": {
                "lodash": { "version": "4.17.21" },
                "chalk": {
                    "version": "5.3.0",
                    "dependencies": { "ansi-styles": { "version": "6.2.1" } }
                }
            }
        }"#;
        let pkgs = parse_npm_lock(text).unwrap();
        assert_eq!(
            names(&pkgs),
            vec![
                ("ansi-styles", "6.2.1"),
                ("chalk", "5.3.0"),
                ("lodash", "4.17.21")
            ]
        );
    }

    #[test]
    fn test_parse_yarn_lock_classic() {
        let text = r#"
# yarn lockfile v1

"@babel/core@^7.0.0", "@babel/core@^7.1.0":
  version "7.24.0"
  resolved "https://registry.yarnpkg.com/..."

lodash@^4.17.0:
  version "4.17.21"
  resolved "https://registry.yarnpkg.com/..."
"#;
        let pkgs = parse_yarn_lock(text).unwrap();
        assert_eq!(
            names(&pkgs),
            vec![("@babel/core", "7.24.0"), ("lodash", "4.17.21")]
        );
    }

    #[test]
    fn test_parse_yarn_lock_berry() {
        let text = r#"
__metadata:
  version: 6
  cacheKey: 8

"lodash@npm:^4.17.0":
  version: 4.17.21
  resolution: "lodash@npm:4.17.21"
"#;
        let pkgs = parse_yarn_lock(text).unwrap();
        assert_eq!(names(&pkgs), vec![("lodash", "4.17.21")]);
    }

    #[test]
    fn test_parse_pnpm_lock_at_form() {
        let text = r#"
lockfileVersion: '6.0'

dependencies:
  lodash:
    specifier: ^4.17.0
    version: 4.17.21

packages:

  /lodash@4.17.21:
    resolution: {integrity: sha512-abc}
    dev: false

  /@babel/core@7.24.0(supports-color@8.1.1):
    resolution: {integrity: sha512-def}
"#;
        let pkgs = parse_pnpm_lock(text).unwrap();
        assert_eq!(
            names(&pkgs),
            vec![("@babel/core", "7.24.0"), ("lodash", "4.17.21")]
        );
    }

    #[test]
    fn test_parse_pnpm_lock_slash_form() {
        let text = r#"
packages:

  /lodash/4.17.21:
    resolution: {integrity: sha512-abc}

  /@babel/core/7.24.0:
    resolution: {integrity: sha512-def}
"#;
        let pkgs = parse_pnpm_lock(text).unwrap();
        assert_eq!(
            names(&pkgs),
            vec![("@babel/core", "7.24.0"), ("lodash", "4.17.21")]
        );
    }

    #[test]
    fn test_parse_bun_lock() {
        let text = r#"{
  "lockfileVersion": 0,
  // bun text lockfile
  "workspaces": {
    "": { "name": "app" },
  },
  "packages": {
    "lodash": ["lodash@4.17.21", "", {}, "sha512-abc"],
    "@babel/core": ["@babel/core@7.24.0", "", {}, "sha512-def"],
  },
}"#;
        let pkgs = parse_bun_lock(text).unwrap();
        assert_eq!(
            names(&pkgs),
            vec![("@babel/core", "7.24.0"), ("lodash", "4.17.21")]
        );
    }

    #[test]
    fn test_strip_jsonc_preserves_strings() {
        let input = r#"{ "url": "http://x/y", "a": 1, /* c */ "b": 2, }"#;
        let cleaned = strip_jsonc(input);
        let v: Value = serde_json::from_str(&cleaned).unwrap();
        assert_eq!(v.get("url").and_then(Value::as_str), Some("http://x/y"));
        assert_eq!(v.get("a").and_then(Value::as_i64), Some(1));
    }

    #[test]
    fn test_split_name_version_filters_non_versions() {
        assert!(split_name_version_at("foo@1.2.3").is_some());
        assert!(split_name_version_at("@scope/bar@2.0.0").is_some());
        // git/file specs have non-digit versions and are skipped.
        assert!(split_name_version_at("foo@github:user/repo").is_none());
        assert!(split_name_version_at("noversion").is_none());
    }

    #[test]
    fn test_dedup_sorts_and_removes_duplicates() {
        let mut pkgs = vec![
            Package {
                ecosystem: "npm".into(),
                name: "b".into(),
                version: "1.0.0".into(),
            },
            Package {
                ecosystem: "npm".into(),
                name: "a".into(),
                version: "1.0.0".into(),
            },
            Package {
                ecosystem: "npm".into(),
                name: "a".into(),
                version: "1.0.0".into(),
            },
        ];
        dedup(&mut pkgs);
        assert_eq!(names(&pkgs), vec![("a", "1.0.0"), ("b", "1.0.0")]);
    }

    #[test]
    fn test_detect_node_lockfile_priority() {
        let dir = std::env::temp_dir().join(format!("ghostctl-lf-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("package-lock.json"), "{}").unwrap();
        std::fs::write(dir.join("bun.lock"), "{}").unwrap();
        let (pm, path) = detect_node_lockfile(&dir).unwrap();
        assert_eq!(pm, NodePackageManager::Bun);
        assert!(path.ends_with("bun.lock"));
        std::fs::remove_dir_all(&dir).ok();
    }
}
