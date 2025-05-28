# ghostctl Command Reference

## Top-Level Commands

- `ghostctl menu` — Launch the interactive menu
- `ghostctl help` — Show in-tool documentation
- `ghostctl diagnostics` — Run self-test and health checks

## Btrfs
- `ghostctl btrfs` — Btrfs snapshot menu
- `ghostctl restore <snapshot> <mountpoint>` — Restore a Btrfs/Snapper snapshot

## Backup
- `ghostctl backup` — Backup menu
- `ghostctl backup verify` — Verify backups
- `ghostctl backup schedule` — Schedule backups
- `ghostctl backup cleanup` — Cleanup old backups

## Systemd
- `ghostctl systemd <service> <action>` — Manage systemd services/timers (start, stop, enable, disable, status)

## Neovim
- `ghostctl nvim interactive` — Interactive Neovim setup
- `ghostctl nvim diagnostics` — Neovim diagnostics
- `ghostctl nvim list-plugins` — List installed plugins
- `ghostctl nvim update-plugins` — Update plugins

## Shell
- `ghostctl shell install-zsh` — Install zsh
- `ghostctl shell install-ohmyzsh` — Install Oh My Zsh
- `ghostctl shell install-spaceship` — Install Spaceship prompt
- `ghostctl shell install-powerlevel10k` — Install Powerlevel10k
- `ghostctl shell set-default-zsh` — Set zsh as default shell

## Terminal
- `ghostctl terminal wezterm` — Setup WezTerm
- `ghostctl terminal kitty` — Setup Kitty
- `ghostctl terminal ghostty` — Setup Ghostty

## Plugins
- `ghostctl plugins list` — List plugins
- `ghostctl plugins run <plugin>` — Run a plugin
- `ghostctl plugins install-from-url <url>` — Install plugin from URL

## User Management
- `ghostctl user add <name>` — Add user
- `ghostctl user remove <name>` — Remove user
- `ghostctl user list` — List users
- `ghostctl user status <name>` — Show user status

## Networking
- `ghostctl dns <domain>` — DNS lookup
- `ghostctl dnssec <domain>` — DNSSEC check
- `ghostctl netcat <host> <port>` — Netcat/port scan
- `ghostctl gc <host> <port>` — Ghostcat (branded netcat)
- `ghostctl route` — Print routing table

## Proxmox
- `ghostctl proxmox` — Proxmox helper scripts menu

## NVIDIA
- `ghostctl nvidia clean` — Clean DKMS/modules
- `ghostctl nvidia fix` — Rebuild DKMS/initramfs
- `ghostctl nvidia diagnostics` — NVIDIA/Wayland diagnostics
- `ghostctl nvidia wayland` — Wayland config helpers

## Arch/Perf
- `ghostctl archfix` — Fix Arch makepkg
- `ghostctl optimize` — System performance tuning
- `ghostctl pkgfix` — Update all packages

---

See `ghostctl help` or the interactive menu for more options and details.
