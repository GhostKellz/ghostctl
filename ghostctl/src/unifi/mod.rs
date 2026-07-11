//! UniFi OS Server integration.
//!
//! Read-only status/inventory plus remote adoption and cross-vendor diagnostics
//! for a self-hosted UniFi OS Server controller. Targets the current UOS Server
//! (HTTPS :11443, API-key auth) — not the legacy Network Application (EOL).

pub mod adopt;
pub mod client;
pub mod config;
pub mod doctor;

use anyhow::Result;
use clap::{Arg, ArgAction, ArgMatches, Command};

use client::UnifiClient;
use config::UnifiConfig;

pub fn command() -> Command {
    Command::new("unifi")
        .about("UniFi OS Server: status, inventory, remote adoption, diagnostics")
        .subcommand(Command::new("status").about("Verify controller connectivity and API-key auth"))
        .subcommand(
            Command::new("devices")
                .about("List adopted devices and their state")
                .arg(
                    Arg::new("pending")
                        .long("pending")
                        .action(ArgAction::SetTrue)
                        .help("Show only devices pending adoption"),
                )
                .arg(
                    Arg::new("offline")
                        .long("offline")
                        .action(ArgAction::SetTrue)
                        .help("Show only devices that are not online"),
                ),
        )
        .subcommand(
            Command::new("adopt")
                .about("Remotely adopt factory devices via SSH set-inform")
                .long_about(
                    "Discover factory-default UniFi devices on a subnet (udp/10001) and \
                     point them at the controller via `mca-cli-op set-inform`.\n\n\
                     Must be run from a host that can reach the device subnet on tcp/22, \
                     and the devices must be able to reach the controller on the inform \
                     port. Requires nmap and ssh (sshpass only for password auth).",
                )
                .arg(
                    Arg::new("subnet")
                        .long("subnet")
                        .required(true)
                        .help("CIDR to scan for devices, e.g. 192.168.1.0/24"),
                )
                .arg(
                    Arg::new("controller")
                        .long("controller")
                        .help("Override the inform host (defaults to the configured controller)"),
                )
                .arg(
                    Arg::new("inform-port")
                        .long("inform-port")
                        .help("Override the inform port (default 8080)"),
                )
                .arg(
                    Arg::new("user")
                        .long("user")
                        .help("Override SSH user (default tries configured ui/ubnt)"),
                ),
        )
        .subcommand(
            Command::new("doctor")
                .about("Diagnose STP, adoption, and firmware issues (Fortinet+UniFi shops)"),
        )
}

pub fn handle(matches: &ArgMatches) -> Result<()> {
    let cfg = UnifiConfig::load();

    match matches.subcommand() {
        Some(("status", _)) => status(&cfg),
        Some(("devices", m)) => devices(&cfg, m.get_flag("pending"), m.get_flag("offline")),
        Some(("adopt", m)) => {
            let subnet = m.get_one::<String>("subnet").expect("subnet is required");
            let controller = m.get_one::<String>("controller").map(String::as_str);
            let inform_port = m
                .get_one::<String>("inform-port")
                .and_then(|p| p.parse::<u16>().ok());
            let user = m.get_one::<String>("user").map(String::as_str);
            adopt::run(&cfg, subnet, controller, inform_port, user)
        }
        Some(("doctor", _)) => doctor::run(&cfg),
        _ => {
            println!("Use `ghostctl unifi --help` to see available subcommands.");
            Ok(())
        }
    }
}

fn status(cfg: &UnifiConfig) -> Result<()> {
    let client = UnifiClient::new(cfg)?;
    let sites = client.list_sites()?;
    let items = sites.get("data").and_then(|d| d.as_array());
    let count = items.map(|a| a.len()).unwrap_or(0);

    println!("UniFi controller: {}", cfg.base());
    println!("  auth:  OK (API key accepted)");
    println!("  sites: {count}");
    if let Some(items) = items {
        for s in items {
            let name = s.get("name").and_then(|n| n.as_str()).unwrap_or("?");
            let id = s.get("id").and_then(|i| i.as_str()).unwrap_or("?");
            let marker = if name.eq_ignore_ascii_case(&cfg.site) {
                " (configured)"
            } else {
                ""
            };
            println!("    - {name}{marker}  [{id}]");
        }
    }
    Ok(())
}

fn devices(cfg: &UnifiConfig, pending_only: bool, offline_only: bool) -> Result<()> {
    let client = UnifiClient::new(cfg)?;
    let site_id = client.resolve_site_id()?;
    let devices = client.list_devices(&site_id)?;
    let items = match devices.get("data").and_then(|d| d.as_array()) {
        Some(a) => a,
        None => {
            println!("No devices reported for site '{}'.", cfg.site);
            return Ok(());
        }
    };

    let mut shown = 0usize;
    println!("Devices on site '{}':", cfg.site);
    for d in items {
        let state = d
            .get("state")
            .and_then(|s| s.as_str())
            .unwrap_or("UNKNOWN")
            .to_string();
        let is_online = state.eq_ignore_ascii_case("online");
        let is_pending = state.eq_ignore_ascii_case("pending")
            || state.eq_ignore_ascii_case("pending_adoption")
            || state.eq_ignore_ascii_case("adopting");

        if pending_only && !is_pending {
            continue;
        }
        if offline_only && is_online {
            continue;
        }

        let name = d
            .get("name")
            .and_then(|n| n.as_str())
            .filter(|n| !n.is_empty())
            .unwrap_or("<unnamed>");
        let model = d.get("model").and_then(|m| m.as_str()).unwrap_or("?");
        let ip = d.get("ipAddress").and_then(|i| i.as_str()).unwrap_or("-");
        let mac = d.get("macAddress").and_then(|m| m.as_str()).unwrap_or("-");

        println!("  {name:<24} {model:<12} {state:<10} {ip:<15} {mac}");
        shown += 1;
    }

    if shown == 0 {
        println!("  (no devices matched the filter)");
    }
    Ok(())
}
