# GhostCTL Command Reference

Auto-generated from `ghostctl docs generate`.

### `system`

Manage system packages and services

**Subcommands:**

- `system update` -- Update system packages
- `system status` -- Show system status
- `system arch` -- Arch Linux management
- `system nixos` -- Manage NixOS system

#### `system update`

Update system packages

#### `system status`

Show system status

#### `system arch`

Arch Linux management

#### `system nixos`

Manage NixOS system

### `dev`

Manage development tools and environments

**Subcommands:**

- `dev menu` -- Development menu
- `dev rust` -- Manage Rust toolchain
- `dev zig` -- Manage Zig toolchain
- `dev go` -- Manage Go toolchain
- `dev python` -- Manage Python toolchain
- `dev js` -- JavaScript/TypeScript toolchain (Node, Bun, Deno)

#### `dev menu`

Development menu

#### `dev rust`

Manage Rust toolchain

#### `dev zig`

Manage Zig toolchain

#### `dev go`

Manage Go toolchain

#### `dev python`

Manage Python toolchain

#### `dev js`

JavaScript/TypeScript toolchain (Node, Bun, Deno)

**Subcommands:**

- `dev js doctor` -- Check JS runtimes, package managers, and project lockfile

##### `dev js doctor`

Check JS runtimes, package managers, and project lockfile

**Options:**

- `<path>` -- Project directory to inspect (default: .)

### `pve`

Proxmox VE management

**Subcommands:**

- `pve menu` -- PVE management menu
- `pve status` -- Show PVE status
- `pve vm` -- Virtual machine management
- `pve ct` -- Container management

#### `pve menu`

PVE management menu

#### `pve status`

Show PVE status

#### `pve vm`

Virtual machine management

**Subcommands:**

- `pve vm list` -- List VMs
- `pve vm create` -- Create VM
- `pve vm start` -- Start VM
- `pve vm stop` -- Stop VM

##### `pve vm list`

List VMs

##### `pve vm create`

Create VM

##### `pve vm start`

Start VM

**Options:**

- `<id>` -- VM ID

##### `pve vm stop`

Stop VM

**Options:**

- `<id>` -- VM ID

#### `pve ct`

Container management

**Subcommands:**

- `pve ct list` -- List containers
- `pve ct create` -- Create container
- `pve ct start` -- Start container
- `pve ct stop` -- Stop container

##### `pve ct list`

List containers

##### `pve ct create`

Create container

##### `pve ct start`

Start container

**Options:**

- `<id>` -- Container ID

##### `pve ct stop`

Stop container

**Options:**

- `<id>` -- Container ID

### `docker`

Manage Docker containers and stacks

**Subcommands:**

- `docker menu` -- Docker menu
- `docker install` -- Install Docker
- `docker status` -- Show Docker service status
- `docker homelab` -- Homelab stacks

#### `docker menu`

Docker menu

#### `docker install`

Install Docker

#### `docker status`

Show Docker service status

#### `docker homelab`

Homelab stacks

### `scripts`

Manage and run local scripts

**Subcommands:**

- `scripts menu` -- Scripts menu
- `scripts local` -- Local scripts
- `scripts run` -- Run a local script by name
- `scripts list` -- List scripts

#### `scripts menu`

Scripts menu

#### `scripts local`

Local scripts

#### `scripts run`

Run a local script by name

**Options:**

- `<script>` -- Script name

#### `scripts list`

List scripts

**Options:**

- `<category>` -- Script category

### `ssl`

SSL certificate management

**Subcommands:**

- `ssl menu` -- SSL menu
- `ssl install` -- Install acme.sh
- `ssl issue` -- Issue certificate
- `ssl renew` -- Renew certificates
- `ssl list` -- List certificates

#### `ssl menu`

SSL menu

#### `ssl install`

Install acme.sh

#### `ssl issue`

Issue certificate

**Options:**

- `<domain>` -- Domain name

#### `ssl renew`

Renew certificates

#### `ssl list`

List certificates

### `nginx`

Manage Nginx web server

**Subcommands:**

- `nginx menu` -- Nginx menu
- `nginx status` -- Nginx status
- `nginx restart` -- Restart Nginx
- `nginx ssl-setup` -- Setup SSL

#### `nginx menu`

Nginx menu

#### `nginx status`

Nginx status

#### `nginx restart`

Restart Nginx

#### `nginx ssl-setup`

Setup SSL

**Options:**

- `<domain>` -- Domain name

### `nvim`

Neovim setup

**Subcommands:**

- `nvim menu` -- Neovim menu
- `nvim install` -- Install Neovim
- `nvim lazyvim` -- Install LazyVim

#### `nvim menu`

Neovim menu

#### `nvim install`

Install Neovim

#### `nvim lazyvim`

Install LazyVim

### `terminal`

Configure terminal emulators

**Subcommands:**

- `terminal menu` -- Terminal menu
- `terminal ghostty` -- Install Ghostty
- `terminal starship` -- Install Starship

#### `terminal menu`

Terminal menu

#### `terminal ghostty`

Install Ghostty

#### `terminal starship`

Install Starship

### `ghost`

Manage Ghost tool suite

**Subcommands:**

- `ghost menu` -- Ghost tools menu
- `ghost install-all` -- Install all Ghost tools
- `ghost reaper` -- Install Reaper
- `ghost oxygen` -- Install Oxygen
- `ghost zion` -- Install Zion
- `ghost status` -- Show Ghost tool suite status

