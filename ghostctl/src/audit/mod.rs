//! `ghostctl audit` - Arch/AUR package security auditing.
//!
//! Complementary checks:
//!   * `cve`  - cross-references installed packages against the Arch Security
//!     Tracker advisory database (and uses `vercmp` to confirm the installed
//!     version is actually still vulnerable).
//!   * `aur`  - inventories foreign/AUR packages and heuristically scans their
//!     PKGBUILDs *and* `.install` hooks for the remote-payload patterns used in
//!     AUR supply-chain attacks.
//!   * `ioc`  - cross-references an external indicator-of-compromise feed of
//!     suspect package names against what is installed now and against the
//!     historical pacman log (catching installed-then-removed packages).

pub mod config;
pub mod ioc;
pub mod scan;
pub mod tracker;

use anyhow::{Context, Result, bail};
use clap::{Arg, ArgMatches, Command};
use config::AuditConfig;
use reqwest::blocking::Client;
use scan::{Finding, Severity};
use std::collections::{BTreeMap, BTreeSet};
use std::io::{IsTerminal, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command as ProcCommand;
use std::time::Duration;
use tracker::severity_rank;

pub fn command() -> Command {
    Command::new("audit")
        .about("Audit Arch/AUR packages for CVEs and malicious PKGBUILDs")
        .subcommand(
            Command::new("cve").about("Check installed packages against the Arch Security Tracker"),
        )
        .subcommand(
            Command::new("aur").about("Scan installed AUR/foreign package PKGBUILDs for red flags"),
        )
        .subcommand(
            Command::new("pkgbuild")
                .about("Scan a single PKGBUILD (local path or AUR package name)")
                .arg(
                    Arg::new("target")
                        .required(true)
                        .help("Path to a PKGBUILD file, or an AUR package name to fetch"),
                ),
        )
        .subcommand(
            Command::new("ioc")
                .about("Check installed packages and pacman history against an IOC package feed")
                .arg(
                    Arg::new("feed")
                        .long("feed")
                        .value_name("PATH|URL")
                        .help("Package-name feed to use (overrides the [audit] ioc_feed setting)"),
                ),
        )
        .subcommand(Command::new("summary").about("Quick package-security overview"))
}

pub fn handle(matches: &ArgMatches) -> Result<()> {
    let cfg = AuditConfig::load();
    match matches.subcommand() {
        Some(("cve", _)) => cve(&cfg),
        Some(("aur", _)) => aur(&cfg),
        Some(("pkgbuild", m)) => {
            let target = m.get_one::<String>("target").unwrap();
            pkgbuild(&cfg, target)
        }
        Some(("ioc", m)) => ioc_check(&cfg, m.get_one::<String>("feed").map(String::as_str)),
        Some(("summary", _)) => summary(&cfg),
        _ => {
            println!("Use `ghostctl audit --help` to see available subcommands.");
            Ok(())
        }
    }
}

// ---- cve ----

#[derive(Debug)]
struct VulnReport {
    severity: String,
    package: String,
    installed: String,
    fixed: String,
    cves: String,
    avg: String,
}

fn cve(cfg: &AuditConfig) -> Result<()> {
    if which::which("pacman").is_err() {
        bail!("pacman not found - this command is for Arch-based systems.");
    }
    println!("🔒 Checking installed packages against the Arch Security Tracker...");
    let installed = installed_packages()?;

    let body = http_client(cfg.timeout_secs)?
        .get(&cfg.tracker_url)
        .send()
        .with_context(|| format!("request failed: {}", cfg.tracker_url))?;
    let status = body.status();
    let text = body.text().unwrap_or_default();
    if !status.is_success() {
        bail!("HTTP {} from {}", status.as_u16(), cfg.tracker_url);
    }
    let entries = tracker::parse_tracker(&text)?;

    let mut reports: Vec<VulnReport> = Vec::new();
    for entry in &entries {
        if entry.status.eq_ignore_ascii_case("Not affected") {
            continue;
        }
        for pkg in &entry.packages {
            let Some(inst_ver) = installed.get(pkg) else {
                continue;
            };
            let fixed = entry.fixed.clone().unwrap_or_default();
            let vulnerable = if entry.is_unfixed() || fixed.is_empty() {
                true
            } else {
                // Still vulnerable only if the installed version is below the fix.
                version_lt(inst_ver, &fixed).unwrap_or(true)
            };
            if !vulnerable {
                continue;
            }
            reports.push(VulnReport {
                severity: if entry.severity.is_empty() {
                    "Unknown".to_string()
                } else {
                    entry.severity.clone()
                },
                package: pkg.clone(),
                installed: inst_ver.clone(),
                fixed: if fixed.is_empty() {
                    "(none yet)".to_string()
                } else {
                    fixed
                },
                cves: entry.issues.join(", "),
                avg: entry.name.clone(),
            });
        }
    }

    if reports.is_empty() {
        println!("✓ No known vulnerabilities affect your installed packages.");
        return Ok(());
    }

    reports.sort_by(|a, b| {
        severity_rank(&b.severity)
            .cmp(&severity_rank(&a.severity))
            .then(a.package.cmp(&b.package))
    });

    println!("\n⚠ {} vulnerable package(s) found:\n", reports.len());
    println!(
        "{:<9} {:<22} {:<16} {:<16} ADVISORY",
        "SEVERITY", "PACKAGE", "INSTALLED", "FIXED"
    );
    for r in &reports {
        println!(
            "{:<9} {:<22} {:<16} {:<16} {} ({})",
            r.severity, r.package, r.installed, r.fixed, r.avg, r.cves
        );
    }
    println!("\nDetails: https://security.archlinux.org/");
    Ok(())
}

// ---- aur ----

fn aur(cfg: &AuditConfig) -> Result<()> {
    if which::which("pacman").is_err() {
        bail!("pacman not found - this command is for Arch-based systems.");
    }
    let foreign = foreign_packages()?;
    if foreign.is_empty() {
        println!("No foreign/AUR packages installed.");
        return Ok(());
    }
    println!(
        "🔎 Scanning {} foreign/AUR package PKGBUILD(s) for suspicious patterns...\n",
        foreign.len()
    );

    let client = http_client(cfg.timeout_secs)?;
    let mut flagged = 0usize;
    let mut unavailable = 0usize;

    // Each package is one (or two) blocking HTTP fetches, so a large AUR set
    // can take a while - show a live counter on an interactive stderr.
    let total = foreign.len();
    let show_progress = std::io::stderr().is_terminal();

    for (idx, (name, _ver)) in foreign.iter().enumerate() {
        if show_progress {
            let short: String = name.chars().take(40).collect();
            eprint!("\r  scanning [{}/{total}] {short:<40}", idx + 1);
            let _ = std::io::stderr().flush();
        }

        let pkgbuild = match fetch_pkgbuild(&client, &cfg.pkgbuild_url(name)) {
            Ok(Some(body)) => body,
            Ok(None) | Err(_) => {
                unavailable += 1;
                continue;
            }
        };
        let pb_findings = scan::scan_pkgbuild(&pkgbuild);
        // The `.install` hook runs as root via pacman - scan it too.
        let install_findings = match install_hook_name(&pkgbuild, name) {
            Some(file) => match fetch_pkgbuild(&client, &cfg.aur_file_url(name, &file)) {
                Ok(Some(body)) => scan::scan_pkgbuild(&body),
                _ => Vec::new(),
            },
            None => Vec::new(),
        };

        if pb_findings.is_empty() && install_findings.is_empty() {
            continue;
        }
        if show_progress {
            // Clear the progress line before emitting a result block.
            eprint!("\r{:<60}\r", "");
            let _ = std::io::stderr().flush();
        }
        flagged += 1;
        let highs = scan::high_count(&pb_findings) + scan::high_count(&install_findings);
        let total = pb_findings.len() + install_findings.len();
        let badge = if highs > 0 { "⚠" } else { "·" };
        println!("{badge} {name}  ({total} finding(s), {highs} high)");
        if !pb_findings.is_empty() {
            println!("    PKGBUILD:");
            print_findings(&pb_findings, 6);
        }
        if !install_findings.is_empty() {
            println!("    .install hook:");
            print_findings(&install_findings, 6);
        }
    }

    if show_progress {
        eprint!("\r{:<60}\r", "");
        let _ = std::io::stderr().flush();
    }

    println!();
    if flagged == 0 {
        println!("✓ No suspicious patterns found in available PKGBUILDs.");
    } else {
        println!("⚠ {flagged} package(s) have suspicious PKGBUILD patterns - review them.");
    }
    if unavailable > 0 {
        println!("  ({unavailable} package(s) had no fetchable AUR PKGBUILD - likely non-AUR.)");
    }
    println!(
        "\nNote: heuristics flag suspicion, not proof. Always read flagged PKGBUILDs yourself."
    );
    Ok(())
}

// ---- pkgbuild ----

fn pkgbuild(cfg: &AuditConfig, target: &str) -> Result<()> {
    let path = std::path::Path::new(target);
    // (body, optional .install body)
    let (body, install_body) = if path.exists() {
        let body =
            std::fs::read_to_string(path).with_context(|| format!("failed to read {target}"))?;
        // Look for a sibling `.install` hook referenced by the PKGBUILD.
        let install_body = scan::parse_install_field(&body).and_then(|file| {
            path.parent()
                .map(|dir| dir.join(&file))
                .filter(|p| p.exists())
                .and_then(|p| std::fs::read_to_string(p).ok())
        });
        (body, install_body)
    } else {
        println!("Fetching PKGBUILD for '{target}' from the AUR...");
        let client = http_client(cfg.timeout_secs)?;
        let body = match fetch_pkgbuild(&client, &cfg.pkgbuild_url(target))? {
            Some(b) => b,
            None => bail!("no PKGBUILD found for '{target}' (not an AUR package?)"),
        };
        let install_body = install_hook_name(&body, target).and_then(|file| {
            fetch_pkgbuild(&client, &cfg.aur_file_url(target, &file))
                .ok()
                .flatten()
        });
        (body, install_body)
    };

    let pb_findings = scan::scan_pkgbuild(&body);
    let install_findings = install_body.as_deref().map(scan::scan_pkgbuild);

    let total = pb_findings.len() + install_findings.as_ref().map_or(0, Vec::len);
    if total == 0 {
        println!("✓ No suspicious patterns found.");
        return Ok(());
    }
    let highs =
        scan::high_count(&pb_findings) + install_findings.as_deref().map_or(0, scan::high_count);
    println!("⚠ {total} finding(s) ({highs} high):\n");
    if !pb_findings.is_empty() {
        println!("PKGBUILD:");
        print_findings(&pb_findings, 2);
    }
    if let Some(f) = &install_findings
        && !f.is_empty()
    {
        println!(".install hook:");
        print_findings(f, 2);
    }
    println!("\nNote: heuristics flag suspicion, not proof. Review the files yourself.");
    Ok(())
}

// ---- ioc ----

fn ioc_check(cfg: &AuditConfig, feed_override: Option<&str>) -> Result<()> {
    if which::which("pacman").is_err() {
        bail!("pacman not found - this command is for Arch-based systems.");
    }

    let Some(feed_src) = feed_override
        .map(str::to_string)
        .or_else(|| cfg.ioc_feed.clone())
    else {
        println!("No IOC feed configured.");
        println!("Provide one with `--feed <path|url>`, or set `ioc_feed` under [audit]");
        println!("in your config.toml. The feed is a plain list of suspect package");
        println!("names (one per line; `#` comments allowed).");
        return Ok(());
    };

    let feed_text = load_feed(cfg, &feed_src)?;
    let flagged = ioc::parse_feed(&feed_text);
    if flagged.is_empty() {
        println!("Feed '{feed_src}' contained no package names.");
        return Ok(());
    }
    println!(
        "🔒 Checking {} flagged package(s) from {feed_src}...\n",
        flagged.len()
    );

    // 1) Currently installed packages that match the feed. An IOC feed is
    // generic, so check *all* installed packages, not just foreign ones - a
    // compromised package can also reach the official repos. Foreign/AUR hits
    // are labelled since the AUR is the more likely vector.
    let foreign: BTreeSet<String> = foreign_packages()?
        .into_iter()
        .map(|(name, _)| name)
        .collect();
    let installed_hits: Vec<(String, String, bool)> = installed_packages()?
        .into_keys()
        .filter(|name| flagged.contains(name))
        .map(|name| {
            let when = install_date(&name).unwrap_or_else(|| "unknown".to_string());
            let is_foreign = foreign.contains(&name);
            (name, when, is_foreign)
        })
        .collect();

    if installed_hits.is_empty() {
        println!("✓ No flagged packages are currently installed.");
    } else {
        println!(
            "⚠ {} flagged package(s) currently INSTALLED:",
            installed_hits.len()
        );
        for (name, when, is_foreign) in &installed_hits {
            let origin = if *is_foreign { "foreign/AUR" } else { "repo" };
            println!("    ⚠ {name}  ({origin}, installed: {when})");
        }
    }

    // 2) Historical pacman.log evidence (catches installed-then-removed).
    let log_files = expand_log_files(&cfg.pacman_log_glob);
    let mut log_hits = Vec::new();
    let mut unreadable = 0usize;
    for file in &log_files {
        match read_log(file) {
            Some(text) => log_hits.extend(ioc::scan_pacman_log(&text, &flagged)),
            None => unreadable += 1,
        }
    }

    println!();
    if log_files.is_empty() {
        println!("No pacman logs matched '{}'.", cfg.pacman_log_glob);
    } else if log_hits.is_empty() {
        println!("✓ No flagged packages found in pacman history.");
    } else {
        println!("⚠ {} historical pacman event(s):", log_hits.len());
        for h in &log_hits {
            println!("    {} {} on {}", h.action, h.package, h.date);
        }
    }
    if unreadable > 0 {
        println!(
            "  ({unreadable} log file(s) could not be read - try sudo, or install the matching decompressor.)"
        );
    }

    if !installed_hits.is_empty() {
        println!("\n‼ Flagged packages are installed. Review them and follow incident response.");
    }
    Ok(())
}

/// Load a feed from a local path or an http(s) URL.
fn load_feed(cfg: &AuditConfig, src: &str) -> Result<String> {
    if src.starts_with("http://") || src.starts_with("https://") {
        let client = http_client(cfg.timeout_secs)?;
        let resp = client
            .get(src)
            .send()
            .with_context(|| format!("request failed: {src}"))?;
        if !resp.status().is_success() {
            bail!("HTTP {} from {src}", resp.status().as_u16());
        }
        Ok(resp.text().unwrap_or_default())
    } else {
        std::fs::read_to_string(src).with_context(|| format!("failed to read feed: {src}"))
    }
}

/// Expand a `pacman.log*`-style glob into matching files (prefix match only).
fn expand_log_files(glob: &str) -> Vec<PathBuf> {
    let p = Path::new(glob);
    let (Some(dir), Some(name)) = (p.parent(), p.file_name()) else {
        return Vec::new();
    };
    let prefix = name.to_string_lossy();
    let prefix = prefix.trim_end_matches('*');
    let mut out: Vec<PathBuf> = match std::fs::read_dir(dir) {
        Ok(rd) => rd
            .flatten()
            .filter(|e| e.file_name().to_string_lossy().starts_with(prefix))
            .map(|e| e.path())
            .collect(),
        Err(_) => Vec::new(),
    };
    out.sort();
    out
}

/// Read a (possibly compressed) pacman log file into a string.
fn read_log(path: &Path) -> Option<String> {
    match path.extension().and_then(|e| e.to_str()) {
        Some("gz") => {
            let file = std::fs::File::open(path).ok()?;
            let mut dec = flate2::read::GzDecoder::new(file);
            let mut s = String::new();
            dec.read_to_string(&mut s).ok()?;
            Some(s)
        }
        Some(ext @ ("xz" | "zst" | "bz2")) => {
            let tool = match ext {
                "xz" => "xz",
                "zst" => "zstd",
                _ => "bzip2",
            };
            if which::which(tool).is_err() {
                return None;
            }
            let out = ProcCommand::new(tool).arg("-dc").arg(path).output().ok()?;
            out.status
                .success()
                .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
        }
        _ => std::fs::read_to_string(path).ok(),
    }
}

/// Read a package's "Install Date" via `pacman -Qi`.
fn install_date(pkg: &str) -> Option<String> {
    let out = ProcCommand::new("pacman")
        .args(["-Qi", pkg])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .find_map(|l| l.strip_prefix("Install Date"))
        .and_then(|v| v.split_once(':'))
        .map(|(_, d)| d.trim().to_string())
}

// ---- summary ----

fn summary(cfg: &AuditConfig) -> Result<()> {
    if which::which("pacman").is_err() {
        bail!("pacman not found - this command is for Arch-based systems.");
    }
    let installed = installed_packages()?;
    let foreign = foreign_packages()?;
    let orphans = orphan_count();

    println!("📋 Package security summary");
    println!("───────────────────────────");
    println!("  Installed packages : {}", installed.len());
    println!("  Foreign/AUR        : {}", foreign.len());
    println!("  Orphans            : {orphans}");
    println!();
    println!("Run `ghostctl audit cve` for vulnerability status,");
    println!("`ghostctl audit aur` to scan AUR PKGBUILDs and .install hooks,");
    println!("and `ghostctl audit ioc --feed <path|url>` to match a compromise feed.");
    let _ = cfg;
    Ok(())
}

// ---- helpers ----

fn print_findings(findings: &[Finding], indent: usize) {
    let pad = " ".repeat(indent);
    for f in findings {
        let icon = match f.severity {
            Severity::High => "⚠",
            Severity::Medium => "•",
            Severity::Low => "·",
        };
        println!(
            "{pad}{icon} [{}] L{:<4} {:<24} {}",
            f.severity.marker(),
            f.line,
            f.rule,
            f.excerpt
        );
    }
}

/// Resolve the `.install` hook filename for a package. Prefers the literal
/// `install=` value; when that depends on a shell variable, falls back to the
/// near-universal `<pkg>.install` convention. Returns `None` if no hook.
fn install_hook_name(pkgbuild: &str, pkg: &str) -> Option<String> {
    if let Some(file) = scan::parse_install_field(pkgbuild) {
        return Some(file);
    }
    if pkgbuild
        .lines()
        .any(|l| l.trim_start().starts_with("install="))
    {
        return Some(format!("{pkg}.install"));
    }
    None
}

fn http_client(timeout_secs: u64) -> Result<Client> {
    Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .user_agent("ghostctl")
        .build()
        .context("failed to build HTTP client")
}

/// Fetch a PKGBUILD body; Ok(None) on 404 (package not on AUR).
fn fetch_pkgbuild(client: &Client, url: &str) -> Result<Option<String>> {
    let resp = client
        .get(url)
        .send()
        .with_context(|| format!("request failed: {url}"))?;
    if resp.status().as_u16() == 404 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        bail!("HTTP {} from {}", resp.status().as_u16(), url);
    }
    Ok(Some(resp.text().unwrap_or_default()))
}

