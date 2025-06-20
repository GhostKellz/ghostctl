# 📋 GhostCTL Commands Reference

Complete command documentation for GhostCTL v0.5.0

## 🚀 Main Application

```bash
# Launch GhostCTL interactive menu
ghostctl

# Quick access commands
ghostctl --help                    # Show help
ghostctl --version                 # Show version
ghostctl --system                  # System management menu
ghostctl --dev                     # Development environment menu
ghostctl --docker                  # Docker management menu
ghostctl --scripts                 # Scripts and tools menu
ghostctl --nginx                   # Nginx management menu
ghostctl --nvim                    # Neovim setup menu
ghostctl --terminal                # Terminal configuration menu
ghostctl --tools                   # External tools menu
```

## 🏠 System Management Commands

### Quick System Commands
```bash
ghostctl system update             # Update system packages
ghostctl system status             # Show system status
ghostctl system services           # Manage system services
ghostctl system arch               # Arch Linux management
ghostctl system nixos              # NixOS management
```

### Arch Linux Management
```bash
ghostctl arch install <package>    # Install package with reaper/paru/yay
ghostctl arch search <query>       # Search packages
ghostctl arch update              # Update system
ghostctl arch aur-helper          # Manage AUR helpers
ghostctl arch reaper install      # Install reaper AUR helper
```

### NixOS Management  
```bash
ghostctl nixos rebuild            # Rebuild NixOS configuration
ghostctl nixos update             # Update channels
ghostctl nixos garbage-collect    # Clean old generations
ghostctl nixos search <package>   # Search packages
```

## 🛠️ Development Environment Commands

### Multi-Language Development
```bash
ghostctl dev menu                 # Development environment menu
ghostctl dev rust                 # Rust development setup
ghostctl dev zig                  # Zig development setup  
ghostctl dev go                   # Go development setup
ghostctl dev python               # Python development setup
ghostctl dev ghost-tools          # Ghost ecosystem tools
```

### Rust Development
```bash
ghostctl dev rust install         # Install Rust toolchain
ghostctl dev rust update          # Update Rust
ghostctl dev rust new <project>   # Create new project
ghostctl dev rust build           # Build project
ghostctl dev rust test            # Run tests
ghostctl dev rust fmt             # Format code
```

### Zig Development  
```bash
ghostctl dev zig install          # Install Zig compiler
ghostctl dev zig zion             # Install Zion meta-tool
ghostctl dev zig new <project>    # Create new project
ghostctl dev zig build            # Build project
ghostctl dev zig test             # Run tests
ghostctl dev zig zls              # Install Zig Language Server
```

### Go Development
```bash
ghostctl dev go install           # Install Go compiler
ghostctl dev go mod init <name>   # Initialize module
ghostctl dev go build             # Build project
ghostctl dev go test              # Run tests
ghostctl dev go get <package>     # Add dependency
```

### Python Development
```bash
ghostctl dev python install       # Install Python
ghostctl dev python venv <name>   # Create virtual environment
ghostctl dev python pip <package> # Install package
ghostctl dev python conda         # Install conda/miniconda
ghostctl dev python tools         # Install dev tools (black, pylint, etc.)
```

### 👻 Ghost Tools Commands
```bash
ghostctl ghost menu               # Ghost tools menu
ghostctl ghost install-all        # Install all Ghost tools
ghostctl ghost reaper             # Install Reaper AUR helper
ghostctl ghost oxygen             # Install Oxygen Rust tool
ghostctl ghost zion               # Install Zion Zig tool
ghostctl ghost nvcontrol          # Install NVControl NVIDIA tool
ghostctl ghost status             # Check Ghost tools status
ghostctl ghost uninstall          # Uninstall Ghost tools
```

## 🐳 Docker & DevOps Commands

### Docker Management
```bash
ghostctl docker menu             # Docker management menu
ghostctl docker install          # Install Docker
ghostctl docker status           # Show Docker status
ghostctl docker compose          # Docker Compose management
ghostctl docker homelab          # Homelab container stacks
ghostctl docker dev-env          # Development environment containers
```

