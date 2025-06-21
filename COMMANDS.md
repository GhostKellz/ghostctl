# 📋 GhostCTL Commands Reference v0.7.0

Complete command documentation for GhostCTL v0.7.0 - The ultimate system and homelab management tool.

## 🚀 Core Commands

### Basic Usage
```bash
# Launch interactive menu
ghostctl

# Show version and help
ghostctl version                   # Show version information
ghostctl help                     # Show help information
```

## 🏠 System Management

### System Commands
```bash
ghostctl system update            # Update system packages
ghostctl system status            # Show system status  
ghostctl system arch              # Arch Linux management
ghostctl system nixos             # NixOS management
```

### 🐧 Arch Linux Management
```bash
# Arch system maintenance and optimization
ghostctl arch menu                # Interactive Arch menu
ghostctl arch fix                 # Fix common Arch issues (pacman, keyring, mirrors)
ghostctl arch optimize            # Optimize system performance (zram/zswap)
ghostctl arch mirrors             # Optimize mirror list with reflector
ghostctl arch orphans             # Remove orphaned packages
ghostctl arch pkgfix              # Clean PKGBUILD/build environment
ghostctl arch keyring             # Refresh Arch keyring
ghostctl arch full                # Run full system maintenance

# New v0.7.0 features
ghostctl arch health              # System health and maintenance
ghostctl arch swap                # Swap and zram management
ghostctl arch dotfiles            # Dotfiles management
ghostctl arch aur                 # AUR helper management
ghostctl arch boot                # Boot and kernel management
ghostctl arch perf                # Performance tuning
```

### 🎯 NVIDIA Management
```bash
# NVIDIA driver and GPU management
ghostctl nvidia menu              # NVIDIA management menu
ghostctl nvidia status            # Check driver status
ghostctl nvidia drivers           # Driver management (proprietary/open/open-beta)
ghostctl nvidia container         # Container GPU support setup
ghostctl nvidia passthrough       # GPU passthrough setup for VMs
ghostctl nvidia fix               # Fix NVIDIA issues
ghostctl nvidia optimize          # Optimize performance
ghostctl nvidia info              # Show GPU information
```

## 🛠️ Development Environment

### Development Commands
```bash
ghostctl dev menu                 # Development environment menu
ghostctl dev rust                 # Rust development setup
ghostctl dev zig                  # Zig development setup
ghostctl dev go                   # Go development setup
ghostctl dev python               # Python development setup
```

### 👻 Ghost Tools Ecosystem
```bash
ghostctl ghost menu               # Ghost tools management menu
ghostctl ghost install-all        # Install all Ghost tools
ghostctl ghost reaper             # Install Reaper AUR helper
ghostctl ghost oxygen             # Install Oxygen Rust tool
ghostctl ghost zion               # Install Zion Zig tool
ghostctl ghost status             # Check Ghost tools status
```

## 📝 Neovim & Editor Management

### Neovim Setup
```bash
ghostctl nvim menu                # Neovim management menu
ghostctl nvim install             # Install Neovim distribution
ghostctl nvim lazyvim             # Install LazyVim configuration
```

### 🔨 Mason.nvim Integration (New in v0.7.0)
```bash
# Mason.nvim LSP/DAP/Tool management
ghostctl nvim mason               # Access Mason management menu

# Through Mason menu:
# - Check Mason status and health
# - Setup essential language servers
# - Language-specific environments (Rust, Python, Go, Zig, Web, DevOps)
# - Install/update specific tools
# - Diagnose Mason issues
```

## 💻 Terminal Configuration

### Terminal Setup
```bash
ghostctl terminal menu            # Terminal configuration menu
ghostctl terminal ghostty         # Setup Ghostty terminal
ghostctl terminal starship        # Install Starship prompt
```

### 🔷 Advanced Terminal Support (New in v0.7.0)
```bash
# Enhanced terminal ecosystem
# - Ghostty with full configuration
# - WezTerm support
# - Alacritty complete setup and theming
# - Nerd Font management
# - Terminal performance optimization
```

