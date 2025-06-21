use crate::terminal::terminal_menu;
use crate::{
    arch, backup, btrfs, cloud, network, nvidia, proxmox, restore, security, shell, systemd, tools,
};
use clap::{Arg, ArgMatches, Command};
use dialoguer::{Select, theme::ColorfulTheme};

// Command-line interface setup
pub fn build_cli() -> Command {
    Command::new("ghostctl")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Christopher Kelley <ckelley@ghostctl.sh>")
        .about("Ghost Infrastructure Control - Complete system and homelab management")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .disable_help_flag(true)
        .disable_version_flag(true)
        .subcommand(
            Command::new("system")
                .about("System management")
                .subcommand(Command::new("update").about("Update system packages"))
                .subcommand(Command::new("status").about("Show system status"))
                .subcommand(Command::new("arch").about("Arch Linux management"))
                .subcommand(Command::new("nixos").about("NixOS management")),
        )
        .subcommand(
            Command::new("dev")
                .about("Development environment")
                .subcommand(Command::new("menu").about("Development menu"))
                .subcommand(Command::new("rust").about("Rust development"))
                .subcommand(Command::new("zig").about("Zig development"))
                .subcommand(Command::new("go").about("Go development"))
                .subcommand(Command::new("python").about("Python development")),
        )
        .subcommand(
            Command::new("pve")
                .about("Proxmox VE management")
                .subcommand(Command::new("menu").about("PVE management menu"))
                .subcommand(Command::new("status").about("Show PVE status"))
                .subcommand(
                    Command::new("vm")
                        .about("Virtual machine management")
                        .subcommand(Command::new("list").about("List VMs"))
                        .subcommand(Command::new("create").about("Create VM"))
                        .subcommand(
                            Command::new("start")
                                .about("Start VM")
                                .arg(Arg::new("id").required(true).help("VM ID")),
                        )
                        .subcommand(
                            Command::new("stop")
                                .about("Stop VM")
                                .arg(Arg::new("id").required(true).help("VM ID")),
                        ),
                )
                .subcommand(
                    Command::new("ct")
                        .about("Container management")
                        .subcommand(Command::new("list").about("List containers"))
                        .subcommand(Command::new("create").about("Create container"))
                        .subcommand(
                            Command::new("start")
                                .about("Start container")
                                .arg(Arg::new("id").required(true).help("Container ID")),
                        )
                        .subcommand(
                            Command::new("stop")
                                .about("Stop container")
                                .arg(Arg::new("id").required(true).help("Container ID")),
                        ),
                ),
        )
        .subcommand(
            Command::new("docker")
                .about("Docker management")
                .subcommand(Command::new("menu").about("Docker menu"))
                .subcommand(Command::new("install").about("Install Docker"))
                .subcommand(Command::new("status").about("Docker status"))
                .subcommand(Command::new("homelab").about("Homelab stacks")),
        )
        .subcommand(
            Command::new("scripts")
                .about("Script management")
                .subcommand(Command::new("menu").about("Scripts menu"))
                .subcommand(Command::new("local").about("Local scripts"))
                .subcommand(
                    Command::new("run")
                        .about("Run script")
                        .arg(Arg::new("script").required(true).help("Script name")),
                )
                .subcommand(
                    Command::new("list")
                        .about("List scripts")
                        .arg(Arg::new("category").help("Script category")),
                ),
        )
        .subcommand(
            Command::new("ssl")
                .about("SSL certificate management")
                .subcommand(Command::new("menu").about("SSL menu"))
                .subcommand(Command::new("install").about("Install acme.sh"))
                .subcommand(
                    Command::new("issue")
                        .about("Issue certificate")
                        .arg(Arg::new("domain").required(true).help("Domain name")),
                )
                .subcommand(Command::new("renew").about("Renew certificates"))
                .subcommand(Command::new("list").about("List certificates")),
        )
        .subcommand(
            Command::new("nginx")
                .about("Nginx management")
                .subcommand(Command::new("menu").about("Nginx menu"))
                .subcommand(Command::new("status").about("Nginx status"))
                .subcommand(Command::new("restart").about("Restart Nginx"))
                .subcommand(
                    Command::new("ssl-setup")
                        .about("Setup SSL")
                        .arg(Arg::new("domain").required(true).help("Domain name")),
                ),
        )
        .subcommand(
            Command::new("nvim")
                .about("Neovim setup")
                .subcommand(Command::new("menu").about("Neovim menu"))
                .subcommand(Command::new("install").about("Install Neovim"))
                .subcommand(Command::new("lazyvim").about("Install LazyVim")),
        )
        .subcommand(
            Command::new("terminal")
                .about("Terminal configuration")
                .subcommand(Command::new("menu").about("Terminal menu"))
                .subcommand(Command::new("ghostty").about("Install Ghostty"))
                .subcommand(Command::new("starship").about("Install Starship")),
        )
        .subcommand(
            Command::new("ghost")
                .about("Ghost tools management")
                .subcommand(Command::new("menu").about("Ghost tools menu"))
                .subcommand(Command::new("install-all").about("Install all Ghost tools"))
                .subcommand(Command::new("reaper").about("Install Reaper"))
                .subcommand(Command::new("oxygen").about("Install Oxygen"))
                .subcommand(Command::new("zion").about("Install Zion"))
                .subcommand(Command::new("status").about("Check status")),
        )
        .subcommand(
            Command::new("homelab")
                .about("Homelab management")
                .subcommand(Command::new("menu").about("Homelab menu"))
                .subcommand(Command::new("init").about("Initialize homelab"))
                .subcommand(Command::new("media-server").about("Deploy media server"))
                .subcommand(Command::new("monitoring").about("Setup monitoring")),
        )
        .subcommand(
            Command::new("arch")
                .about("Arch Linux system management")
                .subcommand(
                    Command::new("fix").about("Fix common Arch issues (pacman, keyring, mirrors)"),
                )
                .subcommand(
                    Command::new("optimize").about("Optimize system performance (zram/zswap)"),
                )
                .subcommand(Command::new("mirrors").about("Optimize mirror list with reflector"))
                .subcommand(Command::new("orphans").about("Remove orphaned packages"))
                .subcommand(Command::new("pkgfix").about("Clean PKGBUILD/build environment"))
                .subcommand(Command::new("keyring").about("Refresh Arch keyring"))
                .subcommand(Command::new("full").about("Run full system maintenance"))
                .subcommand(Command::new("health").about("System health and maintenance"))
                .subcommand(Command::new("swap").about("Swap and zram management"))
                .subcommand(Command::new("dotfiles").about("Dotfiles management"))
                .subcommand(Command::new("aur").about("AUR helper management"))
                .subcommand(Command::new("boot").about("Boot and kernel management"))
                .subcommand(Command::new("perf").about("Performance tuning"))
                .subcommand(Command::new("menu").about("Interactive Arch menu")),
        )
        .subcommand(
            Command::new("nvidia")
                .about("NVIDIA driver and GPU management")
                .subcommand(Command::new("menu").about("NVIDIA management menu"))
                .subcommand(Command::new("status").about("Check driver status"))
                .subcommand(Command::new("drivers").about("Driver management"))
                .subcommand(Command::new("container").about("Container GPU support"))
                .subcommand(Command::new("passthrough").about("GPU passthrough setup"))
                .subcommand(Command::new("fix").about("Fix NVIDIA issues"))
                .subcommand(Command::new("optimize").about("Optimize performance"))
                .subcommand(Command::new("info").about("Show GPU info")),
        )
        .subcommand(
            Command::new("backup")
                .about("Backup and data protection management")
                .subcommand(Command::new("menu").about("Backup management menu"))
                .subcommand(Command::new("setup").about("Setup backup system"))
                .subcommand(Command::new("schedule").about("Schedule automated backups"))
                .subcommand(Command::new("verify").about("Verify backup integrity"))
                .subcommand(Command::new("cleanup").about("Clean old backups")),
        )
        .subcommand(
            Command::new("restore")
                .about("System recovery and restoration")
                .subcommand(Command::new("menu").about("System recovery menu"))
                .subcommand(Command::new("restic").about("Restore from Restic backup"))
                .subcommand(Command::new("btrfs").about("Rollback Btrfs snapshot"))
                .subcommand(Command::new("chroot").about("Enter recovery chroot")),
        )
        .subcommand(
            Command::new("btrfs")
                .about("Btrfs filesystem management")
                .subcommand(Command::new("menu").about("Btrfs management menu"))
                .subcommand(Command::new("snapshot").about("Create snapshot"))
                .subcommand(Command::new("list").about("List snapshots"))
                .subcommand(Command::new("delete").about("Delete snapshot"))
                .subcommand(Command::new("restore").about("Restore snapshot")),
        )
        .subcommand(
            Command::new("network")
                .about("Network diagnostics and management")
                .subcommand(Command::new("menu").about("Network tools menu"))
                .subcommand(Command::new("status").about("Network status"))
                .subcommand(Command::new("test").about("Network connectivity test"))
                .subcommand(Command::new("config").about("Network configuration")),
        )
        .subcommand(
            Command::new("security")
                .about("Security and key management")
                .subcommand(Command::new("menu").about("Security management menu"))
                .subcommand(Command::new("ssh").about("SSH key management"))
                .subcommand(Command::new("gpg").about("GPG key management"))
                .subcommand(Command::new("audit").about("Security audit")),
        )
        .subcommand(
            Command::new("systemd")
                .about("Systemd service management")
                .subcommand(Command::new("menu").about("Systemd management menu"))
                .subcommand(Command::new("status").about("Service status"))
                .subcommand(Command::new("enable").about("Enable service"))
                .subcommand(Command::new("disable").about("Disable service"))
                .subcommand(Command::new("restart").about("Restart service")),
        )
        .subcommand(
            Command::new("cloud")
                .about("Cloud provider management")
                .subcommand(Command::new("menu").about("Cloud management menu"))
                .subcommand(Command::new("aws").about("AWS tools"))
                .subcommand(Command::new("azure").about("Azure tools"))
                .subcommand(Command::new("gcp").about("Google Cloud tools")),
        )
        .subcommand(Command::new("version").about("Show version information"))
}

