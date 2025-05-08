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

`ghostctl` is a modern, extensible CLI utility written in Go using Cobra. Designed for Arch Linux power users and homelab administrators, it streamlines common operations like Btrfs recovery, system snapshot management, Headscale subnet routing, and NVIDIA driver fixes.

It's your all-in-one tool for:

* 🧩 Fixing broken Arch installs
* 🧪 Recovering Btrfs snapshots from a chroot or live system
* ⚡ Managing Tailscale/Headscale routes
* 🎮 Maintaining NVIDIA open drivers (DKMS fixes, rebuilds)
* 💻 Simplifying common server maintenance across Arch, Debian, and Fedora

---

## 🔧 Features

* 🧵 **Btrfs Snapshot Recovery** – recover broken systems with `ghostctl btrfs recover`, built to replace manual chroot procedures.
* 🧠 **Arch Linux System Fixes** – kernel cleanup, rebuild initramfs, resolve bootloader failures.
* 🎮 **NVIDIA Driver Automation** – DKMS rebuilds, patching, and sanity checks for TKG/CachyOS users.
* 📦 **Tailscale/Headscale CLI** – manage ACLs, advertise routes, automate subnet mapping.
* 🧪 **PhantomBoot Integration** – generate bootable recovery ISO with your configs baked in.
* 🛠️ **LXC Tools** – configure and deploy Arch or Debian containers with one command.

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

## 🧭 Planned Subcommands (WIP)

```bash
ghostctl btrfs recover         # Restore snapshot to root
ghostctl nvidia fix            # Patch & rebuild modules
ghostctl net advertise         # Configure subnet advertisements
ghostctl net routes            # Show or add subnet routes
ghostctl boot iso              # Create PhantomBoot live ISO
```

---

## 🧪 PhantomBoot (optional)

Use `ghostctl boot iso` to generate a live recovery environment for Btrfs systems with:

* Automated root subvolume switching
* Snapper/timeshift integration
* Kernel/NVIDIA troubleshooting tools
* A minimal KDE/CLI interface

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
