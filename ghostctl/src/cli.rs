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
        .author("Christopher Kelley <ckelley@ghostkellz.sh>")
        .about("Ghost Infrastructure Control - Complete system and homelab management")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .disable_version_flag(true)
        .disable_help_flag(true)
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
                .subcommand(Command::new("monitoring").about("Setup monitoring")),
        )
        .subcommand(
            Command::new("btrfs")
                .about("Btrfs filesystem management")
                .subcommand(
                    Command::new("list")
                        .about("List snapshots")
                )
                .subcommand(
                    Command::new("create")
                        .about("Create snapshot")
                        .arg(Arg::new("name").required(true).help("Snapshot name"))
                        .arg(Arg::new("subvolume").short('s').long("subvolume").default_value("/").help("Source subvolume"))
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete snapshot")
                        .arg(Arg::new("name").required(true).help("Snapshot name"))
                )
                .subcommand(
                    Command::new("restore")
                        .about("Restore snapshot")
                        .arg(Arg::new("name").required(true).help("Snapshot name"))
                        .arg(Arg::new("target").required(true).help("Target path"))
                )
                .subcommand(
                    Command::new("status")
                        .about("Show filesystem status and health")
                )
                .subcommand(
                    Command::new("scrub")
                        .about("Start filesystem scrub")
                        .arg(Arg::new("mountpoint").default_value("/").help("Mountpoint to scrub"))
                )
                .subcommand(
                    Command::new("balance")
                        .about("Start filesystem balance")
                        .arg(Arg::new("mountpoint").default_value("/").help("Mountpoint to balance"))
                )
                .subcommand(
                    Command::new("usage")
                        .about("Show filesystem usage")
                        .arg(Arg::new("mountpoint").default_value("/").help("Mountpoint to analyze"))
                )
                .subcommand(
                    Command::new("quota")
                        .about("Manage quotas")
                        .arg(Arg::new("mountpoint").default_value("/").help("Mountpoint for quota management"))
                )
                .subcommand(
                    Command::new("snapper")
                        .about("Snapper integration")
                        .subcommand(Command::new("setup").about("Setup snapper configurations"))
                        .subcommand(
                            Command::new("edit")
                                .about("Edit snapper config")
                                .arg(Arg::new("config").required(true).help("Config name"))
                        )
                        .subcommand(Command::new("list").about("List snapper configs"))
                )
        )
        .subcommand(
            Command::new("nvidia")
                .about("NVIDIA GPU management")
                .subcommand(Command::new("install").about("Install NVIDIA drivers"))
                .subcommand(Command::new("optimize").about("Optimize GPU settings"))
                .subcommand(Command::new("passthrough").about("Setup GPU passthrough"))
                .subcommand(Command::new("wayland").about("Configure Wayland support")),
        )
        .subcommand(
            Command::new("security")
                .about("Security management")
                .subcommand(Command::new("ssh").about("SSH configuration"))
                .subcommand(Command::new("gpg").about("GPG management"))
                .subcommand(Command::new("credentials").about("Credential management")),
        )
        .subcommand(
            Command::new("backup")
                .about("Backup management")
                .subcommand(Command::new("setup").about("Setup backup system"))
                .subcommand(Command::new("schedule").about("Schedule backups"))
                .subcommand(Command::new("verify").about("Verify backups"))
                .subcommand(Command::new("cleanup").about("Cleanup old backups")),
        )
        .subcommand(
            Command::new("restore")
                .about("System restore")
                .subcommand(Command::new("btrfs").about("Restore from Btrfs"))
                .subcommand(Command::new("system").about("System restore"))
                .subcommand(Command::new("chroot").about("Chroot restore")),
        )
        .subcommand(
            Command::new("shell")
                .about("Shell configuration")
                .subcommand(Command::new("setup").about("Setup shell environment"))
                .subcommand(Command::new("zsh").about("Install and configure ZSH")),
        )
        .subcommand(
            Command::new("systemd")
                .about("Systemd management")
                .subcommand(
                    Command::new("enable")
                        .about("Enable service")
                        .arg(Arg::new("service").required(true).help("Service name")),
                )
                .subcommand(
                    Command::new("disable")
                        .about("Disable service")
                        .arg(Arg::new("service").required(true).help("Service name")),
                )
                .subcommand(
                    Command::new("status")
                        .about("Show service status")
                        .arg(Arg::new("service").required(true).help("Service name")),
                ),
        )
        .subcommand(
            Command::new("arch")
                .about("Arch Linux management")
                .subcommand(Command::new("fix").about("Fix common Arch issues"))
                .subcommand(Command::new("aur").about("AUR package management"))
                .subcommand(Command::new("boot").about("Boot configuration"))
                .subcommand(Command::new("health").about("System health check"))
                .subcommand(Command::new("performance").about("Performance optimization"))
                .subcommand(Command::new("optimize").about("Optimize system performance"))
                .subcommand(Command::new("mirrors").about("Optimize mirror list"))
                .subcommand(Command::new("orphans").about("Clean orphaned packages")),
        )
        .subcommand(
            Command::new("network")
                .about("Network management")
                .subcommand(
                    Command::new("dns")
                        .about("DNS configuration")
                        .arg(Arg::new("domain").required(true).help("Domain name to lookup"))
                )
                .subcommand(Command::new("mesh").about("Mesh networking"))
                .subcommand(
                    Command::new("scan")
                        .about("Network port scanning (using gscan)")
                        .arg(Arg::new("target").required(true).help("Target IP, CIDR, or range (e.g. 192.168.1.1, 192.168.1.0/24)"))
                        .arg(Arg::new("start-port").short('s').help("Start port [default: 1]"))
                        .arg(Arg::new("end-port").short('e').help("End port [default: 1024]"))
                        .arg(Arg::new("banner").long("banner").action(clap::ArgAction::SetTrue).help("Enable banner grabbing"))
                )
                .subcommand(
                    Command::new("netcat")
                        .about("Netcat utilities for file transfer and communication")
                        .subcommand(
                            Command::new("send")
                                .about("Send a file")
                                .arg(Arg::new("file").required(true).help("File to send"))
                                .arg(Arg::new("host").required(true).help("Target host"))
                                .arg(Arg::new("port").required(true).help("Target port"))
                        )
                        .subcommand(
                            Command::new("receive")
                                .about("Receive a file")
                                .arg(Arg::new("file").required(true).help("File to save as"))
                                .arg(Arg::new("port").required(true).help("Port to listen on"))
                        )
                        .subcommand(
                            Command::new("chat")
                                .about("Start or join a chat session")
                                .arg(Arg::new("host").help("Host to connect to (if not provided, starts server)"))
                                .arg(Arg::new("port").required(true).help("Port to use"))
                        )
                        .subcommand(
                            Command::new("check")
                                .about("Check port connectivity")
                                .arg(Arg::new("host").required(true).help("Host to check"))
                                .arg(Arg::new("port").required(true).help("Port to check"))
                        )
                ),
        )
        .subcommand(
            Command::new("cloud")
                .about("Cloud provider management")
                .subcommand(Command::new("aws").about("AWS management"))
                .subcommand(Command::new("azure").about("Azure management"))
                .subcommand(Command::new("gcp").about("Google Cloud management")),
        )
        .subcommand(
            Command::new("tools")
                .about("System tools and utilities")
                .subcommand(Command::new("install").about("Install development tools"))
                .subcommand(Command::new("configure").about("Configure tools"))
                .subcommand(Command::new("update").about("Update tools")),
        )
        .subcommand(Command::new("version").about("Show version information"))
        .subcommand(Command::new("list").about("List available commands"))
}

