use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::path::Path;
use std::process::Command;

pub fn boot_management() {
    println!("🥾 Arch Linux Boot Management");
    println!("=============================");

    let options = [
        "🐧 Kernel Management",
        "⚙️  Systemd-boot Configuration",
        "📋 List Boot Entries",
        "🔄 Regenerate Boot Entries",
        "🔍 Boot Diagnostics",
        "🎯 Set Default Boot Entry",
        "📊 Kernel Information",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Boot Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => kernel_management(),
        1 => systemd_boot_config(),
        2 => list_boot_entries(),
        3 => regenerate_boot_entries(),
        4 => boot_diagnostics(),
        5 => set_default_boot_entry(),
        6 => kernel_information(),
        _ => return,
    }
}

fn kernel_management() {
    println!("🐧 Kernel Management");
    println!("===================");

    let options = [
        "📋 List Installed Kernels",
        "🔽 Install New Kernel",
        "🗑️  Remove Kernel",
        "🔄 Update All Kernels",
        "⚙️  Kernel Configuration",
        "📦 Popular Kernel Options",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Kernel Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => list_installed_kernels(),
        1 => install_kernel(),
        2 => remove_kernel(),
        3 => update_kernels(),
        4 => kernel_config(),
        5 => popular_kernels(),
        _ => return,
    }
}

fn list_installed_kernels() {
    println!("📋 Installed Kernels:");
    println!("====================");

    // List installed kernel packages and filter for kernels in Rust
    println!("\n🔍 Kernel packages:");
    let output = Command::new("pacman").args(["-Q"]).output();
    if let Ok(out) = output {
        let content = String::from_utf8_lossy(&out.stdout);
        for line in content.lines() {
            if line.starts_with("linux") {
                println!("  {}", line);
            }
        }
    }

    // Check /boot for kernel files
    println!("\n📁 Kernel files in /boot:");
    if Path::new("/boot").exists()
        && let Ok(entries) = std::fs::read_dir("/boot")
    {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("vmlinuz-") {
                println!("  {}", name_str);
            }
        }
    }
}

fn popular_kernels() {
    println!("📦 Popular Arch Linux Kernels");
    println!("=============================");

    let kernels = [
        ("linux", "🐧 Vanilla Linux kernel"),
        ("linux-lts", "🛡️  Long Term Support kernel"),
        ("linux-zen", "⚡ Performance-oriented kernel"),
        ("linux-hardened", "🔒 Security-focused kernel"),
        ("linux-tkg", "🚀 Custom optimized kernel (AUR)"),
        ("linux-cachy", "⚡ CachyOS performance kernel (AUR)"),
        ("linux-xanmod", "🎮 Gaming/desktop optimized (AUR)"),
        ("linux-clear", "📈 Intel Clear Linux patches (AUR)"),
    ];

    println!("Available kernel options:");
    for (i, (name, desc)) in kernels.iter().enumerate() {
        println!("{}. {} - {}", i + 1, name, desc);
    }

    let install_options = kernels
        .iter()
        .map(|(name, desc)| format!("{} - {}", name, desc))
        .collect::<Vec<_>>();
    let mut all_options = install_options;
    all_options.push("⬅️  Back".to_string());

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select kernel to install")
        .items(&all_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if choice < kernels.len() {
        let (kernel_name, _) = kernels[choice];
        install_specific_kernel(kernel_name);
    }
}

fn install_specific_kernel(kernel_name: &str) {
    println!("🔽 Installing kernel: {}", kernel_name);

    let confirm = match Confirm::new()
        .with_prompt(&format!(
            "Install {} kernel? This will also install headers.",
            kernel_name
        ))
        .default(true)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        // Install kernel and headers
        if kernel_name.contains("tkg")
            || kernel_name.contains("cachy")
            || kernel_name.contains("xanmod")
            || kernel_name.contains("clear")
        {
            println!("📦 Installing AUR kernel (requires yay or similar)...");
            let status = Command::new("yay")
                .args([
                    "-S",
                    "--noconfirm",
                    kernel_name,
                    &format!("{}-headers", kernel_name),
                ])
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("✅ Kernel installed successfully");
                    regenerate_boot_entries();
                }
                _ => println!("❌ Failed to install kernel. Make sure yay is installed."),
            }
        } else {
            println!("📦 Installing official repository kernel...");
            let status = Command::new("sudo")
                .args([
                    "pacman",
                    "-S",
                    "--noconfirm",
                    kernel_name,
                    &format!("{}-headers", kernel_name),
                ])
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("✅ Kernel installed successfully");
                    regenerate_boot_entries();
                }
                _ => println!("❌ Failed to install kernel"),
            }
        }
    }
}

