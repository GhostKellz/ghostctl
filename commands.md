# 📋 GhostCTL Commands Reference

**Complete command documentation for GhostCTL v1.0**

## 🏠 Core Commands

### Main Menu
```bash
ghostctl menu                    # Interactive main menu
ghostctl                         # Same as menu (default)
```

### Configuration
```bash
ghostctl config show            # Display current configuration
ghostctl config edit            # Edit configuration file
ghostctl config reset           # Reset to default configuration
ghostctl config validate        # Validate configuration syntax
```

### Help & Information
```bash
ghostctl --help                 # Show help information
ghostctl --version              # Show version information
ghostctl health                 # System health check
ghostctl status                 # Overall system status
```

## 🏗️ System Administration

### Arch Linux Management
```bash
ghostctl arch                   # Arch Linux management menu
ghostctl arch quick-fix         # Apply common system fixes
ghostctl arch pacman-fix        # Fix pacman issues
ghostctl arch mirror-update     # Update mirror list
ghostctl arch gpg-fix           # Fix GPG key issues
ghostctl arch cleanup           # System cleanup
```

#### Arch Quick Fixes
- Fix corrupted GPG keys
- Reset pacman database locks
- Update mirror list with reflector
- Clear package cache
- Refresh package databases
- Fix Python package conflicts
- Fix broken symlinks
- Reset systemd failed units
- Fix /tmp permissions
- Clear old log files

### Package Management
```bash
ghostctl packages               # Package management menu
ghostctl packages update        # Update all packages
ghostctl packages install       # Install packages
ghostctl packages remove        # Remove packages
ghostctl packages search        # Search packages
ghostctl packages cleanup       # Clean package cache
```

### Service Management
```bash
ghostctl systemd                # Systemd management menu
ghostctl systemd status         # Service status overview
ghostctl systemd logs           # View service logs
ghostctl systemd enable         # Enable services
ghostctl systemd disable        # Disable services
ghostctl systemd restart        # Restart services
```

## 💾 Data Management

### Btrfs Operations
```bash
ghostctl btrfs                  # Btrfs management menu
ghostctl btrfs status           # Filesystem status
ghostctl btrfs snapshot         # Snapshot operations
ghostctl btrfs balance          # Balance filesystem
ghostctl btrfs defrag           # Defragment filesystem
ghostctl btrfs scrub            # Scrub filesystem
```

#### Btrfs Snapshot Management
```bash
ghostctl btrfs snapshot list    # List all snapshots
ghostctl btrfs snapshot create  # Create snapshot
ghostctl btrfs snapshot delete  # Delete snapshots
ghostctl btrfs snapshot rollback # Rollback to snapshot
```

### Snapper Integration
```bash
ghostctl snapper                # Snapper management
ghostctl snapper list           # List snapshots
ghostctl snapper create         # Create manual snapshot
ghostctl snapper cleanup        # Cleanup old snapshots
ghostctl snapper config         # Snapper configuration
```

### Backup Management
```bash
ghostctl backup                 # Backup management menu
ghostctl backup create          # Create backup
ghostctl backup restore         # Restore from backup
ghostctl backup schedule        # Schedule automated backups
ghostctl backup status          # Backup status
ghostctl backup repos           # Manage repositories
```

#### Restic Operations
```bash
ghostctl restic init            # Initialize repository
ghostctl restic backup          # Create backup
ghostctl restic restore         # Restore files
ghostctl restic snapshots       # List snapshots
ghostctl restic forget          # Remove old snapshots
ghostctl restic check           # Verify repository
```

## 🐳 DevOps & Containers

### Docker Management
```bash
ghostctl devops                 # DevOps tools menu
ghostctl devops docker          # Docker management
ghostctl devops registry        # Docker registry management
ghostctl devops compose         # Docker Compose operations
```

#### Docker Operations
```bash
ghostctl docker containers      # Manage containers
ghostctl docker images          # Manage images
ghostctl docker networks        # Manage networks
ghostctl docker volumes         # Manage volumes
ghostctl docker logs            # View container logs
ghostctl docker stats           # Container statistics
```

#### Docker Registry
```bash
ghostctl registry status        # Registry status
ghostctl registry deploy        # Deploy registry
ghostctl registry push          # Push images
ghostctl registry pull          # Pull images
ghostctl registry cleanup       # Clean old images
```

### Development Environment
```bash
ghostctl dev                    # Development tools
ghostctl dev setup              # Setup development environment
ghostctl dev github             # GitHub integration
ghostctl dev templates          # Project templates
```

