//! Single GPU passthrough support
//!
//! This module provides functionality for single GPU passthrough scenarios
//! where the host GPU needs to be released for VM use and reclaimed after.
//!
//! # Features
//!
//! - Display manager detection and control
//! - Libvirt hook generation
//! - Start/stop script generation
//! - Safe recovery options

use super::errors::VfioError;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

/// Supported display managers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisplayManager {
    /// GNOME Display Manager
    Gdm,
    /// Simple Desktop Display Manager (KDE)
    Sddm,
    /// Light Display Manager
    Lightdm,
    /// Ly console display manager
    Ly,
    /// No display manager (startx)
    Startx,
    /// Greetd (Wayland)
    Greetd,
    /// Unknown/other
    Other(String),
}

impl std::fmt::Display for DisplayManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayManager::Gdm => write!(f, "gdm"),
            DisplayManager::Sddm => write!(f, "sddm"),
            DisplayManager::Lightdm => write!(f, "lightdm"),
            DisplayManager::Ly => write!(f, "ly"),
            DisplayManager::Startx => write!(f, "none (startx)"),
            DisplayManager::Greetd => write!(f, "greetd"),
            DisplayManager::Other(s) => write!(f, "{}", s),
        }
    }
}

impl DisplayManager {
    /// Get the systemd service name for this display manager
    pub fn service_name(&self) -> Option<&str> {
        match self {
            DisplayManager::Gdm => Some("gdm"),
            DisplayManager::Sddm => Some("sddm"),
            DisplayManager::Lightdm => Some("lightdm"),
            DisplayManager::Ly => Some("ly"),
            DisplayManager::Greetd => Some("greetd"),
            DisplayManager::Startx => None,
            DisplayManager::Other(_) => None,
        }
    }
}

/// Single GPU passthrough configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleGpuConfig {
    /// GPU PCI address
    pub gpu_address: String,
    /// Audio device PCI address (if present)
    pub audio_address: Option<String>,
    /// USB controller PCI address (if passthrough desired)
    pub usb_address: Option<String>,
    /// Detected display manager
    pub display_manager: DisplayManager,
    /// VM name for libvirt hooks
    pub vm_name: String,
    /// Whether to use looking glass
    pub use_looking_glass: bool,
}

impl Default for SingleGpuConfig {
    fn default() -> Self {
        Self {
            gpu_address: String::new(),
            audio_address: None,
            usb_address: None,
            display_manager: DisplayManager::Sddm,
            vm_name: "win10".to_string(),
            use_looking_glass: false,
        }
    }
}

/// Detect the current display manager
pub fn detect_display_manager() -> DisplayManager {
    // Check systemd services
    let services = ["gdm", "sddm", "lightdm", "ly", "greetd"];

    for service in &services {
        let status = Command::new("systemctl")
            .args(["is-active", service])
            .output();

        if let Ok(output) = status {
            if String::from_utf8_lossy(&output.stdout).trim() == "active" {
                return match *service {
                    "gdm" => DisplayManager::Gdm,
                    "sddm" => DisplayManager::Sddm,
                    "lightdm" => DisplayManager::Lightdm,
                    "ly" => DisplayManager::Ly,
                    "greetd" => DisplayManager::Greetd,
                    _ => DisplayManager::Other(service.to_string()),
                };
            }
        }
    }

    // Check if running from tty (startx)
    if let Ok(tty) = std::env::var("XDG_VTNR") {
        if !tty.is_empty() {
            if std::env::var("DISPLAY").is_ok() || std::env::var("WAYLAND_DISPLAY").is_ok() {
                return DisplayManager::Startx;
            }
        }
    }

    DisplayManager::Other("unknown".to_string())
}

