//! UniFi remote adoption.
//!
//! Points factory-default devices at the controller so they show up as pending
//! adoption. Mechanism: discover devices listening on 10001/udp, then SSH in and
//! run `mca-cli-op set-inform http://<controller>:<port>/inform`.
//!
//! Requirements at runtime: this must run from a host with L2/L3 reachability to
//! the device subnet on tcp/22, and the devices must be able to reach the
//! controller on the inform port. `nmap` and `ssh` are required; `sshpass` is
//! only needed for password auth (key auth is preferred).

use anyhow::{Context, Result, bail};
use std::collections::BTreeSet;
use std::net::IpAddr;
use std::process::Command;

use super::config::UnifiConfig;

struct AdoptResult {
    ip: String,
    ok: bool,
    detail: String,
}

pub fn run(
    cfg: &UnifiConfig,
    subnet: &str,
    controller: Option<&str>,
    inform_port: Option<u16>,
    user_override: Option<&str>,
) -> Result<()> {
    let inform_host = match controller {
        Some(host) => UnifiConfig::validate_inform_host(host).map_err(anyhow::Error::msg)?,
        None => cfg.effective_inform_host().map_err(anyhow::Error::msg)?,
    };
    let port = inform_port.unwrap_or(cfg.inform_port);
    let inform_url = format!("http://{}:{port}/inform", inform_url_host(&inform_host));

    let users: Vec<String> = match user_override {
        Some(u) => vec![u.to_string()],
        None => cfg.adopt_ssh_users.clone(),
    };
    if users.is_empty() {
        bail!("no SSH users configured for adoption (set [unifi].adopt_ssh_users or --user)");
    }

    let password = cfg.resolve_adopt_password();
    let key = cfg.adopt_ssh_key.clone();

    // Preflight: required tools.
    let runner = crate::command::runner();
    if !runner.command_exists("nmap") {
        bail!("`nmap` is required for device discovery but was not found in PATH");
    }
    if !runner.command_exists("ssh") {
        bail!("`ssh` is required but was not found in PATH");
    }
    if key.is_none() && password.is_some() && !runner.command_exists("sshpass") {
        bail!("`sshpass` is required for password auth but was not found in PATH");
    }
    if key.is_none() && password.is_none() {
        bail!(
            "no adoption credential configured — set [unifi].adopt_ssh_key or \
             UNIFI_ADOPT_PASSWORD/[unifi].adopt_ssh_password"
        );
    }

    println!("Discovering UniFi devices on {subnet} (udp/10001)...");
    let devices = discover(subnet, runner.is_root())?;
    if devices.is_empty() {
        println!("No unadopted devices found listening on 10001/udp.");
        return Ok(());
    }
    println!(
        "Found {} device(s): {}",
        devices.len(),
        devices_line(&devices)
    );
    println!("Inform target: {inform_url}");

    let dry_run = crate::utils::is_dry_run();
    if dry_run {
        println!("\nDRY-RUN: no changes will be made.");
    } else if !crate::utils::is_headless() && !crate::tui::is_auto_yes() {
        let proceed = dialoguer::Confirm::new()
            .with_prompt(format!(
                "Point {} device(s) at {}?",
                devices.len(),
                inform_url
            ))
            .default(false)
            .interact()
            .unwrap_or(false);
        if !proceed {
            println!("Aborted.");
            return Ok(());
        }
    }

    let mut results = Vec::new();
    for ip in &devices {
        results.push(adopt_one(
            ip,
            &inform_url,
            &users,
            key.as_deref(),
            password.as_deref(),
            dry_run,
        ));
    }

    println!("\nResults:");
    let mut ok = 0usize;
    for r in &results {
        let tag = if r.ok { "OK  " } else { "FAIL" };
        println!("  [{tag}] {:<15} {}", r.ip, r.detail);
        if r.ok {
            ok += 1;
        }
    }
    println!("{ok}/{} adopted (set-inform sent).", results.len());
    Ok(())
}