#### `ghost menu`

Ghost tools menu

#### `ghost install-all`

Install all Ghost tools

#### `ghost reaper`

Install Reaper

#### `ghost oxygen`

Install Oxygen

#### `ghost zion`

Install Zion

#### `ghost status`

Show Ghost tool suite status

### `homelab`

Manage homelab environment

**Subcommands:**

- `homelab menu` -- Homelab menu
- `homelab init` -- Initialize homelab

#### `homelab menu`

Homelab menu

#### `homelab init`

Initialize homelab

### `btrfs`

Manage Btrfs filesystems and snapshots

**Subcommands:**

- `btrfs list` -- List snapshots
- `btrfs create` -- Create snapshot
- `btrfs delete` -- Delete snapshot
- `btrfs restore` -- Restore snapshot
- `btrfs status` -- Show filesystem status and health
- `btrfs scrub` -- Start filesystem scrub
- `btrfs balance` -- Start filesystem balance
- `btrfs usage` -- Show filesystem usage
- `btrfs quota` -- Manage quotas
- `btrfs snapper` -- Snapper integration
- `btrfs cleanup` -- Emergency cleanup snapshots

#### `btrfs list`

List snapshots

#### `btrfs create`

Create snapshot

**Options:**

- `<name>` -- Snapshot name
- `-s`, `--subvolume` -- Source subvolume

#### `btrfs delete`

Delete snapshot

**Options:**

- `<name>` -- Snapshot name

#### `btrfs restore`

Restore snapshot

**Options:**

- `<name>` -- Snapshot name
- `<target>` -- Target path

#### `btrfs status`

Show filesystem status and health

#### `btrfs scrub`

Start filesystem scrub

**Options:**

- `<mountpoint>` -- Mountpoint to scrub

#### `btrfs balance`

Start filesystem balance

**Options:**

- `<mountpoint>` -- Mountpoint to balance

#### `btrfs usage`

Show filesystem usage

**Options:**

- `<mountpoint>` -- Mountpoint to analyze

#### `btrfs quota`

Manage quotas

**Options:**

- `<mountpoint>` -- Mountpoint for quota management

#### `btrfs snapper`

Snapper integration

**Subcommands:**

- `btrfs snapper setup` -- Setup snapper configurations
- `btrfs snapper edit` -- Edit snapper config
- `btrfs snapper list` -- List snapper configs
- `btrfs snapper cleanup` -- Cleanup old snapshots

##### `btrfs snapper setup`

Setup snapper configurations

##### `btrfs snapper edit`

Edit snapper config

**Options:**

- `<config>` -- Config name

##### `btrfs snapper list`

List snapper configs

##### `btrfs snapper cleanup`

Cleanup old snapshots

#### `btrfs cleanup`

Emergency cleanup snapshots

**Options:**

- `--emergency` -- Remove ALL snapshots (DANGEROUS)
- `--days` -- Remove snapshots older than X days
- `--range` -- Remove snapshot range (e.g., 1-100)

### `nvidia`

NVIDIA GPU management

**Subcommands:**

- `nvidia install` -- Install NVIDIA drivers
- `nvidia optimize` -- Optimize GPU settings
- `nvidia passthrough` -- Setup GPU passthrough
- `nvidia wayland` -- Configure Wayland support
- `nvidia build-source` -- Build NVIDIA kernel modules from source
- `nvidia dkms-status` -- Show DKMS module status
- `nvidia dkms-cleanup` -- Clean old DKMS entries

#### `nvidia install`

Install NVIDIA drivers

#### `nvidia optimize`

Optimize GPU settings

#### `nvidia passthrough`

Setup GPU passthrough

#### `nvidia wayland`

Configure Wayland support

#### `nvidia build-source`

Build NVIDIA kernel modules from source

**Options:**

- `--all-kernels` -- Build for all installed kernels (default: current kernel only)
- `--dkms` -- Use DKMS-managed build (default: true)
- `--no-dkms` -- Use direct make install instead of DKMS
- `--auto-clean` -- Automatically clean old DKMS entries without prompting

#### `nvidia dkms-status`

Show DKMS module status

#### `nvidia dkms-cleanup`

Clean old DKMS entries

### `iommu`

IOMMU group management and analysis

**Subcommands:**

- `iommu menu` -- Interactive IOMMU menu
- `iommu status` -- Show IOMMU status
- `iommu groups` -- List IOMMU groups
- `iommu analyze` -- Analyze device for passthrough viability
- `iommu gpus` -- List all GPU devices
- `iommu usb` -- List USB controllers
- `iommu nvme` -- List NVMe controllers
- `iommu sata` -- List SATA controllers
- `iommu tree` -- Show PCIe topology tree
- `iommu acs` -- Check ACS override status

#### `iommu menu`

Interactive IOMMU menu

#### `iommu status`

Show IOMMU status

#### `iommu groups`

List IOMMU groups

**Options:**

- `--gpu` -- Show only groups containing GPUs
- `--json` -- Output in JSON format

#### `iommu analyze`

Analyze device for passthrough viability

**Options:**

- `<device>` -- PCI address (e.g., 01:00.0 or 0000:01:00.0)

#### `iommu gpus`

List all GPU devices

#### `iommu usb`

List USB controllers

#### `iommu nvme`

List NVMe controllers

#### `iommu sata`

List SATA controllers

#### `iommu tree`

Show PCIe topology tree

#### `iommu acs`

Check ACS override status

### `vfio`

