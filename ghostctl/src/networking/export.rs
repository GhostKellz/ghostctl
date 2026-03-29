//! Export functionality for scan results
//!
//! Supports JSON, CSV, and Nmap XML-compatible formats

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use super::scanner::{PortStatus, ScanResult};
use super::services;

/// Export format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExportFormat {
    #[default]
    Json,
    JsonPretty,
    Csv,
    NmapXml,
    Markdown,
}

impl std::str::FromStr for ExportFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(ExportFormat::Json),
            "json-pretty" | "jsonpretty" => Ok(ExportFormat::JsonPretty),
            "csv" => Ok(ExportFormat::Csv),
            "xml" | "nmap" | "nmap-xml" => Ok(ExportFormat::NmapXml),
            "md" | "markdown" => Ok(ExportFormat::Markdown),
            _ => Err(format!("Unknown format: {}", s)),
        }
    }
}

/// Serializable scan result for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportScanResult {
    pub target: String,
    pub port: u16,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    pub response_time_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<bool>,
}

impl From<&ScanResult> for ExportScanResult {
    fn from(r: &ScanResult) -> Self {
        let version = r
            .banner
            .as_ref()
            .and_then(|b| services::extract_version(r.port, b));

        ExportScanResult {
            target: r.target.clone(),
            port: r.port,
            status: format!("{:?}", r.status),
            service: r
                .service
                .clone()
                .or_else(|| services::get_service_name(r.port).map(|s| s.to_string())),
            version,
            banner: r.banner.clone(),
            response_time_ms: r.response_time.as_millis() as u64,
            tls: if services::is_tls_port(r.port) {
                Some(true)
            } else {
                None
            },
        }
    }
}

