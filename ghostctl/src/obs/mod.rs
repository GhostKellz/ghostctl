//! `ghostctl obs` - OBS Studio helper for Linux.
//!
//! Focuses on the parts of OBS that are painful on Linux: Wayland screen
//! capture (which is not native and depends on `xdg-desktop-portal` + PipeWire),
//! the v4l2loopback virtual camera, and NVENC hardware encoding. Commands both
//! diagnose the environment and actively assist by installing/enabling the
//! right pieces.

pub mod config;
pub mod detect;

use anyhow::{Result, bail};
use clap::{Arg, ArgAction, ArgMatches, Command};
use config::ObsConfig;
use detect::{Compositor, SessionType};
use std::process::Command as ProcCommand;

pub fn command() -> Command {
    Command::new("obs")
        .about("OBS Studio helper: Wayland screencapture, virtual camera, NVENC")
        .subcommand(
            Command::new("doctor")
                .about("Full OBS environment report (session, portal, PipeWire, vcam, NVENC)"),
        )
        .subcommand(
            Command::new("portal")
                .about("Wayland screencapture via xdg-desktop-portal + PipeWire")
                .subcommand(
                    Command::new("check").about("Check the portal backend and PipeWire status"),
                )
                .subcommand(
                    Command::new("setup")
                        .about("Install + enable the right portal backend and PipeWire stack"),
                ),
        )
        .subcommand(
            Command::new("vcam")
                .about("OBS virtual camera via the v4l2loopback kernel module")
                .subcommand(Command::new("status").about("Show v4l2loopback and video devices"))
                .subcommand(
                    Command::new("enable")
                        .about("Load v4l2loopback with OBS-friendly options")
                        .arg(
                            Arg::new("persist")
                                .long("persist")
                                .action(ArgAction::SetTrue)
                                .help("Make the virtual camera load on every boot"),
                        ),
                )
                .subcommand(Command::new("disable").about("Unload the v4l2loopback module")),
        )
        .subcommand(
            Command::new("nvenc")
                .about("NVIDIA hardware encoding (NVENC) checks")
                .subcommand(Command::new("check").about("Verify driver + ffmpeg NVENC support")),
        )
        .subcommand(
            Command::new("screencast")
                .about("Verify the Wayland ScreenCast portal is usable")
                .subcommand(Command::new("test").about("Probe the ScreenCast portal interface")),
        )
}

pub fn handle(matches: &ArgMatches) -> Result<()> {
    let cfg = ObsConfig::load();

    match matches.subcommand() {
        Some(("doctor", _)) => doctor(&cfg),
        Some(("portal", m)) => match m.subcommand() {
            Some(("check", _)) => portal_check(&cfg),
            Some(("setup", _)) => portal_setup(&cfg),
            _ => {
                println!("Use `ghostctl obs portal check|setup`.");
                Ok(())
            }
        },
        Some(("vcam", m)) => match m.subcommand() {
            Some(("status", _)) => vcam_status(),
            Some(("enable", em)) => vcam_enable(&cfg, em.get_flag("persist")),
            Some(("disable", _)) => vcam_disable(),
            _ => {
                println!("Use `ghostctl obs vcam status|enable|disable`.");
                Ok(())
            }
        },
        Some(("nvenc", m)) => match m.subcommand() {
            Some(("check", _)) => nvenc_check(),
            _ => nvenc_check(),
        },
        Some(("screencast", m)) => match m.subcommand() {
            Some(("test", _)) => screencast_test(),
            _ => screencast_test(),
        },
        _ => {
            println!("Use `ghostctl obs --help` to see available subcommands.");
            Ok(())
        }
    }
}

// ---- doctor ----

