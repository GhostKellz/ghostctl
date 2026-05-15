use anyhow::{Context, Result};
use chrono::Utc;
use flate2::Compression;
use flate2::write::GzEncoder;
use regex::Regex;
use serde_json::json;
use std::fmt::Write as _;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn state_dir() -> PathBuf {
    if let Some(dir) = dirs::state_dir() {
        dir.join("ghostctl")
    } else if let Some(dir) = dirs::data_local_dir() {
        dir.join("ghostctl").join("state")
    } else {
        std::env::temp_dir().join("ghostctl-state")
    }
}

pub fn support_dir() -> PathBuf {
    state_dir().join("support")
}

pub fn log_dir() -> PathBuf {
    state_dir().join("logs")
}

pub fn history_log_path() -> PathBuf {
    log_dir().join("history.log")
}

pub fn legacy_history_log_path() -> Option<PathBuf> {
    dirs::data_dir().map(|dir| dir.join("ghostctl").join("history.log"))
}

pub fn default_bundle_path(file_name: &str) -> PathBuf {
    let dir = support_dir();
    let _ = std::fs::create_dir_all(&dir);
    dir.join(file_name)
}

pub fn timestamped_bundle_path() -> PathBuf {
    timestamped_bundle_path_for(BundleFormat::Text)
}

