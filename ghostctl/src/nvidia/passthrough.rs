use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::fs;
use std::process::Command;

pub fn passthrough_menu() {
    println!("🖥️  GPU Passthrough & Virtualization");
    println!("====================================");

    let options = [
        "🔍 Check passthrough compatibility",
        "⚙️  Configure VFIO",
        "🐧 Setup libvirt hooks",
        "📋 Generate VM XML configuration",
        "🔧 Fix passthrough issues",
        "📊 Show IOMMU groups",
        "🎮 Looking Glass setup",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("GPU Passthrough")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => check_passthrough_compatibility(),
        1 => configure_vfio(),
        2 => setup_libvirt_hooks(),
        3 => generate_vm_xml(),
        4 => fix_passthrough_issues(),
        5 => show_iommu_groups(),
        6 => setup_looking_glass(),
        _ => return,
    }
}

pub fn check_passthrough_compatibility() {
    println!("🔍 Checking GPU passthrough compatibility...\n");

    // Check IOMMU support
    println!("=== IOMMU SUPPORT ===");
    check_iommu_support();

    // Check virtualization support
    println!("\n=== VIRTUALIZATION SUPPORT ===");
    check_virtualization_support();

    // Check GPU information
    println!("\n=== GPU INFORMATION ===");
    check_gpu_info();

    // Check IOMMU groups
    println!("\n=== IOMMU GROUPS ===");
    check_iommu_groups();

    // Check for multiple GPUs
    println!("\n=== MULTI-GPU SETUP ===");
    check_multi_gpu();

    // Overall assessment
    println!("\n=== PASSTHROUGH ASSESSMENT ===");
    assess_passthrough_viability();
}

fn check_iommu_support() {
    // Check if IOMMU is enabled in kernel
    if let Ok(cmdline) = fs::read_to_string("/proc/cmdline") {
        let has_intel_iommu = cmdline.contains("intel_iommu=on");
        let has_amd_iommu = cmdline.contains("amd_iommu=on");

        if has_intel_iommu || has_amd_iommu {
            println!("✅ IOMMU enabled in kernel parameters");
            if has_intel_iommu {
                println!("  Intel IOMMU: enabled");
            }
            if has_amd_iommu {
                println!("  AMD IOMMU: enabled");
            }
        } else {
            println!("❌ IOMMU not enabled in kernel parameters");
            println!("💡 Add intel_iommu=on (Intel) or amd_iommu=on (AMD) to kernel parameters");
        }
    }

    // Check IOMMU directory
    if std::path::Path::new("/sys/kernel/iommu_groups").exists() {
        let output = Command::new("find")
            .args(&["/sys/kernel/iommu_groups", "-type", "d"])
            .output();

        if let Ok(output) = output {
            let group_count = String::from_utf8_lossy(&output.stdout)
                .lines()
                .count()
                .saturating_sub(1); // Subtract 1 for the root directory

            if group_count > 0 {
                println!("✅ IOMMU groups found: {}", group_count);
            } else {
                println!("❌ No IOMMU groups found");
            }
        }
    } else {
        println!("❌ IOMMU groups directory not found");
    }
}

fn check_virtualization_support() {
    // Check CPU virtualization features
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        let has_vmx = cpuinfo.contains("vmx"); // Intel VT-x
        let has_svm = cpuinfo.contains("svm"); // AMD-V

        if has_vmx {
            println!("✅ Intel VT-x support detected");
        } else if has_svm {
            println!("✅ AMD-V support detected");
        } else {
            println!("❌ No hardware virtualization support detected");
        }
    }

    // Check if KVM is loaded
    let output = Command::new("lsmod").output();
    if let Ok(output) = output {
        let lsmod = String::from_utf8_lossy(&output.stdout);
        if lsmod.contains("kvm") {
            println!("✅ KVM module loaded");
            if lsmod.contains("kvm_intel") {
                println!("  Intel KVM module loaded");
            }
            if lsmod.contains("kvm_amd") {
                println!("  AMD KVM module loaded");
            }
        } else {
            println!("❌ KVM module not loaded");
        }
    }

    // Check if VFIO is loaded
    let vfio_output = Command::new("lsmod").output();
    if let Ok(output) = vfio_output {
        let lsmod = String::from_utf8_lossy(&output.stdout);
        if lsmod.contains("vfio") {
            println!("✅ VFIO modules loaded");
        } else {
            println!("⚠️  VFIO modules not loaded");
        }
    }
}

