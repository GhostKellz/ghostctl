# ghostctl

[![Build Status](https://github.com/GhostKellz/ghostctl/actions/workflows/main.yml/badge.svg?branch=main)](https://github.com/GhostKellz/ghostctl/actions)
![Arch Linux](https://img.shields.io/badge/Arch-Linux-blue?logo=arch-linux&logoColor=white)
![Btrfs](https://img.shields.io/badge/Btrfs-supported-blueviolet?logo=linux)
![Rust](https://img.shields.io/badge/Rust-Toolchain-orange?logo=rust)
![NVIDIA](https://img.shields.io/badge/NVIDIA-supported-green?logo=nvidia)
![Vim](https://img.shields.io/badge/Vim-supported-darkgreen?logo=vim)
![Zsh](https://img.shields.io/badge/Zsh-supported-black?logo=gnu-bash)
![Proxmox](https://img.shields.io/badge/Proxmox-helpers-orange?logo=proxmox)

A modern, modular CLI toolkit for Linux power users, sysadmins, and homelabbers. Written in Rust. Supports Btrfs, Restic, Snapper, systemd, Neovim, plugin management, Proxmox helpers, and more.
---

## Features

- Btrfs: List, create, delete, and restore snapshots interactively
- Backup: Restic/Snapper backup setup, scheduling, verification, and cleanup
- Systemd: Enable, disable, start, stop, and check status of any service/timer
- Neovim: Install/manage multiple distros, diagnostics, plugin management
- Shell: Install zsh, Oh My Zsh, Spaceship, Powerlevel10k, set default shell
- Terminal: WezTerm, Kitty, Ghostty config/installation helpers
- Plugins: Lua and shell plugin support, install from URL, metadata display, interactive runner
- User Management: Add, remove, list, and check status of users
- Networking: DNS, DNSSEC, mesh, netcat, route, and more
- Arch/Perf: System optimization, kernel, and package fixes
- Proxmox: Run any helper script from the ProxmoxVE community-scripts repo
- Diagnostics: Self-test, health checks, actionable suggestions
---

## Usage

```sh
ghostctl menu                # Interactive menu
ghostctl backup              # Backup menu
ghostctl btrfs               # Btrfs snapshot menu
ghostctl nvim interactive    # Neovim interactive setup
ghostctl shell install-zsh   # Install zsh
ghostctl terminal wezterm    # Setup WezTerm
ghostctl plugins list        # List plugins
ghostctl plugins install-from-url <url> # Install plugin from URL
ghostctl user add <name>     # Add user
ghostctl dns <domain>        # DNS lookup
ghostctl systemd <svc> <action> # Manage systemd services
ghostctl proxmox             # Proxmox helper scripts menu
ghostctl help                # In-tool documentation
```

---

## Getting Started

1. Clone this repo and build with Cargo
2. Run `ghostctl menu` or any subcommand
3. Use the interactive menus or CLI for all features

---

## Contributing

- PRs and issues welcome!
- See `docs.md` and `commands.md` for more details (coming soon)

---

## License

MIT License © CK Technology LLC
# ghostctl