pub fn timestamped_bundle_path_for(format: BundleFormat) -> PathBuf {
    let suffix = match format {
        BundleFormat::Text => "txt",
        BundleFormat::Gzip => "txt.gz",
        BundleFormat::Tarball => "tar.gz",
    };

    default_bundle_path(&format!(
        "ghostctl-support-{}.{}",
        Utc::now().format("%Y%m%d-%H%M%S"),
        suffix
    ))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BundleFormat {
    Text,
    Gzip,
    Tarball,
}

pub fn print_paths() {
    println!("State: {}", state_dir().display());
    println!("Support: {}", support_dir().display());
    println!("Logs: {}", log_dir().display());
    println!("History: {}", history_log_path().display());
    if let Some(path) = legacy_history_log_path().filter(|path| path.exists()) {
        println!("Legacy history: {}", path.display());
    }
}

pub fn print_doctor() {
    println!("GhostCTL support doctor");
    println!("=======================");
    println!("version: {}", env!("CARGO_PKG_VERSION"));
    println!("state: {}", state_dir().display());
    println!("support: {}", support_dir().display());
    println!("logs: {}", log_dir().display());

    let state_ok = ensure_dir(&state_dir());
    let support_ok = ensure_dir(&support_dir());
    let logs_ok = ensure_dir(&log_dir());
    println!("state writable: {}", status_label(state_ok));
    println!("support writable: {}", status_label(support_ok));
    println!("logs writable: {}", status_label(logs_ok));

    let present_tools = support_tools()
        .iter()
        .filter(|tool| which::which(tool).is_ok())
        .count();
    println!(
        "external tools visible: {present_tools}/{}",
        support_tools().len()
    );
    println!("bundle command: ghostctl support bundle --redact-paths");

    // Domain-specific diagnostics (only shown when relevant tools detected)
    doctor_docker();
    doctor_proxmox();
    doctor_vfio();
    doctor_nvidia();
    doctor_storage();
    doctor_networking();
    doctor_uefi();
}

fn doctor_docker() {
    let has_docker = which::which("docker").is_ok();
    let has_podman = which::which("podman").is_ok();
    if !has_docker && !has_podman {
        return;
    }

    println!();
    println!("[docker]");

    if has_docker {
        let daemon_ok = Command::new("docker")
            .args(["info"])
            .output()
            .is_ok_and(|o| o.status.success());
        println!(
            "docker daemon: {}",
            if daemon_ok {
                "running"
            } else {
                "not reachable"
            }
        );

        let compose_ok = Command::new("docker")
            .args(["compose", "version"])
            .output()
            .is_ok_and(|o| o.status.success());
        println!(
            "docker compose: {}",
            if compose_ok { "available" } else { "not found" }
        );
    }

    if has_podman {
        let podman_ok = Command::new("podman")
            .args(["info"])
            .output()
            .is_ok_and(|o| o.status.success());
        println!(
            "podman: {}",
            if podman_ok {
                "running"
            } else {
                "not reachable"
            }
        );
    }
}

fn doctor_proxmox() {
    let has_qm = which::which("qm").is_ok();
    let has_pvesh = which::which("pvesh").is_ok();
    let has_pct = which::which("pct").is_ok();
    if !has_qm && !has_pvesh && !has_pct {
        return;
    }

    println!();
    println!("[proxmox]");
    if has_pvesh {
        println!("pvesh: available");
    }
    if has_qm {
        println!("qm: available");
    }
    if has_pct {
        println!("pct: available");
    }
}

fn doctor_vfio() {
    let iommu_present = std::path::Path::new("/sys/class/iommu")
        .read_dir()
        .is_ok_and(|mut d| d.next().is_some());
    if !iommu_present {
        return;
    }

    println!();
    println!("[vfio]");
    println!("iommu: enabled");

    let vfio_loaded = std::path::Path::new("/sys/bus/pci/drivers/vfio-pci").exists();
    println!(
        "vfio-pci: {}",
        if vfio_loaded { "loaded" } else { "not loaded" }
    );

    if let Ok(contents) = std::fs::read_to_string("/proc/cmdline") {
        let has_iommu_param = contents.contains("iommu=on")
            || contents.contains("intel_iommu=on")
            || contents.contains("amd_iommu=on");
        println!(
            "kernel cmdline iommu: {}",
            if has_iommu_param {
                "present"
            } else {
                "not found"
            }
        );
    }
}

fn doctor_nvidia() {
    if which::which("nvidia-smi").is_err() {
        return;
    }

    println!();
    println!("[nvidia]");

    if let Ok(output) = Command::new("nvidia-smi")
        .args(["--query-gpu=driver_version,name", "--format=csv,noheader"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let trimmed = stdout.trim();
        if !trimmed.is_empty() {
            for line in trimmed.lines() {
                let parts: Vec<&str> = line.splitn(2, ", ").collect();
                if parts.len() == 2 {
                    println!("driver: {}", parts[0].trim());
                    println!("gpu: {}", parts[1].trim());
                } else {
                    println!("gpu: {trimmed}");
                }
            }
        }
    }
}

fn doctor_storage() {
    let has_btrfs = which::which("btrfs").is_ok();
    let has_restic = which::which("restic").is_ok();
    let has_snapper = which::which("snapper").is_ok();
    if !has_btrfs && !has_restic && !has_snapper {
        return;
    }

    println!();
    println!("[storage]");
    if has_btrfs {
        println!("btrfs: available");
    }
    if has_snapper {
        println!("snapper: available");
    }
    if has_restic {
        println!("restic: available");
    }
}

fn doctor_networking() {
    let has_nft = which::which("nft").is_ok();
    let has_ufw = which::which("ufw").is_ok();
    if !has_nft && !has_ufw {
        return;
    }

    println!();
    println!("[networking]");
    if has_nft {
        println!("nftables: available");
    }
    if has_ufw {
        println!("ufw: available");
    }
}

fn doctor_uefi() {
    if which::which("virt-fw-vars").is_err() {
        return;
    }

    println!();
    println!("[uefi]");
    println!("virt-fw-vars: available");

    let ovmf_paths = [
        "/usr/share/edk2/x64/OVMF_CODE.secboot.4m.fd",
        "/usr/share/edk2-ovmf/x64/OVMF_CODE.secboot.4m.fd",
        "/usr/share/OVMF/OVMF_CODE.secboot.fd",
    ];
    let ovmf_found = ovmf_paths.iter().any(|p| std::path::Path::new(p).exists());
    println!(
        "ovmf firmware: {}",
        if ovmf_found { "found" } else { "not found" }
    );
}

pub fn write_bundle(
    output_path: &Path,
    redact_paths: bool,
    log_tail: usize,
    format: BundleFormat,
) -> Result<Option<PathBuf>> {
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }

    let mut report = String::new();
    writeln!(report, "ghostctl support bundle")?;
    writeln!(report, "=======================")?;
    writeln!(report, "created={}", Utc::now().to_rfc3339())?;
    writeln!(report, "version={}", env!("CARGO_PKG_VERSION"))?;
    writeln!(report, "state_dir={}", state_dir().display())?;
    writeln!(report)?;

    writeln!(report, "[system]")?;
    write_command_output(&mut report, "uname", &["-a"])?;
    write_command_output(&mut report, "hostnamectl", &[])?;
    write_command_output(&mut report, "systemctl", &["--version"])?;
    write_os_release(&mut report)?;
    writeln!(report)?;

    writeln!(report, "[runtime]")?;
    writeln!(report, "uid={}", current_uid())?;
    writeln!(report, "euid={}", current_euid())?;
    writeln!(report, "current_dir={}", std::env::current_dir()?.display())?;
    writeln!(report)?;

    writeln!(report, "[tools]")?;
    for tool in support_tools() {
        let status = if which::which(tool).is_ok() {
            "present"
        } else {
            "missing"
        };
        writeln!(report, "{tool}={status}")?;
    }
    writeln!(report)?;

    write_tool_versions(&mut report)?;

    writeln!(report, "[recent ghostctl logs]")?;
    for line in recent_log_lines(log_tail) {
        writeln!(report, "{line}")?;
    }

    let report = if redact_paths {
        redact_text(&report, true)
    } else {
        report
    };

    let metadata = json!({
        "schema": "ghostctl.support_bundle.v1",
        "created": Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
        "bundle": redact_text(&output_path.display().to_string(), redact_paths),
        "redacted_paths": redact_paths,
        "log_tail": log_tail,
        "format": format_label(format),
    });

    match format {
        BundleFormat::Text => {
            std::fs::write(output_path, report)
                .with_context(|| format!("writing {}", output_path.display()))?;
            let metadata_path = metadata_path_for(output_path);
            std::fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?)
                .with_context(|| format!("writing {}", metadata_path.display()))?;
            Ok(Some(metadata_path))
        }
        BundleFormat::Gzip => {
            let file = std::fs::File::create(output_path)
                .with_context(|| format!("creating {}", output_path.display()))?;
            let mut encoder = GzEncoder::new(file, Compression::default());
            encoder.write_all(report.as_bytes())?;
            encoder.finish()?;

            let metadata_path = metadata_path_for(output_path);
            std::fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?)
                .with_context(|| format!("writing {}", metadata_path.display()))?;
            Ok(Some(metadata_path))
        }
        BundleFormat::Tarball => {
            let file = std::fs::File::create(output_path)
                .with_context(|| format!("creating {}", output_path.display()))?;
            let encoder = GzEncoder::new(file, Compression::default());
            let mut archive = tar::Builder::new(encoder);
            append_bytes(&mut archive, "ghostctl-support.txt", report.as_bytes())?;
            append_bytes(
                &mut archive,
                "ghostctl-support.json",
                serde_json::to_string_pretty(&metadata)?.as_bytes(),
            )?;
            archive.finish()?;
            Ok(None)
        }
    }
}