VFIO passthrough management

**Subcommands:**

- `vfio menu` -- Interactive VFIO menu
- `vfio setup` -- VFIO setup wizard
- `vfio status` -- Show VFIO status and bound devices
- `vfio bind` -- Bind device to vfio-pci driver
- `vfio unbind` -- Unbind device from vfio-pci driver
- `vfio modules` -- Check and load VFIO modules
- `vfio config` -- Show VFIO configuration status
- `vfio kernel-params` -- Show recommended kernel parameters
- `vfio single-gpu` -- Single GPU passthrough management
- `vfio dump-rom` -- Dump GPU VBIOS ROM
- `vfio rom-list` -- List GPUs with ROM information

#### `vfio menu`

Interactive VFIO menu

#### `vfio setup`

VFIO setup wizard

#### `vfio status`

Show VFIO status and bound devices

#### `vfio bind`

Bind device to vfio-pci driver

**Options:**

- `<device>` -- PCI address (e.g., 01:00.0 or 0000:01:00.0)

#### `vfio unbind`

Unbind device from vfio-pci driver

**Options:**

- `<device>` -- PCI address (e.g., 01:00.0 or 0000:01:00.0)

#### `vfio modules`

Check and load VFIO modules

#### `vfio config`

Show VFIO configuration status

#### `vfio kernel-params`

Show recommended kernel parameters

#### `vfio single-gpu`

Single GPU passthrough management

**Subcommands:**

- `vfio single-gpu status` -- Show single GPU passthrough status
- `vfio single-gpu list` -- List configured VMs
- `vfio single-gpu remove` -- Remove hooks for a VM

##### `vfio single-gpu status`

Show single GPU passthrough status

##### `vfio single-gpu list`

List configured VMs

##### `vfio single-gpu remove`

Remove hooks for a VM

**Options:**

- `<vm>` -- VM name

#### `vfio dump-rom`

Dump GPU VBIOS ROM

**Options:**

- `<device>` -- PCI address (e.g., 01:00.0)
- `-o`, `--output` -- Output file path

#### `vfio rom-list`

List GPUs with ROM information

### `security`

Manage security and credentials

**Subcommands:**

- `security menu` -- Security management menu
- `security ssh` -- Configure SSH keys and agent
- `security gpg` -- Manage GPG keys
- `security credentials` -- Manage stored credentials

#### `security menu`

Security management menu

#### `security ssh`

Configure SSH keys and agent

#### `security gpg`

Manage GPG keys

#### `security credentials`

Manage stored credentials

### `bluetooth`

Manage Bluetooth devices

**Subcommands:**

- `bluetooth menu` -- Interactive Bluetooth menu
- `bluetooth tui` -- Launch Bluetooth TUI
- `bluetooth list` -- List adapters and devices
- `bluetooth scan` -- Scan for nearby devices
- `bluetooth power` -- Toggle adapter power

#### `bluetooth menu`

Interactive Bluetooth menu

#### `bluetooth tui`

Launch Bluetooth TUI

#### `bluetooth list`

List adapters and devices

#### `bluetooth scan`

Scan for nearby devices

#### `bluetooth power`

Toggle adapter power

### `wifi`

WiFi network management (requires iwd)

**Subcommands:**

- `wifi menu` -- Interactive WiFi menu
- `wifi tui` -- Launch WiFi TUI
- `wifi status` -- Show WiFi status
- `wifi list` -- List known networks
- `wifi scan` -- Scan for networks
- `wifi connect` -- Connect to a network
- `wifi disconnect` -- Disconnect from network
- `wifi power` -- Toggle WiFi power

#### `wifi menu`

Interactive WiFi menu

#### `wifi tui`

Launch WiFi TUI

#### `wifi status`

Show WiFi status

#### `wifi list`

List known networks

#### `wifi scan`

Scan for networks

#### `wifi connect`

Connect to a network

#### `wifi disconnect`

Disconnect from network

#### `wifi power`

Toggle WiFi power

### `sysctl`

Kernel parameter browser (systeroid-style)

**Subcommands:**

- `sysctl menu` -- Interactive sysctl menu
- `sysctl tui` -- Launch kernel parameter TUI
- `sysctl list` -- List all parameters
- `sysctl search` -- Search parameters
- `sysctl get` -- Get a parameter value
- `sysctl set` -- Set a parameter value
- `sysctl export` -- Export configuration

#### `sysctl menu`

Interactive sysctl menu

#### `sysctl tui`

Launch kernel parameter TUI

#### `sysctl list`

List all parameters

#### `sysctl search`

Search parameters

#### `sysctl get`

Get a parameter value

#### `sysctl set`

Set a parameter value

#### `sysctl export`

Export configuration

### `backup`

Manage backup systems

**Subcommands:**

- `backup setup` -- Setup backup system
- `backup schedule` -- Schedule backups
- `backup verify` -- Verify backups
- `backup cleanup` -- Cleanup old backups

#### `backup setup`

Setup backup system

#### `backup schedule`

Schedule backups

#### `backup verify`

Verify backups

#### `backup cleanup`

Cleanup old backups

### `restore`

Restore system from backups

**Subcommands:**

- `restore btrfs` -- Restore from Btrfs
- `restore system` -- System restore
- `restore chroot` -- Chroot restore

#### `restore btrfs`

Restore from Btrfs

#### `restore system`

System restore

#### `restore chroot`

Chroot restore

### `shell`

Configure shell environment

**Subcommands:**

