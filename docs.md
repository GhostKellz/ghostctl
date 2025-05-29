# ghostctl Documentation

## Overview

ghostctl is a modular, extensible CLI toolkit for Linux power users, sysadmins, and homelabbers. It provides interactive and scriptable management for:
- Btrfs snapshots
- Backups (Restic, Snapper)
- Systemd services/timers
- Neovim and plugin management
- Shell and terminal setup
- User management
- Networking tools
- Proxmox helper scripts
- Diagnostics and self-test

---

## Architecture
- Written in Rust, modularized by feature (btrfs, backup, nvim, shell, plugins, etc.)
- Interactive menus via dialoguer
- CLI subcommands via clap
- Plugin system supports Lua and shell scripts
- Proxmox helpers fetch and run scripts from the community-scripts repo

---

## Configuration
- User config stored in `~/.config/ghostctl/config.toml`
- History/logs in `~/.local/share/ghostctl/history.log`
- Plugins in `~/.config/ghostctl/plugins/`

---

## Extending ghostctl
- Add new modules for features (see `src/`)
- Add new plugins (Lua or shell) to the plugins directory
- Add new Proxmox helper scripts by referencing their GitHub raw URL

---

## Security
- Destructive/system-changing actions require confirmation
- All critical actions are logged
- Scripts from the internet prompt for confirmation before execution

---

## Troubleshooting
- Use `ghostctl diagnostics` or the Diagnostics menu for health checks
- Check `~/.local/share/ghostctl/history.log` for action history
- For issues, open a GitHub issue with logs and system info

---

## Btrfs and Snapper

- `ghostctl btrfs list` — List all Btrfs snapshots
- `ghostctl btrfs create <name>` — Create a snapshot
- `ghostctl btrfs delete <name>` — Delete a snapshot
- `ghostctl btrfs restore <name> <target>` — Restore a snapshot to a target
- `ghostctl btrfs snapper_setup` — Deploy Snapper base configs for root and home
- `ghostctl btrfs snapper_edit <config>` — Edit Snapper config in $EDITOR
- `ghostctl btrfs snapper_list` — List available Snapper configs

All destructive actions prompt for confirmation. Snapper integration is ready for both CLI and TUI.

---

## Contributing
- Fork, branch, and PR as usual
- See `commands.md` for command reference
- See `README.md` for quickstart and features

---

MIT License © CK Technology LLC