/// Generate the start hook script (runs before VM starts)
pub fn generate_start_hook(config: &SingleGpuConfig) -> String {
    let dm_stop = config
        .display_manager
        .service_name()
        .map(|s| format!("systemctl stop {}", s))
        .unwrap_or_else(|| "# No display manager to stop".to_string());

    let mut unbind_devices = vec![format!(
        r#"echo "0000:{}" > /sys/bus/pci/devices/0000:{}/driver/unbind 2>/dev/null || true"#,
        config.gpu_address, config.gpu_address
    )];

    if let Some(ref audio) = config.audio_address {
        unbind_devices.push(format!(
            r#"echo "0000:{}" > /sys/bus/pci/devices/0000:{}/driver/unbind 2>/dev/null || true"#,
            audio, audio
        ));
    }

    let mut bind_vfio = vec![format!(
        r#"echo "0000:{}" > /sys/bus/pci/drivers/vfio-pci/bind 2>/dev/null || true"#,
        config.gpu_address
    )];

    if let Some(ref audio) = config.audio_address {
        bind_vfio.push(format!(
            r#"echo "0000:{}" > /sys/bus/pci/drivers/vfio-pci/bind 2>/dev/null || true"#,
            audio
        ));
    }

    format!(
        r#"#!/bin/bash
# Single GPU Passthrough - Start Hook
# Generated by ghostctl
# VM: {}

set -e

# Log to journal
exec > >(logger -t "vfio-start-{}") 2>&1

echo "Starting single GPU passthrough for {}"

# Stop display manager
{}

# Wait for display manager to stop
sleep 2

# Kill remaining display processes
killall -9 Xorg 2>/dev/null || true
killall -9 Xwayland 2>/dev/null || true

# Unbind VTconsoles
echo 0 > /sys/class/vtconsole/vtcon0/bind 2>/dev/null || true
echo 0 > /sys/class/vtconsole/vtcon1/bind 2>/dev/null || true

# Unbind EFI framebuffer
echo "efi-framebuffer.0" > /sys/bus/platform/drivers/efi-framebuffer/unbind 2>/dev/null || true

# Unload GPU drivers
modprobe -r nvidia_drm nvidia_modeset nvidia_uvm nvidia 2>/dev/null || true
modprobe -r amdgpu 2>/dev/null || true
modprobe -r radeon 2>/dev/null || true
modprobe -r nouveau 2>/dev/null || true
modprobe -r i915 2>/dev/null || true

# Wait for driver unload
sleep 1

# Unbind GPU from driver
{}

# Load VFIO modules
modprobe vfio
modprobe vfio_pci
modprobe vfio_iommu_type1

# Bind to vfio-pci
{}

echo "GPU passthrough ready"
"#,
        config.vm_name,
        config.vm_name,
        config.vm_name,
        dm_stop,
        unbind_devices.join("\n"),
        bind_vfio.join("\n")
    )
}

/// Generate the stop hook script (runs after VM stops)
pub fn generate_stop_hook(config: &SingleGpuConfig) -> String {
    let dm_start = config
        .display_manager
        .service_name()
        .map(|s| format!("systemctl start {}", s))
        .unwrap_or_else(|| "# No display manager to start".to_string());

    let mut unbind_vfio = vec![format!(
        r#"echo "0000:{}" > /sys/bus/pci/drivers/vfio-pci/unbind 2>/dev/null || true"#,
        config.gpu_address
    )];

    if let Some(ref audio) = config.audio_address {
        unbind_vfio.push(format!(
            r#"echo "0000:{}" > /sys/bus/pci/drivers/vfio-pci/unbind 2>/dev/null || true"#,
            audio
        ));
    }

    // Determine which GPU driver to load based on vendor
    let gpu_driver = if config.gpu_address.contains("10de") || config.gpu_address.starts_with("01")
    {
        // NVIDIA typically on slot 01
        "nvidia"
    } else {
        "amdgpu"
    };

    format!(
        r#"#!/bin/bash
# Single GPU Passthrough - Stop Hook
# Generated by ghostctl
# VM: {}

set -e

# Log to journal
exec > >(logger -t "vfio-stop-{}") 2>&1

echo "Restoring host GPU for {}"

# Unbind from vfio-pci
{}

# Unload vfio-pci (optional, keeps it loaded for next time)
# modprobe -r vfio_pci

# Rebind VTconsoles
echo 1 > /sys/class/vtconsole/vtcon0/bind 2>/dev/null || true
echo 1 > /sys/class/vtconsole/vtcon1/bind 2>/dev/null || true

# Rebind EFI framebuffer
echo "efi-framebuffer.0" > /sys/bus/platform/drivers/efi-framebuffer/bind 2>/dev/null || true

# Load GPU driver
modprobe {} 2>/dev/null || true

# Wait for GPU to initialize
sleep 2

# Trigger PCI rescan
echo 1 > /sys/bus/pci/rescan

# Wait for rescan
sleep 1

# Start display manager
{}

echo "Host GPU restored"
"#,
        config.vm_name,
        config.vm_name,
        config.vm_name,
        unbind_vfio.join("\n"),
        gpu_driver,
        dm_start
    )
}

