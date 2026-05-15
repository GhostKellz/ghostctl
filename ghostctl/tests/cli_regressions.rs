use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

fn ghostctl_command() -> Command {
    Command::cargo_bin("ghostctl").expect("failed to locate ghostctl test binary")
}

fn temp_output_path(suffix: &str) -> std::path::PathBuf {
    let file = tempfile::Builder::new()
        .prefix("ghostctl-regression-")
        .suffix(suffix)
        .tempfile()
        .expect("failed to create temp output path");
    let path = file.path().to_path_buf();
    drop(file);
    let _ = std::fs::remove_file(&path);
    path
}

#[test]
fn completion_generation_works_for_supported_shells() {
    for shell in ["bash", "zsh", "fish"] {
        let output = ghostctl_command()
            .args(["completion", shell])
            .output()
            .unwrap_or_else(|_| panic!("failed to generate {shell} completions"));

        assert!(output.status.success(), "completion failed for {shell}");
        let stdout = String::from_utf8(output.stdout).expect("completion output was not utf-8");
        assert!(stdout.contains("ghostctl"));
        assert!(stdout.contains("support"));
    }
}

#[test]
fn support_paths_reports_xdg_state_layout() {
    let output = ghostctl_command()
        .args(["support", "paths"])
        .output()
        .expect("failed to run support paths");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("support paths output was not utf-8");
    assert!(stdout.contains("State:"));
    assert!(stdout.contains("Support:"));
    assert!(stdout.contains("Logs:"));
    assert!(stdout.contains("History:"));
}

#[test]
fn support_doctor_reports_readiness_summary() {
    let output = ghostctl_command()
        .args(["support", "doctor"])
        .output()
        .expect("failed to run support doctor");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("support doctor output was not utf-8");
    assert!(stdout.contains("GhostCTL support doctor"));
    assert!(stdout.contains("state writable:"));
    assert!(stdout.contains("bundle command:"));
}

#[test]
fn support_bundle_writes_metadata_and_redacts_common_identifiers() {
    let output_path = temp_output_path(".txt");
    let metadata_path = output_path.with_extension("txt.json");
    let output = ghostctl_command()
        .args([
            "support",
            "bundle",
            "--output",
            output_path.to_str().unwrap(),
            "--redact-paths",
            "--log-tail",
            "5",
        ])
        .output()
        .expect("failed to run support bundle");

    assert!(output.status.success());
    assert!(output_path.exists());
    assert!(metadata_path.exists());

    let bundle = std::fs::read_to_string(&output_path).expect("failed to read support bundle");
    let metadata = std::fs::read_to_string(&metadata_path).expect("failed to read metadata");

    assert!(bundle.contains("ghostctl support bundle"));
    assert!(bundle.contains("[system]"));
    assert!(bundle.contains("[runtime]"));
    assert!(bundle.contains("[tools]"));
    assert!(metadata.contains("ghostctl.support_bundle.v1"));
    assert!(metadata.contains("\"redacted_paths\": true"));

    let _ = std::fs::remove_file(output_path);
    let _ = std::fs::remove_file(metadata_path);
}

#[test]
fn support_bundle_gzip_writes_compressed_bundle_and_sidecar() {
    let output_path = temp_output_path(".txt.gz");
    let metadata_path = output_path.with_extension("gz.json");
    let output = ghostctl_command()
        .args([
            "support",
            "bundle",
            "--output",
            output_path.to_str().unwrap(),
            "--gzip",
            "--redact-paths",
        ])
        .output()
        .expect("failed to run gzip support bundle");

    assert!(output.status.success());
    assert!(output_path.exists());
    assert!(metadata_path.exists());

    let metadata = std::fs::read_to_string(&metadata_path).expect("failed to read metadata");
    assert!(metadata.contains("\"format\": \"gzip\""));

    let _ = std::fs::remove_file(output_path);
    let _ = std::fs::remove_file(metadata_path);
}

#[test]
fn support_bundle_tarball_embeds_report_and_metadata() {
    let output_path = temp_output_path(".tar.gz");
    let output = ghostctl_command()
        .args([
            "support",
            "bundle",
            "--output",
            output_path.to_str().unwrap(),
            "--tarball",
            "--redact-paths",
        ])
        .output()
        .expect("failed to run tarball support bundle");

    assert!(output.status.success());
    assert!(output_path.exists());

    let file = std::fs::File::open(&output_path).expect("failed to open tarball");
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    let names = archive
        .entries()
        .expect("failed to read tarball entries")
        .map(|entry| {
            entry
                .expect("failed to read tarball entry")
                .path()
                .expect("failed to read tarball entry path")
                .display()
                .to_string()
        })
        .collect::<Vec<_>>();

    assert!(names.contains(&"ghostctl-support.txt".to_string()));
    assert!(names.contains(&"ghostctl-support.json".to_string()));

    let _ = std::fs::remove_file(output_path);
}
