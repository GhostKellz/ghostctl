# Changelog

All notable changes to GhostCTL will be documented in this file.

## [0.9.8] - 2026-04-03

### Security

- **Dependency audit**: Fixed 1 critical vulnerability and removed 6 unmaintained/unsound transitive dependencies
  - `bytes` 1.10.1 → 1.11.1 (integer overflow fix)
  - `indicatif` 0.17 → 0.18 (removes unmaintained `number_prefix`)
  - `ratatui` 0.28 → 0.30 (removes unsound `lru` 0.12.5, unmaintained `paste`)
  - `reqwest` 0.11 → 0.13 (modern TLS stack, removes unmaintained `rustls-pemfile`)
  - `keyring` now uses `linux-no-secret-service` feature (kernel keyutils instead of D-Bus)

### Changed

- **Keyring backend**: Switched from D-Bus secret-service to kernel-level keyutils for credential storage (more secure, fewer dependencies)

### Build

- All changes verified:
  - `cargo audit` (0 advisories)
  - `cargo clippy --all-features` (0 warnings)
  - `cargo test --all-features` (370 tests passed)
  - `cargo build --release --all-features`

## [0.9.7] - 2026-03-29

### 🔧 CI/CD Improvements

- **rust-toolchain action**: Fixed CI workflow using incorrect action name (`rust-action` → `rust-toolchain`)
- **rust-cache update**: Updated `Swatinem/rust-cache` from v2.7.3 to v2.9.1 for Rust 2024 Cargo.lock format support

### 🧹 Code Quality

- **Clippy compliance**: Resolved all 155+ clippy warnings with `-D warnings` flag
  - Converted `loop`/`match` patterns to idiomatic `while let` loops (8 instances)
  - Replaced `if let Err(_)` with `.is_err()` for cleaner error checks
  - Fixed no-effect `replace()` call (replacing ':' with ':')
  - Used `strip_prefix()` instead of manual string slicing
  - Fixed struct field assignment outside initializer
  - Removed always-true `u32::MAX` comparison
- **Formatting**: Applied `cargo fmt` across entire codebase (52 files)

### 📦 Build

- All changes verified locally before push:
  - `cargo fmt --check` ✓
  - `cargo clippy --release -- -D warnings` ✓
  - `cargo build --release` ✓
  - `cargo test` (370 tests passed) ✓

## [0.9.6] - 2026-03-28

### 🛡️ Security Hardening

This release addresses findings from a comprehensive third-party security audit, significantly improving credential handling, supply-chain security, and runtime reliability.

#### Credential & Secret Handling
- **Docker Registry**: All registry authentication now uses masked password input and `--password-stdin` instead of command-line arguments
- **Proxmox Storage**: CIFS storage credentials use secure temp files with `--smbcredentials` flag instead of `--password` argv exposure
- **PVE Authentication**: Password prompts now use masked input with confirmation

#### Supply-Chain Security
- **CI Actions**: All GitHub Actions SHA-pinned to immutable commit hashes
- **Fork Isolation**: External PR builds run on GitHub-hosted runners, not self-hosted infrastructure
- **Installer Verification**: Both install scripts now verify SHA256 checksums before extraction
- **PKGBUILD**: Updated to use git source with tag verification for Arch Linux builds
- **Regression Guards**: CI now blocks reintroduction of forbidden patterns (docker `-p`, chmod 666, shell injection)

#### Shell Injection Prevention
- **Networking Module**: Removed all `sh -c` shell interpolation from `advanced_firewall.rs` and `troubleshoot.rs`
- **Direct Execution**: All commands now use argument arrays instead of shell strings
- **Input Validation**: Added namespace and profile name validation to prevent path traversal

#### Runtime Reliability
- **Firewall Module**: Replaced 120+ `unwrap()` calls with graceful `interact_opt()` error handling
- **Command Results**: Critical operations in PVE, Docker, and networking modules now check and report command failures
- **Temp File Safety**: Uses `NamedTempFile` for secure, unpredictable temp file creation

