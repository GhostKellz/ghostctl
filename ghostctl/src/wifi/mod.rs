//! WiFi Management Module
//!
//! Provides CLI interface for WiFi management using iwctl (iwd).
//! Uses iwctl commands for maximum compatibility.

use crate::tui::{confirm, error, header, icons, info, input, select_with_back, success, warn};
use std::process::Command;

/// Main WiFi management menu
pub fn wifi_menu() {
    loop {
        header("WiFi Management");

        let options = [
            "Show Status",
            "List Known Networks",
            "Scan for Networks",
            "Connect to Network",
            "Disconnect",
            "Toggle WiFi Power",
            "List Devices",
            "Back",
        ];

        match select_with_back("Choose an option", &options, 0) {
            Some(0) => show_wifi_status(),
            Some(1) => list_known_networks(),
            Some(2) => scan_networks(),
            Some(3) => connect_to_network(),
            Some(4) => disconnect_wifi(),
            Some(5) => toggle_wifi_power(),
            Some(6) => list_devices(),
            _ => break,
        }
    }
}

/// Check if iwd/iwctl is available
fn check_iwctl() -> bool {
    Command::new("which")
        .arg("iwctl")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Get the default WiFi device name
fn get_default_device() -> Option<String> {
    let output = Command::new("iwctl")
        .args(["device", "list"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Parse the device list output - typically the first device after header
    for line in stdout.lines().skip(4) {
        // Skip header lines
        let parts: Vec<&str> = line.split_whitespace().collect();
        if !parts.is_empty() && parts[0].starts_with("wl") {
            return Some(parts[0].to_string());
        }
    }

    None
}

/// List WiFi devices
fn list_devices() {
    if !check_iwctl() {
        error("iwctl not found. Please install iwd: sudo pacman -S iwd");
        return;
    }

    info("WiFi Devices:");
    let _ = Command::new("iwctl").args(["device", "list"]).status();
}

/// Show WiFi status
fn show_wifi_status() {
    if !check_iwctl() {
        error("iwctl not found. Please install iwd: sudo pacman -S iwd");
        return;
    }

    info("WiFi Status:");

    if let Some(device) = get_default_device() {
        println!("\n{} Device: {}", icons::network(), device);

        // Show device status
        let _ = Command::new("iwctl")
            .args(["device", &device, "show"])
            .status();

        // Show station info if in station mode
        println!("\n{} Station Info:", icons::info());
        let _ = Command::new("iwctl")
            .args(["station", &device, "show"])
            .status();
    } else {
        warn("No WiFi device found");
    }
}

/// List known/saved networks
fn list_known_networks() {
    if !check_iwctl() {
        error("iwctl not found. Please install iwd: sudo pacman -S iwd");
        return;
    }

    info("Known Networks:");
    let _ = Command::new("iwctl")
        .args(["known-networks", "list"])
        .status();
}

/// Scan for available networks
fn scan_networks() {
    if !check_iwctl() {
        error("iwctl not found. Please install iwd: sudo pacman -S iwd");
        return;
    }

    let device = match get_default_device() {
        Some(d) => d,
        None => {
            warn("No WiFi device found");
            return;
        }
    };

    info(&format!("Scanning for networks on {}...", device));

    // Trigger scan
    let _ = Command::new("iwctl")
        .args(["station", &device, "scan"])
        .status();

    // Wait a bit for results
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Show results
    println!("\n{} Available Networks:", icons::network());
    let _ = Command::new("iwctl")
        .args(["station", &device, "get-networks"])
        .status();
}

/// Connect to a network
fn connect_to_network() {
    if !check_iwctl() {
        error("iwctl not found. Please install iwd: sudo pacman -S iwd");
        return;
    }

    let device = match get_default_device() {
        Some(d) => d,
        None => {
            warn("No WiFi device found");
            return;
        }
    };

    // First show available networks
    scan_networks();

    // Get SSID from user
    let ssid = match input("Enter network SSID", None) {
        Some(s) if !s.is_empty() => s,
        _ => {
            warn("No SSID provided");
            return;
        }
    };

    info(&format!("Connecting to {}...", ssid));

    // Try to connect - iwctl will prompt for password if needed
    let status = Command::new("iwctl")
        .args(["station", &device, "connect", &ssid])
        .status();

    match status {
        Ok(s) if s.success() => success(&format!("Connected to {}", ssid)),
        Ok(_) => error("Connection failed. Check your password and try again."),
        Err(e) => error(&format!("Failed to run iwctl: {}", e)),
    }
}

/// Connect to a network with password
pub fn connect_with_password(ssid: &str, password: &str) -> Result<(), String> {
    let device = get_default_device().ok_or("No WiFi device found")?;

    // Use iwctl with password via stdin or environment
    let output = Command::new("iwctl")
        .args([
            "--passphrase",
            password,
            "station",
            &device,
            "connect",
            ssid,
        ])
        .output()
        .map_err(|e| format!("Failed to run iwctl: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Disconnect from current network
fn disconnect_wifi() {
    if !check_iwctl() {
        error("iwctl not found. Please install iwd: sudo pacman -S iwd");
        return;
    }

    let device = match get_default_device() {
        Some(d) => d,
        None => {
            warn("No WiFi device found");
            return;
        }
    };

    if !confirm("Disconnect from current network?", true) {
        return;
    }

    let status = Command::new("iwctl")
        .args(["station", &device, "disconnect"])
        .status();

    match status {
        Ok(s) if s.success() => success("Disconnected from network"),
        Ok(_) => warn("Disconnect command completed (may not have been connected)"),
        Err(e) => error(&format!("Failed to disconnect: {}", e)),
    }
}

/// Toggle WiFi power on/off
fn toggle_wifi_power() {
    if !check_iwctl() {
        error("iwctl not found. Please install iwd: sudo pacman -S iwd");
        return;
    }

    let device = match get_default_device() {
        Some(d) => d,
        None => {
            warn("No WiFi device found");
            return;
        }
    };

    // Check current power state
    let output = Command::new("iwctl")
        .args(["device", &device, "show"])
        .output();

    let is_powered = match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.contains("Powered") && stdout.contains("on")
        }
        Err(_) => true, // Assume powered if we can't check
    };

    let new_state = if is_powered { "off" } else { "on" };
    let msg = format!("Turn WiFi {}?", new_state.to_uppercase());

    if !confirm(&msg, true) {
        return;
    }

    let status = Command::new("iwctl")
        .args(["device", &device, "set-property", "Powered", new_state])
        .status();

    match status {
        Ok(s) if s.success() => success(&format!("WiFi is now {}", new_state.to_uppercase())),
        Ok(_) => error("Failed to toggle WiFi power"),
        Err(e) => error(&format!("Failed to run iwctl: {}", e)),
    }
}

/// WiFi TUI - placeholder that launches the menu
pub fn wifi_tui() -> anyhow::Result<()> {
    // For now, just run the menu
    // A full TUI implementation would be similar to the Bluetooth one
    wifi_menu();
    Ok(())
}
