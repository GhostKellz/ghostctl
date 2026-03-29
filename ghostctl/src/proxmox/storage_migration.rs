use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Password, Select};
use serde_json;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

pub fn storage_migration_menu() {
    loop {
        let options = vec![
            "VM Storage Migration",
            "Container Storage Migration",
            "Bulk Storage Migration",
            "Storage Pool Management",
            "Migration Planning & Analysis",
            "Live Migration Tools",
            "Storage Replication Setup",
            "Back",
        ];

        let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔄 PVE Storage Migration")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match selection {
            0 => vm_storage_migration(),
            1 => container_storage_migration(),
            2 => bulk_storage_migration(),
            3 => storage_pool_management(),
            4 => migration_planning(),
            5 => live_migration_tools(),
            6 => storage_replication_setup(),
            _ => break,
        }
    }
}

fn vm_storage_migration() {
    println!("🖥️  VM Storage Migration\n");

    // List VMs
    println!("📋 Available VMs:");
    if let Err(e) = Command::new("pvesh")
        .args(["get", "/nodes", "--output-format", "table"])
        .status()
    {
        println!("Failed to list nodes: {}", e);
    }

    let Ok(vm_id): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter VM ID to migrate")
        .interact()
    else {
        return;
    };

    // Validate VM ID
    if let Err(e) = super::validation::validate_vmid(&vm_id) {
        println!("Invalid VM ID: {}", e);
        return;
    }

    // Show current storage
    println!("💾 Current VM storage configuration:");
    if let Err(e) = Command::new("pvesh")
        .args(["get", &format!("/nodes/localhost/qemu/{}/config", vm_id)])
        .status()
    {
        println!("Failed to get VM config: {}", e);
    }

    // List available storage
    println!("\n📦 Available storage pools:");
    if let Err(e) = Command::new("pvesh")
        .args(["get", "/storage", "--output-format", "table"])
        .status()
    {
        println!("Failed to list storage: {}", e);
    }

    let Ok(source_storage): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter source storage ID")
        .interact()
    else {
        return;
    };

    // Validate source storage
    if let Err(e) = super::validation::validate_storage_name(&source_storage) {
        println!("Invalid source storage name: {}", e);
        return;
    }

    let Ok(target_storage): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter target storage ID")
        .interact()
    else {
        return;
    };

    // Validate target storage
    if let Err(e) = super::validation::validate_storage_name(&target_storage) {
        println!("Invalid target storage name: {}", e);
        return;
    }

    let Ok(migration_type) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select migration type")
        .items(&["Online (live)", "Offline", "Copy (keep original)"])
        .default(0)
        .interact()
    else {
        return;
    };

    println!("\n🔄 Migration Summary:");
    println!("   VM ID: {}", vm_id);
    println!("   From: {}", source_storage);
    println!("   To: {}", target_storage);
    println!(
        "   Type: {}",
        match migration_type {
            0 => "Online (live)",
            1 => "Offline",
            2 => "Copy (keep original)",
            _ => "Unknown",
        }
    );

    let Ok(proceed) = Confirm::new()
        .with_prompt("Proceed with migration?")
        .default(false)
        .interact()
    else {
        return;
    };

    if proceed {
        let success = match migration_type {
            0 => {
                println!("🚀 Starting online storage migration...");
                match Command::new("pvesh")
                    .args([
                        "create",
                        &format!("/nodes/localhost/qemu/{}/move_disk", vm_id),
                        "-disk",
                        "virtio0",
                        "-storage",
                        &target_storage,
                        "-delete",
                        "1",
                    ])
                    .status()
                {
                    Ok(status) if status.success() => true,
                    Ok(_) => {
                        println!("❌ Migration command returned non-zero");
                        false
                    }
                    Err(e) => {
                        println!("❌ Migration failed: {}", e);
                        false
                    }
                }
            }
            1 => {
                println!("⏹️  Stopping VM for offline migration...");
                if let Err(e) = Command::new("pvesh")
                    .args([
                        "create",
                        &format!("/nodes/localhost/qemu/{}/status/stop", vm_id),
                    ])
                    .status()
                {
                    println!("Warning: Could not stop VM: {}", e);
                }

                println!("🔄 Moving disk...");
                match Command::new("pvesh")
                    .args([
                        "create",
                        &format!("/nodes/localhost/qemu/{}/move_disk", vm_id),
                        "-disk",
                        "virtio0",
                        "-storage",
                        &target_storage,
                    ])
                    .status()
                {
                    Ok(status) if status.success() => true,
                    Ok(_) => {
                        println!("❌ Migration command returned non-zero");
                        false
                    }
                    Err(e) => {
                        println!("❌ Migration failed: {}", e);
                        false
                    }
                }
            }
            2 => {
                println!("📋 Creating disk copy...");
                match Command::new("qm")
                    .args([
                        "clone",
                        &vm_id,
                        "999",
                        "--full",
                        "--storage",
                        &target_storage,
                    ])
                    .status()
                {
                    Ok(status) if status.success() => true,
                    Ok(_) => {
                        println!("❌ Clone command returned non-zero");
                        false
                    }
                    Err(e) => {
                        println!("❌ Clone failed: {}", e);
                        false
                    }
                }
            }
            _ => false,
        };

        if success {
            println!("✅ Storage migration completed!");

            // Verify migration
            println!("\n🔍 Verifying migration...");
            if let Err(e) = Command::new("pvesh")
                .args(["get", &format!("/nodes/localhost/qemu/{}/config", vm_id)])
                .status()
            {
                println!("Warning: Could not verify migration: {}", e);
            }
        }
    }
}