### 🔧 Infrastructure Improvements

- **Docker Socket**: Replaced insecure `chmod 666` with proper group-based access (`chgrp docker`)
- **Script Safety**: Default `require_checksum: true` for Proxmox script execution
- **Cache Security**: Script cache uses `~/.cache` with 0700 permissions instead of `/tmp` fallback

### 📋 Files Changed

- `src/docker/registry.rs` - Secure Docker login helper
- `src/proxmox/storage_migration.rs` - Credentials file handling
- `src/proxmox/firewall_automation.rs` - Profile name validation
- `src/proxmox/script_safety.rs` - Hardened defaults
- `src/networking/firewall.rs` - Graceful error handling
- `src/networking/advanced_firewall.rs` - Shell injection removal
- `src/networking/troubleshoot.rs` - Direct sysfs reads
- `src/networking/safe_commands.rs` - Secure temp files
- `src/nvidia/container.rs` - Group-based Docker access
- `src/pve.rs` - Masked password input
- `.github/workflows/*.yml` - SHA-pinned actions
- `install.sh`, `install/install.sh` - Checksum verification
- `PKGBUILD` - Git source with tag verification

## [1.0.0] - 2025-08-08

### 🚀 **MAJOR RELEASE: Professional Homelab & Enterprise Infrastructure Management**

This landmark release transforms GhostCTL into a comprehensive infrastructure management platform with enterprise-grade Proxmox VE capabilities, distributed object storage, and advanced container orchestration.

#### 🏥 **Proxmox VE Management Revolution**

- **NEW**: **Template Management System**
  - Complete lifecycle management for LXC containers, VM ISOs, and appliance templates
  - Upload/download custom templates with integrity verification
  - Template customization with hooks and scripts
  - Template optimization and storage analytics
  - Built-in templates for popular Linux distributions and applications

- **NEW**: **Storage Migration Tools** 
  - VM/Container storage migration with live operations support
  - Bulk migration operations with progress tracking
  - Storage pool management and optimization
  - Cross-storage replication and synchronization
  - Storage performance analysis and recommendations

- **NEW**: **Backup Rotation & Pruning System**
  - Automated backup job management with cron scheduling
  - Advanced retention policies with customizable templates
  - Intelligent pruning with dry-run capabilities and verification
  - Backup integrity checking and restoration testing
  - Storage impact analysis and optimization recommendations
  - Disaster recovery planning and monitoring

- **NEW**: **Firewall Automation with Security Scanning**
  - Advanced firewall rule management with interactive wizards
  - Security profiles and templates (web server, mail server, database, etc.)
  - **gscan Integration**: Network security scanning using your Rust port scanner
  - Automated rule generation based on scan results
  - Firewall configuration backup and monitoring
  - Threat detection and response workflows

#### ☁️ **Object Storage & MinIO Cluster Management**

- **NEW**: **MinIO Distributed Cluster Support**
  - Multi-node cluster setup and management
  - Erasure code configuration with automatic optimization
  - Node health monitoring and maintenance mode
  - Cluster rebalancing and data distribution analysis
  - Performance metrics and Prometheus integration

- **NEW**: **MinIO Performance & Operations**
  - System-level performance tuning (storage, network, memory)
  - Multi-tenant setup with user and policy management
  - Cross-cluster replication and disaster recovery
  - Backup and restoration workflows
  - S3-compatible operations with AWS CLI integration

#### 🐳 **Docker & Container Platform Enhancements**

- **NEW**: **Docker Registry Mirror System**
  - Local registry deployment with Docker Hub mirroring
  - Corporate proxy support and authentication
  - Registry health monitoring and maintenance
  - Multi-registry configuration and load balancing
  - SSL/TLS certificate management for registries

- **NEW**: **Advanced Container Cleanup Tools**
  - Intelligent cleanup for images, volumes, networks, and containers
  - Safety checks and confirmation workflows
  - Storage analysis and optimization recommendations
  - Automated cleanup scheduling with systemd timers

#### 🛡️ **Network Storage & Infrastructure**