- `shell setup` -- Setup shell environment
- `shell zsh` -- Install and configure ZSH

#### `shell setup`

Setup shell environment

#### `shell zsh`

Install and configure ZSH

### `systemd`

Manage systemd services and timers

**Subcommands:**

- `systemd enable` -- Enable service
- `systemd disable` -- Disable service
- `systemd status` -- Show service status

#### `systemd enable`

Enable service

**Options:**

- `<service>` -- Service name

#### `systemd disable`

Disable service

**Options:**

- `<service>` -- Service name

#### `systemd status`

Show service status

**Options:**

- `<service>` -- Service name

### `arch`

Manage Arch Linux system

**Subcommands:**

- `arch fix` -- Fix common Arch issues
- `arch clean` -- Clean specific target
- `arch bouncer` -- Fix and bounce back from issues (auto-detects if no target)
- `arch aur` -- AUR package management
- `arch boot` -- Boot configuration
- `arch health` -- System health check
- `arch performance` -- Performance optimization
- `arch optimize` -- Optimize system performance
- `arch mirrors` -- Optimize mirror list
- `arch orphans` -- Clean orphaned packages

#### `arch fix`

Fix common Arch issues

#### `arch clean`

Clean specific target

**Options:**

- `<target>` -- Target to clean (orphans, mirrors, pkgfix, gpg, locks, all)

#### `arch bouncer`

Fix and bounce back from issues (auto-detects if no target)

**Options:**

- `<target>` -- Optional target to fix (pacman, keyring, mirrors, all). Omit for auto-detection.

#### `arch aur`

AUR package management

#### `arch boot`

Boot configuration

#### `arch health`

System health check

#### `arch performance`

Performance optimization

#### `arch optimize`

Optimize system performance

#### `arch mirrors`

Optimize mirror list

#### `arch orphans`

Clean orphaned packages

### `network`

Manage network configuration and tools

**Subcommands:**

- `network menu` -- Network management menu
- `network dns` -- DNS configuration
- `network mesh` -- Mesh networking
- `network scan` -- Scan network ports
- `network netcat` -- Netcat utilities for file transfer and communication

#### `network menu`

Network management menu

#### `network dns`

DNS configuration

**Options:**

- `<domain>` -- Domain name to lookup

#### `network mesh`

Mesh networking

#### `network scan`

Scan network ports

**Options:**

- `<target>` -- Target IP, CIDR, or range (e.g. 192.168.1.1, 192.168.1.0/24)
- `-s` -- Start port [default: 1]
- `-e` -- End port [default: 1024]
- `--banner` -- Enable banner grabbing

#### `network netcat`

Netcat utilities for file transfer and communication

**Subcommands:**

- `network netcat send` -- Send a file
- `network netcat receive` -- Receive a file
- `network netcat chat` -- Start or join a chat session
- `network netcat check` -- Check port connectivity

##### `network netcat send`

Send a file

**Options:**

- `<file>` -- File to send
- `<host>` -- Target host
- `<port>` -- Target port

##### `network netcat receive`

Receive a file

**Options:**

- `<file>` -- File to save as
- `<port>` -- Port to listen on

##### `network netcat chat`

Start or join a chat session

**Options:**

- `<host>` -- Host to connect to (if not provided, starts server)
- `<port>` -- Port to use (required)

##### `network netcat check`

Check port connectivity

**Options:**

- `<host>` -- Host to check
- `<port>` -- Port to check

### `cloud`

Manage cloud provider integrations

**Subcommands:**

- `cloud aws` -- AWS management
- `cloud azure` -- Azure management
- `cloud gcp` -- Google Cloud management

#### `cloud aws`

AWS management

#### `cloud azure`

Azure management

#### `cloud gcp`

Google Cloud management

### `tools`

Install and manage system tools

**Subcommands:**

- `tools install` -- Install development tools
- `tools configure` -- Configure tools
- `tools update` -- Update tools

#### `tools install`

Install development tools

#### `tools configure`

Configure tools

#### `tools update`

Update tools

### `net`

Manage network configuration (short alias)

**Subcommands:**

- `net menu` -- Network management menu
- `net dns` -- DNS configuration
- `net mesh` -- Mesh networking
- `net scan` -- Network port scanning
- `net netcat` -- Netcat utilities

#### `net menu`

Network management menu

#### `net dns`

DNS configuration

**Options:**

- `<domain>` -- Domain name to lookup

#### `net mesh`

Mesh networking

#### `net scan`

Network port scanning

**Options:**

- `<target>` -- Target IP, CIDR, or range
- `-s` -- Start port
- `-e` -- End port
- `--banner` -- Enable banner grabbing

#### `net netcat`

Netcat utilities

**Subcommands:**

- `net netcat send` -- Send a file
- `net netcat receive` -- Receive a file
- `net netcat chat` -- Start or join a chat session
- `net netcat check` -- Check port connectivity

##### `net netcat send`

Send a file

**Options:**

- `<file>` -- File to send
- `<host>` -- Target host
- `<port>` -- Target port

##### `net netcat receive`

Receive a file

**Options:**

- `<file>` -- File to save as
- `<port>` -- Port to listen on

##### `net netcat chat`

Start or join a chat session

**Options:**

- `<host>` -- Host to connect to
- `<port>` -- Port to use (required)

##### `net netcat check`

Check port connectivity

**Options:**

- `<host>` -- Host to check
- `<port>` -- Port to check

### `sec`

Security management (short alias)

