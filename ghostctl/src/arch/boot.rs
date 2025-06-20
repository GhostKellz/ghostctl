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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Boot Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Kernel Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    // List installed kernel packages
    let _ = Command::new("pacman").args(&["-Q"]).status();

    println!("\n🔍 Filtering kernel packages...");
    let _ = Command::new("bash")
        .arg("-c")
        .arg("pacman -Q | grep -E '^linux|^linux-'")
        .status();

    // Check /boot for kernel files
    println!("\n📁 Kernel files in /boot:");
    if Path::new("/boot").exists() {
        let _ = Command::new("ls")
            .args(&["-la", "/boot/vmlinuz-*"])
            .status();
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select kernel to install")
        .items(&all_options)
        .default(0)
        .interact()
        .unwrap();

    if choice < kernels.len() {
        let (kernel_name, _) = kernels[choice];
        install_specific_kernel(kernel_name);
    }
}

fn install_specific_kernel(kernel_name: &str) {
    println!("🔽 Installing kernel: {}", kernel_name);

    let confirm = Confirm::new()
        .with_prompt(&format!(
            "Install {} kernel? This will also install headers.",
            kernel_name
        ))
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        // Install kernel and headers
        if kernel_name.contains("tkg")
            || kernel_name.contains("cachy")
            || kernel_name.contains("xanmod")
            || kernel_name.contains("clear")
        {
            println!("📦 Installing AUR kernel (requires yay or similar)...");
            let status = Command::new("yay")
                .args(&[
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
                .args(&[
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Systemd-boot Configuration")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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
            .args(&["-la", "/boot/loader/entries/"])
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
        .arg("mkdir")
        .args(&["-p", "/boot/loader"])
        .status();

    let _ = Command::new("sudo")
        .arg("bash")
        .arg("-c")
        .arg(&format!(
            "echo '{}' > /boot/loader/loader.conf",
            default_config
        ))
        .status();

    println!("✅ Default loader.conf created");
}

fn create_boot_entry() {
    println!("🆕 Create Boot Entry");
    println!("===================");

    let entry_name: String = Input::new()
        .with_prompt("Entry name (e.g., 'arch-zen')")
        .interact_text()
        .unwrap();

    let kernel_file: String = Input::new()
        .with_prompt("Kernel file (e.g., 'vmlinuz-linux-zen')")
        .default("vmlinuz-linux".into())
        .interact_text()
        .unwrap();

    let initrd_file: String = Input::new()
        .with_prompt("Initrd file (e.g., 'initramfs-linux-zen.img')")
        .default("initramfs-linux.img".into())
        .interact_text()
        .unwrap();

    // Get root UUID
    println!("🔍 Detecting root partition...");
    let root_uuid = get_root_uuid();

    let kernel_params: String = Input::new()
        .with_prompt("Additional kernel parameters")
        .default("quiet rw".into())
        .interact_text()
        .unwrap();

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

    let confirm = Confirm::new()
        .with_prompt("Create this boot entry?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        let _ = Command::new("sudo")
            .arg("mkdir")
            .args(&["-p", "/boot/loader/entries"])
            .status();

        let _ = Command::new("sudo")
            .arg("bash")
            .arg("-c")
            .arg(&format!("echo '{}' > '{}'", entry_content, entry_filename))
            .status();

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
                .unwrap()
        }
    }
}

fn list_boot_entries() {
    println!("📋 Boot Entries");
    println!("===============");

    let _ = Command::new("bootctl").arg("list").status();
}

fn regenerate_boot_entries() {
    println!("🔄 Regenerating Boot Entries");
    println!("============================");

    // Check if mkinitcpio presets exist
    if Path::new("/etc/mkinitcpio.d").exists() {
        println!("🔍 Found mkinitcpio presets:");
        let _ = Command::new("ls").args(&["/etc/mkinitcpio.d/"]).status();

        let regenerate = Confirm::new()
            .with_prompt("Regenerate initramfs for all kernels?")
            .default(true)
            .interact()
            .unwrap();

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
    println!("🔽 Install New Kernel - TODO: Implement");
}

fn remove_kernel() {
    println!("🗑️  Remove Kernel - TODO: Implement");
}

fn update_kernels() {
    println!("🔄 Update All Kernels");
    let _ = Command::new("sudo").args(&["pacman", "-Syu"]).status();
}

fn kernel_config() {
    println!("⚙️  Kernel Configuration - TODO: Implement");
}

fn manage_boot_entries() {
    println!("📁 Manage Boot Entries - TODO: Implement");
}

fn setup_systemd_boot() {
    println!("🔧 Setup Systemd-boot - TODO: Implement");
}

fn update_systemd_boot() {
    println!("🔄 Update Systemd-boot");
    let _ = Command::new("sudo").args(&["bootctl", "update"]).status();
}
