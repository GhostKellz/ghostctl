# ghostctl Documentation

## Overview

ghostctl is a modular, extensible CLI toolkit for Linux power users, sysadmins, and homelabbers. It provides interactive and scriptable management for:
- Btrfs snapshots
- Backups (Restic, Snapper)
- Systemd services/timers (enable, disable, status, create)
- Neovim and plugin management (install, diagnostics, list, update)
- Shell and terminal setup (ZSH, Oh My Zsh, Powerlevel10k, plugins, tmux, Ghostty, WezTerm)
- User management
- Networking tools (netcat, DNS, route, mesh/Tailscale/Headscale CLI)
- Plugin system (install, list, run, user scripts)
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
- User scripts in `~/.config/ghostctl/scripts/`

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
- Tailscale and headscale CLI support (mesh) is CLI-only for security

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

## Backups
- `ghostctl backup run` — Run a Restic backup
- `ghostctl backup schedule` — Schedule backups (systemd timer)
- `ghostctl backup verify` — Verify backup integrity
- `ghostctl backup cleanup` — Prune old backups
- `ghostctl backup restore` — Restore from backup

---

## Systemd Management
- `ghostctl systemd enable` — Enable and start a service/timer
- `ghostctl systemd disable` — Disable and stop a service/timer
- `ghostctl systemd status` — Show status of a service/timer
- `ghostctl systemd create` — Create a new service/timer (interactive)

---

## Shell & Terminal
- `ghostctl shell` — Full ZSH + Oh My Zsh + Powerlevel10k + plugins setup
- `ghostctl terminal ghostty` — Install and configure Ghostty
- `ghostctl terminal wezterm` — Install and configure WezTerm

---

## Plugins & Scripts
- `ghostctl plugin list` — List installed plugins
- `ghostctl plugin install <url>` — Install plugin from URL
- `ghostctl plugin run` — Run a plugin
- `ghostctl script run` — Run a user script (shell or Lua)

---

## Mesh Networking (CLI only)
- `ghostctl mesh up` — Tailscale up with custom config
- `ghostctl mesh advertise <subnet>` — Advertise subnet route
- `ghostctl mesh status` — Show Tailscale status
- `ghostctl mesh down` — Bring down Tailscale
- `ghostctl mesh api` — Generate Headscale API key

---

## Contributing
- Fork, branch, and PR as usual
- See `commands.md` for command reference
- See `README.md` for quickstart and features

---

MIT License © CK Technology LLC
