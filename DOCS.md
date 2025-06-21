# 📖 GhostCTL Documentation v0.7.0

Complete usage guide and examples for GhostCTL - The ultimate system and homelab management tool.

## 🏗️ Architecture

GhostCTL is built with a modular architecture in Rust, providing:
- **Interactive Menus**: Powered by dialoguer for user-friendly navigation
- **CLI Commands**: Full clap-based command structure for scripting
- **Modular Design**: Each feature is a separate module for maintainability
- **Cross-Platform**: Built for Linux with distro-specific optimizations
- **Extensible**: Plugin system and configuration management

## 📁 Directory Structure

```
~/.config/ghostctl/           # Main configuration directory
├── config.toml              # User configuration
├── plugins/                 # User plugins (Lua/Shell)
├── scripts/                 # Custom user scripts
└── themes/                  # Terminal themes

~/.local/share/ghostctl/      # Data directory
├── history.log              # Action history
├── backups/                 # Backup metadata
└── cache/                   # Temporary files

/data/projects/ghostctl/      # Development directory
├── scripts/                 # Built-in scripts
└── docs/                    # Documentation
```

## 🚀 Getting Started

### Basic Usage
```bash
# Launch interactive menu
ghostctl

# Show version
ghostctl version

# Show help
ghostctl help

# Access specific modules
ghostctl arch menu           # Arch Linux management
ghostctl nvidia menu         # NVIDIA management
ghostctl dev menu            # Development environment
```

## 🐧 Arch Linux System Management

### Quick System Maintenance
```bash
# Complete system maintenance
ghostctl arch full

# Fix common issues
ghostctl arch fix

# Optimize performance
ghostctl arch optimize

# Health check
ghostctl arch health
```

### Detailed Examples

#### System Health Monitoring
```bash
# Check disk space and cleanup
ghostctl arch health
# - Shows disk usage by partition
# - Identifies large files and directories
# - Offers cleanup suggestions
# - Checks for orphaned packages
```

#### Dotfiles Management
```bash
# Initialize dotfiles repository
ghostctl arch dotfiles
# - Scans home directory for dotfiles
# - Creates Git repository
# - Sets up automatic synchronization
# - Manages dotfile conflicts

# Example workflow:
# 1. Scan: Finds ~/.bashrc, ~/.vimrc, ~/.gitconfig
# 2. Backup: Creates timestamped backups
# 3. Initialize: Sets up Git repo in ~/dotfiles
# 4. Sync: Symlinks files and commits changes
```

#### Swap and Zram Configuration
```bash
# Configure optimal swap/zram
ghostctl arch swap
# - Analyzes system RAM (16GB example)
# - Recommends: 8GB zram + 4GB swap file
# - Configures compression (lz4/zstd)
# - Sets up systemd services

# Example configuration for 16GB RAM system:
# - zram0: 8GB compressed memory (lz4)
# - swap file: 4GB on disk
# - swappiness: 10 (prefer RAM)
```

#### AUR Helper Management
```bash
# Manage AUR helpers
ghostctl arch aur
# - Installs preferred helper (yay, paru, reaper)
# - Configures build settings
# - Sets up parallel builds
# - Manages build cache cleanup

# Example setup:
# 1. Install reaper (Ghost's AUR helper)
# 2. Configure 4 parallel builds
# 3. Set CCACHE for faster builds
# 4. Schedule weekly cache cleanup
```

## 🎮 NVIDIA Management

### Driver Installation
```bash
# Interactive driver selection
ghostctl nvidia drivers

# Specific driver types:
# - Proprietary: nvidia, nvidia-lts
# - Open Source: nvidia-open, nvidia-open-dkms  
# - Beta: nvidia-beta (AUR)
```

### Container GPU Support
```bash
# Setup Docker GPU support
ghostctl nvidia container

# Example configuration:
# 1. Install nvidia-container-toolkit
# 2. Configure Docker daemon
# 3. Test with: docker run --gpus all nvidia/cuda:latest nvidia-smi
# 4. Setup Podman GPU support
```

### GPU Passthrough for VMs
```bash
# Configure GPU passthrough
ghostctl nvidia passthrough

# Complete setup includes:
# 1. Enable IOMMU in GRUB/systemd-boot
# 2. Configure VFIO modules
# 3. Create VM XML with GPU
# 4. Setup Looking Glass for display
# 5. Configure USB passthrough
```

