//! VFIO passthrough management for ghostctl
//!
//! This module provides functionality for managing VFIO device passthrough,
//! including device binding, configuration, and single-GPU passthrough.
//!
//! # Features
//!
//! - Runtime device binding/unbinding to vfio-pci
//! - VFIO module status checking and loading
//! - Configuration file generation (modprobe.d, initramfs)
//! - Single GPU passthrough script generation
//! - VBIOS ROM dumping
//!
//! # Example
//!
//! ```no_run
//! use ghostctl::vfio;
//!
//! // Check VFIO modules
//! let modules = vfio::check_vfio_modules().unwrap();
//! if !modules.vfio_pci_loaded {
//!     vfio::load_vfio_modules().unwrap();
//! }
//!
//! // Bind a device
//! vfio::bind_device("01:00.0").unwrap();
//! ```

pub mod bind;
pub mod config;
pub mod errors;
pub mod rom;
pub mod single_gpu;

// Re-export main types and functions (public API)
#[allow(unused_imports)]
pub use bind::{
    VfioBindingStatus, VfioModuleStatus, bind_device, check_vfio_modules, get_current_driver,
    get_vfio_status, load_vfio_modules, print_status, rebind_to_original, unbind_device,
};
#[allow(unused_imports)]
pub use config::{
    InitramfsSystem, VfioConfig, VfioConfigStatus, VfioDeviceConfig, check_existing_config,
    detect_initramfs_system, generate_modprobe_config, get_kernel_params_recommendation,
    print_config_status, write_initramfs_config, write_modprobe_config,
};
#[allow(unused_imports)]
pub use errors::{VfioError, VfioResult};
#[allow(unused_imports)]
pub use rom::{
    RomDeviceInfo, RomDumpResult, check_rom_accessible, dump_rom, dump_rom_nvidia,
    list_rom_devices, print_rom_devices,
};
#[allow(unused_imports)]
pub use single_gpu::{
    DisplayManager, SingleGpuConfig, check_hooks_exist, detect_display_manager,
    generate_libvirt_hook, generate_start_hook, generate_stop_hook, list_configurations,
    print_status as print_single_gpu_status, remove_hooks, write_hooks,
};

use anyhow::Result;
use dialoguer::{Input, Select, theme::ColorfulTheme};

/// Extract the PCI bus prefix (domain:bus) from a PCI address.
///
/// PCI addresses are in format: DDDD:BB:DD.F (e.g., "0000:01:00.0")
/// This extracts "DDDD:BB" (e.g., "0000:01") to match devices on the same bus.
///
/// Returns None if the address is too short or malformed.
fn pci_bus_prefix(address: &str) -> Option<&str> {
    // Format: DDDD:BB:DD.F - we want first 7 chars (DDDD:BB)
    // But we should validate the format first
    if address.len() < 7 {
        return None;
    }
    // Verify the colon is in the right place
    if address.chars().nth(4) != Some(':') {
        return None;
    }
    Some(&address[..7])
}

/// Check if two PCI addresses are on the same bus (same domain:bus prefix)
fn same_pci_bus(addr1: &str, addr2: &str) -> bool {
    match (pci_bus_prefix(addr1), pci_bus_prefix(addr2)) {
        (Some(p1), Some(p2)) => p1 == p2,
        _ => false,
    }
}

/// Interactive VFIO management menu
pub fn vfio_menu() {
    loop {
        let options = vec![
            "Show VFIO Status",
            "Bind Device to VFIO",
            "Unbind Device from VFIO",
            "Load VFIO Modules",
            "List VFIO-Bound Devices",
            "Configuration Status",
            "Setup Wizard",
            "Single GPU Passthrough",
            "Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("VFIO Management")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(sel)) => sel,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => show_vfio_status(),
            1 => bind_device_interactive(),
            2 => unbind_device_interactive(),
            3 => load_modules_interactive(),
            4 => list_bound_devices(),
            5 => print_config_status(),
            6 => {
                if let Err(e) = vfio_setup_wizard() {
                    eprintln!("Setup wizard error: {}", e);
                }
            }
            7 => single_gpu_menu(),
            _ => break,
        }

        println!(); // Add spacing after each operation
    }
}

