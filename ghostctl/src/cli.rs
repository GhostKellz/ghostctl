use crate::terminal::terminal_menu;
use crate::utils::{set_dry_run_mode, set_headless_mode, set_plain_mode};
use crate::{
    arch, backup, bluetooth, btrfs, cloud, network, nvidia, proxmox, restore, security, shell,
    sysctl, systemd, tools, wifi,
};
use clap::{Arg, ArgAction, ArgMatches, Command};
use dialoguer::{Select, theme::ColorfulTheme};

// Command-line interface setup
pub fn build_cli() -> Command {
    Command::new("ghostctl")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Christopher Kelley <ckelley@ghostkellz.sh>")
        .about("👻 GhostCTL - System management and automation toolkit")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .disable_version_flag(false)
        .disable_help_flag(false)
        // Global flags for automation
        .arg(
            Arg::new("headless")
                .long("headless")
                .short('H')
                .help("Run in headless mode (no interactive prompts)")
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .arg(
            Arg::new("non-interactive")
                .long("non-interactive")
                .short('n')
                .help("Alias for --headless")
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .short('D')
                .help("Show what would be done without making changes")
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .arg(
            Arg::new("yes")
                .long("yes")
                .short('y')
                .help("Automatically answer yes to prompts (use with caution)")
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .arg(
            Arg::new("plain")
                .long("plain")
                .short('p')
                .help("Plain output mode (no emojis or colors, for scripting/accessibility)")
                .action(ArgAction::SetTrue)
                .global(true),
        )
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
                .subcommand(Command::new("list").about("List snapshots"))
                .subcommand(
                    Command::new("create")
                        .about("Create snapshot")
                        .arg(Arg::new("name").required(true).help("Snapshot name"))
                        .arg(
                            Arg::new("subvolume")
                                .short('s')
                                .long("subvolume")
                                .default_value("/")
                                .help("Source subvolume"),
                        ),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete snapshot")
                        .arg(Arg::new("name").required(true).help("Snapshot name")),
                )
                .subcommand(
                    Command::new("restore")
                        .about("Restore snapshot")
                        .arg(Arg::new("name").required(true).help("Snapshot name"))
                        .arg(Arg::new("target").required(true).help("Target path")),
                )
                .subcommand(Command::new("status").about("Show filesystem status and health"))
                .subcommand(
                    Command::new("scrub").about("Start filesystem scrub").arg(
                        Arg::new("mountpoint")
                            .default_value("/")
                            .help("Mountpoint to scrub"),
                    ),
                )
                .subcommand(
                    Command::new("balance")
                        .about("Start filesystem balance")
                        .arg(
                            Arg::new("mountpoint")
                                .default_value("/")
                                .help("Mountpoint to balance"),
                        ),
                )
                .subcommand(
                    Command::new("usage").about("Show filesystem usage").arg(
                        Arg::new("mountpoint")
                            .default_value("/")
                            .help("Mountpoint to analyze"),
                    ),
                )
                .subcommand(
                    Command::new("quota").about("Manage quotas").arg(
                        Arg::new("mountpoint")
                            .default_value("/")
                            .help("Mountpoint for quota management"),
                    ),
                )
                .subcommand(
                    Command::new("snapper")
                        .about("Snapper integration")
                        .subcommand(Command::new("setup").about("Setup snapper configurations"))
                        .subcommand(
                            Command::new("edit")
                                .about("Edit snapper config")
                                .arg(Arg::new("config").required(true).help("Config name")),
                        )
                        .subcommand(Command::new("list").about("List snapper configs"))
                        .subcommand(Command::new("cleanup").about("Cleanup old snapshots")),
                )
                .subcommand(
                    Command::new("cleanup")
                        .about("Emergency cleanup snapshots")
                        .arg(
                            Arg::new("emergency")
                                .long("emergency")
                                .action(clap::ArgAction::SetTrue)
                                .help("Remove ALL snapshots (DANGEROUS)"),
                        )
                        .arg(
                            Arg::new("days")
                                .long("days")
                                .value_name("DAYS")
                                .help("Remove snapshots older than X days"),
                        )
                        .arg(
                            Arg::new("range")
                                .long("range")
                                .value_name("RANGE")
                                .help("Remove snapshot range (e.g., 1-100)"),
                        ),
                ),
        )
        .subcommand(
            Command::new("nvidia")
                .about("NVIDIA GPU management")
                .subcommand(Command::new("install").about("Install NVIDIA drivers"))
                .subcommand(Command::new("optimize").about("Optimize GPU settings"))
                .subcommand(Command::new("passthrough").about("Setup GPU passthrough"))
                .subcommand(Command::new("wayland").about("Configure Wayland support"))
                .subcommand(
                    Command::new("build-source")
                        .about("Build NVIDIA kernel modules from source")
                        .arg(
                            Arg::new("all-kernels")
                                .long("all-kernels")
                                .help("Build for all installed kernels (default: current kernel only)")
                                .action(ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("dkms")
                                .long("dkms")
                                .help("Use DKMS-managed build (default: true)")
                                .action(ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("no-dkms")
                                .long("no-dkms")
                                .help("Use direct make install instead of DKMS")
                                .action(ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("auto-clean")
                                .long("auto-clean")
                                .help("Automatically clean old DKMS entries without prompting")
                                .action(ArgAction::SetTrue),
                        ),
                )
                .subcommand(Command::new("dkms-status").about("Show DKMS module status"))
                .subcommand(Command::new("dkms-cleanup").about("Clean old DKMS entries")),
        )
        .subcommand(
            Command::new("security")
                .about("Security management")
                .subcommand(Command::new("menu").about("Security management menu"))
                .subcommand(Command::new("ssh").about("SSH configuration"))
                .subcommand(Command::new("gpg").about("GPG management"))
                .subcommand(Command::new("credentials").about("Credential management")),
        )
        .subcommand(
            Command::new("bluetooth")
                .about("Bluetooth device management")
                .visible_alias("bt")
                .subcommand(Command::new("menu").about("Interactive Bluetooth menu"))
                .subcommand(Command::new("tui").about("Launch Bluetooth TUI"))
                .subcommand(Command::new("list").about("List adapters and devices"))
                .subcommand(Command::new("scan").about("Scan for nearby devices"))
                .subcommand(Command::new("power").about("Toggle adapter power")),
        )
        .subcommand(
            Command::new("wifi")
                .about("WiFi network management (requires iwd)")
                .visible_alias("wlan")
                .subcommand(Command::new("menu").about("Interactive WiFi menu"))
                .subcommand(Command::new("tui").about("Launch WiFi TUI"))
                .subcommand(Command::new("status").about("Show WiFi status"))
                .subcommand(Command::new("list").about("List known networks"))
                .subcommand(Command::new("scan").about("Scan for networks"))
                .subcommand(Command::new("connect").about("Connect to a network"))
                .subcommand(Command::new("disconnect").about("Disconnect from network"))
                .subcommand(Command::new("power").about("Toggle WiFi power")),
        )
        .subcommand(
            Command::new("sysctl")
                .about("Kernel parameter browser (systeroid-style)")
                .visible_alias("kernel")
                .subcommand(Command::new("menu").about("Interactive sysctl menu"))
                .subcommand(Command::new("tui").about("Launch kernel parameter TUI"))
                .subcommand(Command::new("list").about("List all parameters"))
                .subcommand(Command::new("search").about("Search parameters"))
                .subcommand(Command::new("get").about("Get a parameter value"))
                .subcommand(Command::new("set").about("Set a parameter value"))
                .subcommand(Command::new("export").about("Export configuration")),
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
                .subcommand(
                    Command::new("clean").about("Clean specific target").arg(
                        Arg::new("target")
                            .required(true)
                            .help("Target to clean (orphans, mirrors, pkgfix, gpg, locks, all)"),
                    ),
                )
                .subcommand(
                    Command::new("bouncer")
                        .about("Fix and bounce back from issues (auto-detects if no target)")
                        .arg(
                            Arg::new("target")
                                .required(false)
                                .help("Optional target to fix (pacman, keyring, mirrors, all). Omit for auto-detection."),
                        ),
                )
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
                .subcommand(Command::new("menu").about("Network management menu"))
                .subcommand(
                    Command::new("dns").about("DNS configuration").arg(
                        Arg::new("domain")
                            .required(true)
                            .help("Domain name to lookup"),
                    ),
                )
                .subcommand(Command::new("mesh").about("Mesh networking"))
                .subcommand(
                    Command::new("scan")
                        .about("Network port scanning with native Rust implementation")
                        .arg(
                            Arg::new("target").required(true).help(
                                "Target IP, CIDR, or range (e.g. 192.168.1.1, 192.168.1.0/24)",
                            ),
                        )
                        .arg(
                            Arg::new("start-port")
                                .short('s')
                                .help("Start port [default: 1]"),
                        )
                        .arg(
                            Arg::new("end-port")
                                .short('e')
                                .help("End port [default: 1024]"),
                        )
                        .arg(
                            Arg::new("banner")
                                .long("banner")
                                .action(clap::ArgAction::SetTrue)
                                .help("Enable banner grabbing"),
                        ),
                )
                .subcommand(
                    Command::new("netcat")
                        .about("Netcat utilities for file transfer and communication")
                        .subcommand(
                            Command::new("send")
                                .about("Send a file")
                                .arg(Arg::new("file").required(true).help("File to send"))
                                .arg(Arg::new("host").required(true).help("Target host"))
                                .arg(Arg::new("port").required(true).help("Target port")),
                        )
                        .subcommand(
                            Command::new("receive")
                                .about("Receive a file")
                                .arg(Arg::new("file").required(true).help("File to save as"))
                                .arg(Arg::new("port").required(true).help("Port to listen on")),
                        )
                        .subcommand(
                            Command::new("chat")
                                .about("Start or join a chat session")
                                .arg(
                                    Arg::new("host").help(
                                        "Host to connect to (if not provided, starts server)",
                                    ),
                                )
                                .arg(Arg::new("port").required(true).help("Port to use")),
                        )
                        .subcommand(
                            Command::new("check")
                                .about("Check port connectivity")
                                .arg(Arg::new("host").required(true).help("Host to check"))
                                .arg(Arg::new("port").required(true).help("Port to check")),
                        ),
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
        // Short aliases (hidden from main help)
        .subcommand(
            Command::new("net")
                .about("Network management (short alias)")
                .subcommand(Command::new("menu").about("Network management menu"))
                .subcommand(
                    Command::new("dns").about("DNS configuration").arg(
                        Arg::new("domain")
                            .required(true)
                            .help("Domain name to lookup"),
                    ),
                )
                .subcommand(Command::new("mesh").about("Mesh networking"))
                .subcommand(
                    Command::new("scan")
                        .about("Network port scanning")
                        .arg(
                            Arg::new("target")
                                .required(true)
                                .help("Target IP, CIDR, or range"),
                        )
                        .arg(Arg::new("start-port").short('s').help("Start port"))
                        .arg(Arg::new("end-port").short('e').help("End port"))
                        .arg(
                            Arg::new("banner")
                                .long("banner")
                                .action(clap::ArgAction::SetTrue)
                                .help("Enable banner grabbing"),
                        ),
                )
                .subcommand(
                    Command::new("netcat")
                        .about("Netcat utilities")
                        .subcommand(
                            Command::new("send")
                                .about("Send a file")
                                .arg(Arg::new("file").required(true).help("File to send"))
                                .arg(Arg::new("host").required(true).help("Target host"))
                                .arg(Arg::new("port").required(true).help("Target port")),
                        )
                        .subcommand(
                            Command::new("receive")
                                .about("Receive a file")
                                .arg(Arg::new("file").required(true).help("File to save as"))
                                .arg(Arg::new("port").required(true).help("Port to listen on")),
                        )
                        .subcommand(
                            Command::new("chat")
                                .about("Start or join a chat session")
                                .arg(Arg::new("host").help("Host to connect to"))
                                .arg(Arg::new("port").required(true).help("Port to use")),
                        )
                        .subcommand(
                            Command::new("check")
                                .about("Check port connectivity")
                                .arg(Arg::new("host").required(true).help("Host to check"))
                                .arg(Arg::new("port").required(true).help("Port to check")),
                        ),
                )
                .hide(true),
        )
        .subcommand(
            Command::new("sec")
                .about("Security management (short alias)")
                .subcommand(Command::new("menu").about("Security management menu"))
                .subcommand(Command::new("ssh").about("SSH configuration"))
                .subcommand(Command::new("gpg").about("GPG management"))
                .subcommand(Command::new("credentials").about("Credential management"))
                .hide(true),
        )
        .subcommand(
            Command::new("ssh")
                .about("SSH configuration and management")
                .subcommand(Command::new("menu").about("Interactive SSH management menu"))
                .subcommand(Command::new("generate").about("Generate new SSH key pair"))
                .subcommand(Command::new("list").about("List SSH keys"))
                .subcommand(
                    Command::new("copy-id")
                        .about("Copy SSH key to remote host")
                        .arg(Arg::new("target").required(true).help("user@hostname")),
                )
                .subcommand(Command::new("config").about("SSH configuration management"))
                .hide(true),
        )
        .subcommand(Command::new("gpg").about("GPG key management").hide(true))
        .subcommand(
            Command::new("dns")
                .about("DNS lookup and management")
                .arg(Arg::new("domain").help("Domain name to lookup"))
                .arg(
                    Arg::new("type")
                        .long("type")
                        .short('t')
                        .help("DNS record type (A, AAAA, MX, NS, TXT, etc.)"),
                )
                .arg(
                    Arg::new("reverse")
                        .long("reverse")
                        .short('r')
                        .action(clap::ArgAction::SetTrue)
                        .help("Perform reverse DNS lookup"),
                )
                .arg(
                    Arg::new("server")
                        .long("server")
                        .short('s')
                        .help("DNS server to use"),
                )
                .hide(true),
        )
        .subcommand(
            Command::new("nc")
                .about("Netcat utilities")
                .subcommand(
                    Command::new("send")
                        .about("Send file to host")
                        .arg(Arg::new("file").required(true).help("File to send"))
                        .arg(Arg::new("host").required(true).help("Target host"))
                        .arg(Arg::new("port").required(true).help("Target port")),
                )
                .subcommand(
                    Command::new("receive")
                        .about("Receive file on port")
                        .arg(Arg::new("file").required(true).help("Output file"))
                        .arg(Arg::new("port").required(true).help("Listen port")),
                )
                .subcommand(
                    Command::new("chat")
                        .about("Start chat session")
                        .arg(Arg::new("host").help("Host to connect to (omit for server mode)"))
                        .arg(Arg::new("port").required(true).help("Port")),
                )
                .subcommand(
                    Command::new("check")
                        .about("Check port connectivity")
                        .arg(Arg::new("host").required(true).help("Target host"))
                        .arg(Arg::new("port").required(true).help("Target port")),
                )
                .hide(true),
        )
        .subcommand(
            Command::new("scan")
                .about("Network scanner with beautiful TUI")
                .arg(
                    Arg::new("target")
                        .required(true)
                        .help("Target IP/hostname/CIDR range"),
                )
                .arg(
                    Arg::new("ports")
                        .short('p')
                        .long("ports")
                        .help("Port specification (e.g., 80,443,8080 or 1-1000)")
                        .default_value("1-1000"),
                )
                .arg(
                    Arg::new("threads")
                        .short('t')
                        .long("threads")
                        .help("Number of concurrent threads")
                        .default_value("100"),
                )
                .arg(
                    Arg::new("full")
                        .long("full")
                        .action(clap::ArgAction::SetTrue)
                        .help("Scan all 65535 ports"),
                )
                .arg(
                    Arg::new("service")
                        .long("service")
                        .action(clap::ArgAction::SetTrue)
                        .help("Enable service detection"),
                )
                .arg(
                    Arg::new("json")
                        .long("json")
                        .action(clap::ArgAction::SetTrue)
                        .help("Output results in JSON format (no TUI)"),
                )
                .arg(
                    Arg::new("quiet")
                        .short('q')
                        .long("quiet")
                        .action(clap::ArgAction::SetTrue)
                        .help("Minimal output"),
                ),
        )
        .subcommand(Command::new("version").about("Show version information"))
        .subcommand(Command::new("list").about("List available commands"))
}

pub fn handle_cli_args(matches: &ArgMatches) {
    // Process global flags first - set plain mode FIRST so other messages respect it
    if matches.get_flag("plain") {
        set_plain_mode(true);
    }

    if matches.get_flag("headless") || matches.get_flag("non-interactive") {
        set_headless_mode(true);
        println!("{} Running in headless mode", crate::tui::icons::robot());
    }

    if matches.get_flag("dry-run") {
        set_dry_run_mode(true);
        println!(
            "{} Dry-run mode enabled - no changes will be made",
            crate::tui::icons::search()
        );
    }

    if matches.get_flag("yes") {
        // Set auto-yes mode (handled by individual prompts)
        unsafe { std::env::set_var("GHOSTCTL_YES", "1") };
    }

    // Handle subcommands
    match matches.subcommand() {
        Some(("version", _)) => {
            println!("ghostctl v{}", env!("CARGO_PKG_VERSION"));
            println!("{} GhostCTL", crate::tui::icons::ghost());
            println!("Author: Christopher Kelley <ckelley@ghostkellz.sh>");
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
        Some(("net", matches)) => handle_network_commands(matches), // Short alias for network
        Some(("security", matches)) => handle_security_commands(matches),
        Some(("sec", matches)) => handle_security_commands(matches), // Short alias for security
        Some(("bluetooth", matches)) | Some(("bt", matches)) => handle_bluetooth_commands(matches),
        Some(("wifi", matches)) | Some(("wlan", matches)) => handle_wifi_commands(matches),
        Some(("sysctl", matches)) | Some(("kernel", matches)) => handle_sysctl_commands(matches),
        Some(("ssh", matches)) => handle_ssh_management(matches), // SSH management with subcommands
        Some(("gpg", matches)) => handle_gpg_management(matches), // GPG management with subcommands
        Some(("dns", matches)) => handle_dnslookup_commands(matches), // DNS lookup with options
        Some(("nc", matches)) => handle_netcat_commands(matches), // Netcat utilities
        Some(("scan", matches)) => handle_scan_command(matches),  // Network scanner
        Some(("cloud", matches)) => handle_cloud_commands(matches),
        Some(("nginx", matches)) => handle_nginx_commands(matches),
        Some(("tools", matches)) => handle_tools_commands(matches),
        Some(("btrfs", matches)) => handle_btrfs_commands(matches),
        Some(("nvidia", matches)) => handle_nvidia_commands(matches),
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
                crate::docker::container::stop_container(id.to_string());
            }
        }
        None => container_management_menu(),
        _ => unreachable!(),
    }
}

fn handle_docker_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => crate::docker::docker_menu(),
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
        Some(("menu", _)) => crate::nginx::acme::acme_management(),
        Some(("install", _)) => crate::nginx::acme::install_acme_sh(),
        Some(("issue", sub_matches)) => {
            if let Some(domain) = sub_matches.get_one::<String>("domain") {
                issue_certificate_cli(domain);
            }
        }
        Some(("renew", _)) => crate::nginx::acme::renew_all_certificates(),
        Some(("list", _)) => crate::nginx::acme::list_certificates(),
        None => crate::nginx::acme::acme_management(),
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
        Some(("clean", sub_matches)) => {
            if let Some(target) = sub_matches.get_one::<String>("target") {
                handle_arch_clean(target);
            }
        }
        Some(("bouncer", sub_matches)) => {
            if let Some(target) = sub_matches.get_one::<String>("target") {
                handle_arch_bouncer(Some(target));
            } else {
                handle_arch_bouncer(None);
            }
        }
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
        Some(("menu", _)) => network::network_menu(),
        Some(("dns", sub_matches)) => {
            if let Some(domain) = sub_matches.get_one::<String>("domain") {
                network::dns::lookup(domain);
            } else {
                println!("❌ Please provide a domain name. Usage: ghostctl network dns <domain>");
            }
        }
        Some(("mesh", _)) => network::mesh::status(),
        Some(("scan", sub_matches)) => {
            // Redirect to new scan command handler
            handle_scan_command(sub_matches);
        }
        Some(("netcat", sub_matches)) => match sub_matches.subcommand() {
            Some(("send", send_matches)) => {
                if let (Some(file), Some(host), Some(port_str)) = (
                    send_matches.get_one::<String>("file"),
                    send_matches.get_one::<String>("host"),
                    send_matches.get_one::<String>("port"),
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
                    receive_matches.get_one::<String>("port"),
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
                    check_matches.get_one::<String>("port"),
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
                println!("Use 'ghostctl nc help' for more details");
            }
            _ => {
                println!(
                    "❌ Unknown netcat subcommand. Use 'ghostctl network netcat help' for available options."
                );
            }
        },
        None => network::network_menu(),
        _ => {
            println!(
                "❌ Unknown network subcommand. Use 'ghostctl network help' for available options."
            );
            network::network_menu();
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
            println!(
                "❌ Unknown cloud subcommand. Use 'ghostctl cloud help' for available options."
            );
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
            crate::nginx::acme::acme_management();
        }
        Some(("update", _)) => {
            println!("🔄 Updating tools...");
            tools::external_tools_menu();
        }
        None => tools::external_tools_menu(),
        _ => {
            println!(
                "❌ Unknown tools subcommand. Use 'ghostctl tools help' for available options."
            );
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
                let subvolume = sub_matches
                    .get_one::<String>("subvolume")
                    .unwrap_or(&default_subvolume);
                btrfs::handle_btrfs_action(crate::BtrfsAction::Create {
                    name: name.clone(),
                    subvolume: subvolume.clone(),
                });
            }
        }
        Some(("delete", sub_matches)) => {
            if let Some(name) = sub_matches.get_one::<String>("name") {
                btrfs::handle_btrfs_action(crate::BtrfsAction::Delete { name: name.clone() });
            }
        }
        Some(("restore", sub_matches)) => {
            if let (Some(name), Some(target)) = (
                sub_matches.get_one::<String>("name"),
                sub_matches.get_one::<String>("target"),
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
            let mountpoint = sub_matches
                .get_one::<String>("mountpoint")
                .unwrap_or(&default_mountpoint);
            btrfs::handle_btrfs_action(crate::BtrfsAction::Scrub {
                mountpoint: mountpoint.clone(),
            });
        }
        Some(("balance", sub_matches)) => {
            let default_mountpoint = String::from("/");
            let mountpoint = sub_matches
                .get_one::<String>("mountpoint")
                .unwrap_or(&default_mountpoint);
            btrfs::handle_btrfs_action(crate::BtrfsAction::Balance {
                mountpoint: mountpoint.clone(),
            });
        }
        Some(("usage", sub_matches)) => {
            let default_mountpoint = String::from("/");
            let mountpoint = sub_matches
                .get_one::<String>("mountpoint")
                .unwrap_or(&default_mountpoint);
            btrfs::handle_btrfs_action(crate::BtrfsAction::Usage {
                mountpoint: mountpoint.clone(),
            });
        }
        Some(("quota", sub_matches)) => {
            let default_mountpoint = String::from("/");
            let mountpoint = sub_matches
                .get_one::<String>("mountpoint")
                .unwrap_or(&default_mountpoint);
            btrfs::handle_btrfs_action(crate::BtrfsAction::Quota {
                mountpoint: mountpoint.clone(),
            });
        }
        Some(("snapper", snapper_matches)) => match snapper_matches.subcommand() {
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
            Some(("cleanup", _)) => {
                btrfs::handle_btrfs_action(crate::BtrfsAction::SnapperCleanup);
            }
            _ => btrfs::btrfs_menu(),
        },
        Some(("cleanup", sub_matches)) => {
            if sub_matches.get_flag("emergency") {
                btrfs::handle_btrfs_action(crate::BtrfsAction::EmergencyCleanup);
            } else if let Some(days) = sub_matches.get_one::<String>("days") {
                btrfs::handle_btrfs_action(crate::BtrfsAction::CleanupByAge { days: days.clone() });
            } else if let Some(range) = sub_matches.get_one::<String>("range") {
                btrfs::handle_btrfs_action(crate::BtrfsAction::CleanupByRange {
                    range: range.clone(),
                });
            } else {
                // Show disk space and cleanup menu
                btrfs::handle_btrfs_action(crate::BtrfsAction::DiskSpace);
            }
        }
        None => btrfs::btrfs_menu(),
        _ => {
            println!(
                "❌ Unknown btrfs subcommand. Use 'ghostctl btrfs help' for available options."
            );
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
        Some(("build-source", sub_matches)) => {
            use crate::nvidia::source_build::{SourceBuildOptions, source_build_workflow};
            use crate::utils::is_dry_run;

            let opts = SourceBuildOptions {
                all_kernels: sub_matches.get_flag("all-kernels"),
                use_dkms: !sub_matches.get_flag("no-dkms"),
                auto_clean: sub_matches.get_flag("auto-clean"),
                dry_run: is_dry_run(),
            };

            if let Err(e) = source_build_workflow(&opts) {
                eprintln!("Build failed: {}", e);
                std::process::exit(1);
            }
        }
        Some(("dkms-status", _)) => {
            use crate::nvidia::source_build::get_old_dkms_entries;
            let entries = get_old_dkms_entries();
            if entries.is_empty() {
                println!("No NVIDIA DKMS entries found.");
            } else {
                println!("NVIDIA DKMS entries:");
                for (module, kernel, status) in entries {
                    println!("  {} - kernel {} ({})", module, kernel, status);
                }
            }
        }
        Some(("dkms-cleanup", _)) => {
            use crate::nvidia::source_build::cleanup_old_versions;
            use crate::utils::is_dry_run;
            if let Err(e) = cleanup_old_versions(false, is_dry_run()) {
                eprintln!("Cleanup failed: {}", e);
            }
        }
        _ => nvidia::nvidia_menu(),
    }
}

fn handle_security_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => security::security_menu(),
        Some(("ssh", _)) => security::ssh::ssh_management(),
        Some(("gpg", _)) => security::gpg::gpg_key_management(),
        Some(("credentials", _)) => security::credentials::credential_management(),
        None => security::security_menu(),
        _ => security::ssh::ssh_management(),
    }
}

#[cfg(target_os = "linux")]
fn handle_bluetooth_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => bluetooth::bluetooth_menu(),
        Some(("tui", _)) => {
            if let Err(e) = bluetooth::bluetooth_tui() {
                crate::tui::error(&format!("Bluetooth TUI error: {}", e));
            }
        }
        Some(("list", _)) => {
            crate::tui::header("Bluetooth Devices");
            // Uses the list functionality from the bluetooth module
            bluetooth::bluetooth_menu(); // Shows the menu which has list option
        }
        Some(("scan", _)) => {
            crate::tui::info("Starting Bluetooth scan...");
            bluetooth::bluetooth_menu(); // Uses the scan functionality
        }
        Some(("power", _)) => {
            crate::tui::info("Toggling Bluetooth adapter power...");
            bluetooth::bluetooth_menu();
        }
        None => bluetooth::bluetooth_menu(),
        _ => bluetooth::bluetooth_menu(),
    }
}

#[cfg(not(target_os = "linux"))]
fn handle_bluetooth_commands(_matches: &ArgMatches) {
    crate::tui::error("Bluetooth management is only available on Linux (requires BlueZ D-Bus)");
}

fn handle_wifi_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => wifi::wifi_menu(),
        Some(("tui", _)) => {
            if let Err(e) = wifi::wifi_tui() {
                crate::tui::error(&format!("WiFi TUI error: {}", e));
            }
        }
        Some(("status", _)) => {
            crate::tui::header("WiFi Status");
            wifi::wifi_menu();
        }
        Some(("list", _)) => {
            crate::tui::header("Known Networks");
            wifi::wifi_menu();
        }
        Some(("scan", _)) => {
            crate::tui::info("Scanning for WiFi networks...");
            wifi::wifi_menu();
        }
        Some(("connect", _)) => wifi::wifi_menu(),
        Some(("disconnect", _)) => wifi::wifi_menu(),
        Some(("power", _)) => wifi::wifi_menu(),
        None => wifi::wifi_menu(),
        _ => wifi::wifi_menu(),
    }
}