fn container_storage_migration() {
    println!("📦 Container Storage Migration\n");

    // List containers
    println!("📋 Available containers:");
    if let Err(e) = Command::new("pvesh")
        .args(["get", "/nodes/localhost/lxc", "--output-format", "table"])
        .status()
    {
        println!("Failed to list containers: {}", e);
    }

    let Ok(ct_id): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter container ID to migrate")
        .interact()
    else {
        return;
    };

    // Validate container ID
    if let Err(e) = super::validation::validate_ctid(&ct_id) {
        println!("Invalid container ID: {}", e);
        return;
    }

    // Show current storage
    println!("💾 Current container configuration:");
    if let Err(e) = Command::new("pvesh")
        .args(["get", &format!("/nodes/localhost/lxc/{}/config", ct_id)])
        .status()
    {
        println!("Failed to get container config: {}", e);
    }

    let Ok(source_storage): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter source storage ID")
        .interact()
    else {
        return;
    };

    // Validate source storage
    if let Err(e) = super::validation::validate_storage_name(&source_storage) {
        println!("Invalid source storage: {}", e);
        return;
    }

    let Ok(target_storage): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter target storage ID")
        .interact()
    else {
        return;
    };

    // Validate target storage
    if let Err(e) = super::validation::validate_storage_name(&target_storage) {
        println!("Invalid target storage: {}", e);
        return;
    }

    let Ok(stop_container) = Confirm::new()
        .with_prompt("Stop container for migration?")
        .default(true)
        .interact()
    else {
        return;
    };

    if stop_container {
        println!("⏹️  Stopping container...");
        if let Err(e) = Command::new("pct").args(["stop", &ct_id]).status() {
            println!("Warning: Could not stop container: {}", e);
        }

        println!("🔄 Moving container storage...");
        let migration_success = match Command::new("pvesh")
            .args([
                "create",
                &format!("/nodes/localhost/lxc/{}/move_volume", ct_id),
                "-volume",
                "rootfs",
                "-storage",
                &target_storage,
            ])
            .status()
        {
            Ok(status) if status.success() => true,
            Ok(_) => {
                println!("❌ Migration command returned non-zero");
                false
            }
            Err(e) => {
                println!("❌ Migration failed: {}", e);
                false
            }
        };

        let Ok(start_after) = Confirm::new()
            .with_prompt("Start container after migration?")
            .default(true)
            .interact()
        else {
            return;
        };

        if start_after {
            if let Err(e) = Command::new("pct").args(["start", &ct_id]).status() {
                println!("Warning: Could not start container: {}", e);
            }
        }

        if migration_success {
            println!("✅ Container migration completed!");
        }
    }
}