/// Single GPU passthrough submenu
fn single_gpu_menu() {
    loop {
        let options = vec![
            "Show Status",
            "Setup Single GPU Passthrough",
            "List Configurations",
            "Remove Configuration",
            "Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Single GPU Passthrough")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(sel)) => sel,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => print_single_gpu_status(),
            1 => {
                if let Err(e) = setup_single_gpu_interactive() {
                    eprintln!("Error: {}", e);
                }
            }
            2 => {
                let configs = list_configurations();
                if configs.is_empty() {
                    println!("\nNo single GPU passthrough configurations found.");
                } else {
                    println!("\nConfigured VMs:");
                    for config in configs {
                        println!("  - {}", config);
                    }
                }
            }
            3 => {
                let configs = list_configurations();
                if configs.is_empty() {
                    println!("\nNo configurations to remove.");
                } else {
                    let selection = match Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select VM to remove hooks")
                        .items(&configs)
                        .interact_opt()
                    {
                        Ok(Some(sel)) => sel,
                        Ok(None) | Err(_) => continue,
                    };

                    if let Err(e) = remove_hooks(&configs[selection]) {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            _ => break,
        }

        println!();
    }
}

/// Interactive single GPU passthrough setup
fn setup_single_gpu_interactive() -> Result<()> {
    use dialoguer::Confirm;

    println!("\nSingle GPU Passthrough Setup");
    println!("{}", "=".repeat(50));

    // Detect display manager
    let dm = detect_display_manager();
    println!("\nDetected display manager: {}", dm);

    // Select GPU
    let gpus = match crate::iommu::list_gpus() {
        Ok(g) => g,
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to list GPUs: {}", e));
        }
    };

    if gpus.is_empty() {
        println!("No GPUs found.");
        return Ok(());
    }

    println!("\nAvailable GPUs:");
    let gpu_options: Vec<String> = gpus
        .iter()
        .map(|g| {
            let driver = g
                .current_driver
                .as_ref()
                .map(|d| format!(" [{}]", d))
                .unwrap_or_default();
            format!("{} - {}{}", g.address, g.description, driver)
        })
        .collect();

    let gpu_selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select GPU for passthrough")
        .items(&gpu_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(sel)) => sel,
        Ok(None) | Err(_) => return Ok(()),
    };

    let selected_gpu = &gpus[gpu_selection];

    // Find audio device on same bus (same domain:bus prefix)
    let audio_address = crate::iommu::get_iommu_group(selected_gpu.iommu_group)
        .ok()
        .and_then(|group| {
            group
                .devices
                .iter()
                .find(|d| {
                    same_pci_bus(&d.address, &selected_gpu.address)
                        && d.address != selected_gpu.address
                        && d.class.description().contains("Audio")
                })
                .map(|d| d.address.clone())
        });

    if let Some(ref audio) = audio_address {
        println!("Found audio device: {}", audio);
    }

    // Get VM name
    let vm_name: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("VM name (for libvirt hooks)")
        .default("win10".to_string())
        .interact_text()
    {
        Ok(name) => name,
        Err(_) => return Ok(()),
    };

    // Create configuration
    let config = SingleGpuConfig {
        gpu_address: selected_gpu.address.trim_start_matches("0000:").to_string(),
        audio_address: audio_address.map(|a| a.trim_start_matches("0000:").to_string()),
        usb_address: None,
        display_manager: dm,
        vm_name: vm_name.clone(),
        use_looking_glass: false,
    };

    // Preview hooks
    println!("\n{}", "-".repeat(50));
    println!("Start hook preview:");
    println!("{}", "-".repeat(50));
    let start_hook = generate_start_hook(&config);
    let start_preview: Vec<&str> = start_hook.lines().take(20).collect();
    for line in start_preview {
        println!("{}", line);
    }
    println!("...");

    // Confirm and write
    let write = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Write libvirt hooks? (requires root)")
        .default(false)
        .interact_opt()
    {
        Ok(Some(v)) => v,
        Ok(None) | Err(_) => false,
    };

    if write {
        write_hooks(&config)?;
        println!("\nSingle GPU passthrough configured for VM: {}", vm_name);
        println!("\nTo use:");
        println!("  1. Start your VM with: virsh start {}", vm_name);
        println!("  2. The host display will stop and GPU will be passed to VM");
        println!("  3. When VM shuts down, host display will be restored");
    }

    Ok(())
}

/// Show VFIO status
fn show_vfio_status() {
    print_status(false);
}