fn check_gpu_info() {
    // Get GPU information from lspci
    let output = Command::new("lspci").args(&["-nn", "-k"]).output();

    if let Ok(output) = output {
        let lspci_output = String::from_utf8_lossy(&output.stdout);
        let mut gpu_count = 0;

        for line in lspci_output.lines() {
            if (line.to_lowercase().contains("vga") || line.to_lowercase().contains("3d"))
                && line.to_lowercase().contains("nvidia")
            {
                gpu_count += 1;
                println!("🎮 GPU {}: {}", gpu_count, line);

                // Check if bound to vfio-pci
                if line.contains("vfio-pci") {
                    println!("  ✅ Bound to vfio-pci");
                } else if line.contains("nvidia") {
                    println!("  ⚠️  Bound to nvidia driver");
                }
            }
        }

        if gpu_count == 0 {
            println!("❌ No NVIDIA GPUs detected");
        } else {
            println!("📊 Total NVIDIA GPUs: {}", gpu_count);
        }
    }
}

fn check_iommu_groups() {
    let iommu_path = "/sys/kernel/iommu_groups";
    if !std::path::Path::new(iommu_path).exists() {
        println!("❌ IOMMU groups not available");
        return;
    }

    // Find NVIDIA devices in IOMMU groups
    let output = Command::new("sh")
        .arg("-c")
        .arg("for g in $(find /sys/kernel/iommu_groups/* -maxdepth 0 -type d | sort -V); do echo \"IOMMU Group ${g##*/}:\"; for d in $g/devices/*; do echo -e \"\\t$(lspci -nns ${d##*/})\"; done; done | grep -A 10 -B 1 -i nvidia")
        .output();

    if let Ok(output) = output {
        let iommu_info = String::from_utf8_lossy(&output.stdout);
        if iommu_info.trim().is_empty() {
            println!("⚠️  No NVIDIA devices found in IOMMU groups");
        } else {
            println!("🔍 NVIDIA devices in IOMMU groups:");
            println!("{}", iommu_info);
        }
    }
}

fn check_multi_gpu() {
    let output = Command::new("nvidia-smi").args(&["-L"]).output();

    if let Ok(output) = output {
        let gpu_list = String::from_utf8_lossy(&output.stdout);
        let gpu_count = gpu_list.lines().count();

        if gpu_count > 1 {
            println!("✅ Multiple GPUs detected ({})", gpu_count);
            println!("💡 You can pass through secondary GPUs while keeping primary for host");
        } else if gpu_count == 1 {
            println!("⚠️  Only one GPU detected");
            println!("💡 Single GPU passthrough requires stopping display manager");
        } else {
            println!("❌ No GPUs detected via nvidia-smi");
        }
    }
}

fn assess_passthrough_viability() {
    let mut score = 0;
    let mut issues = Vec::new();

    // Check IOMMU
    if let Ok(cmdline) = fs::read_to_string("/proc/cmdline") {
        if cmdline.contains("iommu=on")
            || cmdline.contains("intel_iommu=on")
            || cmdline.contains("amd_iommu=on")
        {
            score += 2;
        } else {
            issues.push("IOMMU not enabled in kernel");
        }
    }

    // Check IOMMU groups
    if std::path::Path::new("/sys/kernel/iommu_groups").exists() {
        score += 1;
    } else {
        issues.push("IOMMU groups not found");
    }

    // Check virtualization
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        if cpuinfo.contains("vmx") || cpuinfo.contains("svm") {
            score += 1;
        } else {
            issues.push("Hardware virtualization not supported");
        }
    }

    // Check GPU count
    let output = Command::new("nvidia-smi").args(&["-L"]).output();
    if let Ok(output) = output {
        let gpu_count = String::from_utf8_lossy(&output.stdout).lines().count();
        if gpu_count > 1 {
            score += 2;
        } else if gpu_count == 1 {
            score += 1;
            issues.push("Single GPU setup - requires stopping display manager");
        }
    }

    // Assessment
    match score {
        5..=6 => println!("✅ Excellent passthrough compatibility"),
        3..=4 => println!("⚠️  Good passthrough compatibility with minor issues"),
        1..=2 => println!("❌ Poor passthrough compatibility"),
        _ => println!("❌ Passthrough not viable"),
    }

    if !issues.is_empty() {
        println!("\n🔧 Issues to address:");
        for issue in issues {
            println!("  • {}", issue);
        }
    }
}

