use dialoguer::{Select, Input, Confirm, theme::ColorfulTheme};
use std::process::Command;
use std::fs;

pub fn pve_management_menu() {
    println!("🏠 Proxmox VE Management");
    println!("========================");
    
    let options = [
        "🖥️  Virtual Machine Management",
        "📦 Container (LXC) Management",
        "💾 Storage Management", 
        "🌐 Network Configuration",
        "📊 Backup & Recovery",
        "🏗️  Template Management",
        "📈 Monitoring & Status",
        "🔧 Cluster Management",
        "🚀 Homelab Automation",
        "⬅️  Back",
    ];
    
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Proxmox VE Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    
    match choice {
        0 => vm_management_menu(),
        1 => container_management_menu(),
        2 => storage_management_menu(),
        3 => network_configuration_menu(),
        4 => backup_recovery_menu(),
        5 => template_management_menu(),
        6 => monitoring_status_menu(),
        7 => cluster_management_menu(),
        8 => homelab_automation_menu(),
        _ => return,
    }
}

pub fn vm_management_menu() {
    println!("🖥️  Virtual Machine Management");
    println!("==============================");
    
    let options = [
        "📋 List VMs",
        "🆕 Create VM",
        "📋 VM Status", 
        "▶️  Start VM",
        "⏹️  Stop VM",
        "🔄 Restart VM",
        "📸 VM Snapshots",
        "🏗️  Clone VM",
        "🚚 Migrate VM",
        "🗑️  Delete VM",
        "⬅️  Back",
    ];
    
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("VM Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    
    match choice {
        0 => list_vms(),
        1 => create_vm_wizard(),
        2 => show_vm_status(),
        3 => start_vm_interactive(),
        4 => stop_vm_interactive(),
        5 => restart_vm_interactive(),
        6 => vm_snapshot_menu(),
        7 => clone_vm_interactive(),
        8 => migrate_vm_interactive(),
        9 => delete_vm_interactive(),
        _ => return,
    }
}

pub fn list_vms() {
    println!("📋 Listing Virtual Machines");
    println!("===========================");
    
    let output = Command::new("qm")
        .arg("list")
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                let output_str = String::from_utf8_lossy(&result.stdout);
                println!("{}", output_str);
            } else {
                let error_str = String::from_utf8_lossy(&result.stderr);
                println!("❌ Error listing VMs: {}", error_str);
            }
        },
        Err(e) => {
            println!("❌ Failed to execute qm command: {}", e);
            println!("💡 Make sure you're running this on a Proxmox VE host");
        }
    }
}

pub fn create_vm_wizard() {
    println!("🆕 Create Virtual Machine Wizard");
    println!("=================================");
    
    let vm_id: String = Input::new()
        .with_prompt("VM ID (100-999999)")
        .default("100")
        .interact_text()
        .unwrap();
    
    let vm_name: String = Input::new()
        .with_prompt("VM Name")
        .default(&format!("vm-{}", vm_id))
        .interact_text()
        .unwrap();
    
    let memory: String = Input::new()
        .with_prompt("Memory (MB)")
        .default("2048")
        .interact_text()
        .unwrap();
    
    let cores: String = Input::new()
        .with_prompt("CPU Cores")
        .default("2")
        .interact_text()
        .unwrap();
    
    let creation_methods = [
        "📋 Clone from template",
        "💿 Install from ISO",
        "🌐 Download cloud image",
        "📦 Import existing disk",
    ];
    
    let method = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Creation method")
        .items(&creation_methods)
        .default(0)
        .interact()
        .unwrap();
    
    match method {
        0 => create_vm_from_template(&vm_id, &vm_name, &memory, &cores),
        1 => create_vm_from_iso(&vm_id, &vm_name, &memory, &cores),
        2 => create_vm_from_cloud_image(&vm_id, &vm_name, &memory, &cores),
        3 => create_vm_import_disk(&vm_id, &vm_name, &memory, &cores),
        _ => return,
    }
}