/// Generate libvirt hook script
pub fn generate_libvirt_hook(config: &SingleGpuConfig) -> String {
    format!(
        r#"#!/bin/bash
# Libvirt hook for single GPU passthrough
# Generated by ghostctl
# Place in /etc/libvirt/hooks/qemu

GUEST_NAME="$1"
HOOK_NAME="$2"
STATE_NAME="$3"

# Only handle our VM
if [ "$GUEST_NAME" != "{}" ]; then
    exit 0
fi

HOOKS_DIR="/etc/libvirt/hooks/qemu.d/$GUEST_NAME"

case "$HOOK_NAME/$STATE_NAME" in
    prepare/begin)
        if [ -x "$HOOKS_DIR/prepare/begin/start.sh" ]; then
            "$HOOKS_DIR/prepare/begin/start.sh"
        fi
        ;;
    release/end)
        if [ -x "$HOOKS_DIR/release/end/stop.sh" ]; then
            "$HOOKS_DIR/release/end/stop.sh"
        fi
        ;;
esac
"#,
        config.vm_name
    )
}

/// Write all single GPU passthrough hooks
pub fn write_hooks(config: &SingleGpuConfig) -> Result<()> {
    // Create directory structure
    let hooks_base = "/etc/libvirt/hooks";
    let vm_hooks = format!("{}/qemu.d/{}", hooks_base, config.vm_name);
    let start_dir = format!("{}/prepare/begin", vm_hooks);
    let stop_dir = format!("{}/release/end", vm_hooks);

    for dir in &[&start_dir, &stop_dir] {
        fs::create_dir_all(dir).with_context(|| format!("Failed to create {}", dir))?;
    }

    // Write main hook
    let main_hook_path = format!("{}/qemu", hooks_base);
    let main_hook_content = generate_libvirt_hook(config);
    fs::write(&main_hook_path, &main_hook_content)
        .with_context(|| format!("Failed to write {}", main_hook_path))?;
    set_executable(&main_hook_path)?;
    println!("Written: {}", main_hook_path);

    // Write start hook
    let start_hook_path = format!("{}/start.sh", start_dir);
    let start_hook_content = generate_start_hook(config);
    fs::write(&start_hook_path, &start_hook_content)
        .with_context(|| format!("Failed to write {}", start_hook_path))?;
    set_executable(&start_hook_path)?;
    println!("Written: {}", start_hook_path);

    // Write stop hook
    let stop_hook_path = format!("{}/stop.sh", stop_dir);
    let stop_hook_content = generate_stop_hook(config);
    fs::write(&stop_hook_path, &stop_hook_content)
        .with_context(|| format!("Failed to write {}", stop_hook_path))?;
    set_executable(&stop_hook_path)?;
    println!("Written: {}", stop_hook_path);

    // Restart libvirtd to pick up hooks
    println!("\nRestart libvirtd to apply hooks:");
    println!("  sudo systemctl restart libvirtd");

    Ok(())
}