pub fn configure_vfio() {
    println!("⚙️  Configuring VFIO for GPU passthrough...");

    let options = [
        "Enable VFIO modules",
        "Bind GPU to VFIO",
        "Configure kernel parameters",
        "Setup VFIO permissions",
        "All of the above",
        "Cancel",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("VFIO Configuration")
        .items(&options)
        .default(4)
        .interact()
        .unwrap();

    match choice {
        0 => enable_vfio_modules(),
        1 => bind_gpu_to_vfio(),
        2 => configure_kernel_parameters(),
        3 => setup_vfio_permissions(),
        4 => {
            enable_vfio_modules();
            configure_kernel_parameters();
            bind_gpu_to_vfio();
            setup_vfio_permissions();
        }
        _ => return,
    }
}

fn enable_vfio_modules() {
    println!("🔧 Enabling VFIO modules...");

    let modules = ["vfio", "vfio_iommu_type1", "vfio_pci", "vfio_virqfd"];

    // Create modules-load.d file
    let modules_content = modules.join("\n") + "\n";
    let _ = fs::write("/tmp/vfio.conf", &modules_content);
    let _ = Command::new("sudo")
        .args(&["mv", "/tmp/vfio.conf", "/etc/modules-load.d/"])
        .status();

    // Load modules now
    for module in &modules {
        let _ = Command::new("sudo").args(&["modprobe", module]).status();
    }

    println!("✅ VFIO modules enabled");
}

fn bind_gpu_to_vfio() {
    println!("🔗 Binding GPU to VFIO...");

    // Get GPU PCI IDs
    let gpu_ids = get_nvidia_pci_ids();
    if gpu_ids.is_empty() {
        println!("❌ No NVIDIA GPUs found");
        return;
    }

    println!("🎮 Found NVIDIA devices:");
    for (i, (id, name)) in gpu_ids.iter().enumerate() {
        println!("  {}: {} ({})", i + 1, name, id);
    }

    let indices = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select GPUs to bind to VFIO")
        .items(
            &gpu_ids
                .iter()
                .map(|(_, name)| name.as_str())
                .collect::<Vec<_>>(),
        )
        .interact()
        .unwrap();

    if indices.is_empty() {
        println!("❌ No GPUs selected");
        return;
    }

    // Get device IDs for selected GPUs
    let mut device_ids = Vec::new();
    for &idx in &indices {
        let (pci_id, _) = &gpu_ids[idx];

        // Get vendor and device ID
        let output = Command::new("lspci").args(&["-n", "-s", pci_id]).output();

        if let Ok(output) = output {
            let lspci_line = String::from_utf8_lossy(&output.stdout);
            if let Some(ids_part) = lspci_line.split_whitespace().nth(2) {
                device_ids.push(ids_part.to_string());
            }
        }
    }

    if device_ids.is_empty() {
        println!("❌ Could not get device IDs");
        return;
    }

    // Create VFIO configuration
    let vfio_content = format!("options vfio-pci ids={}\n", device_ids.join(","));
    let _ = fs::write("/tmp/vfio.conf", &vfio_content);
    let _ = Command::new("sudo")
        .args(&["mv", "/tmp/vfio.conf", "/etc/modprobe.d/"])
        .status();

    println!("✅ GPU binding configured");
    println!("🔄 Reboot required for changes to take effect");
}

fn get_nvidia_pci_ids() -> Vec<(String, String)> {
    let mut gpu_ids = Vec::new();

    let output = Command::new("lspci").args(&["-nn"]).output();

    if let Ok(output) = output {
        let lspci_output = String::from_utf8_lossy(&output.stdout);

        for line in lspci_output.lines() {
            if (line.to_lowercase().contains("vga") || line.to_lowercase().contains("3d"))
                && line.to_lowercase().contains("nvidia")
            {
                if let Some(pci_id) = line.split_whitespace().next() {
                    let name = line
                        .split("NVIDIA Corporation")
                        .nth(1)
                        .unwrap_or("Unknown NVIDIA GPU")
                        .trim()
                        .split('[')
                        .next()
                        .unwrap_or("Unknown")
                        .trim();

                    gpu_ids.push((pci_id.to_string(), name.to_string()));
                }
            }
        }
    }

    gpu_ids
}

fn configure_kernel_parameters() {
    println!("⚙️  Configuring kernel parameters...");

    // Detect CPU vendor
    let cpu_vendor = detect_cpu_vendor();

    let iommu_param = match cpu_vendor.as_str() {
        "intel" => "intel_iommu=on",
        "amd" => "amd_iommu=on",
        _ => "iommu=pt",
    };

    println!("💡 Detected CPU: {}", cpu_vendor);
    println!("💡 Required kernel parameters:");
    println!("  {}", iommu_param);
    println!("  iommu=pt");
    println!("  vfio-pci.ids=<device_ids>");

    let grub_line = format!("GRUB_CMDLINE_LINUX_DEFAULT=\"{} iommu=pt\"", iommu_param);
    println!("\n📝 Add to /etc/default/grub:");
    println!("{}", grub_line);

    let auto_configure = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Automatically configure GRUB?")
        .interact()
        .unwrap();

    if auto_configure {
        configure_grub_automatically(&iommu_param);
    }
}

fn detect_cpu_vendor() -> String {
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if line.starts_with("vendor_id") {
                if line.contains("GenuineIntel") {
                    return "intel".to_string();
                } else if line.contains("AuthenticAMD") {
                    return "amd".to_string();
                }
            }
        }
    }
    "unknown".to_string()
}

