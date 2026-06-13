//! `ghostctl monitor` - observability helper for a Prometheus/Loki/Alertmanager/Grafana stack.
//!
//! All operations are read-only HTTP calls (plus the two `/-/reload` POSTs) against
//! services configured under `[monitor]` in the ghostctl config. Endpoints default
//! to localhost so the commands work on the monitoring host with zero config.

pub mod client;
pub mod config;
pub mod parse;

use anyhow::{Result, bail};
use clap::{Arg, ArgAction, ArgMatches, Command};
use client::MonitorClient;
use config::MonitorConfig;
use std::thread;
use std::time::Duration;

pub fn command() -> Command {
    Command::new("monitor")
        .visible_alias("mon")
        .about("Observability helper (Prometheus, Loki, Alertmanager, Grafana)")
        .subcommand(Command::new("health").about("Probe all configured services for liveness"))
        .subcommand(
            Command::new("targets")
                .about("List Prometheus scrape targets and their health")
                .arg(
                    Arg::new("down")
                        .long("down")
                        .action(ArgAction::SetTrue)
                        .help("Show only targets that are not up"),
                ),
        )
        .subcommand(Command::new("alerts").about("List alerts currently known to Alertmanager"))
        .subcommand(
            Command::new("logs")
                .about("Query Loki with a LogQL expression")
                .arg(
                    Arg::new("query")
                        .required(true)
                        .help("LogQL query, e.g. '{source_type=\"fortigate\"}'"),
                )
                .arg(
                    Arg::new("limit")
                        .long("limit")
                        .default_value("50")
                        .help("Maximum number of lines to return"),
                ),
        )
        .subcommand(
            Command::new("tail")
                .about("Follow new Loki log lines for a LogQL query")
                .arg(
                    Arg::new("query")
                        .required(true)
                        .help("LogQL query to follow"),
                )
                .arg(
                    Arg::new("follow")
                        .long("follow")
                        .short('f')
                        .action(ArgAction::SetTrue)
                        .help("Keep polling for new lines (Ctrl-C to stop)"),
                ),
        )
        .subcommand(
            Command::new("query")
                .about("Run a pre-baked PromQL query")
                .arg(
                    Arg::new("metric")
                        .required(true)
                        .value_parser(["cpu", "mem", "disk"])
                        .help("Which metric to query"),
                )
                .arg(
                    Arg::new("host")
                        .long("host")
                        .help("Filter results to instances containing this string"),
                ),
        )
        .subcommand(
            Command::new("reload")
                .about("Hot-reload a service config (no restart)")
                .arg(
                    Arg::new("service")
                        .required(true)
                        .value_parser(["prometheus", "alertmanager"])
                        .help("Service to reload"),
                ),
        )
        .subcommand(
            Command::new("datasources")
                .about("Check Grafana datasource health (needs grafana_token)"),
        )
}

pub fn handle(matches: &ArgMatches) -> Result<()> {
    let cfg = MonitorConfig::load();
    let mc = MonitorClient::new(cfg.timeout_secs)?;

    match matches.subcommand() {
        Some(("health", _)) => health(&cfg, &mc),
        Some(("targets", m)) => targets(&cfg, &mc, m.get_flag("down")),
        Some(("alerts", _)) => alerts(&cfg, &mc),
        Some(("logs", m)) => {
            let query = m.get_one::<String>("query").unwrap();
            let limit = m.get_one::<String>("limit").unwrap();
            logs(&cfg, &mc, query, limit)
        }
        Some(("tail", m)) => {
            let query = m.get_one::<String>("query").unwrap();
            tail(&cfg, &mc, query, m.get_flag("follow"))
        }
        Some(("query", m)) => {
            let metric = m.get_one::<String>("metric").unwrap();
            let host = m.get_one::<String>("host").map(String::as_str);
            query(&cfg, &mc, metric, host)
        }
        Some(("reload", m)) => {
            let service = m.get_one::<String>("service").unwrap();
            reload(&cfg, &mc, service)
        }
        Some(("datasources", _)) => datasources(&cfg, &mc),
        _ => {
            println!("Use `ghostctl monitor --help` to see available subcommands.");
            Ok(())
        }
    }
}

fn mark(up: bool) -> &'static str {
    if up { "✓" } else { "✗" }
}