fn handle_sysctl_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("menu", _)) => sysctl::sysctl_menu(),
        Some(("tui", _)) => {
            if let Err(e) = sysctl::sysctl_tui() {
                crate::tui::error(&format!("Sysctl TUI error: {}", e));
            }
        }
        Some(("list", _)) => sysctl::sysctl_menu(),
        Some(("search", _)) => sysctl::sysctl_menu(),
        Some(("get", _)) => sysctl::sysctl_menu(),
        Some(("set", _)) => sysctl::sysctl_menu(),
        Some(("export", _)) => sysctl::sysctl_menu(),
        None => sysctl::sysctl_menu(),
        _ => sysctl::sysctl_menu(),
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

fn handle_arch_clean(target: &str) {
    println!("🧹 Cleaning target: {}", target);
    match target {
        "orphans" => arch::archfix::orphans(),
        "mirrors" => arch::optimize_mirrors(),
        "pkgfix" => arch::archfix::pkgfix(),
        "gpg" => arch::fix_gpg_keys(),
        "locks" => arch::reset_pacman_locks(),
        "all" => {
            println!("🧹 Performing comprehensive system cleanup...");
            arch::cleanup_orphans();
            arch::reset_pacman_locks();
            arch::fix_gpg_keys();
            arch::optimize_mirrors();
            println!("✅ System cleanup complete!");
        }
        _ => {
            println!("❌ Unknown clean target: {}", target);
            println!("📋 Available clean targets:");
            println!("  orphans  - Remove orphaned packages");
            println!("  mirrors  - Clean and optimize mirror list");
            println!("  pkgfix   - Clean PKGBUILD issues");
            println!("  gpg      - Clean and fix GPG keys");
            println!("  locks    - Clear pacman locks");
            println!("  all      - Perform all cleanup operations");
        }
    }
}

