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

# 📖 GhostCTL User Guide

**Complete user documentation for GhostCTL v1.0**

## 🎯 Introduction

GhostCTL is a comprehensive system administration platform designed to simplify complex Linux operations through intuitive, interactive workflows. Whether you're a system administrator, DevOps engineer, or homelab enthusiast, GhostCTL provides the tools you need in a unified interface.

## 🚀 Getting Started

### First Launch
After installation, start GhostCTL with:
```bash
ghostctl menu
```

This opens the main interactive menu where you can explore all available features.

### Initial Setup
1. **Configuration Check**: Run `ghostctl config show` to see current settings
2. **Health Check**: Run `ghostctl health` to verify system status
3. **Update System**: Use `ghostctl arch quick-fix` (on Arch Linux) to resolve common issues

## 🏗️ Core Concepts

### Interactive Menus
GhostCTL uses dialog-based menus for user-friendly operation:
- Navigate with arrow keys
- Select with Enter
- Use space for multi-select options
- ESC or "Back" to return to previous menu

### Workflow Automation
Many operations can be automated:
- Create custom scripts and workflows
- Schedule operations with systemd timers
- Chain multiple operations together
- Save and reuse complex configurations

### Multi-System Support
GhostCTL adapts to your environment:
- Auto-detects installed tools and services
- Provides distribution-specific optimizations
- Gracefully handles missing dependencies
- Offers installation assistance for required tools

## 📋 Main Menu Overview

### 📦 Package & System Management
**Purpose**: Handle package operations and system maintenance
**Key Features**:
- Distribution-specific package management
- System cleanup and optimization
- Dependency resolution
- Update management

**Quick Actions**:
```bash
ghostctl arch quick-fix     # Fix common Arch issues
ghostctl packages update    # Update all packages
```

### 💾 Backup & Snapshot Management
**Purpose**: Protect your data with comprehensive backup solutions
**Key Features**:
- Btrfs snapshot management with Snapper
- Restic backup automation
- Custom backup workflows
- Automated scheduling

**Quick Actions**:
```bash
ghostctl backup create      # Create immediate backup
ghostctl btrfs snapshot     # Create filesystem snapshot
```

### 🐳 DevOps & Container Tools
**Purpose**: Manage containers and development environments
**Key Features**:
- Docker container lifecycle management
- Private Docker registry operations
- Docker Compose orchestration
- Development environment setup

**Quick Actions**:
```bash
ghostctl devops docker      # Docker management
ghostctl dev setup          # Development environment
```

### 🏗️ Infrastructure as Code
**Purpose**: Automate infrastructure management
**Key Features**:
- Ansible playbook management
- Terraform operations
- Multi-cloud provider support
- CI/CD integration

**Quick Actions**:
```bash
ghostctl infrastructure ansible    # Ansible management
ghostctl infrastructure terraform  # Terraform operations
```

### 🔧 Plugin & Script Management
**Purpose**: Extend GhostCTL functionality
**Key Features**:
- System administration scripts
- Custom script integration
- Plugin management
- Script discovery and execution

**Quick Actions**:
```bash
ghostctl scripts sysadmin   # System admin scripts
ghostctl scripts ghostcert  # SSL certificate management
```

### 🔑 Security & Key Management
**Purpose**: Comprehensive security operations
**Key Features**:
- SSH key lifecycle management
- GPG encryption and signing
- SSL certificate management
- Security auditing

**Quick Actions**:
```bash
ghostctl network ssh        # SSH key management
ghostctl arch gpg           # GPG operations
```

## 🛠️ Detailed Feature Guides

### 🔐 SSH Key Management

**Generate SSH Key**:
1. Navigate to Security & Key Management → SSH Key Management
2. Select "Generate new SSH key"
3. Choose key type (Ed25519 recommended)
4. Provide email/identifier
5. Set passphrase (recommended)

**Deploy to Server**:
1. Select "Copy public key to server"
2. Choose the public key to deploy
3. Enter server details (user@hostname)
4. Specify SSH port if not 22
5. Test connection after deployment

**GitHub Integration**:
1. Select "GitHub/GitLab integration"
2. Choose "Add key to GitHub"
3. Copy the displayed public key
4. Add to GitHub SSH settings