### Docker Operations
```bash
ghostctl docker ps               # List containers (via GhostCTL)
ghostctl docker images           # List images
ghostctl docker logs <container> # Show container logs
ghostctl docker exec <container> # Enter container
ghostctl docker cleanup         # Clean unused containers/images
```

### Homelab Stacks
```bash
ghostctl docker media-stack     # Deploy media server stack
ghostctl docker monitor-stack   # Deploy monitoring stack
ghostctl docker proxy-stack     # Deploy reverse proxy stack
ghostctl docker dev-stack       # Deploy development stack
```

## 🏠 Proxmox VE Commands

### PVE Management  
```bash
ghostctl pve menu               # Proxmox management menu
ghostctl pve status             # Show PVE status
ghostctl pve vm                 # Virtual machine management
ghostctl pve ct                 # Container management
ghostctl pve storage            # Storage management
ghostctl pve backup             # Backup management
ghostctl pve cluster            # Cluster management
```

### VM Management
```bash
ghostctl pve vm list            # List VMs
ghostctl pve vm create          # Create VM wizard
ghostctl pve vm template        # Create VM template
ghostctl pve vm clone <id>      # Clone VM
ghostctl pve vm start <id>      # Start VM
ghostctl pve vm stop <id>       # Stop VM
ghostctl pve vm status <id>     # VM status
ghostctl pve vm migrate <id>    # Migrate VM
```

### Container Management
```bash
ghostctl pve ct list            # List containers
ghostctl pve ct create          # Create container wizard
ghostctl pve ct start <id>      # Start container
ghostctl pve ct stop <id>       # Stop container
ghostctl pve ct enter <id>      # Enter container
ghostctl pve ct backup <id>     # Backup container
```

### Homelab Automation
```bash
ghostctl pve deploy-farm        # Deploy VM farm
ghostctl pve docker-host        # Setup Docker host
ghostctl pve k8s-cluster        # Deploy Kubernetes cluster
ghostctl pve monitoring         # Setup monitoring
ghostctl pve backup-schedule    # Configure backups
```

## 📋 Scripts & Tools Commands

### Script Management
```bash
ghostctl scripts menu          # Scripts management menu
ghostctl scripts local         # Local script management
ghostctl scripts homelab       # Homelab scripts
ghostctl scripts dev           # Development scripts
ghostctl scripts templates     # Script templates
ghostctl scripts create        # Create new script
```

### Local Scripts
```bash
ghostctl scripts run <script>      # Run specific script
ghostctl scripts edit <script>     # Edit script
ghostctl scripts list homelab      # List homelab scripts
ghostctl scripts list dev          # List dev scripts
ghostctl scripts browse <category> # Browse script category
```

### Script Templates
```bash
ghostctl scripts template bash     # Create bash script template
ghostctl scripts template service  # Create service management template
ghostctl scripts template package  # Create package installer template
ghostctl scripts template maintenance # Create maintenance script template
```

## 🔐 SSL Certificate Management (acme.sh)

### acme.sh Commands
```bash
ghostctl ssl menu              # SSL management menu
ghostctl ssl install           # Install acme.sh
ghostctl ssl status             # Check acme.sh status
ghostctl ssl issue <domain>     # Issue certificate
ghostctl ssl renew             # Renew certificates
ghostctl ssl list              # List certificates
ghostctl ssl dns-api           # Configure DNS API
```

### Certificate Operations
```bash
ghostctl ssl webroot <domain>  # Issue with webroot validation
ghostctl ssl standalone <domain> # Issue with standalone validation
ghostctl ssl dns <domain>      # Issue with DNS validation
ghostctl ssl install-cert <domain> # Install to custom location
ghostctl ssl auto-renew        # Setup automatic renewal
```

## 🌐 Nginx Management Commands

### Nginx Operations
```bash
ghostctl nginx menu            # Nginx management menu
ghostctl nginx install         # Install Nginx
ghostctl nginx status          # Check Nginx status
ghostctl nginx config          # Configuration management
ghostctl nginx ssl             # SSL configuration
ghostctl nginx vhost           # Virtual host management
```

