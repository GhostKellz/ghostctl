//! `ghostctl crowdsec` - threat-intel and security-posture helper.
//!
//! From the workstation this wraps read-only checks: the public threat feed, the
//! CrowdSec LAPI Prometheus metrics endpoint, DNS resolver posture, and (when run
//! on the LAPI host) a passthrough to the local `cscli` binary.

pub mod config;

use anyhow::{Context, Result, bail};
use clap::{Arg, ArgMatches, Command};
use config::CrowdsecConfig;
use ipnet::IpNet;
use reqwest::blocking::Client;
use serde::Serialize;
use std::collections::BTreeSet;
use std::net::IpAddr;
use std::process::Command as ProcCommand;
use std::time::Duration;

pub fn command() -> Command {
    Command::new("crowdsec")
        .about("Threat-feed, CrowdSec metrics, and DNS posture checks")
        .subcommand(
            Command::new("feed")
                .about("Inspect the public threat feed")
                .subcommand(
                    Command::new("check").about("Fetch the feed and report entry count + size"),
                )
                .subcommand(
                    Command::new("sample")
                        .about("Show the first N entries of the feed")
                        .arg(
                            Arg::new("count")
                                .default_value("10")
                                .help("Number of entries to show"),
                        ),
                ),
        )
        .subcommand(
            Command::new("metrics")
                .about("Summarize CrowdSec LAPI Prometheus metrics (if configured)"),
        )
        .subcommand(
            Command::new("cli")
                .about("Passthrough to local cscli (only works on the LAPI host)")
                .arg(
                    Arg::new("category")
                        .required(true)
                        .value_parser(["decisions", "bouncers", "machines", "alerts"])
                        .help("cscli category to list"),
                ),
        )
        .subcommand(
            Command::new("dns")
                .about("Check DNS resolver reachability and DNSSEC")
                .subcommand(
                    Command::new("check").about("Test lookups against the configured resolvers"),
                ),
        )
        .subcommand(
            Command::new("unifi-exempt")
                .about("Generate a CrowdSec whitelist for UniFi mgmt/inform + Tailscale")
                .subcommand(
                    Command::new("generate")
                        .about("Print (or --apply) the UniFi whitelist parser YAML")
                        .arg(
                            Arg::new("apply")
                                .long("apply")
                                .num_args(0..=1)
                                .default_missing_value(
                                    "/etc/crowdsec/parsers/s02-enrich/unifi-whitelist.yaml",
                                )
                                .help("Write the whitelist to a file (default path if no value)"),
                        ),
                ),
        )
}

pub fn handle(matches: &ArgMatches) -> Result<()> {
    let cfg = CrowdsecConfig::load();

    match matches.subcommand() {
        Some(("feed", m)) => match m.subcommand() {
            Some(("check", _)) => feed_check(&cfg),
            Some(("sample", sm)) => {
                let count: usize = sm
                    .get_one::<String>("count")
                    .and_then(|c| c.parse().ok())
                    .unwrap_or(10);
                feed_sample(&cfg, count)
            }
            _ => {
                println!("Use `ghostctl crowdsec feed --help`.");
                Ok(())
            }
        },
        Some(("metrics", _)) => metrics(&cfg),
        Some(("cli", m)) => {
            let category = m.get_one::<String>("category").unwrap();
            cscli_passthrough(category)
        }
        Some(("dns", m)) => match m.subcommand() {
            Some(("check", _)) => dns_check(&cfg),
            _ => {
                println!("Use `ghostctl crowdsec dns check`.");
                Ok(())
            }
        },
        Some(("unifi-exempt", m)) => match m.subcommand() {
            Some(("generate", gm)) => {
                unifi_exempt_generate(gm.get_one::<String>("apply").map(String::as_str))
            }
            _ => {
                println!("Use `ghostctl crowdsec unifi-exempt generate`.");
                Ok(())
            }
        },
        _ => {
            println!("Use `ghostctl crowdsec --help` to see available subcommands.");
            Ok(())
        }
    }
}

fn http_client(timeout_secs: u64) -> Result<Client> {
    Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .user_agent("ghostctl")
        .build()
        .context("failed to build HTTP client")
}

fn fetch_feed(cfg: &CrowdsecConfig) -> Result<String> {
    let resp = http_client(cfg.timeout_secs)?
        .get(&cfg.threat_feed_url)
        .send()
        .with_context(|| format!("request failed: {}", cfg.threat_feed_url))?;
    let status = resp.status();
    let body = resp.text().unwrap_or_default();
    if !status.is_success() {
        bail!("HTTP {} from {}", status.as_u16(), cfg.threat_feed_url);
    }
    Ok(body)
}