fn systemd_boot_config() {
    println!("⚙️  Systemd-boot Configuration");
    println!("==============================");

    let options = [
        "📋 Show Current Configuration",
        "📝 Edit loader.conf",
        "🆕 Create Boot Entry",
        "📁 Manage Boot Entries",
        "🔧 Setup Systemd-boot",
        "🔄 Update Systemd-boot",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Systemd-boot Configuration")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => show_systemd_boot_config(),
        1 => edit_loader_conf(),
        2 => create_boot_entry(),
        3 => manage_boot_entries(),
        4 => setup_systemd_boot(),
        5 => update_systemd_boot(),
        _ => return,
    }
}

fn show_systemd_boot_config() {
    println!("📋 Current Systemd-boot Configuration");
    println!("=====================================");

    if Path::new("/boot/loader/loader.conf").exists() {
        println!("📄 /boot/loader/loader.conf:");
        let _ = Command::new("cat").arg("/boot/loader/loader.conf").status();
    } else {
        println!("❌ loader.conf not found. Systemd-boot may not be installed.");
    }

    println!("\n📁 Boot entries:");
    if Path::new("/boot/loader/entries").exists() {
        let _ = Command::new("ls")
            .args(["-la", "/boot/loader/entries/"])
            .status();
    } else {
        println!("❌ Boot entries directory not found.");
    }
}

fn edit_loader_conf() {
    println!("📝 Editing loader.conf");

    if !Path::new("/boot/loader/loader.conf").exists() {
        println!("❌ loader.conf not found. Creating default configuration...");
        create_default_loader_conf();
    }

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    let _ = Command::new("sudo")
        .args(&[&editor, "/boot/loader/loader.conf"])
        .status();
}

fn create_default_loader_conf() {
    let default_config = r#"default  arch.conf
timeout  4
console-mode max
editor   no
"#;

    let _ = Command::new("sudo")
        .args(["mkdir", "-p", "/boot/loader"])
        .status();

    // Write to a temp file and move with sudo
    if std::fs::write("/tmp/loader.conf.tmp", default_config).is_ok() {
        let _ = Command::new("sudo")
            .args(["mv", "/tmp/loader.conf.tmp", "/boot/loader/loader.conf"])
            .status();
    }

    println!("✅ Default loader.conf created");
}

fn create_boot_entry() {
    println!("🆕 Create Boot Entry");
    println!("===================");

    let entry_name: String = match Input::new()
        .with_prompt("Entry name (e.g., 'arch-zen')")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    let kernel_file: String = match Input::new()
        .with_prompt("Kernel file (e.g., 'vmlinuz-linux-zen')")
        .default("vmlinuz-linux".into())
        .interact_text()
    {
        Ok(k) => k,
        Err(_) => return,
    };

    let initrd_file: String = match Input::new()
        .with_prompt("Initrd file (e.g., 'initramfs-linux-zen.img')")
        .default("initramfs-linux.img".into())
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    // Get root UUID
    println!("🔍 Detecting root partition...");
    let root_uuid = get_root_uuid();

    let kernel_params: String = match Input::new()
        .with_prompt("Additional kernel parameters")
        .default("quiet rw".into())
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let entry_content = format!(
        r#"title   Arch Linux ({})
linux   /{}
initrd  /{}
options root=UUID={} {} 
"#,
        entry_name, kernel_file, initrd_file, root_uuid, kernel_params
    );

    let entry_filename = format!("/boot/loader/entries/{}.conf", entry_name);

    println!("📝 Creating boot entry: {}", entry_filename);
    println!("Content:\n{}", entry_content);

    let confirm = match Confirm::new()
        .with_prompt("Create this boot entry?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        let _ = Command::new("sudo")
            .args(["mkdir", "-p", "/boot/loader/entries"])
            .status();

        // Write to temp file and move with sudo
        let temp_file = "/tmp/boot_entry.conf.tmp";
        if std::fs::write(temp_file, &entry_content).is_ok() {
            let _ = Command::new("sudo")
                .args(["mv", temp_file, &entry_filename])
                .status();
        }

        println!("✅ Boot entry created successfully");
    }
}

