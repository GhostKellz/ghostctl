<p align="center">
  <img src="assets/ghostctl-logo.png" alt="GhostCTL Logo" width="200" height="200" />
</p>

<h1 align="center">GhostCTL</h1>

<p align="center">
  <strong>Professional System Administration Toolkit</strong>
</p>

<p align="center">
  <strong>The Ultimate Linux Management Suite for Power Users, DevOps Engineers & Homelabbers</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-B7410E?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black" alt="Linux">
  <img src="https://img.shields.io/badge/Arch_Linux-1793D1?style=for-the-badge&logo=arch-linux&logoColor=white" alt="Arch Linux">
  <img src="https://img.shields.io/badge/Btrfs-8A2BE2?style=for-the-badge&logo=linux&logoColor=white" alt="Btrfs">
  <img src="https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white" alt="Docker">
  <img src="https://img.shields.io/badge/Proxmox-E57000?style=for-the-badge&logo=proxmox&logoColor=white" alt="Proxmox">
  <img src="https://img.shields.io/badge/NVIDIA-76B900?style=for-the-badge&logo=nvidia&logoColor=white" alt="NVIDIA">
  <img src="https://img.shields.io/badge/Vim-117A65?style=for-the-badge&logo=vim&logoColor=white" alt="Vim">
  <img src="https://img.shields.io/badge/Zsh-428850?style=for-the-badge&logo=gnu-bash&logoColor=white" alt="Zsh">
</p>



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
- **Restic CLI Tools**: Interactive restic backup management with repository initialization, snapshot browsing, restoration workflows, and integrity checking
- **Automated Workflows**: Custom backup scripts with systemd timer integration

### ☁️ **Object Storage & S3 Management**
- **MinIO Cluster Management**: Distributed cluster setup, node management, and health monitoring
- **Erasure Code Configuration**: Automated EC setup with performance vs storage optimization
- **Performance Tuning**: System-level optimization for storage, network, and memory usage
- **Multi-Tenant Setup**: User management, policy configuration, and access control
- **Backup & Replication**: Cross-cluster replication and disaster recovery planning
- **S3 Compatible Operations**: Bucket management, file operations, and AWS CLI integration

### 🐳 **DevOps & Container Management**
- **Docker Registry Mirror Setup**: Local registry deployment with corporate proxy support and authentication
- **Container Cleanup Tools**: Automated cleanup for images, volumes, networks, and containers with safety checks
- **Docker Registry**: Private registry management
- **Container Orchestration**: Docker Compose, Swarm, and deployment automation
- **GitHub Templates**: Direct deployment from repository templates
- **Environment Management**: Multi-environment project isolation

### 🏥 **Proxmox VE Management**
- **Template Management**: Complete lifecycle management for LXC containers, VM ISOs, and appliance templates
- **Storage Migration**: VM/container storage migration tools with bulk operations and storage pool management
- **Backup Rotation & Pruning**: Automated backup job management, retention policies, and verification
- **Firewall Automation**: Advanced firewall rule management with security profiles and network scanning
- **Cluster Management**: Join/leave cluster operations, node management, and cluster status monitoring
- **Bulk Operations**: Mass VM/container start/stop/restart with confirmation prompts
- **Community Scripts Integration**: Access to 40+ categorized Proxmox helper scripts

### 🛠️ **System Administration**
- **Arch Linux Optimization**: Package management, AUR helpers with persistent preferences, system fixes
- **AUR Helper Management**: Preference system for reaper/paru/yay with automatic detection and installation
- **Service Management**: Systemd operations, log analysis, performance monitoring
- **Network Diagnostics**: DNS tools with DNSSEC verification, network scanning, and troubleshooting

### 🔒 **UEFI & Virtualization**
- **Secure Boot Management**: Generate OVMF VARS with Microsoft keys for Windows 11 VMs
- **Key Enrollment**: Automated Secure Boot key enrollment via virt-fw-vars
- **VARS Verification**: Validate Secure Boot configuration in OVMF firmware files

### 💻 **Development Environment**
- **Neovim Management**: Health checks, plugin management, LSP configuration
- **Shell Enhancement**: ZSH, Oh My Zsh, Powerlevel10k with automated setup
- **Terminal Optimization**: Tmux, screen, and terminal multiplexer management
- **Git Integration**: Repository management and workflow automation

## 🚀 Quick Start

### Installation

```bash
curl -sSL https://ghostctl.cktech.sh | bash
```

#### Manual Installation
```bash
git clone https://github.com/ghostkellz/ghostctl.git
cd ghostctl/ghostctl
cargo build --release
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
ghostctl pve menu              # Proxmox VE management
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

### Direct Subcommands
```bash
# Jump directly to specific menus
ghostctl dev menu              # Development environment
ghostctl docker menu           # Docker management
ghostctl pve menu              # Proxmox VE management
ghostctl arch menu             # Arch Linux tools
```

### Quick Operations
```bash
ghostctl version                 # Show version info
ghostctl backup menu             # Backup management
ghostctl btrfs menu              # Btrfs snapshots

# Virtualization
ghostctl pve menu                # Proxmox VE management
ghostctl uefi status             # Check UEFI dependencies
ghostctl uefi enroll -o file.fd  # Create Secure Boot VARS

# Containers & Storage
ghostctl docker menu             # Docker management
ghostctl storage s3              # MinIO/S3 management

# Development
ghostctl dev menu                # Development tools
ghostctl nvim menu               # Neovim management
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

- **[Full Documentation](docs/index.md)** - Complete documentation index
- **[Commands Reference](docs/reference/COMMANDS.md)** - All commands and syntax
- **[Installation Guide](docs/deployment/INSTALL.md)** - Installation options
- **[Change Log](CHANGELOG.md)** - Version history

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
- **Core**: Rust 1.91+, OpenSSL
- **Optional**: Docker, Ansible, Terraform, virt-firmware (for UEFI)
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

Report security issues to: ckelley@ghostkellz.sh

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
- 📧 **Email**: ckelley@ghostkellz.sh

---

**Made for the Linux community**

*GhostCTL - Simplifying Linux administration, one command at a time.*

