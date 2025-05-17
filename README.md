# ghostctl
[![Arch Linux](https://img.shields.io/badge/Arch%20Linux-optimized-blue?logo=arch-linux)](https://archlinux.org)
[![Tailscale](https://img.shields.io/badge/Tailscale-enabled-blue?logo=tailscale)](https://tailscale.com)
[![Headscale](https://img.shields.io/badge/Headscale-compatible-brightgreen)](https://headscale.net)
[![Btrfs](https://img.shields.io/badge/Btrfs-snapshots%20%26%20recovery-lightgrey?logo=linux)](https://btrfs.readthedocs.io)
[![NVIDIA](https://img.shields.io/badge/NVIDIA-dkms%20helper-green?logo=nvidia)](https://github.com/NVIDIA/open-gpu-kernel-modules)
[![CLI Tool](https://img.shields.io/badge/CLI-Go%20+Cobra-orange?logo=go)](https://github.com/spf13/cobra)

> ⚙️ A powerful CLI tool for managing Linux systems, self-hosted networks, Btrfs recovery, Tailscale/Headscale, and system configuration.

---

## 👻 ✨ What is ghostctl?

`ghostctl` is a modern, extensible CLI utility written in Go using Cobra. Designed for Arch Linux power users and homelab administrators, it streamlines common operations like Btrfs recovery, system snapshot management, Headscale subnet routing, NVIDIA driver fixes, and more.

It's your all-in-one tool for:

* 🧩 Fixing broken Arch installs
* 🧪 Recovering Btrfs snapshots from a chroot or live system
* ⚡ Managing Tailscale/Headscale routes
* 🎮 Maintaining NVIDIA open drivers (DKMS fixes, rebuilds)
* 💻 Simplifying common server maintenance across Arch, Debian, and Fedora
* 🛡️ Automated backups with Restic and Snapper

---

## 🔧 Features

* 🧵 **Btrfs Snapshot Recovery** – recover broken systems with `ghostctl restore-snapshot` or the interactive recovery menu.
* 🧠 **Arch Linux System Fixes** – kernel cleanup, rebuild initramfs, resolve bootloader failures, and more with `ghostctl arch-fix`.
* 🎮 **NVIDIA Driver Automation** – DKMS rebuilds, patching, and sanity checks for proprietary, open, and beta drivers with `ghostctl nvidia-dkms-fix`.
* 📦 **Tailscale/Headscale CLI** – manage ACLs, advertise routes, automate subnet mapping, debug with `ghostctl tailscale ...`.
* 💾 **Automated Backups** – setup Restic (systemd, .env) and Snapper configs interactively with `ghostctl backup-menu`.
* 🛠️ **Systemd Service Management** – enable, disable, start, stop, and check status of services.
* 🧪 **Interactive Main Menu** – access all features from a single menu: `ghostctl menu`.
* 📝 **Verbose & Config Support** – use `--verbose` for detailed output and `--config` for custom config files.

---

## 🚀 Commands & Usage

### Main Menu
```bash
ghostctl menu
```

### Btrfs/Snapper
```bash
ghostctl restore-snapshot [snapshot] [mountpoint]   # Restore a Btrfs/Snapper snapshot
ghostctl recovery-menu                              # Interactive recovery menu
ghostctl backup-menu                                # Setup Restic/Snapper backups
```

### Arch System Maintenance
```bash
ghostctl arch-fix                                   # Run system maintenance (update, mkinitcpio, hooks)
ghostctl fix-makepkg                                # Fix makepkg/dev environment issues
```

### NVIDIA DKMS/Open
```bash
ghostctl nvidia-dkms-fix                            # Diagnose/fix NVIDIA DKMS, open, or beta drivers
```

### Tailscale/Headscale
```bash
ghostctl tailscale status                           # Show Tailscale status
ghostctl tailscale up                               # Re-advertise routes
ghostctl tailscale routes                           # Show/debug routes
ghostctl tailscale debug                            # Show debug info
```

### Systemd Service Management
```bash
ghostctl systemd-service                            # Interactive systemd service management
```

---

## 📦 Installation

> **Coming soon...** Binary releases for Arch, Debian, and static builds.

Until then, build it manually:

```bash
git clone https://github.com/GhostKellz/ghostctl.git
cd ghostctl
go build -o ghostctl
./ghostctl --help
```

---

## 🤝 Contributing

Open PRs, submit issues, or suggest features. Looking for collaborators familiar with:

* Arch Linux recovery tooling
* Btrfs internals
* Kernel patching and module packaging
* Headscale and Tailscale auth flows

---

**License**: MIT  
**Author**: GhostKellz