fn get_root_uuid() -> String {
    let output = Command::new("findmnt")
        .args(&["-n", "-o", "UUID", "/"])
        .output();

    match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        Err(_) => {
            println!("❌ Could not detect root UUID. Please enter manually.");
            Input::new()
                .with_prompt("Root partition UUID")
                .interact_text()
                .unwrap_or_default()
        }
    }
}

fn list_boot_entries() {
    println!("📋 Boot Entries");
    println!("===============");

    let _ = Command::new("bootctl").arg("list").status();
}

pub fn regenerate_boot_entries() {
    println!("🔄 Regenerating Boot Entries");
    println!("============================");

    // Check if mkinitcpio presets exist
    if Path::new("/etc/mkinitcpio.d").exists() {
        println!("🔍 Found mkinitcpio presets:");
        let _ = Command::new("ls").args(&["/etc/mkinitcpio.d/"]).status();

        let regenerate = match Confirm::new()
            .with_prompt("Regenerate initramfs for all kernels?")
            .default(true)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            Ok(None) | Err(_) => return,
        };

        if regenerate {
            println!("🔄 Regenerating initramfs...");
            let _ = Command::new("sudo").args(&["mkinitcpio", "-P"]).status();
        }
    }

    // Update systemd-boot if installed
    if Path::new("/boot/EFI/systemd").exists() {
        println!("🔄 Updating systemd-boot...");
        let _ = Command::new("sudo").args(&["bootctl", "update"]).status();
    }

    println!("✅ Boot entries regenerated");
}

fn boot_diagnostics() {
    println!("🔍 Boot Diagnostics");
    println!("==================");

    println!("📊 Boot loader status:");
    let _ = Command::new("bootctl").arg("status").status();

    println!("\n🔍 EFI variables:");
    let _ = Command::new("efibootmgr").arg("-v").status();

    println!("\n📁 Boot partition info:");
    let _ = Command::new("df").args(&["-h", "/boot"]).status();
}

fn set_default_boot_entry() {
    println!("🎯 Set Default Boot Entry");
    // Implementation for setting default boot entry
}

fn kernel_information() {
    println!("📊 Kernel Information");
    println!("====================");

    println!("🐧 Current kernel:");
    let _ = Command::new("uname").arg("-a").status();

    println!("\n📦 Kernel modules:");
    let _ = Command::new("lsmod").status();
}

fn install_kernel() {
    println!("🔽 Install New Kernel");
    println!("====================");

    let kernels = [
        ("linux", "Vanilla Linux kernel (stable)"),
        ("linux-lts", "Long Term Support kernel"),
        ("linux-zen", "Performance-oriented kernel"),
        ("linux-hardened", "Security-focused kernel"),
    ];

    let kernel_options: Vec<String> = kernels
        .iter()
        .map(|(name, desc)| format!("{} - {}", name, desc))
        .collect();

    let mut options = kernel_options.clone();
    options.push("Back".to_string());

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select kernel to install")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if choice < kernels.len() {
        let (kernel_name, _) = kernels[choice];
        install_specific_kernel(kernel_name);
    }
}