## 🏗️ Infrastructure as Code

### Ansible Management
```bash
ghostctl infrastructure ansible # Ansible management menu
ghostctl ansible playbooks      # Manage playbooks
ghostctl ansible inventory      # Manage inventory
ghostctl ansible run            # Run playbooks
ghostctl ansible galaxy         # Ansible Galaxy operations
```

#### Ansible Operations
```bash
ghostctl ansible init           # Initialize project
ghostctl ansible create-playbook # Create new playbook
ghostctl ansible run-playbook   # Execute playbook
ghostctl ansible check          # Syntax check
ghostctl ansible vault          # Manage secrets
```

### Terraform Management
```bash
ghostctl infrastructure terraform # Terraform management menu
ghostctl terraform init         # Initialize project
ghostctl terraform plan         # Plan changes
ghostctl terraform apply        # Apply changes
ghostctl terraform destroy      # Destroy infrastructure
ghostctl terraform state        # State management
```

### Cloud Provider Tools
```bash
ghostctl infrastructure cloud   # Multi-cloud management
ghostctl cloud azure            # Azure CLI tools
ghostctl cloud aws              # AWS CLI tools
ghostctl cloud gcp              # Google Cloud tools
ghostctl cloud status           # Multi-cloud status
```

#### Azure Operations
```bash
ghostctl azure login            # Azure login
ghostctl azure subscriptions    # List subscriptions
ghostctl azure resources        # Resource management
ghostctl azure vms              # Virtual machines
ghostctl azure storage          # Storage accounts
ghostctl azure cost             # Cost analysis
```

## 🔐 Security & Key Management

### SSH Key Management
```bash
ghostctl network ssh            # SSH management menu
ghostctl ssh keys               # SSH key management
ghostctl ssh generate           # Generate new SSH key
ghostctl ssh copy               # Copy key to server
ghostctl ssh audit              # Security audit
```

#### SSH Operations
```bash
ghostctl ssh list               # List SSH keys
ghostctl ssh create             # Create new key
ghostctl ssh deploy             # Deploy to server
ghostctl ssh github             # GitHub integration
ghostctl ssh gitlab             # GitLab integration
ghostctl ssh test               # Test connections
```

### GPG Key Management
```bash
ghostctl arch gpg               # GPG management menu
ghostctl gpg list               # List GPG keys
ghostctl gpg generate           # Generate new key
ghostctl gpg export             # Export public key
ghostctl gpg import             # Import public key
ghostctl gpg sign               # Sign files/messages
```

### Certificate Management
```bash
ghostctl scripts ghostcert      # GhostCert SSL management
ghostctl cert status            # Certificate status
ghostctl cert generate          # Generate certificate
ghostctl cert renew             # Renew certificates
ghostctl cert deploy            # Deploy certificates
```

## 🏥 Proxmox VE Management

### Proxmox Operations
```bash
ghostctl proxmox                # Proxmox management menu
ghostctl proxmox health         # Health check
ghostctl proxmox vms            # Virtual machine management
ghostctl proxmox containers     # Container management
ghostctl proxmox storage        # Storage management
```

#### SDN (Software Defined Networking)
```bash
ghostctl proxmox sdn            # SDN management
ghostctl proxmox zones          # Zone management
ghostctl proxmox vnets          # Virtual networks
ghostctl proxmox bridges        # Bridge configuration
```

#### Firewall Management
```bash
ghostctl proxmox firewall       # Firewall management
ghostctl proxmox security-groups # Security groups
ghostctl proxmox rules          # Firewall rules
ghostctl proxmox ddos           # DDoS protection
```

## 💻 Development Tools

### Neovim Management
```bash
ghostctl nvim                   # Neovim management menu
ghostctl nvim health            # Health check
ghostctl nvim plugins           # Plugin management
ghostctl nvim config            # Configuration management
ghostctl nvim lsp               # LSP server management
```

#### Neovim Operations
```bash
ghostctl nvim check             # Full health check
ghostctl nvim install-plugins   # Install plugins
ghostctl nvim update-plugins    # Update plugins
ghostctl nvim setup-lsp         # Setup LSP servers
ghostctl nvim backup-config     # Backup configuration
```

### Shell & Terminal
```bash
ghostctl shell                  # Shell management menu
ghostctl shell zsh              # ZSH setup
ghostctl shell ohmyzsh          # Oh My Zsh setup
ghostctl shell powerlevel10k    # Powerlevel10k theme
ghostctl shell tmux             # Tmux configuration
```