fn doctor(cfg: &ObsConfig) -> Result<()> {
    println!("🎥 OBS environment doctor");
    println!("─────────────────────────");

    let session = detect::session_type();
    let comp = detect::detect_compositor();
    println!("  Session     : {}", session.label());
    println!("  Compositor  : {}", comp.label());

    let backend = portal_backend(cfg, comp);
    let backend_ok = pkg_installed(backend);
    mark(backend_ok, &format!("Portal backend: {backend}"));

    let portal_ok = user_service_active("xdg-desktop-portal");
    mark(portal_ok, "xdg-desktop-portal service");

    let pipewire_ok = user_service_active("pipewire");
    mark(pipewire_ok, "PipeWire service");
    let wp_ok = user_service_active("wireplumber");
    mark(wp_ok, "WirePlumber session manager");

    let obs_ok = has("obs");
    mark(obs_ok, "OBS Studio installed");

    let vcam_loaded = module_loaded("v4l2loopback");
    mark(vcam_loaded, "v4l2loopback (virtual camera) loaded");

    let nvenc_ok = ffmpeg_has_nvenc();
    mark(nvenc_ok, "ffmpeg NVENC encoder available");

    println!();
    if session == SessionType::Wayland && !(backend_ok && portal_ok && pipewire_ok) {
        println!("⚠ Wayland screencapture needs work. Run: ghostctl obs portal setup");
    }
    if !vcam_loaded {
        println!("ℹ Virtual camera not active. Run: ghostctl obs vcam enable");
    }
    Ok(())
}

// ---- portal / Wayland screencapture ----

fn portal_check(cfg: &ObsConfig) -> Result<()> {
    let session = detect::session_type();
    let comp = detect::detect_compositor();
    let backend = portal_backend(cfg, comp);

    println!("🖥  Wayland screencapture readiness");
    println!("  Session       : {}", session.label());
    println!("  Compositor    : {}", comp.label());
    println!("  Needs backend : {backend}");
    mark(pkg_installed(backend), &format!("{backend} installed"));
    mark(
        pkg_installed("xdg-desktop-portal"),
        "xdg-desktop-portal installed",
    );
    mark(pkg_installed("pipewire"), "pipewire installed");
    mark(user_service_active("pipewire"), "pipewire running");
    mark(user_service_active("wireplumber"), "wireplumber running");
    mark(
        user_service_active("xdg-desktop-portal"),
        "xdg-desktop-portal running",
    );

    if session != SessionType::Wayland {
        println!("\nℹ Not a Wayland session; native X11 capture should work without portals.");
    } else {
        println!("\nTo install/enable everything: ghostctl obs portal setup");
    }
    Ok(())
}

fn portal_setup(cfg: &ObsConfig) -> Result<()> {
    let comp = detect::detect_compositor();
    let backend = portal_backend(cfg, comp);

    println!("🛠  Setting up Wayland screencapture for {}", comp.label());

    // Install the base portal, the compositor backend, and the PipeWire stack.
    let mut to_install: Vec<&str> = Vec::new();
    for pkg in [
        "xdg-desktop-portal",
        backend,
        "pipewire",
        "pipewire-pulse",
        "wireplumber",
    ] {
        if !pkg_installed(pkg) {
            to_install.push(pkg);
        }
    }

    if to_install.is_empty() {
        println!("  ✓ All required packages already installed.");
    } else {
        println!("  Installing: {}", to_install.join(" "));
        let mut args = vec!["-S", "--needed", "--noconfirm"];
        args.extend(to_install.iter().copied());
        let res = crate::utils::sudo_pacman(&args)
            .map_err(|e| anyhow::anyhow!("package install failed: {e}"))?;
        if !res.success {
            bail!("failed to install screencapture packages");
        }
    }

    // Enable the per-user services (no sudo: these are user units).
    for svc in [
        "pipewire",
        "pipewire-pulse",
        "wireplumber",
        "xdg-desktop-portal",
    ] {
        enable_user_service(svc);
    }

    println!("\n✓ Screencapture stack configured.");
    println!("  In OBS add a 'Screen Capture (PipeWire)' source; a portal picker will appear.");
    println!("  If sources stay black, log out and back in to restart the portal.");
    Ok(())
}

// ---- virtual camera ----