fn remove_kernel() {
    println!("🗑️  Remove Kernel");
    println!("=================");

    // Get list of installed kernel packages using direct command and Rust filtering
    let output = match Command::new("pacman").args(["-Q"]).output() {
        Ok(out) => out,
        Err(e) => {
            println!("Failed to list installed kernels: {}", e);
            return;
        }
    };

    let output_str = String::from_utf8_lossy(&output.stdout);
    // Filter for kernel packages (start with "linux" and don't contain "headers")
    let installed_kernels: Vec<&str> = output_str
        .lines()
        .filter(|line| {
            let pkg_name = line.split_whitespace().next().unwrap_or("");
            pkg_name.starts_with("linux")
                && !pkg_name.contains("headers")
                && (pkg_name == "linux" || pkg_name.starts_with("linux-"))
        })
        .collect();

    if installed_kernels.is_empty() {
        println!("No kernel packages found.");
        return;
    }

    if installed_kernels.len() == 1 {
        println!("Only one kernel is installed. Cannot remove the last kernel.");
        println!("Install another kernel first before removing this one.");
        return;
    }

    println!("Installed kernels:");
    for kernel in &installed_kernels {
        println!("  - {}", kernel);
    }

    let kernel_names: Vec<String> = installed_kernels
        .iter()
        .map(|k| k.split_whitespace().next().unwrap_or("").to_string())
        .collect();

    let mut options = kernel_names.clone();
    options.push("Back".to_string());

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select kernel to remove")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if choice >= kernel_names.len() {
        return;
    }

    let kernel_to_remove = &kernel_names[choice];

    // Check if this is the currently running kernel
    let uname_output = Command::new("uname").arg("-r").output();
    if let Ok(out) = uname_output {
        let current_kernel = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if current_kernel.contains(
            kernel_to_remove
                .trim_start_matches("linux-")
                .trim_start_matches("linux"),
        ) {
            println!("Warning: This appears to be the currently running kernel.");
            println!("You should reboot into another kernel before removing it.");
        }
    }

    let confirm = match Confirm::new()
        .with_prompt(&format!(
            "Are you sure you want to remove {}? This will also remove its headers.",
            kernel_to_remove
        ))
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        let headers_package = format!("{}-headers", kernel_to_remove);

        println!("Removing {} and {}...", kernel_to_remove, headers_package);

        // Try to remove both kernel and headers
        let status = Command::new("sudo")
            .args(["pacman", "-R", kernel_to_remove, &headers_package])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("Kernel removed successfully.");
                println!("Remember to update your boot entries if needed.");
            }
            Ok(_) => {
                // Headers might not exist, try removing just the kernel
                let status2 = Command::new("sudo")
                    .args(["pacman", "-R", kernel_to_remove])
                    .status();
                match status2 {
                    Ok(s) if s.success() => {
                        println!("Kernel removed successfully.");
                    }
                    _ => println!("Failed to remove kernel."),
                }
            }
            Err(e) => println!("Failed to remove kernel: {}", e),
        }
    }
}

fn update_kernels() {
    println!("🔄 Update All Kernels");
    let _ = Command::new("sudo").args(&["pacman", "-Syu"]).status();
}

fn kernel_config() {
    println!("⚙️  Kernel Configuration");
    println!("========================");

    let options = [
        "Edit kernel parameters",
        "Manage modules",
        "Configure mkinitcpio",
        "Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Kernel Configuration")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => edit_kernel_parameters(),
        1 => manage_modules(),
        2 => configure_mkinitcpio(),
        _ => return,
    }
}

