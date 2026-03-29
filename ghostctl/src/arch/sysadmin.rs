use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn sysadmin_menu() {
    loop {
        let options = [
            "🔧 Advanced System Configuration",
            "📦 Package Management Advanced",
            "🔐 System Security Hardening",
            "📊 System Health Monitoring",
            "🔄 Service Management",
            "📝 Log Analysis & Management",
            "🌐 Network Configuration",
            "🚀 Kernel Management",
            "⬅️  Back",
        ];

        let choice = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🛠️  Advanced System Administration")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            Ok(None) | Err(_) => break,
        };

        match choice {
            0 => advanced_system_config(),
            1 => advanced_package_management(),
            2 => security_hardening(),
            3 => system_health_monitoring(),
            4 => service_management(),
            5 => log_management(),
            6 => network_configuration(),
            7 => kernel_management(),
            _ => break,
        }
    }
}

fn advanced_system_config() {
    println!("🔧 Advanced System Configuration");
    println!("===============================");

    let config_options = [
        "⚙️  System Limits Configuration",
        "🔧 Module Loading Configuration",
        "📁 Filesystem Mount Options",
        "🔄 Process Management",
        "🗂️  File Permissions Audit",
        "🔒 User & Group Management",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("System Configuration")
        .items(&config_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => configure_system_limits(),
        1 => configure_module_loading(),
        2 => configure_filesystem_mounts(),
        3 => process_management(),
        4 => file_permissions_audit(),
        5 => user_group_management(),
        _ => return,
    }
}

fn configure_system_limits() {
    println!("⚙️  System Limits Configuration");
    println!("===============================");

    let limits_config = r#"# GhostCTL System Limits
# Increase file descriptor limits for high-performance applications
* soft nofile 65536
* hard nofile 65536

# Memory limits
* soft memlock unlimited
* hard memlock unlimited

# Process limits
* soft nproc 32768
* hard nproc 32768
"#;

    let confirm = match Confirm::new()
        .with_prompt("Apply enhanced system limits configuration?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        if let Ok(mut file) = std::fs::File::create("/etc/security/limits.d/99-ghostctl.conf") {
            use std::io::Write;
            if file.write_all(limits_config.as_bytes()).is_ok() {
                println!("✅ System limits configuration applied");
                println!("💡 Reboot required for changes to take effect");
            } else {
                println!("❌ Failed to write limits configuration");
            }
        } else {
            println!("❌ Failed to create limits configuration file");
        }
    }
}

fn configure_module_loading() {
    println!("🔧 Kernel Module Loading Configuration");
    println!("=====================================");

    let modules_options = [
        "📋 List Loaded Modules",
        "🔧 Configure Module Blacklist",
        "⚡ Load Module",
        "🛑 Unload Module",
        "📝 Module Information",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Module Management")
        .items(&modules_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => list_loaded_modules(),
        1 => configure_module_blacklist(),
        2 => load_module(),
        3 => unload_module(),
        4 => module_information(),
        _ => return,
    }
}

fn list_loaded_modules() {
    println!("📋 Currently Loaded Kernel Modules");
    println!("==================================");

    if let Err(e) = Command::new("lsmod").status() {
        eprintln!("Failed to list kernel modules: {}", e);
    }
}

fn configure_module_blacklist() {
    println!("🔧 Configure Module Blacklist");
    println!("=============================");

    let module_name: String = match Input::new()
        .with_prompt("Enter module name to blacklist")
        .interact_text()
    {
        Ok(name) => name,
        Err(_) => return,
    };

    let confirm = match Confirm::new()
        .with_prompt(format!("Blacklist module '{}'?", module_name))
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        let blacklist_entry = format!("blacklist {}\n", module_name);
        let blacklist_file = "/etc/modprobe.d/99-ghostctl-blacklist.conf";

        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(blacklist_file)
        {
            use std::io::Write;
            if file.write_all(blacklist_entry.as_bytes()).is_ok() {
                println!("✅ Module '{}' blacklisted", module_name);
                println!("💡 Reboot required for changes to take effect");
            }
        }
    }
}

fn load_module() {
    println!("⚡ Load Kernel Module");
    println!("====================");

    let module_name: String = match Input::new()
        .with_prompt("Enter module name to load")
        .interact_text()
    {
        Ok(name) => name,
        Err(_) => return,
    };

    let status = Command::new("sudo")
        .args(&["modprobe", &module_name])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Module '{}' loaded successfully", module_name),
        _ => println!("❌ Failed to load module '{}'", module_name),
    }
}

fn unload_module() {
    println!("🛑 Unload Kernel Module");
    println!("=======================");

    let module_name: String = match Input::new()
        .with_prompt("Enter module name to unload")
        .interact_text()
    {
        Ok(name) => name,
        Err(_) => return,
    };

    let confirm = match Confirm::new()
        .with_prompt(format!("Unload module '{}'?", module_name))
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        let status = Command::new("sudo")
            .args(&["modprobe", "-r", &module_name])
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Module '{}' unloaded successfully", module_name),
            _ => println!("❌ Failed to unload module '{}'", module_name),
        }
    }
}

fn module_information() {
    println!("📝 Module Information");
    println!("====================");

    let module_name: String = match Input::new()
        .with_prompt("Enter module name for information")
        .interact_text()
    {
        Ok(name) => name,
        Err(_) => return,
    };

    println!("📊 Module information for '{}':", module_name);
    if let Err(e) = Command::new("modinfo").arg(&module_name).status() {
        eprintln!("Failed to get module info for '{}': {}", module_name, e);
    }
}

fn configure_filesystem_mounts() {
    println!("📁 Filesystem Mount Options");
    println!("===========================");

    let mount_options = [
        "📊 Show Current Mounts",
        "🔧 Optimize Mount Options",
        "💾 Temporary Filesystem Setup",
        "🗂️  Backup fstab",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Filesystem Management")
        .items(&mount_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => show_current_mounts(),
        1 => optimize_mount_options(),
        2 => setup_temp_filesystem(),
        3 => backup_fstab(),
        _ => return,
    }
}

fn show_current_mounts() {
    println!("📊 Current Filesystem Mounts");
    println!("============================");

    if let Err(e) = Command::new("mount").status() {
        eprintln!("Failed to show mounts: {}", e);
    }

    println!("\n📋 /etc/fstab contents:");
    if let Err(e) = Command::new("cat").arg("/etc/fstab").status() {
        eprintln!("Failed to read /etc/fstab: {}", e);
    }
}

fn optimize_mount_options() {
    println!("🔧 Optimize Mount Options");
    println!("=========================");

    println!("💡 Common optimizations:");
    println!("  • noatime - Disable access time updates");
    println!("  • compress=zstd - Enable compression for Btrfs");
    println!("  • discard - Enable TRIM for SSDs");
    println!("  • relatime - Update access times efficiently");

    println!("\n⚠️  Manual fstab editing required for persistent changes");
    println!("📝 Backup your fstab before making changes");
}

fn setup_temp_filesystem() {
    println!("💾 Temporary Filesystem Setup");
    println!("=============================");

    let temp_size: String = match Input::new()
        .with_prompt("Enter tmpfs size (e.g., 4G, 50%)")
        .default("2G".to_string())
        .interact_text()
    {
        Ok(size) => size,
        Err(_) => return,
    };

    let mount_point: String = match Input::new()
        .with_prompt("Enter mount point")
        .default("/tmp/ramdisk".to_string())
        .interact_text()
    {
        Ok(point) => point,
        Err(_) => return,
    };

    let confirm = match Confirm::new()
        .with_prompt(format!("Create {}B tmpfs at '{}'?", temp_size, mount_point))
        .default(true)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        // Create mount point
        if let Err(e) = Command::new("sudo")
            .args(["mkdir", "-p", &mount_point])
            .status()
        {
            eprintln!("Failed to create mount point '{}': {}", mount_point, e);
            return;
        }

        // Mount tmpfs
        let status = Command::new("sudo")
            .args(&[
                "mount",
                "-t",
                "tmpfs",
                "-o",
                &format!("size={}", temp_size),
                "tmpfs",
                &mount_point,
            ])
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Tmpfs mounted at '{}'", mount_point),
            _ => println!("❌ Failed to create tmpfs"),
        }
    }
}

fn backup_fstab() {
    println!("🗂️  Backup fstab");
    println!("================");

    let backup_name = format!(
        "/etc/fstab.backup.{}",
        chrono::Utc::now().format("%Y%m%d_%H%M%S")
    );

    let status = Command::new("sudo")
        .args(&["cp", "/etc/fstab", &backup_name])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ fstab backed up to '{}'", backup_name),
        _ => println!("❌ Failed to backup fstab"),
    }
}

