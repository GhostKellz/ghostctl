# 📋 GhostCTL Commands Reference

Complete command documentation for GhostCTL - The ultimate system and homelab management tool.

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
ghostctl nvidia                   # Interactive NVIDIA menu
ghostctl nvidia install           # Install NVIDIA drivers
ghostctl nvidia optimize          # Optimize GPU performance
ghostctl nvidia passthrough       # GPU passthrough setup for VMs
ghostctl nvidia wayland           # Configure Wayland support
ghostctl nvidia build-source      # Build kernel modules from source
ghostctl nvidia dkms-status       # Show DKMS module status
ghostctl nvidia dkms-cleanup      # Clean old DKMS entries
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

### 🔨 Mason.nvim Integration
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

### 🔷 Advanced Terminal Support
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

## 🔐 Security & Key Management

### Security Commands
```bash
ghostctl security menu            # Security management menu
ghostctl security ssh             # SSH configuration
ghostctl security gpg             # GPG management
ghostctl security credentials     # Credential management

# Short aliases
ghostctl sec menu                 # Security menu (short)
ghostctl sec ssh                  # SSH (short)
ghostctl sec gpg                  # GPG (short)
ghostctl sec credentials          # Credentials (short)

# Standalone commands
ghostctl ssh                      # SSH key management
ghostctl gpg                      # GPG key management
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
ghostctl gpg                      # Interactive GPG key management menu
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

## 💾 Backup & Recovery

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

## 🗃️ Filesystem Management

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

## 🌐 Network & Infrastructure

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
ghostctl scan TARGET              # Network port scanning with TUI
ghostctl network scan TARGET      # Legacy alias (deprecated)
```

### DNS Operations
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

### Network Scanning with Native TUI Scanner
```bash
# Basic scan with beautiful TUI
ghostctl scan 192.168.1.1

# Scan specific ports
ghostctl scan 192.168.1.1 -p 80,443,8080
ghostctl scan 192.168.1.1 -p 1-1000

# Full port scan (all 65535 ports)
ghostctl scan 192.168.1.1 --full

# Scan with service detection
ghostctl scan 192.168.1.1 --service

# Scan with custom thread count
ghostctl scan 192.168.1.1 -t 200

# Network range scan (CIDR)
ghostctl scan 192.168.1.0/24

# Output formats (disable TUI)
ghostctl scan 192.168.1.1 --json      # JSON output
ghostctl scan 192.168.1.1 --quiet     # Minimal output

# Combined options
ghostctl scan 192.168.1.1 -p 1-10000 --service -t 150
```

**✨ Scanner Features:**
- 🎨 **Beautiful TUI** - Real-time progress with ratatui interface
- ⚡ **Async Performance** - Concurrent scanning with configurable threads
- 🔍 **Service Detection** - Identify services running on open ports
- 🌐 **CIDR Support** - Scan entire network ranges
- 📊 **Real-time Stats** - Live progress, ETA, and port statistics
- 🎛️ **Interactive Controls** - Navigate results with keyboard shortcuts
- 📈 **Multiple Views** - Overview, results, statistics, and settings tabs
- 🚀 **Zero Dependencies** - Native Rust implementation (replaces gscan)

**🎮 TUI Controls:**
- `←` `→` : Switch between tabs
- `↑` `↓` : Navigate scan results
- `q` : Quit scanner

```bash
# Legacy network menu access (still available):
# ghostctl network menu → Network Scanner & Discovery
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

## ⚙️ System Services

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

## 💾 Restic Backup CLI

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

## 📦 AUR Helper Management

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
ghostctl tools install            # Install development tools
ghostctl tools configure          # Configure tools
ghostctl tools update             # Update tools
```

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
- **README.md** - Project overview and quick start
- **INSTALL.md** - Installation guide

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

### Updates
```bash
ghostctl system update        # Update system packages
ghostctl tools update         # Update Ghost tools
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

## 🔐 UEFI Secure Boot Management

### Status Check
```bash
ghostctl uefi status
```

### Key Enrollment
```bash
# Create VARS with Secure Boot keys for Windows 11
ghostctl uefi enroll -o /var/lib/libvirt/qemu/nvram/win11_VARS.fd

# Fix ownership for libvirt
sudo chown libvirt-qemu:libvirt-qemu /var/lib/libvirt/qemu/nvram/win11_VARS.fd
```

### Verify VARS
```bash
ghostctl uefi verify /path/to/vars.fd
```

---

For more detailed information, see:
- [Docker Guide](../docker/README.md)
- [Proxmox Integration](../proxmox/README.md)
- [Network Scanner](../networking/scanner.md)
- [UEFI Secure Boot](../uefi/README.md)