fn health(cfg: &MonitorConfig, mc: &MonitorClient) -> Result<()> {
    println!("📊 Monitoring stack health");
    println!("──────────────────────────");

    let p = MonitorConfig::base(&cfg.prometheus_url);
    let l = MonitorConfig::base(&cfg.loki_url);
    let a = MonitorConfig::base(&cfg.alertmanager_url);
    let g = MonitorConfig::base(&cfg.grafana_url);

    let mut checks: Vec<(&str, String, bool)> = vec![
        (
            "Prometheus",
            cfg.prometheus_url.clone(),
            mc.is_up(&format!("{p}/-/healthy")),
        ),
        (
            "Loki",
            cfg.loki_url.clone(),
            mc.is_up(&format!("{l}/ready")),
        ),
        (
            "Alertmanager",
            cfg.alertmanager_url.clone(),
            mc.is_up(&format!("{a}/-/healthy")),
        ),
        (
            "Grafana",
            cfg.grafana_url.clone(),
            mc.is_up(&format!("{g}/api/health")),
        ),
    ];
    if let Some(ne) = &cfg.node_exporter_url {
        let url = MonitorConfig::base(ne);
        checks.push((
            "node_exporter",
            ne.clone(),
            mc.is_up(&format!("{url}/metrics")),
        ));
    }
    if let Some(ca) = &cfg.cadvisor_url {
        let url = MonitorConfig::base(ca);
        checks.push(("cAdvisor", ca.clone(), mc.is_up(&format!("{url}/healthz"))));
    }

    let mut all_up = true;
    for (name, url, up) in &checks {
        all_up &= up;
        println!("  {} {:<14} {}", mark(*up), name, url);
    }
    println!();
    if all_up {
        println!("All services reachable.");
    } else {
        println!("Some services are unreachable (see ✗ above).");
    }
    Ok(())
}

fn targets(cfg: &MonitorConfig, mc: &MonitorClient, only_down: bool) -> Result<()> {
    let url = format!(
        "{}/api/v1/targets",
        MonitorConfig::base(&cfg.prometheus_url)
    );
    let body = mc.get_text(&url)?;
    let mut targets = parse::parse_targets(&body)?;
    if only_down {
        targets.retain(|t| t.health != "up");
    }
    if targets.is_empty() {
        println!("No matching targets.");
        return Ok(());
    }
    println!("{:<4} {:<20} {:<24} SCRAPE URL", "", "JOB", "INSTANCE");
    for t in &targets {
        println!(
            "{:<4} {:<20} {:<24} {}",
            mark(t.health == "up"),
            t.job,
            t.instance,
            t.scrape_url
        );
        if t.health != "up" && !t.last_error.is_empty() {
            println!("       └─ {}", t.last_error);
        }
    }
    Ok(())
}

fn alerts(cfg: &MonitorConfig, mc: &MonitorClient) -> Result<()> {
    let url = format!(
        "{}/api/v2/alerts",
        MonitorConfig::base(&cfg.alertmanager_url)
    );
    let body = mc.get_text(&url)?;
    let alerts = parse::parse_alerts(&body)?;
    if alerts.is_empty() {
        println!("No active alerts. 🎉");
        return Ok(());
    }
    println!("{} alert(s):", alerts.len());
    for a in &alerts {
        let sev = if a.severity.is_empty() {
            "-".to_string()
        } else {
            a.severity.clone()
        };
        println!("  [{}] {} ({})", sev, a.name, a.state);
        if !a.summary.is_empty() {
            println!("        {}", a.summary);
        }
    }
    Ok(())
}

fn logs(cfg: &MonitorConfig, mc: &MonitorClient, query: &str, limit: &str) -> Result<()> {
    let url = format!(
        "{}/loki/api/v1/query_range",
        MonitorConfig::base(&cfg.loki_url)
    );
    let body = mc.get_text_query(&url, &[("query", query), ("limit", limit)])?;
    let lines = parse::parse_loki_lines(&body)?;
    if lines.is_empty() {
        println!("No log lines matched.");
        return Ok(());
    }
    for l in &lines {
        println!("{}", l.line);
    }
    Ok(())
}

