# 📋 GhostCTL Commands Reference v1 

Complete command documentation for GhostCTL v0.8.2 - The ultimate system and homelab management tool.

⚠️ **Version Note**: This documentation covers the latest v0.8.2 features including enhanced Docker ecosystem, multi-registry support, enhanced Proxmox VE management, Restic CLI integration, AUR helper preference system, and improved network/DNS tools.

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
ghostctl arch fix                 # Fix common Arch issues (pacman, keyring, mirrors)
ghostctl arch clean <target>      # Clean specific targets (orphans, mirrors, pkgfix, gpg, locks, all)
ghostctl arch bouncer <target>    # Fix and bounce back from issues (pacman, keyring, mirrors, all)
ghostctl arch aur                 # AUR package management
ghostctl arch boot                # Boot configuration management
ghostctl arch health              # System health check and maintenance
ghostctl arch performance         # Performance optimization
ghostctl arch optimize            # Optimize system performance (zram/zswap)
ghostctl arch mirrors             # Optimize mirror list with reflector
ghostctl arch orphans             # Remove orphaned packages

# Clean target examples:
ghostctl arch clean orphans       # Remove orphaned packages
ghostctl arch clean mirrors       # Clean and optimize mirror list
ghostctl arch clean locks         # Clear pacman locks
ghostctl arch clean all           # Perform all cleanup operations

# Bouncer target examples:
ghostctl arch bouncer pacman      # Fix pacman database and bounce back
ghostctl arch bouncer keyring     # Fix keyring issues and bounce back
ghostctl arch bouncer mirrors     # Fix mirrors and test connectivity
ghostctl arch bouncer all         # Full system recovery sequence
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

### 🏠 Proxmox VE Management (Enhanced in v0.8.0)
```bash
ghostctl pve menu                 # PVE management menu
ghostctl proxmox menu             # Enhanced Proxmox tools with categories

# Through enhanced Proxmox menu:
# - Container Templates (Docker, Portainer, Nginx Proxy Manager, Pi-hole, etc.)
# - Virtual Machines (Home Assistant OS, Windows 11, Ubuntu, Debian)
# - System Administration (Post install, PBS, Dark theme, CPU scaling)
# - Monitoring & Logging (Prometheus, Grafana, InfluxDB, Zabbix)
# - Development Tools (GitLab, Jenkins, Code Server, Docker Registry)
# - Proxmox Management Tools (Bulk VM/CT operations, backup management)
# - Cluster Management (status, join, add/remove nodes)
# - Resource usage reports and system information

# Legacy commands still available:
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

## 🔐 Security & Key Management (Updated in v0.7.0)

### Security Commands
```bash
# Full security menu (both long and short form)
ghostctl security menu            # Security management menu
ghostctl sec menu                 # Security management menu (short alias)

# Direct command access
ghostctl ssh                      # SSH key management
ghostctl gpg                      # GPG key management
ghostctl security credentials     # Credential management
ghostctl sec credentials          # Credential management (short alias)
ghostctl security audit           # Security audit
ghostctl sec audit                # Security audit (short alias)
```

### SSH Key Management
```bash
# SSH operations (direct access)
ghostctl ssh                      # Interactive SSH key management
ghostctl ssh generate             # Generate new SSH key pair
ghostctl ssh list                 # List SSH keys
ghostctl ssh copy-id user@host    # Copy SSH key to remote host
ghostctl ssh config               # SSH configuration management
```

### GPG Key Management
```bash
# GPG operations (direct access)
ghostctl gpg                      # Interactive GPG key management
ghostctl gpg generate             # Generate new GPG key
ghostctl gpg list                 # List GPG keys
ghostctl gpg export               # Export GPG keys
ghostctl gpg import               # Import GPG keys
```

### Credential Management
```bash
# Secure credential storage
ghostctl security credentials     # Credential management menu
ghostctl sec credentials          # Credential management menu (short)

