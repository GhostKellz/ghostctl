//! `ghostctl openshell` - OpenShell readiness checks and CLI passthrough.
//!
//! OpenShell runs sandboxed, policy-governed runtimes for autonomous AI agents
//! behind a gateway control plane. ghostctl does not reimplement that CLI; it
//! provides a `doctor` that verifies prerequisites (binary, docker, gateway,
//! registration) and thin passthroughs to the `openshell` binary for the common
//! verbs so isolated-session workflows stay in one place.

pub mod config;

use anyhow::{Result, bail};
use clap::{Arg, ArgMatches, Command};
use config::OpenshellConfig;
use std::path::PathBuf;
use std::process::Command as ProcCommand;
use std::time::Duration;

use crate::utils::is_plain_mode;

pub fn command() -> Command {
    Command::new("openshell")
        .about("OpenShell sandbox runtime: readiness checks and CLI passthrough")
        .subcommand(
            Command::new("doctor")
                .about("Check OpenShell prerequisites (binary, docker, gateway, registration)"),
        )
        .subcommand(Command::new("status").about("Show active gateway connection (passthrough)"))
        .subcommand(
            Command::new("gateway")
                .about("Manage gateways (passthrough to `openshell gateway`)")
                .arg(passthrough_args()),
        )
        .subcommand(
            Command::new("sandbox")
                .about("Manage isolated sandboxes (passthrough to `openshell sandbox`)")
                .arg(passthrough_args()),
        )
        .subcommand(
            Command::new("policy")
                .about("Manage sandbox policy (passthrough to `openshell policy`)")
                .arg(passthrough_args()),
        )
}

fn passthrough_args() -> Arg {
    Arg::new("args")
        .num_args(0..)
        .trailing_var_arg(true)
        .allow_hyphen_values(true)
        .help("Arguments forwarded to the `openshell` CLI")
}

pub fn handle(matches: &ArgMatches) -> Result<()> {
    let cfg = OpenshellConfig::load();

    match matches.subcommand() {
        Some(("doctor", _)) => doctor(&cfg),
        Some(("status", _)) => passthrough(&cfg.bin, &["status".to_string()]),
        Some(("gateway", m)) => passthrough(&cfg.bin, &forwarded("gateway", m)),
        Some(("sandbox", m)) => passthrough(&cfg.bin, &forwarded("sandbox", m)),
        Some(("policy", m)) => passthrough(&cfg.bin, &forwarded("policy", m)),
        _ => {
            println!("Use `ghostctl openshell --help` to see available subcommands.");
            Ok(())
        }
    }
}

/// Prefix the forwarded args with the verb so `openshell <verb> <args...>` runs.
fn forwarded(verb: &str, m: &ArgMatches) -> Vec<String> {
    let mut out = vec![verb.to_string()];
    if let Some(vals) = m.get_many::<String>("args") {
        out.extend(vals.cloned());
    }
    out
}

/// Run `openshell <args...>`, inheriting stdio so interactive flows work.
fn passthrough(bin: &str, args: &[String]) -> Result<()> {
    if which::which(bin).is_err() {
        bail!(
            "openshell CLI '{}' not found in PATH. Build it from source (mise) or set \
             [openshell].bin in config. See `ghostctl openshell doctor`.",
            bin
        );
    }
    let status = ProcCommand::new(bin)
        .args(args)
        .status()
        .map_err(|e| anyhow::anyhow!("failed to launch {bin}: {e}"))?;
    if !status.success() {
        if let Some(code) = status.code() {
            std::process::exit(code);
        }
        bail!("{bin} terminated by signal");
    }
    Ok(())
}

fn doctor(cfg: &OpenshellConfig) -> Result<()> {
    println!("OpenShell readiness");
    println!("───────────────────");

    let bin_ok = which::which(&cfg.bin).is_ok();
    mark(bin_ok, &format!("openshell binary on PATH ({})", cfg.bin));
    if !bin_ok {
        println!(
            "    Build from source with `mise run gateway`, or `uv tool install -U openshell`."
        );
    }

    let docker_present = which::which("docker").is_ok();
    mark(docker_present, "docker binary on PATH");
    let docker_up = docker_present && docker_daemon_up();
    mark(docker_up, "docker daemon reachable");

    let gw_up = gateway_reachable(&cfg.gateway_url, cfg.timeout_secs);
    mark(gw_up, &format!("gateway reachable ({})", cfg.gateway_url));
    if !gw_up {
        println!("    Start it with `mise run gateway` (runs in the foreground).");
    }

    match active_gateway() {
        Some(name) => mark(true, &format!("active gateway registered: {name}")),
        None => {
            mark(false, "active gateway registered");
            println!("    Select one with `openshell gateway select <name>`.");
        }
    }

    let registry_ok = gateways_dir().map(|p| p.is_dir()).unwrap_or(false);
    mark(
        registry_ok,
        "CLI gateway registry present (~/.config/openshell)",
    );

    println!();
    if bin_ok && docker_up && gw_up {
        println!("OpenShell looks ready. Create an isolated session:");
        println!("  ghostctl openshell sandbox create -- claude");
    } else {
        println!("Resolve the ✗ items above, then re-run `ghostctl openshell doctor`.");
    }
    Ok(())
}

fn docker_daemon_up() -> bool {
    ProcCommand::new("docker")
        .args(["info", "--format", "{{.ServerVersion}}"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Any HTTP response (including 404) means the gateway is listening; only a
/// transport error counts as down.
fn gateway_reachable(url: &str, timeout_secs: u64) -> bool {
    reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .user_agent("ghostctl")
        .build()
        .ok()
        .and_then(|c| c.get(url).send().ok())
        .is_some()
}

fn config_home() -> Option<PathBuf> {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME")
        && !xdg.is_empty()
    {
        return Some(PathBuf::from(xdg));
    }
    std::env::var("HOME")
        .ok()
        .map(|h| PathBuf::from(h).join(".config"))
}

fn gateways_dir() -> Option<PathBuf> {
    config_home().map(|c| c.join("openshell").join("gateways"))
}

fn active_gateway() -> Option<String> {
    let pointer = config_home()?.join("openshell").join("active_gateway");
    let name = std::fs::read_to_string(pointer).ok()?.trim().to_string();
    if name.is_empty() { None } else { Some(name) }
}

fn mark(ok: bool, label: &str) {
    let icon = if is_plain_mode() {
        if ok { "[OK]" } else { "[--]" }
    } else if ok {
        "✓"
    } else {
        "✗"
    };
    println!("  {icon} {label}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::ArgMatches;

    fn matches_for(args: &[&str]) -> ArgMatches {
        command().get_matches_from(std::iter::once("openshell").chain(args.iter().copied()))
    }

    #[test]
    fn test_forwarded_prefixes_verb() {
        let m = matches_for(&["sandbox", "list"]);
        let (_, sub) = m.subcommand().unwrap();
        assert_eq!(forwarded("sandbox", sub), vec!["sandbox", "list"]);
    }

    #[test]
    fn test_forwarded_verb_only() {
        let m = matches_for(&["gateway"]);
        let (_, sub) = m.subcommand().unwrap();
        assert_eq!(forwarded("gateway", sub), vec!["gateway"]);
    }

    #[test]
    fn test_forwarded_passes_hyphen_flags() {
        let m = matches_for(&["sandbox", "create", "--from", "ollama"]);
        let (_, sub) = m.subcommand().unwrap();
        assert_eq!(
            forwarded("sandbox", sub),
            vec!["sandbox", "create", "--from", "ollama"]
        );
    }
}
