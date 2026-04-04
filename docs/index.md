# GhostCTL Documentation

## Quick Start

- [Installation Guide](deployment/INSTALL.md) - Get GhostCTL running
- [Command Reference](reference/COMMANDS.md) - All commands and syntax

## System Management

| Topic | Description |
|-------|-------------|
| [Arch Linux](arch/README.md) | Pacman, AUR, mirrors, system maintenance |
| [Btrfs](btrfs/README.md) | Snapshots, snapper, filesystem management |
| [NVIDIA](nvidia/README.md) | Drivers, DKMS, optimization, Wayland |

## Infrastructure

| Topic | Description |
|-------|-------------|
| [Proxmox](proxmox/README.md) | PVE management, templates, clustering |
| [Docker](docker/README.md) | Containers, compose, homelab stacks |
| [Virtualization](virtualization/README.md) | VFIO, IOMMU, GPU passthrough |
| [Storage](storage/README.md) | S3, MinIO, local and network storage |

## Networking & Security

| Topic | Description |
|-------|-------------|
| [Networking](networking/README.md) | Firewalls, DNS, port scanning |
| [Security](security/README.md) | SSH, GPG, credentials |
| [UEFI](uefi/README.md) | Secure Boot, key enrollment |

## Development & Gaming

| Topic | Description |
|-------|-------------|
| [Development](development/README.md) | Rust, Zig, Go, Python environments |
| [Gaming](gaming/README.md) | Proton, graphics optimization |

## Quick Examples

```bash
# System management
ghostctl arch fix                 # Fix Arch issues
ghostctl btrfs create backup      # Create snapshot

# Network scanning
ghostctl scan 192.168.1.0/24      # Scan network

# Proxmox
ghostctl pve menu                 # PVE management

# GPU passthrough
ghostctl iommu groups --gpu       # List GPU IOMMU groups
ghostctl vfio setup               # VFIO wizard
```