## 🐳 Container & DevOps

### Docker Management
```bash
ghostctl docker menu              # Docker management menu
ghostctl docker install           # Install Docker
ghostctl docker status            # Docker status
ghostctl docker homelab           # Homelab container stacks
```

### 🏠 Proxmox VE Management
```bash
ghostctl pve menu                 # PVE management menu
ghostctl pve status               # Show PVE status
ghostctl pve vm list              # List VMs
ghostctl pve vm create            # Create VM wizard
ghostctl pve vm start <id>        # Start VM
ghostctl pve vm stop <id>         # Stop VM
ghostctl pve ct list              # List containers
ghostctl pve ct create            # Create container
ghostctl pve ct start <id>        # Start container
ghostctl pve ct stop <id>         # Stop container
```

## 🔐 Security & Key Management (New in v0.7.0)

### Security Commands
```bash
ghostctl security menu            # Security management menu
ghostctl security ssh             # SSH key management
ghostctl security gpg             # GPG key management  
ghostctl security audit           # Security audit
```

## 💾 Backup & Recovery (New in v0.7.0)

### Backup Management
```bash
ghostctl backup menu              # Backup management menu
ghostctl backup setup             # Setup backup system
ghostctl backup schedule          # Schedule automated backups
ghostctl backup verify            # Verify backup integrity
ghostctl backup cleanup           # Clean old backups
```

### System Recovery
```bash
ghostctl restore menu             # System recovery menu
ghostctl restore restic           # Restore from Restic backup
ghostctl restore btrfs            # Rollback Btrfs snapshot
ghostctl restore chroot           # Enter recovery chroot
```

## 🗃️ Filesystem Management (New in v0.7.0)

### Btrfs Operations
```bash
ghostctl btrfs menu               # Btrfs management menu
ghostctl btrfs snapshot           # Create snapshot
ghostctl btrfs list               # List snapshots
ghostctl btrfs delete             # Delete snapshot
ghostctl btrfs restore            # Restore snapshot
```

## 🌐 Network & Infrastructure (New in v0.7.0)

### Network Management
```bash
ghostctl network menu             # Network tools menu
ghostctl network status           # Network status
ghostctl network test             # Network connectivity test
ghostctl network config           # Network configuration
```

### ☁️ Cloud Provider Management
```bash
ghostctl cloud menu               # Cloud management menu
ghostctl cloud aws                # AWS tools
ghostctl cloud azure              # Azure tools
ghostctl cloud gcp                # Google Cloud tools
```

## ⚙️ System Services (New in v0.7.0)

### Systemd Management
```bash
ghostctl systemd menu             # Systemd management menu
ghostctl systemd status           # Service status
ghostctl systemd enable           # Enable service
ghostctl systemd disable          # Disable service
ghostctl systemd restart          # Restart service
```

## 🔐 SSL Certificate Management

### SSL Operations
```bash
ghostctl ssl menu                 # SSL management menu
ghostctl ssl install              # Install acme.sh
ghostctl ssl issue <domain>       # Issue certificate
ghostctl ssl renew                # Renew certificates
ghostctl ssl list                 # List certificates
```

## 🌐 Web Server Management

### Nginx Operations
```bash
ghostctl nginx menu               # Nginx management menu
ghostctl nginx status             # Nginx status
ghostctl nginx restart            # Restart Nginx
ghostctl nginx ssl-setup <domain> # Setup SSL for domain
```

## 📋 Script Management

### Script Operations
```bash
ghostctl scripts menu             # Scripts management menu
ghostctl scripts local            # Local script management
ghostctl scripts run <script>     # Run specific script
ghostctl scripts list <category>  # List scripts by category
```

## 🏠 Homelab Integration

### Homelab Commands
```bash
ghostctl homelab menu             # Homelab management menu
ghostctl homelab init             # Initialize homelab environment
ghostctl homelab media-server     # Deploy media server
ghostctl homelab monitoring       # Setup monitoring stack
```