pub fn handle_cli_args(matches: &ArgMatches) {
    // Handle subcommands
    match matches.subcommand() {
        Some(("version", _)) => {
            println!("ghostctl v{}", env!("CARGO_PKG_VERSION"));
            println!("Ghost Infrastructure Control - Complete system and homelab management");
            println!("Author: Christopher Kelley <ckelley@ghostctl.sh>");
            println!("Repository: https://github.com/ghostkellz/ghostctl");
            return;
        }
        Some(("system", matches)) => handle_system_commands(matches),
        Some(("arch", matches)) => handle_arch_commands(matches),
        Some(("dev", matches)) => handle_dev_commands(matches),
        Some(("docker", matches)) => handle_docker_commands(matches),
        Some(("nvim", matches)) => handle_nvim_commands(matches),
        Some(("shell", matches)) => handle_shell_commands(matches),
        Some(("systemd", matches)) => handle_systemd_commands(matches),
        Some(("proxmox", matches)) => handle_proxmox_commands(matches),
        Some(("pve", matches)) => handle_pve_commands(matches),
        Some(("network", matches)) => handle_network_commands(matches),
        Some(("cloud", matches)) => handle_cloud_commands(matches),
        Some(("nginx", matches)) => handle_nginx_commands(matches),
        Some(("tools", matches)) => handle_tools_commands(matches),
        Some(("btrfs", matches)) => handle_btrfs_commands(matches),
        Some(("nvidia", matches)) => handle_nvidia_commands(matches),
        Some(("security", matches)) => handle_security_commands(matches),
        Some(("backup", matches)) => handle_backup_commands(matches),
        Some(("restore", matches)) => handle_restore_commands(matches),
        Some(("scripts", matches)) => handle_scripts_commands(matches),
        Some(("ssl", matches)) => handle_ssl_commands(matches),
        Some(("ghost", matches)) => handle_ghost_commands(matches),
        Some(("homelab", matches)) => handle_homelab_commands(matches),
        Some(("terminal", matches)) => handle_terminal_commands(matches),
        Some(("menu", _)) | None => crate::menu::show(),
        Some((cmd, _)) => {
            eprintln!("Unknown command: {}", cmd);
            std::process::exit(1);
        }
    }
}