### Example GPU Passthrough VM
```xml
<!-- VM Configuration Example -->
<hostdev mode='subsystem' type='pci' managed='yes'>
  <source>
    <address domain='0x0000' bus='0x01' slot='0x00' function='0x0'/>
  </source>
  <address type='pci' domain='0x0000' bus='0x05' slot='0x00' function='0x0'/>
</hostdev>
```

## 📝 Development Environment

### Mason.nvim Integration
```bash
# Setup complete development environment
ghostctl nvim mason

# Language-specific environments:
# 1. Rust: rust-analyzer, rustfmt, taplo
# 2. Python: pyright, black, isort, pylint, debugpy
# 3. Go: gopls, gofmt, goimports, delve
# 4. Zig: zls
# 5. Web: typescript-language-server, prettier, eslint_d
```

### Development Workflow Examples

#### Rust Development Setup
```bash
# Complete Rust environment
ghostctl dev rust
ghostctl nvim mason  # Then select Rust environment

# Results in:
# - rust-analyzer for LSP
# - rustfmt for formatting
# - taplo for TOML files
# - Cargo.toml syntax highlighting
# - Integrated debugging
```

#### Python Development Setup
```bash
# Python environment with virtual environments
ghostctl dev python
ghostctl nvim mason  # Select Python environment

# Configured tools:
# - pyright for type checking
# - black for formatting
# - isort for import sorting
# - pylint for linting
# - debugpy for debugging
```

## 💻 Terminal Ecosystem

### Alacritty Complete Setup
```bash
# Full Alacritty configuration
ghostctl terminal menu  # Select Alacritty

# Features configured:
# - Theme selection (Tokyo Night, Dracula, Nord, etc.)
# - Font management (Nerd Fonts)
# - Performance optimization
# - Key bindings
# - GPU acceleration
```

### Alacritty Configuration Example
```yaml
# ~/.config/alacritty/alacritty.yml (Generated by GhostCTL)
window:
  opacity: 0.9
  padding: { x: 10, y: 10 }

font:
  normal: { family: "FiraCode Nerd Font" }
  size: 12.0

colors:
  primary:
    background: '#1a1b26'  # Tokyo Night
    foreground: '#c0caf5'

key_bindings:
  - { key: V, mods: Control|Shift, action: Paste }
  - { key: C, mods: Control|Shift, action: Copy }
```

### Ghostty Setup
```bash
# Modern terminal with GPU acceleration
ghostctl terminal ghostty

# Installation methods:
# 1. AUR package (recommended)
# 2. Build from source (Zig required)
# 3. AppImage (when available)

# Configuration includes:
# - Catppuccin theme
# - FiraCode Nerd Font
# - Shell integration
# - Performance optimization
```

## 🔐 Security & Key Management

### SSH Key Management
```bash
# Comprehensive SSH setup
ghostctl security ssh

# Features:
# - Generate secure SSH keys (Ed25519/RSA)
# - Configure SSH client/server
# - Manage authorized_keys
# - Setup SSH agent
# - Configure host-based authentication
```

### SSH Key Generation Example
```bash
# Generated by GhostCTL
ssh-keygen -t ed25519 -C "user@hostname-$(date +%Y%m%d)" -f ~/.ssh/id_ed25519

# Automatically configures:
# - ~/.ssh/config with host entries
# - SSH agent auto-start
# - Secure permissions (600/700)
```

### GPG Key Management
```bash
# GPG key lifecycle management
ghostctl security gpg

# Capabilities:
# - Generate GPG keys (RSA 4096/ECC)
# - Import/export keys
# - Configure Git signing
# - Setup email encryption
# - Key backup and recovery
```

## 💾 Backup & Recovery

### Automated Backup Setup
```bash
# Complete backup solution
ghostctl backup setup

# Configures:
# - Restic repository
# - Backup paths selection
# - Encryption keys
# - Systemd timers
# - Verification schedules
```

### Backup Workflow Example
```bash
# 1. Initialize repository
ghostctl backup setup
# Creates: /backup/restic-repo with encryption

# 2. Schedule daily backups
ghostctl backup schedule
# Creates: systemd timer for 2 AM daily

# 3. Verify integrity weekly
ghostctl backup verify
# Checks: repository consistency and file integrity

# 4. Cleanup old backups monthly
ghostctl backup cleanup
# Removes: backups older than retention policy
```

### Backup Configuration Example
```toml
# ~/.config/ghostctl/backup.toml
[repository]
path = "/backup/restic-repo"
password_file = "~/.config/ghostctl/backup-key"

[paths]
include = [
    "~/Documents",
    "~/Projects", 
    "~/.config",
    "/etc"
]
exclude = [
    "**/.git",
    "**/node_modules",
    "**/target"
]

[schedule]
frequency = "daily"
time = "02:00"
retention = "30 days"
```