fn edit_kernel_parameters() {
    println!("📝 Edit Kernel Parameters");
    println!("=========================");

    let param_options = [
        "Edit /etc/kernel/cmdline (for unified kernel images)",
        "Edit boot entry directly",
        "View current kernel parameters",
        "Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Kernel Parameters")
        .items(&param_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            let cmdline_path = "/etc/kernel/cmdline";
            if !Path::new(cmdline_path).exists() {
                println!("File {} does not exist.", cmdline_path);
                let create = match Confirm::new()
                    .with_prompt("Create it with current kernel parameters?")
                    .default(true)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => return,
                };

                if create {
                    // Get current parameters from /proc/cmdline and write via temp file
                    let _ = Command::new("sudo")
                        .args(["mkdir", "-p", "/etc/kernel"])
                        .status();
                    if let Ok(cmdline) = std::fs::read_to_string("/proc/cmdline")
                        && std::fs::write("/tmp/cmdline.tmp", &cmdline).is_ok()
                    {
                        let _ = Command::new("sudo")
                            .args(["mv", "/tmp/cmdline.tmp", "/etc/kernel/cmdline"])
                            .status();
                    }
                }
            }

            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
            let _ = Command::new("sudo").args([&editor, cmdline_path]).status();
        }
        1 => {
            // List boot entries and let user select one to edit
            let entries_path = Path::new("/boot/loader/entries");
            if !entries_path.exists() {
                println!("Boot entries directory not found at /boot/loader/entries");
                return;
            }

            let entries: Vec<String> = match std::fs::read_dir(entries_path) {
                Ok(dir) => dir
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().extension().is_some_and(|ext| ext == "conf"))
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .collect(),
                Err(e) => {
                    println!("Failed to read boot entries: {}", e);
                    return;
                }
            };

            if entries.is_empty() {
                println!("No boot entries found.");
                return;
            }

            let mut options = entries.clone();
            options.push("Back".to_string());

            let choice = match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select boot entry to edit")
                .items(&options)
                .default(0)
                .interact_opt()
            {
                Ok(Some(c)) => c,
                Ok(None) | Err(_) => return,
            };

            if choice < entries.len() {
                let entry_path = format!("/boot/loader/entries/{}", entries[choice]);
                let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
                let _ = Command::new("sudo").args([&editor, &entry_path]).status();
            }
        }
        2 => {
            println!("Current kernel parameters:");
            let _ = Command::new("cat").arg("/proc/cmdline").status();
        }
        _ => return,
    }
}

fn manage_modules() {
    println!("📦 Manage Kernel Modules");
    println!("========================");

    let module_options = [
        "View currently loaded modules",
        "Edit MODULES in mkinitcpio.conf",
        "Add module to load at boot",
        "Blacklist a module",
        "Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Module Management")
        .items(&module_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            println!("Currently loaded modules:");
            let _ = Command::new("lsmod").status();
        }
        1 => {
            let mkinitcpio_path = "/etc/mkinitcpio.conf";
            if Path::new(mkinitcpio_path).exists() {
                println!("Current MODULES configuration:");
                // Read and filter mkinitcpio.conf in Rust instead of grep
                if let Ok(content) = std::fs::read_to_string(mkinitcpio_path) {
                    for line in content.lines() {
                        if line.starts_with("MODULES=") {
                            println!("{}", line);
                        }
                    }
                }

                let module_name: String = match Input::new()
                    .with_prompt("Enter module name to add (or leave empty to edit file directly)")
                    .allow_empty(true)
                    .interact_text()
                {
                    Ok(m) => m,
                    Err(_) => return,
                };

                if module_name.is_empty() {
                    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
                    let _ = Command::new("sudo")
                        .args([&editor, mkinitcpio_path])
                        .status();
                } else {
                    println!("Adding module '{}' to MODULES array...", module_name);
                    // Use sed directly without shell wrapper
                    let sed_pattern =
                        format!("s/^MODULES=(\\(.*\\))/MODULES=(\\1 {})'/", module_name);
                    let _ = Command::new("sudo")
                        .args(["sed", "-i", &sed_pattern, mkinitcpio_path])
                        .status();

                    let regenerate = match Confirm::new()
                        .with_prompt("Regenerate initramfs now?")
                        .default(true)
                        .interact_opt()
                    {
                        Ok(Some(c)) => c,
                        Ok(None) | Err(_) => return,
                    };

                    if regenerate {
                        let _ = Command::new("sudo").args(["mkinitcpio", "-P"]).status();
                    }
                }
            } else {
                println!("mkinitcpio.conf not found at {}", mkinitcpio_path);
            }
        }
        2 => {
            let module_name: String = match Input::new()
                .with_prompt("Enter module name to load at boot")
                .interact_text()
            {
                Ok(m) => m,
                Err(_) => return,
            };

            let conf_file = format!("/etc/modules-load.d/{}.conf", module_name);
            println!("Creating {}...", conf_file);

            // Write to temp file and move with sudo
            let temp_file = "/tmp/module_load.conf.tmp";
            if std::fs::write(temp_file, &format!("{}\n", module_name)).is_ok() {
                let status = Command::new("sudo")
                    .args(["mv", temp_file, &conf_file])
                    .status();
                match status {
                    Ok(s) if s.success() => {
                        println!("Module '{}' will be loaded at boot.", module_name);
                    }
                    _ => println!("Failed to create module configuration."),
                }
            } else {
                println!("Failed to create module configuration.");
            }
        }
        3 => {
            let module_name: String = match Input::new()
                .with_prompt("Enter module name to blacklist")
                .interact_text()
            {
                Ok(m) => m,
                Err(_) => return,
            };

            let conf_file = format!("/etc/modprobe.d/blacklist-{}.conf", module_name);
            println!("Creating {}...", conf_file);

            // Write to temp file and move with sudo
            let temp_file = "/tmp/blacklist.conf.tmp";
            if std::fs::write(temp_file, &format!("blacklist {}\n", module_name)).is_ok() {
                let status = Command::new("sudo")
                    .args(["mv", temp_file, &conf_file])
                    .status();
                match status {
                    Ok(s) if s.success() => {
                        println!("Module '{}' has been blacklisted.", module_name);
                        println!(
                            "Reboot or manually unload the module for changes to take effect."
                        );
                    }
                    _ => println!("Failed to create blacklist configuration."),
                }
            } else {
                println!("Failed to create blacklist configuration.");
            }
        }
        _ => return,
    }
}