/// Full scan report for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanReport {
    pub scan_info: ScanInfo,
    pub hosts: Vec<HostReport>,
    pub summary: ScanSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanInfo {
    pub start_time: String,
    pub end_time: String,
    pub duration_secs: f64,
    pub scanner: String,
    pub version: String,
    pub args: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostReport {
    pub address: String,
    pub hostname: Option<String>,
    pub status: String,
    pub ports: Vec<ExportScanResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanSummary {
    pub total_hosts: usize,
    pub hosts_up: usize,
    pub total_ports_scanned: usize,
    pub open_ports: usize,
    pub closed_ports: usize,
    pub filtered_ports: usize,
}

/// Export scan results to a file
pub fn export_results(
    results: &[ScanResult],
    path: &Path,
    format: ExportFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    match format {
        ExportFormat::Json => export_json(results, &mut writer, false)?,
        ExportFormat::JsonPretty => export_json(results, &mut writer, true)?,
        ExportFormat::Csv => export_csv(results, &mut writer)?,
        ExportFormat::NmapXml => export_nmap_xml(results, &mut writer)?,
        ExportFormat::Markdown => export_markdown(results, &mut writer)?,
    }

    writer.flush()?;
    Ok(())
}

/// Export results as JSON
fn export_json<W: Write>(
    results: &[ScanResult],
    writer: &mut W,
    pretty: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let report = build_report(results);

    if pretty {
        serde_json::to_writer_pretty(writer, &report)?;
    } else {
        serde_json::to_writer(writer, &report)?;
    }

    Ok(())
}

/// Export results as CSV
fn export_csv<W: Write>(
    results: &[ScanResult],
    writer: &mut W,
) -> Result<(), Box<dyn std::error::Error>> {
    // Header
    writeln!(
        writer,
        "target,port,status,service,version,banner,response_time_ms,tls"
    )?;

    for result in results {
        let export = ExportScanResult::from(result);
        writeln!(
            writer,
            "{},{},{},{},{},{},{},{}",
            escape_csv(&export.target),
            export.port,
            escape_csv(&export.status),
            escape_csv(&export.service.unwrap_or_default()),
            escape_csv(&export.version.unwrap_or_default()),
            escape_csv(&export.banner.unwrap_or_default()),
            export.response_time_ms,
            export.tls.map(|t| t.to_string()).unwrap_or_default(),
        )?;
    }

    Ok(())
}

/// Export results as Nmap-compatible XML
fn export_nmap_xml<W: Write>(
    results: &[ScanResult],
    writer: &mut W,
) -> Result<(), Box<dyn std::error::Error>> {
    let report = build_report(results);

    writeln!(writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
    writeln!(writer, r#"<!DOCTYPE nmaprun>"#)?;
    writeln!(
        writer,
        r#"<nmaprun scanner="{}" args="" start="{}" version="{}">"#,
        xml_escape(&report.scan_info.scanner),
        xml_escape(&report.scan_info.start_time),
        xml_escape(&report.scan_info.version)
    )?;

    // Scan info
    writeln!(writer, r#"<scaninfo type="connect" protocol="tcp" />"#)?;

    // Hosts
    for host in &report.hosts {
        writeln!(
            writer,
            r#"<host><status state="{}" />"#,
            xml_escape(&host.status)
        )?;
        writeln!(
            writer,
            r#"<address addr="{}" addrtype="ipv4" />"#,
            xml_escape(&host.address)
        )?;

        if let Some(ref hostname) = host.hostname {
            writeln!(
                writer,
                r#"<hostnames><hostname name="{}" /></hostnames>"#,
                xml_escape(hostname)
            )?;
        }

        writeln!(writer, "<ports>")?;
        for port in &host.ports {
            let state = match port.status.as_str() {
                "Open" => "open",
                "Closed" => "closed",
                _ => "filtered",
            };
            write!(writer, r#"<port protocol="tcp" portid="{}">"#, port.port)?;
            write!(writer, r#"<state state="{}" />"#, state)?;

            if let Some(ref service) = port.service {
                write!(writer, r#"<service name="{}""#, xml_escape(service))?;
                if let Some(ref version) = port.version {
                    write!(writer, r#" version="{}""#, xml_escape(version))?;
                }
                write!(writer, " />")?;
            }

            writeln!(writer, "</port>")?;
        }
        writeln!(writer, "</ports>")?;
        writeln!(writer, "</host>")?;
    }

    // Run stats
    writeln!(
        writer,
        r#"<runstats><finished time="{}" elapsed="{}" />"#,
        xml_escape(&report.scan_info.end_time),
        report.scan_info.duration_secs
    )?;
    writeln!(
        writer,
        r#"<hosts up="{}" down="{}" total="{}" /></runstats>"#,
        report.summary.hosts_up,
        report.summary.total_hosts - report.summary.hosts_up,
        report.summary.total_hosts
    )?;

    writeln!(writer, "</nmaprun>")?;

    Ok(())
}

/// Export results as Markdown
fn export_markdown<W: Write>(
    results: &[ScanResult],
    writer: &mut W,
) -> Result<(), Box<dyn std::error::Error>> {
    let report = build_report(results);

    writeln!(writer, "# Scan Report")?;
    writeln!(writer)?;
    writeln!(writer, "## Summary")?;
    writeln!(writer)?;
    writeln!(writer, "| Metric | Value |")?;
    writeln!(writer, "|--------|-------|")?;
    writeln!(writer, "| Total Hosts | {} |", report.summary.total_hosts)?;
    writeln!(writer, "| Hosts Up | {} |", report.summary.hosts_up)?;
    writeln!(writer, "| Open Ports | {} |", report.summary.open_ports)?;
    writeln!(writer, "| Closed Ports | {} |", report.summary.closed_ports)?;
    writeln!(
        writer,
        "| Filtered Ports | {} |",
        report.summary.filtered_ports
    )?;
    writeln!(
        writer,
        "| Duration | {:.2}s |",
        report.scan_info.duration_secs
    )?;
    writeln!(writer)?;

    for host in &report.hosts {
        writeln!(writer, "## Host: {}", host.address)?;
        writeln!(writer)?;

        if host.ports.is_empty() {
            writeln!(writer, "No open ports found.")?;
        } else {
            writeln!(writer, "| Port | Status | Service | Version | Response |")?;
            writeln!(writer, "|------|--------|---------|---------|----------|")?;

            for port in &host.ports {
                writeln!(
                    writer,
                    "| {} | {} | {} | {} | {}ms |",
                    port.port,
                    port.status,
                    port.service.as_deref().unwrap_or("-"),
                    port.version.as_deref().unwrap_or("-"),
                    port.response_time_ms
                )?;
            }
        }
        writeln!(writer)?;
    }

    Ok(())
}

/// Build a full report from scan results
fn build_report(results: &[ScanResult]) -> ScanReport {
    let now = chrono::Utc::now();

    // Group by host
    let mut hosts_map: HashMap<String, Vec<&ScanResult>> = HashMap::new();
    for result in results {
        hosts_map
            .entry(result.target.clone())
            .or_default()
            .push(result);
    }

    let hosts: Vec<HostReport> = hosts_map
        .into_iter()
        .map(|(addr, ports)| {
            let has_open = ports.iter().any(|p| matches!(p.status, PortStatus::Open));
            HostReport {
                address: addr,
                hostname: None,
                status: if has_open {
                    "up".to_string()
                } else {
                    "down".to_string()
                },
                ports: ports.iter().map(|p| ExportScanResult::from(*p)).collect(),
            }
        })
        .collect();

    let open_count = results
        .iter()
        .filter(|r| matches!(r.status, PortStatus::Open))
        .count();
    let closed_count = results
        .iter()
        .filter(|r| matches!(r.status, PortStatus::Closed))
        .count();
    let filtered_count = results
        .iter()
        .filter(|r| matches!(r.status, PortStatus::Filtered))
        .count();

    ScanReport {
        scan_info: ScanInfo {
            start_time: now.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            end_time: now.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            duration_secs: 0.0,
            scanner: "ghostctl".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            args: None,
        },
        summary: ScanSummary {
            total_hosts: hosts.len(),
            hosts_up: hosts.iter().filter(|h| h.status == "up").count(),
            total_ports_scanned: results.len(),
            open_ports: open_count,
            closed_ports: closed_count,
            filtered_ports: filtered_count,
        },
        hosts,
    }
}

/// Escape CSV special characters
fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

/// Escape XML special characters
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn mock_result() -> ScanResult {
        ScanResult {
            target: "192.168.1.1".to_string(),
            port: 22,
            status: PortStatus::Open,
            service: Some("ssh".to_string()),
            banner: Some("SSH-2.0-OpenSSH_8.9".to_string()),
            response_time: Duration::from_millis(5),
        }
    }

    #[test]
    fn test_export_csv() {
        let results = vec![mock_result()];
        let mut output = Vec::new();
        export_csv(&results, &mut output).unwrap();

        let csv = String::from_utf8(output).unwrap();
        assert!(csv.contains("192.168.1.1"));
        assert!(csv.contains("22"));
        assert!(csv.contains("Open"));
        assert!(csv.contains("ssh"));
    }

    #[test]
    fn test_export_json() {
        let results = vec![mock_result()];
        let mut output = Vec::new();
        export_json(&results, &mut output, true).unwrap();

        let json = String::from_utf8(output).unwrap();
        assert!(json.contains("192.168.1.1"));
        assert!(json.contains("\"port\": 22"));
    }

    #[test]
    fn test_format_parsing() {
        assert_eq!("json".parse::<ExportFormat>().unwrap(), ExportFormat::Json);
        assert_eq!("csv".parse::<ExportFormat>().unwrap(), ExportFormat::Csv);
        assert_eq!(
            "nmap-xml".parse::<ExportFormat>().unwrap(),
            ExportFormat::NmapXml
        );
    }
}