**Subcommands:**

- `sec menu` -- Security management menu
- `sec ssh` -- SSH configuration
- `sec gpg` -- GPG management
- `sec credentials` -- Credential management

#### `sec menu`

Security management menu

#### `sec ssh`

SSH configuration

#### `sec gpg`

GPG management

#### `sec credentials`

Credential management

### `ssh`

SSH configuration and management

**Subcommands:**

- `ssh menu` -- Interactive SSH management menu
- `ssh generate` -- Generate new SSH key pair
- `ssh list` -- List SSH keys
- `ssh copy-id` -- Copy SSH key to remote host
- `ssh config` -- SSH configuration management

#### `ssh menu`

Interactive SSH management menu

#### `ssh generate`

Generate new SSH key pair

#### `ssh list`

List SSH keys

#### `ssh copy-id`

Copy SSH key to remote host

**Options:**

- `<target>` -- user@hostname

#### `ssh config`

SSH configuration management

### `gpg`

GPG key management

**Subcommands:**

- `gpg list` -- List all GPG keys
- `gpg info` -- Show key details
- `gpg export` -- Export public key to stdout
- `gpg renew` -- Extend key expiration
- `gpg menu` -- Launch interactive GPG menu

#### `gpg list`

List all GPG keys

#### `gpg info`

Show key details

**Options:**

- `<KEY_ID>` -- Key ID, email, or name

#### `gpg export`

Export public key to stdout

**Options:**

- `<KEY_ID>` -- Key ID, email, or name

#### `gpg renew`

Extend key expiration

**Options:**

- `<KEY_ID>` -- Key ID to renew
- `-d`, `--duration` -- Duration to extend

#### `gpg menu`

Launch interactive GPG menu

### `dns`

DNS lookup and management

**Options:**

- `<domain>` -- Domain name to lookup
- `-t`, `--type` -- DNS record type (A, AAAA, MX, NS, TXT, etc.)
- `-r`, `--reverse` -- Perform reverse DNS lookup
- `-s`, `--server` -- DNS server to use

### `nc`

Netcat utilities

**Subcommands:**

- `nc send` -- Send file to host
- `nc receive` -- Receive file on port
- `nc chat` -- Start chat session
- `nc check` -- Check port connectivity

#### `nc send`

Send file to host

**Options:**

- `<file>` -- File to send
- `<host>` -- Target host
- `<port>` -- Target port

#### `nc receive`

Receive file on port

**Options:**

- `<file>` -- Output file
- `<port>` -- Listen port

#### `nc chat`

Start chat session

**Options:**

- `<host>` -- Host to connect to (omit for server mode)
- `<port>` -- Port (required)

#### `nc check`

Check port connectivity

**Options:**

- `<host>` -- Target host
- `<port>` -- Target port

### `scan`

Network scanner with beautiful TUI

**Options:**

- `<target>` -- Target IP/hostname/CIDR range
- `--ports` -- Port specification (e.g., 80,443,8080 or 1-1000)
- `-t`, `--threads` -- Number of concurrent threads
- `--full` -- Scan all 65535 ports
- `--service` -- Enable service detection
- `--json` -- Output results in JSON format (no TUI)
- `-q`, `--quiet` -- Minimal output

### `completion`

Generate shell completions

**Options:**

- `<shell>` -- Shell to generate completions for

### `support`

Support diagnostics and local state

**Subcommands:**

- `support doctor` -- Run quick support readiness checks
- `support paths` -- Show support and log paths
- `support logs` -- Show recent GhostCTL activity logs
- `support bundle` -- Write a shareable support bundle

#### `support doctor`

Run quick support readiness checks

#### `support paths`

Show support and log paths

#### `support logs`

Show recent GhostCTL activity logs

#### `support bundle`

Write a shareable support bundle

**Options:**

- `-o`, `--output` -- Output path for the support bundle
- `--redact-paths` -- Redact home directory paths from the bundle
- `--gzip` -- Write gzip-compressed text bundle
- `--tarball` -- Write tar.gz archive containing bundle and metadata
- `--log-tail` -- Number of recent activity log lines to include

### `docs`

Documentation utilities

**Subcommands:**

- `docs generate` -- Generate command reference from CLI definition

#### `docs generate`

Generate command reference from CLI definition

**Options:**

- `-o`, `--output` -- Write to file instead of stdout

### `version`

Show version information

### `list`

List available commands

### `config`

Manage ghostctl's own configuration (~/.config/ghostctl/config.toml)

**Subcommands:**

- `config show` -- Print all resolved configuration sections
- `config edit` -- Open the config file in $EDITOR
- `config path` -- Print the config file path
- `config reset` -- Delete the config and regenerate defaults

#### `config show`

Print all resolved configuration sections

#### `config edit`

Open the config file in $EDITOR

#### `config path`

Print the config file path

#### `config reset`

Delete the config and regenerate defaults

### `uefi`

UEFI Secure Boot management for VMs

**Subcommands:**

- `uefi enroll` -- Create OVMF VARS with Secure Boot keys for Windows 11
- `uefi verify` -- Check if VARS file has Secure Boot keys enrolled
- `uefi status` -- Check OVMF firmware, key enrollment tools, and swtpm

#### `uefi enroll`

Create OVMF VARS with Secure Boot keys for Windows 11

**Options:**

- `-o`, `--output` -- Output path for enrolled VARS file
- `--template` -- Path to OVMF_VARS.fd template
- `-v`, `--verbose` -- Show detailed output