fn configure_mkinitcpio() {
    println!("🔧 Configure mkinitcpio");
    println!("=======================");

    let mkinitcpio_path = "/etc/mkinitcpio.conf";

    if !Path::new(mkinitcpio_path).exists() {
        println!("mkinitcpio.conf not found.");
        return;
    }

    println!("Current mkinitcpio configuration:");
    // Read and filter mkinitcpio.conf in Rust instead of using grep
    if let Ok(content) = std::fs::read_to_string(mkinitcpio_path) {
        for line in content.lines() {
            if line.starts_with("MODULES=")
                || line.starts_with("BINARIES=")
                || line.starts_with("FILES=")
                || line.starts_with("HOOKS=")
            {
                println!("{}", line);
            }
        }
    }

    let config_options = [
        "Edit mkinitcpio.conf",
        "Regenerate initramfs for all kernels",
        "Regenerate initramfs for specific kernel",
        "View available hooks",
        "Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("mkinitcpio Configuration")
        .items(&config_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => {
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
            let _ = Command::new("sudo")
                .args([&editor, mkinitcpio_path])
                .status();

            let regenerate = match Confirm::new()
                .with_prompt("Regenerate initramfs after editing?")
                .default(true)
                .interact_opt()
            {
                Ok(Some(c)) => c,
                Ok(None) | Err(_) => return,
            };

            if regenerate {
                println!("Regenerating initramfs...");
                let _ = Command::new("sudo").args(["mkinitcpio", "-P"]).status();
            }
        }
        1 => {
            println!("Regenerating initramfs for all kernels...");
            let _ = Command::new("sudo").args(["mkinitcpio", "-P"]).status();
        }
        2 => {
            // List available presets
            let presets_output = Command::new("ls").arg("/etc/mkinitcpio.d/").output();
            if let Ok(output) = presets_output {
                let presets_str = String::from_utf8_lossy(&output.stdout);
                let presets: Vec<&str> = presets_str
                    .lines()
                    .filter(|l| l.ends_with(".preset"))
                    .collect();

                if presets.is_empty() {
                    println!("No presets found.");
                    return;
                }

                let mut options: Vec<String> = presets.iter().map(|p| p.to_string()).collect();
                options.push("Back".to_string());

                let choice = match Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select preset")
                    .items(&options)
                    .default(0)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => return,
                };

                if choice < presets.len() {
                    let preset = presets[choice].trim_end_matches(".preset");
                    println!("Regenerating initramfs for {}...", preset);
                    let _ = Command::new("sudo")
                        .args(["mkinitcpio", "-p", preset])
                        .status();
                }
            }
        }
        3 => {
            println!("Available mkinitcpio hooks:");
            let _ = Command::new("ls").arg("/usr/lib/initcpio/hooks/").status();
            println!("\nInstall hooks:");
            let _ = Command::new("ls")
                .arg("/usr/lib/initcpio/install/")
                .status();
        }
        _ => return,
    }
}