/// Map of installed package name -> version via `pacman -Q`.
fn installed_packages() -> Result<BTreeMap<String, String>> {
    let out = ProcCommand::new("pacman")
        .arg("-Q")
        .output()
        .context("failed to run pacman -Q")?;
    if !out.status.success() {
        bail!("pacman -Q failed");
    }
    Ok(parse_pacman_q(&String::from_utf8_lossy(&out.stdout)))
}

/// List of foreign (AUR/manual) packages via `pacman -Qm`.
fn foreign_packages() -> Result<Vec<(String, String)>> {
    let out = ProcCommand::new("pacman")
        .arg("-Qm")
        .output()
        .context("failed to run pacman -Qm")?;
    if !out.status.success() {
        bail!("pacman -Qm failed");
    }
    Ok(parse_pacman_q(&String::from_utf8_lossy(&out.stdout))
        .into_iter()
        .collect())
}

fn orphan_count() -> usize {
    ProcCommand::new("pacman")
        .args(["-Qtdq"])
        .output()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .filter(|l| !l.trim().is_empty())
                .count()
        })
        .unwrap_or(0)
}

/// Parse `pacman -Q`/`-Qm` output ("name version" per line).
fn parse_pacman_q(text: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for line in text.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(name), Some(ver)) = (parts.next(), parts.next()) {
            map.insert(name.to_string(), ver.to_string());
        }
    }
    map
}

/// True if version `a` is strictly older than `b`, using pacman's `vercmp`.
/// Returns None if `vercmp` is unavailable or output can't be parsed.
fn version_lt(a: &str, b: &str) -> Option<bool> {
    let out = ProcCommand::new("vercmp").args([a, b]).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let n: i32 = String::from_utf8_lossy(&out.stdout).trim().parse().ok()?;
    Some(n < 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pacman_q() {
        let text = "linux 6.9.1.arch1-1\nfirefox 126.0-1\nbad-line-no-version\n";
        let map = parse_pacman_q(text);
        assert_eq!(map.get("linux").map(String::as_str), Some("6.9.1.arch1-1"));
        assert_eq!(map.get("firefox").map(String::as_str), Some("126.0-1"));
        assert_eq!(map.len(), 2);
    }
}