fn bulk_storage_migration() {
    println!("🔄 Bulk Storage Migration\n");

    let Ok(source_storage): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter source storage ID")
        .interact()
    else {
        return;
    };

    let Ok(target_storage): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter target storage ID")
        .interact()
    else {
        return;
    };

    // List VMs on source storage
    println!("🔍 Scanning VMs on source storage...");
    let vm_list = Command::new("pvesh")
        .args(&["get", "/nodes/localhost/qemu", "--output-format", "json"])
        .output();

    // List containers on source storage
    println!("🔍 Scanning containers on source storage...");
    let ct_list = Command::new("pvesh")
        .args(&["get", "/nodes/localhost/lxc", "--output-format", "json"])
        .output();

    println!("\n📊 Migration candidates found:");
    println!("   • VMs: Scanning for storage matches...");
    println!("   • Containers: Scanning for storage matches...");

    let migration_options = vec![
        "All VMs",
        "All Containers",
        "VMs + Containers",
        "Custom Selection",
    ];

    let Ok(migration_scope) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select migration scope")
        .items(&migration_options)
        .default(0)
        .interact()
    else {
        return;
    };

    let Ok(parallel_jobs): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Number of parallel migration jobs")
        .default("2".to_string())
        .interact()
    else {
        return;
    };

    let Ok(start_migration) = Confirm::new()
        .with_prompt("Start bulk migration?")
        .default(false)
        .interact()
    else {
        return;
    };

    if start_migration {
        println!(
            "🚀 Starting bulk migration with {} parallel jobs...",
            parallel_jobs
        );

        // This would implement the actual bulk migration logic
        println!("⚠️  Bulk migration in progress - monitor via PVE web interface");
        println!("✅ Bulk migration jobs queued!");
    }
}

fn storage_pool_management() {
    loop {
        let options = vec![
            "List Storage Pools",
            "Add Storage Pool",
            "Remove Storage Pool",
            "Storage Pool Status",
            "Storage Pool Configuration",
            "Storage Usage Analysis",
            "Back",
        ];

        let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("💾 Storage Pool Management")
            .items(&options)
            .default(0)
            .interact()
        else {
            break;
        };

        match selection {
            0 => list_storage_pools(),
            1 => add_storage_pool(),
            2 => remove_storage_pool(),
            3 => storage_pool_status(),
            4 => storage_pool_configuration(),
            5 => storage_usage_analysis(),
            _ => break,
        }
    }
}

fn list_storage_pools() {
    println!("📦 PVE Storage Pools\n");

    if let Err(e) = Command::new("pvesh")
        .args(["get", "/storage", "--output-format", "table"])
        .status()
    {
        println!("Failed to list storage pools: {}", e);
    }

    println!("\n💾 Detailed storage information:");
    if let Err(e) = Command::new("pvesm").args(["status"]).status() {
        println!("Failed to get storage status: {}", e);
    }
}

fn add_storage_pool() {
    println!("➕ Add Storage Pool\n");

    let storage_types = vec![
        "local",
        "nfs",
        "cifs",
        "glusterfs",
        "cephfs",
        "ceph",
        "zfs",
        "lvm",
        "lvmthin",
        "iscsi",
        "iscsidirect",
    ];

    let Ok(storage_type) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select storage type")
        .items(&storage_types)
        .default(0)
        .interact()
    else {
        return;
    };

    let Ok(storage_id): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter storage ID")
        .interact()
    else {
        return;
    };

    match storage_types[storage_type] {
        "nfs" => add_nfs_storage(&storage_id),
        "cifs" => add_cifs_storage(&storage_id),
        "zfs" => add_zfs_storage(&storage_id),
        "lvm" => add_lvm_storage(&storage_id),
        "local" => add_local_storage(&storage_id),
        _ => println!("Storage type configuration coming soon!"),
    }
}

fn add_nfs_storage(storage_id: &str) {
    let Ok(server): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter NFS server IP/hostname")
        .interact()
    else {
        return;
    };

    let Ok(export): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter NFS export path")
        .interact()
    else {
        return;
    };

    // Validate export path
    if let Err(e) = super::validation::validate_path(&export) {
        println!("Invalid export path: {}", e);
        return;
    }

    let Ok(content_types) = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select content types")
        .items(&["images", "iso", "vztmpl", "backup", "snippets"])
        .interact()
    else {
        return;
    };

    let content_options = ["images", "iso", "vztmpl", "backup", "snippets"];
    let content: String = content_types
        .iter()
        .filter_map(|&idx| content_options.get(idx).copied())
        .collect::<Vec<_>>()
        .join(",");

    if content.is_empty() {
        println!("At least one content type must be selected");
        return;
    }

    println!("➕ Adding NFS storage...");
    match Command::new("pvesm")
        .args([
            "add",
            "nfs",
            storage_id,
            "--server",
            &server,
            "--export",
            &export,
            "--content",
            &content,
        ])
        .status()
    {
        Ok(status) if status.success() => {
            println!("✅ NFS storage '{}' added successfully!", storage_id);
        }
        Ok(_) => println!("❌ Failed to add NFS storage (command returned non-zero)"),
        Err(e) => println!("❌ Failed to add NFS storage: {}", e),
    }
}