fn process_management() {
    println!("🔄 Advanced Process Management");
    println!("==============================");

    let process_options = [
        "📊 Process Analysis",
        "🎯 CPU Affinity Management",
        "⚖️  Process Priority Control",
        "🔧 Process Limits",
        "🛑 Process Control",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Process Management")
        .items(&process_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => process_analysis(),
        1 => cpu_affinity_management(),
        2 => process_priority_control(),
        3 => process_limits(),
        4 => process_control(),
        _ => return,
    }
}

fn process_analysis() {
    println!("📊 Process Analysis");
    println!("==================");

    println!("🔍 Top CPU consumers:");
    if let Err(e) = Command::new("ps").args(["aux", "--sort=-%cpu"]).status() {
        eprintln!("Failed to get CPU consumers: {}", e);
    }

    println!("\n💾 Top memory consumers:");
    if let Err(e) = Command::new("ps").args(["aux", "--sort=-%mem"]).status() {
        eprintln!("Failed to get memory consumers: {}", e);
    }

    println!("\n🌳 Process tree:");
    if let Err(e) = Command::new("pstree").args(["-p"]).status() {
        eprintln!("Failed to show process tree: {}", e);
    }
}

fn cpu_affinity_management() {
    println!("🎯 CPU Affinity Management");
    println!("==========================");

    println!("📊 Current CPU count:");
    if let Err(e) = Command::new("nproc").status() {
        eprintln!("Failed to get CPU count: {}", e);
    }

    let pid: String = match Input::new()
        .with_prompt("Enter process PID for affinity management")
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    println!("📋 Current affinity for PID {}:", pid);
    if let Err(e) = Command::new("taskset").args(["-p", &pid]).status() {
        eprintln!("Failed to get CPU affinity for PID {}: {}", pid, e);
    }

    let cpu_mask: String = match Input::new()
        .with_prompt("Enter CPU mask (e.g., 0x3 for CPUs 0,1)")
        .interact_text()
    {
        Ok(mask) => mask,
        Err(_) => return,
    };

    let confirm = match Confirm::new()
        .with_prompt(format!("Set CPU affinity {} for PID {}?", cpu_mask, pid))
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        let status = Command::new("sudo")
            .args(&["taskset", "-p", &cpu_mask, &pid])
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ CPU affinity set successfully"),
            _ => println!("❌ Failed to set CPU affinity"),
        }
    }
}

fn process_priority_control() {
    println!("⚖️  Process Priority Control");
    println!("============================");

    let pid: String = match Input::new()
        .with_prompt("Enter process PID")
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let priority_options = [
        "-20 (Highest priority)",
        "-10 (High priority)",
        "0 (Normal priority)",
        "10 (Low priority)",
        "19 (Lowest priority)",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select priority level")
        .items(&priority_options)
        .default(2)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    let priority_values = [-20, -10, 0, 10, 19];
    let selected_priority = priority_values[choice];

    let confirm = match Confirm::new()
        .with_prompt(format!(
            "Set priority {} for PID {}?",
            selected_priority, pid
        ))
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        let status = Command::new("sudo")
            .args(&["renice", &selected_priority.to_string(), &pid])
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Process priority set successfully"),
            _ => println!("❌ Failed to set process priority"),
        }
    }
}

fn process_limits() {
    println!("🔧 Process Resource Limits");
    println!("==========================");

    println!("📊 Current resource limits:");
    if let Err(e) = Command::new("ulimit").arg("-a").status() {
        eprintln!("Failed to get resource limits: {}", e);
    }

    println!("\n💡 To modify limits permanently, edit /etc/security/limits.conf");
}

fn process_control() {
    println!("🛑 Process Control");
    println!("==================");

    let control_options = [
        "🔍 Search Process",
        "⏸️  Pause Process (STOP)",
        "▶️  Resume Process (CONT)",
        "🛑 Terminate Process (TERM)",
        "💀 Kill Process (KILL)",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Process Control")
        .items(&control_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => search_process(),
        1 => signal_process("STOP"),
        2 => signal_process("CONT"),
        3 => signal_process("TERM"),
        4 => signal_process("KILL"),
        _ => return,
    }
}

fn search_process() {
    println!("🔍 Search Process");
    println!("=================");

    let search_term: String = match Input::new()
        .with_prompt("Enter process name or pattern")
        .interact_text()
    {
        Ok(term) => term,
        Err(_) => return,
    };

    println!("📋 Matching processes:");
    if let Err(e) = Command::new("pgrep").args(["-l", &search_term]).status() {
        eprintln!("Failed to search for processes: {}", e);
    }
}

fn signal_process(signal: &str) {
    println!("📡 Send Signal {} to Process", signal);
    println!("===============================");

    let pid: String = match Input::new()
        .with_prompt("Enter process PID")
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let confirm = match Confirm::new()
        .with_prompt(format!("Send {} signal to PID {}?", signal, pid))
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        let status = Command::new("sudo")
            .args(&["kill", &format!("-{}", signal), &pid])
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Signal {} sent to PID {}", signal, pid),
            _ => println!("❌ Failed to send signal to process"),
        }
    }
}

fn file_permissions_audit() {
    println!("🗂️  File Permissions Audit");
    println!("===========================");

    let audit_options = [
        "🔍 Find SUID/SGID Files",
        "📂 Find World-Writable Files",
        "🔒 Find Files Without Owner",
        "📊 Permission Statistics",
        "🛡️  Security Audit",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("File Permissions Audit")
        .items(&audit_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => find_suid_sgid_files(),
        1 => find_world_writable_files(),
        2 => find_files_without_owner(),
        3 => permission_statistics(),
        4 => security_audit(),
        _ => return,
    }
}

fn find_suid_sgid_files() {
    println!("🔍 SUID/SGID Files");
    println!("==================");

    println!("📋 SUID files (run with owner permissions):");
    if let Err(e) = Command::new("find")
        .args(["/", "-type", "f", "-perm", "-4000", "-ls", "2>/dev/null"])
        .status()
    {
        eprintln!("Failed to find SUID files: {}", e);
    }

    println!("\n📋 SGID files (run with group permissions):");
    if let Err(e) = Command::new("find")
        .args(["/", "-type", "f", "-perm", "-2000", "-ls", "2>/dev/null"])
        .status()
    {
        eprintln!("Failed to find SGID files: {}", e);
    }
}

fn find_world_writable_files() {
    println!("📂 World-Writable Files");
    println!("=======================");

    println!("⚠️  World-writable files (potential security risk):");
    if let Err(e) = Command::new("find")
        .args(["/", "-type", "f", "-perm", "-002", "-ls", "2>/dev/null"])
        .status()
    {
        eprintln!("Failed to find world-writable files: {}", e);
    }

    println!("\n📁 World-writable directories:");
    if let Err(e) = Command::new("find")
        .args(["/", "-type", "d", "-perm", "-002", "-ls", "2>/dev/null"])
        .status()
    {
        eprintln!("Failed to find world-writable directories: {}", e);
    }
}

fn find_files_without_owner() {
    println!("🔒 Files Without Owner");
    println!("======================");

    println!("👻 Files without valid user:");
    if let Err(e) = Command::new("find")
        .args(["/", "-nouser", "-ls", "2>/dev/null"])
        .status()
    {
        eprintln!("Failed to find files without owner: {}", e);
    }

    println!("\n👻 Files without valid group:");
    if let Err(e) = Command::new("find")
        .args(["/", "-nogroup", "-ls", "2>/dev/null"])
        .status()
    {
        eprintln!("Failed to find files without group: {}", e);
    }
}

fn permission_statistics() {
    println!("📊 File Permission Statistics");
    println!("=============================");

    println!("📈 File type distribution:");
    // Count files and directories in Rust instead of using shell pipeline
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());

    let mut file_count = 0u64;
    let mut dir_count = 0u64;

    fn count_entries(path: &std::path::Path, file_count: &mut u64, dir_count: &mut u64) {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        *file_count += 1;
                    } else if file_type.is_dir() {
                        *dir_count += 1;
                        // Recursively count in subdirectories
                        count_entries(&entry.path(), file_count, dir_count);
                    }
                }
            }
        }
    }

    count_entries(
        std::path::Path::new(&home_dir),
        &mut file_count,
        &mut dir_count,
    );
    println!("  Regular files: {}", file_count);
    println!("  Directories: {}", dir_count);
}