fn vcam_status() -> Result<()> {
    println!("📷 Virtual camera (v4l2loopback)");
    let loaded = module_loaded("v4l2loopback");
    mark(loaded, "module loaded");
    mark(
        pkg_installed("v4l2loopback-dkms"),
        "v4l2loopback-dkms installed",
    );

    let devices = video_devices();
    if devices.is_empty() {
        println!("  No /dev/video* devices found.");
    } else {
        println!("  Video devices: {}", devices.join(", "));
    }
    if has("v4l2-ctl") {
        let out = ProcCommand::new("v4l2-ctl").arg("--list-devices").output();
        if let Ok(o) = out
            && o.status.success()
        {
            let text = String::from_utf8_lossy(&o.stdout);
            for line in text.lines() {
                println!("    {line}");
            }
        }
    }
    Ok(())
}

fn vcam_enable(cfg: &ObsConfig, persist: bool) -> Result<()> {
    if !pkg_installed("v4l2loopback-dkms") && !module_available("v4l2loopback") {
        println!("Installing v4l2loopback-dkms (required for the virtual camera)...");
        let res =
            crate::utils::sudo_pacman(&["-S", "--needed", "--noconfirm", "v4l2loopback-dkms"])
                .map_err(|e| anyhow::anyhow!("install failed: {e}"))?;
        if !res.success {
            bail!("failed to install v4l2loopback-dkms");
        }
    }

    let video_nr = format!("video_nr={}", cfg.vcam_video_nr);
    let card_label = format!("card_label={}", cfg.vcam_label);
    println!(
        "Loading v4l2loopback (device /dev/video{}, label \"{}\")...",
        cfg.vcam_video_nr, cfg.vcam_label
    );
    let res = crate::utils::sudo_run(
        "modprobe",
        &["v4l2loopback", "exclusive_caps=1", &video_nr, &card_label],
    )
    .map_err(|e| anyhow::anyhow!("modprobe failed: {e}"))?;
    if !res.success {
        bail!("modprobe v4l2loopback failed: {}", res.stderr.trim());
    }
    println!("✓ Virtual camera ready at /dev/video{}.", cfg.vcam_video_nr);

    if persist {
        persist_vcam(cfg)?;
    }
    Ok(())
}

fn persist_vcam(cfg: &ObsConfig) -> Result<()> {
    crate::utils::sudo_write_file("/etc/modules-load.d/v4l2loopback.conf", "v4l2loopback\n")
        .map_err(|e| anyhow::anyhow!("failed to write modules-load.d: {e}"))?;
    let opts = format!(
        "options v4l2loopback exclusive_caps=1 video_nr={} card_label=\"{}\"\n",
        cfg.vcam_video_nr, cfg.vcam_label
    );
    crate::utils::sudo_write_file("/etc/modprobe.d/v4l2loopback.conf", &opts)
        .map_err(|e| anyhow::anyhow!("failed to write modprobe.d: {e}"))?;
    println!("✓ Virtual camera will load automatically on boot.");
    Ok(())
}

fn vcam_disable() -> Result<()> {
    if !module_loaded("v4l2loopback") {
        println!("v4l2loopback is not loaded.");
        return Ok(());
    }
    let res = crate::utils::sudo_run("modprobe", &["-r", "v4l2loopback"])
        .map_err(|e| anyhow::anyhow!("modprobe -r failed: {e}"))?;
    if !res.success {
        bail!(
            "failed to unload v4l2loopback (is it in use?): {}",
            res.stderr.trim()
        );
    }
    println!("✓ Virtual camera disabled.");
    Ok(())
}

// ---- NVENC ----

