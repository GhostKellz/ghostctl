//! Project dependency auditing: parse lockfiles → query OSV → unified report.
//!
//! Ties the native lockfile parsers (`lockfile.rs`) to the OSV client
//! (`osv.rs`) and the shared vulnerability report (`vuln.rs`). `audit cargo` and
//! `audit node` target one ecosystem; `audit deps` auto-detects every supported
//! lockfile in the project and reports them together.

use anyhow::{Context, Result, anyhow, bail};
use std::io::IsTerminal;
use std::path::{Path, PathBuf};

use super::config::AuditConfig;
use super::http_client;
use super::lockfile::{self, Package};
use super::osv;
use super::vuln;

/// One audited lockfile and the packages it locked.
struct AuditSource {
    label: String,
    packages: Vec<Package>,
}

/// Audit a Rust project's `Cargo.lock`.
pub fn audit_cargo(cfg: &AuditConfig, dir: &Path, json: bool) -> Result<()> {
    let dir = canonical(dir);
    let source = collect_cargo(&dir)?.ok_or_else(|| {
        anyhow!(
            "no Cargo.lock found in {} or any parent directory",
            dir.display()
        )
    })?;
    run(cfg, vec![source], json)
}

/// Audit a Node project's lockfile (bun/pnpm/yarn/npm).
pub fn audit_node(cfg: &AuditConfig, dir: &Path, json: bool) -> Result<()> {
    let dir = canonical(dir);
    let source = collect_node(&dir)?.ok_or_else(|| {
        anyhow!(
            "no Node lockfile (bun.lock/pnpm-lock.yaml/yarn.lock/package-lock.json) found in {} or any parent directory",
            dir.display()
        )
    })?;
    run(cfg, vec![source], json)
}

/// Auto-detect every supported lockfile in the project and audit them together.
pub fn audit_deps(cfg: &AuditConfig, dir: &Path, json: bool) -> Result<()> {
    let dir = canonical(dir);
    let mut sources = Vec::new();
    if let Some(s) = collect_cargo(&dir)? {
        sources.push(s);
    }
    if let Some(s) = collect_node(&dir)? {
        sources.push(s);
    }
    if sources.is_empty() {
        bail!(
            "no supported lockfiles found in {} (looked for Cargo.lock and Node lockfiles)",
            dir.display()
        );
    }
    run(cfg, sources, json)
}

/// Pure detection of which ecosystems are present directly in `dir`.
pub fn detect_project_kinds(dir: &Path) -> Vec<&'static str> {
    let mut kinds = Vec::new();
    if dir.join("Cargo.lock").exists() {
        kinds.push("cargo");
    }
    if lockfile::detect_node_lockfile(dir).is_some() {
        kinds.push("node");
    }
    kinds
}

fn collect_cargo(dir: &Path) -> Result<Option<AuditSource>> {
    let Some(path) = find_up(dir, |d| {
        let p = d.join("Cargo.lock");
        p.exists().then_some(p)
    }) else {
        return Ok(None);
    };
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    let packages = lockfile::parse_cargo_lock(&text)?;
    Ok(Some(AuditSource {
        label: format!("{} (crates.io)", lockfile_name(&path)),
        packages,
    }))
}

fn collect_node(dir: &Path) -> Result<Option<AuditSource>> {
    let Some((pm, path)) = find_up(dir, lockfile::detect_node_lockfile) else {
        return Ok(None);
    };
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    let packages = lockfile::parse_node_lockfile(pm, &text)?;
    Ok(Some(AuditSource {
        label: format!("{} ({}, npm)", lockfile_name(&path), pm.label()),
        packages,
    }))
}

fn run(cfg: &AuditConfig, sources: Vec<AuditSource>, json: bool) -> Result<()> {
    // Unique package set across all sources keeps the OSV query minimal.
    let mut unique: Vec<Package> = sources
        .iter()
        .flat_map(|s| s.packages.iter().cloned())
        .collect();
    unique.sort_by(|a, b| {
        a.ecosystem
            .cmp(&b.ecosystem)
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| a.version.cmp(&b.version))
    });
    unique.dedup();

    if !json {
        for s in &sources {
            println!("Scanning {} — {} package(s)", s.label, s.packages.len());
        }
        println!("Querying OSV.dev for {} unique package(s)...", unique.len());
        println!();
    }

    let client = http_client(cfg.timeout_secs)?;
    let mut findings = osv::audit_packages(&client, &unique)?;
    vuln::sort_findings(&mut findings);

    if json {
        println!("{}", vuln::to_json(&findings));
    } else {
        vuln::print_report(&findings);
    }

    // CI-friendly: a High/Critical finding fails the command in
    // non-interactive/JSON use, while interactive terminals stay informational.
    let interactive = std::io::stdout().is_terminal() && !json;
    if vuln::has_high_or_critical(&findings) && !interactive {
        std::process::exit(1);
    }
    Ok(())
}

/// Walk from `start` toward the filesystem root, returning the first match.
fn find_up<T>(start: &Path, probe: impl Fn(&Path) -> Option<T>) -> Option<T> {
    let mut cur = Some(start);
    while let Some(dir) = cur {
        if let Some(found) = probe(dir) {
            return Some(found);
        }
        cur = dir.parent();
    }
    None
}

fn canonical(dir: &Path) -> PathBuf {
    dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf())
}

fn lockfile_name(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_project_kinds() {
        let dir = std::env::temp_dir().join(format!("ghostctl-deps-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        assert!(detect_project_kinds(&dir).is_empty());

        std::fs::write(dir.join("Cargo.lock"), "version = 3\n").unwrap();
        std::fs::write(dir.join("package-lock.json"), "{}").unwrap();
        let kinds = detect_project_kinds(&dir);
        assert!(kinds.contains(&"cargo"));
        assert!(kinds.contains(&"node"));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_find_up_locates_parent_file() {
        let base = std::env::temp_dir().join(format!("ghostctl-findup-{}", std::process::id()));
        let nested = base.join("a").join("b");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(base.join("Cargo.lock"), "version = 3\n").unwrap();

        let found = find_up(&nested, |d| {
            let p = d.join("Cargo.lock");
            p.exists().then_some(p)
        });
        assert!(found.is_some());
        assert!(found.unwrap().ends_with("Cargo.lock"));

        std::fs::remove_dir_all(&base).ok();
    }
}