# Available operations through menu:
# - Unlock credential store
# - Store new credential
# - List stored credentials
# - Retrieve credential
# - Delete credential
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
ghostctl btrfs list               # List snapshots
ghostctl btrfs create NAME        # Create snapshot with specified name
ghostctl btrfs create NAME -s /home  # Create snapshot of specific subvolume
ghostctl btrfs delete NAME        # Delete snapshot by name
ghostctl btrfs restore NAME PATH  # Restore snapshot to target path
ghostctl btrfs status             # Show filesystem status and health
ghostctl btrfs scrub [PATH]       # Start filesystem scrub (default: /)
ghostctl btrfs balance [PATH]     # Start filesystem balance (default: /)
ghostctl btrfs usage [PATH]       # Show filesystem usage (default: /)
ghostctl btrfs quota [PATH]       # Manage quotas (default: /)
```

### Snapper Integration
```bash
ghostctl btrfs snapper setup     # Setup snapper configurations
ghostctl btrfs snapper edit CONFIG # Edit snapper configuration
ghostctl btrfs snapper list      # List snapper configurations
```

## 🌐 Network & Infrastructure (Updated in v0.7.0)

### Network Management
```bash
# Full network menu (both long and short form)
ghostctl network menu             # Network management menu
ghostctl net menu                 # Network management menu (short alias)

# Direct command access
ghostctl dns DOMAIN               # DNS lookup and configuration
ghostctl nc                       # Netcat utilities
ghostctl network mesh             # Mesh networking (Tailscale/Headscale)
ghostctl net mesh                 # Mesh networking (short alias)
ghostctl network scan TARGET      # Network port scanning with gscan
ghostctl net scan TARGET          # Network port scanning (short alias)
```

### DNS Operations (Enhanced in v0.8.0)
```bash
# DNS lookup and management
ghostctl dns google.com           # DNS lookup for domain
ghostctl dns --type MX domain.com # Specific record type lookup
ghostctl dns --reverse 8.8.8.8    # Reverse DNS lookup

# Through network menu:
# - DNS lookup with multiple record types
# - DNSSEC verification and validation
# - Reverse DNS lookups
# - DNS performance testing
```

### Network Scanning with gscan (Enhanced in v0.8.0)
```bash
# Basic scan
ghostctl network scan 192.168.1.1
ghostctl net scan 192.168.1.1     # Short form

# Network range scan
ghostctl network scan 192.168.1.0/24
ghostctl net scan 192.168.1.0/24  # Short form

# Custom port range
ghostctl network scan TARGET -s START_PORT -e END_PORT
ghostctl net scan TARGET -s START_PORT -e END_PORT

# Scan with banner grabbing
ghostctl network scan TARGET --banner
ghostctl net scan TARGET --banner

# Through network menu:
# - Target-based scanning with custom parameters
# - Interactive scan mode with real-time results
# - Service detection and banner grabbing
# - Network discovery and host enumeration
```

### Netcat Utilities
```bash
# File transfer
ghostctl nc send FILE HOST PORT    # Send file to host
ghostctl nc receive FILE PORT      # Receive file on port

# Communication
ghostctl nc chat HOST PORT         # Connect to chat session
ghostctl nc chat PORT              # Start chat server

# Connectivity testing
ghostctl nc check HOST PORT        # Check port connectivity
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

## 🔐 SSL Certificate Management (Enhanced in v0.8.0)

### SSL Operations
```bash
ghostctl ssl menu                 # SSL management menu
ghostctl nginx menu               # Enhanced nginx management with ACME.sh

# Through nginx menu:
# - ACME.sh installation and setup
# - Certificate issuance with multiple DNS providers
# - PowerDNS API integration
# - Azure DNS provider support  
# - Automated certificate deployment
# - Certificate renewal and monitoring

# Legacy commands still available:
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

## 💾 Restic Backup CLI (New in v0.8.0)

### Restic Operations
```bash
ghostctl restic menu              # Interactive restic CLI tools
ghostctl backup menu              # Full backup management system

