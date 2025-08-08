use crate::{
    arch, backup, btrfs, cloud, dev, docker, network, nvidia, nvim, proxmox, restore, scripts,
    security, shell, storage, systemd, terminal,
};
use dialoguer::{Select, theme::ColorfulTheme};

pub fn show() {
    loop {
        let opts = [
            "🔧 Fix Arch Issues (Pacman, PKGBUILD, Optimize)",
            "🛠️  Stage Dev Project (Rust/Go/Zig)",
            "📸 Manage Btrfs Snapshots",
            "🎮 NVIDIA Management (Drivers, Container, Passthrough)",
            "🚀 Neovim Configurator",
            "🐚 Shell Setup (ZSH, Oh My Zsh, Powerlevel10k, tmux)",
            "💻 Terminal Setup (Ghostty, WezTerm)",
            "🔧 Ghost Tools (Install/Uninstall)",
            "💾 Backup Management",
            "🚨 System Recovery & Restore",
            "☁️  Storage Management (S3, Local, Network)",
            "🐳 DevOps & Container Tools",
            "🏗️  Infrastructure as Code",
            "🌐 Nginx Configuration",
            "❄️  NixOS Management",
            "🖥️  Proxmox VE Helper Scripts",
            "🔧 Systemd Management",
            "📋 Plugin & Script Management",
            "🌐 Mesh (Tailscale/Headscale)",
            "🔐 Security & Key Management",
            "📊 Diagnostics/Status",
            "🚪 Exit",
        ];

        println!("ghostctl :: Menu");
        println!("================");

        for (i, opt) in opts.iter().enumerate() {
            println!("{}. {}", i + 1, opt);
        }

        println!();
        print!("Enter your choice (1-{}): ", opts.len());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Ok(choice) = input.trim().parse::<usize>() {
                    if choice >= 1 && choice <= opts.len() {
                        let choice = choice - 1;
                        match choice {
                            0 => arch::arch_menu(),
                            1 => dev::development_menu(),
                            2 => btrfs::btrfs_menu(),
                            3 => nvidia::nvidia_menu(),
                            4 => nvim::nvim_menu(),
                            5 => shell::setup(),
                            6 => terminal::terminal_menu(),
                            7 => dev::gtools::ghost_ecosystem_menu(),
                            8 => backup::backup_menu(),
                            9 => restore::restore_menu(),
                            10 => storage::storage_menu(),
                            11 => docker::devops::docker_management(),
                            12 => cloud::infrastructure_menu(),
                            13 => crate::nginx::nginx_menu(),
                            14 => crate::nix::nixos_menu(),
                            15 => proxmox::proxmox_menu(),
                            16 => systemd_management(),
                            17 => scripts::scripts_menu(),
                            18 => network_mesh_menu(),
                            19 => security_key_management(),
                            20 => show_diagnostics(),
                            _ => {
                                println!("👋 Goodbye!");
                                break;
                            }
                        }
                    } else {
                        println!(
                            "Invalid choice. Please enter a number between 1 and {}.",
                            opts.len()
                        );
                    }
                } else {
                    println!("Invalid input. Please enter a number.");
                }
            }
            Err(e) => {
                println!("Error reading input: {}", e);
                break;
            }
        }
    }
}

// NVIDIA menu moved to nvidia::nvidia_menu() for comprehensive functionality

fn systemd_management() {
    let options = [
        "📊 Service Status",
        "🔧 Enable Service",
        "🛑 Disable Service",
        "📝 Create Service",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Systemd Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => systemd::status(),
        1 => systemd::enable(),
        2 => systemd::disable(),
        3 => systemd::create(),
        _ => return,
    }
}

fn network_mesh_menu() {
    let options = [
        "🔗 Mesh Up",
        "📡 Advertise Subnet",
        "📊 Status",
        "🔽 Mesh Down",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Mesh Networking")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => network::mesh::up(),
        1 => {
            use dialoguer::Input;
            let subnet: String = Input::new()
                .with_prompt("Subnet to advertise")
                .interact_text()
                .unwrap();
            network::mesh::advertise(&subnet);
        }
        2 => network::mesh::status(),
        3 => network::mesh::down(),
        _ => return,
    }
}

fn security_key_management() {
    let options = [
        "🔑 SSH Key Management",
        "🔐 GPG Key Management",
        "🛡️  Security Audit",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Security & Key Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => security::ssh::ssh_management(),
        1 => security::gpg::gpg_key_management(),
        2 => combined_security_audit(),
        _ => return,
    }
}

fn show_diagnostics() {
    println!("📊 System Diagnostics");
    println!("====================");

    // Basic system info
    println!("🖥️  System Information:");
    let _ = std::process::Command::new("uname").arg("-a").status();

    println!("\n💾 Memory Usage:");
    let _ = std::process::Command::new("free").arg("-h").status();

    println!("\n💿 Disk Usage:");
    let _ = std::process::Command::new("df").arg("-h").status();

    println!("\n🔄 Load Average:");
    let _ = std::process::Command::new("cat")
        .arg("/proc/loadavg")
        .status();

    println!("\n📊 GhostCTL Module Status:");
    check_module_availability();
}

fn check_module_availability() {
    let modules = [
        ("Docker", "docker"),
        ("Nginx", "nginx"),
        ("Git", "git"),
        ("Neovim", "nvim"),
        ("Restic", "restic"),
        ("Btrfs", "btrfs"),
        ("Systemd", "systemctl"),
    ];

    for (name, cmd) in &modules {
        let status = std::process::Command::new("which")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if status {
            println!("  ✅ {} available", name);
        } else {
            println!("  ❌ {} not found", name);
        }
    }
}

fn combined_security_audit() {
    println!("🛡️  Comprehensive Security Audit");
    println!("=================================");

    println!("🔍 1. SSH Security Audit");
    crate::network::security_audit();

    println!("\n🔍 2. GPG Security Check");
    let _ = crate::security::gpg::list_gpg_keys();

    println!("\n🔍 3. System Security Overview");
    // Check for common security tools
    let security_tools = [
        ("fail2ban", "Intrusion prevention"),
        ("ufw", "Uncomplicated Firewall"),
        ("iptables", "Netfilter firewall"),
        ("rkhunter", "Rootkit Hunter"),
        ("lynis", "Security auditing tool"),
    ];

    for (tool, description) in &security_tools {
        let available = std::process::Command::new("which")
            .arg(tool)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if available {
            println!("  ✅ {} - {}", tool, description);
        } else {
            println!("  ❌ {} - {} (not installed)", tool, description);
        }
    }

    println!("\n💡 Security Recommendations:");
    println!("  🔑 Use SSH keys instead of passwords");
    println!("  🔒 Enable automatic security updates");
    println!("  🛡️  Configure firewall rules");
    println!("  📊 Monitor system logs regularly");
    println!("  🔐 Keep GPG keys backed up securely");
}