fn handle_system_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("update", _)) => {
            println!("🔄 Updating system packages...");
            crate::arch::arch_menu();
        }
        Some(("status", _)) => {
            println!("📊 System status:");
            show_system_status();
        }
        Some(("arch", _)) => crate::arch::arch_menu(),
        Some(("nixos", _)) => crate::nix::nixos_menu(),
        None => crate::menu::show(),
        _ => unreachable!(),
    }
}

fn handle_dev_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => crate::dev::development_menu(),
        Some(("rust", _)) => crate::dev::rust_development_menu(),
        Some(("zig", _)) => crate::dev::zig::zig_development_menu(),
        Some(("go", _)) => crate::dev::go::go_development_menu(),
        Some(("python", _)) => crate::dev::python::python_development_menu(),
        None => crate::dev::development_menu(),
        _ => unreachable!(),
    }
}

fn handle_pve_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => pve_management_menu(),
        Some(("status", _)) => show_pve_status(),
        Some(("vm", vm_matches)) => handle_vm_commands(vm_matches),
        Some(("ct", ct_matches)) => handle_ct_commands(ct_matches),
        None => pve_management_menu(),
        _ => unreachable!(),
    }
}

fn handle_vm_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("list", _)) => list_vms(),
        Some(("create", _)) => create_vm_wizard(),
        Some(("start", sub_matches)) => {
            if let Some(id) = sub_matches.get_one::<String>("id") {
                start_vm(id.to_string());
            }
        }
        Some(("stop", sub_matches)) => {
            if let Some(id) = sub_matches.get_one::<String>("id") {
                stop_vm(id.to_string());
            }
        }
        None => vm_management_menu(),
        _ => unreachable!(),
    }
}