# Through restic menu:
# - Initialize repository
# - Create backups with custom paths
# - List and browse snapshots  
# - Restore from specific snapshots
# - Forget old snapshots with retention policies
# - Check repository integrity
```

## 📦 AUR Helper Management (Enhanced in v0.8.0)

### AUR Operations  
```bash
ghostctl arch aur                 # AUR helper management menu

# Through AUR menu:
# - Check installed AUR helpers (reaper/paru/yay/trizen/pikaur)
# - Set preferred AUR helper with persistent config
# - Install AUR helpers (reaper recommended, paru, yay)
# - Update AUR packages using preferred helper
# - Clean AUR cache with confirmation
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

## 🆕 What's New in v0.8.0

### 🚀 Enhanced Infrastructure Management 
- **Restic CLI Integration**: Complete interactive restic backup management with repository initialization, backup creation, snapshot management, restoration, and integrity checking
- **AUR Helper Preference System**: Persistent AUR helper selection (reap/paru/yay) with automatic fallback and installation management  
- **Enhanced Proxmox VE Tools**: Comprehensive Proxmox management with categorized scripts, cluster management, bulk operations, and system administration tools

### 🌐 Network & DNS Improvements
- **Enhanced DNS Tools**: DNSSEC verification, comprehensive record type support, and DNS performance testing
- **Interactive Network Scanning**: Target-based scanning with real-time results, service detection, and network discovery
- **SSL Certificate Enhancements**: PowerDNS and Azure DNS provider support for ACME.sh integration

### 💾 Backup & Security Enhancements  
- **Restic CLI Menu**: Full interactive menu for repository management, backup scheduling, snapshot browsing, and restoration workflows
- **Container Security**: Enhanced Docker container management with security scanning and vulnerability assessment
- **Credential Management**: Improved secure credential storage with interactive management interface

## 🆕 What's New in v0.7.0

### 🚀 Enhanced CLI Experience (New in v0.7.0)
- **Short Command Aliases**: Quick access with `ghostctl net` and `ghostctl sec` shortcuts
- **Direct Command Access**: Run `ghostctl ssh`, `ghostctl gpg`, `ghostctl dns`, `ghostctl nc` directly without menus
- **Unified Menu System**: Both long and short forms support interactive menus (`ghostctl network menu` or `ghostctl net menu`)
- **Intuitive Command Structure**: Unix-like command patterns for better usability
- **Backward Compatibility**: All existing commands continue to work as before

### 🎯 Enhanced System Management
- **Arch Linux Enhancements**: Complete system maintenance with dotfiles management, disk space checking, package database rebuild, swap/zram configuration
- **NVIDIA Complete Suite**: Driver management (proprietary/open/open-beta), container runtime setup, GPU passthrough for VMs
- **Health Monitoring**: System health checks, performance optimization, maintenance automation

### 🛠️ Development Environment Improvements  
- **Mason.nvim Integration**: Zero-config Neovim development environments for multiple languages
- **Terminal Ecosystem**: Full Alacritty support, enhanced Ghostty/WezTerm configurations
- **Language Support**: Comprehensive Rust, Python, Go, Zig development environments

### 🔐 Security & Infrastructure
- **Enhanced Security Management**: Direct command access for SSH (`ghostctl ssh`) and GPG (`ghostctl gpg`) operations
- **Short Command Aliases**: Use `ghostctl sec` and `ghostctl net` for quick access to security and network menus
- **Credential Management**: Secure credential storage and management with interactive menus
- **Backup & Recovery**: Automated backup systems, integrity verification, system recovery
- **Network Tools**: Direct access to DNS lookups (`ghostctl dns`), netcat utilities (`ghostctl nc`), and network diagnostics

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
ghostctl arch clean all          # Comprehensive cleanup
ghostctl arch bouncer all        # Full recovery sequence

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
ghostctl sec audit               # Short form
ghostctl ssh                     # SSH key management
ghostctl gpg                     # GPG key management
ghostctl security credentials    # Credential management
ghostctl sec credentials         # Short form

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
ghostctl arch clean all          # Complete system cleanup
ghostctl arch bouncer all        # Full system recovery
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
               # Update GhostCTL
