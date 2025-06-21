# 🚀 GhostCTL - Professional System Administration Toolkit

**The Ultimate Linux Management Suite for Power Users, DevOps Engineers & Homelabbers**


[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-linux-lightgrey.svg)](https://www.kernel.org)
[![Build Status](https://github.com/GhostKellz/ghostctl/actions/workflows/main.yml/badge.svg)](https://github.com/GhostKellz/ghostctl/actions/workflows/main.yml)
[![Arch Linux](https://img.shields.io/badge/Arch--Linux-blue?logo=arch-linux&logoColor=white)](https://archlinux.org)
[![Btrfs](https://img.shields.io/badge/Btrfs--supported-blueviolet?logo=linux)](https://btrfs.readthedocs.io)
[![Rust](https://img.shields.io/badge/Rust--Toolchain-orange?logo=rust)](https://www.rust-lang.org/)
[![NVIDIA](https://img.shields.io/badge/NVIDIA--supported-green?logo=nvidia)](https://developer.nvidia.com/)
[![Vim](https://img.shields.io/badge/Vim--supported-darkgreen?logo=vim)](https://www.vim.org/)
[![Zsh](https://img.shields.io/badge/Zsh--supported-black?logo=gnu-bash)](https://www.zsh.org/)
[![Proxmox](https://img.shields.io/badge/Proxmox--helpers-orange?logo=proxmox)](https://www.proxmox.com/)
[![Docker](https://img.shields.io/badge/Docker--supported-blue?logo=docker)](https://www.docker.com/)



GhostCTL is a comprehensive system administration platform that transforms complex Linux operations into intuitive, interactive workflows. Built in Rust for performance and reliability, it provides enterprise-grade tools through a user-friendly interface.

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

### 💾 **Data Protection & Backups**
- **Btrfs Integration**: Snapshot management, subvolume operations, filesystem optimization
- **Snapper Automation**: Automated snapshot creation, cleanup, and rollback capabilities
- **Restic Backups**: Multi-destination backups with encryption and deduplication
- **Automated Workflows**: Custom backup scripts with systemd timer integration

### 🐳 **DevOps & Container Management**
- **Docker Registry**: Private registry management (`docker.cktechx.io` integration)
- **Container Orchestration**: Docker Compose, Swarm, and deployment automation
- **GitHub Templates**: Direct deployment from repository templates
- **Environment Management**: Multi-environment project isolation

### 🏥 **Proxmox VE Management**
- **SDN Configuration**: Software Defined Networking with zone and VNet management
- **Advanced Firewall**: Security groups, DDoS protection, intrusion detection
- **VM/Container Lifecycle**: Complete virtualization management
- **Cluster Operations**: Multi-node Proxmox cluster administration

### 🛠️ **System Administration**
- **Arch Linux Optimization**: Package management, AUR helpers, system fixes
- **Package Management**: Multi-distro package operations and conflict resolution
- **Service Management**: Systemd operations, log analysis, performance monitoring
- **Network Diagnostics**: Comprehensive network troubleshooting and optimization

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
ghostctl proxmox menu          # Proxmox VE management
ghostctl cloud menu            # Cloud provider tools

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
ghostctl backup quick-backup     # Immediate backup
ghostctl devops docker-status    # Docker overview

# Infrastructure
ghostctl infrastructure ansible  # Ansible management
ghostctl infrastructure terraform # Terraform operations
ghostctl infrastructure cloud    # Multi-cloud tools

# Development
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
│   └── Backup Automation
│
├── 🐳 DevOps Platform
│   ├── Container Management
│   ├── Registry Operations
│   └── CI/CD Integration
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

- **[Commands Reference](commands.md)** - Complete command documentation
- **[User Guide](docs.md)** - Detailed usage instructions
- **[Configuration](config.md)** - Setup and customization
- **[API Documentation](api.md)** - Plugin development
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
- **Optional**: Docker, Ansible, Terraform, Azure CLI, AWS CLI
- **Recommended**: Snapper, Restic, Neovim, ZSH

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