fn create_vm_from_template(vm_id: &str, vm_name: &str, memory: &str, cores: &str) {
    let template_id: String = Input::new()
        .with_prompt("Template ID to clone from")
        .default("9000")
        .interact_text()
        .unwrap();
    
    println!("🏗️  Creating VM from template...");
    
    // Clone from template
    let status = Command::new("qm")
        .args(&["clone", &template_id, vm_id, "--name", vm_name, "--full"])
        .status();
    
    if status.is_ok() && status.unwrap().success() {
        // Configure VM
        let _ = Command::new("qm")
            .args(&["set", vm_id, 
                   "--memory", memory,
                   "--cores", cores,
                   "--net0", "virtio,bridge=vmbr0,firewall=1"])
            .status();
        
        println!("✅ VM {} created successfully", vm_name);
        
        let start_vm = Confirm::new()
            .with_prompt("Start VM now?")
            .default(true)
            .interact()
            .unwrap();
        
        if start_vm {
            start_vm(vm_id);
        }
    } else {
        println!("❌ Failed to create VM from template");
    }
}

fn create_vm_from_iso(vm_id: &str, vm_name: &str, memory: &str, cores: &str) {
    let iso_file: String = Input::new()
        .with_prompt("ISO file path (e.g., local:iso/ubuntu-22.04.iso)")
        .interact_text()
        .unwrap();
    
    let disk_size: String = Input::new()
        .with_prompt("Disk size (e.g., 32G)")
        .default("32G")
        .interact_text()
        .unwrap();
    
    println!("💿 Creating VM with ISO...");
    
    // Create VM
    let status = Command::new("qm")
        .args(&["create", vm_id,
               "--name", vm_name,
               "--memory", memory,
               "--cores", cores,
               "--net0", "virtio,bridge=vmbr0",
               "--ide2", &format!("{},media=cdrom", iso_file),
               "--scsi0", &format!("local-lvm:{}", disk_size),
               "--scsihw", "virtio-scsi-pci",
               "--boot", "order=ide2;scsi0"])
        .status();
    
    if status.is_ok() && status.unwrap().success() {
        println!("✅ VM {} created with ISO", vm_name);
        println!("💡 Boot the VM and install the operating system");
    } else {
        println!("❌ Failed to create VM with ISO");
    }
}

fn create_vm_from_cloud_image(vm_id: &str, vm_name: &str, memory: &str, cores: &str) {
    let cloud_images = [
        "Ubuntu 22.04 LTS",
        "Ubuntu 20.04 LTS", 
        "Debian 12",
        "CentOS Stream 9",
        "Custom URL",
    ];
    
    let image_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Cloud image")
        .items(&cloud_images)
        .default(0)
        .interact()
        .unwrap();
    
    let image_url = match image_choice {
        0 => "https://cloud-images.ubuntu.com/jammy/current/jammy-server-cloudimg-amd64.img",
        1 => "https://cloud-images.ubuntu.com/focal/current/focal-server-cloudimg-amd64.img",
        2 => "https://cloud.debian.org/images/cloud/bookworm/latest/debian-12-generic-amd64.qcow2",
        3 => "https://cloud.centos.org/centos/9-stream/x86_64/images/CentOS-Stream-GenericCloud-9-latest.x86_64.qcow2",
        4 => {
            Input::new()
                .with_prompt("Custom image URL")
                .interact_text()
                .unwrap()
        },
        _ => return,
    };
    
    download_and_create_cloud_vm(vm_id, vm_name, memory, cores, &image_url);
}

fn download_and_create_cloud_vm(vm_id: &str, vm_name: &str, memory: &str, cores: &str, image_url: &str) {
    println!("📥 Downloading cloud image...");
    
    let image_name = format!("cloud-image-{}.img", vm_id);
    
    // Download image
    let download_status = Command::new("wget")
        .args(&[image_url, "-O", &format!("/tmp/{}", image_name)])
        .status();
    
    if download_status.is_ok() && download_status.unwrap().success() {
        println!("🏗️  Creating VM with cloud image...");
        
        // Create VM
        let _ = Command::new("qm")
            .args(&["create", vm_id,
                   "--name", vm_name,
                   "--memory", memory,
                   "--cores", cores,
                   "--net0", "virtio,bridge=vmbr0"])
            .status();
        
        // Import disk
        let _ = Command::new("qm")
            .args(&["importdisk", vm_id, &format!("/tmp/{}", image_name), "local-lvm"])
            .status();
        
        // Configure VM
        let _ = Command::new("qm")
            .args(&["set", vm_id,
                   "--scsihw", "virtio-scsi-pci",
                   "--scsi0", &format!("local-lvm:vm-{}-disk-0", vm_id),
                   "--ide2", "local-lvm:cloudinit",
                   "--boot", "c",
                   "--bootdisk", "scsi0",
                   "--serial0", "socket",
                   "--vga", "serial0",
                   "--agent", "enabled=1"])
            .status();
        
        // Cleanup
        let _ = std::fs::remove_file(&format!("/tmp/{}", image_name));
        
        println!("✅ VM {} created with cloud image", vm_name);
    } else {
        println!("❌ Failed to download cloud image");
    }
}