ghostctl update system        # Update system packages
ghostctl update ghost-tools   # Update Ghost tools
ghostctl update all           # Update everything
```

### Maintenance
```bash
ghostctl maintenance          # Run maintenance tasks
ghostctl cleanup              # Clean temporary files
ghostctl backup-config        # Backup configurations
ghostctl restore-config       # Restore configurations
```

## 🚨 Troubleshooting Commands

### Diagnostics
```bash
ghostctl diagnose            # Run diagnostics
ghostctl logs                # View all logs
ghostctl debug               # Enable debug mode
ghostctl reset               # Reset to defaults
```

### Repair Operations
```bash
ghostctl repair docker       # Repair Docker installation
ghostctl repair nginx        # Repair Nginx configuration
ghostctl repair ssl          # Repair SSL certificates
ghostctl repair permissions  # Fix file permissions
```

## 💡 Help & Documentation

### Help System
```bash
ghostctl help                # Main help
ghostctl help <command>      # Command-specific help
ghostctl docs                # Open documentation
ghostctl examples            # Show examples
ghostctl tips                # Usage tips
```

### Interactive Guides
```bash
ghostctl guide homelab       # Homelab setup guide
ghostctl guide dev-env       # Development environment guide
ghostctl guide ssl           # SSL setup guide
ghostctl guide docker        # Docker guide
```

---

## 📝 Notes

- All commands can be run interactively through the main menu: `ghostctl`
- Most commands have both interactive and non-interactive modes
- Use `ghostctl help <command>` for detailed command information
- Configuration files are stored in `~/.config/ghostctl/`
- Scripts are located in `/data/projects/ghostctl/scripts/`
- Logs are available via `ghostctl logs` or `journalctl -u ghostctl`

## 🛠️ Development Environment

### Multi-Language Support

#### 🦀 Rust Development
```bash
# Toolchain management
rustup update                # Update Rust
cargo new project           # New project
cargo build --release      # Release build
cargo test                 # Run tests
cargo clippy              # Linting
cargo fmt                 # Formatting
```

#### ⚡ Zig Development
```bash
# With Zion meta-tool
zion new project           # New project
zion build                # Build project
zion test                 # Run tests

# Direct Zig commands
zig init-exe              # Initialize executable
zig build                 # Build project
zig test                  # Run tests
```

#### 🐹 Go Development
```bash
# Module management
go mod init module-name    # Initialize module
go mod tidy               # Clean dependencies
go get package            # Add dependency
go build                  # Build project
go test ./...             # Run tests
go run .                  # Run project
```

#### 🐍 Python Development
```bash
# Virtual environments
python3 -m venv env       # Create venv
source env/bin/activate   # Activate venv
conda create -n env       # Create conda env
conda activate env       # Activate conda env

# Package management
pip install package       # Install package
pip freeze > requirements.txt  # Export deps
pip install -r requirements.txt  # Install deps
```

### 👻 Ghost Tools Commands

#### ⚡ Reaper (AUR Helper)
```bash
reap <package>            # Install package
reap -S <query>           # Search packages
reap -R <package>         # Remove package
reap -Syu                 # System update
reap -Qm                  # List AUR packages
```

#### 🦀 Oxygen (Rust Tool)
```bash
oxygen new <project>      # New Rust project
oxygen build             # Build project
oxygen test              # Run tests
oxygen deploy            # Deploy project
oxygen bench             # Run benchmarks
```

#### ⚡ Zion (Zig Tool)
```bash
zion new <project>        # New Zig project
zion build               # Build project
zion test                # Run tests
zion clean               # Clean build
zion deps                # Manage dependencies
```

#### 🎮 NVControl
```bash
nvcontrol status         # GPU status
nvcontrol temp           # Temperature info
nvcontrol fan            # Fan control
nvcontrol overclock      # Performance tuning
```

## 🔐 SSL Certificate Management (acme.sh)

### Installation & Setup
```bash
# Install acme.sh
curl https://get.acme.sh | sh
source ~/.bashrc

