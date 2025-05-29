# ghostctl Command Reference

## Top-Level Commands

- `ghostctl menu` — Launch the interactive menu
- `ghostctl help` — Show in-tool documentation
- `ghostctl diagnostics` — Run self-test and health checks

## Btrfs and Snapper
- `ghostctl btrfs list` — List Btrfs snapshots
- `ghostctl btrfs create <name>` — Create a Btrfs snapshot
- `ghostctl btrfs delete <name>` — Delete a Btrfs snapshot
- `ghostctl btrfs restore <name> <target>` — Restore a Btrfs snapshot
- `ghostctl btrfs snapperSetup` — Deploy Snapper base configs
- `ghostctl btrfs snapperEdit <config>` — Edit Snapper config
- `ghostctl btrfs snapperList` — List Snapper configs

## Backups
- `ghostctl backup run` — Run backups
- `ghostctl backup schedule` — Schedule backups
- `ghostctl backup verify` — Verify backups
- `ghostctl backup cleanup` — Cleanup old backups
- `ghostctl backup restore` — Restore backups

## Systemd Management
- `ghostctl systemd enable` — Enable systemd services/timers
- `ghostctl systemd disable` — Disable systemd services/timers
- `ghostctl systemd status` — Show systemd service/timer status
- `ghostctl systemd create` — Create new systemd service/timer

## Shell & Terminal
- `ghostctl shell` — Shell management menu
- `ghostctl terminal ghostty` — Setup Ghostty
- `ghostctl terminal wezterm` — Setup WezTerm

## Plugins & Scripts
- `ghostctl plugin list` — List plugins
- `ghostctl plugin install <url>` — Install plugin from URL
- `ghostctl plugin run` — Run a plugin
- `ghostctl script run` — Run a script

## Mesh Networking (CLI only)
- `ghostctl mesh up` — Bring up mesh network
- `ghostctl mesh advertise <subnet>` — Advertise subnet
- `ghostctl mesh status` — Show mesh network status
- `ghostctl mesh down` — Bring down mesh network
- `ghostctl mesh api` — Access mesh network API

## Arch Maintenance
- `ghostctl arch fix <target>` — Fix Arch makepkg
- `ghostctl arch keyring` — Update Arch keyring
- `ghostctl arch mirrors` — Update Arch mirrors
- `ghostctl arch orphans` — Remove orphaned packages
- `ghostctl arch pkgfix` — Update all packages
- `ghostctl arch optimize` — System performance tuning
- `ghostctl arch full` — Full system maintenance

## NVIDIA Tools
- `ghostctl nvidia clean` — Clean NVIDIA DKMS/modules
- `ghostctl nvidia fix` — Rebuild DKMS/initramfs
- `ghostctl nvidia diagnostics` — NVIDIA/Wayland diagnostics
- `ghostctl nvidia install` — Install NVIDIA proprietary driver
- `ghostctl nvidia open` — Install NVIDIA open driver
- `ghostctl nvidia openbeta` — Install NVIDIA open beta from AUR
- `ghostctl nvidia info` — Show NVIDIA driver info
- `ghostctl nvidia status` — Show NVIDIA driver status
- `ghostctl nvidia optimize` — Run NVIDIA optimization (performance mode)
- `ghostctl nvidia waylandCheck` — Check Wayland compatibility
- `ghostctl nvidia waylandConfig` — Show Wayland config tips
- `ghostctl nvidia perfMode` — Enable NVIDIA performance mode
- `ghostctl nvidia troubleshoot` — Troubleshooting tips
- `ghostctl nvidia writeNvidiaConf` — Write NVIDIA configuration file

## Neovim
- `ghostctl nvim` — Neovim setup menu
- `ghostctl nvim diagnostics` — Neovim diagnostics
- `ghostctl nvim listPlugins` — List installed plugins
- `ghostctl nvim updatePlugins` — Update plugins

## Networking Tools
- `ghostctl netcat <host> <port>` — Netcat/port scan
- `ghostctl dns <domain>` — DNS lookup
- `ghostctl dnssec <domain>` — DNSSEC check
- `ghostctl route` — Print routing table
- `ghostctl gc <host> <port>` — Ghostcat (branded netcat)

## Proxmox
- `ghostctl proxmox run <script_url>` — Run Proxmox helper script

---

For more details, see docs.md and README.md.