### 💾 Backup Workflows

**Create Automated Backup**:
1. Go to Backup & Snapshot Management → Automated Backup Workflows
2. Select "Create new workflow"
3. Choose "Snapshot + Backup" workflow
4. Configure:
   - Snapper configuration
   - Restic repository
   - Retention policy
5. Schedule with systemd timer

**Manual Backup**:
1. Navigate to Backup & Snapshot Management
2. Select "Backup Integration (Restic + Btrfs)"
3. Choose "Backup Btrfs snapshots to Restic"
4. Select repository and backup mode

### 🏥 Proxmox VE Management

**SDN Configuration**:
1. Go to Proxmox VE Management → Network & SDN Configuration
2. Select "Zone Management" → "Create new zone"
3. Choose zone type (Simple, VLAN, VXLAN, etc.)
4. Configure zone parameters
5. Apply SDN configuration

**Firewall Setup**:
1. Navigate to Proxmox VE Management → Firewall Management
2. Select "Security Group Management" → "Create security group"
3. Add firewall rules:
   - Define action (ACCEPT/REJECT/DROP)
   - Set direction (IN/OUT)
   - Configure protocol and ports
   - Specify source/destination

### 🏗️ Infrastructure Automation

**Ansible Project Setup**:
1. Go to Infrastructure as Code → Ansible Management
2. Select "Quick Start" to create project structure
3. Configure inventory and SSH keys
4. Create or edit playbooks
5. Run playbooks with "Execute Playbook"

**Terraform Workflow**:
1. Navigate to Infrastructure as Code → Terraform Management
2. Select "Initialize Project" for new projects
3. Use "Plan Changes" to preview modifications
4. Apply changes with "Apply Changes"
5. Manage state with "State Management"

## 🔧 System Administration

### Arch Linux Optimization

**Quick System Fixes**:
The Arch quick-fix utility addresses common issues:
- **GPG Key Issues**: Corrupted keyrings, failed package verification
- **Package Conflicts**: Python package conflicts, dependency issues
- **Mirror Problems**: Outdated mirrors, slow download speeds
- **Database Issues**: Locked databases, corrupted package cache

**Usage**:
```bash
ghostctl arch quick-fix
```
Select the fixes you need and let GhostCTL handle the resolution.

### Neovim Management

**Health Check**:
1. Navigate to Development Tools → Neovim Management
2. Select "Health Check & Diagnostics" → "Full Health Check"
3. Review the comprehensive report covering:
   - Neovim installation and version
   - Plugin manager status
   - LSP server availability
   - External tool dependencies

**Plugin Management**:
1. Go to "Plugin Management" in Neovim menu
2. GhostCTL detects your plugin manager (lazy.nvim, packer, vim-plug)
3. Use "Plugin Status Check" to see installed plugins
4. Update plugins through your detected manager

## 🌐 Network & Connectivity

### Network Diagnostics
**Comprehensive Testing**:
1. Navigate to Network & SSH Management
2. Select "Network Diagnostics"
3. Run automated tests for:
   - Connectivity
   - DNS resolution
   - Port accessibility
   - Bandwidth testing

### VPN Management
**Tailscale Setup**:
1. Go to Network Management → VPN Management
2. Select "Tailscale setup"
3. Follow guided installation and configuration
4. Join your Tailscale network

## 📊 Monitoring & Maintenance

### System Health Monitoring
**Regular Checks**:
- Run `ghostctl health` daily for quick status
- Use `ghostctl monitor resources` for detailed analysis
- Schedule automated health checks with systemd

**Performance Analysis**:
1. Navigate to System Monitoring → Performance Analysis
2. Select specific areas (CPU, Memory, Disk, Network)
3. View detailed metrics and recommendations

### Log Analysis
**Centralized Logging**:
1. Go to System Management → Service Management
2. Select "View service logs"
3. Choose services to monitor
4. Use filtering and search capabilities

## 🔄 Automation & Scheduling

### Workflow Creation
**Custom Workflows**:
1. Navigate to Backup Management → Automated Workflows
2. Select "Create new workflow"
3. Choose workflow type and parameters
4. GhostCTL generates executable scripts
5. Schedule with systemd timers