fn nvenc_check() -> Result<()> {
    println!("🎬 NVENC (NVIDIA hardware encoding)");
    let smi = has("nvidia-smi");
    mark(smi, "nvidia-smi present");
    if smi
        && let Ok(o) = ProcCommand::new("nvidia-smi")
            .args(["--query-gpu=driver_version,name", "--format=csv,noheader"])
            .output()
        && o.status.success()
    {
        let text = String::from_utf8_lossy(&o.stdout);
        if let Some(line) = text.lines().next() {
            println!("  GPU/driver : {}", line.trim());
        }
    }

    let ffmpeg = has("ffmpeg");
    mark(ffmpeg, "ffmpeg present");
    let nvenc = ffmpeg_has_nvenc();
    mark(nvenc, "h264_nvenc / hevc_nvenc encoder available");

    if !nvenc {
        println!("\nℹ NVENC not detected. Ensure the proprietary/open NVIDIA driver is");
        println!("  installed and ffmpeg was built with nvenc support (Arch's ffmpeg is).");
    } else {
        println!("\n✓ NVENC is available. Select NVENC in OBS Output settings.");
    }
    Ok(())
}

// ---- screencast portal probe ----

fn screencast_test() -> Result<()> {
    println!("📡 ScreenCast portal probe");
    if detect::session_type() != SessionType::Wayland {
        println!("  Session is not Wayland; portal screencapture is not required.");
        return Ok(());
    }
    if !has("busctl") {
        println!("  `busctl` not found (systemd) - cannot introspect the portal.");
        println!("  Install systemd or check `ghostctl obs portal check` instead.");
        return Ok(());
    }
    let out = ProcCommand::new("busctl")
        .args([
            "--user",
            "introspect",
            "org.freedesktop.portal.Desktop",
            "/org/freedesktop/portal/desktop",
        ])
        .output();
    let has_screencast = out
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("org.freedesktop.portal.ScreenCast"))
        .unwrap_or(false);
    mark(has_screencast, "ScreenCast portal interface available");
    if has_screencast {
        println!("\n✓ Wayland screencapture is ready. Use 'Screen Capture (PipeWire)' in OBS.");
    } else {
        println!("\n⚠ ScreenCast interface missing. Run: ghostctl obs portal setup");
    }
    Ok(())
}

// ---- helpers ----

fn portal_backend(cfg: &ObsConfig, comp: Compositor) -> &str {
    match &cfg.portal_backend {
        Some(b) => b.as_str(),
        None => detect::portal_backend_package(comp),
    }
}

fn mark(ok: bool, label: &str) {
    println!("  {} {}", if ok { "✓" } else { "✗" }, label);
}

fn has(cmd: &str) -> bool {
    which::which(cmd).is_ok()
}

/// True if the kernel module is currently loaded.
fn module_loaded(name: &str) -> bool {
    std::path::Path::new(&format!("/sys/module/{name}")).exists()
}

/// True if the module is installed/available to load (DKMS or in-tree).
fn module_available(name: &str) -> bool {
    ProcCommand::new("modinfo")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn pkg_installed(pkg: &str) -> bool {
    ProcCommand::new("pacman")
        .args(["-Q", pkg])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn user_service_active(svc: &str) -> bool {
    ProcCommand::new("systemctl")
        .args(["--user", "is-active", "--quiet", svc])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn enable_user_service(svc: &str) {
    if crate::utils::is_dry_run() {
        println!("[DRY RUN] Would enable user service: {svc}");
        return;
    }
    let status = ProcCommand::new("systemctl")
        .args(["--user", "enable", "--now", svc])
        .status();
    match status {
        Ok(s) if s.success() => println!("  ✓ enabled {svc}"),
        _ => println!("  ⚠ could not enable {svc} (may already be running)"),
    }
}

fn ffmpeg_has_nvenc() -> bool {
    ProcCommand::new("ffmpeg")
        .args(["-hide_banner", "-encoders"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("nvenc"))
        .unwrap_or(false)
}

/// List /dev/video* device paths.
fn video_devices() -> Vec<String> {
    let mut devs = Vec::new();
    if let Ok(entries) = std::fs::read_dir("/dev") {
        for e in entries.flatten() {
            let name = e.file_name();
            let name = name.to_string_lossy();
            if name.starts_with("video") {
                devs.push(format!("/dev/{name}"));
            }
        }
    }
    devs.sort();
    devs
}