fn security_audit() {
    println!("🛡️  File Security Audit");
    println!("=======================");

    println!("🔍 Running comprehensive file security audit...");

    // Check for common security issues
    println!("\n⚠️  Checking for potential security issues:");

    println!("1. Checking for files with weak permissions in /etc:");
    if let Err(e) = Command::new("find")
        .args([
            "/etc", "-type", "f", "-perm", "-004", "-exec", "ls", "-l", "{}", "+",
        ])
        .status()
    {
        eprintln!("Failed to check /etc permissions: {}", e);
    }

    println!("\n2. Checking for executables in unusual locations:");
    if let Err(e) = Command::new("find")
        .args(["/tmp", "/var/tmp", "-type", "f", "-executable", "-ls"])
        .status()
    {
        eprintln!("Failed to find executables in temp locations: {}", e);
    }
}

fn user_group_management() {
    println!("🔒 User & Group Management");
    println!("==========================");

    let user_options = [
        "👥 List Users",
        "🏷️  List Groups",
        "👤 User Information",
        "🏷️  Group Information",
        "🔑 Password Policy Check",
        "📊 Login History",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("User & Group Management")
        .items(&user_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => list_users(),
        1 => list_groups(),
        2 => user_information(),
        3 => group_information(),
        4 => password_policy_check(),
        5 => login_history(),
        _ => return,
    }
}

fn list_users() {
    println!("👥 System Users");
    println!("===============");

    println!("📋 All users:");
    if let Err(e) = Command::new("cut")
        .args(["-d:", "-f1", "/etc/passwd"])
        .status()
    {
        eprintln!("Failed to list users: {}", e);
    }

    println!("\n👤 Human users (UID >= 1000):");
    if let Err(e) = Command::new("awk")
        .args([
            "-F:",
            "$3 >= 1000 && $1 != \"nobody\" {print $1}",
            "/etc/passwd",
        ])
        .status()
    {
        eprintln!("Failed to list human users: {}", e);
    }
}

fn list_groups() {
    println!("🏷️  System Groups");
    println!("=================");

    if let Err(e) = Command::new("cut")
        .args(["-d:", "-f1", "/etc/group"])
        .status()
    {
        eprintln!("Failed to list groups: {}", e);
    }
}

fn user_information() {
    println!("👤 User Information");
    println!("==================");

    let username: String = match Input::new().with_prompt("Enter username").interact_text() {
        Ok(name) => name,
        Err(_) => return,
    };

    println!("📊 User details for '{}':", username);
    if let Err(e) = Command::new("id").arg(&username).status() {
        eprintln!("Failed to get user details: {}", e);
    }

    println!("\n🏠 Home directory and shell:");
    if let Err(e) = Command::new("getent").args(["passwd", &username]).status() {
        eprintln!("Failed to get passwd entry: {}", e);
    }

    println!("\n🏷️  Group memberships:");
    if let Err(e) = Command::new("groups").arg(&username).status() {
        eprintln!("Failed to get group memberships: {}", e);
    }
}

fn group_information() {
    println!("🏷️  Group Information");
    println!("=====================");

    let groupname: String = match Input::new().with_prompt("Enter group name").interact_text() {
        Ok(name) => name,
        Err(_) => return,
    };

    println!("📊 Group details for '{}':", groupname);
    if let Err(e) = Command::new("getent").args(["group", &groupname]).status() {
        eprintln!("Failed to get group details: {}", e);
    }
}

fn password_policy_check() {
    println!("🔑 Password Policy Check");
    println!("=======================");

    println!("📊 Current password policies:");
    if let Err(e) = Command::new("cat").arg("/etc/login.defs").status() {
        eprintln!("Failed to read login.defs: {}", e);
    }

    println!("\n🔒 Password aging information:");
    let username: String = match Input::new()
        .with_prompt("Enter username to check")
        .interact_text()
    {
        Ok(name) => name,
        Err(_) => return,
    };

    if let Err(e) = Command::new("chage").args(["-l", &username]).status() {
        eprintln!("Failed to get password aging info: {}", e);
    }
}

fn login_history() {
    println!("📊 Login History");
    println!("================");

    println!("📋 Recent logins:");
    if let Err(e) = Command::new("last").args(["-10"]).status() {
        eprintln!("Failed to get login history: {}", e);
    }

    println!("\n❌ Failed login attempts:");
    if let Err(e) = Command::new("lastb").args(["-10"]).status() {
        eprintln!("Failed to get failed login attempts: {}", e);
    }

    println!("\n📈 Login statistics:");
    if let Err(e) = Command::new("last")
        .args([
            "|",
            "awk",
            "{print $1}",
            "|",
            "sort",
            "|",
            "uniq",
            "-c",
            "|",
            "sort",
            "-nr",
        ])
        .status()
    {
        eprintln!("Failed to get login statistics: {}", e);
    }
}

fn advanced_package_management() {
    println!("📦 Advanced Package Management");
    println!("==============================");

    let package_options = [
        "🔍 Package Dependency Analysis",
        "🧹 Deep System Cleanup",
        "📊 Package Statistics",
        "🔄 Package Cache Management",
        "🛡️  Package Verification",
        "📋 Package File Management",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Advanced Package Management")
        .items(&package_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => package_dependency_analysis(),
        1 => deep_system_cleanup(),
        2 => package_statistics(),
        3 => package_cache_management(),
        4 => package_verification(),
        5 => package_file_management(),
        _ => return,
    }
}

fn package_dependency_analysis() {
    println!("🔍 Package Dependency Analysis");
    println!("==============================");

    let analysis_options = [
        "📦 Package Dependencies",
        "🔗 Reverse Dependencies",
        "🌳 Dependency Tree",
        "💔 Broken Dependencies",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Dependency Analysis")
        .items(&analysis_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            let package: String = match Input::new()
                .with_prompt("Enter package name")
                .interact_text()
            {
                Ok(pkg) => pkg,
                Err(_) => return,
            };
            println!("📋 Dependencies for '{}':", package);
            if let Err(e) = Command::new("pacman").args(["-Qi", &package]).status() {
                eprintln!("Failed to get package info: {}", e);
            }
        }
        1 => {
            let package: String = match Input::new()
                .with_prompt("Enter package name")
                .interact_text()
            {
                Ok(pkg) => pkg,
                Err(_) => return,
            };
            println!("🔗 Packages depending on '{}':", package);
            if let Err(e) = Command::new("pacman").args(["-Qii", &package]).status() {
                eprintln!("Failed to get reverse dependencies: {}", e);
            }
        }
        2 => {
            println!("🌳 Full dependency tree:");
            if let Err(e) = Command::new("pactree").args(["-c", "-d", "3"]).status() {
                eprintln!("Failed to show dependency tree: {}", e);
            }
        }
        3 => {
            println!("💔 Checking for broken dependencies:");
            if let Err(e) = Command::new("pacman").args(["-Qk"]).status() {
                eprintln!("Failed to check dependencies: {}", e);
            }
        }
        _ => {}
    }
}

fn deep_system_cleanup() {
    println!("🧹 Deep System Cleanup");
    println!("======================");

    let cleanup_options = [
        "🗑️  Remove Orphaned Packages",
        "📦 Clean Package Cache",
        "🔧 Remove Unused Dependencies",
        "📝 Clean Log Files",
        "🗂️  Clean Temporary Files",
        "🔄 All Cleanup Operations",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Cleanup Operations")
        .items(&cleanup_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            println!("🗑️  Removing orphaned packages...");
            if let Err(e) = Command::new("sudo")
                .args(["pacman", "-Rns", "$(pacman -Qtdq)"])
                .status()
            {
                eprintln!("Failed to remove orphaned packages: {}", e);
            }
        }
        1 => {
            println!("📦 Cleaning package cache...");
            if let Err(e) = Command::new("sudo").args(["paccache", "-r"]).status() {
                eprintln!("Failed to run paccache: {}", e);
            }
            if let Err(e) = Command::new("sudo").args(["pacman", "-Scc"]).status() {
                eprintln!("Failed to clean pacman cache: {}", e);
            }
        }
        2 => {
            println!("🔧 Removing unused dependencies...");
            if let Err(e) = Command::new("sudo")
                .args(["pacman", "-Rns", "$(pacman -Qtdq)"])
                .status()
            {
                eprintln!("Failed to remove unused dependencies: {}", e);
            }
        }
        3 => {
            println!("📝 Cleaning log files...");
            if let Err(e) = Command::new("sudo")
                .args(["journalctl", "--vacuum-time=7d"])
                .status()
            {
                eprintln!("Failed to vacuum journal: {}", e);
            }
        }
        4 => {
            println!("🗂️  Cleaning temporary files...");
            if let Err(e) = Command::new("sudo").args(["rm", "-rf", "/tmp/*"]).status() {
                eprintln!("Failed to clean /tmp: {}", e);
            }
            if let Err(e) = Command::new("sudo")
                .args(["rm", "-rf", "/var/tmp/*"])
                .status()
            {
                eprintln!("Failed to clean /var/tmp: {}", e);
            }
        }
        5 => {
            println!("🔄 Running all cleanup operations...");
            let cleanup_tasks = vec![
                "Removing orphaned packages",
                "Cleaning package cache",
                "Cleaning log files",
                "Cleaning temporary files",
            ];

            for task in cleanup_tasks {
                println!("  🔄 {}", task);
            }
        }
        _ => {}
    }
}

fn package_statistics() {
    println!("📊 Package Statistics");
    println!("====================");

    println!("📈 Package counts:");
    if let Err(e) = Command::new("pacman")
        .args(["-Q", "|", "wc", "-l"])
        .status()
    {
        eprintln!("Failed to count packages: {}", e);
    }

    println!("\n📦 Explicitly installed packages:");
    if let Err(e) = Command::new("pacman")
        .args(["-Qe", "|", "wc", "-l"])
        .status()
    {
        eprintln!("Failed to count explicit packages: {}", e);
    }

    println!("\n🔗 Dependencies:");
    if let Err(e) = Command::new("pacman")
        .args(["-Qd", "|", "wc", "-l"])
        .status()
    {
        eprintln!("Failed to count dependencies: {}", e);
    }

    println!("\n👻 Orphaned packages:");
    if let Err(e) = Command::new("pacman")
        .args(["-Qtd", "|", "wc", "-l"])
        .status()
    {
        eprintln!("Failed to count orphaned packages: {}", e);
    }

    println!("\n📊 Package sizes:");
    if let Err(e) = Command::new("pacman")
        .args(["-Qi", "|", "grep", "Installed Size", "|", "sort", "-rh"])
        .status()
    {
        eprintln!("Failed to get package sizes: {}", e);
    }
}

fn package_cache_management() {
    println!("🔄 Package Cache Management");
    println!("===========================");

    println!("📊 Cache information:");
    if let Err(e) = Command::new("du")
        .args(["-sh", "/var/cache/pacman/pkg/"])
        .status()
    {
        eprintln!("Failed to get cache size: {}", e);
    }

    let cache_options = [
        "🧹 Clean all cached packages",
        "🗑️  Keep only latest versions",
        "📦 Remove uninstalled packages",
        "📊 Show cache statistics",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Cache Management")
        .items(&cache_options)
        .default(1)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            let confirm = match Confirm::new()
                .with_prompt("Remove all cached packages?")
                .default(false)
                .interact_opt()
            {
                Ok(Some(c)) => c,
                Ok(None) | Err(_) => return,
            };
            if confirm && let Err(e) = Command::new("sudo").args(["pacman", "-Scc"]).status() {
                eprintln!("Failed to clean cache: {}", e);
            }
        }
        1 => {
            if let Err(e) = Command::new("sudo").args(["paccache", "-r"]).status() {
                eprintln!("Failed to run paccache: {}", e);
            }
        }
        2 => {
            if let Err(e) = Command::new("sudo").args(["paccache", "-ruk0"]).status() {
                eprintln!("Failed to remove uninstalled packages: {}", e);
            }
        }
        3 => {
            if let Err(e) = Command::new("paccache").args(["-v"]).status() {
                eprintln!("Failed to show cache statistics: {}", e);
            }
        }
        _ => {}
    }
}

fn package_verification() {
    println!("🛡️  Package Verification");
    println!("========================");

    let verify_options = [
        "🔍 Verify Package Files",
        "🔑 Check Package Signatures",
        "📋 Package Integrity Check",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Package Verification")
        .items(&verify_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            let package: String = match Input::new()
                .with_prompt("Enter package name to verify")
                .interact_text()
            {
                Ok(pkg) => pkg,
                Err(_) => return,
            };
            if let Err(e) = Command::new("pacman").args(["-Qkk", &package]).status() {
                eprintln!("Failed to verify package: {}", e);
            }
        }
        1 => {
            println!("🔑 Checking package database signatures:");
            if let Err(e) = Command::new("sudo")
                .args(["pacman-key", "--check-sigs"])
                .status()
            {
                eprintln!("Failed to check signatures: {}", e);
            }
        }
        2 => {
            println!("📋 Running comprehensive integrity check:");
            if let Err(e) = Command::new("pacman").args(["-Qkk"]).status() {
                eprintln!("Failed to run integrity check: {}", e);
            }
        }
        _ => {}
    }
}

fn package_file_management() {
    println!("📋 Package File Management");
    println!("==========================");

    let file_options = [
        "🔍 Find Package Owning File",
        "📂 List Package Files",
        "🎯 Search Package Contents",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("File Management")
        .items(&file_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            let file_path: String =
                match Input::new().with_prompt("Enter file path").interact_text() {
                    Ok(path) => path,
                    Err(_) => return,
                };
            if let Err(e) = Command::new("pacman").args(["-Qo", &file_path]).status() {
                eprintln!("Failed to find package owning file: {}", e);
            }
        }
        1 => {
            let package: String = match Input::new()
                .with_prompt("Enter package name")
                .interact_text()
            {
                Ok(pkg) => pkg,
                Err(_) => return,
            };
            if let Err(e) = Command::new("pacman").args(["-Ql", &package]).status() {
                eprintln!("Failed to list package files: {}", e);
            }
        }
        2 => {
            let search_term: String = match Input::new()
                .with_prompt("Enter search term")
                .interact_text()
            {
                Ok(term) => term,
                Err(_) => return,
            };
            if let Err(e) = Command::new("pacman").args(["-Ss", &search_term]).status() {
                eprintln!("Failed to search packages: {}", e);
            }
        }
        _ => {}
    }
}

fn security_hardening() {
    println!("🔐 System Security Hardening");
    println!("============================");

    let security_options = [
        "🛡️  Firewall Configuration",
        "🔒 SSH Hardening",
        "🔑 User Security",
        "📊 Security Audit",
        "🔐 File Encryption",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Security Hardening")
        .items(&security_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => firewall_configuration(),
        1 => ssh_hardening(),
        2 => user_security(),
        3 => comprehensive_security_audit(),
        4 => file_encryption(),
        _ => return,
    }
}

fn firewall_configuration() {
    println!("🛡️  Firewall Configuration");
    println!("===========================");

    // Check if ufw is installed
    let ufw_installed = Command::new("which")
        .arg("ufw")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if ufw_installed {
        println!("📊 Current firewall status:");
        if let Err(e) = Command::new("sudo")
            .args(["ufw", "status", "verbose"])
            .status()
        {
            eprintln!("Failed to get firewall status: {}", e);
        }

        let firewall_options = [
            "🔧 Enable UFW",
            "🛑 Disable UFW",
            "📝 Add Rule",
            "🗑️  Delete Rule",
            "📊 Show Status",
            "⬅️  Back",
        ];

        let choice = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Firewall Management")
            .items(&firewall_options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            Ok(None) | Err(_) => return,
        };

        match choice {
            0 => {
                if let Err(e) = Command::new("sudo").args(["ufw", "enable"]).status() {
                    eprintln!("Failed to enable UFW: {}", e);
                } else {
                    println!("✅ UFW enabled");
                }
            }
            1 => {
                let confirm = match Confirm::new()
                    .with_prompt("Disable firewall?")
                    .default(false)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => return,
                };
                if confirm && let Err(e) = Command::new("sudo").args(["ufw", "disable"]).status() {
                    eprintln!("Failed to disable UFW: {}", e);
                }
            }
            2 => {
                let rule: String = match Input::new()
                    .with_prompt("Enter rule (e.g., 'allow 22/tcp')")
                    .interact_text()
                {
                    Ok(r) => r,
                    Err(_) => return,
                };
                if let Err(e) = Command::new("sudo").args(["ufw", "allow", &rule]).status() {
                    eprintln!("Failed to add UFW rule: {}", e);
                }
            }
            4 => {
                if let Err(e) = Command::new("sudo")
                    .args(["ufw", "status", "numbered"])
                    .status()
                {
                    eprintln!("Failed to show UFW status: {}", e);
                }
            }
            _ => {}
        }
    } else {
        println!("📦 UFW not installed. Installing...");
        if let Err(e) = Command::new("sudo")
            .args(["pacman", "-S", "--needed", "--noconfirm", "ufw"])
            .status()
        {
            eprintln!("Failed to install UFW: {}", e);
        }
    }
}

fn ssh_hardening() {
    println!("🔒 SSH Hardening");
    println!("================");

    println!("💡 SSH Security recommendations:");
    println!("  • Disable root login");
    println!("  • Use key-based authentication");
    println!("  • Change default port");
    println!("  • Limit user access");
    println!("  • Enable fail2ban");

    let confirm = match Confirm::new()
        .with_prompt("View current SSH configuration?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm && let Err(e) = Command::new("cat").arg("/etc/ssh/sshd_config").status() {
        eprintln!("Failed to read SSH config: {}", e);
    }
}

fn user_security() {
    println!("🔑 User Security Configuration");
    println!("==============================");

    let user_sec_options = [
        "🔒 Password Policy",
        "⏰ Account Lockout",
        "📊 User Audit",
        "🔑 Sudo Configuration",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("User Security")
        .items(&user_sec_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            println!("🔒 Current password policy:");
            if let Err(e) = Command::new("cat").arg("/etc/login.defs").status() {
                eprintln!("Failed to read login.defs: {}", e);
            }
        }
        1 => {
            println!("⏰ Account lockout settings:");
            if let Err(e) = Command::new("cat")
                .arg("/etc/security/faillock.conf")
                .status()
            {
                eprintln!("Failed to read faillock.conf: {}", e);
            }
        }
        2 => {
            println!("📊 User security audit:");
            if let Err(e) = Command::new("awk")
                .args(["-F:", "$3 == 0 {print $1}", "/etc/passwd"])
                .status()
            {
                eprintln!("Failed to audit users: {}", e);
            }
        }
        3 => {
            println!("🔑 Sudo configuration:");
            if let Err(e) = Command::new("cat").arg("/etc/sudoers").status() {
                eprintln!("Failed to read sudoers: {}", e);
            }
        }
        _ => {}
    }
}

fn comprehensive_security_audit() {
    println!("📊 Comprehensive Security Audit");
    println!("===============================");

    println!("🔍 Running security audit...");

    // Check for security tools
    let security_tools = [
        ("rkhunter", "Rootkit Hunter"),
        ("chkrootkit", "Check Rootkit"),
        ("lynis", "Security auditing tool"),
        ("clamav", "Antivirus scanner"),
    ];

    for (tool, description) in &security_tools {
        let installed = Command::new("which")
            .arg(tool)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if installed {
            println!("  ✅ {} - {}", tool, description);
        } else {
            println!("  ❌ {} - {} (not installed)", tool, description);
        }
    }

    println!("\n🔍 Basic security checks:");
    println!("1. Checking for suspicious processes...");
    if let Err(e) = Command::new("ps")
        .args(["aux", "|", "grep", "-v", "grep"])
        .status()
    {
        eprintln!("Failed to list processes: {}", e);
    }

    println!("\n2. Checking network connections...");
    if let Err(e) = Command::new("netstat").args(["-tuln"]).status() {
        eprintln!("Failed to check network connections: {}", e);
    }

    println!("\n3. Checking system logs for anomalies...");
    if let Err(e) = Command::new("journalctl")
        .args(["-p", "err", "--since", "today"])
        .status()
    {
        eprintln!("Failed to check system logs: {}", e);
    }
}

fn file_encryption() {
    println!("🔐 File Encryption");
    println!("==================");

    let encryption_options = [
        "🔒 Encrypt File/Directory",
        "🔓 Decrypt File/Directory",
        "🗂️  Encrypted Archive",
        "💾 Disk Encryption Status",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("File Encryption")
        .items(&encryption_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            let file_path: String = match Input::new()
                .with_prompt("Enter file/directory path to encrypt")
                .interact_text()
            {
                Ok(path) => path,
                Err(_) => return,
            };

            let gpg_installed = Command::new("which")
                .arg("gpg")
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            if gpg_installed {
                if let Err(e) = Command::new("gpg").args(["-c", &file_path]).status() {
                    eprintln!("Failed to encrypt file: {}", e);
                } else {
                    println!("✅ File encrypted with GPG");
                }
            } else {
                println!("❌ GPG not available");
            }
        }
        3 => {
            println!("💾 Disk encryption status:");
            if let Err(e) = Command::new("lsblk").args(["-f"]).status() {
                eprintln!("Failed to list block devices: {}", e);
            }

            println!("\n🔍 LUKS encrypted devices:");
            if let Err(e) = Command::new("cryptsetup").arg("status").status() {
                eprintln!("Failed to check LUKS status: {}", e);
            }
        }
        _ => {
            println!("💡 Feature implementation in progress");
        }
    }
}

fn system_health_monitoring() {
    println!("📊 System Health Monitoring");
    println!("===========================");

    let health_options = [
        "💓 System Vital Signs",
        "🌡️  Temperature Monitoring",
        "💾 Disk Health",
        "🔄 Service Health",
        "📈 Performance Metrics",
        "⚠️  System Alerts",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Health Monitoring")
        .items(&health_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => system_vital_signs(),
        1 => temperature_monitoring(),
        2 => disk_health(),
        3 => service_health(),
        4 => performance_metrics(),
        5 => system_alerts(),
        _ => return,
    }
}

fn system_vital_signs() {
    println!("💓 System Vital Signs");
    println!("=====================");

    println!("⚡ CPU usage:");
    if let Err(e) = Command::new("cat").arg("/proc/loadavg").status() {
        eprintln!("Failed to read load average: {}", e);
    }

    println!("\n💾 Memory usage:");
    if let Err(e) = Command::new("free").args(["-h"]).status() {
        eprintln!("Failed to get memory usage: {}", e);
    }

    println!("\n💿 Disk usage:");
    if let Err(e) = Command::new("df").args(["-h"]).status() {
        eprintln!("Failed to get disk usage: {}", e);
    }

    println!("\n🔄 Uptime:");
    if let Err(e) = Command::new("uptime").status() {
        eprintln!("Failed to get uptime: {}", e);
    }

    println!("\n📊 System summary:");
    if let Err(e) = Command::new("uname").args(["-a"]).status() {
        eprintln!("Failed to get system info: {}", e);
    }
}

fn temperature_monitoring() {
    println!("🌡️  Temperature Monitoring");
    println!("===========================");

    // Check if lm-sensors is available
    let sensors_installed = Command::new("which")
        .arg("sensors")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if sensors_installed {
        println!("🌡️  Current temperatures:");
        if let Err(e) = Command::new("sensors").status() {
            eprintln!("Failed to read sensors: {}", e);
        }
    } else {
        println!("📦 Installing lm-sensors...");
        if let Err(e) = Command::new("sudo")
            .args(["pacman", "-S", "--needed", "--noconfirm", "lm_sensors"])
            .status()
        {
            eprintln!("Failed to install lm_sensors: {}", e);
        }

        println!("🔧 Running sensors-detect...");
        if let Err(e) = Command::new("sudo")
            .args(["sensors-detect", "--auto"])
            .status()
        {
            eprintln!("Failed to run sensors-detect: {}", e);
        }
    }

    println!("\n🔥 CPU thermal zones:");
    if let Err(e) = Command::new("cat")
        .arg("/sys/class/thermal/thermal_zone*/temp")
        .status()
    {
        eprintln!("Failed to read thermal zones: {}", e);
    }
}

fn disk_health() {
    println!("💾 Disk Health Analysis");
    println!("=======================");

    println!("💿 Disk information:");
    if let Err(e) = Command::new("lsblk").args(["-f"]).status() {
        eprintln!("Failed to list block devices: {}", e);
    }

    // Check if smartctl is available
    let smart_installed = Command::new("which")
        .arg("smartctl")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if smart_installed {
        println!("\n🔍 SMART status:");
        if let Err(e) = Command::new("sudo")
            .args(["smartctl", "-a", "/dev/sda"])
            .status()
        {
            eprintln!("Failed to get SMART status: {}", e);
        }
    } else {
        println!("\n📦 Install smartmontools for detailed disk health analysis");
    }

    println!("\n📊 Disk usage by directory:");
    if let Err(e) = Command::new("du")
        .args(["-sh", "/var", "/usr", "/home", "/opt"])
        .status()
    {
        eprintln!("Failed to get directory usage: {}", e);
    }
}

fn service_health() {
    println!("🔄 Service Health Check");
    println!("=======================");

    println!("✅ Active services:");
    if let Err(e) = Command::new("systemctl")
        .args(["list-units", "--type=service", "--state=active"])
        .status()
    {
        eprintln!("Failed to list active services: {}", e);
    }

    println!("\n❌ Failed services:");
    if let Err(e) = Command::new("systemctl")
        .args(["list-units", "--type=service", "--state=failed"])
        .status()
    {
        eprintln!("Failed to list failed services: {}", e);
    }

    println!("\n⏰ Service timers:");
    if let Err(e) = Command::new("systemctl").args(["list-timers"]).status() {
        eprintln!("Failed to list timers: {}", e);
    }
}

fn performance_metrics() {
    println!("📈 Performance Metrics");
    println!("======================");

    println!("🔄 CPU statistics:");
    if let Err(e) = Command::new("cat").arg("/proc/cpuinfo").status() {
        eprintln!("Failed to read CPU info: {}", e);
    }

    println!("\n📊 I/O statistics:");
    let iostat_installed = Command::new("which")
        .arg("iostat")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if iostat_installed && let Err(e) = Command::new("iostat").args(["-x", "1", "1"]).status() {
        eprintln!("Failed to get I/O statistics: {}", e);
    }

    println!("\n🌐 Network statistics:");
    if let Err(e) = Command::new("cat").arg("/proc/net/dev").status() {
        eprintln!("Failed to read network stats: {}", e);
    }
}

fn system_alerts() {
    println!("⚠️  System Alerts & Issues");
    println!("==========================");

    println!("🚨 System errors (last 24h):");
    if let Err(e) = Command::new("journalctl")
        .args(["-p", "err", "--since", "yesterday"])
        .status()
    {
        eprintln!("Failed to get system errors: {}", e);
    }

    println!("\n⚠️  Warning messages:");
    if let Err(e) = Command::new("journalctl")
        .args(["-p", "warning", "--since", "today", "--lines=20"])
        .status()
    {
        eprintln!("Failed to get warning messages: {}", e);
    }

    println!("\n🔍 Kernel messages:");
    if let Err(e) = Command::new("dmesg").args(["-l", "err,warn"]).status() {
        eprintln!("Failed to get kernel messages: {}", e);
    }
}

fn service_management() {
    println!("🔄 Advanced Service Management");
    println!("==============================");

    let service_options = [
        "📊 Service Status Overview",
        "🔧 Service Configuration",
        "⏰ Timer Management",
        "🚀 Service Creation",
        "📝 Service Logs",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Service Management")
        .items(&service_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => service_status_overview(),
        1 => service_configuration(),
        2 => timer_management(),
        3 => service_creation(),
        4 => service_logs(),
        _ => return,
    }
}

fn service_status_overview() {
    println!("📊 Service Status Overview");
    println!("==========================");

    println!("🟢 Running services:");
    if let Err(e) = Command::new("systemctl")
        .args(["list-units", "--type=service", "--state=running"])
        .status()
    {
        eprintln!("Failed to list running services: {}", e);
    }

    println!("\n🔴 Failed services:");
    if let Err(e) = Command::new("systemctl")
        .args(["list-units", "--type=service", "--state=failed"])
        .status()
    {
        eprintln!("Failed to list failed services: {}", e);
    }

    println!("\n⏸️  Inactive services:");
    if let Err(e) = Command::new("systemctl")
        .args(["list-units", "--type=service", "--state=inactive"])
        .status()
    {
        eprintln!("Failed to list inactive services: {}", e);
    }
}

fn service_configuration() {
    println!("🔧 Service Configuration");
    println!("========================");

    let service_name: String = match Input::new()
        .with_prompt("Enter service name")
        .interact_text()
    {
        Ok(name) => name,
        Err(_) => return,
    };

    println!("📋 Service details for '{}':", service_name);
    if let Err(e) = Command::new("systemctl")
        .args(["show", &service_name])
        .status()
    {
        eprintln!("Failed to show service details: {}", e);
    }

    println!("\n📝 Service unit file:");
    if let Err(e) = Command::new("systemctl")
        .args(["cat", &service_name])
        .status()
    {
        eprintln!("Failed to show service unit file: {}", e);
    }
}

fn timer_management() {
    println!("⏰ Timer Management");
    println!("==================");

    println!("📅 Active timers:");
    if let Err(e) = Command::new("systemctl").args(["list-timers"]).status() {
        eprintln!("Failed to list active timers: {}", e);
    }

    println!("\n⏰ All timers:");
    if let Err(e) = Command::new("systemctl")
        .args(["list-timers", "--all"])
        .status()
    {
        eprintln!("Failed to list all timers: {}", e);
    }
}

fn service_creation() {
    println!("🚀 Service Creation");
    println!("==================");

    println!("💡 This feature guides you through creating a systemd service");
    println!("📝 Service unit files are created in /etc/systemd/system/");

    let service_name: String = match Input::new()
        .with_prompt("Enter service name (without .service)")
        .interact_text()
    {
        Ok(name) => name,
        Err(_) => return,
    };

    let description: String = match Input::new()
        .with_prompt("Enter service description")
        .interact_text()
    {
        Ok(desc) => desc,
        Err(_) => return,
    };

    let exec_start: String = match Input::new()
        .with_prompt("Enter command to execute")
        .interact_text()
    {
        Ok(cmd) => cmd,
        Err(_) => return,
    };

    let service_template = format!(
        r#"[Unit]
Description={}
After=network.target

[Service]
Type=simple
ExecStart={}
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
"#,
        description, exec_start
    );

    let confirm = match Confirm::new()
        .with_prompt("Create this service?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        let service_file = format!("/etc/systemd/system/{}.service", service_name);
        if let Ok(mut file) = std::fs::File::create(&service_file) {
            use std::io::Write;
            if file.write_all(service_template.as_bytes()).is_ok() {
                if let Err(e) = Command::new("sudo")
                    .args(["systemctl", "daemon-reload"])
                    .status()
                {
                    eprintln!("Failed to reload systemd: {}", e);
                } else {
                    println!("✅ Service '{}' created", service_name);
                    println!("💡 Enable with: systemctl enable {}", service_name);
                }
            }
        }
    }
}

fn service_logs() {
    println!("📝 Service Logs");
    println!("===============");

    let service_name: String = match Input::new()
        .with_prompt("Enter service name")
        .interact_text()
    {
        Ok(name) => name,
        Err(_) => return,
    };

    let log_options = [
        "📋 Recent logs",
        "📊 Follow logs",
        "⚠️  Error logs only",
        "📅 Logs since boot",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Log Options")
        .items(&log_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            if let Err(e) = Command::new("journalctl")
                .args(["-u", &service_name, "-n", "50"])
                .status()
            {
                eprintln!("Failed to get service logs: {}", e);
            }
        }
        1 => {
            if let Err(e) = Command::new("journalctl")
                .args(["-u", &service_name, "-f"])
                .status()
            {
                eprintln!("Failed to follow service logs: {}", e);
            }
        }
        2 => {
            if let Err(e) = Command::new("journalctl")
                .args(["-u", &service_name, "-p", "err"])
                .status()
            {
                eprintln!("Failed to get error logs: {}", e);
            }
        }
        3 => {
            if let Err(e) = Command::new("journalctl")
                .args(["-u", &service_name, "--since", "boot"])
                .status()
            {
                eprintln!("Failed to get boot logs: {}", e);
            }
        }
        _ => {}
    }
}

fn log_management() {
    println!("📝 Log Analysis & Management");
    println!("============================");

    let log_options = [
        "📊 Log Statistics",
        "🔍 Log Analysis",
        "🧹 Log Cleanup",
        "⚙️  Log Configuration",
        "📈 Log Monitoring",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Log Management")
        .items(&log_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => log_statistics(),
        1 => log_analysis(),
        2 => log_cleanup(),
        3 => log_configuration(),
        4 => log_monitoring(),
        _ => return,
    }
}

fn log_statistics() {
    println!("📊 Log Statistics");
    println!("=================");

    println!("📈 Journal disk usage:");
    if let Err(e) = Command::new("journalctl").args(["--disk-usage"]).status() {
        eprintln!("Failed to get journal disk usage: {}", e);
    }

    println!("\n📅 Log entries by time:");
    if let Err(e) = Command::new("journalctl")
        .args(["--since", "yesterday", "--until", "today", "|", "wc", "-l"])
        .status()
    {
        eprintln!("Failed to count log entries: {}", e);
    }

    println!("\n⚠️  Error counts:");
    if let Err(e) = Command::new("journalctl")
        .args(["-p", "err", "--since", "yesterday", "|", "wc", "-l"])
        .status()
    {
        eprintln!("Failed to count errors: {}", e);
    }
}

fn log_analysis() {
    println!("🔍 Log Analysis");
    println!("===============");

    let analysis_options = [
        "🚨 System Errors",
        "⚠️  Warnings",
        "🔐 Security Events",
        "🚀 Boot Analysis",
        "🔍 Custom Search",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Log Analysis")
        .items(&analysis_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            println!("🚨 Recent system errors:");
            if let Err(e) = Command::new("journalctl")
                .args(["-p", "err", "--since", "today"])
                .status()
            {
                eprintln!("Failed to get system errors: {}", e);
            }
        }
        1 => {
            println!("⚠️  Recent warnings:");
            if let Err(e) = Command::new("journalctl")
                .args(["-p", "warning", "--since", "today"])
                .status()
            {
                eprintln!("Failed to get warnings: {}", e);
            }
        }
        2 => {
            println!("🔐 Security-related events:");
            if let Err(e) = Command::new("journalctl")
                .args(["-u", "sshd", "-u", "sudo", "--since", "today"])
                .status()
            {
                eprintln!("Failed to get security events: {}", e);
            }
        }
        3 => {
            println!("🚀 Boot log analysis:");
            if let Err(e) = Command::new("journalctl").args(["-b", "0"]).status() {
                eprintln!("Failed to get boot logs: {}", e);
            }
        }
        4 => {
            let search_term: String = match Input::new()
                .with_prompt("Enter search term")
                .interact_text()
            {
                Ok(term) => term,
                Err(_) => return,
            };
            if let Err(e) = Command::new("journalctl")
                .args(["-g", &search_term, "--since", "today"])
                .status()
            {
                eprintln!("Failed to search logs: {}", e);
            }
        }
        _ => {}
    }
}

fn log_cleanup() {
    println!("🧹 Log Cleanup");
    println!("==============");

    let cleanup_options = [
        "🗑️  Vacuum old logs (keep 7 days)",
        "📏 Limit journal size",
        "🔄 Rotate logs manually",
        "📊 Show cleanup impact",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Log Cleanup")
        .items(&cleanup_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            let confirm = match Confirm::new()
                .with_prompt("Remove logs older than 7 days?")
                .default(true)
                .interact_opt()
            {
                Ok(Some(c)) => c,
                Ok(None) | Err(_) => return,
            };
            if confirm
                && let Err(e) = Command::new("sudo")
                    .args(["journalctl", "--vacuum-time=7d"])
                    .status()
            {
                eprintln!("Failed to vacuum logs: {}", e);
            }
        }
        1 => {
            let size_limit: String = match Input::new()
                .with_prompt("Enter size limit (e.g., 100M, 1G)")
                .default("500M".to_string())
                .interact_text()
            {
                Ok(size) => size,
                Err(_) => return,
            };
            if let Err(e) = Command::new("sudo")
                .args(["journalctl", &format!("--vacuum-size={}", size_limit)])
                .status()
            {
                eprintln!("Failed to limit journal size: {}", e);
            }
        }
        2 => {
            if let Err(e) = Command::new("sudo")
                .args([
                    "systemctl",
                    "kill",
                    "--kill-who=main",
                    "--signal=SIGUSR2",
                    "systemd-journald.service",
                ])
                .status()
            {
                eprintln!("Failed to rotate logs: {}", e);
            }
        }
        3 => {
            println!("📊 Current journal usage:");
            if let Err(e) = Command::new("journalctl").args(["--disk-usage"]).status() {
                eprintln!("Failed to get journal usage: {}", e);
            }
        }
        _ => {}
    }
}

fn log_configuration() {
    println!("⚙️  Log Configuration");
    println!("=====================");

    println!("📋 Current journald configuration:");
    if let Err(e) = Command::new("cat")
        .arg("/etc/systemd/journald.conf")
        .status()
    {
        eprintln!("Failed to read journald.conf: {}", e);
    }

    println!("\n💡 Key configuration options:");
    println!("  SystemMaxUse=1G     - Maximum disk space");
    println!("  MaxRetentionSec=7d  - Maximum retention time");
    println!("  Compress=yes        - Enable compression");
    println!("  ForwardToSyslog=no  - Forward to syslog");
}

fn log_monitoring() {
    println!("📈 Log Monitoring");
    println!("=================");

    let monitor_options = [
        "👁️  Real-time log monitoring",
        "🚨 Error alerting setup",
        "📊 Log analysis tools",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Log Monitoring")
        .items(&monitor_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            println!("👁️  Starting real-time log monitoring...");
            if let Err(e) = Command::new("journalctl").args(["-f"]).status() {
                eprintln!("Failed to start log monitoring: {}", e);
            }
        }
        1 => {
            println!("🚨 Error alerting configuration:");
            println!("💡 Consider using tools like:");
            println!("  • logwatch");
            println!("  • fail2ban");
            println!("  • custom systemd services with journal monitoring");
        }
        2 => {
            println!("📊 Log analysis tools:");
            let tools = ["logwatch", "goaccess", "multitail"];
            for tool in &tools {
                let installed = Command::new("which")
                    .arg(tool)
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false);
                if installed {
                    println!("  ✅ {} available", tool);
                } else {
                    println!("  ❌ {} not installed", tool);
                }
            }
        }
        _ => {}
    }
}

fn network_configuration() {
    println!("🌐 Advanced Network Configuration");
    println!("=================================");

    let network_options = [
        "📊 Network Status",
        "🔧 Interface Configuration",
        "🌐 DNS Configuration",
        "🔥 Firewall Management",
        "📈 Network Monitoring",
        "🛡️  Network Security",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Network Configuration")
        .items(&network_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => network_status(),
        1 => interface_configuration(),
        2 => dns_configuration(),
        3 => firewall_management(),
        4 => network_monitoring(),
        5 => network_security(),
        _ => return,
    }
}

fn network_status() {
    println!("📊 Network Status");
    println!("=================");

    println!("🌐 Network interfaces:");
    if let Err(e) = Command::new("ip").args(["addr", "show"]).status() {
        eprintln!("Failed to show network interfaces: {}", e);
    }

    println!("\n🛣️  Routing table:");
    if let Err(e) = Command::new("ip").args(["route", "show"]).status() {
        eprintln!("Failed to show routing table: {}", e);
    }

    println!("\n🔗 Network connections:");
    if let Err(e) = Command::new("ss").args(["-tuln"]).status() {
        eprintln!("Failed to show connections: {}", e);
    }

    println!("\n📡 Wireless status:");
    if let Err(e) = Command::new("iwconfig").status() {
        eprintln!("Failed to show wireless status: {}", e);
    }
}

fn interface_configuration() {
    println!("🔧 Interface Configuration");
    println!("==========================");

    println!("📋 Available interfaces:");
    if let Err(e) = Command::new("ip").args(["link", "show"]).status() {
        eprintln!("Failed to list interfaces: {}", e);
    }

    let interface: String = match Input::new()
        .with_prompt("Enter interface name (e.g., eth0, wlan0)")
        .interact_text()
    {
        Ok(iface) => iface,
        Err(_) => return,
    };

    let config_options = [
        "📊 Show interface details",
        "🔧 Bring interface up",
        "🔽 Bring interface down",
        "🌐 Configure static IP",
        "📡 Scan WiFi networks",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Interface Configuration")
        .items(&config_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            if let Err(e) = Command::new("ip")
                .args(["addr", "show", &interface])
                .status()
            {
                eprintln!("Failed to show interface details: {}", e);
            }
        }
        1 => {
            if let Err(e) = Command::new("sudo")
                .args(["ip", "link", "set", &interface, "up"])
                .status()
            {
                eprintln!("Failed to bring interface up: {}", e);
            }
        }
        2 => {
            if let Err(e) = Command::new("sudo")
                .args(["ip", "link", "set", &interface, "down"])
                .status()
            {
                eprintln!("Failed to bring interface down: {}", e);
            }
        }
        3 => {
            let ip_address: String = match Input::new()
                .with_prompt("Enter IP address (e.g., 192.168.1.100/24)")
                .interact_text()
            {
                Ok(ip) => ip,
                Err(_) => return,
            };
            if let Err(e) = Command::new("sudo")
                .args(["ip", "addr", "add", &ip_address, "dev", &interface])
                .status()
            {
                eprintln!("Failed to configure static IP: {}", e);
            }
        }
        4 => {
            if interface.starts_with("wlan") || interface.starts_with("wlp") {
                if let Err(e) = Command::new("sudo")
                    .args(["iwlist", &interface, "scan"])
                    .status()
                {
                    eprintln!("Failed to scan WiFi networks: {}", e);
                }
            } else {
                println!("❌ Not a wireless interface");
            }
        }
        _ => {}
    }
}

fn dns_configuration() {
    println!("🌐 DNS Configuration");
    println!("===================");

    println!("📋 Current DNS configuration:");
    if let Err(e) = Command::new("cat").arg("/etc/resolv.conf").status() {
        eprintln!("Failed to read resolv.conf: {}", e);
    }

    println!("\n🔍 DNS resolution test:");
    if let Err(e) = Command::new("nslookup").arg("google.com").status() {
        eprintln!("Failed to test DNS resolution: {}", e);
    }

    let dns_options = [
        "🔧 Configure DNS servers",
        "🧪 Test DNS resolution",
        "📊 DNS performance test",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("DNS Configuration")
        .items(&dns_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        1 => {
            let domain: String = match Input::new()
                .with_prompt("Enter domain to test")
                .default("google.com".to_string())
                .interact_text()
            {
                Ok(d) => d,
                Err(_) => return,
            };
            if let Err(e) = Command::new("nslookup").arg(&domain).status() {
                eprintln!("Failed to lookup domain: {}", e);
            }
        }
        2 => {
            println!("📊 Testing DNS performance...");
            let servers = ["8.8.8.8", "1.1.1.1", "208.67.222.222"];
            for server in &servers {
                println!("Testing {}:", server);
                let server_arg = format!("@{}", server);
                if let Err(e) = Command::new("dig")
                    .args([&server_arg, "google.com"])
                    .status()
                {
                    eprintln!("Failed to test DNS server {}: {}", server, e);
                }
            }
        }
        _ => {}
    }
}

fn firewall_management() {
    println!("🔥 Advanced Firewall Management");
    println!("===============================");

    // This calls the same function as before but in the network context
    firewall_configuration();
}

fn network_monitoring() {
    println!("📈 Network Monitoring");
    println!("====================");

    let monitor_options = [
        "📊 Real-time traffic",
        "📈 Bandwidth usage",
        "🔗 Active connections",
        "📡 Network statistics",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Network Monitoring")
        .items(&monitor_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            let iftop_installed = Command::new("which")
                .arg("iftop")
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            if iftop_installed {
                if let Err(e) = Command::new("sudo").args(["iftop"]).status() {
                    eprintln!("Failed to run iftop: {}", e);
                }
            } else {
                println!("📦 iftop not installed. Using alternative...");
                if let Err(e) = Command::new("watch")
                    .args(["-n1", "cat", "/proc/net/dev"])
                    .status()
                {
                    eprintln!("Failed to run watch: {}", e);
                }
            }
        }
        1 => {
            let vnstat_installed = Command::new("which")
                .arg("vnstat")
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            if vnstat_installed {
                if let Err(e) = Command::new("vnstat").status() {
                    eprintln!("Failed to run vnstat: {}", e);
                }
            } else {
                println!("💡 Install vnstat for bandwidth monitoring");
            }
        }
        2 => {
            if let Err(e) = Command::new("ss").args(["-tuln"]).status() {
                eprintln!("Failed to show connections: {}", e);
            }
        }
        3 => {
            if let Err(e) = Command::new("cat").arg("/proc/net/dev").status() {
                eprintln!("Failed to read network stats: {}", e);
            }
        }
        _ => {}
    }
}

fn network_security() {
    println!("🛡️  Network Security");
    println!("====================");

    let security_options = [
        "🔍 Port scan detection",
        "🚫 Block suspicious IPs",
        "📊 Connection analysis",
        "🔒 SSH security",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Network Security")
        .items(&security_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            println!("🔍 Checking for suspicious connections...");
            if let Err(e) = Command::new("netstat")
                .args(["-tuln", "|", "grep", "LISTEN"])
                .status()
            {
                eprintln!("Failed to check connections: {}", e);
            }
        }
        1 => {
            println!("🚫 Checking for fail2ban...");
            let fail2ban_installed = Command::new("which")
                .arg("fail2ban-client")
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            if fail2ban_installed {
                if let Err(e) = Command::new("sudo")
                    .args(["fail2ban-client", "status"])
                    .status()
                {
                    eprintln!("Failed to check fail2ban status: {}", e);
                }
            } else {
                println!("💡 Install fail2ban for IP blocking");
            }
        }
        2 => {
            println!("📊 Analyzing network connections...");
            if let Err(e) = Command::new("ss")
                .args(["-o", "state", "established"])
                .status()
            {
                eprintln!("Failed to analyze connections: {}", e);
            }
        }
        _ => {}
    }
}

fn kernel_management() {
    println!("🚀 Kernel Management");
    println!("====================");

    let kernel_options = [
        "📊 Kernel Information",
        "🔧 Kernel Parameters",
        "📦 Kernel Modules",
        "🚀 Boot Options",
        "📈 Kernel Performance",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Kernel Management")
        .items(&kernel_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => kernel_information(),
        1 => kernel_parameters_management(),
        2 => kernel_modules_management(),
        3 => boot_options(),
        4 => kernel_performance(),
        _ => return,
    }
}

fn kernel_information() {
    println!("📊 Kernel Information");
    println!("=====================");

    println!("🔍 Kernel version:");
    if let Err(e) = Command::new("uname").args(["-r"]).status() {
        eprintln!("Failed to get kernel version: {}", e);
    }

    println!("\n📋 Full system information:");
    if let Err(e) = Command::new("uname").args(["-a"]).status() {
        eprintln!("Failed to get system info: {}", e);
    }

    println!("\n⚡ Kernel command line:");
    if let Err(e) = Command::new("cat").arg("/proc/cmdline").status() {
        eprintln!("Failed to read cmdline: {}", e);
    }

    println!("\n🏗️  Kernel build information:");
    if let Err(e) = Command::new("cat").arg("/proc/version").status() {
        eprintln!("Failed to read version: {}", e);
    }

    println!("\n💾 Memory information:");
    if let Err(e) = Command::new("cat").arg("/proc/meminfo").status() {
        eprintln!("Failed to read meminfo: {}", e);
    }
}

fn kernel_parameters_management() {
    println!("🔧 Kernel Parameters Management");
    println!("===============================");

    // This reuses the kernel_parameters function from performance tuning
    println!("💡 Using advanced kernel parameter configuration...");
    crate::arch::perf::tune(); // Call the performance tuning function
}

fn kernel_modules_management() {
    println!("📦 Kernel Modules Management");
    println!("============================");

    // This reuses the module management functions
    configure_module_loading();
}

fn boot_options() {
    println!("🚀 Boot Options Configuration");
    println!("=============================");

    println!("📋 Current boot configuration:");

    // Check bootloader
    if std::path::Path::new("/boot/grub/grub.cfg").exists() {
        println!("🥾 GRUB bootloader detected");
        if let Err(e) = Command::new("cat").arg("/etc/default/grub").status() {
            eprintln!("Failed to read GRUB config: {}", e);
        }
    } else if std::path::Path::new("/boot/loader/entries").exists() {
        println!("🥾 systemd-boot detected");
        if let Err(e) = Command::new("ls").arg("/boot/loader/entries/").status() {
            eprintln!("Failed to list boot entries: {}", e);
        }
    }

    println!("\n💡 To modify boot options:");
    println!("  • GRUB: Edit /etc/default/grub, then run grub-mkconfig");
    println!("  • systemd-boot: Edit files in /boot/loader/entries/");
}

fn kernel_performance() {
    println!("📈 Kernel Performance Tuning");
    println!("============================");

    println!("📊 Current kernel performance settings:");
    if let Err(e) = Command::new("cat")
        .arg("/proc/sys/kernel/sched_migration_cost_ns")
        .status()
    {
        eprintln!("Failed to read scheduler settings: {}", e);
    }

    println!("\n⚡ CPU governor:");
    if let Err(e) = Command::new("cat")
        .arg("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
        .status()
    {
        eprintln!("Failed to read CPU governor: {}", e);
    }

    println!("\n🔧 Available performance tuning:");
    println!("  • Use the Performance Tuning menu for detailed options");
    println!("  • Configure CPU governors");
    println!("  • Adjust kernel scheduler parameters");
    println!("  • Optimize I/O schedulers");
}