**Systemd Integration**:
- Workflows automatically create systemd services and timers
- Monitor with `systemctl status ghostctl-<workflow>.timer`
- View logs with `journalctl -u ghostctl-<workflow>.service`

## 🛡️ Security Best Practices

### Key Management
**SSH Keys**:
- Use Ed25519 for new keys (better security and performance)
- Always use passphrases for private keys
- Regularly audit and rotate keys
- Use different keys for different purposes

**GPG Keys**:
- Generate keys with 4096-bit RSA or Ed25519
- Set reasonable expiration dates
- Backup private keys securely
- Use subkeys for different operations

### System Security
**Regular Audits**:
1. Run security audits monthly: `ghostctl network ssh audit`
2. Check for system updates: `ghostctl packages update`
3. Review firewall rules: `ghostctl proxmox firewall`
4. Monitor system logs: `ghostctl systemd logs`

## 🚨 Troubleshooting

### Common Issues

**Package Manager Problems**:
- **Locked Database**: Use `ghostctl arch quick-fix` → "Reset pacman locks"
- **GPG Errors**: Use `ghostctl arch quick-fix` → "Fix corrupted GPG keys"
- **Mirror Issues**: Use `ghostctl arch quick-fix` → "Update mirror list"

**Backup Failures**:
- Check repository access: `ghostctl backup repos`
- Verify permissions: Ensure proper file system access
- Review logs: Check systemd journal for error details

**SSH Connection Issues**:
- Test key: `ghostctl network ssh test`
- Check permissions: `ghostctl network ssh audit`
- Verify server configuration: Ensure SSH daemon allows key auth

**Docker Problems**:
- Check service: `systemctl status docker`
- Clean system: `ghostctl devops docker cleanup`
- Restart daemon: `ghostctl systemd restart docker`

### Getting Help

**Built-in Help**:
- Use `--help` with any command
- Check command documentation: `ghostctl commands`
- Run health checks: `ghostctl health`

**Community Support**:
- GitHub Issues: Report bugs and request features
- GitHub Discussions: Ask questions and share tips
- Documentation: Visit docs.ghostctl.dev

## 📚 Advanced Usage

### Custom Scripts
**Integration**:
1. Place scripts in `~/.config/ghostctl/scripts/`
2. Make executable: `chmod +x script.sh`
3. Scripts appear in Plugin & Script Management menu
4. Use GhostCTL's dialog functions for consistency

**Script Template**:
```bash
#!/bin/bash
# Custom GhostCTL script

set -e

echo "🚀 Running custom operation..."

# Your logic here

echo "✅ Operation completed!"
```

### Configuration Customization
**Config File**: `~/.config/ghostctl/config.toml`
```toml
[general]
github_user = "your-username"
default_editor = "nvim"
log_level = "info"

[backup]
default_paths = ["/home", "/etc"]
retention_daily = 7

[scripts]
local_scripts_dir = "~/.config/ghostctl/scripts"
auto_discover = true
```

### Plugin Development
**Lua Plugins**: Create `.lua` files in `~/.config/ghostctl/plugins/`
**Shell Plugins**: Create `.sh` files with GhostCTL integration

## 🎯 Best Practices

### Daily Usage
1. **Morning Check**: `ghostctl health`
2. **System Updates**: Weekly `ghostctl packages update`
3. **Backup Verification**: Check backup status regularly
4. **Security Review**: Monthly security audits

### Infrastructure Management
1. **Version Control**: Keep infrastructure code in Git
2. **Testing**: Use plan/dry-run modes before applying changes
3. **Documentation**: Document custom configurations
4. **Monitoring**: Set up automated monitoring and alerts

### Development Environment
1. **Consistency**: Use GhostCTL for team environment setup
2. **Automation**: Create setup scripts for new team members
3. **Documentation**: Maintain environment documentation
4. **Backup**: Regular backup of development configurations

---

**Remember**: GhostCTL is designed to be intuitive. When in doubt, explore the menus and use the built-in help system. Most operations include confirmation prompts to prevent accidental changes.

For the latest documentation and updates, visit: https://docs.ghostctl.dev