fn handle_arch_bouncer(target: Option<&String>) {
    use arch::diagnostics::SystemDiagnostics;

    match target {
        Some(target_str) => {
            // Manual target mode
            println!("🏀 Bouncing back from issues with target: {}", target_str);
            match target_str.as_str() {
                "pacman" => {
                    arch::reset_pacman_locks();
                    arch::archfix::fix();
                }
                "keyring" => {
                    arch::fix_gpg_keys();
                    arch::archfix::fix();
                }
                "mirrors" => {
                    arch::optimize_mirrors();
                    println!("🔄 Testing mirror connectivity...");
                    let _ = std::process::Command::new("sudo")
                        .args(&["pacman", "-Sy"])
                        .status();
                }
                "all" => {
                    println!("🏀 Full system bounce-back sequence...");
                    arch::reset_pacman_locks();
                    arch::fix_gpg_keys();
                    arch::optimize_mirrors();
                    arch::archfix::fix();
                    println!("✅ System bounce-back complete!");
                }
                _ => {
                    println!("❌ Unknown bouncer target: {}", target_str);
                    println!("📋 Available bouncer targets:");
                    println!("  (none)   - Auto-detect and fix issues (recommended)");
                    println!("  pacman   - Fix pacman database and bounce back");
                    println!("  keyring  - Fix keyring issues and bounce back");
                    println!("  mirrors  - Fix mirrors and test connectivity");
                    println!("  all      - Full system recovery sequence");
                }
            }
        }
        None => {
            // Auto-detect mode (default)
            println!("🏀 Auto-Bouncer: Detecting and fixing issues...");
            println!("==============================================\n");

            // Run diagnostics
            let diag = SystemDiagnostics::scan();
            diag.print_summary();

            if !diag.has_issues() {
                println!("✅ No issues detected! System is healthy.");
                println!("💡 Running a quick sync to be safe...");
                let _ = std::process::Command::new("sudo")
                    .args(&["pacman", "-Sy"])
                    .status();
                return;
            }

            // Get and execute fix sequence
            let actions = diag.get_fix_sequence();
            println!("🔧 Executing {} fix action(s)...\n", actions.len());

            for (idx, action) in actions.iter().enumerate() {
                println!("[{}/{}] {}", idx + 1, actions.len(), action.description());
                if action.execute() {
                    println!("  ✅ Success");
                } else {
                    println!("  ⚠️  Had issues (continuing...)");
                }
                println!();
            }

            println!("🎯 Running final system sync...");
            let _ = std::process::Command::new("sudo")
                .args(&["pacman", "-Sy"])
                .status();

            println!("\n✅ Auto-bouncer complete! Your system should be back on track.");
        }
    }
}