/// Interactive device binding
fn bind_device_interactive() {
    println!("\nBind Device to VFIO");
    println!("{}", "=".repeat(40));

    // Show available GPUs
    println!("\nAvailable GPUs:");
    if let Ok(gpus) = crate::iommu::list_gpus() {
        for gpu in &gpus {
            let driver = gpu
                .current_driver
                .as_ref()
                .map(|d| format!("[{}]", d))
                .unwrap_or_default();
            println!(
                "  {} - {} {} (Group {})",
                gpu.address, gpu.description, driver, gpu.iommu_group
            );
        }
    }

    let address: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter PCI address to bind (e.g., 01:00.0)")
        .interact_text()
    {
        Ok(addr) => addr,
        Err(_) => return,
    };

    match bind_device(&address) {
        Ok(_) => {
            println!("\nDevice {} successfully bound to vfio-pci", address);
            println!("Note: This binding is runtime only. Use configuration for persistence.");
        }
        Err(e) => {
            eprintln!("\nError binding device: {}", e);
            eprintln!("Hint: Make sure you're running as root (sudo)");
        }
    }
}

/// Interactive device unbinding
fn unbind_device_interactive() {
    println!("\nUnbind Device from VFIO");
    println!("{}", "=".repeat(40));

    // Show VFIO-bound devices
    println!("\nCurrently VFIO-bound devices:");
    let statuses = match get_vfio_status() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error getting VFIO status: {}", e);
            return;
        }
    };

    if statuses.is_empty() {
        println!("  No devices currently bound to vfio-pci");
        return;
    }

    for status in &statuses {
        println!(
            "  {} - {} (Group {})",
            status.device, status.description, status.iommu_group
        );
    }

    let address: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter PCI address to unbind")
        .interact_text()
    {
        Ok(addr) => addr,
        Err(_) => return,
    };

    match unbind_device(&address) {
        Ok(_) => {
            println!("\nDevice {} unbound from vfio-pci", address);
        }
        Err(e) => {
            eprintln!("\nError unbinding device: {}", e);
            eprintln!("Hint: Make sure you're running as root (sudo)");
        }
    }
}

/// Load VFIO modules
fn load_modules_interactive() {
    println!("\nLoading VFIO Modules");
    println!("{}", "=".repeat(40));

    // Check current status
    if let Ok(modules) = check_vfio_modules()
        && modules.vfio_loaded
        && modules.vfio_pci_loaded
        && modules.vfio_iommu_type1_loaded
    {
        println!("All VFIO modules are already loaded.");
        return;
    }

    match load_vfio_modules() {
        Ok(_) => {
            println!("VFIO modules loaded successfully.");

            // Verify
            if let Ok(modules) = check_vfio_modules() {
                println!("\nModule Status:");
                println!(
                    "  vfio:            {}",
                    if modules.vfio_loaded {
                        "Loaded"
                    } else {
                        "Not loaded"
                    }
                );
                println!(
                    "  vfio_pci:        {}",
                    if modules.vfio_pci_loaded {
                        "Loaded"
                    } else {
                        "Not loaded"
                    }
                );
                println!(
                    "  vfio_iommu_type1: {}",
                    if modules.vfio_iommu_type1_loaded {
                        "Loaded"
                    } else {
                        "Not loaded"
                    }
                );
            }
        }
        Err(e) => {
            eprintln!("Error loading modules: {}", e);
            eprintln!("Hint: Make sure you're running as root (sudo)");
        }
    }
}