fn handle_ct_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("list", _)) => crate::docker::container::list_containers(),
        Some(("create", _)) => create_container_wizard(),
        Some(("start", sub_matches)) => {
            if let Some(id) = sub_matches.get_one::<String>("id") {
                start_container(id.to_string());
            }
        }
        Some(("stop", sub_matches)) => {
            if let Some(id) = sub_matches.get_one::<String>("id") {
                stop_container(id.to_string());
            }
        }
        None => container_management_menu(),
        _ => unreachable!(),
    }
}

fn handle_docker_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => crate::docker::devops::docker_management(),
        Some(("install", _)) => install_docker(),
        Some(("status", _)) => show_docker_status(),
        Some(("homelab", _)) => docker_homelab_menu(),
        None => crate::docker::devops::docker_management(),
        _ => unreachable!(),
    }
}

fn handle_scripts_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => crate::scripts::scripts_menu(),
        Some(("local", _)) => crate::scripts::local_script_management(),
        Some(("run", sub_matches)) => {
            if let Some(script) = sub_matches.get_one::<String>("script") {
                run_script_by_name(script);
            }
        }
        Some(("list", sub_matches)) => {
            let category = sub_matches
                .get_one::<String>("category")
                .map(|s| s.as_str())
                .unwrap_or("all");
            list_scripts_by_category(category);
        }
        None => crate::scripts::scripts_menu(),
        _ => unreachable!(),
    }
}

fn handle_ssl_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => crate::tools::acme_sh_management(),
        Some(("install", _)) => crate::tools::install_acme_sh(),
        Some(("issue", sub_matches)) => {
            if let Some(domain) = sub_matches.get_one::<String>("domain") {
                issue_certificate_cli(domain);
            }
        }
        Some(("renew", _)) => crate::tools::renew_certificates(),
        Some(("list", _)) => crate::tools::list_certificates(),
        None => crate::tools::acme_sh_management(),
        _ => unreachable!(),
    }
}

fn handle_nginx_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => crate::nginx::nginx_menu(),
        Some(("status", _)) => show_nginx_status(),
        Some(("restart", _)) => restart_nginx(),
        Some(("ssl-setup", sub_matches)) => {
            if let Some(domain) = sub_matches.get_one::<String>("domain") {
                setup_nginx_ssl(domain);
            }
        }
        None => crate::nginx::nginx_menu(),
        _ => unreachable!(),
    }
}

fn handle_nvim_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => crate::nvim::nvim_menu(),
        Some(("install", _)) => install_neovim(),
        Some(("lazyvim", _)) => install_lazyvim(),
        None => crate::nvim::nvim_menu(),
        _ => unreachable!(),
    }
}