fn feed_check(cfg: &CrowdsecConfig) -> Result<()> {
    println!("🛡  Threat feed: {}", cfg.threat_feed_url);
    let body = fetch_feed(cfg)?;
    let entries = count_feed_entries(&body);
    println!("  Entries : {entries}");
    println!("  Size    : {} bytes", body.len());
    if let Some(notafter) = cert_not_after(&cfg.threat_feed_url, cfg.timeout_secs) {
        println!("  TLS cert: expires {notafter}");
    }
    Ok(())
}

fn feed_sample(cfg: &CrowdsecConfig, count: usize) -> Result<()> {
    let body = fetch_feed(cfg)?;
    let entries: Vec<&str> = feed_entries(&body).take(count).collect();
    if entries.is_empty() {
        println!("Feed is empty.");
        return Ok(());
    }
    for e in entries {
        println!("{e}");
    }
    Ok(())
}

fn metrics(cfg: &CrowdsecConfig) -> Result<()> {
    let url = match &cfg.lapi_metrics_url {
        Some(u) => u,
        None => {
            println!(
                "⚠ No [crowdsec].lapi_metrics_url configured (e.g. http://10.0.0.23:6060/metrics)."
            );
            return Ok(());
        }
    };
    let resp = http_client(cfg.timeout_secs)?
        .get(url)
        .send()
        .with_context(|| format!("request failed: {url}"))?;
    let status = resp.status();
    let body = resp.text().unwrap_or_default();
    if !status.is_success() {
        bail!("HTTP {} from {}", status.as_u16(), url);
    }

    println!("📈 CrowdSec LAPI metrics ({url})");
    let interesting = [
        ("cs_active_decisions", "Active decisions"),
        ("cs_alerts", "Alerts"),
        ("cs_lapi_machine_requests_total", "Machine requests"),
        ("cs_lapi_bouncer_requests_total", "Bouncer requests"),
    ];
    let mut any = false;
    for (metric, label) in interesting {
        if let Some(sum) = sum_prom_metric(&body, metric) {
            println!("  {:<18} {}", label, sum as u64);
            any = true;
        }
    }
    if !any {
        println!("  (no recognized cs_* metrics found in response)");
    }
    Ok(())
}

fn cscli_passthrough(category: &str) -> Result<()> {
    if which::which("cscli").is_err() {
        bail!("cscli not found in PATH. This command only works on the CrowdSec LAPI host.");
    }
    let status = ProcCommand::new("cscli")
        .args([category, "list"])
        .status()
        .map_err(|e| anyhow::anyhow!("failed to launch cscli: {e}"))?;
    if !status.success() {
        bail!("cscli {category} list failed");
    }
    Ok(())
}

fn dns_check(cfg: &CrowdsecConfig) -> Result<()> {
    if which::which("dig").is_err() {
        bail!("`dig` not found in PATH (install bind/bind-tools) for DNS checks.");
    }
    println!("🔎 DNS posture check (test domain: example.com)");

    let mut resolvers = vec![("primary", cfg.dns_primary.clone())];
    if let Some(b) = &cfg.dns_backup {
        resolvers.push(("backup", b.clone()));
    }

    for (label, server) in resolvers {
        let resolves = dig_resolves(&server, "example.com");
        let dnssec = dig_dnssec_ok(&server, "cloudflare.com");
        println!(
            "  {} {} ({}): resolve {}, DNSSEC {}",
            if resolves { "✓" } else { "✗" },
            label,
            server,
            if resolves { "ok" } else { "FAILED" },
            if dnssec { "validated" } else { "not validated" }
        );
    }
    Ok(())
}

// ---- helpers ----

/// Iterate over feed entries, skipping blank lines and comments.
fn feed_entries(body: &str) -> impl Iterator<Item = &str> {
    body.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
}

fn count_feed_entries(body: &str) -> usize {
    feed_entries(body).count()
}

/// Sum all samples of a Prometheus counter/gauge by metric name (ignores labels).
fn sum_prom_metric(body: &str, metric: &str) -> Option<f64> {
    let mut total = 0.0;
    let mut found = false;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // Match `metric` or `metric{...}` exactly at the start.
        let rest = match line.strip_prefix(metric) {
            Some(r) => r,
            None => continue,
        };
        let after = rest.starts_with('{') || rest.starts_with(' ') || rest.starts_with('\t');
        if !after {
            continue; // avoid matching metric prefixes (e.g. cs_alerts vs cs_alerts_total)
        }
        if let Some(value) = line.split_whitespace().last()
            && let Ok(v) = value.parse::<f64>()
        {
            total += v;
            found = true;
        }
    }
    if found { Some(total) } else { None }
}