fn handle_ssh_management(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("help", _)) => {
            println!("� SSH Key Management");
            println!("=====================");
            println!();
            println!("📋 Available commands:");
            println!("  ghostctl ssh help    - Show this help message");
            println!("  ghostctl ssh menu    - Launch interactive SSH management menu");
            println!();
            println!(
                "� Use 'ghostctl ssh menu' to access the full interactive SSH management interface"
            );
        }
        Some(("menu", _)) => security::ssh::ssh_management(),
        _ => {
            // No subcommand provided, launch the existing SSH menu directly
            security::ssh::ssh_management();
        }
    }
}

fn handle_gpg_management(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("help", _)) => {
            println!("🔑 GPG Key Management");
            println!("=====================");
            println!();
            println!("📋 Available commands:");
            println!("  ghostctl gpg help    - Show this help message");
            println!("  ghostctl gpg menu    - Launch interactive GPG management menu");
            println!();
            println!(
                "� Use 'ghostctl gpg menu' to access the full interactive GPG management interface"
            );
        }
        Some(("menu", _)) => security::gpg::gpg_key_management(),
        _ => {
            // No subcommand provided, launch the existing GPG menu directly
            security::gpg::gpg_key_management();
        }
    }
}

fn handle_dnslookup_commands(matches: &ArgMatches) {
    if let Some(domain) = matches.get_one::<String>("domain") {
        if domain == "help" {
            println!("🌐 DNS Lookup and Management");
            println!("============================");
            println!();
            println!("📋 Usage:");
            println!("  ghostctl dns <domain>                    - Basic DNS lookup");
            println!("  ghostctl dns <domain> --type MX         - Specific record type");
            println!("  ghostctl dns <ip> --reverse              - Reverse DNS lookup");
            println!("  ghostctl dns <domain> --server 8.8.8.8  - Use specific DNS server");
            println!("  ghostctl dns help                       - Show this help message");
            println!();
            println!("📖 Examples:");
            println!("  ghostctl dns google.com");
            println!("  ghostctl dns google.com --type MX");
            println!("  ghostctl dns 8.8.8.8 --reverse");
            println!("  ghostctl dns example.com --server 1.1.1.1");
            println!();
            println!("💡 Tip: Use 'ghostctl network menu' for more network tools");
            return;
        }

        let record_type = matches
            .get_one::<String>("type")
            .map(|s| s.as_str())
            .unwrap_or("A");
        let dns_server = matches.get_one::<String>("server");
        let is_reverse = matches.get_flag("reverse");

        println!("🌐 DNS Lookup for: {}", domain);
        if is_reverse {
            println!("🔄 Performing reverse DNS lookup...");
        } else {
            println!("📋 Record type: {}", record_type);
        }
        if let Some(server) = dns_server {
            println!("🎯 Using DNS server: {}", server);
        }

        // Call DNS lookup function
        network::dns::lookup(domain);
    } else {
        // No domain provided, show help
        println!("🌐 DNS Lookup and Management");
        println!("============================");
        println!();
        println!("📋 Usage:");
        println!("  ghostctl dns <domain>                    - Basic DNS lookup");
        println!("  ghostctl dns <domain> --type MX         - Specific record type");
        println!("  ghostctl dns <ip> --reverse              - Reverse DNS lookup");
        println!("  ghostctl dns <domain> --server 8.8.8.8  - Use specific DNS server");
        println!();
        println!("📖 Examples:");
        println!("  ghostctl dns google.com");
        println!("  ghostctl dns google.com --type MX");
        println!("  ghostctl dns 8.8.8.8 --reverse");
        println!("  ghostctl dns example.com --server 1.1.1.1");
        println!();
        println!("💡 Tip: Use 'ghostctl network menu' for more network tools");
    }
}