pub fn handle_cli_args(matches: &ArgMatches) {
    // Handle subcommands
    match matches.subcommand() {
        Some(("version", _)) => {
            println!("ghostctl v{}", env!("CARGO_PKG_VERSION"));
            println!("Ghost Infrastructure Control - Complete system and homelab management");
            println!("Author: Christopher Kelley <ckelley@ghostctl.sh>");
            println!("Repository: https://github.com/ghostkellz/ghostctl");
        }
        Some(("list", _)) => {
            show_command_list();
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
        Some(("monitoring", _)) => setup_homelab_monitoring(),
        None => homelab_management_menu(),
        _ => unreachable!(),
    }
}

fn handle_arch_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("fix", _)) => arch::archfix::fix(),
        Some(("aur", _)) => arch::aur::aur_helper_management(),
        Some(("boot", _)) => arch::boot::boot_management(),
        Some(("health", _)) => arch::health::health_menu(),
        Some(("performance", _)) => arch::perf::tune(),
        Some(("optimize", _)) => arch::archfix::optimize(),
        Some(("mirrors", _)) => arch::archfix::mirrors(),
        Some(("orphans", _)) => arch::archfix::orphans(),
        None => arch::arch_menu(),
        _ => {
            println!("❌ Unknown arch subcommand. Use 'ghostctl arch help' for available options.");
            arch::arch_menu();
        }
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
        Some(("enable", sub_matches)) => {
            if let Some(service) = sub_matches.get_one::<String>("service") {
                systemd::manage_service(&format!("enable {}", service));
            }
        }
        Some(("disable", sub_matches)) => {
            if let Some(service) = sub_matches.get_one::<String>("service") {
                systemd::manage_service(&format!("disable {}", service));
            }
        }
        Some(("status", sub_matches)) => {
            if let Some(service) = sub_matches.get_one::<String>("service") {
                systemd::manage_service(&format!("status {}", service));
            }
        }
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
        Some(("dns", sub_matches)) => {
            if let Some(domain) = sub_matches.get_one::<String>("domain") {
                network::dns::lookup(domain);
            } else {
                println!("❌ Please provide a domain name. Usage: ghostctl network dns <domain>");
            }
        }
        Some(("mesh", _)) => network::mesh::status(),
        Some(("scan", sub_matches)) => {
            if let Some(target) = sub_matches.get_one::<String>("target") {
                let start_port = sub_matches.get_one::<String>("start-port");
                let end_port = sub_matches.get_one::<String>("end-port");
                let banner = sub_matches.get_flag("banner");
                network::scan::gscan_port_scan(target, start_port, end_port, banner);
            } else {
                println!("❌ Please provide a target. Usage: ghostctl network scan <target>");
            }
        }
        Some(("netcat", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("send", send_matches)) => {
                    if let (Some(file), Some(host), Some(port_str)) = (
                        send_matches.get_one::<String>("file"),
                        send_matches.get_one::<String>("host"),
                        send_matches.get_one::<String>("port")
                    ) {
                        if let Ok(port) = port_str.parse::<u16>() {
                            network::netcat::send_file(file, host, port);
                        } else {
                            println!("❌ Invalid port number: {}", port_str);
                        }
                    } else {
                        println!("❌ Usage: ghostctl network netcat send <file> <host> <port>");
                    }
                }
                Some(("receive", receive_matches)) => {
                    if let (Some(file), Some(port_str)) = (
                        receive_matches.get_one::<String>("file"),
                        receive_matches.get_one::<String>("port")
                    ) {
                        if let Ok(port) = port_str.parse::<u16>() {
                            network::netcat::receive_file(file, port);
                        } else {
                            println!("❌ Invalid port number: {}", port_str);
                        }
                    } else {
                        println!("❌ Usage: ghostctl network netcat receive <file> <port>");
                    }
                }
                Some(("chat", chat_matches)) => {
                    if let Some(port_str) = chat_matches.get_one::<String>("port") {
                        if let Ok(port) = port_str.parse::<u16>() {
                            let host = chat_matches.get_one::<String>("host");
                            network::netcat::chat(host.map(|s| s.as_str()), port);
                        } else {
                            println!("❌ Invalid port number: {}", port_str);
                        }
                    } else {
                        println!("❌ Usage: ghostctl network netcat chat [host] <port>");
                    }
                }
                Some(("check", check_matches)) => {
                    if let (Some(host), Some(port_str)) = (
                        check_matches.get_one::<String>("host"),
                        check_matches.get_one::<String>("port")
                    ) {
                        if let Ok(port) = port_str.parse::<u16>() {
                            network::netcat::check_port(host, port);
                        } else {
                            println!("❌ Invalid port number: {}", port_str);
                        }
                    } else {
                        println!("❌ Usage: ghostctl network netcat check <host> <port>");
                    }
                }
                None => {
                    println!("🌐 Netcat utilities available:");
                    println!("  send     - Send a file to a remote host");
                    println!("  receive  - Receive a file from a remote host");
                    println!("  chat     - Start or join a chat session");
                    println!("  check    - Check port connectivity");
                    println!();
                    println!("Use 'ghostctl network netcat <subcommand> --help' for details");
                }
                _ => {
                    println!("❌ Unknown netcat subcommand. Use 'ghostctl network netcat help' for available options.");
                }
            }
        }
        None => network::mesh::status(),
        _ => {
            println!("❌ Unknown network subcommand. Use 'ghostctl network help' for available options.");
            network::mesh::status();
        }
    }
}