fn configure_grub_automatically(iommu_param: &str) {
    println!("🔧 Configuring GRUB automatically...");

    // Read current GRUB config
    let grub_file = "/etc/default/grub";
    if let Ok(content) = fs::read_to_string(grub_file) {
        let mut new_content = String::new();
        let mut modified = false;

        for line in content.lines() {
            if line.starts_with("GRUB_CMDLINE_LINUX_DEFAULT=") {
                // Modify this line
                let params = format!("{} iommu=pt", iommu_param);
                if !line.contains(&iommu_param) {
                    let new_line = if line.contains("\"") {
                        line.replace("\"", &format!("{} \"", params))
                    } else {
                        format!("GRUB_CMDLINE_LINUX_DEFAULT=\"{}\"", params)
                    };
                    new_content.push_str(&new_line);
                    modified = true;
                } else {
                    new_content.push_str(line);
                }
            } else {
                new_content.push_str(line);
            }
            new_content.push('\n');
        }

        if modified {
            let _ = fs::write("/tmp/grub", &new_content);
            let _ = Command::new("sudo")
                .args(&["mv", "/tmp/grub", grub_file])
                .status();

            // Update GRUB
            let _ = Command::new("sudo")
                .args(&["grub-mkconfig", "-o", "/boot/grub/grub.cfg"])
                .status();

            println!("✅ GRUB configured automatically");
        }
    }
}

fn setup_vfio_permissions() {
    println!("🔐 Setting up VFIO permissions...");

    // Create VFIO group and add user
    if let Ok(username) = std::env::var("USER") {
        let _ = Command::new("sudo")
            .args(&["groupadd", "-f", "vfio"])
            .status();

        let _ = Command::new("sudo")
            .args(&["usermod", "-aG", "vfio", &username])
            .status();

        println!("✅ Added {} to vfio group", username);
    }

    // Create udev rules
    let udev_rules = r#"SUBSYSTEM=="vfio", GROUP="vfio", MODE="0664"
KERNEL=="vfio", GROUP="vfio", MODE="0664"
"#;

    let _ = fs::write("/tmp/10-vfio.rules", udev_rules);
    let _ = Command::new("sudo")
        .args(&["mv", "/tmp/10-vfio.rules", "/etc/udev/rules.d/"])
        .status();

    // Reload udev rules
    let _ = Command::new("sudo")
        .args(&["udevadm", "control", "--reload-rules"])
        .status();

    println!("✅ VFIO permissions configured");
}

pub fn setup_libvirt_hooks() {
    println!("🐧 Setting up libvirt hooks for GPU passthrough...");

    // Check if libvirt is installed
    if !Command::new("which").arg("virsh").status().is_ok() {
        println!("❌ libvirt not found. Install libvirt first.");
        return;
    }

    // Create hooks directory
    let hooks_dir = "/etc/libvirt/hooks";
    let _ = Command::new("sudo")
        .args(&["mkdir", "-p", hooks_dir])
        .status();

    // Create QEMU hook script
    let hook_script = create_qemu_hook_script();
    let hook_path = format!("{}/qemu", hooks_dir);

    let _ = fs::write("/tmp/qemu_hook", &hook_script);
    let _ = Command::new("sudo")
        .args(&["mv", "/tmp/qemu_hook", &hook_path])
        .status();

    let _ = Command::new("sudo")
        .args(&["chmod", "+x", &hook_path])
        .status();

    println!("✅ Libvirt hooks configured");
    println!("💡 Hook script installed at: {}", hook_path);
}