- **NEW**: **Network Storage Management**
  - NFS server/client configuration with performance tuning
  - CIFS/SMB mount management and optimization
  - Network storage troubleshooting and diagnostics
  - Performance analysis and optimization recommendations

- **NEW**: **Local Storage Management**
  - Disk health monitoring with SMART analysis
  - Filesystem tools and optimization
  - RAID management and monitoring
  - Storage performance benchmarking

### 🔧 **Technical Improvements**

- **ENHANCED**: Modular architecture with improved separation of concerns
- **ENHANCED**: Robust error handling and user feedback throughout
- **ENHANCED**: Interactive menu system with comprehensive options
- **ENHANCED**: Configuration management and persistence
- **ENHANCED**: Integration testing and validation workflows

### 📋 **New Command Structure**

#### Proxmox VE Management
```bash
ghostctl pve menu                    # Enhanced PVE management hub
ghostctl pve templates               # Template lifecycle management  
ghostctl pve storage-migration       # Storage migration tools
ghostctl pve backup-rotation         # Backup management & pruning
ghostctl pve firewall               # Firewall automation with gscan
```

#### Object Storage Management
```bash
ghostctl storage s3                 # MinIO/S3 cluster management
ghostctl s3 cluster                 # Distributed cluster operations
ghostctl s3 performance             # Performance tuning tools
```

#### Container & Registry Management  
```bash
ghostctl docker registry-mirror     # Registry mirror setup
ghostctl docker cleanup             # Advanced cleanup tools
ghostctl docker registry            # Registry management
```

### 🎯 **Integration Features**

- **gscan Integration**: Seamless integration with your Rust port scanner for security analysis
- **AWS CLI Compatibility**: Full S3 compatibility with existing AWS workflows  
- **Proxmox API Integration**: Native PVE API usage for all operations
- **Systemd Integration**: Timer-based automation and service management

### 📊 **Performance & Reliability**

- **Binary Size**: Optimized to 7.9MB (stripped release build)
- **Memory Usage**: Efficient Rust implementation with minimal overhead
- **Error Handling**: Comprehensive error handling with graceful degradation
- **Safety Checks**: Multiple confirmation layers for destructive operations

### 🏢 **Enterprise Readiness**

- **Production Testing**: Tested in homelab and enterprise environments
- **Comprehensive Logging**: Detailed logging for all operations
- **Backup & Recovery**: Built-in backup verification and recovery testing
- **Security First**: Security-focused design with privilege validation
- **Documentation**: Complete command reference and usage guides

This release represents a major milestone in GhostCTL's evolution from a system administration tool to a complete infrastructure management platform suitable for professional homelabs, SMBs, and enterprise environments.

## [0.8.0] - 2025-06-21
- **NEW**: Complete shortnames for network and security, seperately calling ssh, gpg, dns outside of security and network is allowed.



## [0.7.0] - 2025-06-20

### 🎯 Major Features & Enhancements

#### 🐧 Arch Linux System Management
- **NEW**: Complete dotfiles management system with Git integration
- **NEW**: System health monitoring and maintenance automation
- **NEW**: Swap and zram configuration with intelligent recommendations
- **NEW**: AUR helper management and optimization
- **NEW**: Boot and kernel management (linux-tkg, cachy, etc.)
- **NEW**: Performance tuning and system optimization
- **ENHANCED**: Expanded arch fix functionality with comprehensive maintenance

#### 🎮 NVIDIA Complete Suite
- **NEW**: Multi-driver support (proprietary/open/open-beta from AUR)
- **NEW**: NVIDIA Container Runtime setup for Docker/Podman GPU acceleration
- **NEW**: GPU passthrough configuration for VMs with VFIO/IOMMU setup
- **NEW**: Driver status monitoring and diagnostics
- **NEW**: Performance optimization and GPU information display
- **ENHANCED**: Complete NVIDIA ecosystem management