fn handle_terminal_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => crate::terminal::terminal_menu(),
        Some(("ghostty", _)) => install_ghostty(),
        Some(("starship", _)) => install_starship(),
        None => terminal_menu(),
        _ => unreachable!(),
    }
}

fn handle_ghost_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => crate::dev::ghost_ecosystem_menu(),
        Some(("install-all", _)) => crate::dev::install_all_ghost_tools(),
        Some(("reaper", _)) => crate::dev::install_reaper(),
        Some(("oxygen", _)) => crate::dev::install_oxygen(),
        Some(("zion", _)) => crate::dev::install_zion(),
        Some(("status", _)) => crate::dev::check_tool_status(),
        None => crate::dev::ghost_ecosystem_menu(),
        _ => unreachable!(),
    }
}

fn handle_homelab_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => homelab_management_menu(),
        Some(("init", _)) => initialize_homelab(),
        Some(("media-server", _)) => deploy_media_server(),
        Some(("monitoring", _)) => setup_homelab_monitoring(),
        None => homelab_management_menu(),
        _ => unreachable!(),
    }
}

fn handle_arch_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("fix", _)) => arch::archfix::fix(),
        Some(("optimize", _)) => arch::archfix::optimize(),
        Some(("mirrors", _)) => arch::archfix::mirrors(),
        Some(("orphans", _)) => arch::archfix::orphans(),
        Some(("pkgfix", _)) => arch::archfix::pkgfix(),
        Some(("keyring", _)) => arch::archfix::keyring(),
        Some(("full", _)) => arch::archfix::full(),
        Some(("health", _)) => arch::health::health_menu(),
        Some(("swap", _)) => arch::swap::swap_menu(),
        Some(("dotfiles", _)) => arch::dotfiles::dotfiles_menu(),
        Some(("aur", _)) => arch::aur::aur_helper_management(),
        Some(("boot", _)) => arch::boot::boot_management(),
        Some(("perf", _)) => arch::perf::tune(),
        Some(("menu", _)) => arch::arch_menu(),
        None => arch::arch_menu(),
        _ => unreachable!(),
    }
}

fn handle_shell_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("setup", _)) => shell::setup(),
        Some(("zsh", _)) => shell::zsh::install_zsh(),
        _ => shell::setup(),
    }
}

fn handle_systemd_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => systemd::enable(),
        Some(("status", _)) => systemd::enable(),
        Some(("enable", _)) => systemd::enable(),
        Some(("disable", _)) => systemd::enable(),
        Some(("restart", _)) => systemd::enable(),
        _ => systemd::enable(),
    }
}

fn handle_proxmox_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("enhanced", _)) => proxmox::enhanced::enhanced_proxmox_menu(),
        Some(("helper", _)) => proxmox::helper::cktech_helper_scripts(),
        _ => proxmox::enhanced::enhanced_proxmox_menu(),
    }
}

fn handle_network_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => network::mesh::status(),
        Some(("status", _)) => network::mesh::status(),
        Some(("test", _)) => network::mesh::status(),
        Some(("config", _)) => network::mesh::status(),
        _ => network::mesh::status(),
    }
}

fn handle_cloud_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => cloud::infrastructure_menu(),
        Some(("aws", _)) => cloud::infrastructure_menu(),
        Some(("azure", _)) => cloud::infrastructure_menu(),
        Some(("gcp", _)) => cloud::infrastructure_menu(),
        _ => cloud::infrastructure_menu(),
    }
}

fn handle_tools_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("external", _)) => tools::external_tools_menu(),
        Some(("acme", _)) => tools::install_acme_sh(),
        _ => tools::external_tools_menu(),
    }
}

fn handle_btrfs_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => btrfs::btrfs_menu(),
        Some(("snapshot", _)) => btrfs::btrfs_menu(),
        Some(("list", _)) => btrfs::list_snapshots(),
        Some(("delete", _)) => btrfs::btrfs_menu(),
        Some(("restore", _)) => btrfs::btrfs_menu(),
        _ => btrfs::btrfs_menu(),
    }
}

