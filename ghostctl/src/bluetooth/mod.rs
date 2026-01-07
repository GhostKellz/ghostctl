//! Bluetooth Management Module
//!
//! Provides CLI and TUI interfaces for managing Bluetooth adapters and devices.
//! Based on the bluer crate for Linux BlueZ DBus interface.

mod tui;

use crate::tui::{confirm, error, header, icons, info, select_with_back, success, warn};
use std::time::Duration;

pub use tui::bluetooth_tui;

/// Main Bluetooth management menu
pub fn bluetooth_menu() {
    loop {
        header("Bluetooth Management");

        let options = [
            "Launch TUI (Interactive Manager)",
            "List Adapters",
            "List Paired Devices",
            "Scan for Devices",
            "Toggle Power",
            "Toggle Discoverable",
            "Back",
        ];

        match select_with_back("Choose an option", &options, 0) {
            Some(0) => {
                if let Err(e) = bluetooth_tui() {
                    error(&format!("Failed to launch Bluetooth TUI: {}", e));
                }
            }
            Some(1) => list_adapters(),
            Some(2) => list_paired_devices(),
            Some(3) => scan_for_devices(),
            Some(4) => toggle_adapter_power(),
            Some(5) => toggle_discoverable(),
            _ => break,
        }
    }
}

/// List Bluetooth adapters
fn list_adapters() {
    info("Fetching Bluetooth adapters...");

    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            error(&format!("Failed to create runtime: {}", e));
            return;
        }
    };

    rt.block_on(async {
        match bluer::Session::new().await {
            Ok(session) => {
                let adapter_names = session.adapter_names().await.unwrap_or_default();

                if adapter_names.is_empty() {
                    warn("No Bluetooth adapters found");
                    return;
                }

                for name in adapter_names {
                    match session.adapter(&name) {
                        Ok(adapter) => {
                            let powered = adapter.is_powered().await.unwrap_or(false);
                            let discoverable = adapter.is_discoverable().await.unwrap_or(false);
                            let address = adapter
                                .address()
                                .await
                                .map(|a| a.to_string())
                                .unwrap_or_else(|_| "unknown".to_string());

                            println!(
                                "\n{} Adapter: {}",
                                if powered {
                                    icons::success()
                                } else {
                                    icons::error()
                                },
                                name
                            );
                            println!("  Address: {}", address);
                            println!("  Power: {}", if powered { "ON" } else { "OFF" });
                            println!(
                                "  Discoverable: {}",
                                if discoverable { "YES" } else { "NO" }
                            );
                        }
                        Err(e) => {
                            error(&format!("Failed to get adapter {}: {}", name, e));
                        }
                    }
                }
            }
            Err(e) => {
                error(&format!("Failed to connect to BlueZ: {}", e));
                info("Make sure the bluetooth service is running: sudo systemctl start bluetooth");
            }
        }
    });
}