#### 📝 Development Environment Revolution
- **NEW**: Mason.nvim integration for zero-config language server setup
- **NEW**: Language-specific development environments (Rust, Python, Go, Zig, Web, DevOps)
- **NEW**: Automated LSP/DAP/Tool management and updates
- **NEW**: Mason health checks and diagnostics
- **ENHANCED**: Complete Neovim development workflow

#### 💻 Terminal Ecosystem Expansion
- **NEW**: Full Alacritty support with theme management and performance tuning
- **NEW**: Enhanced WezTerm configuration and optimization
- **NEW**: Nerd Font management and installation
- **NEW**: Terminal performance optimization features
- **ENHANCED**: Ghostty configuration with advanced customization

#### 🔐 Security & Infrastructure Management
- **NEW**: Comprehensive SSH key management with secure generation
- **NEW**: GPG key management and encryption workflows
- **NEW**: Security auditing and vulnerability assessment
- **NEW**: Automated backup systems with Restic integration
- **NEW**: Backup verification and integrity checking
- **NEW**: System recovery and restoration tools

#### 🗃️ Filesystem & Storage
- **NEW**: Complete Btrfs snapshot management
- **NEW**: Filesystem recovery and rollback capabilities
- **NEW**: Storage optimization and cleanup automation
- **NEW**: Disk space monitoring and alerting

#### 🌐 Network & Cloud Integration
- **NEW**: Network diagnostics and connectivity testing
- **NEW**: Network configuration management
- **NEW**: Cloud provider tool integration (AWS, Azure, GCP)
- **NEW**: Infrastructure automation and deployment

#### ⚙️ System Services & Management
- **NEW**: Systemd service management interface
- **NEW**: Service status monitoring and control
- **NEW**: Automated service configuration

### 🔧 Technical Improvements

#### CLI Architecture Overhaul
- **BREAKING**: Replaced `--help` and `--version` flags with `ghostctl help` and `ghostctl version` subcommands
- **NEW**: Complete CLI subcommand coverage for all modules
- **NEW**: Consistent command structure across all features
- **ENHANCED**: Better error handling and user feedback

#### Code Quality & Structure
- **NEW**: Comprehensive error handling with proper Result types
- **NEW**: Modular architecture with clear separation of concerns
- **NEW**: Extensive documentation and inline comments
- **ENHANCED**: Code organization and maintainability

### 📚 Documentation & User Experience

#### Documentation Rewrite
- **NEW**: Complete COMMANDS.md rewrite with all v0.7.0 features
- **NEW**: Comprehensive feature documentation
- **NEW**: Usage examples and best practices
- **NEW**: Command reference with examples

#### User Interface Improvements
- **NEW**: Consistent emoji-based navigation
- **NEW**: Improved menu organization and flow
- **NEW**: Better status reporting and progress indicators
- **ENHANCED**: Overall user experience and accessibility

### 🛠️ Module Enhancements

#### Arch Linux (`ghostctl arch`)
```bash
ghostctl arch health              # System health and maintenance
ghostctl arch swap                # Swap and zram management  
ghostctl arch dotfiles            # Dotfiles management
ghostctl arch aur                 # AUR helper management
ghostctl arch boot                # Boot and kernel management
ghostctl arch perf                # Performance tuning
```

#### NVIDIA Management (`ghostctl nvidia`)
```bash
ghostctl nvidia drivers           # Driver management (all types)
ghostctl nvidia container         # Container GPU support
ghostctl nvidia passthrough       # GPU passthrough for VMs
ghostctl nvidia optimize          # Performance optimization
```

#### Development (`ghostctl dev` + `ghostctl nvim mason`)
```bash
ghostctl nvim mason              # Mason.nvim LSP/DAP management
# Language environments: Rust, Python, Go, Zig, Web, DevOps, Documentation
```

#### Terminal (`ghostctl terminal`)
```bash
# Full Alacritty support with themes, fonts, performance tuning
# Enhanced Ghostty and WezTerm configurations
# Nerd Font management and installation
```

#### Security (`ghostctl security`)
```bash
ghostctl ssh             # SSH key management
ghostctl gpg             # GPG key management
ghostctl security audit  # Security auditing
```

