use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::fs;
use std::process::Command;

pub fn network_storage_menu() {
    loop {
        let options = vec![
            "NFS Management",
            "CIFS/SMB Management",
            "Network Mount Tools",
            "Storage Diagnostics",
            "Automated Mount Setup",
            "Back",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🌐 Network Storage Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => nfs_menu(),
            1 => cifs_menu(),
            2 => mount_tools_menu(),
            3 => storage_diagnostics(),
            4 => automated_mount_setup(),
            _ => break,
        }
    }
}

fn nfs_menu() {
    loop {
        let options = vec![
            "Setup NFS Server",
            "Setup NFS Client",
            "Manage NFS Exports",
            "Test NFS Connection",
            "NFS Performance Tuning",
            "Back",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("📁 NFS Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => setup_nfs_server(),
            1 => setup_nfs_client(),
            2 => println!("🔧 NFS exports management coming soon!"),
            3 => println!("🧪 NFS connection testing coming soon!"),
            4 => nfs_performance_tuning(),
            _ => break,
        }
    }
}

fn setup_nfs_server() {
    println!("🏗️  Setting up NFS Server\n");

    // Check if NFS server is installed
    println!("🔍 Checking NFS server installation...");
    let nfs_installed = Command::new("systemctl")
        .args(&["status", "nfs-kernel-server"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !nfs_installed {
        println!("📦 NFS server not installed. Installing...");

        // Detect package manager and install
        if Command::new("which")
            .args(&["apt"])
            .output()
            .unwrap()
            .status
            .success()
        {
            let _ = Command::new("apt").args(&["update"]).status();
            let _ = Command::new("apt")
                .args(&["install", "-y", "nfs-kernel-server", "nfs-common"])
                .status();
        } else if Command::new("which")
            .args(&["pacman"])
            .output()
            .unwrap()
            .status
            .success()
        {
            let _ = Command::new("pacman")
                .args(&["-S", "--noconfirm", "nfs-utils"])
                .status();
        } else if Command::new("which")
            .args(&["yum"])
            .output()
            .unwrap()
            .status
            .success()
        {
            let _ = Command::new("yum")
                .args(&["install", "-y", "nfs-utils", "nfs-utils-lib"])
                .status();
        }
    }

    // Create export directory
    let export_path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter export directory path")
        .default("/srv/nfs/share".to_string())
        .interact()
        .unwrap();

    println!("📁 Creating export directory: {}", export_path);
    let _ = Command::new("mkdir").args(&["-p", &export_path]).status();

    // Set permissions
    let _ = Command::new("chown")
        .args(&["nobody:nogroup", &export_path])
        .status();

    let _ = Command::new("chmod").args(&["755", &export_path]).status();

    // Configure exports
    let client_subnet: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter client subnet (e.g., 192.168.1.0/24)")
        .default("192.168.1.0/24".to_string())
        .interact()
        .unwrap();

    let export_options: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter export options")
        .default("rw,sync,no_subtree_check,no_root_squash".to_string())
        .interact()
        .unwrap();

    let export_line = format!("{} {}({})\n", export_path, client_subnet, export_options);

    // Add to /etc/exports
    if let Ok(mut exports) = fs::read_to_string("/etc/exports") {
        if !exports.contains(&export_path) {
            exports.push_str(&export_line);
            let _ = fs::write("/etc/exports", exports);
        }
    } else {
        let _ = fs::write("/etc/exports", export_line);
    }

    // Reload exports
    println!("♻️  Reloading NFS exports...");
    let _ = Command::new("exportfs").args(&["-ra"]).status();

    // Start and enable NFS services
    println!("🚀 Starting NFS services...");
    let _ = Command::new("systemctl")
        .args(&["enable", "nfs-kernel-server"])
        .status();
    let _ = Command::new("systemctl")
        .args(&["start", "nfs-kernel-server"])
        .status();

    println!("✅ NFS server setup complete!");
    println!("📋 Export: {}", export_path);
    println!("🌐 Clients: {}", client_subnet);
    println!("⚙️  Options: {}", export_options);
}

fn setup_nfs_client() {
    println!("💻 Setting up NFS Client\n");

    // Install NFS client tools
    println!("📦 Installing NFS client tools...");

    if Command::new("which")
        .args(&["apt"])
        .output()
        .unwrap()
        .status
        .success()
    {
        let _ = Command::new("apt")
            .args(&["install", "-y", "nfs-common"])
            .status();
    } else if Command::new("which")
        .args(&["pacman"])
        .output()
        .unwrap()
        .status
        .success()
    {
        let _ = Command::new("pacman")
            .args(&["-S", "--noconfirm", "nfs-utils"])
            .status();
    }

    // Get mount details
    let nfs_server: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter NFS server IP/hostname")
        .interact()
        .unwrap();

    let remote_path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter remote export path")
        .default("/srv/nfs/share".to_string())
        .interact()
        .unwrap();

    let local_mount: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter local mount point")
        .default("/mnt/nfs".to_string())
        .interact()
        .unwrap();

    // Create mount point
    println!("📁 Creating mount point: {}", local_mount);
    let _ = Command::new("mkdir").args(&["-p", &local_mount]).status();

    // Test mount
    let nfs_share = format!("{}:{}", nfs_server, remote_path);
    println!("🔌 Testing NFS mount: {}", nfs_share);

    let mount_result = Command::new("mount")
        .args(&["-t", "nfs", &nfs_share, &local_mount])
        .status();

    if mount_result.unwrap().success() {
        println!("✅ NFS mount successful!");

        if Confirm::new()
            .with_prompt("Add to /etc/fstab for permanent mount?")
            .default(true)
            .interact()
            .unwrap()
        {
            let mount_options: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter mount options")
                .default("defaults,_netdev".to_string())
                .interact()
                .unwrap();

            let fstab_line = format!("{} {} nfs {} 0 0\n", nfs_share, local_mount, mount_options);

            if let Ok(mut fstab) = fs::read_to_string("/etc/fstab")
                && !fstab.contains(&nfs_share) {
                    fstab.push_str(&fstab_line);
                    let _ = fs::write("/etc/fstab", fstab);
                    println!("✅ Added to /etc/fstab");
                }
        }
    } else {
        println!("❌ NFS mount failed!");
    }
}

fn cifs_menu() {
    loop {
        let options = vec![
            "Setup CIFS/SMB Client",
            "Mount Windows Share",
            "Mount Samba Share",
            "CIFS Performance Tuning",
            "Manage CIFS Credentials",
            "Back",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🖥️  CIFS/SMB Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => setup_cifs_client(),
            1 => mount_windows_share(),
            2 => mount_samba_share(),
            3 => cifs_performance_tuning(),
            4 => manage_cifs_credentials(),
            _ => break,
        }
    }
}

fn setup_cifs_client() {
    println!("🔧 Setting up CIFS/SMB Client\n");

    // Install CIFS utilities
    println!("📦 Installing CIFS utilities...");

    if Command::new("which")
        .args(&["apt"])
        .output()
        .unwrap()
        .status
        .success()
    {
        let _ = Command::new("apt")
            .args(&["install", "-y", "cifs-utils"])
            .status();
    } else if Command::new("which")
        .args(&["pacman"])
        .output()
        .unwrap()
        .status
        .success()
    {
        let _ = Command::new("pacman")
            .args(&["-S", "--noconfirm", "cifs-utils"])
            .status();
    } else if Command::new("which")
        .args(&["yum"])
        .output()
        .unwrap()
        .status
        .success()
    {
        let _ = Command::new("yum")
            .args(&["install", "-y", "cifs-utils"])
            .status();
    }

    println!("✅ CIFS client tools installed!");

    // Create credentials directory
    let _ = Command::new("mkdir").args(&["-p", "/etc/cifs"]).status();

    let _ = Command::new("chmod").args(&["700", "/etc/cifs"]).status();

    println!("📁 Created secure credentials directory: /etc/cifs");
}

fn mount_windows_share() {
    println!("🪟 Mounting Windows Share\n");

    let server: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Windows server IP/hostname")
        .interact()
        .unwrap();

    let share_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter share name")
        .interact()
        .unwrap();

    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter username")
        .interact()
        .unwrap();

    let password: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter password")
        .with_initial_text("")
        .interact()
        .unwrap();

    let local_mount: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter local mount point")
        .default(format!("/mnt/{}", share_name))
        .interact()
        .unwrap();

    // Create mount point
    let _ = Command::new("mkdir").args(&["-p", &local_mount]).status();

    // Create credentials file
    let cred_file = format!("/etc/cifs/{}.cred", share_name);
    let cred_content = format!("username={}\npassword={}\n", username, password);
    let _ = fs::write(&cred_file, cred_content);
    let _ = Command::new("chmod").args(&["600", &cred_file]).status();

    // Mount share
    let share_path = format!("//{}/{}", server, share_name);
    let mount_result = Command::new("mount")
        .args(&[
            "-t",
            "cifs",
            &share_path,
            &local_mount,
            "-o",
            &format!("credentials={},uid=1000,gid=1000,iocharset=utf8", cred_file),
        ])
        .status();

    if mount_result.unwrap().success() {
        println!("✅ Windows share mounted successfully!");
        println!("📁 Available at: {}", local_mount);

        if Confirm::new()
            .with_prompt("Add to /etc/fstab for permanent mount?")
            .default(true)
            .interact()
            .unwrap()
        {
            let fstab_line = format!(
                "{} {} cifs credentials={},uid=1000,gid=1000,iocharset=utf8,_netdev 0 0\n",
                share_path, local_mount, cred_file
            );

            if let Ok(mut fstab) = fs::read_to_string("/etc/fstab")
                && !fstab.contains(&share_path) {
                    fstab.push_str(&fstab_line);
                    let _ = fs::write("/etc/fstab", fstab);
                    println!("✅ Added to /etc/fstab");
                }
        }
    } else {
        println!("❌ Failed to mount Windows share!");
    }
}

fn mount_samba_share() {
    println!("🐧 Mounting Samba Share\n");

    // Similar to Windows share but with Samba-specific optimizations
    mount_windows_share(); // Reuse the same logic for now
}

fn mount_tools_menu() {
    loop {
        let options = vec![
            "List All Mounts",
            "Show Network Mounts",
            "Unmount Network Share",
            "Test Mount Connectivity",
            "Mount Troubleshooting",
            "Back",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🔧 Network Mount Tools")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => list_all_mounts(),
            1 => show_network_mounts(),
            2 => unmount_network_share(),
            3 => test_mount_connectivity(),
            4 => println!("🔧 Mount troubleshooting coming soon!"),
            _ => break,
        }
    }
}

fn list_all_mounts() {
    println!("📋 All Current Mounts\n");
    let _ = Command::new("mount").args(&["-t", "nfs,cifs"]).status();

    println!("\n💾 Disk Usage for Network Mounts:");
    let _ = Command::new("df")
        .args(&["-h", "-t", "nfs", "-t", "cifs"])
        .status();
}

fn show_network_mounts() {
    println!("🌐 Network Mounts Only\n");

    // Show NFS mounts
    println!("📁 NFS Mounts:");
    let _ = Command::new("mount").args(&["-t", "nfs"]).status();

    println!("\n🖥️  CIFS/SMB Mounts:");
    let _ = Command::new("mount").args(&["-t", "cifs"]).status();
}

fn unmount_network_share() {
    println!("🔌 Unmount Network Share\n");

    let mount_point: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter mount point to unmount")
        .interact()
        .unwrap();

    println!("🔄 Attempting to unmount: {}", mount_point);

    let result = Command::new("umount").args(&[&mount_point]).status();

    if result.unwrap().success() {
        println!("✅ Successfully unmounted: {}", mount_point);
    } else {
        println!("⚠️  Normal unmount failed, trying lazy unmount...");
        let lazy_result = Command::new("umount").args(&["-l", &mount_point]).status();

        if lazy_result.unwrap().success() {
            println!("✅ Lazy unmount successful");
        } else {
            println!("❌ Failed to unmount. Check if files are in use.");
        }
    }
}

fn test_mount_connectivity() {
    println!("🧪 Testing Mount Connectivity\n");

    let server: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter server IP/hostname")
        .interact()
        .unwrap();

    // Ping test
    println!("🏓 Testing ping connectivity...");
    let ping_result = Command::new("ping").args(&["-c", "3", &server]).status();

    if ping_result.unwrap().success() {
        println!("✅ Ping successful");
    } else {
        println!("❌ Ping failed");
    }

    // Port tests
    println!("\n🔌 Testing service ports...");

    // NFS ports
    println!("📁 NFS (port 2049):");
    let _ = Command::new("nc")
        .args(&["-z", "-v", &server, "2049"])
        .status();

    // SMB ports
    println!("🖥️  SMB (port 445):");
    let _ = Command::new("nc")
        .args(&["-z", "-v", &server, "445"])
        .status();

    println!("🖥️  SMB (port 139):");
    let _ = Command::new("nc")
        .args(&["-z", "-v", &server, "139"])
        .status();
}

fn storage_diagnostics() {
    println!("🔍 Network Storage Diagnostics\n");

    println!("📊 Network Mount Statistics:");
    let _ = Command::new("nfsstat").args(&["-c"]).status();

    println!("\n🌐 Network Interface Statistics:");
    let _ = Command::new("cat").args(&["/proc/net/dev"]).status();

    println!("\n💾 I/O Statistics:");
    let _ = Command::new("iostat").args(&["-x", "1", "3"]).status();

    println!("\n🔧 Network Storage Kernel Modules:");
    let _ = Command::new("lsmod")
        .args(&["grep", "nfs\\|cifs"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn automated_mount_setup() {
    println!("🤖 Automated Mount Setup\n");

    println!("This will create an automated mount configuration...");

    let mount_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select mount type")
        .items(&["NFS", "CIFS/SMB"])
        .default(0)
        .interact()
        .unwrap();

    match mount_type {
        0 => automated_nfs_setup(),
        1 => automated_cifs_setup(),
        _ => {}
    }
}

fn automated_nfs_setup() {
    println!("📁 Automated NFS Setup\n");

    // This would implement a wizard-based NFS setup
    // with automatic discovery and configuration
    setup_nfs_client();
}

fn automated_cifs_setup() {
    println!("🖥️  Automated CIFS Setup\n");

    // This would implement a wizard-based CIFS setup
    mount_windows_share();
}

fn nfs_performance_tuning() {
    println!("⚡ NFS Performance Tuning\n");

    println!("🔧 Current NFS mount options:");
    let _ = Command::new("mount").args(&["-t", "nfs"]).status();

    println!("\n⚙️  Recommended NFS mount options for performance:");
    println!("  • rsize=65536,wsize=65536 - Larger read/write sizes");
    println!("  • proto=tcp - Use TCP instead of UDP");
    println!("  • nfsvers=4.1 - Use NFSv4.1 for better performance");
    println!("  • fsc - Enable local caching");

    if Confirm::new()
        .with_prompt("Apply performance optimizations to existing mounts?")
        .default(false)
        .interact()
        .unwrap()
    {
        println!("⚠️  You'll need to remount with optimized options manually");
        println!("Example: mount -o remount,rsize=65536,wsize=65536,proto=tcp /mount/point");
    }
}

fn cifs_performance_tuning() {
    println!("⚡ CIFS Performance Tuning\n");

    println!("⚙️  Recommended CIFS mount options for performance:");
    println!("  • cache=strict - Enable aggressive caching");
    println!("  • vers=3.0 - Use SMB3 for better security and performance");
    println!("  • rsize=65536,wsize=65536 - Larger transfer sizes");
    println!("  • mfsymlinks - Better symlink handling");
}

fn manage_cifs_credentials() {
    println!("🔐 CIFS Credentials Management\n");

    println!("📁 Existing credential files:");
    let _ = Command::new("ls").args(&["-la", "/etc/cifs/"]).status();

    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action")
        .items(&[
            "Create new credential file",
            "Edit existing file",
            "Delete credential file",
            "Back",
        ])
        .default(0)
        .interact()
        .unwrap();

    match action {
        0 => create_cifs_credential(),
        1 => edit_cifs_credential(),
        2 => delete_cifs_credential(),
        _ => {}
    }
}

fn create_cifs_credential() {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter credential file name")
        .interact()
        .unwrap();

    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter username")
        .interact()
        .unwrap();

    let password: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter password")
        .interact()
        .unwrap();

    let domain: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter domain (optional)")
        .default("".to_string())
        .interact()
        .unwrap();

    let cred_file = format!("/etc/cifs/{}.cred", name);
    let mut content = format!("username={}\npassword={}\n", username, password);

    if !domain.is_empty() {
        content.push_str(&format!("domain={}\n", domain));
    }

    if fs::write(&cred_file, content).is_ok() {
        let _ = Command::new("chmod").args(&["600", &cred_file]).status();
        println!("✅ Credential file created: {}", cred_file);
    } else {
        println!("❌ Failed to create credential file");
    }
}

fn edit_cifs_credential() {
    let file: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter credential file path")
        .interact()
        .unwrap();

    let _ = Command::new("nano").args(&[&file]).status();
}

fn delete_cifs_credential() {
    let file: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter credential file path to delete")
        .interact()
        .unwrap();

    if Confirm::new()
        .with_prompt(&format!("Really delete {}?", file))
        .default(false)
        .interact()
        .unwrap()
    {
        if fs::remove_file(&file).is_ok() {
            println!("✅ Credential file deleted");
        } else {
            println!("❌ Failed to delete file");
        }
    }
}