fn metadata_path_for(output_path: &Path) -> PathBuf {
    output_path.with_extension(format!(
        "{}json",
        output_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| format!("{ext}."))
            .unwrap_or_default()
    ))
}

fn format_label(format: BundleFormat) -> &'static str {
    match format {
        BundleFormat::Text => "text",
        BundleFormat::Gzip => "gzip",
        BundleFormat::Tarball => "tarball",
    }
}

fn append_bytes<W: Write>(archive: &mut tar::Builder<W>, path: &str, bytes: &[u8]) -> Result<()> {
    let mut header = tar::Header::new_gnu();
    header.set_path(path)?;
    header.set_size(bytes.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    archive.append(&header, bytes)?;
    Ok(())
}

fn support_tools() -> &'static [&'static str] {
    &[
        "bash",
        "zsh",
        "fish",
        "docker",
        "podman",
        "nft",
        "ufw",
        "virsh",
        "qm",
        "pct",
        "pvesh",
        "restic",
        "btrfs",
        "snapper",
        "virt-fw-vars",
    ]
}

fn ensure_dir(path: &Path) -> bool {
    std::fs::create_dir_all(path).is_ok()
        && tempfile::Builder::new()
            .prefix("write-check-")
            .tempfile_in(path)
            .is_ok()
}

fn status_label(ok: bool) -> &'static str {
    if ok { "ok" } else { "failed" }
}

fn write_os_release(report: &mut String) -> Result<()> {
    match std::fs::read_to_string("/etc/os-release") {
        Ok(content) => {
            writeln!(report, "/etc/os-release:")?;
            for line in content.lines() {
                writeln!(report, "  {line}")?;
            }
        }
        Err(e) => writeln!(report, "/etc/os-release=unavailable ({e})")?,
    }
    Ok(())
}

fn current_uid() -> u32 {
    // SAFETY: getuid has no preconditions and does not dereference pointers.
    unsafe { libc::getuid() }
}