fn manage_boot_entries() {
    println!("📁 Manage Boot Entries");
    println!("======================");

    let entries_path = Path::new("/boot/loader/entries");
    if !entries_path.exists() {
        println!("Boot entries directory not found at /boot/loader/entries");
        println!("Systemd-boot may not be installed. Use 'Setup Systemd-boot' first.");
        return;
    }

    let entries: Vec<String> = match std::fs::read_dir(entries_path) {
        Ok(dir) => dir
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "conf"))
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect(),
        Err(e) => {
            println!("Failed to read boot entries: {}", e);
            return;
        }
    };

    if entries.is_empty() {
        println!("No boot entries found.");
        return;
    }

    let mut options = entries.clone();
    options.push("Back".to_string());

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select boot entry")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if choice >= entries.len() {
        return;
    }

    let selected_entry = &entries[choice];
    let entry_path = format!("/boot/loader/entries/{}", selected_entry);

    // Show entry content
    println!("\nContent of {}:", selected_entry);
    println!("---");
    let _ = Command::new("cat").arg(&entry_path).status();
    println!("---\n");

    let actions = ["View entry", "Edit entry", "Delete entry", "Back"];

    let action = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!("Action for {}", selected_entry))
        .items(&actions)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match action {
        0 => {
            // Already shown above, show again
            println!("Content of {}:", selected_entry);
            let _ = Command::new("cat").arg(&entry_path).status();
        }
        1 => {
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
            let _ = Command::new("sudo").args([&editor, &entry_path]).status();
            println!("Boot entry updated.");
        }
        2 => {
            let confirm = match Confirm::new()
                .with_prompt(&format!(
                    "Are you sure you want to delete {}? This cannot be undone.",
                    selected_entry
                ))
                .default(false)
                .interact_opt()
            {
                Ok(Some(c)) => c,
                Ok(None) | Err(_) => return,
            };

            if confirm {
                let status = Command::new("sudo").args(["rm", &entry_path]).status();
                match status {
                    Ok(s) if s.success() => {
                        println!("Boot entry deleted successfully.");
                    }
                    _ => println!("Failed to delete boot entry."),
                }
            }
        }
        _ => return,
    }
}