/// Set file as executable
fn set_executable(path: &str) -> Result<()> {
    let metadata = fs::metadata(path)?;
    let mut perms = metadata.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms)?;
    Ok(())
}

/// Check if libvirt hooks exist for a VM
pub fn check_hooks_exist(vm_name: &str) -> bool {
    let vm_hooks = format!("/etc/libvirt/hooks/qemu.d/{}", vm_name);
    Path::new(&vm_hooks).exists()
}

/// List existing single GPU passthrough configurations
pub fn list_configurations() -> Vec<String> {
    let hooks_dir = "/etc/libvirt/hooks/qemu.d";
    let mut configs = Vec::new();

    if let Ok(entries) = fs::read_dir(hooks_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                if let Some(name) = entry.file_name().to_str() {
                    configs.push(name.to_string());
                }
            }
        }
    }

    configs
}

/// Print single GPU passthrough status
pub fn print_status() {
    println!("\nSingle GPU Passthrough Status");
    println!("{}", "=".repeat(50));

    // Detect display manager
    let dm = detect_display_manager();
    println!("\nDisplay Manager: {}", dm);

    // Check libvirt
    let libvirt_running = Command::new("systemctl")
        .args(["is-active", "libvirtd"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "active")
        .unwrap_or(false);

    println!(
        "Libvirtd:        {}",
        if libvirt_running {
            "Running"
        } else {
            "Not running"
        }
    );

    // Check existing configurations
    let configs = list_configurations();
    println!("\nConfigured VMs:");
    if configs.is_empty() {
        println!("  No single GPU passthrough configurations found");
    } else {
        for config in configs {
            println!("  - {}", config);
        }
    }

    // Check hooks directory
    let hooks_exist = Path::new("/etc/libvirt/hooks/qemu").exists();
    println!(
        "\nLibvirt hooks:   {}",
        if hooks_exist {
            "Configured"
        } else {
            "Not configured"
        }
    );
}

/// Remove single GPU passthrough hooks for a VM
pub fn remove_hooks(vm_name: &str) -> Result<()> {
    let vm_hooks = format!("/etc/libvirt/hooks/qemu.d/{}", vm_name);

    if !Path::new(&vm_hooks).exists() {
        return Err(
            VfioError::SingleGpuError(format!("No hooks found for VM '{}'", vm_name)).into(),
        );
    }

    fs::remove_dir_all(&vm_hooks)
        .with_context(|| format!("Failed to remove hooks for {}", vm_name))?;

    println!("Removed hooks for VM: {}", vm_name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_manager_service_name() {
        assert_eq!(DisplayManager::Gdm.service_name(), Some("gdm"));
        assert_eq!(DisplayManager::Sddm.service_name(), Some("sddm"));
        assert_eq!(DisplayManager::Startx.service_name(), None);
    }

    #[test]
    fn test_generate_start_hook() {
        let config = SingleGpuConfig {
            gpu_address: "01:00.0".to_string(),
            audio_address: Some("01:00.1".to_string()),
            usb_address: None,
            display_manager: DisplayManager::Sddm,
            vm_name: "win10".to_string(),
            use_looking_glass: false,
        };

        let hook = generate_start_hook(&config);
        assert!(hook.contains("systemctl stop sddm"));
        assert!(hook.contains("0000:01:00.0"));
        assert!(hook.contains("0000:01:00.1"));
        assert!(hook.contains("modprobe vfio_pci"));
    }

    #[test]
    fn test_generate_stop_hook() {
        let config = SingleGpuConfig {
            gpu_address: "01:00.0".to_string(),
            audio_address: None,
            usb_address: None,
            display_manager: DisplayManager::Gdm,
            vm_name: "test".to_string(),
            use_looking_glass: false,
        };

        let hook = generate_stop_hook(&config);
        assert!(hook.contains("systemctl start gdm"));
        assert!(hook.contains("0000:01:00.0"));
    }
}