fn create_qemu_hook_script() -> String {
    r#"#!/bin/bash

# QEMU hook script for GPU passthrough
# This script handles GPU unbinding/binding during VM start/stop

GUEST_NAME="$1"
HOOK_NAME="$2"
STATE_NAME="$3"
MISC="${@:4}"

VIRSH_GPU_VIDEO="pci_0000_01_00_0"  # Adjust to your GPU's PCI address
VIRSH_GPU_AUDIO="pci_0000_01_00_1"  # Adjust to your GPU's audio PCI address

function bind_vfio() {
    echo "Binding GPU to VFIO..."
    virsh nodedev-detach $VIRSH_GPU_VIDEO
    virsh nodedev-detach $VIRSH_GPU_AUDIO
}

function release_vfio() {
    echo "Releasing GPU from VFIO..."
    virsh nodedev-reattach $VIRSH_GPU_VIDEO
    virsh nodedev-reattach $VIRSH_GPU_AUDIO
}

# For single GPU passthrough, you might want to stop/start display manager
function stop_display_manager() {
    echo "Stopping display manager..."
    systemctl stop gdm  # or sddm, lightdm, etc.
    # Kill any remaining processes using GPU
    pkill -f nvidia
}

function start_display_manager() {
    echo "Starting display manager..."
    systemctl start gdm  # or sddm, lightdm, etc.
}

case $STATE_NAME in
    "prepare")
        bind_vfio
        # Uncomment for single GPU passthrough:
        # stop_display_manager
        ;;
    "release")
        release_vfio
        # Uncomment for single GPU passthrough:
        # start_display_manager
        ;;
esac
"#
    .to_string()
}

pub fn generate_vm_xml() {
    println!("📋 Generating VM XML configuration for GPU passthrough...");

    let vm_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("VM name")
        .default("gpu-passthrough-vm".to_string())
        .interact()
        .unwrap();

    let memory: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Memory (GB)")
        .default("8".to_string())
        .interact()
        .unwrap();

    let vcpus: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("vCPUs")
        .default("4".to_string())
        .interact()
        .unwrap();

    // Get GPU PCI addresses
    let gpu_ids = get_nvidia_pci_ids();
    if gpu_ids.is_empty() {
        println!("❌ No NVIDIA GPUs found");
        return;
    }

    println!("🎮 Available GPUs:");
    for (i, (id, name)) in gpu_ids.iter().enumerate() {
        println!("  {}: {} ({})", i + 1, name, id);
    }

    let gpu_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select GPU for passthrough")
        .items(
            &gpu_ids
                .iter()
                .map(|(_, name)| name.as_str())
                .collect::<Vec<_>>(),
        )
        .interact()
        .unwrap();

    let (gpu_pci, _) = &gpu_ids[gpu_choice];

    // Generate XML
    let xml_config = generate_vm_xml_content(&vm_name, &memory, &vcpus, gpu_pci);

    // Save to file
    let filename = format!("{}.xml", vm_name);
    let _ = fs::write(&filename, &xml_config);

    println!("✅ VM XML configuration generated: {}", filename);
    println!("💡 Import with: virsh define {}", filename);
}