/// List paired/known devices
fn list_paired_devices() {
    info("Fetching paired devices...");

    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            error(&format!("Failed to create runtime: {}", e));
            return;
        }
    };

    rt.block_on(async {
        match bluer::Session::new().await {
            Ok(session) => {
                let adapter_names = session.adapter_names().await.unwrap_or_default();

                if adapter_names.is_empty() {
                    warn("No Bluetooth adapters found");
                    return;
                }

                for adapter_name in adapter_names {
                    if let Ok(adapter) = session.adapter(&adapter_name) {
                        println!("\n{} Devices on adapter: {}", icons::info(), adapter_name);

                        match adapter.device_addresses().await {
                            Ok(addresses) => {
                                if addresses.is_empty() {
                                    warn("  No devices found");
                                    continue;
                                }

                                for addr in addresses {
                                    if let Ok(device) = adapter.device(addr) {
                                        let name = device
                                            .name()
                                            .await
                                            .ok()
                                            .flatten()
                                            .unwrap_or_else(|| "Unknown".to_string());
                                        let connected =
                                            device.is_connected().await.unwrap_or(false);
                                        let paired = device.is_paired().await.unwrap_or(false);
                                        let trusted = device.is_trusted().await.unwrap_or(false);

                                        let status_icon = if connected {
                                            icons::success()
                                        } else if paired {
                                            icons::info()
                                        } else {
                                            icons::warn()
                                        };

                                        println!("\n  {} {} ({})", status_icon, name, addr);
                                        println!(
                                            "    Connected: {} | Paired: {} | Trusted: {}",
                                            if connected { "Yes" } else { "No" },
                                            if paired { "Yes" } else { "No" },
                                            if trusted { "Yes" } else { "No" }
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                error(&format!("  Failed to list devices: {}", e));
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error(&format!("Failed to connect to BlueZ: {}", e));
            }
        }
    });
}

/// Scan for nearby Bluetooth devices
fn scan_for_devices() {
    info("Starting Bluetooth scan (10 seconds)...");

    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            error(&format!("Failed to create runtime: {}", e));
            return;
        }
    };

    rt.block_on(async {
        match bluer::Session::new().await {
            Ok(session) => {
                let adapter_names = session.adapter_names().await.unwrap_or_default();
                if adapter_names.is_empty() {
                    warn("No Bluetooth adapters found");
                    return;
                }

                let adapter_name = &adapter_names[0];
                match session.adapter(adapter_name) {
                    Ok(adapter) => {
                        // Ensure adapter is powered
                        if !adapter.is_powered().await.unwrap_or(false)
                            && let Err(e) = adapter.set_powered(true).await {
                                error(&format!("Failed to power on adapter: {}", e));
                                return;
                            }

                        // Start discovery
                        match adapter.discover_devices().await {
                            Ok(mut stream) => {
                                use futures::StreamExt;

                                println!("\nDiscovered devices:");
                                let scan_duration = Duration::from_secs(10);
                                let start = std::time::Instant::now();

                                while start.elapsed() < scan_duration {
                                    match tokio::time::timeout(
                                        Duration::from_millis(500),
                                        stream.next(),
                                    )
                                    .await
                                    {
                                        Ok(Some(event)) => {
                                            if let bluer::AdapterEvent::DeviceAdded(addr) = event
                                                && let Ok(device) = adapter.device(addr) {
                                                    let name = device
                                                        .name()
                                                        .await
                                                        .ok()
                                                        .flatten()
                                                        .unwrap_or_else(|| "Unknown".to_string());
                                                    let rssi = device
                                                        .rssi()
                                                        .await
                                                        .ok()
                                                        .flatten()
                                                        .map(|r| format!("{}dBm", r))
                                                        .unwrap_or_else(|| "N/A".to_string());
                                                    println!(
                                                        "  {} {} ({}) [RSSI: {}]",
                                                        icons::success(),
                                                        name,
                                                        addr,
                                                        rssi
                                                    );
                                                }
                                        }
                                        Ok(None) => break,
                                        Err(_) => {
                                            // Timeout - continue scanning
                                            print!(".");
                                            use std::io::Write;
                                            let _ = std::io::stdout().flush();
                                        }
                                    }
                                }
                                println!();
                                success("Scan complete");
                            }
                            Err(e) => {
                                error(&format!("Failed to start discovery: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        error(&format!("Failed to get adapter: {}", e));
                    }
                }
            }
            Err(e) => {
                error(&format!("Failed to connect to BlueZ: {}", e));
            }
        }
    });
}

/// Toggle adapter power on/off
fn toggle_adapter_power() {
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            error(&format!("Failed to create runtime: {}", e));
            return;
        }
    };

    rt.block_on(async {
        match bluer::Session::new().await {
            Ok(session) => {
                let adapter_names = session.adapter_names().await.unwrap_or_default();
                if adapter_names.is_empty() {
                    warn("No Bluetooth adapters found");
                    return;
                }

                // If multiple adapters, let user choose
                let adapter_name = if adapter_names.len() == 1 {
                    adapter_names[0].clone()
                } else {
                    let names: Vec<&str> = adapter_names.iter().map(|s| s.as_str()).collect();
                    match select_with_back("Select adapter", &names, 0) {
                        Some(idx) => adapter_names[idx].clone(),
                        None => return,
                    }
                };

                match session.adapter(&adapter_name) {
                    Ok(adapter) => {
                        let current = adapter.is_powered().await.unwrap_or(false);
                        let new_state = !current;

                        let msg = format!(
                            "Turn {} adapter {}?",
                            if new_state { "ON" } else { "OFF" },
                            adapter_name
                        );

                        if confirm(&msg, true) {
                            match adapter.set_powered(new_state).await {
                                Ok(_) => {
                                    success(&format!(
                                        "Adapter {} is now {}",
                                        adapter_name,
                                        if new_state { "ON" } else { "OFF" }
                                    ));
                                }
                                Err(e) => {
                                    error(&format!("Failed to toggle power: {}", e));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error(&format!("Failed to get adapter: {}", e));
                    }
                }
            }
            Err(e) => {
                error(&format!("Failed to connect to BlueZ: {}", e));
            }
        }
    });
}

/// Toggle adapter discoverable mode
fn toggle_discoverable() {
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            error(&format!("Failed to create runtime: {}", e));
            return;
        }
    };

    rt.block_on(async {
        match bluer::Session::new().await {
            Ok(session) => {
                let adapter_names = session.adapter_names().await.unwrap_or_default();
                if adapter_names.is_empty() {
                    warn("No Bluetooth adapters found");
                    return;
                }

                let adapter_name = &adapter_names[0];
                match session.adapter(adapter_name) {
                    Ok(adapter) => {
                        let current = adapter.is_discoverable().await.unwrap_or(false);
                        let new_state = !current;

                        let msg = format!(
                            "Make adapter {} {}?",
                            adapter_name,
                            if new_state { "discoverable" } else { "hidden" }
                        );

                        if confirm(&msg, true) {
                            match adapter.set_discoverable(new_state).await {
                                Ok(_) => {
                                    success(&format!(
                                        "Adapter {} is now {}",
                                        adapter_name,
                                        if new_state { "discoverable" } else { "hidden" }
                                    ));
                                }
                                Err(e) => {
                                    error(&format!("Failed to toggle discoverable: {}", e));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error(&format!("Failed to get adapter: {}", e));
                    }
                }
            }
            Err(e) => {
                error(&format!("Failed to connect to BlueZ: {}", e));
            }
        }
    });
}