### System Recovery
```bash
# Emergency recovery options
ghostctl restore menu

# Recovery methods:
# 1. Restic restore from backup
# 2. Btrfs snapshot rollback
# 3. Chroot rescue environment
```

## 🗃️ Filesystem Management

### Btrfs Snapshot Management
```bash
# Complete snapshot lifecycle
ghostctl btrfs menu

# Snapshot operations:
# - Create named snapshots
# - List with metadata
# - Restore to any location
# - Delete with confirmation
```

### Btrfs Workflow Example
```bash
# 1. Create pre-upgrade snapshot
ghostctl btrfs snapshot
# Input: "pre-kernel-update-2024-12-21"
# Creates: /.snapshots/pre-kernel-update-2024-12-21

# 2. Perform system upgrade
sudo pacman -Syu

# 3. If issues occur, rollback
ghostctl btrfs restore
# Selects: pre-kernel-update-2024-12-21
# Restores: to / with confirmation
```

## 🌐 Network & Cloud Integration

### Network Diagnostics
```bash
# Comprehensive network testing
ghostctl network status

# Information provided:
# - Network interfaces and status
# - Connectivity tests (ping, DNS)
# - Tailscale mesh status
# - Open ports and services
# - Network performance metrics
```

### Cloud Provider Integration
```bash
# Multi-cloud tool management
ghostctl cloud menu

# Supported providers:
# - AWS: aws-cli, eksctl, sam-cli
# - Azure: azure-cli, bicep
# - GCP: gcloud, kubectl, terraform
```

### Cloud Setup Example
```bash
# AWS development environment
ghostctl cloud aws

# Installs and configures:
# - AWS CLI v2
# - Configure profiles
# - Setup MFA
# - Install eksctl for Kubernetes
# - Configure terraform AWS provider
```

## ⚙️ System Services Management

### Systemd Service Control
```bash
# Service lifecycle management
ghostctl systemd menu

# Operations:
# - Enable/disable services
# - Start/stop/restart
# - View status and logs
# - Create custom services
```

### Service Management Example
```bash
# Enable and start Docker
ghostctl systemd enable
# Prompts: service selection (docker)
# Executes: systemctl enable --now docker

# Check service status
ghostctl systemd status
# Shows: Active services, failed services, recent logs
```

## 🏠 Homelab Integration

### Complete Homelab Setup
```bash
# Initialize homelab environment
ghostctl homelab init

# Creates directory structure:
# /data/homelab/
# ├── config/      # Service configurations  
# ├── data/        # Persistent data
# ├── backups/     # Backup storage
# ├── scripts/     # Automation scripts
# └── compose/     # Docker Compose files
```

### Media Server Deployment
```bash
# Deploy complete media stack
ghostctl homelab media-server

# Includes:
# - Plex/Jellyfin media server
# - Sonarr/Radarr content management
# - qBittorrent download client
# - Reverse proxy (Nginx/Traefik)
# - SSL certificates (Let's Encrypt)
```

### Monitoring Stack
```bash
# Deploy monitoring infrastructure
ghostctl homelab monitoring

# Components:
# - Prometheus metrics collection
# - Grafana dashboards
# - Node Exporter system metrics
# - Alertmanager notifications
# - Loki log aggregation
```

## 🔧 Advanced Configuration

### Configuration Management
```toml
# ~/.config/ghostctl/config.toml
[general]
editor = "nvim"
shell = "/bin/zsh"
theme = "dark"

[arch]
aur_helper = "reaper"
parallel_builds = 4
use_ccache = true

[nvidia]
driver_type = "open"
container_runtime = true
passthrough_enabled = true

[backup]
default_repo = "/backup/restic"
schedule = "daily"
retention = "30d"

[development]
default_lsp = "rust-analyzer"
format_on_save = true
auto_imports = true
```

### Environment Variables
```bash
# ~/.config/ghostctl/env
export GHOSTCTL_CONFIG_DIR="$HOME/.config/ghostctl"
export GHOSTCTL_DATA_DIR="$HOME/.local/share/ghostctl"
export GHOSTCTL_CACHE_DIR="$HOME/.cache/ghostctl"
export GHOSTCTL_LOG_LEVEL="info"
export GHOSTCTL_THEME="tokyo-night"
```

## 🔍 Troubleshooting

### Common Issues

#### Permission Errors
```bash
# Fix file permissions
sudo chown -R $USER:$USER ~/.config/ghostctl
chmod 700 ~/.config/ghostctl
chmod 600 ~/.config/ghostctl/config.toml
```