fn create_vm_import_disk(vm_id: &str, vm_name: &str, memory: &str, cores: &str) {
    let disk_path: String = Input::new()
        .with_prompt("Path to disk image")
        .interact_text()
        .unwrap();
    
    println!("📦 Creating VM with imported disk...");
    
    // Create VM
    let _ = Command::new("qm")
        .args(&["create", vm_id,
               "--name", vm_name,
               "--memory", memory,
               "--cores", cores,
               "--net0", "virtio,bridge=vmbr0"])
        .status();
    
    // Import disk
    let status = Command::new("qm")
        .args(&["importdisk", vm_id, &disk_path, "local-lvm"])
        .status();
    
    if status.is_ok() && status.unwrap().success() {
        // Configure VM to use imported disk
        let _ = Command::new("qm")
            .args(&["set", vm_id,
                   "--scsihw", "virtio-scsi-pci",
                   "--scsi0", &format!("local-lvm:vm-{}-disk-0", vm_id),
                   "--boot", "c",
                   "--bootdisk", "scsi0"])
            .status();
        
        println!("✅ VM {} created with imported disk", vm_name);
    } else {
        println!("❌ Failed to import disk");
    }
}

pub fn start_vm(vm_id: &str) {
    println!("▶️  Starting VM {}...", vm_id);
    
    let status = Command::new("qm")
        .args(&["start", vm_id])
        .status();
    
    match status {
        Ok(s) if s.success() => println!("✅ VM {} started successfully", vm_id),
        _ => println!("❌ Failed to start VM {}", vm_id),
    }
}

pub fn stop_vm(vm_id: &str) {
    println!("⏹️  Stopping VM {}...", vm_id);
    
    let status = Command::new("qm")
        .args(&["stop", vm_id])
        .status();
    
    match status {
        Ok(s) if s.success() => println!("✅ VM {} stopped successfully", vm_id),
        _ => println!("❌ Failed to stop VM {}", vm_id),
    }
}

fn start_vm_interactive() {
    let vm_id: String = Input::new()
        .with_prompt("VM ID to start")
        .interact_text()
        .unwrap();
    
    start_vm(&vm_id);
}

fn stop_vm_interactive() {
    let vm_id: String = Input::new()
        .with_prompt("VM ID to stop")
        .interact_text()
        .unwrap();
    
    stop_vm(&vm_id);
}

fn restart_vm_interactive() {
    let vm_id: String = Input::new()
        .with_prompt("VM ID to restart")
        .interact_text()
        .unwrap();
    
    println!("🔄 Restarting VM {}...", vm_id);
    
    let status = Command::new("qm")
        .args(&["restart", &vm_id])
        .status();
    
    match status {
        Ok(s) if s.success() => println!("✅ VM {} restarted successfully", vm_id),
        _ => println!("❌ Failed to restart VM {}", vm_id),
    }
}

fn show_vm_status() {
    list_vms();
    
    let vm_id: String = Input::new()
        .with_prompt("VM ID for detailed status")
        .interact_text()
        .unwrap();
    
    println!("📊 VM {} Status:", vm_id);
    
    let _ = Command::new("qm")
        .args(&["status", &vm_id])
        .status();
    
    let _ = Command::new("qm")
        .args(&["config", &vm_id])
        .status();
}