fn current_euid() -> u32 {
    // SAFETY: geteuid has no preconditions and does not dereference pointers.
    unsafe { libc::geteuid() }
}

fn write_tool_versions(report: &mut String) -> Result<()> {
    writeln!(report, "[tool versions]")?;

    for (tool, args) in [
        ("ghostctl", vec!["version"]),
        ("docker", vec!["--version"]),
        ("podman", vec!["--version"]),
        ("nft", vec!["--version"]),
        ("ufw", vec!["--version"]),
        ("virsh", vec!["--version"]),
        ("qm", vec!["version"]),
        ("pct", vec!["version"]),
        ("pvesh", vec!["version"]),
        ("restic", vec!["version"]),
        ("btrfs", vec!["--version"]),
        ("snapper", vec!["--version"]),
        ("virt-fw-vars", vec!["--version"]),
        ("nvidia-smi", vec!["--version"]),
    ] {
        if which::which(tool).is_ok() {
            write_command_output(report, tool, &args)?;
        }
    }

    writeln!(report)?;
    Ok(())
}

fn write_command_output(report: &mut String, command: &str, args: &[&str]) -> Result<()> {
    let output = Command::new(command).args(args).output();
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            writeln!(
                report,
                "{} {}: status={}",
                command,
                args.join(" "),
                output.status
            )?;
            if !stdout.trim().is_empty() {
                writeln!(report, "{}", stdout.trim())?;
            }
            if !stderr.trim().is_empty() {
                writeln!(report, "stderr: {}", stderr.trim())?;
            }
        }
        Err(e) => writeln!(report, "{} {}: unavailable ({e})", command, args.join(" "))?,
    }
    Ok(())
}

fn recent_log_lines(limit: usize) -> Vec<String> {
    let path = if history_log_path().exists() {
        history_log_path()
    } else if let Some(path) = legacy_history_log_path().filter(|path| path.exists()) {
        path
    } else {
        return vec!["no history log found".to_string()];
    };

    match std::fs::read_to_string(path) {
        Ok(content) => content
            .lines()
            .rev()
            .take(limit)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect(),
        Err(e) => vec![format!("could not read history log: {e}")],
    }
}

fn redact_text(text: &str, enabled: bool) -> String {
    if !enabled {
        return text.to_string();
    }

    let mut redacted = text.to_string();

    if let Some(home) = dirs::home_dir().and_then(|path| path.into_os_string().into_string().ok()) {
        redacted = redacted.replace(&home, "~");
    }

    if let Ok(user) = std::env::var("USER")
        && !user.is_empty()
    {
        redacted = redacted.replace(&user, "<user>");
    }

    if let Ok(hostname) = gethostname::gethostname().into_string()
        && !hostname.is_empty()
    {
        redacted = redacted.replace(&hostname, "<host>");
    }

    for (pattern, replacement) in [
        (
            r"\b(?:25[0-5]|2[0-4]\d|1?\d?\d)(?:\.(?:25[0-5]|2[0-4]\d|1?\d?\d)){3}\b",
            "<ipv4>",
        ),
        (r"\b[0-9a-fA-F]{2}(?::[0-9a-fA-F]{2}){5}\b", "<mac>"),
        (
            r"\b[0-9a-fA-F]{4}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}\.[0-7]\b",
            "<pci-id>",
        ),
        (r"(?i)\b(serial|uuid|machine-id)=\S+", "$1=<redacted>"),
    ] {
        if let Ok(regex) = Regex::new(pattern) {
            redacted = regex.replace_all(&redacted, replacement).to_string();
        }
    }

    redacted
}

#[cfg(test)]
mod tests {
    use super::redact_text;

    #[test]
    fn redaction_covers_common_identifiers() {
        let input = "host 192.168.1.10 mac aa:bb:cc:dd:ee:ff pci 0000:0a:00.0 serial=abc123";
        let redacted = redact_text(input, true);

        assert!(!redacted.contains("192.168.1.10"));
        assert!(!redacted.contains("aa:bb:cc:dd:ee:ff"));
        assert!(!redacted.contains("0000:0a:00.0"));
        assert!(!redacted.contains("serial=abc123"));
        assert!(redacted.contains("<ipv4>"));
        assert!(redacted.contains("<mac>"));
        assert!(redacted.contains("<pci-id>"));
    }

    #[test]
    fn redaction_can_be_disabled() {
        let input = "192.168.1.10";
        assert_eq!(redact_text(input, false), input);
    }
}