fn handle_nvidia_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => nvidia::nvidia_menu(),
        Some(("status", _)) => nvidia::status(),
        Some(("drivers", _)) => nvidia::drivers::driver_menu(),
        Some(("container", _)) => nvidia::container::container_menu(),
        Some(("passthrough", _)) => nvidia::passthrough::passthrough_menu(),
        Some(("fix", _)) => nvidia::fix(),
        Some(("optimize", _)) => nvidia::optimize(),
        Some(("info", _)) => nvidia::info(),
        _ => nvidia::nvidia_menu(),
    }
}

fn handle_security_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => security::ssh::ssh_management(),
        Some(("ssh", _)) => security::ssh::ssh_management(),
        Some(("gpg", _)) => security::gpg::gpg_key_management(),
        Some(("audit", _)) => security::ssh::ssh_management(),
        _ => security::ssh::ssh_management(),
    }
}

fn handle_backup_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => backup::backup_menu(),
        Some(("setup", _)) => backup::setup::setup(),
        Some(("schedule", _)) => backup::schedule::setup_schedule(),
        Some(("verify", _)) => backup::verify::verify_backups(),
        Some(("cleanup", _)) => backup::cleanup::cleanup_old_backups(),
        _ => backup::backup_menu(),
    }
}

fn handle_restore_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => restore::restore_menu(),
        Some(("restic", _)) => restore::system::restore_from_restic(),
        Some(("btrfs", _)) => restore::system::rollback_btrfs_snapshot(),
        Some(("chroot", _)) => restore::system::enter_recovery_chroot(),
        _ => restore::restore_menu(),
    }
}

// Additional CLI function implementations

fn show_system_status() {
    println!("📊 System Status");
    println!("================");

    // Show system info
    let _ = std::process::Command::new("uname").arg("-a").status();

    // Show uptime
    let _ = std::process::Command::new("uptime").status();

    // Show memory usage
    let _ = std::process::Command::new("free").arg("-h").status();

    // Show disk usage
    let _ = std::process::Command::new("df").arg("-h").status();
}

fn install_docker() {
    println!("🐳 Installing Docker...");
    crate::docker::install_docker();
}

fn show_docker_status() {
    println!("🐳 Docker Status");
    println!("================");

    let _ = std::process::Command::new("docker")
        .args(&["version"])
        .status();
    let _ = std::process::Command::new("docker").args(&["ps"]).status();
}

fn docker_homelab_menu() {
    println!("🏠 Docker Homelab Stacks");
    crate::docker::homelab_stacks_menu();
}

fn run_script_by_name(script_name: &str) {
    println!("🏃 Running script: {}", script_name);
    crate::scripts::run_specific_script(script_name);
}

fn list_scripts_by_category(category: &str) {
    println!("📋 Scripts in category: {}", category);
    crate::scripts::list_category_scripts(category);
}

fn issue_certificate_cli(domain: &str) {
    println!("🔐 Issuing certificate for: {}", domain);
    crate::tools::issue_certificate_for_domain(domain);
}

fn show_nginx_status() {
    println!("🌐 Nginx Status");
    println!("===============");

    let _ = std::process::Command::new("systemctl")
        .args(&["status", "nginx"])
        .status();
}

fn restart_nginx() {
    println!("🔄 Restarting Nginx...");

    let _ = std::process::Command::new("sudo")
        .args(&["systemctl", "restart", "nginx"])
        .status();

    println!("✅ Nginx restarted");
}

fn setup_nginx_ssl(domain: &str) {
    println!("🔐 Setting up SSL for: {}", domain);
    crate::nginx::setup_ssl_for_domain(domain);
}

fn install_neovim() {
    println!("📝 Installing Neovim...");
    crate::nvim::install_neovim();
}

fn install_lazyvim() {
    println!("⚡ Installing LazyVim...");
    crate::nvim::install_lazyvim();
}

fn install_ghostty() {
    println!("👻 Installing Ghostty...");
    crate::shell::terminals::setup_ghostty();
}

fn install_starship() {
    println!("🚀 Installing Starship...");
    crate::shell::terminals::setup_starship();
}