## 🌐 Network Management

### Network Diagnostics
```bash
ghostctl network               # Network management menu
ghostctl network status        # Network status
ghostctl network test          # Network tests
ghostctl network scan          # Network scanning
ghostctl network monitor       # Network monitoring
```

### VPN Management
```bash
ghostctl network vpn           # VPN management
ghostctl network tailscale     # Tailscale setup
ghostctl network wireguard     # WireGuard setup
ghostctl network openvpn       # OpenVPN setup
```

## 🔧 Plugin & Script Management

### Plugin Operations
```bash
ghostctl plugins               # Plugin management menu
ghostctl plugins list          # List plugins
ghostctl plugins install       # Install plugin
ghostctl plugins remove        # Remove plugin
ghostctl plugins update        # Update plugins
```

### Script Management
```bash
ghostctl scripts               # Script management menu
ghostctl scripts sysadmin      # System admin scripts
ghostctl scripts backup        # Backup scripts
ghostctl scripts network       # Network scripts
ghostctl scripts custom        # Custom scripts
```

## 📦 Release & Installation

### Release Management
```bash
ghostctl release create        # Create release packages
ghostctl release package       # Generate distribution files
ghostctl release install       # Show installation info
```

### Installation Commands
```bash
# Universal installer
curl -sSL https://raw.githubusercontent.com/ghostkellz/ghostctl/main/install/install.sh | bash

# Proxmox installer
curl -sSL https://raw.githubusercontent.com/ghostkellz/ghostctl/main/install/install-proxmox.sh | bash

# Arch Linux
yay -S ghostctl

# Manual build
git clone https://github.com/ghostkellz/ghostctl.git
cd ghostctl/ghostctl && cargo build --release
```

## 🔄 Automation & Workflows

### Automated Workflows
```bash
ghostctl workflows             # Workflow management
ghostctl workflows create      # Create new workflow
ghostctl workflows list        # List workflows
ghostctl workflows run         # Run workflow
ghostctl workflows schedule    # Schedule workflow
```

### Systemd Integration
```bash
ghostctl systemd timers        # Manage timers
ghostctl systemd services      # Manage services
ghostctl systemd logs          # View logs
ghostctl systemd status        # Service status
```

## 📊 Monitoring & Analytics

### System Monitoring
```bash
ghostctl monitor               # System monitoring
ghostctl monitor resources     # Resource usage
ghostctl monitor processes     # Process monitoring
ghostctl monitor logs          # Log analysis
```

### Performance Analysis
```bash
ghostctl performance           # Performance analysis
ghostctl performance cpu       # CPU analysis
ghostctl performance memory    # Memory analysis
ghostctl performance disk      # Disk analysis
ghostctl performance network   # Network analysis
```

## 🚨 Emergency & Recovery

### System Recovery
```bash
ghostctl recovery              # Recovery operations
ghostctl recovery boot         # Boot issues
ghostctl recovery filesystem   # Filesystem repair
ghostctl recovery backup       # Emergency backup
```

### Quick Fixes
```bash
ghostctl emergency             # Emergency fixes
ghostctl emergency pacman      # Pacman issues
ghostctl emergency network     # Network issues
ghostctl emergency boot        # Boot problems
```

## 🎯 Common Use Cases

### Daily Administration
```bash
# Morning system check
ghostctl health && ghostctl status

# Update system
ghostctl arch quick-fix && ghostctl packages update

# Backup important data
ghostctl backup create --auto

# Monitor services
ghostctl systemd status
```

### Development Setup
```bash
# Setup development environment
ghostctl dev setup

# Configure Neovim
ghostctl nvim health && ghostctl nvim setup-lsp

# Setup shell
ghostctl shell zsh && ghostctl shell ohmyzsh
```

### Infrastructure Management
```bash
# Deploy infrastructure
ghostctl infrastructure terraform plan
ghostctl infrastructure terraform apply

# Run configuration management
ghostctl infrastructure ansible run

# Monitor cloud resources
ghostctl infrastructure cloud status
```

### Security Operations
```bash
# Security audit
ghostctl ssh audit && ghostctl gpg list

# Generate new keys
ghostctl ssh generate && ghostctl gpg generate

# Update certificates
ghostctl cert renew
```

---

**Note**: All commands support `--help` flag for detailed usage information.

For more information, visit: https://docs.ghostctl.dev