#### Backup & Recovery (`ghostctl backup` + `ghostctl restore`)
```bash
ghostctl backup setup             # Automated backup configuration
ghostctl backup schedule          # Backup scheduling
ghostctl backup verify            # Integrity verification
ghostctl restore restic           # Restic restoration
ghostctl restore btrfs            # Btrfs snapshot rollback
```

#### Filesystem (`ghostctl btrfs`)
```bash
ghostctl btrfs snapshot           # Create snapshots
ghostctl btrfs list               # List snapshots
ghostctl btrfs restore            # Restore snapshots
```

#### Network & Cloud (`ghostctl network` + `ghostctl cloud`)
```bash
ghostctl network status           # Network diagnostics
ghostctl network test             # Connectivity testing
ghostctl cloud aws                # AWS tools
ghostctl cloud azure              # Azure tools
ghostctl cloud gcp                # Google Cloud tools
```

#### System Services (`ghostctl systemd`)
```bash
ghostctl systemd status           # Service status
ghostctl systemd enable           # Enable services
ghostctl systemd restart          # Restart services
```

### 🐛 Bug Fixes
- Fixed SSH module compilation errors with proper Result types
- Resolved unused import warnings across codebase
- Fixed CLI argument parsing and command routing
- Corrected module interdependencies and function visibility
- Resolved lifetime and ownership issues in dotfiles management

### ⚡ Performance Improvements
- Optimized CLI command parsing and execution
- Reduced binary size through better dependency management
- Improved startup time with lazy loading
- Enhanced memory usage in large file operations

### 🔄 Breaking Changes
- **CLI Structure**: Removed `--help` and `--version` flags in favor of subcommands
- **Command Routing**: All commands now use consistent subcommand structure
- **Module Organization**: Some functions moved between modules for better organization

### 📦 Dependencies
- Updated clap for better CLI argument parsing
- Added chrono for timestamp handling in backups
- Enhanced dialoguer integration for better user interaction
- Improved dirs crate usage for cross-platform compatibility

### 🏗️ Infrastructure
- Comprehensive testing of all new CLI commands
- Improved build process with better error handling
- Enhanced development workflow with clearer module boundaries

### 🎉 What's Next (v0.8.0 Preview)
- Enhanced container orchestration with Kubernetes integration
- Advanced monitoring with Prometheus/Grafana automation
- AI-powered system optimization recommendations
- Advanced security scanning and vulnerability management
- Plugin system for community extensions

---

## [0.6.0] - 2024-12-20

### Added
- Basic CLI structure with major subsystems
- Arch Linux package management
- Docker container management
- Proxmox VE integration
- NVIDIA driver management basics
- Neovim setup automation
- SSL certificate management with acme.sh

### Enhanced
- Improved menu system organization
- Better error handling in critical operations
- Enhanced Docker integration

### Fixed
- Various stability issues
- Package installation reliability
- Configuration file handling

---

## [0.5.1] - 2024-12-19

### Fixed
- Major clippy warnings (needless_return, needless_borrows_for_generic_args)
- Self-hosted runner issues in CI/CD
- Release workflow stability

---

## [0.5.0] - 2024-12-18

### Added
- Initial public release
- Core system management functionality
- Basic homelab automation
- Multi-platform support foundation

---

### Legend
- 🎯 **Major Features**: Significant new functionality
- 🔧 **Technical**: Under-the-hood improvements
- 📚 **Documentation**: Documentation and user experience
- 🛠️ **Enhancements**: Improvements to existing features
- 🐛 **Bug Fixes**: Resolved issues
- ⚡ **Performance**: Speed and efficiency improvements
- 🔄 **Breaking**: Changes that may require user action
- 📦 **Dependencies**: Library and dependency updates
- 🏗️ **Infrastructure**: Build and development improvements

For detailed command usage, see [COMMANDS.md](COMMANDS.md).
For usage guides and examples, see [DOCS.md](DOCS.md).