/// Run nmap and return the sorted, de-duplicated set of device IPs.
fn discover(subnet: &str, is_root: bool) -> Result<Vec<String>> {
    // `-sU` requires root; use sudo when we're not already root.
    let mut cmd = if is_root {
        Command::new("nmap")
    } else {
        let mut c = Command::new("sudo");
        c.arg("nmap");
        c
    };
    cmd.args(["-n", "-sU", "-p10001,22", subnet, "-oG", "-"]);
    let out = cmd
        .output()
        .context("failed to run nmap (is it installed and, if non-root, is sudo available?)")?;
    if !out.status.success() {
        bail!(
            "nmap failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    let text = String::from_utf8_lossy(&out.stdout);
    Ok(parse_nmap(&text))
}

/// Parse nmap grepable output for hosts with 10001 open (device discovery beacon).
fn parse_nmap(grepable: &str) -> Vec<String> {
    let mut ips: BTreeSet<String> = BTreeSet::new();
    for line in grepable.lines() {
        if !line.starts_with("Host:") || !line.contains("10001/open") {
            continue;
        }
        // Format: `Host: <ip> (<name>)\tPorts: ...`
        if let Some(ip) = line.split_whitespace().nth(1) {
            ips.insert(ip.to_string());
        }
    }
    ips.into_iter().collect()
}

fn devices_line(devices: &[String]) -> String {
    devices.join(", ")
}

fn adopt_one(
    ip: &str,
    inform_url: &str,
    users: &[String],
    key: Option<&str>,
    password: Option<&str>,
    dry_run: bool,
) -> AdoptResult {
    let remote_cmd = set_inform_command(inform_url);
    for user in users {
        if dry_run {
            let auth = if key.is_some() { "key" } else { "password" };
            return AdoptResult {
                ip: ip.to_string(),
                ok: true,
                detail: format!("would run `{remote_cmd}` as {user}@{ip} ({auth} auth)"),
            };
        }

        let mut cmd = build_ssh(ip, user, key, password.is_some());
        cmd.arg(&remote_cmd);
        if let Some(pw) = password {
            // sshpass -e reads the password from SSHPASS, keeping it out of argv.
            cmd.env("SSHPASS", pw);
        }

        match cmd.output() {
            Ok(out) if out.status.success() => {
                return AdoptResult {
                    ip: ip.to_string(),
                    ok: true,
                    detail: format!("set-inform sent as {user}"),
                };
            }
            Ok(_) => continue, // try next user (likely auth failure)
            Err(e) => {
                return AdoptResult {
                    ip: ip.to_string(),
                    ok: false,
                    detail: format!("ssh error: {e}"),
                };
            }
        }
    }
    AdoptResult {
        ip: ip.to_string(),
        ok: false,
        detail: "all credentials failed".to_string(),
    }
}

fn set_inform_command(inform_url: &str) -> String {
    format!("mca-cli-op set-inform {}", shell_single_quote(inform_url))
}

fn inform_url_host(host: &str) -> String {
    match host.parse::<IpAddr>() {
        Ok(IpAddr::V6(_)) => format!("[{host}]"),
        _ => host.to_string(),
    }
}

fn shell_single_quote(value: &str) -> String {
    let mut quoted = String::with_capacity(value.len() + 2);
    quoted.push('\'');
    for ch in value.chars() {
        if ch == '\'' {
            quoted.push_str("'\\''");
        } else {
            quoted.push(ch);
        }
    }
    quoted.push('\'');
    quoted
}

/// Build the ssh invocation (wrapped in sshpass for password auth).
fn build_ssh(ip: &str, user: &str, key: Option<&str>, use_password: bool) -> Command {
    let mut cmd = if use_password {
        let mut c = Command::new("sshpass");
        c.arg("-e").arg("ssh");
        c
    } else {
        Command::new("ssh")
    };
    cmd.args([
        "-q",
        "-o",
        "StrictHostKeyChecking=no",
        "-o",
        "UserKnownHostsFile=/dev/null",
        "-o",
        "ConnectTimeout=5",
    ]);
    if let Some(k) = key {
        cmd.args(["-o", "BatchMode=yes", "-i", k]);
    }
    cmd.arg(format!("{user}@{ip}"));
    cmd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nmap_extracts_open_hosts() {
        let sample = "\
# Nmap done
Host: 192.168.1.20 ()\tPorts: 22/open/tcp//ssh///, 10001/open|filtered/udp//////\tIgnored State: closed (0)
Host: 192.168.1.21 ()\tPorts: 22/closed/tcp//ssh///, 10001/closed/udp//////
Host: 192.168.1.22 ()\tPorts: 10001/open/udp//////
";
        let ips = parse_nmap(sample);
        assert_eq!(ips, vec!["192.168.1.20", "192.168.1.22"]);
    }

    #[test]
    fn test_parse_nmap_dedupes_and_sorts() {
        let sample = "\
Host: 10.0.0.5 ()\tPorts: 10001/open/udp
Host: 10.0.0.5 ()\tPorts: 10001/open|filtered/udp
Host: 10.0.0.2 ()\tPorts: 10001/open/udp
";
        let ips = parse_nmap(sample);
        assert_eq!(ips, vec!["10.0.0.2", "10.0.0.5"]);
    }

    #[test]
    fn test_set_inform_command_quotes_url_for_remote_shell() {
        assert_eq!(
            set_inform_command("http://unifi.example.com:8080/inform"),
            "mca-cli-op set-inform 'http://unifi.example.com:8080/inform'"
        );
        assert_eq!(
            set_inform_command("http://host'bad:8080/inform"),
            "mca-cli-op set-inform 'http://host'\\''bad:8080/inform'"
        );
    }

    #[test]
    fn test_inform_url_host_brackets_ipv6_literals() {
        assert_eq!(inform_url_host("unifi.example.com"), "unifi.example.com");
        assert_eq!(inform_url_host("192.168.1.10"), "192.168.1.10");
        assert_eq!(inform_url_host("fd00::1"), "[fd00::1]");
    }
}