fn handle_netcat_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("send", sub_matches)) => {
            let file = sub_matches.get_one::<String>("file").unwrap();
            let host = sub_matches.get_one::<String>("host").unwrap();
            let port_str = sub_matches.get_one::<String>("port").unwrap();

            if let Ok(port) = port_str.parse::<u16>() {
                println!("📤 Sending file '{}' to {}:{}", file, host, port);
                network::netcat::send_file(file, host, port);
            } else {
                println!("❌ Invalid port number: {}", port_str);
            }
        }
        Some(("receive", sub_matches)) => {
            let file = sub_matches.get_one::<String>("file").unwrap();
            let port_str = sub_matches.get_one::<String>("port").unwrap();

            if let Ok(port) = port_str.parse::<u16>() {
                println!("📥 Receiving file '{}' on port {}", file, port);
                network::netcat::receive_file(file, port);
            } else {
                println!("❌ Invalid port number: {}", port_str);
            }
        }
        Some(("chat", sub_matches)) => {
            let port_str = sub_matches.get_one::<String>("port").unwrap();
            if let Ok(port) = port_str.parse::<u16>() {
                if let Some(host) = sub_matches.get_one::<String>("host") {
                    println!("💬 Connecting to chat at {}:{}", host, port);
                    network::netcat::chat(Some(host), port);
                } else {
                    println!("💬 Starting chat server on port {}", port);
                    network::netcat::chat(None, port);
                }
            } else {
                println!("❌ Invalid port number: {}", port_str);
            }
        }
        Some(("check", sub_matches)) => {
            let host = sub_matches.get_one::<String>("host").unwrap();
            let port_str = sub_matches.get_one::<String>("port").unwrap();

            if let Ok(port) = port_str.parse::<u16>() {
                println!("🔍 Checking connectivity to {}:{}", host, port);
                network::netcat::check_port(host, port);
            } else {
                println!("❌ Invalid port number: {}", port_str);
            }
        }
        _ => {
            // No subcommand provided, show help
            println!("🔌 Netcat Utilities");
            println!("===================");
            println!();
            println!("📋 Available commands:");
            println!("  ghostctl nc send <file> <host> <port>   - Send file to host");
            println!("  ghostctl nc receive <file> <port>       - Receive file on port");
            println!("  ghostctl nc chat <host> <port>          - Connect to chat");
            println!("  ghostctl nc chat <port>                 - Start chat server");
            println!("  ghostctl nc check <host> <port>         - Check port connectivity");
            println!();
            println!("📖 Examples:");
            println!("  ghostctl nc send backup.tar.gz 192.168.1.100 8080");
            println!("  ghostctl nc receive backup.tar.gz 8080");
            println!("  ghostctl nc chat 192.168.1.100 9999");
            println!("  ghostctl nc chat 9999");
            println!("  ghostctl nc check google.com 80");
            println!();
            println!("💡 Tip: Use 'ghostctl network menu' for more network tools");
        }
    }
}