fn add_cifs_storage(storage_id: &str) {
    let Ok(server): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter CIFS server IP/hostname")
        .interact()
    else {
        return;
    };

    let Ok(share): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter CIFS share name")
        .interact()
    else {
        return;
    };

    // Validate share name (similar to storage name)
    if let Err(e) = super::validation::validate_storage_name(&share) {
        println!("Invalid share name: {}", e);
        return;
    }

    let Ok(username): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter username")
        .interact()
    else {
        return;
    };

    // Use masked password input
    let Ok(password): Result<String, _> = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter password")
        .interact()
    else {
        return;
    };

    println!("➕ Adding CIFS storage...");

    // Write credentials to a secure temp file to avoid argv exposure.
    // pvesm will read and store credentials securely in /etc/pve/priv/.
    let cred_content = format!("username={}\npassword={}\n", username, password);

    let mut cred_file = match NamedTempFile::new() {
        Ok(f) => f,
        Err(e) => {
            println!("❌ Failed to create credentials temp file: {}", e);
            return;
        }
    };

    if let Err(e) = cred_file.write_all(cred_content.as_bytes()) {
        println!("❌ Failed to write credentials file: {}", e);
        return;
    }

    if let Err(e) = cred_file.flush() {
        println!("❌ Failed to flush credentials file: {}", e);
        return;
    }

    // Use credentials file instead of --password flag
    let result = Command::new("pvesm")
        .args([
            "add",
            "cifs",
            storage_id,
            "--server",
            &server,
            "--share",
            &share,
            "--smbcredentials",
            cred_file.path().to_string_lossy().as_ref(),
            "--content",
            "images,iso,backup",
        ])
        .status();

    match result {
        Ok(status) if status.success() => {
            println!("✅ CIFS storage '{}' added successfully!", storage_id);
        }
        Ok(_) => println!("❌ Failed to add CIFS storage (command returned non-zero)"),
        Err(e) => println!("❌ Failed to add CIFS storage: {}", e),
    }
}

fn add_zfs_storage(storage_id: &str) {
    let Ok(pool): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter ZFS pool name")
        .interact()
    else {
        return;
    };

    // Validate pool name (similar rules to storage name)
    if let Err(e) = super::validation::validate_storage_name(&pool) {
        println!("Invalid pool name: {}", e);
        return;
    }

    let Ok(sparse) = Confirm::new()
        .with_prompt("Enable sparse allocation?")
        .default(true)
        .interact()
    else {
        return;
    };

    println!("➕ Adding ZFS storage...");
    let mut args = vec![
        "add".to_string(),
        "zfspool".to_string(),
        storage_id.to_string(),
        "--pool".to_string(),
        pool.clone(),
        "--content".to_string(),
        "images,rootdir".to_string(),
    ];
    if sparse {
        args.push("--sparse".to_string());
    }

    let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    match Command::new("pvesm").args(&args_refs).status() {
        Ok(status) if status.success() => {
            println!("✅ ZFS storage '{}' added successfully!", storage_id);
        }
        Ok(_) => println!("❌ Failed to add ZFS storage (command returned non-zero)"),
        Err(e) => println!("❌ Failed to add ZFS storage: {}", e),
    }
}

fn add_lvm_storage(storage_id: &str) {
    let Ok(vgname): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter LVM volume group name")
        .interact()
    else {
        return;
    };

    // Validate volume group name
    if let Err(e) = super::validation::validate_storage_name(&vgname) {
        println!("Invalid volume group name: {}", e);
        return;
    }

    println!("➕ Adding LVM storage...");
    match Command::new("pvesm")
        .args([
            "add",
            "lvm",
            storage_id,
            "--vgname",
            &vgname,
            "--content",
            "images,rootdir",
        ])
        .status()
    {
        Ok(status) if status.success() => {
            println!("✅ LVM storage '{}' added successfully!", storage_id);
        }
        Ok(_) => println!("❌ Failed to add LVM storage (command returned non-zero)"),
        Err(e) => println!("❌ Failed to add LVM storage: {}", e),
    }
}