#### `uefi verify`

Check if VARS file has Secure Boot keys enrolled

**Options:**

- `<file>` -- VARS file to verify
- `-v`, `--verbose` -- Show full variable dump

#### `uefi status`

Check OVMF firmware, key enrollment tools, and swtpm

### `sign`

[EXPERIMENTAL] Code signing via Azure Key Vault

**Subcommands:**

- `sign file` -- Sign a file using Azure Key Vault
- `sign config` -- Show or initialize signing configuration
- `sign status` -- Check signing dependencies and Azure connectivity
- `sign export-key` -- Export the signing public key from Azure Key Vault
- `sign verify` -- Verify a file signature against Azure Key Vault certificate
- `sign list-keys` -- List signing certificates in Azure Key Vault

#### `sign file`

Sign a file using Azure Key Vault

**Options:**

- `<FILE>` -- File to sign
- `--format` -- Signing format (auto-detect by default)
- `--vault-url` -- Azure Key Vault URL (overrides config)
- `--cert-name` -- Key/certificate name in Key Vault (overrides config)
- `-a`, `--algorithm` -- Signing algorithm (default: from config or RS256)
- `-o`, `--output` -- Output path for signature file
- `--auth` -- Authentication method override
- `--dry-run` -- Show what would be signed without calling Key Vault
- `--timestamp` -- Request RFC 3161 timestamp and fail if timestamping fails
- `--no-timestamp` -- Disable timestamping
- `--native` -- Use native package signing (RPM: embed in header, DEB: dpkg-sig format)
- `-v`, `--verbose` -- Verbose output

#### `sign config`

Show or initialize signing configuration

**Options:**

- `--init` -- Interactive signing configuration setup

#### `sign status`

Check signing dependencies and Azure connectivity

#### `sign export-key`

Export the signing public key from Azure Key Vault

**Options:**

- `--format` -- Export format: pgp (ASCII-armored OpenPGP), pem (X.509 PEM), der (raw DER)
- `-o`, `--output` -- Output file (stdout if omitted)
- `--vault-url` -- Azure Key Vault URL (overrides config)
- `--cert-name` -- Key/certificate name in Key Vault (overrides config)
- `--auth` -- Authentication method override

#### `sign verify`

Verify a file signature against Azure Key Vault certificate

**Options:**

- `<FILE>` -- File to verify
- `-s`, `--signature` -- Signature file path (default: FILE.sig)
- `--vault-url` -- Azure Key Vault URL (overrides config)
- `--cert-name` -- Key/certificate name in Key Vault (overrides config)
- `--auth` -- Authentication method override
- `-v`, `--verbose` -- Verbose output

#### `sign list-keys`

List signing certificates in Azure Key Vault

**Options:**

- `--vault-url` -- Azure Key Vault URL (overrides config)
- `--auth` -- Authentication method override

### `monitor`

Observability helper (Prometheus, Loki, Alertmanager, Grafana)

**Subcommands:**

- `monitor health` -- Probe all configured services for liveness
- `monitor targets` -- List Prometheus scrape targets and their health
- `monitor alerts` -- List alerts currently known to Alertmanager
- `monitor logs` -- Query Loki with a LogQL expression
- `monitor tail` -- Follow new Loki log lines for a LogQL query
- `monitor query` -- Run a pre-baked PromQL query
- `monitor reload` -- Hot-reload a service config (no restart)
- `monitor datasources` -- Check Grafana datasource health (needs grafana_token)

#### `monitor health`

Probe all configured services for liveness

#### `monitor targets`

List Prometheus scrape targets and their health

**Options:**

- `--down` -- Show only targets that are not up

#### `monitor alerts`

List alerts currently known to Alertmanager

#### `monitor logs`

Query Loki with a LogQL expression

**Options:**

- `<query>` -- LogQL query, e.g. '{source_type="fortigate"}'
- `--limit` -- Maximum number of lines to return

#### `monitor tail`

Follow new Loki log lines for a LogQL query

**Options:**

- `<query>` -- LogQL query to follow
- `-f`, `--follow` -- Keep polling for new lines (Ctrl-C to stop)

#### `monitor query`

Run a pre-baked PromQL query

**Options:**

- `<metric>` -- Which metric to query
- `--host` -- Filter results to instances containing this string

#### `monitor reload`

Hot-reload a service config (no restart)

**Options:**

- `<service>` -- Service to reload

#### `monitor datasources`

Check Grafana datasource health (needs grafana_token)

### `ai`

Local AI helper (Ollama models + Hermes agent)

**Subcommands:**

- `ai status` -- Ollama service health, GPU detection, and loaded models
- `ai models` -- List installed Ollama models
- `ai pull` -- Pull a model from the Ollama registry
- `ai rm` -- Delete an installed model
- `ai show` -- Show a model's architecture and max context window
- `ai ctx-check` -- Verify a model's context window meets the configured minimum
- `ai run` -- Run a one-shot prompt against a model (streams output)
- `ai ps` -- Show currently loaded models and VRAM usage
- `ai tune` -- Inspect or apply Ollama server tuning (systemd override env)
- `ai hermes` -- Pass through to the Hermes agent CLI

#### `ai status`

Ollama service health, GPU detection, and loaded models

#### `ai models`

List installed Ollama models

#### `ai pull`

Pull a model from the Ollama registry

**Options:**

- `<model>` -- Model name, e.g. qwen3-coder:30b

#### `ai rm`