pub fn container_management_menu() {
    println!("📦 Container (LXC) Management");
    println!("=============================");
    
    let options = [
        "📋 List Containers",
        "🆕 Create Container",
        "📋 Container Status",
        "▶️  Start Container",
        "⏹️  Stop Container", 
        "🔄 Restart Container",
        "📸 Container Snapshots",
        "🏗️  Clone Container",
        "🚪 Enter Container",
        "🗑️  Delete Container",
        "⬅️  Back",
    ];
    
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Container Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    
    match choice {
        0 => list_containers(),
        1 => create_container_wizard(),
        2 => show_container_status(),
        3 => start_container_interactive(),
        4 => stop_container_interactive(),
        5 => restart_container_interactive(),
        6 => container_snapshot_menu(),
        7 => clone_container_interactive(),
        8 => enter_container_interactive(),
        9 => delete_container_interactive(),
        _ => return,
    }
}

pub fn list_containers() {
    println!("📋 Listing Containers");
    println!("=====================");
    
    let output = Command::new("pct")
        .arg("list")
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                let output_str = String::from_utf8_lossy(&result.stdout);
                println!("{}", output_str);
            } else {
                let error_str = String::from_utf8_lossy(&result.stderr);
                println!("❌ Error listing containers: {}", error_str);
            }
        },
        Err(e) => {
            println!("❌ Failed to execute pct command: {}", e);
            println!("💡 Make sure you're running this on a Proxmox VE host");
        }
    }
}

pub fn create_container_wizard() {
    println!("🆕 Create Container Wizard");
    println!("==========================");
    
    let ct_id: String = Input::new()
        .with_prompt("Container ID (100-999999)")
        .default("200")
        .interact_text()
        .unwrap();
    
    let ct_name: String = Input::new()
        .with_prompt("Container hostname")
        .default(&format!("ct-{}", ct_id))
        .interact_text()
        .unwrap();
    
    let templates = [
        "ubuntu-22.04-standard",
        "ubuntu-20.04-standard",
        "debian-12-standard", 
        "centos-9-stream-default",
        "alpine-3.18-default",
        "Custom template",
    ];
    
    let template_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Container template")
        .items(&templates)
        .default(0)
        .interact()
        .unwrap();
    
    let template = if template_choice == templates.len() - 1 {
        Input::new()
            .with_prompt("Custom template name")
            .interact_text()
            .unwrap()
    } else {
        templates[template_choice].to_string()
    };
    
    let memory: String = Input::new()
        .with_prompt("Memory (MB)")
        .default("1024")
        .interact_text()
        .unwrap();
    
    let cores: String = Input::new()
        .with_prompt("CPU Cores")
        .default("1")
        .interact_text()
        .unwrap();
    
    let disk_size: String = Input::new()
        .with_prompt("Root filesystem size (GB)")
        .default("8")
        .interact_text()
        .unwrap();
    
    let password: String = Input::new()
        .with_prompt("Root password")
        .default("changeme")
        .interact_text()
        .unwrap();
    
    println!("🏗️  Creating container...");
    
    let status = Command::new("pct")
        .args(&["create", &ct_id,
               &format!("local:vztmpl/{}.tar.xz", template),
               "--hostname", &ct_name,
               "--memory", &memory,
               "--cores", &cores,
               "--rootfs", &format!("local-lvm:{}", disk_size),
               "--net0", "name=eth0,bridge=vmbr0,ip=dhcp",
               "--password", &password,
               "--unprivileged", "1",
               "--features", "keyctl=1,nesting=1"])
        .status();
    
    if status.is_ok() && status.unwrap().success() {
        println!("✅ Container {} created successfully", ct_name);
        
        let start_ct = Confirm::new()
            .with_prompt("Start container now?")
            .default(true)
            .interact()
            .unwrap();
        
        if start_ct {
            start_container(&ct_id);
        }
    } else {
        println!("❌ Failed to create container");
    }
}

pub fn start_container(ct_id: &str) {
    println!("▶️  Starting container {}...", ct_id);
    
    let status = Command::new("pct")
        .args(&["start", ct_id])
        .status();
    
    match status {
        Ok(s) if s.success() => println!("✅ Container {} started successfully", ct_id),
        _ => println!("❌ Failed to start container {}", ct_id),
    }
}

pub fn stop_container(ct_id: &str) {
    println!("⏹️  Stopping container {}...", ct_id);
    
    let status = Command::new("pct")
        .args(&["stop", ct_id])
        .status();
    
    match status {
        Ok(s) if s.success() => println!("✅ Container {} stopped successfully", ct_id),
        _ => println!("❌ Failed to stop container {}", ct_id),
    }
}

