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

### `uefi`

UEFI Secure Boot management for VMs

**Subcommands:**

- `uefi enroll` -- Create OVMF VARS with Secure Boot keys for Windows 11
- `uefi verify` -- Check if VARS file has Secure Boot keys enrolled
- `uefi status` -- Check OVMF firmware and tools

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

Check OVMF firmware and tools