# Account registration
acme.sh --register-account -m your@email.com
```

### Certificate Operations
```bash
# Issue certificate (webroot)
acme.sh --issue -d domain.com --webroot /var/www/html

# Issue certificate (standalone)
acme.sh --issue -d domain.com --standalone

# Issue certificate (DNS API - Cloudflare)
export CF_Token="your-token"
acme.sh --issue -d domain.com --dns dns_cf

# Install certificate to custom location
acme.sh --install-cert -d domain.com \
  --cert-file /etc/nginx/certs/domain.com/cert.pem \
  --key-file /etc/nginx/certs/domain.com/private.key \
  --fullchain-file /etc/nginx/certs/domain.com/fullchain.pem \
  --reloadcmd "systemctl reload nginx"

# Renew certificates
acme.sh --renew-all         # Renew all
acme.sh --renew -d domain.com  # Renew specific

# List certificates
acme.sh --list

# Certificate info
acme.sh --info -d domain.com
```

### DNS API Configuration
```bash
# Cloudflare
export CF_Token="your-cloudflare-token"
export CF_Account_ID="your-account-id"

# DigitalOcean
export DO_API_KEY="your-digitalocean-api-key"

# AWS Route53
export AWS_ACCESS_KEY_ID="your-access-key"
export AWS_SECRET_ACCESS_KEY="your-secret-key"

# Add to shell config for persistence
echo 'export CF_Token="your-token"' >> ~/.bashrc
```

## 🐳 Docker Management

### Container Operations
```bash
# Basic commands
docker ps                 # List running containers
docker ps -a              # List all containers
docker images             # List images
docker logs container     # View logs
docker exec -it container bash  # Enter container

# Container lifecycle
docker run image          # Run container
docker start container    # Start container
docker stop container     # Stop container
docker restart container  # Restart container
docker rm container       # Remove container
```

### Docker Compose
```bash
# Compose operations
docker-compose up         # Start services
docker-compose up -d      # Start in background
docker-compose down       # Stop services
docker-compose logs       # View logs
docker-compose ps         # List services
docker-compose restart    # Restart services
```

### Image Management
```bash
# Image operations
docker build -t name .    # Build image
docker pull image         # Pull image
docker push image         # Push image
docker rmi image          # Remove image
docker system prune       # Clean unused data
```

## 🌐 Nginx Management

### Service Control
```bash
# Systemd service
sudo systemctl start nginx     # Start Nginx
sudo systemctl stop nginx      # Stop Nginx
sudo systemctl restart nginx   # Restart Nginx
sudo systemctl reload nginx    # Reload config
sudo systemctl status nginx    # Check status
sudo systemctl enable nginx    # Enable auto-start
```

### Configuration
```bash
# Test configuration
sudo nginx -t             # Test config syntax
sudo nginx -T             # Test and dump config

# Reload configuration
sudo nginx -s reload      # Graceful reload
sudo nginx -s quit        # Graceful shutdown
```

### SSL Configuration
```bash
# Certificate paths (GhostCTL standard)
/etc/nginx/certs/domain.com/cert.pem
/etc/nginx/certs/domain.com/private.key
/etc/nginx/certs/domain.com/fullchain.pem

