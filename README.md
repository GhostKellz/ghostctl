<div align="center">
  <img src="assets/ghostctl-logo.png" alt="GhostCTL Logo" width="200" height="200" />

  # 🚀 GhostCTL - Professional System Administration Toolkit

  **The Ultimate Linux Management Suite for Power Users, DevOps Engineers & Homelabbers**
</div>


[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-linux-lightgrey.svg)](https://www.kernel.org)
[![Build Status](https://github.com/GhostKellz/ghostctl/actions/workflows/ci.yml/badge.svg)](https://github.com/GhostKellz/ghostctl/actions/workflows/ci.yml)
[![Arch Linux](https://img.shields.io/badge/Arch--Linux-blue?logo=arch-linux&logoColor=white)](https://archlinux.org)
[![Btrfs](https://img.shields.io/badge/Btrfs--supported-blueviolet?logo=linux)](https://btrfs.readthedocs.io)
[![Rust](https://img.shields.io/badge/Rust--Toolchain-orange?logo=rust)](https://www.rust-lang.org/)
[![NVIDIA](https://img.shields.io/badge/NVIDIA--supported-green?logo=nvidia)](https://developer.nvidia.com/)
[![Vim](https://img.shields.io/badge/Vim--supported-darkgreen?logo=vim)](https://www.vim.org/)
[![Zsh](https://img.shields.io/badge/Zsh--supported-black?logo=gnu-bash)](https://www.zsh.org/)
[![Proxmox](https://img.shields.io/badge/Proxmox--helpers-orange?logo=proxmox)](https://www.proxmox.com/)
[![Docker](https://img.shields.io/badge/Docker--supported-blue?logo=docker)](https://www.docker.com/)



GhostCTL is a comprehensive system administration platform that transforms complex Linux operations into intuitive, interactive workflows. Built in Rust for performance and reliability, it provides enterprise-grade tools through a user-friendly interface.

## 🚀 Quick Install

```bash
# One-line installation for all Linux distributions
curl -sSL https://ghostctl.cktech.sh | bash
```

**Supports:** Arch Linux, Ubuntu/Debian, Fedora/RHEL, openSUSE, Alpine, macOS
**Auto-detects:** Your OS and installs via package manager, binary, or source build

[📋 **Detailed Installation Guide**](INSTALL.md) • [🎯 **All Installation Options**](INSTALL.md#installation-options)

## 🎯 Why GhostCTL?

- **🔧 All-in-One Solution**: Replace dozens of tools with one comprehensive platform
- **⚡ Performance**: Rust-powered for speed and memory efficiency  
- **🎨 User Experience**: Interactive menus replace complex command combinations
- **🏢 Enterprise-Ready**: Production-tested with professional-grade features
- **🔄 Automation**: Reduce manual work with intelligent workflows
- **📚 Learning-Friendly**: Built-in help and guided operations

## ✨ Core Features

### 🏗️ **Infrastructure as Code**
- **Ansible Management**: Complete playbook lifecycle, inventory management, and execution
- **Terraform Integration**: Plan/Apply/Destroy workflows with state management  
- **Multi-Cloud Support**: AWS, Azure, GCP, DigitalOcean, Hetzner, Linode
- **CI/CD Integration**: Pipeline templates and automated deployments

### 🔐 **Security & Key Management**
- **SSH Key Management**: Generation, deployment, GitHub/GitLab integration, security auditing
- **GPG Key Operations**: Full lifecycle management, encryption, signing, keyserver sync
- **SSL Certificate Management**: GhostCert integration for automated certificate handling
- **Security Auditing**: Comprehensive system security assessment and recommendations

### 💾 **Data Protection & Backups** (Enhanced in v0.8.0)
- **Btrfs Integration**: Snapshot management, subvolume operations, filesystem optimization
- **Snapper Automation**: Automated snapshot creation, cleanup, and rollback capabilities
- **Restic CLI Tools**: Interactive restic backup management with repository initialization, snapshot browsing, restoration workflows, and integrity checking
- **Automated Workflows**: Custom backup scripts with systemd timer integration

### ☁️ **Object Storage & S3 Management** (New in v1.0.0)
- **MinIO Cluster Management**: Distributed cluster setup, node management, and health monitoring
- **Erasure Code Configuration**: Automated EC setup with performance vs storage optimization
- **Performance Tuning**: System-level optimization for storage, network, and memory usage
- **Multi-Tenant Setup**: User management, policy configuration, and access control
- **Backup & Replication**: Cross-cluster replication and disaster recovery planning
- **S3 Compatible Operations**: Bucket management, file operations, and AWS CLI integration

### 🐳 **DevOps & Container Management** (Enhanced in v1.0.0)
- **Docker Registry Mirror Setup**: Local registry deployment with corporate proxy support and authentication
- **Container Cleanup Tools**: Automated cleanup for images, volumes, networks, and containers with safety checks
- **Docker Registry**: Private registry management (`docker.cktechx.io` integration)
- **Container Orchestration**: Docker Compose, Swarm, and deployment automation
- **GitHub Templates**: Direct deployment from repository templates
- **Environment Management**: Multi-environment project isolation

### 🏥 **Proxmox VE Management** (Major v1.0.0 Update)
- **Template Management**: Complete lifecycle management for LXC containers, VM ISOs, and appliance templates with upload/download/customization capabilities
- **Storage Migration**: VM/container storage migration tools with bulk operations and storage pool management
- **Backup Rotation & Pruning**: Automated backup job management, retention policies, verification, and pruning with comprehensive analytics
- **Firewall Automation**: Advanced firewall rule management with security profiles, network scanning (gscan integration), and automated policy enforcement
- **Enhanced Script Categories**: Container templates, VMs, system administration, monitoring tools, and development environments
- **Cluster Management**: Join/leave cluster operations, node management, and cluster status monitoring
- **Bulk Operations**: Mass VM/container start/stop/restart with confirmation prompts
- **System Administration**: Post-install setup, backup management, resource usage reports, and network configuration
- **Community Scripts Integration**: Access to 40+ categorized Proxmox helper scripts with preview and execution

### 🛠️ **System Administration** (Enhanced in v0.8.0)
- **Arch Linux Optimization**: Package management, AUR helpers with persistent preferences, system fixes
- **AUR Helper Management**: Preference system for reaper/paru/yay with automatic detection and installation
- **Service Management**: Systemd operations, log analysis, performance monitoring
- **Network Diagnostics**: Enhanced DNS tools with DNSSEC verification, interactive network scanning, and comprehensive troubleshooting

### 💻 **Development Environment**
- **Neovim Management**: Health checks, plugin management, LSP configuration
- **Shell Enhancement**: ZSH, Oh My Zsh, Powerlevel10k with automated setup
- **Terminal Optimization**: Tmux, screen, and terminal multiplexer management
- **Git Integration**: Repository management and workflow automation

## 🚀 Quick Start

### Installation

#### Arch Linux (AUR)
```bash
yay -S ghostctl
```

#### Universal Installer (All Distributions) - **Recommended**
```bash
curl -sSL https://raw.githubusercontent.com/ghostkellz/ghostctl/main/install/install.sh | bash
```

The universal installer supports:
- **Linux**: x86_64 and aarch64 (GNU and musl)
- **Automatic OS Detection**: Arch, Debian/Ubuntu, RHEL/Fedora
- **Fallback Building**: Builds from source if binaries aren't available
- **Custom Install Locations**: Use `--install-dir` for custom paths

#### Manual Installation
```bash
# Clone repository
git clone https://github.com/ghostkellz/ghostctl.git
cd ghostctl/ghostctl

# Build from source
cargo build --release

# Install
sudo install target/release/ghostctl /usr/local/bin/
```

### First Run
```bash
# Interactive main menu
ghostctl

# Show version information
ghostctl version

# View all available commands
ghostctl --help

# Quick access to specific tools
ghostctl dev menu               # Development environment
ghostctl docker menu          # Docker management
ghostctl --dev                # Development menu flag
```

## 📋 Command Overview

### Core Commands
```bash
# Main interactive menu
ghostctl                        # Launch main menu
ghostctl version               # Show version information

# Package & system management
ghostctl arch menu             # Arch Linux specific tools
ghostctl backup menu           # Backup management
ghostctl btrfs menu            # Btrfs operations

# DevOps & Infrastructure  
ghostctl docker menu           # Docker/container management
ghostctl proxmox menu          # Proxmox VE management (enhanced v1.0.0)
ghostctl cloud menu            # Cloud provider tools

# Storage Management
ghostctl storage s3            # MinIO/S3 cluster management
ghostctl storage local         # Local storage tools
ghostctl storage network       # Network storage (NFS/CIFS)

# Development & Configuration
ghostctl dev menu              # Development environment
ghostctl nvim menu             # Neovim management
ghostctl shell menu            # Shell & terminal setup

# Security & Network
ghostctl network menu          # Network diagnostics & tools
ghostctl security menu         # Security & key management
ghostctl nginx menu            # Nginx configuration
```

### Quick Access Flags
```bash
# Direct menu access
ghostctl --dev                 # Development environment menu
ghostctl --docker             # Docker management menu  
ghostctl --pve                 # Proxmox VE menu
ghostctl --system             # System management menu
```

### Quick Operations
```bash
# Version and help
ghostctl version              # Show detailed version info
ghostctl backup menu             # Backup management system
ghostctl restic menu             # Interactive restic CLI tools

# Infrastructure & Virtualization
ghostctl proxmox menu            # Enhanced Proxmox VE management (v1.0.0)
ghostctl pve templates           # PVE template management
ghostctl pve firewall            # PVE firewall automation with gscan
ghostctl pve storage             # PVE storage migration tools
ghostctl pve backup              # PVE backup rotation & pruning

# Object Storage & MinIO
ghostctl storage s3              # MinIO cluster management
ghostctl s3 cluster              # MinIO distributed setup
ghostctl s3 performance         # MinIO performance tuning

# Infrastructure as Code  
ghostctl infrastructure ansible  # Ansible management
ghostctl infrastructure terraform # Terraform operations

# Development & System
ghostctl arch aur               # AUR helper management
ghostctl nvim health-check       # Neovim diagnostics
ghostctl shell setup-zsh         # ZSH configuration
```

## 🏗️ Architecture

```
GhostCTL/
├── 🏠 Core System
│   ├── Interactive Menu System
│   ├── Configuration Management
│   └── Plugin Architecture
│
├── 🔧 System Administration
│   ├── Package Management
│   ├── Service Control
│   └── System Diagnostics
│
├── 💾 Data Management
│   ├── Btrfs Operations
│   ├── Snapshot Management
│   ├── Backup Automation
│   └── Object Storage (S3/MinIO)
│
├── 🐳 DevOps Platform
│   ├── Container Management
│   ├── Registry Operations
│   ├── Registry Mirroring
│   └── CI/CD Integration
│
├── 🏥 Virtualization Platform
│   ├── Proxmox VE Management
│   ├── Template Lifecycle
│   ├── Storage Migration
│   ├── Backup & Pruning
│   └── Firewall Automation
│
├── 🏗️ Infrastructure Tools
│   ├── Ansible Automation
│   ├── Terraform Management
│   └── Multi-Cloud Support
│
└── 🔐 Security Suite
    ├── Key Management
    ├── Certificate Handling
    └── Security Auditing
```

## 📚 Documentation

- **[Commands Reference](COMMANDS.md)** - Complete command documentation with v0.8.0 features
- **[User Guide](DOCS.md)** - Detailed usage instructions and examples  
- **[Change Log](CHANGELOG.md)** - Version history and feature updates
- **[Configuration](config.md)** - Setup and customization
- **[Troubleshooting](troubleshooting.md)** - Common issues and solutions

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup
```bash
# Clone and enter directory
git clone https://github.com/ghostkellz/ghostctl.git
cd ghostctl

# Install dependencies
cargo build

# Run tests
cargo test

# Install development version
cargo install --path ghostctl
```

## 📦 Package Information

### Dependencies
- **Core**: Rust 1.70+, OpenSSL
- **Optional**: Docker, Ansible, Terraform, Azure CLI, AWS CLI, PowerDNS
- **Recommended**: Snapper, Restic, Neovim, ZSH, gscan (for network scanning)

### Supported Distributions
- ✅ Arch Linux (native package)
- ✅ Ubuntu/Debian (deb package)
- ✅ Fedora/RHEL (rpm package)
- ✅ openSUSE (universal installer)
- ✅ Proxmox VE (specialized installer)

## 🛡️ Security

GhostCTL follows security best practices:

- 🔐 Secure key generation and management
- 🛡️ Permission validation and enforcement
- 🔍 Security auditing and recommendations
- 📊 Regular security assessments
- 🔒 Encrypted backup and communication

Report security issues to: security@ghostctl.dev

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Rust Community** - For the amazing ecosystem
- **Linux Community** - For inspiration and feedback
- **Contributors** - For making this project better
- **Users** - For trust and valuable feedback

## 📞 Support

- 🐛 **Issues**: [GitHub Issues](https://github.com/ghostkellz/ghostctl/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/ghostkellz/ghostctl/discussions)
- 📧 **Email**: support@ghostctl.io
- 📖 **Documentation**: [docs.ghostctl.dev](https://docs.ghostctl.dev)

---

**Made for for the Linux community**

*GhostCTL - Simplifying Linux administration, one command at a time.*

