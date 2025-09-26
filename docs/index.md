# 📚 GhostCTL Documentation Index

## 🚀 **Quick Start**
- [Installation Guide](deployment/INSTALL.md) - Get GhostCTL running on your system
- [Command Reference](reference/COMMANDS.md) - Essential commands and syntax

## 🔥 **Core Features**

### 🔍 **Network & Security**
- [**Scanner**](features/SCANNER.md) - Native Rust port scanner with TUI
- [**Networking**](features/NETWORKING.md) - Advanced networking, firewalls, and virtualization
- [**Docker**](features/DOCKER.md) - Container management and DevOps tools

### 🖥️ **Virtualization & Cloud**
- [**Proxmox VE**](features/PROXMOX.md) - Complete PVE management and automation
- [**PVE v9 Features**](features/pve_v9.md) - Latest Proxmox VE capabilities

### 🎮 **Gaming & Desktop**
- [**Proton & Gaming**](features/PROTON.md) - Gaming optimization and compatibility

## 🏗️ **System Administration**
- [**Architecture Overview**](architecture/) - System design and module structure
- [**API Reference**](api/) - REST API and integration interfaces
- [**Development Guides**](guides/) - Contributing and extending GhostCTL

## 📋 **Reference Materials**
- [**TODO & Roadmap**](reference/TODO.md) - Planned features and development timeline
- [**Command Reference**](reference/COMMANDS.md) - Complete command documentation

## 🔧 **Migration & Updates**

### Recent Changes (v1.0+)
- **✅ Native Scanner** - Replaced external `gscan` with high-performance Rust implementation
- **✅ Enhanced Documentation** - Reorganized docs with comprehensive feature coverage
- **✅ Advanced Networking** - Enterprise-grade nftables, UFW, and libvirt integration
- **✅ Proxmox Integration** - Complete PVE automation with native scanner support

## 🎯 **Popular Use Cases**

### 🔐 **Security & Compliance**
```bash
# Network security scanning
ghostctl scan 192.168.1.0/24

# Firewall automation with scan integration
ghostctl pve firewall

# Advanced nftables management
ghostctl network firewall advanced
```

### 🏢 **Enterprise Infrastructure**
```bash
# Proxmox cluster management
ghostctl proxmox menu

# Storage migration and backup rotation
ghostctl pve storage-migration
ghostctl pve backup-rotation

# Container registry and DevOps
ghostctl docker registry
```

### 🎮 **Gaming & Desktop**
```bash
# Gaming environment optimization
ghostctl gaming setup

# Graphics and performance tuning
ghostctl gaming graphics
ghostctl gaming performance
```

## 🆘 **Support & Troubleshooting**

### Common Issues
- **Scanner Performance**: Use appropriate timing templates for your network
- **Permission Errors**: Some features require elevated privileges
- **Integration Issues**: Ensure target services (Docker, Proxmox) are running

### Getting Help
- Check the [Command Reference](reference/COMMANDS.md) for syntax
- Review feature-specific documentation in [`features/`](features/)
- Examine the [development roadmap](reference/TODO.md) for planned enhancements

## 📈 **Performance Characteristics**

| Feature | Performance | Resource Usage |
|---------|-------------|----------------|
| Native Scanner | 1000 ports/30-60s | <50MB RAM |
| Network Management | Real-time | Minimal overhead |
| PVE Integration | API-limited | Low CPU impact |
| Container Ops | Docker-native | Efficient delegation |

---

**📍 Quick Navigation:**
- [📖 Main README](../README.md) - Project overview and features
- [⚡ Installation](deployment/INSTALL.md) - Get started immediately
- [🔍 Scanner Guide](features/SCANNER.md) - Network discovery and analysis
- [🌐 Networking](features/NETWORKING.md) - Advanced network management
- [🏢 Enterprise](features/PROXMOX.md) - Business infrastructure automation