#### Package Conflicts
```bash
# Check for conflicts
ghostctl arch health
# Shows: Conflicting packages, orphaned dependencies

# Fix AUR issues
ghostctl arch pkgfix
# Cleans: PKGBUILD cache, resets makepkg
```

#### NVIDIA Issues
```bash
# Diagnose NVIDIA problems
ghostctl nvidia status
# Shows: Driver version, kernel modules, DKMS status

# Fix common issues
ghostctl nvidia fix
# Resolves: Module loading, X configuration, Wayland compatibility
```

### Diagnostic Commands
```bash
# System health check
ghostctl arch health

# Development environment check
ghostctl nvim mason  # Then select diagnosis

# Network connectivity test
ghostctl network test

# Security audit
ghostctl security audit
```

### Log Analysis
```bash
# View GhostCTL logs
tail -f ~/.local/share/ghostctl/history.log

# System logs
journalctl -u ghostctl-backup.timer  # Backup logs
journalctl -u docker                 # Container logs
dmesg | grep nvidia                  # GPU logs
```

## 🎯 Best Practices

### System Maintenance Routine
```bash
# Weekly maintenance script
#!/bin/bash
ghostctl arch full              # Complete system maintenance
ghostctl backup verify          # Verify backup integrity
ghostctl security audit         # Security check
ghostctl nvidia optimize        # GPU optimization
```

### Development Workflow
```bash
# Daily development setup
ghostctl dev rust               # Rust environment
ghostctl nvim mason            # LSP servers updated
ghostctl terminal alacritty    # Terminal optimized
```

### Backup Strategy
```bash
# 3-2-1 Backup strategy
# 3 copies: Original + 2 backups
# 2 different media: Local + Cloud
# 1 offsite: Cloud storage

ghostctl backup setup           # Local backup
# Configure: Restic + cloud storage (S3/B2)
```

## 🚀 Performance Optimization

### System Performance
```bash
# Complete performance tuning
ghostctl arch perf

# Optimizations include:
# - CPU governor settings
# - I/O schedulers
# - Kernel parameters
# - Memory management
# - Network stack tuning
```

### Development Performance
```bash
# Development optimization
ghostctl dev menu

# Optimizations:
# - Rust: sccache, lld linker
# - Python: PyPy, conda environments
# - Node.js: pnpm, esbuild
# - General: ccache, parallel builds
```

## 📱 Integration Examples

### VS Code Integration
```json
// settings.json
{
  "terminal.integrated.shell.linux": "/usr/bin/alacritty",
  "rust-analyzer.server.path": "~/.local/share/nvim/mason/bin/rust-analyzer",
  "python.defaultInterpreterPath": "~/.local/share/ghostctl/python/bin/python"
}
```

### GitHub Actions Integration
```yaml
# .github/workflows/ci.yml
- name: Setup development environment
  run: |
    ghostctl dev rust
    ghostctl nvim mason
```

### Docker Integration
```dockerfile
# Dockerfile.dev
FROM archlinux:latest
RUN pacman -Sy --noconfirm ghostctl
RUN ghostctl dev rust --non-interactive
```

## 🔗 External Tools Integration

### Git Integration
```bash
# Git hooks integration
ghostctl security gpg           # Setup GPG signing
git config --global commit.gpgsign true
git config --global user.signingkey "YOUR_KEY_ID"
```

### Shell Integration
```bash
# Zsh integration
echo 'eval "$(ghostctl completion zsh)"' >> ~/.zshrc

# Bash integration  
echo 'eval "$(ghostctl completion bash)"' >> ~/.bashrc
```

---

## 📚 Additional Resources

- **Command Reference**: See [COMMANDS.md](COMMANDS.md) for complete command list
- **Quick Start**: See [README.md](README.md) for installation and overview
- **Changelog**: See [CHANGELOG.md](CHANGELOG.md) for version history
- **GitHub**: [https://github.com/ghostkellz/ghostctl](https://github.com/ghostkellz/ghostctl)

---

## 🤝 Contributing

### Development Setup
```bash
# Clone and setup development environment
git clone https://github.com/ghostkellz/ghostctl
cd ghostctl
ghostctl dev rust
cargo build
```

### Adding New Features
1. Create module in `src/`
2. Add CLI commands in `src/cli.rs`
3. Update documentation
4. Add tests
5. Submit PR

---

**License**: MIT © 2024 CK Technology LLC

For support, issues, or feature requests, visit our [GitHub repository](https://github.com/ghostkellz/ghostctl).