### SSL Integration
```bash
ghostctl nginx ssl-setup <domain>  # Setup SSL for domain
ghostctl nginx cert-check      # Check certificate status
ghostctl nginx auto-ssl        # Enable automatic SSL
ghostctl nginx ssl-renew       # Renew SSL certificates
```

### Configuration
```bash
ghostctl nginx test-config     # Test configuration
ghostctl nginx reload          # Reload configuration
ghostctl nginx restart         # Restart Nginx
ghostctl nginx logs            # View logs
```

## 📝 Neovim Setup Commands

### Neovim Installation
```bash
ghostctl nvim menu            # Neovim setup menu
ghostctl nvim install         # Install Neovim
ghostctl nvim lazyvim         # Install LazyVim
ghostctl nvim config          # Configure Neovim
ghostctl nvim plugins         # Manage plugins
ghostctl nvim lsp             # Setup Language Servers
```

### Development Setup
```bash
ghostctl nvim rust-setup      # Setup Rust development
ghostctl nvim go-setup        # Setup Go development
ghostctl nvim python-setup    # Setup Python development
ghostctl nvim zig-setup       # Setup Zig development
```

### Tools & Plugins
```bash
ghostctl nvim treesitter      # Install Tree-sitter
ghostctl nvim telescope       # Setup Telescope
ghostctl nvim mason           # Install Mason LSP manager
ghostctl nvim null-ls         # Setup null-ls
```

## 💻 Terminal Configuration Commands

### Ghostty Setup
```bash
ghostctl terminal menu        # Terminal configuration menu
ghostctl terminal ghostty     # Install Ghostty
ghostctl terminal config      # Configure terminal
ghostctl terminal themes      # Manage themes
ghostctl terminal fonts       # Font management
```

### Shell Configuration  
```bash
ghostctl terminal starship    # Install Starship prompt
ghostctl terminal zsh         # Setup Zsh
ghostctl terminal bash        # Configure Bash
ghostctl terminal fish        # Setup Fish shell
```

## 🔧 External Tools Commands

### Tool Management
```bash
ghostctl tools menu          # External tools menu
ghostctl tools acme-sh       # acme.sh management
ghostctl tools package-managers # Additional package managers
ghostctl tools system        # System utilities
ghostctl tools network       # Network tools
ghostctl tools monitoring    # Monitoring tools
```

### Package Managers
```bash
ghostctl tools snap          # Install Snap
ghostctl tools flatpak       # Install Flatpak
ghostctl tools appimage      # AppImage support
ghostctl tools nix           # Install Nix package manager
```

## 🏠 Homelab Integration Commands

### Quick Homelab Setup
```bash
ghostctl homelab menu        # Homelab management menu
ghostctl homelab init        # Initialize homelab environment
ghostctl homelab proxmox     # Proxmox VE setup
ghostctl homelab docker      # Docker homelab stacks
ghostctl homelab monitoring  # Setup monitoring
ghostctl homelab backup      # Backup configuration
```

### Infrastructure Deployment
```bash
ghostctl homelab media-server   # Deploy media server
ghostctl homelab game-server    # Deploy game servers
ghostctl homelab dev-env        # Development environment
ghostctl homelab monitoring     # Monitoring stack
ghostctl homelab reverse-proxy  # Reverse proxy setup
```

## 📊 Status & Information Commands

### System Information
```bash
ghostctl status               # Overall system status
ghostctl info                 # System information
ghostctl health               # Health check
ghostctl version              # Version information
ghostctl log                  # View GhostCTL logs
```

### Service Status
```bash
ghostctl status docker        # Docker status
ghostctl status nginx         # Nginx status
ghostctl status pve           # Proxmox status
ghostctl status ssl           # SSL certificates status
ghostctl status ghost-tools   # Ghost tools status
```

## 🔄 Update & Maintenance Commands

### Updates
```bash
ghostctl update               # Update GhostCTL
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