Delete an installed model

**Options:**

- `<model>` -- Model name to delete

#### `ai show`

Show a model's architecture and max context window

**Options:**

- `<model>` -- Model name

#### `ai ctx-check`

Verify a model's context window meets the configured minimum

**Options:**

- `<model>` -- Model name (defaults to [ai].default_model)

#### `ai run`

Run a one-shot prompt against a model (streams output)

**Options:**

- `<model>` -- Model name
- `<prompt>` -- Prompt text
- `--no-stream` -- Wait for the full response instead of streaming
- `--ctx` -- Context window size in tokens (options.num_ctx)
- `--temp` -- Sampling temperature (options.temperature)
- `--num-predict` -- Max tokens to generate, -1 for unlimited (options.num_predict)
- `--seed` -- RNG seed for reproducible output (options.seed)

#### `ai ps`

Show currently loaded models and VRAM usage

#### `ai tune`

Inspect or apply Ollama server tuning (systemd override env)

**Subcommands:**

- `ai tune show` -- Show the current Ollama override environment
- `ai tune recommend` -- Print recommended tuning for the detected GPU VRAM
- `ai tune apply` -- Write the recommended override (sudo) and restart ollama

##### `ai tune show`

Show the current Ollama override environment

##### `ai tune recommend`

Print recommended tuning for the detected GPU VRAM

##### `ai tune apply`

Write the recommended override (sudo) and restart ollama

#### `ai hermes`

Pass through to the Hermes agent CLI

**Options:**

- `<args>` -- Arguments forwarded to `hermes` (e.g. doctor, status, model)

### `crowdsec`

Threat-feed, CrowdSec metrics, and DNS posture checks

**Subcommands:**

- `crowdsec feed` -- Inspect the public threat feed
- `crowdsec metrics` -- Summarize CrowdSec LAPI Prometheus metrics (if configured)
- `crowdsec cli` -- Passthrough to local cscli (only works on the LAPI host)
- `crowdsec dns` -- Check DNS resolver reachability and DNSSEC

#### `crowdsec feed`

Inspect the public threat feed

**Subcommands:**

- `crowdsec feed check` -- Fetch the feed and report entry count + size
- `crowdsec feed sample` -- Show the first N entries of the feed

##### `crowdsec feed check`

Fetch the feed and report entry count + size

##### `crowdsec feed sample`

Show the first N entries of the feed

**Options:**

- `<count>` -- Number of entries to show

#### `crowdsec metrics`

Summarize CrowdSec LAPI Prometheus metrics (if configured)

#### `crowdsec cli`

Passthrough to local cscli (only works on the LAPI host)

**Options:**

- `<category>` -- cscli category to list

#### `crowdsec dns`

Check DNS resolver reachability and DNSSEC

**Subcommands:**

- `crowdsec dns check` -- Test lookups against the configured resolvers

##### `crowdsec dns check`

Test lookups against the configured resolvers

### `obs`

OBS Studio helper: Wayland screencapture, virtual camera, NVENC

**Subcommands:**

- `obs doctor` -- Full OBS environment report (session, portal, PipeWire, vcam, NVENC)
- `obs portal` -- Wayland screencapture via xdg-desktop-portal + PipeWire
- `obs vcam` -- OBS virtual camera via the v4l2loopback kernel module
- `obs nvenc` -- NVIDIA hardware encoding (NVENC) checks
- `obs screencast` -- Verify the Wayland ScreenCast portal is usable

#### `obs doctor`

Full OBS environment report (session, portal, PipeWire, vcam, NVENC)

#### `obs portal`

Wayland screencapture via xdg-desktop-portal + PipeWire

**Subcommands:**

- `obs portal check` -- Check the portal backend and PipeWire status
- `obs portal setup` -- Install + enable the right portal backend and PipeWire stack

##### `obs portal check`

Check the portal backend and PipeWire status

##### `obs portal setup`

Install + enable the right portal backend and PipeWire stack

#### `obs vcam`

OBS virtual camera via the v4l2loopback kernel module

**Subcommands:**

- `obs vcam status` -- Show v4l2loopback and video devices
- `obs vcam enable` -- Load v4l2loopback with OBS-friendly options
- `obs vcam disable` -- Unload the v4l2loopback module

##### `obs vcam status`

Show v4l2loopback and video devices

##### `obs vcam enable`

Load v4l2loopback with OBS-friendly options

**Options:**

- `--persist` -- Make the virtual camera load on every boot

##### `obs vcam disable`

Unload the v4l2loopback module

#### `obs nvenc`

NVIDIA hardware encoding (NVENC) checks

**Subcommands:**

- `obs nvenc check` -- Verify driver + ffmpeg NVENC support

##### `obs nvenc check`

Verify driver + ffmpeg NVENC support

#### `obs screencast`

Verify the Wayland ScreenCast portal is usable

**Subcommands:**

- `obs screencast test` -- Probe the ScreenCast portal interface

##### `obs screencast test`

Probe the ScreenCast portal interface

### `openshell`

OpenShell sandbox runtime: readiness checks and CLI passthrough

**Subcommands:**

- `openshell doctor` -- Check OpenShell prerequisites (binary, docker, gateway, registration)
- `openshell status` -- Show active gateway connection (passthrough)
- `openshell gateway` -- Manage gateways (passthrough to `openshell gateway`)
- `openshell sandbox` -- Manage isolated sandboxes (passthrough to `openshell sandbox`)
- `openshell policy` -- Manage sandbox policy (passthrough to `openshell policy`)