fn handle_cloud_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("aws", _)) => cloud::aws::aws_cli_tools(),
        Some(("azure", _)) => cloud::azure::azure_cli_tools(),
        Some(("gcp", _)) => cloud::gcp::gcloud_tools(),
        None => cloud::infrastructure_menu(),
        _ => {
            println!("❌ Unknown cloud subcommand. Use 'ghostctl cloud help' for available options.");
            cloud::infrastructure_menu();
        }
    }
}

fn handle_tools_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("install", _)) => {
            println!("🛠️  Installing development tools...");
            tools::external_tools_menu();
        }
        Some(("configure", _)) => {
            println!("⚙️  Configuring tools...");
            tools::acme_sh_management();
        }
        Some(("update", _)) => {
            println!("🔄 Updating tools...");
            tools::external_tools_menu();
        }
        None => tools::external_tools_menu(),
        _ => {
            println!("❌ Unknown tools subcommand. Use 'ghostctl tools help' for available options.");
            tools::external_tools_menu();
        }
    }
}

fn handle_btrfs_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("list", _)) => {
            btrfs::handle_btrfs_action(crate::BtrfsAction::List);
        }
        Some(("create", sub_matches)) => {
            if let Some(name) = sub_matches.get_one::<String>("name") {
                let default_subvolume = String::from("/");
                let subvolume = sub_matches.get_one::<String>("subvolume").unwrap_or(&default_subvolume);
                btrfs::handle_btrfs_action(crate::BtrfsAction::Create {
                    name: name.clone(),
                    subvolume: subvolume.clone(),
                });
            }
        }
        Some(("delete", sub_matches)) => {
            if let Some(name) = sub_matches.get_one::<String>("name") {
                btrfs::handle_btrfs_action(crate::BtrfsAction::Delete {
                    name: name.clone(),
                });
            }
        }
        Some(("restore", sub_matches)) => {
            if let (Some(name), Some(target)) = (
                sub_matches.get_one::<String>("name"),
                sub_matches.get_one::<String>("target")
            ) {
                btrfs::handle_btrfs_action(crate::BtrfsAction::Restore {
                    name: name.clone(),
                    target: target.clone(),
                });
            }
        }
        Some(("status", _)) => {
            btrfs::handle_btrfs_action(crate::BtrfsAction::Status);
        }
        Some(("scrub", sub_matches)) => {
            let default_mountpoint = String::from("/");
            let mountpoint = sub_matches.get_one::<String>("mountpoint").unwrap_or(&default_mountpoint);
            btrfs::handle_btrfs_action(crate::BtrfsAction::Scrub {
                mountpoint: mountpoint.clone(),
            });
        }
        Some(("balance", sub_matches)) => {
            let default_mountpoint = String::from("/");
            let mountpoint = sub_matches.get_one::<String>("mountpoint").unwrap_or(&default_mountpoint);
            btrfs::handle_btrfs_action(crate::BtrfsAction::Balance {
                mountpoint: mountpoint.clone(),
            });
        }
        Some(("usage", sub_matches)) => {
            let default_mountpoint = String::from("/");
            let mountpoint = sub_matches.get_one::<String>("mountpoint").unwrap_or(&default_mountpoint);
            btrfs::handle_btrfs_action(crate::BtrfsAction::Usage {
                mountpoint: mountpoint.clone(),
            });
        }
        Some(("quota", sub_matches)) => {
            let default_mountpoint = String::from("/");
            let mountpoint = sub_matches.get_one::<String>("mountpoint").unwrap_or(&default_mountpoint);
            btrfs::handle_btrfs_action(crate::BtrfsAction::Quota {
                mountpoint: mountpoint.clone(),
            });
        }
        Some(("snapper", snapper_matches)) => {
            match snapper_matches.subcommand() {
                Some(("setup", _)) => {
                    btrfs::handle_btrfs_action(crate::BtrfsAction::SnapperSetup);
                }
                Some(("edit", sub_matches)) => {
                    if let Some(config) = sub_matches.get_one::<String>("config") {
                        btrfs::handle_btrfs_action(crate::BtrfsAction::SnapperEdit {
                            config: config.clone(),
                        });
                    }
                }
                Some(("list", _)) => {
                    btrfs::handle_btrfs_action(crate::BtrfsAction::SnapperList);
                }
                _ => btrfs::btrfs_menu(),
            }
        }
        None => btrfs::btrfs_menu(),
        _ => {
            println!("❌ Unknown btrfs subcommand. Use 'ghostctl btrfs help' for available options.");
            btrfs::btrfs_menu();
        }
    }
}