/// Best-effort TLS notAfter via `openssl`; returns None if unavailable.
fn cert_not_after(url: &str, _timeout_secs: u64) -> Option<String> {
    if which::which("openssl").is_err() {
        return None;
    }
    let host_port = url.strip_prefix("https://")?.split('/').next()?.to_string();
    let host_port = if host_port.contains(':') {
        host_port
    } else {
        format!("{host_port}:443")
    };
    let servername = host_port.split(':').next().unwrap_or("").to_string();

    let output = ProcCommand::new("openssl")
        .args([
            "s_client",
            "-connect",
            &host_port,
            "-servername",
            &servername,
        ])
        .arg("-verify_quiet")
        .stdin(std::process::Stdio::null())
        .output()
        .ok()?;

    let text = String::from_utf8_lossy(&output.stdout);
    // Pipe through openssl x509 -enddate would be cleaner, but parse inline to
    // avoid a second process: look for the notAfter line if present.
    text.lines()
        .find(|l| l.contains("NotAfter") || l.contains("notAfter"))
        .map(|l| {
            l.split(':')
                .skip(1)
                .collect::<Vec<_>>()
                .join(":")
                .trim()
                .to_string()
        })
}

fn dig_resolves(server: &str, domain: &str) -> bool {
    ProcCommand::new("dig")
        .args([
            &format!("@{server}"),
            domain,
            "+short",
            "+time=3",
            "+tries=1",
        ])
        .output()
        .map(|o| o.status.success() && !o.stdout.is_empty())
        .unwrap_or(false)
}

fn dig_dnssec_ok(server: &str, domain: &str) -> bool {
    ProcCommand::new("dig")
        .args([
            &format!("@{server}"),
            domain,
            "+dnssec",
            "+time=3",
            "+tries=1",
        ])
        .output()
        .map(|o| {
            // The `ad` (authenticated data) flag indicates DNSSEC validation.
            String::from_utf8_lossy(&o.stdout).contains("flags:")
                && String::from_utf8_lossy(&o.stdout).contains(" ad;")
        })
        .unwrap_or(false)
}

/// Build and print (or write) a CrowdSec whitelist parser for UniFi.
///
/// Whitelists the configured exempt CIDRs (Tailscale CGNAT + mgmt subnets) and
/// the controller/inform host so legitimate device check-ins and admin access
/// are never banned by the bouncer sitting in front of the exposed frontend.
fn unifi_exempt_generate(apply: Option<&str>) -> Result<()> {
    let ucfg = crate::unifi::config::UnifiConfig::load();
    let (cidrs, ips) = collect_unifi_exempt_lists(&ucfg)?;
    let yaml = render_unifi_whitelist(&cidrs, &ips)?;

    match apply {
        Some(path) => {
            if crate::utils::is_dry_run() {
                println!("DRY-RUN: would write UniFi whitelist to {path}:\n\n{yaml}");
                return Ok(());
            }
            std::fs::write(path, &yaml)
                .with_context(|| format!("failed to write {path} (try running with sudo)"))?;
            println!("Wrote UniFi whitelist to {path}");
            println!("Reload CrowdSec to apply: sudo systemctl reload crowdsec");
        }
        None => print!("{yaml}"),
    }
    Ok(())
}

fn collect_unifi_exempt_lists(
    ucfg: &crate::unifi::config::UnifiConfig,
) -> Result<(Vec<String>, Vec<String>)> {
    let mut cidrs: BTreeSet<String> = BTreeSet::new();
    let mut ips: BTreeSet<String> = BTreeSet::new();
    for entry in &ucfg.exempt_cidrs {
        let e = entry.trim();
        if e.is_empty() {
            continue;
        }
        if e.contains('/') {
            let net: IpNet = e
                .parse()
                .with_context(|| format!("invalid [unifi].exempt_cidrs entry '{e}'"))?;
            cidrs.insert(net.to_string());
        } else {
            let ip: IpAddr = e
                .parse()
                .with_context(|| format!("invalid [unifi].exempt_cidrs entry '{e}'"))?;
            ips.insert(ip.to_string());
        }
    }

    // Add the controller/inform host when it is a literal IP (hostnames can't go
    // in a CrowdSec ip whitelist — those must be resolved first).
    for host in [
        ucfg.controller_host().map_err(anyhow::Error::msg)?,
        ucfg.effective_inform_host().map_err(anyhow::Error::msg)?,
    ] {
        if let Ok(ip) = host.parse::<IpAddr>() {
            ips.insert(ip.to_string());
        }
    }

    let cidrs: Vec<String> = cidrs.into_iter().collect();
    let ips: Vec<String> = ips.into_iter().collect();
    Ok((cidrs, ips))
}

#[derive(Debug, Serialize)]
struct CrowdsecWhitelistParser {
    name: &'static str,
    description: &'static str,
    whitelist: CrowdsecWhitelist,
}