fn tail(cfg: &MonitorConfig, mc: &MonitorClient, query: &str, follow: bool) -> Result<()> {
    let url = format!(
        "{}/loki/api/v1/query_range",
        MonitorConfig::base(&cfg.loki_url)
    );

    // Initial fetch
    let body = mc.get_text_query(&url, &[("query", query), ("limit", "50")])?;
    let lines = parse::parse_loki_lines(&body)?;
    let mut last_ts = lines.last().map(|l| l.timestamp_ns).unwrap_or(0);
    for l in &lines {
        println!("{}", l.line);
    }

    if !follow {
        return Ok(());
    }

    println!("(following — Ctrl-C to stop)");
    loop {
        thread::sleep(Duration::from_secs(2));
        // Loki `start` is inclusive nanoseconds; advance by 1 to avoid duplicates.
        let start = (last_ts + 1).to_string();
        let body = match mc.get_text_query(
            &url,
            &[
                ("query", query),
                ("limit", "100"),
                ("start", start.as_str()),
                ("direction", "forward"),
            ],
        ) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("poll error: {e}");
                continue;
            }
        };
        let new_lines = parse::parse_loki_lines(&body)?;
        for l in &new_lines {
            if l.timestamp_ns > last_ts {
                println!("{}", l.line);
                last_ts = l.timestamp_ns;
            }
        }
    }
}

fn query(cfg: &MonitorConfig, mc: &MonitorClient, metric: &str, host: Option<&str>) -> Result<()> {
    let promql = match metric {
        "cpu" => {
            "100 - (avg by (instance) (rate(node_cpu_seconds_total{mode=\"idle\"}[5m])) * 100)"
        }
        "mem" => "100 * (1 - (node_memory_MemAvailable_bytes / node_memory_MemTotal_bytes))",
        "disk" => {
            "100 * (1 - (node_filesystem_avail_bytes{mountpoint=\"/\"} / node_filesystem_size_bytes{mountpoint=\"/\"}))"
        }
        other => bail!("unknown metric '{other}'"),
    };

    let url = format!("{}/api/v1/query", MonitorConfig::base(&cfg.prometheus_url));
    let body = mc.get_text_query(&url, &[("query", promql)])?;
    let mut samples = parse::parse_instant_query(&body)?;

    if let Some(h) = host {
        samples.retain(|s| {
            s.labels
                .get("instance")
                .map(|i| i.contains(h))
                .unwrap_or(false)
        });
    }

    if samples.is_empty() {
        println!("No samples returned.");
        return Ok(());
    }

    let unit = "%";
    println!("{} usage:", metric);
    for s in &samples {
        let instance = s.labels.get("instance").cloned().unwrap_or_default();
        let val: f64 = s.value.parse().unwrap_or(f64::NAN);
        println!("  {:<24} {:>6.1}{}", instance, val, unit);
    }
    Ok(())
}

fn reload(cfg: &MonitorConfig, mc: &MonitorClient, service: &str) -> Result<()> {
    let base = match service {
        "prometheus" => MonitorConfig::base(&cfg.prometheus_url),
        "alertmanager" => MonitorConfig::base(&cfg.alertmanager_url),
        other => bail!("unknown service '{other}'"),
    };
    let url = format!("{base}/-/reload");
    mc.post_empty(&url)?;
    println!("✓ Reloaded {service} config");
    Ok(())
}

fn datasources(cfg: &MonitorConfig, mc: &MonitorClient) -> Result<()> {
    if cfg.grafana_token.is_none() {
        println!(
            "⚠ No grafana_token configured under [monitor]; datasource health needs admin credentials."
        );
        return Ok(());
    }
    let base = MonitorConfig::base(&cfg.grafana_url);
    let user = cfg.grafana_user.as_deref();
    let token = cfg.grafana_token.as_deref();

    println!("Grafana datasource health:");
    for uid in ["prometheus", "loki", "alertmanager", "wazuh"] {
        let url = format!("{base}/api/datasources/uid/{uid}/health");
        match mc.get_text_auth(&url, user, token) {
            Ok(body) => match parse::parse_datasource_health(&body) {
                Ok((status, msg)) => {
                    let ok = status.eq_ignore_ascii_case("ok");
                    println!("  {} {:<14} {} {}", mark(ok), uid, status, msg);
                }
                Err(_) => println!("  ? {:<14} (unparseable response)", uid),
            },
            Err(e) => println!("  ✗ {:<14} {}", uid, e),
        }
    }
    Ok(())
}