fn homelab_management_menu() {
    println!("🏠 Homelab Management");
    println!("=====================");

    let options = [
        "🏗️  Initialize Homelab Environment",
        "🖥️  Proxmox VE Management",
        "🐳 Docker Homelab Stacks",
        "📊 Monitoring Setup",
        "🔄 Backup Configuration",
        "🌐 Network Configuration",
        "📱 Media Server Setup",
        "🎮 Game Server Setup",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Homelab Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => initialize_homelab(),
        1 => homelab_management_menu(),
        2 => docker_homelab_menu(),
        3 => setup_homelab_monitoring(),
        4 => setup_homelab_backup(),
        5 => setup_homelab_network(),
        6 => deploy_media_server(),
        7 => deploy_game_server(),
        _ => return,
    }
}

fn initialize_homelab() {
    println!("🏗️  Initializing Homelab Environment");
    println!("=====================================");

    println!("🔧 Setting up essential homelab tools...");

    // Create homelab directories
    let homelab_dirs = [
        "/data/homelab/config",
        "/data/homelab/data",
        "/data/homelab/backups",
        "/data/homelab/scripts",
        "/data/homelab/docker-compose",
    ];

    for dir in &homelab_dirs {
        let _ = std::fs::create_dir_all(dir);
        println!("📁 Created: {}", dir);
    }

    println!("✅ Homelab environment initialized");
}

fn setup_homelab_monitoring() {
    println!("📊 Setting up Homelab Monitoring");
    println!("=================================");

    println!("💡 Deploying monitoring stack:");
    println!("  • Prometheus (metrics collection)");
    println!("  • Grafana (visualization)");
    println!("  • Node Exporter (system metrics)");
    println!("  • Alertmanager (notifications)");

    // This would deploy the monitoring stack
    println!("🚀 Monitoring stack deployment - Coming soon!");
}

fn setup_homelab_backup() {
    println!("🔄 Setting up Homelab Backup");
    println!("=============================");

    println!("💾 Configuring backup solutions:");
    println!("  • Restic (file backups)");
    println!("  • Proxmox Backup Server");
    println!("  • Database backups");
    println!("  • Configuration backups");

    println!("💾 Backup configuration - Coming soon!");
}

fn setup_homelab_network() {
    println!("🌐 Setting up Homelab Network");
    println!("==============================");

    println!("🔧 Network configuration options:");
    println!("  • VLAN setup");
    println!("  • VPN configuration");
    println!("  • DNS server");
    println!("  • Reverse proxy");

    println!("🌐 Network setup - Coming soon!");
}

fn deploy_media_server() {
    println!("📱 Deploying Media Server");
    println!("=========================");

    println!("🎬 Media server options:");
    println!("  • Plex Media Server");
    println!("  • Jellyfin");
    println!("  • Emby");
    println!("  • *arr stack (Sonarr, Radarr, etc.)");

    println!("🎬 Media server deployment - Coming soon!");
}

fn deploy_game_server() {
    println!("🎮 Deploying Game Server");
    println!("========================");

    println!("🕹️  Game server options:");
    println!("  • Minecraft server");
    println!("  • Valheim server");
    println!("  • CS:GO server");
    println!("  • Terraria server");

    println!("🎮 Game server deployment - Coming soon!");
}

// PVE CLI functions placeholder
// TODO: Add PVE module back when implemented
fn pve_management_menu() {
    println!("PVE Management Menu - Coming Soon!");
}

fn show_pve_status() {
    println!("PVE Status - Coming Soon!");
}

fn list_vms() {
    println!("VM List - Coming Soon!");
}

fn create_vm_wizard() {
    println!("VM Creation Wizard - Coming Soon!");
}

fn start_vm(_id: String) {
    println!("Starting VM - Coming Soon!");
}

fn stop_vm(_id: String) {
    println!("Stopping VM - Coming Soon!");
}

fn vm_management_menu() {
    println!("VM Management Menu - Coming Soon!");
}

fn create_container_wizard() {
    println!("Container Creation Wizard - Coming Soon!");
}

fn start_container(_id: String) {
    println!("Starting Container - Coming Soon!");
}

fn container_management_menu() {
    println!("Container Management Menu - Coming Soon!");
}

fn stop_container(_id: String) {
    println!("Stopping Container - Coming Soon!");
}

fn ssl_management_menu() {
    println!("SSL Management Menu - Coming Soon!");
}