fn generate_vm_xml_content(name: &str, memory: &str, vcpus: &str, gpu_pci: &str) -> String {
    let memory_kb = memory.parse::<u64>().unwrap_or(8) * 1024 * 1024;

    // Parse PCI address
    let pci_parts: Vec<&str> = gpu_pci.split(':').collect();
    let domain = "0x0000";
    let bus = if pci_parts.len() > 1 {
        format!("0x{}", pci_parts[0])
    } else {
        "0x01".to_string()
    };
    let slot = if pci_parts.len() > 2 {
        format!("0x{}", pci_parts[1].split('.').next().unwrap_or("00"))
    } else {
        "0x00".to_string()
    };
    let function = if pci_parts.len() > 2 && pci_parts[1].contains('.') {
        format!("0x{}", pci_parts[1].split('.').nth(1).unwrap_or("0"))
    } else {
        "0x0".to_string()
    };

    format!(
        r#"<domain type='kvm'>
  <name>{}</name>
  <uuid>UUID_PLACEHOLDER</uuid>
  <memory unit='KiB'>{}</memory>
  <currentMemory unit='KiB'>{}</currentMemory>
  <vcpu placement='static'>{}</vcpu>
  
  <features>
    <acpi/>
    <apic/>
    <hyperv>
      <relaxed state='on'/>
      <vapic state='on'/>
      <spinlocks state='on' retries='8191'/>
      <vendor_id state='on' value='whatever'/>
    </hyperv>
    <kvm>
      <hidden state='on'/>
    </kvm>
    <vmport state='off'/>
  </features>
  
  <cpu mode='host-passthrough' check='none'>
    <topology sockets='1' cores='{}' threads='1'/>
  </cpu>
  
  <clock offset='localtime'>
    <timer name='rtc' tickpolicy='catchup'/>
    <timer name='pit' tickpolicy='delay'/>
    <timer name='hpet' present='no'/>
    <timer name='hypervclock' present='yes'/>
  </clock>
  
  <on_poweroff>destroy</on_poweroff>
  <on_reboot>restart</on_reboot>
  <on_crash>destroy</on_crash>
  
  <pm>
    <suspend-to-mem enabled='no'/>
    <suspend-to-disk enabled='no'/>
  </pm>
  
  <devices>
    <emulator>/usr/bin/qemu-system-x86_64</emulator>
    
    <!-- GPU Passthrough -->
    <hostdev mode='subsystem' type='pci' managed='yes'>
      <source>
        <address domain='{}' bus='{}' slot='{}' function='{}'/>
      </source>
      <address type='pci' domain='0x0000' bus='0x01' slot='0x00' function='0x0'/>
    </hostdev>
    
    <!-- Add GPU audio if present -->
    <!-- 
    <hostdev mode='subsystem' type='pci' managed='yes'>
      <source>
        <address domain='{}' bus='{}' slot='{}' function='0x1'/>
      </source>
      <address type='pci' domain='0x0000' bus='0x01' slot='0x01' function='0x0'/>
    </hostdev>
    -->
    
    <!-- Disk -->
    <disk type='file' device='disk'>
      <driver name='qemu' type='qcow2'/>
      <source file='/var/lib/libvirt/images/{}.qcow2'/>
      <target dev='vda' bus='virtio'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x07' function='0x0'/>
    </disk>
    
    <!-- Network -->
    <interface type='network'>
      <source network='default'/>
      <model type='virtio'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x03' function='0x0'/>
    </interface>
    
    <!-- USB Controllers -->
    <controller type='usb' index='0' model='qemu-xhci' ports='15'>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x0a' function='0x0'/>
    </controller>
    
    <!-- Input devices -->
    <input type='mouse' bus='ps2'/>
    <input type='keyboard' bus='ps2'/>
    
    <!-- Graphics (remove for full GPU passthrough) -->
    <graphics type='spice' autoport='yes'>
      <listen type='address'/>
    </graphics>
    <video>
      <model type='qxl' ram='65536' vram='65536' vgamem='16384' heads='1' primary='yes'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x02' function='0x0'/>
    </video>
  </devices>
</domain>"#,
        name,
        memory_kb,
        memory_kb,
        vcpus,
        vcpus,
        domain,
        bus,
        slot,
        function,
        domain,
        bus,
        slot, // For audio device
        name
    )
}