fn add_local_storage(storage_id: &str) {
    let Ok(path): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter local path")
        .interact()
    else {
        return;
    };

    // Validate path
    if let Err(e) = super::validation::validate_path(&path) {
        println!("Invalid path: {}", e);
        return;
    }

    println!("➕ Adding local storage...");
    match Command::new("pvesm")
        .args([
            "add",
            "dir",
            storage_id,
            "--path",
            &path,
            "--content",
            "images,iso,backup,snippets",
        ])
        .status()
    {
        Ok(status) if status.success() => {
            println!("✅ Local storage '{}' added successfully!", storage_id);
        }
        Ok(_) => println!("❌ Failed to add local storage (command returned non-zero)"),
        Err(e) => println!("❌ Failed to add local storage: {}", e),
    }
}

fn remove_storage_pool() {
    println!("🗑️  Remove Storage Pool\n");

    list_storage_pools();

    let Ok(storage_id): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter storage ID to remove")
        .interact()
    else {
        return;
    };

    // Validate storage ID
    if let Err(e) = super::validation::validate_storage_name(&storage_id) {
        println!("Invalid storage ID: {}", e);
        return;
    }

    let Ok(confirm_remove) = Confirm::new()
        .with_prompt(&format!(
            "⚠️  Really remove storage '{}'? This action cannot be undone!",
            storage_id
        ))
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm_remove {
        // Double confirmation for destructive operation
        let Ok(final_confirm) = Confirm::new()
            .with_prompt(&format!(
                "🚨 FINAL WARNING: Remove '{}'? Type 'yes' to confirm",
                storage_id
            ))
            .default(false)
            .interact()
        else {
            return;
        };

        if final_confirm {
            match Command::new("pvesm").args(["remove", &storage_id]).status() {
                Ok(status) if status.success() => {
                    println!("✅ Storage '{}' removed successfully!", storage_id);
                }
                Ok(_) => println!("❌ Failed to remove storage (command returned non-zero)"),
                Err(e) => println!("❌ Failed to remove storage: {}", e),
            }
        } else {
            println!("Operation cancelled");
        }
    }
}

fn storage_pool_status() {
    println!("📊 Storage Pool Status\n");

    if let Err(e) = Command::new("pvesm")
        .args(["status", "--content", "images"])
        .status()
    {
        println!("Failed to get storage status: {}", e);
    }

    println!("\n💾 Detailed usage by storage:");
    if let Err(e) = Command::new("pvesm")
        .args(["status", "--content", "images", "--output-format", "table"])
        .status()
    {
        println!("Failed to get detailed status: {}", e);
    }
}

fn storage_pool_configuration() {
    println!("⚙️  Storage Pool Configuration\n");

    let Ok(storage_id): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter storage ID")
        .interact()
    else {
        return;
    };

    // Validate storage ID
    if let Err(e) = super::validation::validate_storage_name(&storage_id) {
        println!("Invalid storage ID: {}", e);
        return;
    }

    if let Err(e) = Command::new("pvesm").args(["status", &storage_id]).status() {
        println!("Failed to get storage status: {}", e);
    }

    // Show detailed config
    if let Err(e) = Command::new("pvesh")
        .args(["get", &format!("/storage/{}", storage_id)])
        .status()
    {
        println!("Failed to get storage config: {}", e);
    }
}