/// List all VFIO-bound devices
fn list_bound_devices() {
    println!("\nVFIO-Bound Devices");
    println!("{}", "=".repeat(40));

    match get_vfio_status() {
        Ok(statuses) => {
            if statuses.is_empty() {
                println!("No devices currently bound to vfio-pci");
            } else {
                for status in &statuses {
                    println!("\nDevice: {}", status.device);
                    println!("  ID:          {}", status.device_id);
                    println!("  Description: {}", status.description);
                    println!("  IOMMU Group: {}", status.iommu_group);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

/// VFIO setup wizard
pub fn vfio_setup_wizard() -> Result<()> {
    use dialoguer::Confirm;

    println!("\nVFIO Setup Wizard");
    println!("{}", "=".repeat(50));
    println!("\nThis wizard will help you configure VFIO for GPU passthrough.");

    // Step 1: Check IOMMU status
    println!("\n[Step 1/5] Checking IOMMU status...");
    match crate::iommu::get_iommu_status() {
        Ok(status) => {
            if !status.enabled {
                println!("\nIOMMU is NOT enabled!");
                println!("{}", get_kernel_params_recommendation());
                println!("\nPlease enable IOMMU and reboot before continuing.");
                return Ok(());
            }
            println!("  IOMMU is enabled ({}).", status.mode);
            if !status.passthrough_pt {
                println!("  Recommendation: Add 'iommu=pt' for better performance.");
            }
        }
        Err(e) => {
            eprintln!("Warning: Could not check IOMMU status: {}", e);
        }
    }

    // Step 2: Select GPU for passthrough
    println!("\n[Step 2/5] Selecting GPU for passthrough...");
    let gpus = match crate::iommu::list_gpus() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error listing GPUs: {}", e);
            return Ok(());
        }
    };

    if gpus.is_empty() {
        println!("No GPUs found for passthrough.");
        return Ok(());
    }

    println!("\nAvailable GPUs:");
    let gpu_options: Vec<String> = gpus
        .iter()
        .map(|g| {
            let driver = g
                .current_driver
                .as_ref()
                .map(|d| format!(" [{}]", d))
                .unwrap_or_default();
            format!(
                "{} - {} (Group {}){}",
                g.address, g.description, g.iommu_group, driver
            )
        })
        .collect();

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select GPU to pass through")
        .items(&gpu_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(sel)) => sel,
        Ok(None) | Err(_) => return Ok(()),
    };

    let selected_gpu = &gpus[selection];
    println!(
        "\nSelected: {} - {}",
        selected_gpu.address, selected_gpu.description
    );

    // Step 3: Find related devices (audio function, etc.)
    println!(
        "\n[Step 3/5] Finding related devices in IOMMU group {}...",
        selected_gpu.iommu_group
    );
    let group = match crate::iommu::get_iommu_group(selected_gpu.iommu_group) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error getting IOMMU group: {}", e);
            return Ok(());
        }
    };

    let mut devices_to_configure: Vec<VfioDeviceConfig> = Vec::new();

    for device in &group.devices {
        // Include GPU and related devices on the same bus (audio, etc.)
        let include = device.is_gpu() || same_pci_bus(&device.address, &selected_gpu.address);

        if include {
            println!("  {} - {}", device.address, device.description);
            devices_to_configure.push(VfioDeviceConfig {
                pci_address: device.address.clone(),
                vendor_id: device.vendor_id.clone(),
                device_id: device.device_id.clone(),
                description: device.description.clone(),
            });
        }
    }

    if devices_to_configure.is_empty() {
        println!("No devices to configure.");
        return Ok(());
    }

    // Step 4: Generate configuration
    println!("\n[Step 4/5] Configuration preview...");

    let config = VfioConfig {
        devices: devices_to_configure,
        unsafe_interrupts: false,
        acs_override: false,
    };

    println!("\nmodprobe.d configuration:");
    println!("{}", "-".repeat(40));
    println!("{}", generate_modprobe_config(&config));
    println!("{}", "-".repeat(40));

    let initramfs = detect_initramfs_system();
    println!("\nDetected initramfs system: {}", initramfs);

    // Step 5: Apply configuration
    println!("\n[Step 5/5] Apply configuration");

    let apply = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Write configuration files? (requires root)")
        .default(false)
        .interact_opt()
    {
        Ok(Some(v)) => v,
        Ok(None) | Err(_) => false,
    };

    if apply {
        // Write modprobe config
        match write_modprobe_config(&config) {
            Ok(_) => println!("modprobe.d configuration written."),
            Err(e) => eprintln!("Error writing modprobe config: {}", e),
        }

        // Write initramfs config
        match write_initramfs_config(initramfs) {
            Ok(_) => {}
            Err(e) => eprintln!("Error writing initramfs config: {}", e),
        }

        // Offer to regenerate initramfs
        let regenerate = match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Regenerate initramfs now?")
            .default(false)
            .interact_opt()
        {
            Ok(Some(v)) => v,
            Ok(None) | Err(_) => false,
        };

        if regenerate {
            match config::regenerate_initramfs(initramfs) {
                Ok(_) => println!("Initramfs regenerated."),
                Err(e) => eprintln!("Error regenerating initramfs: {}", e),
            }
        }

        println!("\nConfiguration complete!");
        println!("Please reboot for changes to take effect.");
    } else {
        println!("\nConfiguration not applied.");
        println!("You can manually create the configuration files shown above.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify that main types are accessible
        let _: fn() -> VfioResult<()> = || Ok(());
    }
}