#[derive(Debug, Serialize)]
struct CrowdsecWhitelist {
    reason: &'static str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    ip: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    cidr: Vec<String>,
}

/// Render a CrowdSec whitelist parser (s02-enrich stage) as YAML.
fn render_unifi_whitelist(cidrs: &[String], ips: &[String]) -> Result<String> {
    let doc = CrowdsecWhitelistParser {
        name: "ghostctl/unifi-whitelist",
        description: "Whitelist UniFi mgmt/inform + Tailscale so legit traffic is never banned",
        whitelist: CrowdsecWhitelist {
            reason: "UniFi controller management, inform sources, and Tailscale",
            ip: ips.to_vec(),
            cidr: cidrs.to_vec(),
        },
    };
    serde_yaml::to_string(&doc).context("failed to render CrowdSec whitelist YAML")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_unifi_whitelist_has_cidr_and_ip() {
        let yaml = render_unifi_whitelist(
            &["100.64.0.0/10".to_string(), "10.0.0.0/24".to_string()],
            &["69.169.98.98".to_string()],
        )
        .unwrap();
        assert!(yaml.contains("name: ghostctl/unifi-whitelist"));
        assert!(yaml.contains("cidr:"));
        assert!(yaml.contains("- 100.64.0.0/10"));
        assert!(yaml.contains("ip:"));
        assert!(yaml.contains("- 69.169.98.98"));
    }

    #[test]
    fn test_render_unifi_whitelist_omits_empty_sections() {
        let yaml = render_unifi_whitelist(&["100.64.0.0/10".to_string()], &[]).unwrap();
        assert!(yaml.contains("cidr:"));
        assert!(!yaml.contains("\n  ip:\n"));
    }

    #[test]
    fn test_render_unifi_whitelist_escapes_strings_as_yaml_data() {
        let yaml = render_unifi_whitelist(&["10.0.0.0/24\"\n  ip:\n  - 1.2.3.4".to_string()], &[])
            .unwrap();
        let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(
            parsed["whitelist"]["cidr"][0].as_str(),
            Some("10.0.0.0/24\"\n  ip:\n  - 1.2.3.4")
        );
    }

    #[test]
    fn test_collect_unifi_exempt_lists_rejects_invalid_entries() {
        let cfg = crate::unifi::config::UnifiConfig {
            exempt_cidrs: vec!["10.0.0.0/24\n  ip:\n  - 1.2.3.4".to_string()],
            ..Default::default()
        };
        assert!(collect_unifi_exempt_lists(&cfg).is_err());
    }

    #[test]
    fn test_collect_unifi_exempt_lists_adds_literal_controller_ips() {
        let cfg = crate::unifi::config::UnifiConfig {
            controller_url: "https://10.0.0.10:11443".to_string(),
            inform_host: Some("10.0.0.11".to_string()),
            exempt_cidrs: vec!["100.64.0.0/10".to_string()],
            ..Default::default()
        };
        let (cidrs, ips) = collect_unifi_exempt_lists(&cfg).unwrap();
        assert_eq!(cidrs, vec!["100.64.0.0/10"]);
        assert_eq!(ips, vec!["10.0.0.10", "10.0.0.11"]);
    }

    #[test]
    fn test_count_feed_entries() {
        let body = "# comment\n1.2.3.4\n5.6.7.0/24\n\n  10.0.0.1  \n# another\n";
        assert_eq!(count_feed_entries(body), 3);
    }

    #[test]
    fn test_feed_sample_skips_comments() {
        let body = "#h\n1.1.1.1\n2.2.2.2\n";
        let v: Vec<&str> = feed_entries(body).collect();
        assert_eq!(v, vec!["1.1.1.1", "2.2.2.2"]);
    }

    #[test]
    fn test_sum_prom_metric() {
        let body = "\
# HELP cs_active_decisions number of active decisions
# TYPE cs_active_decisions gauge
cs_active_decisions{origin=\"crowdsec\"} 1000
cs_active_decisions{origin=\"CAPI\"} 74000
cs_alerts 5
";
        assert_eq!(sum_prom_metric(body, "cs_active_decisions"), Some(75000.0));
        assert_eq!(sum_prom_metric(body, "cs_alerts"), Some(5.0));
        assert_eq!(sum_prom_metric(body, "cs_missing"), None);
    }

    #[test]
    fn test_sum_prom_metric_no_prefix_collision() {
        let body = "cs_alerts_total 99\ncs_alerts 3\n";
        // cs_alerts must not also match cs_alerts_total
        assert_eq!(sum_prom_metric(body, "cs_alerts"), Some(3.0));
        assert_eq!(sum_prom_metric(body, "cs_alerts_total"), Some(99.0));
    }
}