pub fn fix_passthrough_issues() {
    println!("🔧 Fixing GPU passthrough issues...");

    let options = [
        "Reset GPU binding",
        "Fix VFIO permissions",
        "Restart libvirt services",
        "Check IOMMU groups",
        "Fix kernel modules",
        "Cancel",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select fix option")
        .items(&options)
        .default(5)
        .interact()
        .unwrap();

    match choice {
        0 => reset_gpu_binding(),
        1 => setup_vfio_permissions(),
        2 => restart_libvirt_services(),
        3 => show_iommu_groups(),
        4 => fix_kernel_modules(),
        _ => return,
    }
}

fn reset_gpu_binding() {
    println!("🔄 Resetting GPU binding...");

    // Detach and reattach GPU
    let gpu_ids = get_nvidia_pci_ids();
    for (pci_id, name) in gpu_ids {
        println!("  Processing: {} ({})", name, pci_id);

        let virsh_id = format!("pci_0000_{}", pci_id.replace(':', "_").replace('.', "_"));

        let _ = Command::new("sudo")
            .args(&["virsh", "nodedev-reattach", &virsh_id])
            .status();

        let _ = Command::new("sudo")
            .args(&["virsh", "nodedev-detach", &virsh_id])
            .status();
    }

    println!("✅ GPU binding reset");
}

fn restart_libvirt_services() {
    println!("🔄 Restarting libvirt services...");

    let services = ["libvirtd", "virtlogd", "virtlockd"];

    for service in &services {
        let _ = Command::new("sudo")
            .args(&["systemctl", "restart", service])
            .status();
        println!("  Restarted: {}", service);
    }

    println!("✅ Libvirt services restarted");
}

fn fix_kernel_modules() {
    println!("🔧 Fixing kernel modules...");

    // Reload VFIO modules
    let vfio_modules = ["vfio_pci", "vfio", "vfio_iommu_type1"];

    // Unload modules
    for module in vfio_modules.iter().rev() {
        let _ = Command::new("sudo")
            .args(&["modprobe", "-r", module])
            .status();
    }

    // Load modules
    for module in &vfio_modules {
        let _ = Command::new("sudo").args(&["modprobe", module]).status();
    }

    println!("✅ Kernel modules reloaded");
}

pub fn show_iommu_groups() {
    println!("📊 IOMMU Groups...\n");

    let _ = Command::new("sh")
        .arg("-c")
        .arg("for g in $(find /sys/kernel/iommu_groups/* -maxdepth 0 -type d | sort -V); do echo \"IOMMU Group ${g##*/}:\"; for d in $g/devices/*; do echo -e \"\\t$(lspci -nns ${d##*/})\"; done; done")
        .status();
}

pub fn setup_looking_glass() {
    println!("🎮 Setting up Looking Glass for near-native GPU passthrough performance...");

    // Check if Looking Glass is available
    let lg_status = Command::new("which").arg("looking-glass-client").status();
    if !lg_status.map(|s| s.success()).unwrap_or(false) {
        println!("📦 Looking Glass not found. Installing...");

        let _ = Command::new("sudo")
            .args(&["pacman", "-S", "--noconfirm", "looking-glass"])
            .status();
    }

    // Setup shared memory
    setup_looking_glass_shm();

    // Generate Looking Glass config
    generate_looking_glass_config();

    println!("✅ Looking Glass setup complete");
    println!("💡 Install Looking Glass host software in your Windows VM");
    println!("💡 Add shared memory device to your VM XML configuration");
}

fn setup_looking_glass_shm() {
    println!("🔧 Setting up shared memory for Looking Glass...");

    // Create tmpfs mount for shared memory
    let fstab_entry = "tmpfs /dev/shm tmpfs defaults,size=128M 0 0\n";

    // Check if already in fstab
    if let Ok(fstab_content) = fs::read_to_string("/etc/fstab") {
        if !fstab_content.contains("looking-glass") {
            let _ = fs::write("/tmp/fstab_append", fstab_entry);
            let _ = Command::new("sudo")
                .args(&["sh", "-c", "cat /tmp/fstab_append >> /etc/fstab"])
                .status();
        }
    }

    // Create shared memory file
    let _ = Command::new("sudo")
        .args(&["touch", "/dev/shm/looking-glass"])
        .status();

    let _ = Command::new("sudo")
        .args(&["chmod", "660", "/dev/shm/looking-glass"])
        .status();

    // Set ownership
    if let Ok(username) = std::env::var("USER") {
        let _ = Command::new("sudo")
            .args(&[
                "chown",
                &format!("{}:kvm", username),
                "/dev/shm/looking-glass",
            ])
            .status();
    }

    println!("✅ Shared memory configured");
}

fn generate_looking_glass_config() {
    println!("📝 Generating Looking Glass configuration...");

    let config_dir = format!(
        "{}/.config/looking-glass",
        std::env::var("HOME").unwrap_or_else(|_| ".".to_string())
    );
    let _ = fs::create_dir_all(&config_dir);

    let config_content = r#"[app]
shmFile=/dev/shm/looking-glass

[win]
title=Looking Glass
position=center
size=1920x1080
fpsMin=60

[input]
grabKeyboard=yes
grabKeyboardOnFocus=yes
releaseKeysOnFocusLoss=yes

[spice]
enable=yes
host=127.0.0.1
port=5900
"#;

    let config_path = format!("{}/client.ini", config_dir);
    let _ = fs::write(&config_path, config_content);

    println!("✅ Looking Glass configuration created: {}", config_path);
}