fn handle_nvidia_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("fix", _)) => nvidia::fix(),
        Some(("optimize", _)) => nvidia::optimize(),
        Some(("status", _)) => nvidia::status(),
        Some(("info", _)) => nvidia::info(),
        _ => nvidia::fix(),
    }
}

fn handle_security_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("ssh", _)) => security::ssh::ssh_management(),
        Some(("gpg", _)) => security::gpg::gpg_key_management(),
        _ => security::ssh::ssh_management(),
    }
}

fn handle_backup_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("setup", _)) => backup::setup::setup(),
        Some(("schedule", _)) => backup::schedule::setup_schedule(),
        Some(("verify", _)) => backup::verify::verify_backups(),
        Some(("cleanup", _)) => backup::cleanup::cleanup_old_backups(),
        _ => backup::backup_menu(),
    }
}

fn handle_restore_commands(matches: &ArgMatches) {
    match matches.subcommand() {
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
        .args(["version"])
        .status();
    let _ = std::process::Command::new("docker").args(["ps"]).status();
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
        .args(["status", "nginx"])
        .status();
}

fn restart_nginx() {
    println!("🔄 Restarting Nginx...");

    let _ = std::process::Command::new("sudo")
        .args(["systemctl", "restart", "nginx"])
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
        _ => (),
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

fn show_command_list() {
    println!("ghostctl Available Commands");
    println!("====================================");
    println!();
    println!("System Management:");
    println!("  system update    - Update system packages");
    println!("  system status    - Show system status");
    println!("  system arch      - Arch Linux management");
    println!("  system nixos     - NixOS management");
    println!();
    println!("Development:");
    println!("  dev menu         - Development environment menu");
    println!("  dev rust         - Rust development tools");
    println!("  dev zig          - Zig development tools");
    println!("  dev go           - Go development tools");
    println!("  dev python       - Python development tools");
    println!();
    println!("Infrastructure:");
    println!("  docker menu      - Docker management");
    println!("  pve menu         - Proxmox VE management");
    println!("  nginx menu       - Nginx configuration");
    println!("  ssl menu         - SSL certificate management");
    println!();
    println!("Utilities:");
    println!("  scripts menu     - Script management");
    println!("  backup menu      - Backup management");
    println!("  restore menu     - System recovery");
    println!("  security menu    - Security tools");
    println!();
    println!("General:");
    println!("  version          - Show version information");
    println!("  help             - Show help information");
    println!("  menu             - Show interactive menu");
    println!("  list             - Show this command list");
    println!();
    println!("For detailed help on any command, use: ghostctl <command> help");
}