#### `openshell doctor`

Check OpenShell prerequisites (binary, docker, gateway, registration)

#### `openshell status`

Show active gateway connection (passthrough)

#### `openshell gateway`

Manage gateways (passthrough to `openshell gateway`)

**Options:**

- `<args>` -- Arguments forwarded to the `openshell` CLI

#### `openshell sandbox`

Manage isolated sandboxes (passthrough to `openshell sandbox`)

**Options:**

- `<args>` -- Arguments forwarded to the `openshell` CLI

#### `openshell policy`

Manage sandbox policy (passthrough to `openshell policy`)

**Options:**

- `<args>` -- Arguments forwarded to the `openshell` CLI

### `gitlab`

Self-hosted GitLab: connectivity, CI lint, pipelines, MRs, and runners

**Subcommands:**

- `gitlab status` -- Verify connectivity and authentication to the configured instance
- `gitlab ci-lint` -- Validate a GitLab CI file via the CI Lint API
- `gitlab pipelines` -- List recent pipelines for the configured project
- `gitlab pipeline` -- Show one pipeline and its jobs, grouped by stage
- `gitlab trace` -- Print a job's log (useful for debugging a failed CI job)
- `gitlab runners` -- List CI runners available to the project (online/status)
- `gitlab mrs` -- List open merge requests for the project
- `gitlab projects` -- List projects you are a member of (with their ids and paths)
- `gitlab run` -- Trigger a new pipeline (write; honors --dry-run/--yes)
- `gitlab retry` -- Retry a pipeline (write; honors --dry-run/--yes)
- `gitlab cancel` -- Cancel a pipeline (write; honors --dry-run/--yes)

#### `gitlab status`

Verify connectivity and authentication to the configured instance

#### `gitlab ci-lint`

Validate a GitLab CI file via the CI Lint API

**Options:**

- `<file>` -- CI file to validate (default: .gitlab-ci.yml)

#### `gitlab pipelines`

List recent pipelines for the configured project

#### `gitlab pipeline`

Show one pipeline and its jobs, grouped by stage

**Options:**

- `<id>` -- Pipeline id (see `gitlab pipelines`)

#### `gitlab trace`

Print a job's log (useful for debugging a failed CI job)

**Options:**

- `<job>` -- Job id

#### `gitlab runners`

List CI runners available to the project (online/status)

#### `gitlab mrs`

List open merge requests for the project

#### `gitlab projects`

List projects you are a member of (with their ids and paths)

#### `gitlab run`

Trigger a new pipeline (write; honors --dry-run/--yes)

**Options:**

- `<ref>` -- Branch or tag to run (default: the project's default branch)

#### `gitlab retry`

Retry a pipeline (write; honors --dry-run/--yes)

**Options:**

- `<id>` -- Pipeline id

#### `gitlab cancel`

Cancel a pipeline (write; honors --dry-run/--yes)

**Options:**

- `<id>` -- Pipeline id

### `audit`

Audit Arch/AUR packages for CVEs and malicious PKGBUILDs

**Subcommands:**

- `audit cve` -- Check installed packages against the Arch Security Tracker
- `audit aur` -- Scan installed AUR/foreign package PKGBUILDs for red flags
- `audit pkgbuild` -- Scan a single PKGBUILD (local path or AUR package name)
- `audit ioc` -- Check installed packages and pacman history against an IOC package feed
- `audit cargo` -- Audit a Rust project's Cargo.lock against OSV (RustSec) advisories
- `audit node` -- Audit a Node project's lockfile (bun/pnpm/yarn/npm) against OSV advisories
- `audit deps` -- Auto-detect project lockfiles (cargo + node) and audit them together
- `audit ci` -- Audit CI/CD workflows (GitHub Actions, GitLab CI) for deprecated/outdated constructs
- `audit summary` -- Quick package-security overview

#### `audit cve`

Check installed packages against the Arch Security Tracker

#### `audit aur`

Scan installed AUR/foreign package PKGBUILDs for red flags

#### `audit pkgbuild`

Scan a single PKGBUILD (local path or AUR package name)

**Options:**

- `<target>` -- Path to a PKGBUILD file, or an AUR package name to fetch

#### `audit ioc`

Check installed packages and pacman history against an IOC package feed

**Options:**

- `--feed` -- Package-name feed to use (overrides the [audit] ioc_feed setting)

#### `audit cargo`

Audit a Rust project's Cargo.lock against OSV (RustSec) advisories

**Options:**

- `<path>` -- Project directory to audit (default: current directory)
- `--json` -- Emit findings as JSON (exits non-zero on High/Critical)

#### `audit node`

Audit a Node project's lockfile (bun/pnpm/yarn/npm) against OSV advisories

**Options:**

- `<path>` -- Project directory to audit (default: current directory)
- `--json` -- Emit findings as JSON (exits non-zero on High/Critical)

#### `audit deps`

Auto-detect project lockfiles (cargo + node) and audit them together

**Options:**

- `<path>` -- Project directory to audit (default: current directory)
- `--json` -- Emit findings as JSON (exits non-zero on High/Critical)

#### `audit ci`

Audit CI/CD workflows (GitHub Actions, GitLab CI) for deprecated/outdated constructs

**Options:**

- `<path>` -- Project directory to audit (default: current directory)
- `--json` -- Emit findings as JSON (exits non-zero on High/Critical)

#### `audit summary`

Quick package-security overview