fn setup_systemd_boot() {
    println!("🔧 Setup Systemd-boot");
    println!("=====================");

    // Check if systemd-boot is already installed
    let bootctl_status = Command::new("bootctl").arg("status").output();

    let is_installed = match bootctl_status {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.contains("systemd-boot") && output.status.success()
        }
        Err(_) => false,
    };

    if is_installed {
        println!("Systemd-boot appears to be already installed.");
        let _ = Command::new("bootctl").arg("status").status();

        let update = match Confirm::new()
            .with_prompt("Would you like to update systemd-boot instead?")
            .default(true)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            Ok(None) | Err(_) => return,
        };

        if update {
            let _ = Command::new("sudo").args(["bootctl", "update"]).status();
            println!("Systemd-boot updated.");
        }
        return;
    }

    println!("Systemd-boot is not installed on this system.");
    println!("\nPrerequisites:");
    println!("  - UEFI system (not BIOS/Legacy)");
    println!("  - EFI System Partition (ESP) mounted at /boot or /efi");
    println!("  - Root access");

    // Check if we're on a UEFI system
    if !Path::new("/sys/firmware/efi").exists() {
        println!("\nWarning: This does not appear to be a UEFI system.");
        println!("Systemd-boot only works on UEFI systems.");
        return;
    }

    // Check if ESP is mounted
    let esp_mount = Command::new("findmnt")
        .args(["-n", "-o", "TARGET", "-t", "vfat"])
        .output();

    let esp_path = match esp_mount {
        Ok(output) => {
            let paths = String::from_utf8_lossy(&output.stdout);
            if paths.contains("/boot") {
                "/boot".to_string()
            } else if paths.contains("/efi") {
                "/efi".to_string()
            } else {
                println!("\nWarning: Could not detect EFI System Partition.");
                let path: String = match Input::new()
                    .with_prompt("Enter ESP mount point (e.g., /boot)")
                    .default("/boot".into())
                    .interact_text()
                {
                    Ok(p) => p,
                    Err(_) => return,
                };
                path
            }
        }
        Err(_) => {
            println!("Failed to detect mount points.");
            "/boot".to_string()
        }
    };

    println!("\nDetected ESP at: {}", esp_path);

    let confirm = match Confirm::new()
        .with_prompt("Install systemd-boot to this location?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if !confirm {
        return;
    }

    println!("\nInstalling systemd-boot...");
    let install_status = Command::new("sudo")
        .args(["bootctl", "--esp-path", &esp_path, "install"])
        .status();

    match install_status {
        Ok(s) if s.success() => {
            println!("Systemd-boot installed successfully.");
        }
        Ok(_) => {
            println!("Failed to install systemd-boot. Check permissions and ESP setup.");
            return;
        }
        Err(e) => {
            println!("Error running bootctl: {}", e);
            return;
        }
    }

    // Create default loader.conf
    println!("\nCreating default loader.conf...");
    create_default_loader_conf();

    // Create initial boot entry for current kernel
    println!("\nCreating boot entry for current kernel...");

    // Detect current kernel
    let uname_output = Command::new("uname").arg("-r").output();
    let kernel_version = match uname_output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        Err(_) => "linux".to_string(),
    };

    // Determine kernel package name from version
    let kernel_name = if kernel_version.contains("lts") {
        "linux-lts"
    } else if kernel_version.contains("zen") {
        "linux-zen"
    } else if kernel_version.contains("hardened") {
        "linux-hardened"
    } else {
        "linux"
    };

    let root_uuid = get_root_uuid();

    let entry_content = format!(
        r#"title   Arch Linux
linux   /vmlinuz-{}
initrd  /initramfs-{}.img
options root=UUID={} rw quiet
"#,
        kernel_name, kernel_name, root_uuid
    );

    let entry_path = format!("{}/loader/entries/arch.conf", esp_path);

    println!("Creating boot entry at {}...", entry_path);
    let _ = Command::new("sudo")
        .args(["mkdir", "-p", &format!("{}/loader/entries", esp_path)])
        .status();

    // Write to temp file and move with sudo
    let temp_entry = "/tmp/arch_entry.conf.tmp";
    if std::fs::write(temp_entry, &entry_content).is_ok() {
        let _ = Command::new("sudo")
            .args(["mv", temp_entry, &entry_path])
            .status();
    }

    // Create fallback entry
    let fallback_content = format!(
        r#"title   Arch Linux (fallback)
linux   /vmlinuz-{}
initrd  /initramfs-{}-fallback.img
options root=UUID={} rw
"#,
        kernel_name, kernel_name, root_uuid
    );

    let fallback_path = format!("{}/loader/entries/arch-fallback.conf", esp_path);
    // Write to temp file and move with sudo
    let temp_fallback = "/tmp/arch_fallback.conf.tmp";
    if std::fs::write(temp_fallback, &fallback_content).is_ok() {
        let _ = Command::new("sudo")
            .args(["mv", temp_fallback, &fallback_path])
            .status();
    }

    // Final update
    println!("\nRunning bootctl update...");
    let _ = Command::new("sudo").args(["bootctl", "update"]).status();

    println!("\nSystemd-boot setup complete!");
    println!("Created boot entries:");
    println!("  - arch.conf (default)");
    println!("  - arch-fallback.conf");
    println!("\nYou may want to:");
    println!("  - Edit /boot/loader/loader.conf to adjust timeout and defaults");
    println!("  - Create additional boot entries for other kernels");
    println!("  - Run 'bootctl list' to verify entries");
}

fn update_systemd_boot() {
    println!("🔄 Update Systemd-boot");
    let _ = Command::new("sudo").args(&["bootctl", "update"]).status();
}
