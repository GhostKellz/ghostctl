//! Cross-vendor UniFi diagnostics.
//!
//! Focuses on the L2/adoption problems the UniFi UI can't correlate on its own,
//! which is exactly where a Fortinet + UniFi shop loses time: STP-blocked ports,
//! adoption/migration failures, and firmware skew.
//!
//! Deep data comes from the PRIVATE `/stat/device` endpoint, which Ubiquiti does
//! not guarantee across UOS upgrades — so this degrades gracefully to the
//! documented integration API when the private surface is unavailable.

use anyhow::Result;
use serde_json::Value;

use super::client::UnifiClient;
use super::config::UnifiConfig;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Severity {
    Crit,
    Warn,
    Info,
}

impl Severity {
    fn tag(self) -> &'static str {
        match self {
            Severity::Crit => "CRIT",
            Severity::Warn => "WARN",
            Severity::Info => "INFO",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Finding {
    severity: Severity,
    device: String,
    message: String,
}

pub fn run(cfg: &UnifiConfig) -> Result<()> {
    let client = UnifiClient::new(cfg)?;

    println!("UniFi doctor — site '{}'", cfg.site);

    // Prefer the rich private stats; fall back to the integration API.
    let findings = match client.stat_device() {
        Ok(v) => {
            let devices = v
                .get("data")
                .and_then(|d| d.as_array())
                .cloned()
                .unwrap_or_default();
            println!("  source: private stats API ({} devices)\n", devices.len());
            analyze(&devices)
        }
        Err(e) => {
            println!("  note: deep stats unavailable ({e})");
            println!("  falling back to integration API (basic checks only)\n");
            let site_id = client.resolve_site_id()?;
            let v = client.list_devices(&site_id)?;
            let devices = v
                .get("data")
                .and_then(|d| d.as_array())
                .cloned()
                .unwrap_or_default();
            analyze_basic(&devices)
        }
    };

    if findings.is_empty() {
        println!("No adoption/STP/firmware issues detected.");
    } else {
        for f in &findings {
            println!("  [{}] {:<20} {}", f.severity.tag(), f.device, f.message);
        }
        let crit = findings
            .iter()
            .filter(|f| f.severity == Severity::Crit)
            .count();
        let warn = findings
            .iter()
            .filter(|f| f.severity == Severity::Warn)
            .count();
        println!(
            "\n{crit} critical, {warn} warnings, {} total.",
            findings.len()
        );
    }

    print_fortigate_hints();
    Ok(())
}

/// Rich analysis over the private `/stat/device` shape (has `port_table`).
fn analyze(devices: &[Value]) -> Vec<Finding> {
    let mut out = Vec::new();

    // Firmware-skew heuristic: flag devices not on the majority version, which
    // usually means a stuck upgrade or an interrupted migration.
    let majority = majority_version(devices);

    for dev in devices {
        let name = device_name(dev);

        // Adoption / connection state.
        if let Some(msg) = adoption_issue(dev) {
            out.push(Finding {
                severity: Severity::Crit,
                device: name.clone(),
                message: msg,
            });
        }

        // Firmware skew.
        if let (Some(maj), Some(ver)) = (&majority, dev.get("version").and_then(|v| v.as_str()))
            && ver != maj
        {
            out.push(Finding {
                severity: Severity::Warn,
                device: name.clone(),
                message: format!(
                    "firmware {ver} differs from fleet majority {maj} (stuck upgrade/migration?)"
                ),
            });
        }

        // Per-port STP + error checks.
        if let Some(ports) = dev.get("port_table").and_then(|p| p.as_array()) {
            for p in ports {
                out.extend(port_findings(&name, p));
            }
        }
    }

    out
}

/// Basic analysis over the integration API shape (no `port_table`).
fn analyze_basic(devices: &[Value]) -> Vec<Finding> {
    let mut out = Vec::new();
    for dev in devices {
        let name = device_name(dev);
        let state = dev.get("state").and_then(|s| s.as_str()).unwrap_or("");
        if !state.eq_ignore_ascii_case("online") && !state.is_empty() {
            let sev = if state.to_lowercase().contains("pending")
                || state.to_lowercase().contains("adopt")
            {
                Severity::Crit
            } else {
                Severity::Warn
            };
            out.push(Finding {
                severity: sev,
                device: name,
                message: format!("device state: {state}"),
            });
        }
    }
    out
}

fn device_name(dev: &Value) -> String {
    dev.get("name")
        .and_then(|n| n.as_str())
        .filter(|n| !n.is_empty())
        .or_else(|| dev.get("mac").and_then(|m| m.as_str()))
        .or_else(|| dev.get("macAddress").and_then(|m| m.as_str()))
        .unwrap_or("<unknown>")
        .to_string()
}

/// Interpret the private-API integer `state` (and `adopted` flag). Returns a
/// message when the device is not cleanly connected.
fn adoption_issue(dev: &Value) -> Option<String> {
    let adopted = dev.get("adopted").and_then(|a| a.as_bool());
    let state = dev.get("state").and_then(|s| s.as_i64());

    match state {
        Some(1) => None, // connected
        Some(0) => Some("disconnected (state=0) — check power/uplink/inform reachability".into()),
        Some(2) => Some("pending adoption (state=2) — run `ghostctl unifi adopt`".into()),
        Some(4) | Some(5) | Some(9) => Some(format!(
            "stuck in adopting/provisioning (state={})",
            state.unwrap()
        )),
        Some(6) => Some("heartbeat missed (state=6) — device unreachable by controller".into()),
        Some(7) => {
            Some("adoption failed (state=7) — likely inform URL/L3 or firmware issue".into())
        }
        Some(other) => Some(format!("unexpected state={other}")),
        None => {
            if adopted == Some(false) {
                Some("not adopted".into())
            } else {
                None
            }
        }
    }
}

/// STP-blocked ports and error counters from a `port_table` entry.
fn port_findings(device: &str, port: &Value) -> Vec<Finding> {
    let mut out = Vec::new();
    let idx = port
        .get("port_idx")
        .and_then(|i| i.as_i64())
        .map(|i| i.to_string())
        .unwrap_or_else(|| "?".into());
    let up = port.get("up").and_then(|u| u.as_bool()).unwrap_or(false);
    let is_uplink = port
        .get("is_uplink")
        .and_then(|u| u.as_bool())
        .unwrap_or(false);

    // STP: forwarding/disabled are fine; blocking/discarding on a live port is a loop symptom.
    if let Some(stp) = port.get("stp_state").and_then(|s| s.as_str()) {
        let s = stp.to_lowercase();
        if (s == "blocking" || s == "discarding") && up {
            out.push(Finding {
                severity: Severity::Crit,
                device: device.to_string(),
                message: format!("port {idx} STP {stp} (loop/redundant path — classic Fortinet<->UniFi STP mismatch)"),
            });
        }
    }

    // A down uplink is worth surfacing.
    if is_uplink && !up {
        out.push(Finding {
            severity: Severity::Warn,
            device: device.to_string(),
            message: format!("uplink port {idx} is down"),
        });
    }

    // Interface error counters.
    let rx = port.get("rx_errors").and_then(|v| v.as_u64()).unwrap_or(0);
    let tx = port.get("tx_errors").and_then(|v| v.as_u64()).unwrap_or(0);
    if rx + tx > 1000 {
        out.push(Finding {
            severity: Severity::Warn,
            device: device.to_string(),
            message: format!(
                "port {idx} error counters high (rx={rx} tx={tx}) — cabling/duplex/SFP"
            ),
        });
    }

    out
}

/// Most common non-empty `version` across devices, if any.
fn majority_version(devices: &[Value]) -> Option<String> {
    use std::collections::HashMap;
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for d in devices {
        if let Some(v) = d
            .get("version")
            .and_then(|v| v.as_str())
            .filter(|v| !v.is_empty())
        {
            *counts.entry(v).or_insert(0) += 1;
        }
    }
    counts
        .into_iter()
        .max_by_key(|(_, c)| *c)
        .map(|(v, _)| v.to_string())
}

fn print_fortigate_hints() {
    println!(
        "\nFortiGate <-> UniFi checklist (common in mixed shops):\n\
         - Edge ports to APs/switches: enable STP edge / 'edge-port' + BPDU guard on the\n\
           FortiSwitch/FortiGate side so a UniFi uplink doesn't get root-blocked.\n\
         - STP mode must match (RSTP vs MSTP) end-to-end; MSTP region mismatch = blocked ports.\n\
         - If adoption fails across an L3 boundary, set DHCP Option 43 or a `unifi` DNS record,\n\
           or use `ghostctl unifi adopt` (set-inform).\n\
         - Storm-control / loop-guard on FortiSwitch can drop UniFi inform/discovery traffic.\n\
         - Native/untagged VLAN must match on the trunk or the device never reaches the controller."
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_adoption_issue_states() {
        assert!(adoption_issue(&json!({"state": 1})).is_none());
        assert!(
            adoption_issue(&json!({"state": 2}))
                .unwrap()
                .contains("pending adoption")
        );
        assert!(
            adoption_issue(&json!({"state": 7}))
                .unwrap()
                .contains("adoption failed")
        );
        assert!(
            adoption_issue(&json!({"adopted": false}))
                .unwrap()
                .contains("not adopted")
        );
    }

    #[test]
    fn test_port_findings_flags_blocking_stp() {
        let port = json!({"port_idx": 5, "up": true, "stp_state": "blocking"});
        let f = port_findings("sw01", &port);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].severity, Severity::Crit);
        assert!(f[0].message.contains("STP blocking"));
    }

    #[test]
    fn test_port_findings_ignores_forwarding() {
        let port = json!({"port_idx": 1, "up": true, "stp_state": "forwarding", "rx_errors": 0, "tx_errors": 0});
        assert!(port_findings("sw01", &port).is_empty());
    }

    #[test]
    fn test_port_findings_high_errors() {
        let port = json!({"port_idx": 2, "up": true, "stp_state": "forwarding", "rx_errors": 900, "tx_errors": 900});
        let f = port_findings("sw01", &port);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].severity, Severity::Warn);
    }

    #[test]
    fn test_majority_version() {
        let devs = vec![
            json!({"version": "6.6.55"}),
            json!({"version": "6.6.55"}),
            json!({"version": "6.5.28"}),
        ];
        assert_eq!(majority_version(&devs).as_deref(), Some("6.6.55"));
    }

    #[test]
    fn test_analyze_flags_firmware_skew() {
        let devs = vec![
            json!({"name": "ap1", "state": 1, "version": "6.6.55"}),
            json!({"name": "ap2", "state": 1, "version": "6.6.55"}),
            json!({"name": "ap3", "state": 1, "version": "6.5.28"}),
        ];
        let findings = analyze(&devs);
        assert!(
            findings
                .iter()
                .any(|f| f.device == "ap3" && f.message.contains("differs from fleet majority"))
        );
    }
}