fn handle_scan_command(matches: &ArgMatches) {
    let target = matches.get_one::<String>("target").unwrap();
    let ports = matches.get_one::<String>("ports").map(|s| s.as_str());
    let threads = matches
        .get_one::<String>("threads")
        .and_then(|s| s.parse().ok());
    let full_scan = matches.get_flag("full");
    let service_detection = matches.get_flag("service");
    let json_output = matches.get_flag("json");
    let quiet = matches.get_flag("quiet");

    if !quiet {
        println!("🔍 GhostCTL Network Scanner");
        println!("===========================");
        println!("Target: {}", target);
        if full_scan {
            println!("Ports: 1-65535 (full scan)");
        } else {
            println!("Ports: {}", ports.unwrap_or("1-1000"));
        }
        println!(
            "Service Detection: {}",
            if service_detection {
                "Enabled"
            } else {
                "Disabled"
            }
        );
        println!("Threads: {}", threads.unwrap_or(100));
        println!();
    }

    // Parse ports for full scan
    let port_spec = if full_scan { Some("1-65535") } else { ports };

    // Convert target to vector for the scanner
    let targets = if target.contains('/') {
        // CIDR range - expand it (simplified for now)
        vec![target.split('/').next().unwrap().to_string()]
    } else {
        vec![target.to_string()]
    };

    // Launch the scanner
    if let Err(e) =
        crate::network::scan::scan_cli(targets, port_spec.map(|s| s.to_string()), threads)
    {
        eprintln!("❌ Scan failed: {}", e);
        std::process::exit(1);
    }
}