## 🔧 External Tools

### Tool Management
```bash
ghostctl tools menu               # External tools menu
ghostctl tools external           # External tools management
ghostctl tools acme               # acme.sh SSL management
```

---

## 🆕 What's New in v0.7.0

### 🎯 Enhanced System Management
- **Arch Linux Enhancements**: Complete system maintenance with dotfiles management, disk space checking, package database rebuild, swap/zram configuration
- **NVIDIA Complete Suite**: Driver management (proprietary/open/open-beta), container runtime setup, GPU passthrough for VMs
- **Health Monitoring**: System health checks, performance optimization, maintenance automation

### 🛠️ Development Environment Improvements  
- **Mason.nvim Integration**: Zero-config Neovim development environments for multiple languages
- **Terminal Ecosystem**: Full Alacritty support, enhanced Ghostty/WezTerm configurations
- **Language Support**: Comprehensive Rust, Python, Go, Zig development environments

### 🔐 Security & Infrastructure
- **Security Management**: SSH/GPG key management, security auditing
- **Backup & Recovery**: Automated backup systems, integrity verification, system recovery
- **Network Tools**: Network diagnostics, configuration management

### 📁 Filesystem & Storage
- **Btrfs Management**: Snapshot creation, restoration, management
- **Storage Optimization**: Disk space monitoring, cleanup automation

### ☁️ Cloud & DevOps
- **Cloud Provider Support**: AWS, Azure, GCP tool integration
- **Container Ecosystem**: Enhanced Docker management, GPU container support
- **Infrastructure as Code**: Network configuration, service management

---

## 💡 Command Examples

### Quick System Maintenance
```bash
# Complete Arch system maintenance
ghostctl arch full

# Check system health
ghostctl arch health

# Optimize system performance  
ghostctl arch optimize
```

### Development Setup
```bash
# Setup complete Rust environment
ghostctl dev rust
ghostctl nvim mason  # Then select Rust environment

# Setup GPU development
ghostctl nvidia container
```

### Security & Backup
```bash
# Setup security
ghostctl security audit
ghostctl security ssh

# Setup automated backups
ghostctl backup setup
ghostctl backup schedule
```

### Homelab Deployment
```bash
# Initialize homelab
ghostctl homelab init
ghostctl homelab monitoring
ghostctl homelab media-server
```

---

## 📚 Documentation Structure

- **COMMANDS.md** (this file) - Complete command reference
- **DOCS.md** - Usage guides and examples
- **README.md** - Overview and quick start
- **Individual module docs** - Detailed feature documentation

---

## ⚡ Performance & Optimization

### System Optimization Commands
```bash
# Memory management
ghostctl arch swap                # Configure swap/zram optimal settings

# Performance tuning  
ghostctl arch perf               # System performance optimization
ghostctl nvidia optimize         # GPU performance tuning

# Maintenance automation
ghostctl arch full               # Complete system maintenance
ghostctl backup verify           # Backup integrity verification
```

### Development Optimization
```bash
# Language servers and tools
ghostctl nvim mason              # Install/update development tools

# Container performance
ghostctl nvidia container        # GPU acceleration for containers
ghostctl docker homelab         # Optimized homelab stacks
```

---

## 🛠️ Advanced Usage

### Automation & Scripting
```bash
# Backup automation
ghostctl backup schedule         # Setup automated backups
ghostctl scripts run backup     # Run backup scripts

# System maintenance  
ghostctl arch full              # Automated system maintenance
ghostctl security audit         # Security scanning
```

### Infrastructure Management
```bash
# Network configuration
ghostctl network config         # Network setup and optimization
ghostctl cloud aws              # Cloud infrastructure tools

# Service management
ghostctl systemd menu           # System service management
ghostctl nginx ssl-setup        # Automated SSL setup
```

---

For detailed usage examples and guides, see [DOCS.md](DOCS.md).
For quick start and overview, see [README.md](README.md).