use crate::utils::run_command;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDevice {
    pub pci_address: String,
    pub vendor_id: String,
    pub device_id: String,
    pub description: String,
    pub iommu_group: Option<String>,
    pub current_driver: Option<String>,
    pub audio_function: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VfioConfig {
    pub gpu_address: String,
    pub vendor_device_ids: Vec<String>,
    pub persist: bool,
}

const VFIO_CONF_PATH: &str = "/etc/modprobe.d/vfio.conf";
const BLACKLIST_NOUVEAU_PATH: &str = "/etc/modprobe.d/blacklist-nouveau.conf";
const BLACKLIST_AMD_PATH: &str = "/etc/modprobe.d/blacklist-amdgpu.conf";
const GRUB_CONFIG_PATH: &str = "/etc/default/grub";
const MARKER: &str = "# ghostctl-managed";

pub fn vfio_menu() {
    loop {
        let options = vec![
            "Enable VFIO for GPU",
            "Disable VFIO",
            "Rescue Mode (Restore Console)",
            "Runtime Bind/Unbind GPU",
            "NVIDIA Passthrough Setup",
            "AMD Passthrough Setup",
            "Status & Diagnostics",
            "Back",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🎮 PVE VFIO Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => enable_vfio(),
            1 => disable_vfio(),
            2 => rescue_mode(),
            3 => runtime_bind_menu(),
            4 => nvidia_passthrough_menu(),
            5 => amd_passthrough_menu(),
            6 => diagnostics_menu(),
            _ => break,
        }
    }
}

fn enable_vfio() {
    println!("🔍 Detecting GPUs...");
    let gpus = detect_gpus();
    
    if gpus.is_empty() {
        println!("❌ No GPUs detected!");
        return;
    }

    let gpu_strings: Vec<String> = gpus
        .iter()
        .map(|g| format!("{} - {} ({})", g.pci_address, g.description, g.current_driver.as_ref().unwrap_or(&"no driver".to_string())))
        .collect();

    let gpu_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select GPU for passthrough")
        .items(&gpu_strings)
        .interact()
        .unwrap();

    let selected_gpu = &gpus[gpu_idx];
    
    let mut ids = vec![format!("{}:{}", selected_gpu.vendor_id, selected_gpu.device_id)];
    
    if let Some(audio) = &selected_gpu.audio_function {
        println!("🔊 Audio function detected at {}", audio);
        if let Some(audio_device) = detect_device_at_address(audio) {
            ids.push(format!("{}:{}", audio_device.vendor_id, audio_device.device_id));
        }
    }

    let persist = Confirm::new()
        .with_prompt("Make changes persistent? (requires reboot)")
        .default(true)
        .interact()
        .unwrap();

    println!("📝 Configuring VFIO with IDs: {}", ids.join(","));
    
    if let Err(e) = configure_vfio(&ids, persist) {
        println!("❌ Failed to configure VFIO: {}", e);
        return;
    }

    if persist {
        println!("✅ VFIO configured! System will reboot for changes to take effect.");
        if Confirm::new()
            .with_prompt("Reboot now?")
            .default(false)
            .interact()
            .unwrap()
        {
            let _ = Command::new("systemctl").arg("reboot").status();
        }
    } else {
        println!("✅ VFIO configured for runtime only.");
    }
}

fn disable_vfio() {
    let persist = Confirm::new()
        .with_prompt("Remove persistent VFIO configuration?")
        .default(true)
        .interact()
        .unwrap();

    println!("🔄 Removing VFIO configuration...");
    
    if Path::new(VFIO_CONF_PATH).exists() {
        fs::remove_file(VFIO_CONF_PATH).ok();
        println!("✅ Removed {}", VFIO_CONF_PATH);
    }

    remove_grub_vfio_options();
    
    if persist {
        rebuild_initramfs();
        refresh_boot();
        
        println!("✅ VFIO disabled! Reboot required for changes to take effect.");
        if Confirm::new()
            .with_prompt("Reboot now?")
            .default(false)
            .interact()
            .unwrap()
        {
            let _ = Command::new("systemctl").arg("reboot").status();
        }
    }
}

fn rescue_mode() {
    println!("🚨 RESCUE MODE - Restoring console visibility");
    println!("This will:");
    println!("  • Force console boot flags");
    println!("  • Remove GPU blacklists");
    println!("  • Remove VFIO bindings");
    
    if !Confirm::new()
        .with_prompt("Continue with rescue mode?")
        .default(true)
        .interact()
        .unwrap()
    {
        return;
    }

    // Add console visibility flags
    add_grub_console_flags();
    
    // Remove VFIO and blacklists
    for path in &[VFIO_CONF_PATH, BLACKLIST_NOUVEAU_PATH, BLACKLIST_AMD_PATH] {
        if Path::new(path).exists() {
            fs::remove_file(path).ok();
            println!("✅ Removed {}", path);
        }
    }
    
    rebuild_initramfs();
    refresh_boot();
    
    println!("✅ Rescue mode configured! Your console should be visible after reboot.");
    println!("⚠️  IMPORTANT: Reboot required!");
    
    if Confirm::new()
        .with_prompt("Reboot now?")
        .default(true)
        .interact()
        .unwrap()
    {
        let _ = Command::new("systemctl").arg("reboot").status();
    }
}

fn runtime_bind_menu() {
    println!("🔗 Runtime GPU Bind/Unbind");
    
    let options = vec![
        "Unbind GPU from current driver",
        "Bind GPU to vfio-pci",
        "Show current bindings",
        "Back",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select operation")
        .items(&options)
        .interact()
        .unwrap();

    match selection {
        0 => runtime_unbind(),
        1 => runtime_bind(),
        2 => show_bindings(),
        _ => {}
    }
}

fn runtime_unbind() {
    let gpus = detect_gpus();
    if gpus.is_empty() {
        println!("❌ No GPUs detected!");
        return;
    }

    let gpu_strings: Vec<String> = gpus
        .iter()
        .map(|g| format!("{} - {} ({})", g.pci_address, g.description, g.current_driver.as_ref().unwrap_or(&"unbound".to_string())))
        .collect();

    let gpu_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select GPU to unbind")
        .items(&gpu_strings)
        .interact()
        .unwrap();

    let gpu = &gpus[gpu_idx];
    
    if gpu.current_driver.is_none() {
        println!("⚠️  GPU is not bound to any driver");
        return;
    }

    let unbind_path = format!("/sys/bus/pci/devices/{}/driver/unbind", gpu.pci_address);
    
    match fs::write(&unbind_path, &gpu.pci_address) {
        Ok(_) => println!("✅ GPU {} unbound from driver", gpu.pci_address),
        Err(e) => println!("❌ Failed to unbind GPU: {}", e),
    }
}

fn runtime_bind() {
    let gpus = detect_gpus();
    if gpus.is_empty() {
        println!("❌ No GPUs detected!");
        return;
    }

    let gpu_strings: Vec<String> = gpus
        .iter()
        .map(|g| format!("{} - {} ({})", g.pci_address, g.description, g.current_driver.as_ref().unwrap_or(&"unbound".to_string())))
        .collect();

    let gpu_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select GPU to bind to vfio-pci")
        .items(&gpu_strings)
        .interact()
        .unwrap();

    let gpu = &gpus[gpu_idx];
    
    // First ensure vfio-pci knows about this device
    let new_id_path = "/sys/bus/pci/drivers/vfio-pci/new_id";
    let vendor_device = format!("{} {}", gpu.vendor_id.replace("0x", ""), gpu.device_id.replace("0x", ""));
    
    match fs::write(new_id_path, &vendor_device) {
        Ok(_) => println!("✅ Registered device with vfio-pci"),
        Err(e) => println!("⚠️  Could not register with vfio-pci: {}", e),
    }
    
    // Now bind it
    let bind_path = "/sys/bus/pci/drivers/vfio-pci/bind";
    
    match fs::write(bind_path, &gpu.pci_address) {
        Ok(_) => println!("✅ GPU {} bound to vfio-pci", gpu.pci_address),
        Err(e) => println!("❌ Failed to bind GPU: {}", e),
    }
}

fn show_bindings() {
    println!("\n🔍 Current GPU Bindings:");
    let gpus = detect_gpus();
    
    for gpu in gpus {
        println!("\n📍 {}: {}", gpu.pci_address, gpu.description);
        println!("   Vendor:Device = {}:{}", gpu.vendor_id, gpu.device_id);
        println!("   Driver = {}", gpu.current_driver.unwrap_or_else(|| "unbound".to_string()));
        if let Some(group) = gpu.iommu_group {
            println!("   IOMMU Group = {}", group);
        }
    }
}

fn nvidia_passthrough_menu() {
    loop {
        let options = vec![
            "NVIDIA Prepare (Setup for Passthrough)",
            "NVIDIA Clean (Remove Settings)",
            "Show NVIDIA Configuration Hints",
            "Back",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🟢 NVIDIA Passthrough Helper")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => nvidia_prepare(),
            1 => nvidia_clean(),
            2 => show_nvidia_hints(),
            _ => break,
        }
    }
}

fn nvidia_prepare() {
    println!("🟢 NVIDIA Passthrough Preparation");
    println!("This will:");
    println!("  • Blacklist nouveau driver");
    println!("  • Configure VFIO for NVIDIA GPU");
    println!("  • Add recommended KVM flags");
    println!("  • Detect and include audio function");
    
    let gpus = detect_gpus();
    let nvidia_gpus: Vec<&GpuDevice> = gpus
        .iter()
        .filter(|g| g.vendor_id == "0x10de" || g.description.to_lowercase().contains("nvidia"))
        .collect();
    
    if nvidia_gpus.is_empty() {
        println!("❌ No NVIDIA GPUs detected!");
        return;
    }

    let gpu_strings: Vec<String> = nvidia_gpus
        .iter()
        .map(|g| format!("{} - {}", g.pci_address, g.description))
        .collect();

    let gpu_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select NVIDIA GPU")
        .items(&gpu_strings)
        .interact()
        .unwrap();

    let gpu = nvidia_gpus[gpu_idx];
    
    // Blacklist nouveau
    let blacklist_content = format!("{}\nblacklist nouveau\noptions nouveau modeset=0\n", MARKER);
    fs::write(BLACKLIST_NOUVEAU_PATH, blacklist_content).expect("Failed to write nouveau blacklist");
    println!("✅ Blacklisted nouveau driver");
    
    // Setup VFIO with GPU and audio
    let mut ids = vec![format!("{}:{}", gpu.vendor_id, gpu.device_id)];
    
    if let Some(audio_addr) = &gpu.audio_function {
        if let Some(audio) = detect_device_at_address(audio_addr) {
            ids.push(format!("{}:{}", audio.vendor_id, audio.device_id));
            println!("🔊 Including audio function: {}", audio_addr);
        }
    }
    
    configure_vfio(&ids, true).expect("Failed to configure VFIO");
    
    // Add NVIDIA-specific GRUB flags
    add_nvidia_grub_flags();
    
    rebuild_initramfs();
    refresh_boot();
    
    println!("\n✅ NVIDIA GPU prepared for passthrough!");
    show_nvidia_vm_config(gpu);
    
    if Confirm::new()
        .with_prompt("Reboot now to apply changes?")
        .default(true)
        .interact()
        .unwrap()
    {
        let _ = Command::new("systemctl").arg("reboot").status();
    }
}

fn nvidia_clean() {
    println!("🧹 Cleaning NVIDIA passthrough configuration...");
    
    if Path::new(BLACKLIST_NOUVEAU_PATH).exists() {
        fs::remove_file(BLACKLIST_NOUVEAU_PATH).ok();
        println!("✅ Removed nouveau blacklist");
    }
    
    remove_nvidia_grub_flags();
    
    rebuild_initramfs();
    refresh_boot();
    
    println!("✅ NVIDIA configuration cleaned!");
}

fn show_nvidia_hints() {
    println!("\n📋 NVIDIA Passthrough VM Configuration:");
    println!("\n1. VM Settings:");
    println!("   • BIOS: OVMF (UEFI)");
    println!("   • Machine: q35");
    println!("   • CPU: host");
    println!("\n2. PCI Device (example for GPU at 0a:00):");
    println!("   hostpci0: 0000:0a:00,pcie=1,x-vga=1,rombar=0");
    println!("\n3. For GPU with audio (0a:00.0 + 0a:00.1):");
    println!("   hostpci0: 0000:0a:00,pcie=1,x-vga=1,rombar=0");
    println!("\n4. Hide KVM from NVIDIA driver (avoid Code 43):");
    println!("   args: -cpu host,kvm=off");
    println!("   OR in newer Proxmox:");
    println!("   hostpci0: 0000:0a:00,pcie=1,x-vga=1,rombar=0,hidden=1");
    println!("\n5. Optional ROM file:");
    println!("   romfile=<path-to-vbios.rom>");
    println!("\n⚠️  Windows: Install NVIDIA drivers AFTER VM boots successfully");
}

fn show_nvidia_vm_config(gpu: &GpuDevice) {
    println!("\n📋 Proxmox VM Configuration for your GPU:");
    println!("\n# Add to VM config (/etc/pve/qemu-server/<vmid>.conf):");
    println!("bios: ovmf");
    println!("machine: q35");
    println!("cpu: host");
    
    if gpu.audio_function.is_some() {
        println!("hostpci0: {},pcie=1,x-vga=1,rombar=0", gpu.pci_address.replace('.', ":"));
    } else {
        println!("hostpci0: {},pcie=1,x-vga=1,rombar=0", gpu.pci_address);
    }
    
    println!("args: -cpu host,kvm=off");
    println!("\n# Alternative for newer Proxmox (7.2+):");
    println!("hostpci0: {},pcie=1,x-vga=1,rombar=0,hidden=1", gpu.pci_address);
}

fn amd_passthrough_menu() {
    loop {
        let options = vec![
            "AMD Prepare (Setup for Passthrough)",
            "AMD Clean (Remove Settings)",
            "Setup Vendor Reset (Fix Reset Bug)",
            "Show AMD Configuration Hints",
            "Back",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔴 AMD Passthrough Helper")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => amd_prepare(),
            1 => amd_clean(),
            2 => setup_vendor_reset(),
            3 => show_amd_hints(),
            _ => break,
        }
    }
}

fn amd_prepare() {
    println!("🔴 AMD GPU Passthrough Preparation");
    
    let gpus = detect_gpus();
    let amd_gpus: Vec<&GpuDevice> = gpus
        .iter()
        .filter(|g| g.vendor_id == "0x1002" || g.description.to_lowercase().contains("amd") || g.description.to_lowercase().contains("radeon"))
        .collect();
    
    if amd_gpus.is_empty() {
        println!("❌ No AMD GPUs detected!");
        return;
    }

    let gpu_strings: Vec<String> = amd_gpus
        .iter()
        .map(|g| format!("{} - {}", g.pci_address, g.description))
        .collect();

    let gpu_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select AMD GPU")
        .items(&gpu_strings)
        .interact()
        .unwrap();

    let gpu = amd_gpus[gpu_idx];
    
    // Blacklist amdgpu
    let blacklist_content = format!("{}\nblacklist amdgpu\nblacklist radeon\n", MARKER);
    fs::write(BLACKLIST_AMD_PATH, blacklist_content).expect("Failed to write AMD blacklist");
    println!("✅ Blacklisted amdgpu/radeon drivers");
    
    // Setup VFIO
    let mut ids = vec![format!("{}:{}", gpu.vendor_id, gpu.device_id)];
    
    if let Some(audio_addr) = &gpu.audio_function {
        if let Some(audio) = detect_device_at_address(audio_addr) {
            ids.push(format!("{}:{}", audio.vendor_id, audio.device_id));
            println!("🔊 Including audio function: {}", audio_addr);
        }
    }
    
    configure_vfio(&ids, true).expect("Failed to configure VFIO");
    
    rebuild_initramfs();
    refresh_boot();
    
    println!("\n✅ AMD GPU prepared for passthrough!");
    show_amd_vm_config(gpu);
    
    if Confirm::new()
        .with_prompt("Reboot now to apply changes?")
        .default(true)
        .interact()
        .unwrap()
    {
        let _ = Command::new("systemctl").arg("reboot").status();
    }
}

fn amd_clean() {
    println!("🧹 Cleaning AMD passthrough configuration...");
    
    if Path::new(BLACKLIST_AMD_PATH).exists() {
        fs::remove_file(BLACKLIST_AMD_PATH).ok();
        println!("✅ Removed AMD driver blacklist");
    }
    
    rebuild_initramfs();
    refresh_boot();
    
    println!("✅ AMD configuration cleaned!");
}

fn show_amd_hints() {
    println!("\n📋 AMD Passthrough VM Configuration:");
    println!("\n1. VM Settings:");
    println!("   • BIOS: OVMF (UEFI) or SeaBIOS");
    println!("   • Machine: q35");
    println!("   • CPU: host");
    println!("\n2. PCI Device (example for GPU at 0d:00):");
    println!("   hostpci0: 0000:0d:00,pcie=1,x-vga=1");
    println!("\n3. For GPU with audio:");
    println!("   hostpci0: 0000:0d:00,pcie=1,x-vga=1");
    println!("\n4. Reset bug workaround (if needed):");
    println!("   echo 1 > /sys/bus/pci/devices/0000:0d:00.0/remove");
    println!("   echo 1 > /sys/bus/pci/rescan");
}

fn show_amd_vm_config(gpu: &GpuDevice) {
    println!("\n📋 Proxmox VM Configuration for your GPU:");
    println!("\n# Add to VM config:");
    println!("bios: ovmf");
    println!("machine: q35");
    println!("cpu: host");
    println!("hostpci0: {},pcie=1,x-vga=1", gpu.pci_address);
}

fn diagnostics_menu() {
    loop {
        let options = vec![
            "Check IOMMU Status",
            "Show IOMMU Groups",
            "List GPU Mappings",
            "Check Driver Conflicts",
            "Show GRUB Configuration",
            "Back",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔍 VFIO Diagnostics")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => check_iommu_status(),
            1 => show_iommu_groups(),
            2 => list_gpu_mappings(),
            3 => check_driver_conflicts(),
            4 => show_grub_config(),
            _ => break,
        }
    }
}

fn check_iommu_status() {
    println!("\n🔍 Checking IOMMU Status...\n");
    
    // Check kernel cmdline
    if let Ok(cmdline) = fs::read_to_string("/proc/cmdline") {
        println!("📋 Kernel Command Line:");
        println!("{}", cmdline);
        
        let iommu_enabled = cmdline.contains("intel_iommu=on") || cmdline.contains("amd_iommu=on");
        if iommu_enabled {
            println!("✅ IOMMU is enabled in kernel parameters");
        } else {
            println!("❌ IOMMU is NOT enabled in kernel parameters");
        }
    }
    
    // Check dmesg for IOMMU
    let output = Command::new("dmesg")
        .arg("|")
        .arg("grep")
        .arg("-i")
        .arg("iommu")
        .output();
    
    if let Ok(output) = output {
        let dmesg_out = String::from_utf8_lossy(&output.stdout);
        if !dmesg_out.is_empty() {
            println!("\n📋 IOMMU Messages from dmesg:");
            for line in dmesg_out.lines().take(10) {
                println!("  {}", line);
            }
        }
    }
}

fn show_iommu_groups() {
    println!("\n🔍 IOMMU Groups:\n");
    
    let iommu_path = Path::new("/sys/kernel/iommu_groups");
    if !iommu_path.exists() {
        println!("❌ IOMMU groups not found! IOMMU may not be enabled.");
        return;
    }
    
    let mut groups = Vec::new();
    
    if let Ok(entries) = fs::read_dir(iommu_path) {
        for entry in entries.flatten() {
            if let Some(group_num) = entry.file_name().to_str() {
                let devices_path = entry.path().join("devices");
                if let Ok(devices) = fs::read_dir(devices_path) {
                    let mut group_devices = Vec::new();
                    for device in devices.flatten() {
                        if let Some(device_name) = device.file_name().to_str() {
                            group_devices.push(device_name.to_string());
                        }
                    }
                    if !group_devices.is_empty() {
                        groups.push((group_num.to_string(), group_devices));
                    }
                }
            }
        }
    }
    
    groups.sort_by(|a, b| a.0.parse::<u32>().unwrap_or(0).cmp(&b.0.parse::<u32>().unwrap_or(0)));
    
    for (group, devices) in groups {
        println!("Group {}: {}", group, devices.join(", "));
    }
}

fn list_gpu_mappings() {
    println!("\n🎮 GPU Device Mappings:\n");
    
    let output = Command::new("lspci")
        .args(&["-nnk", "-d", "::0300"])
        .output();
    
    if let Ok(output) = output {
        println!("VGA Controllers:");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    
    let output = Command::new("lspci")
        .args(&["-nnk", "-d", "::0302"])
        .output();
    
    if let Ok(output) = output {
        println!("\n3D Controllers:");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn check_driver_conflicts() {
    println!("\n🔍 Checking for driver conflicts...\n");
    
    let drivers = ["nvidia", "nouveau", "amdgpu", "radeon", "vfio-pci"];
    
    for driver in &drivers {
        let output = Command::new("lsmod")
            .arg("|")
            .arg("grep")
            .arg(driver)
            .output();
        
        if let Ok(output) = output {
            if !output.stdout.is_empty() {
                println!("✅ {} module is loaded", driver);
            }
        }
    }
    
    println!("\n📋 GPU Driver Bindings:");
    let gpus = detect_gpus();
    for gpu in gpus {
        println!("  {} - {} => {}", 
            gpu.pci_address, 
            gpu.description,
            gpu.current_driver.unwrap_or_else(|| "no driver".to_string())
        );
    }
}

fn show_grub_config() {
    println!("\n📋 GRUB Configuration:\n");
    
    if let Ok(content) = fs::read_to_string(GRUB_CONFIG_PATH) {
        for line in content.lines() {
            if line.contains("GRUB_CMDLINE_LINUX") {
                println!("{}", line);
            }
        }
    }
}

// Helper functions

fn detect_gpus() -> Vec<GpuDevice> {
    let mut gpus = Vec::new();
    
    // Detect VGA controllers
    if let Ok(output) = Command::new("lspci")
        .args(&["-nn", "-d", "::0300"])
        .output() {
        parse_lspci_output(&String::from_utf8_lossy(&output.stdout), &mut gpus);
    }
    
    // Detect 3D controllers
    if let Ok(output) = Command::new("lspci")
        .args(&["-nn", "-d", "::0302"])
        .output() {
        parse_lspci_output(&String::from_utf8_lossy(&output.stdout), &mut gpus);
    }
    
    // Get additional details for each GPU
    for gpu in &mut gpus {
        // Get current driver
        if let Ok(driver_link) = fs::read_link(format!("/sys/bus/pci/devices/{}/driver", gpu.pci_address)) {
            if let Some(driver_name) = driver_link.file_name() {
                gpu.current_driver = Some(driver_name.to_string_lossy().to_string());
            }
        }
        
        // Get IOMMU group
        if let Ok(group_link) = fs::read_link(format!("/sys/bus/pci/devices/{}/iommu_group", gpu.pci_address)) {
            if let Some(group_name) = group_link.file_name() {
                gpu.iommu_group = Some(group_name.to_string_lossy().to_string());
            }
        }
        
        // Check for audio function (typically .1)
        let base_addr = gpu.pci_address.split('.').next().unwrap_or(&gpu.pci_address);
        let audio_addr = format!("{}.1", base_addr);
        if Path::new(&format!("/sys/bus/pci/devices/{}", audio_addr)).exists() {
            gpu.audio_function = Some(audio_addr);
        }
    }
    
    gpus
}

fn parse_lspci_output(output: &str, gpus: &mut Vec<GpuDevice>) {
    for line in output.lines() {
        if let Some((addr, rest)) = line.split_once(' ') {
            let pci_address = format!("0000:{}", addr);
            
            // Extract vendor:device IDs
            let (vendor_id, device_id) = if let Some(ids_start) = rest.rfind('[') {
                if let Some(ids_end) = rest.rfind(']') {
                    let ids = &rest[ids_start + 1..ids_end];
                    if let Some((v, d)) = ids.split_once(':') {
                        (format!("0x{}", v), format!("0x{}", d))
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            };
            
            // Extract description
            let description = if let Some(desc_start) = rest.find(": ") {
                let desc = &rest[desc_start + 2..];
                if let Some(ids_start) = desc.rfind('[') {
                    desc[..ids_start].trim().to_string()
                } else {
                    desc.trim().to_string()
                }
            } else {
                "Unknown GPU".to_string()
            };
            
            gpus.push(GpuDevice {
                pci_address,
                vendor_id,
                device_id,
                description,
                iommu_group: None,
                current_driver: None,
                audio_function: None,
            });
        }
    }
}

fn detect_device_at_address(address: &str) -> Option<GpuDevice> {
    let output = Command::new("lspci")
        .args(&["-nns", address])
        .output()
        .ok()?;
    
    let line = String::from_utf8_lossy(&output.stdout);
    let line = line.trim();
    
    if line.is_empty() {
        return None;
    }
    
    // Parse the single line output
    let mut gpus = Vec::new();
    parse_lspci_output(&line, &mut gpus);
    gpus.into_iter().next()
}

fn configure_vfio(ids: &[String], persist: bool) -> Result<(), String> {
    // Write VFIO configuration
    let vfio_content = format!(
        "{}\nblacklist amdgpu\nblacklist nouveau\nblacklist radeon\noptions vfio-pci ids={}\noptions vfio-pci disable_vga=1\n",
        MARKER,
        ids.join(",")
    );
    
    fs::write(VFIO_CONF_PATH, vfio_content)
        .map_err(|e| format!("Failed to write VFIO config: {}", e))?;
    
    // Update initramfs modules
    ensure_initramfs_modules();
    
    // Update GRUB for IOMMU
    ensure_iommu_grub();
    
    if persist {
        rebuild_initramfs();
        refresh_boot();
    }
    
    Ok(())
}

fn ensure_initramfs_modules() {
    let modules_path = "/etc/modules";
    if let Ok(content) = fs::read_to_string(modules_path) {
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        for module in &["vfio", "vfio_iommu_type1", "vfio_pci", "vfio_virqfd"] {
            if !lines.iter().any(|l| l == module) {
                lines.push(module.to_string());
            }
        }
        
        let _ = fs::write(modules_path, lines.join("\n"));
    }
}

fn ensure_iommu_grub() {
    if let Ok(content) = fs::read_to_string(GRUB_CONFIG_PATH) {
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        for (i, line) in lines.iter_mut().enumerate() {
            if line.starts_with("GRUB_CMDLINE_LINUX_DEFAULT=") {
                let cpu_vendor = detect_cpu_vendor();
                let iommu_flag = if cpu_vendor == "Intel" {
                    "intel_iommu=on"
                } else {
                    "amd_iommu=on"
                };
                
                if !line.contains(iommu_flag) {
                    // Add IOMMU flags
                    if let Some(end_quote) = line.rfind('"') {
                        let new_line = format!("{} {} iommu=pt\"", &line[..end_quote], iommu_flag);
                        lines[i] = new_line;
                    }
                }
                break;
            }
        }
        
        let _ = fs::write(GRUB_CONFIG_PATH, lines.join("\n"));
        update_grub();
    }
}

fn add_grub_console_flags() {
    if let Ok(content) = fs::read_to_string(GRUB_CONFIG_PATH) {
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        for (i, line) in lines.iter_mut().enumerate() {
            if line.starts_with("GRUB_CMDLINE_LINUX_DEFAULT=") {
                let console_flags = "video=efifb:force fbcon=map:1 console=tty1";
                
                if !line.contains("video=efifb") {
                    if let Some(end_quote) = line.rfind('"') {
                        let new_line = format!("{} {}\"", &line[..end_quote], console_flags);
                        lines[i] = new_line;
                    }
                }
                break;
            }
        }
        
        let _ = fs::write(GRUB_CONFIG_PATH, lines.join("\n"));
        update_grub();
    }
}

fn add_nvidia_grub_flags() {
    if let Ok(content) = fs::read_to_string(GRUB_CONFIG_PATH) {
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        for (i, line) in lines.iter_mut().enumerate() {
            if line.starts_with("GRUB_CMDLINE_LINUX_DEFAULT=") {
                let nvidia_flags = "kvm.ignore_msrs=1";
                
                if !line.contains("kvm.ignore_msrs") {
                    if let Some(end_quote) = line.rfind('"') {
                        let new_line = format!("{} {}\"", &line[..end_quote], nvidia_flags);
                        lines[i] = new_line;
                    }
                }
                break;
            }
        }
        
        let _ = fs::write(GRUB_CONFIG_PATH, lines.join("\n"));
        update_grub();
    }
}

fn remove_grub_vfio_options() {
    if let Ok(content) = fs::read_to_string(GRUB_CONFIG_PATH) {
        let lines: Vec<String> = content.lines().map(|s| {
            if s.starts_with("GRUB_CMDLINE_LINUX_DEFAULT=") {
                // Remove VFIO-related options
                s.replace(" intel_iommu=on", "")
                 .replace(" amd_iommu=on", "")
                 .replace(" iommu=pt", "")
                 .replace(" video=efifb:force", "")
                 .replace(" fbcon=map:1", "")
                 .replace(" console=tty1", "")
            } else {
                s.to_string()
            }
        }).collect();
        
        let _ = fs::write(GRUB_CONFIG_PATH, lines.join("\n"));
        update_grub();
    }
}

fn remove_nvidia_grub_flags() {
    if let Ok(content) = fs::read_to_string(GRUB_CONFIG_PATH) {
        let lines: Vec<String> = content.lines().map(|s| {
            if s.starts_with("GRUB_CMDLINE_LINUX_DEFAULT=") {
                s.replace(" kvm.ignore_msrs=1", "")
                 .replace(" pci=noaer", "")
            } else {
                s.to_string()
            }
        }).collect();
        
        let _ = fs::write(GRUB_CONFIG_PATH, lines.join("\n"));
        update_grub();
    }
}

fn detect_cpu_vendor() -> &'static str {
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        if cpuinfo.contains("GenuineIntel") {
            return "Intel";
        } else if cpuinfo.contains("AuthenticAMD") {
            return "AMD";
        }
    }
    "Unknown"
}

fn rebuild_initramfs() {
    println!("🔄 Rebuilding initramfs...");
    let _ = Command::new("update-initramfs").args(&["-u", "-k", "all"]).status();
}

fn refresh_boot() {
    println!("🔄 Refreshing boot configuration...");
    let _ = Command::new("proxmox-boot-tool").arg("refresh").status();
}

fn update_grub() {
    let _ = Command::new("update-grub").status();
}

fn setup_vendor_reset() {
    println!("🔧 AMD Vendor Reset Setup");
    println!("This addresses the AMD GPU reset bug that prevents proper VM restart.");
    println!("");
    
    if !Confirm::new()
        .with_prompt("Install vendor-reset kernel module?")
        .default(true)
        .interact()
        .unwrap()
    {
        return;
    }
    
    // Check if DKMS is available
    if !Path::new("/usr/bin/dkms").exists() {
        println!("📦 Installing DKMS...");
        let _ = Command::new("apt")
            .args(&["update", "&&", "apt", "install", "-y", "dkms", "build-essential", "linux-headers-$(uname -r)"])
            .status();
    }
    
    // Check if vendor-reset is already installed
    let vendor_reset_path = "/usr/src/vendor-reset-0.1.1";
    if Path::new(vendor_reset_path).exists() {
        println!("ℹ️  Vendor-reset already installed");
        
        if Confirm::new()
            .with_prompt("Reinstall vendor-reset?")
            .default(false)
            .interact()
            .unwrap()
        {
            uninstall_vendor_reset();
        } else {
            configure_vendor_reset_usage();
            return;
        }
    }
    
    install_vendor_reset();
    configure_vendor_reset_usage();
}

fn install_vendor_reset() {
    println!("📥 Installing vendor-reset...");
    
    // Create temporary directory
    let temp_dir = "/tmp/vendor-reset-install";
    let _ = fs::create_dir_all(temp_dir);
    
    // Download vendor-reset
    let git_url = "https://github.com/gnif/vendor-reset.git";
    
    println!("📥 Cloning vendor-reset repository...");
    let clone_result = Command::new("git")
        .args(&["clone", git_url, temp_dir])
        .status();
    
    if !clone_result.map(|s| s.success()).unwrap_or(false) {
        println!("❌ Failed to clone repository. Trying alternative method...");
        download_vendor_reset_tarball(temp_dir);
    }
    
    // Build and install
    println!("🔨 Building vendor-reset module...");
    
    let build_result = Command::new("make")
        .current_dir(temp_dir)
        .status();
    
    if !build_result.map(|s| s.success()).unwrap_or(false) {
        println!("❌ Build failed! Check build dependencies.");
        return;
    }
    
    // Install with DKMS
    println!("📦 Installing with DKMS...");
    
    // Copy to DKMS source directory
    let dkms_dir = "/usr/src/vendor-reset-0.1.1";
    let _ = fs::create_dir_all(&dkms_dir);
    
    let _ = Command::new("cp")
        .args(&["-r", &format!("{}/*", temp_dir), &dkms_dir])
        .status();
    
    // Create dkms.conf
    let dkms_conf = r#"PACKAGE_NAME="vendor-reset"
PACKAGE_VERSION="0.1.1"
BUILT_MODULE_NAME[0]="vendor-reset"
DEST_MODULE_LOCATION[0]="/updates/dkms/"
AUTOINSTALL="yes"
"#;
    
    fs::write(format!("{}/dkms.conf", dkms_dir), dkms_conf).ok();
    
    // Add to DKMS
    let _ = Command::new("dkms")
        .args(&["add", "-m", "vendor-reset", "-v", "0.1.1"])
        .status();
    
    // Build with DKMS
    let _ = Command::new("dkms")
        .args(&["build", "-m", "vendor-reset", "-v", "0.1.1"])
        .status();
    
    // Install with DKMS
    let install_result = Command::new("dkms")
        .args(&["install", "-m", "vendor-reset", "-v", "0.1.1"])
        .status();
    
    if install_result.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Vendor-reset installed successfully");
    } else {
        println!("❌ DKMS install failed. Trying manual install...");
        manual_install_vendor_reset(temp_dir);
    }
    
    // Cleanup
    let _ = fs::remove_dir_all(temp_dir);
}

fn download_vendor_reset_tarball(temp_dir: &str) {
    println!("📥 Downloading vendor-reset tarball...");
    
    let tarball_url = "https://github.com/gnif/vendor-reset/archive/refs/heads/master.tar.gz";
    
    let _ = Command::new("wget")
        .args(&["-O", &format!("{}/vendor-reset.tar.gz", temp_dir), tarball_url])
        .status();
    
    let _ = Command::new("tar")
        .args(&["-xzf", &format!("{}/vendor-reset.tar.gz", temp_dir), "-C", temp_dir, "--strip-components=1"])
        .status();
}

fn manual_install_vendor_reset(temp_dir: &str) {
    println!("🔨 Manual installation...");
    
    let _ = Command::new("make")
        .args(&["install"])
        .current_dir(temp_dir)
        .status();
    
    // Load the module
    let _ = Command::new("modprobe")
        .arg("vendor-reset")
        .status();
    
    // Add to modules to load at boot
    let modules_file = "/etc/modules";
    if let Ok(mut content) = fs::read_to_string(modules_file) {
        if !content.contains("vendor-reset") {
            content.push_str("\nvendor-reset\n");
            fs::write(modules_file, content).ok();
        }
    }
}

fn uninstall_vendor_reset() {
    println!("🗑️  Uninstalling vendor-reset...");
    
    // Remove from DKMS
    let _ = Command::new("dkms")
        .args(&["remove", "-m", "vendor-reset", "-v", "0.1.1", "--all"])
        .status();
    
    // Remove source
    let _ = fs::remove_dir_all("/usr/src/vendor-reset-0.1.1");
    
    // Remove module
    let _ = Command::new("modprobe")
        .args(&["-r", "vendor-reset"])
        .status();
    
    println!("✅ Vendor-reset uninstalled");
}

fn configure_vendor_reset_usage() {
    println!("\n🔧 Configuring vendor-reset usage...");
    
    let gpus = detect_gpus();
    let amd_gpus: Vec<&GpuDevice> = gpus
        .iter()
        .filter(|g| g.vendor_id == "0x1002" || g.description.to_lowercase().contains("amd") || g.description.to_lowercase().contains("radeon"))
        .collect();
    
    if amd_gpus.is_empty() {
        println!("❌ No AMD GPUs detected!");
        return;
    }
    
    println!("\n📋 Detected AMD GPUs:");
    for gpu in &amd_gpus {
        println!("  {} - {}", gpu.pci_address, gpu.description);
    }
    
    let reset_methods = vec![
        "device_specific - GPU-specific reset (recommended)",
        "function_level - PCI function-level reset",
        "bus - PCI bus reset (may affect other devices)",
        "Auto-detect best method",
    ];
    
    let method_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select reset method")
        .items(&reset_methods)
        .default(0)
        .interact()
        .unwrap();
    
    let reset_method = match method_idx {
        0 => "device_specific",
        1 => "function_level", 
        2 => "bus",
        3 => {
            // Auto-detect based on GPU model
            let gpu = amd_gpus[0];
            if gpu.description.to_lowercase().contains("rx") {
                "device_specific"
            } else {
                "function_level"
            }
        }
        _ => "device_specific",
    };
    
    // Configure reset method in modprobe
    let reset_conf = format!(
        "# AMD GPU Vendor Reset Configuration\n\
         # Generated by ghostctl\n\
         options vendor-reset reset_method={}\n",
        reset_method
    );
    
    fs::write("/etc/modprobe.d/vendor-reset.conf", reset_conf).ok();
    
    // Load the module
    let _ = Command::new("modprobe")
        .arg("vendor-reset")
        .status();
    
    println!("✅ Vendor-reset configured with {} method", reset_method);
    
    // Test if module loaded successfully
    let lsmod_output = Command::new("lsmod")
        .arg("vendor-reset")
        .output();
    
    if let Ok(lsmod_output) = lsmod_output {
        if lsmod_output.status.success() && !lsmod_output.stdout.is_empty() {
            println!("✅ Vendor-reset module loaded successfully");
        } else {
            println!("⚠️  Vendor-reset module may not have loaded. Check dmesg for errors.");
        }
    } else {
        println!("⚠️  Could not check module status.");
    }
    
    show_vendor_reset_usage();
}

fn show_vendor_reset_usage() {
    println!("\n📋 Vendor-Reset Usage Instructions:");
    println!("\n1. The vendor-reset module is now active");
    println!("2. VM configuration tips:");
    println!("   • Use UEFI (OVMF) firmware");
    println!("   • Enable 'Pre-Enroll keys' in UEFI settings");
    println!("   • Use machine type q35");
    
    println!("\n3. Reset test command:");
    println!("   echo 1 > /sys/bus/pci/devices/0000:XX:XX.X/reset");
    println!("   (Replace XX:XX.X with your GPU PCI address)");
    
    println!("\n4. VM shutdown/restart should now work properly");
    
    println!("\n5. If issues persist:");
    println!("   • Check dmesg for vendor-reset messages");
    println!("   • Try different reset methods");
    println!("   • Ensure GPU is not in use by host");
    
    println!("\n⚠️  Note: Some GPUs may still have issues - this is a hardware/firmware limitation");
}