fn start_container_interactive() {
    let ct_id: String = Input::new()
        .with_prompt("Container ID to start")
        .interact_text()
        .unwrap();
    
    start_container(&ct_id);
}

fn stop_container_interactive() {
    let ct_id: String = Input::new()
        .with_prompt("Container ID to stop")
        .interact_text()
        .unwrap();
    
    stop_container(&ct_id);
}

fn restart_container_interactive() {
    let ct_id: String = Input::new()
        .with_prompt("Container ID to restart")
        .interact_text()
        .unwrap();
    
    println!("🔄 Restarting container {}...", ct_id);
    
    let status = Command::new("pct")
        .args(&["restart", &ct_id])
        .status();
    
    match status {
        Ok(s) if s.success() => println!("✅ Container {} restarted successfully", ct_id),
        _ => println!("❌ Failed to restart container {}", ct_id),
    }
}

fn show_container_status() {
    list_containers();
    
    let ct_id: String = Input::new()
        .with_prompt("Container ID for detailed status")
        .interact_text()
        .unwrap();
    
    println!("📊 Container {} Status:", ct_id);
    
    let _ = Command::new("pct")
        .args(&["status", &ct_id])
        .status();
    
    let _ = Command::new("pct")
        .args(&["config", &ct_id])
        .status();
}

fn enter_container_interactive() {
    let ct_id: String = Input::new()
        .with_prompt("Container ID to enter")
        .interact_text()
        .unwrap();
    
    println!("🚪 Entering container {}...", ct_id);
    println!("💡 Type 'exit' to return to GhostCTL");
    
    let _ = Command::new("pct")
        .args(&["enter", &ct_id])
        .status();
}

// Placeholder functions for other menus
fn storage_management_menu() {
    println!("💾 Storage Management - Coming soon!");
}

fn network_configuration_menu() {
    println!("🌐 Network Configuration - Coming soon!");
}

fn backup_recovery_menu() {
    println!("📊 Backup & Recovery - Coming soon!");
}

fn template_management_menu() {
    println!("🏗️  Template Management - Coming soon!");
}

fn monitoring_status_menu() {
    println!("📈 Monitoring & Status - Coming soon!");
}

fn cluster_management_menu() {
    println!("🔧 Cluster Management - Coming soon!");
}

fn homelab_automation_menu() {
    println!("🚀 Homelab Automation - Coming soon!");
}

// Snapshot menus (placeholders)
fn vm_snapshot_menu() {
    println!("📸 VM Snapshots - Coming soon!");
}

fn container_snapshot_menu() {
    println!("📸 Container Snapshots - Coming soon!");
}

// Clone functions (placeholders)
fn clone_vm_interactive() {
    println!("🏗️  VM Cloning - Coming soon!");
}

fn clone_container_interactive() {
    println!("🏗️  Container Cloning - Coming soon!");
}

// Migration functions (placeholders)
fn migrate_vm_interactive() {
    println!("🚚 VM Migration - Coming soon!");
}

// Delete functions (placeholders)
fn delete_vm_interactive() {
    println!("🗑️  VM Deletion - Coming soon!");
}

fn delete_container_interactive() {
    println!("🗑️  Container Deletion - Coming soon!");
}

pub fn show_pve_status() {
    println!("📊 Proxmox VE Status");
    println!("====================");
    
    // Check if we're on a PVE system
    if !std::path::Path::new("/usr/bin/qm").exists() {
        println!("❌ This doesn't appear to be a Proxmox VE system");
        println!("💡 Install Proxmox VE or run this on a PVE host");
        return;
    }
    
    println!("🖥️  Node Status:");
    let _ = Command::new("pvesh")
        .args(&["get", &format!("/nodes/{}/status", hostname())])
        .status();
    
    println!("\n📦 VM Summary:");
    list_vms();
    
    println!("\n📦 Container Summary:");
    list_containers();
    
    println!("\n💾 Storage Status:");
    let _ = Command::new("pvesm")
        .args(&["status"])
        .status();
}

fn hostname() -> String {
    std::fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| "localhost".to_string())
        .trim()
        .to_string()
}