# Nginx SSL config example
server {
    listen 443 ssl;
    server_name domain.com;
    
    ssl_certificate /etc/nginx/certs/domain.com/fullchain.pem;
    ssl_certificate_key /etc/nginx/certs/domain.com/private.key;
    
    # SSL security settings
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
}
```

## 📝 Script Management

### Local Scripts
```bash
# Script directories
/data/projects/ghostctl/scripts/homelab/     # Homelab automation
/data/projects/ghostctl/scripts/dev/         # Development scripts
/data/projects/ghostctl/scripts/docker/      # Container scripts
/data/projects/ghostctl/scripts/templates/   # Script templates

# Execute scripts
bash /path/to/script.sh   # Run script
chmod +x script.sh        # Make executable
./script.sh              # Execute directly
```

### Script Templates
```bash
# Create from template
cp templates/basic_bash.sh new_script.sh
cp templates/service_manager.sh my_service.sh
cp templates/package_installer.sh install_deps.sh
```

## 📝 Editor Configuration

### Neovim with LazyVim
```bash
# LazyVim commands
:Lazy                     # Plugin manager
:LazyUpdate              # Update plugins
:LazyClean               # Clean unused plugins
:LazyProfile             # Performance profiling

# LSP commands
:LspInfo                 # Language server info
:LspRestart              # Restart language server
:Mason                   # LSP installer

# File operations
:Telescope find_files    # Find files
:Telescope live_grep     # Search in files
:NvimTreeToggle         # File explorer
```

### Terminal (Ghostty)
```bash
# Ghostty configuration
~/.config/ghostty/config  # Config file location

# Key bindings (default)
Ctrl+Shift+T             # New tab
Ctrl+Shift+N             # New window
Ctrl+Shift+C             # Copy
Ctrl+Shift+V             # Paste
```

## 🏠 Homelab Commands

### Proxmox VE
```bash
# VM management
qm list                  # List VMs
qm start vmid           # Start VM
qm stop vmid            # Stop VM
qm restart vmid         # Restart VM
qm status vmid          # VM status

# Container management
pct list                # List containers
pct start ctid          # Start container
pct stop ctid           # Stop container
pct enter ctid          # Enter container

# Storage management
pvesm status            # Storage status
pvesm list storage      # List storage
```

### System Monitoring
```bash
# Resource monitoring
htop                    # Interactive process viewer
df -h                   # Disk usage
free -h                 # Memory usage
iostat                  # I/O statistics
netstat -tulpn         # Network connections

# Service monitoring
systemctl list-units   # List services
journalctl -f          # Follow system logs
journalctl -u service  # Service-specific logs
```

## 🔧 Troubleshooting Commands

### System Diagnostics
```bash
# System information
uname -a                # System info
lsb_release -a         # Distribution info
cat /proc/version      # Kernel version
systemctl --failed     # Failed services

# Network diagnostics
ping google.com        # Network connectivity
dig domain.com         # DNS lookup
nslookup domain.com    # DNS resolution
ss -tulpn             # Socket statistics

# Disk diagnostics
lsblk                  # Block devices
fdisk -l              # Disk partitions
fsck /dev/device      # File system check
smartctl -a /dev/sda  # SMART disk info
```

### Log Analysis
```bash
# System logs
journalctl --since yesterday    # Recent logs
journalctl -p err              # Error logs only
journalctl -u nginx            # Service logs
tail -f /var/log/nginx/error.log  # Follow error log

# Application logs
docker logs container          # Container logs
tail -f application.log       # Follow app log
grep -i error /var/log/syslog # Search for errors
```

## 🔄 Backup & Recovery

### Restic Backup
```bash
# Initialize repository
restic init --repo /backup/location

# Create backup
restic backup ~/important --repo /backup/location

# List snapshots
restic snapshots --repo /backup/location

# Restore backup
restic restore latest --repo /backup/location --target /restore/path

# Check repository
restic check --repo /backup/location
```

---

For more detailed information, see the specific documentation files:
- [Docker Guide](DOCKER.md)
- [Proxmox Integration](PROXMOX.md) 
- [Cloud Services](CLOUD.md)
- [Backup with Restic](RESTIC.md)