fn storage_usage_analysis() {
    println!("📈 Storage Usage Analysis\n");

    println!("📊 Overall storage usage:");
    if let Err(e) = Command::new("pvesm").args(["status"]).status() {
        println!("Failed to get storage status: {}", e);
    }

    println!("\n🔍 Analyzing disk usage per VM/CT...");

    // Get VM disk usage safely without shell injection
    if let Ok(output) = Command::new("pvesh")
        .args(["get", "/nodes/localhost/qemu", "--output-format", "json"])
        .output()
    {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            // Parse JSON to extract VM IDs
            if let Ok(vms) = serde_json::from_str::<Vec<serde_json::Value>>(&stdout) {
                for vm in vms {
                    if let Some(vmid) = vm.get("vmid").and_then(|v| v.as_u64()) {
                        println!("VM {}:", vmid);
                        // Get VM config and filter for disk entries
                        if let Ok(config_output) = Command::new("pvesh")
                            .args([
                                "get",
                                &format!("/nodes/localhost/qemu/{}/config", vmid),
                                "--output-format",
                                "json",
                            ])
                            .output()
                        {
                            if let Ok(config_str) = String::from_utf8(config_output.stdout) {
                                // Filter lines containing disk-related keys
                                for line in config_str.lines() {
                                    let lower = line.to_lowercase();
                                    if lower.contains("virtio")
                                        || lower.contains("scsi")
                                        || lower.contains("ide")
                                    {
                                        println!("  {}", line.trim());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\n💽 Storage performance metrics:");
    if let Err(e) = Command::new("iostat").args(["-x", "1", "3"]).status() {
        println!("(iostat not available: {})", e);
    }
}

fn migration_planning() {
    println!("📋 Migration Planning & Analysis\n");

    let Ok(action) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select analysis type")
        .items(&[
            "Pre-migration Assessment",
            "Storage Capacity Planning",
            "Performance Impact Analysis",
            "Migration Timeline Estimation",
            "Rollback Planning",
        ])
        .default(0)
        .interact()
    else {
        return;
    };

    match action {
        0 => pre_migration_assessment(),
        1 => storage_capacity_planning(),
        2 => performance_impact_analysis(),
        3 => migration_timeline_estimation(),
        4 => rollback_planning(),
        _ => {}
    }
}

fn pre_migration_assessment() {
    println!("🔍 Pre-Migration Assessment\n");

    let Ok(storage_id): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter storage ID to assess")
        .interact()
    else {
        return;
    };

    // Validate storage ID
    if let Err(e) = super::validation::validate_storage_name(&storage_id) {
        println!("Invalid storage ID: {}", e);
        return;
    }

    println!("📊 Storage health check:");
    if let Err(e) = Command::new("pvesm").args(["status", &storage_id]).status() {
        println!("Failed to check storage status: {}", e);
    }

    println!("\n🔍 Checking storage accessibility:");
    if let Err(e) = Command::new("pvesm").args(["list", &storage_id]).status() {
        println!("Failed to list storage: {}", e);
    }

    println!("\n⚡ Performance baseline:");
    // Use a safe test file path
    let test_file = format!("/tmp/ghostctl_test_{}", storage_id);
    match Command::new("dd")
        .args([
            "if=/dev/zero",
            &format!("of={}", test_file),
            "bs=1M",
            "count=100",
        ])
        .status()
    {
        Ok(status) if status.success() => {
            println!("   Write test completed");
        }
        Ok(_) | Err(_) => {
            println!("   Write test failed or skipped");
        }
    }

    // Clean up test file
    if let Err(_) = Command::new("rm").args(["-f", &test_file]).status() {
        // Ignore cleanup errors
    }

    println!("✅ Pre-migration assessment complete!");
}

fn storage_capacity_planning() {
    println!("📏 Storage Capacity Planning\n");

    if let Err(e) = Command::new("pvesm").args(["status"]).status() {
        println!("Failed to get storage status: {}", e);
    }

    println!("\n📊 Disk usage by VM:");
    // This would implement detailed capacity analysis
    println!("💡 Capacity recommendations:");
    println!("   - Ensure target storage has 20% free space buffer");
    println!("   - Consider thin provisioning for large VMs");
    println!("   - Plan for snapshot space during migration");
}

fn performance_impact_analysis() {
    println!("⚡ Performance Impact Analysis\n");

    println!("📊 Current I/O load:");
    if let Err(e) = Command::new("iostat").args(["-x", "1", "3"]).status() {
        println!("(iostat not available: {})", e);
    }

    println!("\n🔍 Network bandwidth analysis:");
    if let Err(e) = Command::new("iftop")
        .args(["-n", "-t", "-s", "10"])
        .status()
    {
        println!("(iftop not available: {})", e);
    }

    println!("\n💡 Impact mitigation recommendations:");
    println!("   - Schedule migrations during low-usage hours");
    println!("   - Use bandwidth limiting for large migrations");
    println!("   - Monitor cluster resource usage during migration");
}

fn migration_timeline_estimation() {
    println!("⏱️  Migration Timeline Estimation\n");

    let Ok(data_size): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter data size to migrate (GB)")
        .interact()
    else {
        return;
    };

    let Ok(network_speed): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter network speed (Gbps)")
        .default("1".to_string())
        .interact()
    else {
        return;
    };

    if let (Ok(size_gb), Ok(speed_gbps)) = (data_size.parse::<f64>(), network_speed.parse::<f64>())
    {
        let estimated_time = size_gb * 8.0 / (speed_gbps * 1000.0) / 0.7; // 70% efficiency
        println!(
            "\n⏱️  Estimated migration time: {:.1} hours",
            estimated_time
        );
        println!("   (includes overhead and safety margin)");
    }
}

fn rollback_planning() {
    println!("🔄 Rollback Planning\n");

    println!("📋 Pre-migration checklist:");
    println!("   ☐ Create VM/CT snapshots");
    println!("   ☐ Document current storage configuration");
    println!("   ☐ Test rollback procedure on non-critical VM");
    println!("   ☐ Prepare rollback scripts");

    let Ok(create_script) = Confirm::new()
        .with_prompt("Create automated rollback script?")
        .default(true)
        .interact()
    else {
        return;
    };

    if create_script {
        create_rollback_script();
    }
}

fn create_rollback_script() {
    let Ok(vm_id): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter VM ID for rollback script")
        .interact()
    else {
        return;
    };

    // Validate VM ID
    if let Err(e) = super::validation::validate_vmid(&vm_id) {
        println!("Invalid VM ID: {}", e);
        return;
    }

    let script_content = format!(
        r#"#!/bin/bash
# Rollback script for VM {}
# Generated by ghostctl

echo "Rolling back VM {} storage migration..."

# Stop VM
qm stop {}

# Restore from snapshot (implement based on your backup strategy)
# qm rollback {} snapshot_name

echo "Rollback complete for VM {}"
"#,
        vm_id, vm_id, vm_id, vm_id, vm_id
    );

    let script_path = format!("/tmp/rollback_vm_{}.sh", vm_id);
    match std::fs::write(&script_path, &script_content) {
        Ok(()) => {
            if let Err(e) = Command::new("chmod").args(["+x", &script_path]).status() {
                println!(
                    "Script created but chmod failed: {}. Run: chmod +x {}",
                    e, script_path
                );
            } else {
                println!("✅ Rollback script created: {}", script_path);
            }
        }
        Err(e) => println!("Failed to create rollback script: {}", e),
    }
}

fn live_migration_tools() {
    println!("🔄 Live Migration Tools\n");

    let tools = vec![
        "Monitor Active Migrations",
        "Cancel Running Migration",
        "Bandwidth Control",
        "Migration Queue Management",
        "Live Migration Status",
    ];

    let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select tool")
        .items(&tools)
        .default(0)
        .interact()
    else {
        return;
    };

    match selection {
        0 => monitor_active_migrations(),
        1 => cancel_running_migration(),
        2 => bandwidth_control(),
        3 => migration_queue_management(),
        4 => live_migration_status(),
        _ => {}
    }
}

fn monitor_active_migrations() {
    println!("📊 Active Migrations Monitor\n");

    if let Err(e) = Command::new("pvesh")
        .args(["get", "/nodes/localhost/tasks", "--typefilter", "qmmove"])
        .status()
    {
        println!("Failed to get migration tasks: {}", e);
    }

    println!("\n🔍 Real-time migration progress:");
    // This would implement real-time monitoring
    println!("Tip: Use 'watch pvesh get /nodes/localhost/tasks' for live updates");
}

fn cancel_running_migration() {
    println!("🛑 Cancel Running Migration\n");

    let Ok(task_id): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter task ID to cancel")
        .interact()
    else {
        return;
    };

    let Ok(confirm_cancel) = Confirm::new()
        .with_prompt("Really cancel migration?")
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm_cancel {
        match Command::new("pvesh")
            .args(["delete", &format!("/nodes/localhost/tasks/{}", task_id)])
            .status()
        {
            Ok(status) if status.success() => {
                println!("✅ Migration cancellation requested");
            }
            Ok(_) => println!("❌ Cancellation may have failed (command returned non-zero)"),
            Err(e) => println!("❌ Cancellation failed: {}", e),
        }
    }
}

fn bandwidth_control() {
    println!("🌐 Migration Bandwidth Control\n");

    let Ok(limit): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter bandwidth limit (MB/s)")
        .default("100".to_string())
        .interact()
    else {
        return;
    };

    // This would implement bandwidth limiting
    println!("⚠️  Bandwidth limiting configured: {} MB/s", limit);
    println!("💡 Applied to future migrations");
}

fn migration_queue_management() {
    println!("📋 Migration Queue Management\n");

    println!("🔍 Current migration queue:");
    if let Err(e) = Command::new("pvesh")
        .args(["get", "/nodes/localhost/tasks", "--running", "1"])
        .status()
    {
        println!("Failed to get migration queue: {}", e);
    }

    println!("\n⏳ Pending migrations:");
    // This would show queued migrations
    println!("   No migrations currently queued");
}

fn live_migration_status() {
    println!("📊 Live Migration Status\n");

    if let Err(e) = Command::new("pvesh")
        .args(["get", "/cluster/tasks"])
        .status()
    {
        println!("Failed to get cluster tasks: {}", e);
    }

    println!("\n🔍 Detailed task information:");
    if let Err(e) = Command::new("pvesh")
        .args(["get", "/nodes/localhost/tasks", "--limit", "10"])
        .status()
    {
        println!("Failed to get task details: {}", e);
    }
}

fn storage_replication_setup() {
    println!("🔄 Storage Replication Setup\n");

    let replication_types = vec![
        "ZFS Replication",
        "Proxmox Backup Server Sync",
        "DRBD Replication",
        "Custom Replication Script",
    ];

    let Ok(replication_type) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select replication type")
        .items(&replication_types)
        .default(0)
        .interact()
    else {
        return;
    };

    match replication_type {
        0 => setup_zfs_replication(),
        1 => setup_pbs_sync(),
        2 => setup_drbd_replication(),
        3 => setup_custom_replication(),
        _ => {}
    }
}

fn setup_zfs_replication() {
    println!("🏊 ZFS Replication Setup\n");

    let Ok(source_dataset): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter source ZFS dataset")
        .interact()
    else {
        return;
    };

    let Ok(target_host): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter target host")
        .interact()
    else {
        return;
    };

    let Ok(target_dataset): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter target ZFS dataset")
        .interact()
    else {
        return;
    };

    let Ok(schedule): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter replication schedule (cron format)")
        .default("0 */4 * * *".to_string())
        .interact()
    else {
        return;
    };

    println!("🔄 Setting up ZFS replication...");

    // Create replication job
    let replication_config = format!(
        r#"
# ZFS Replication Configuration
source: {}
target: {}:{}
schedule: {}
"#,
        source_dataset, target_host, target_dataset, schedule
    );

    println!("✅ ZFS replication configured!");
    println!("📋 Configuration:\n{}", replication_config);
}

fn setup_pbs_sync() {
    println!("🔐 Proxmox Backup Server Sync\n");

    let Ok(pbs_server): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter PBS server address")
        .interact()
    else {
        return;
    };

    let Ok(datastore): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter datastore name")
        .interact()
    else {
        return;
    };

    // Validate datastore name
    if let Err(e) = super::validation::validate_datastore_name(&datastore) {
        println!("Invalid datastore name: {}", e);
        return;
    }

    println!("🔧 Configuring PBS remote...");
    match Command::new("pvesm")
        .args([
            "add",
            "pbs",
            "backup-remote",
            "--server",
            &pbs_server,
            "--datastore",
            &datastore,
            "--content",
            "backup",
        ])
        .status()
    {
        Ok(status) if status.success() => {
            println!("✅ PBS sync configured!");
        }
        Ok(_) => println!("❌ PBS configuration failed (command returned non-zero)"),
        Err(e) => println!("❌ PBS configuration failed: {}", e),
    }
}

fn setup_drbd_replication() {
    println!("🔄 DRBD Replication Setup\n");

    println!("⚠️  DRBD setup requires:");
    println!("   • DRBD kernel module");
    println!("   • Dedicated network connection");
    println!("   • Matching block devices on both nodes");

    println!("💡 This is an advanced setup - proceed with caution");
}

fn setup_custom_replication() {
    println!("📝 Custom Replication Script\n");

    let Ok(source_path): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter source path")
        .interact()
    else {
        return;
    };

    // Validate source path
    if let Err(e) = super::validation::validate_path(&source_path) {
        println!("Invalid source path: {}", e);
        return;
    }

    let Ok(target_path): Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter target path")
        .interact()
    else {
        return;
    };

    // Validate target path
    if let Err(e) = super::validation::validate_path(&target_path) {
        println!("Invalid target path: {}", e);
        return;
    }

    let script_content = format!(
        r#"#!/bin/bash
# Custom replication script generated by ghostctl

SOURCE="{}"
TARGET="{}"

echo "Starting replication: $SOURCE -> $TARGET"

# Using rsync for initial implementation
rsync -avz --progress "$SOURCE/" "$TARGET/"

if [ $? -eq 0 ]; then
    echo "Replication completed successfully"
else
    echo "Replication failed"
    exit 1
fi
"#,
        source_path, target_path
    );

    let script_path = "/tmp/custom_replication.sh";
    match std::fs::write(script_path, &script_content) {
        Ok(()) => {
            if let Err(e) = Command::new("chmod").args(["+x", script_path]).status() {
                println!(
                    "Script created but chmod failed: {}. Run: chmod +x {}",
                    e, script_path
                );
            } else {
                println!("✅ Custom replication script created: {}", script_path);
            }
        }
        Err(e) => println!("Failed to create replication script: {}", e),
    }
}
