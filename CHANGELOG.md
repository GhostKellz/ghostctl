# Changelog

All notable changes to GhostCTL will be documented in this file.

## [0.12.1] - 2026-06-24

### Changed

- **Hotfix dependency refresh**: updated the direct Cargo crates covered by the open Dependabot PRs: `crossterm`, `duct`, `gethostname`, `libloading`, `nix`, `sha1`, `sha2`, `sysinfo`, `toml`, and `which`; refreshed the lockfile with compatible transitive updates.
- **RustSec audit pass**: refreshed the advisory database with `cargo audit` and verified the updated lockfile has no reported vulnerabilities.
- **CI action pins**: updated the open GitHub Actions Dependabot group to newer SHA-pinned actions while keeping commit SHA pinning for supply-chain control. The workflow comments now document that Node 24-based actions require self-hosted runners at `2.327.1` or newer.
- **Release metadata**: bumped Cargo, Arch PKGBUILD, Debian changelog, Fedora spec, and release notes to `0.12.1`.
- **Documentation polish**: added `docs/advisories/` for hotfix notes, accepted risks, and resolved advisories; rebuilt `docs/README.md` into a complete documentation index; added production-oriented Mermaid diagrams for the documentation map, command architecture, installation/release path, dependency audit flow, CI workflow audit flow, security command surface, advisory flow, and support bundle lifecycle.

### Fixed

- **SHA digest rendering**: restored lowercase hex rendering for `sha2` 0.11 digest outputs used by script verification flows.
- **Zsh setup failure handling**: replaced runtime panics during Zsh asset installation with clean error messages when `HOME` is unset or `git` cannot be executed.

### Verified

- `cargo fmt --check`
- `cargo check`
- `cargo test` (591 unit tests and 6 CLI regression tests)
- `cargo clippy --release -- -D warnings`
- `cargo build --release`
- `cargo audit`

## [0.12.0] - 2026-06-13

### Added

- **Dependency audit (`ghostctl audit cargo|node|deps`)**: scans locked dependencies against the [OSV.dev](https://osv.dev) vulnerability database. Lockfiles are parsed natively (no package-manager binary invoked): `Cargo.lock` (crates.io), and `package-lock.json` / `yarn.lock` / `pnpm-lock.yaml` / `bun.lock` (npm). `audit cargo` and `audit node` target a single ecosystem; `audit deps` auto-detects both. Findings report severity (from CVSS where available), the fixed version, and the advisory URL; `--json` emits machine-readable output and exits non-zero on High/Critical. No new dependencies were added — `cargo audit` stays clean ([docs](docs/security/dependency-audit.md))
- **CI/CD workflow audit (`ghostctl audit ci`)**: offline auditor for GitHub Actions (`.github/workflows/*.yml`) and GitLab CI (`.gitlab-ci.yml`). Flags outdated/end-of-life action major versions against a curated policy table, unpinned `@main`/`@master` action refs, deprecated runner commands (`::set-output`, `::save-state`, `::set-env`, `::add-path`), and GitLab `only`/`except`/`type` deprecations plus unpinned container images; `--json` supported ([docs](docs/security/ci-workflow-audit.md))
- **JavaScript toolchain doctor (`ghostctl dev js doctor`)**: reports installed JS runtimes (Node, Bun, Deno) and package managers (npm, pnpm, yarn, bun) with versions, and detects the project's package manager from its lockfile, pointing at `audit node` for follow-up ([docs](docs/development/javascript.md))
- **GitLab (`ghostctl gitlab`)**: client for self-hosted GitLab and gitlab.com. Read-only checks: `status` (connectivity + auth), `ci-lint` (validate a CI file via the CI Lint API), `pipelines` (recent pipeline status), `pipeline <id>` (one pipeline + its jobs grouped by stage), `trace <job>` (print a job's log), `runners` (CI runners, project-scoped when configured), `mrs`/`mr` (open merge requests), and `projects` (member projects, for discovering the `[gitlab].project` value). Pipeline write actions — `run [ref]` (trigger, defaulting to the project's default branch), `retry <id>`, and `cancel <id>` — honor the global `--dry-run` (print intent, no request) and `--yes` flags and otherwise prompt before acting. The access token resolves from `GITLAB_TOKEN`, then `GHOSTCTL_GITLAB_TOKEN`, then `[gitlab].token`; read-only calls work with `read_api`, writes need the `api` scope; configured under `[gitlab]` ([docs](docs/gitlab/gitlab.md))

### Changed

- **Documentation**: added `docs/security/dependency-audit.md`, `docs/security/ci-workflow-audit.md`, `docs/development/javascript.md`, and `docs/gitlab/`; updated `docs/README.md` topic index and regenerated `docs/reference/COMMANDS.md`
- **Man page**: `man/ghostctl.1` documents the new `audit cargo|node|deps|ci`, `dev js`, and `gitlab` commands
- **Config surface**: `GhostConfig` gains an optional `[gitlab]` section (`url`, `token`, `project`, `timeout_secs`)
- **Packaging**: PKGBUILD, debian/changelog, and fedora spec bumped to 0.12.0

## [0.11.0] - 2026-06-12

### Added

- **Monitoring (`ghostctl monitor`, alias `mon`)**: client for a Prometheus / Loki / Alertmanager / Grafana stack with `health`, `targets`, `alerts`, `logs`, `tail`, `query`, `reload`, and `datasources` subcommands; configured under `[monitor]` ([docs](docs/monitor/monitoring.md))
- **Local AI (`ghostctl ai`)**: Ollama management with `status`, `models`, `pull`, `rm`, `show`, `ctx-check`, `run`, `ps`, and a `hermes` agent passthrough; configured under `[ai]` ([docs](docs/ai/ollama.md))
  - `ai run` now accepts per-request tuning flags (`--ctx`, `--temp`, `--num-predict`, `--seed`) that map to the Ollama `options` object
  - **`ai tune <show|recommend|apply>`**: inspects and applies the Ollama systemd override (`/etc/systemd/system/ollama.service.d/override.conf`); `recommend` is VRAM-aware via `nvidia-smi`, and `apply` *merges* recommended keys into the existing override (preserving unmanaged keys like `OLLAMA_MODELS`/`OLLAMA_HOST`) before a `daemon-reload` + `ollama` restart (sudo; honors `--dry-run`/`--yes`/`--plain`)
- **OpenShell (`ghostctl openshell`)**: `doctor` readiness checks (binary, docker daemon, gateway reachability, gateway registration) plus thin passthroughs to the `openshell` CLI for `status`, `gateway`, `sandbox`, and `policy`; configured under `[openshell]` ([docs](docs/openshell/openshell.md))
- **Config command (`ghostctl config`)**: exposes ghostctl's own settings file (`~/.config/ghostctl/config.toml`) with `show` (print all resolved sections), `edit` (open in `$EDITOR`), `path` (print the file location), and `reset` (regenerate defaults)
- **CrowdSec (`ghostctl crowdsec`)**: threat-feed inspection, LAPI Prometheus metrics summary, `cscli` passthrough, and DNS resolver/DNSSEC checks; configured under `[crowdsec]` ([docs](docs/security/crowdsec.md))
- **OBS / Wayland screencapture (`ghostctl obs`)**: detects session and compositor, installs and enables the correct `xdg-desktop-portal` backend, sets up the OBS virtual camera via `v4l2loopback` (with `--persist`), verifies NVIDIA NVENC, and tests the Wayland ScreenCast portal; configured under `[obs]` ([docs](docs/obs/wayland-screencapture.md))
- **Package audit (`ghostctl audit`)**: cross-references installed packages against the Arch Security Tracker (with `vercmp` confirmation) via `cve`; heuristically scans AUR/foreign PKGBUILDs **and their `.install` hooks** for remote-payload patterns via `aur`/`pkgbuild`; adds `registry-install`/`registry-install-js` scan rules that flag pulling a *named* package from npm/bun/pip/cargo/go/gem during build (the `atomic-lockfile`/`js-digest` supply-chain vector); adds `ioc` to match an external compromise feed of package names against installed packages and the historical `pacman.log` (incl. `.gz`/`.xz`/`.zst`/`.bz2` rotations); plus a `summary` overview; configured under `[audit]` ([docs](docs/security/package-audit.md))

### Changed

- **Documentation**: added `docs/monitor/`, `docs/ai/`, `docs/obs/`, `docs/openshell/`, and grouped the new security docs under `docs/security/` (`package-audit.md`, `crowdsec.md`); updated `docs/README.md` topic index and regenerated `docs/reference/COMMANDS.md`
- **Man page**: `man/ghostctl.1` documents the `monitor`, `ai`, `crowdsec`, `obs`, `audit`, and `openshell` commands
- **Config surface**: `ghostctl config show` now reports Monitor, AI, CrowdSec, OBS, Audit, and OpenShell sections; `[audit]` gains `ioc_feed` (path or URL) and `pacman_log_glob` keys
- **Packaging**: PKGBUILD, debian/changelog, and fedora spec bumped to 0.11.0 with optional dependencies for the new modules

### Security

- **Audit hardening (inspired by the June 2026 `atomic-lockfile`/`js-digest` AUR campaign)**: `audit aur`/`audit pkgbuild` now also scan a package's `.install` hook (which runs as root via pacman), not just the PKGBUILD
- **Build-time registry-install detection**: new `registry-install-js` (HIGH) and `registry-install` (MED) scan rules flag pulling a *named* package from npm/bun/pnpm/yarn or pip/cargo/go/gem during build; a bare `npm install` (declared dependencies) is not flagged
- **IOC feed matching**: new `audit ioc --feed <path|url>` cross-references an external list of suspect package names against currently-installed foreign packages and the historical `pacman.log` (including `.gz`/`.xz`/`.zst`/`.bz2` rotations), surfacing installed-then-removed packages; the feed is user-supplied so no campaign-specific data is baked into the binary

## [0.10.0] - 2026-05-15

### Added

- **Support command surface**: `ghostctl support doctor`, `ghostctl support paths`, `ghostctl support logs`, `ghostctl support bundle` with `--redact-paths`, `--gzip`, `--tarball`, and `--log-tail` options
- **Domain-specific diagnostics**: `support doctor` shows conditional sections for Docker, Proxmox, VFIO/IOMMU, NVIDIA, Storage, Networking, and UEFI when relevant tools are detected
- **Shell completions generation**: `ghostctl completion <bash|zsh|fish>` for runtime completion generation
- **Completion installer**: `scripts/install-completions.sh` auto-detects shell and installs completions to system directories
- **Man page**: `man/ghostctl.1` installed by all package recipes
- **Command reference generation**: `ghostctl docs generate [-o PATH]` introspects the clap command tree and outputs markdown
- **Issue templates**: `.github/ISSUE_TEMPLATE/` with diagnostic-first bug report and feature request forms
- **Regression test suite**: `ghostctl/tests/cli_regressions.rs` covering completions, support paths, doctor, and bundle formats
- **Makefile**: build, install, test, fmt, clippy, audit, dev-cycle, release, completions, package-arch, clean, help targets
- **Support bundle formats**: plain text, gzip-compressed, and tar.gz archive with embedded metadata
- **Redaction**: home paths, usernames, hostnames, IPv4, MAC addresses, PCI IDs, serial identifiers in support bundles
- **GitHub issue templates**: diagnostic-first bug reports requiring support bundles, structured feature requests

### Changed

- **XDG state directory**: Activity logs and support data now use `$XDG_STATE_HOME/ghostctl/` (typically `~/.local/state/ghostctl/`) while still reading legacy XDG data history if present
- **Package recipes overhauled**: PKGBUILD, debian/changelog, fedora spec updated to v0.10.0 with shell completions, man page, and documentation installation
- **Release archive**: now includes CHANGELOG.md, man page, and generated shell completions; removed references to non-existent root-level COMMANDS.md and DOCS.md
- **Documentation links**: release body now points to `docs/reference/COMMANDS.md` and `docs/README.md`
- **CLI help text normalized**: all top-level and subcommand help strings now use consistent imperative verb phrases
- **Docker security checks**: replaced 8 hardcoded stub functions with real checks using `docker inspect` and `docker ps` for non-root containers, read-only filesystems, resource limits, security options, trusted registries, secrets exposure, image sizes, and update freshness
- **nftables path handling**: backup/export paths migrated from hardcoded `/tmp` and `/var/lib/ghostctl` to XDG state and cache directories via `dirs` crate
- **Gaming temp paths**: Proton-GE downloads and Wine tool scripts now use `tempfile::TempDir` instead of hardcoded `/tmp`
- **CLI handler honesty**: bluetooth, wifi, and sysctl subcommands now inform the user they are launching the interactive menu instead of silently ignoring arguments
- **Homelab CLI trimmed**: removed `monitoring` subcommand from `--help` (still available via interactive menu)

### Fixed

- **FIXME sentinel in recovery.rs**: replaced `"FIXME"` fallback with `"unknown"` for missing PARTUUID in UEFI boot entry generation
- **Docker cleanup logic bug**: fixed contradictory condition in `image_cleanup()` that could never match MB-sized images; replaced with proper `parse_image_size_mb()` parser
- **TODO markers removed**: replaced 11 user-facing "TODO" messages with professional "not yet available" wording across lsp, devops, network, nvim, and systemd modules

### Removed

- **Dead code**: deleted orphaned `ghostctl/src/devops.rs` placeholder

### Security

- Dependency audit carried forward from v0.9.8 (`cargo audit` clean)
- **Shell injection hardening**: replaced ~21 `sh -c` + `format!()` patterns in wine_tools.rs with direct `Command::new()` calls, eliminating shell interpretation of user input
- **nftables input validation**: added IP/CIDR, port, interface, and identifier validators to the interactive rule builder
- **Error visibility**: replaced 27 silent `.ok()` calls on critical operations (backups, file writes, command execution) with `eprintln!` error reporting in wine_tools.rs

## [0.9.9] - 2026-04-04

### Added

- **UEFI Secure Boot support**: Added `ghostctl uefi` for preparing Windows 11 VM firmware with pre-enrolled Secure Boot keys
  - `ghostctl uefi status` checks OVMF firmware files and `virt-fw-vars` availability
  - `ghostctl uefi enroll -o <file>` creates an OVMF VARS file using the supported Red Hat plus Microsoft key enrollment path
  - `ghostctl uefi verify <file>` performs a best-effort check for `PK`, `KEK`, `db`, and Secure Boot state in a generated VARS file
  - output includes the required libvirt `<loader>` and `<nvram>` snippets for VM configuration
- **SECURITY.md**: Security policy with vulnerability reporting and dependency auditing

### Documentation

#### Structure Overhaul
Reorganized documentation from monolithic files to topic-based directories with proper breakout files:

- **docs/btrfs/**: snapshots.md, snapper.md, maintenance.md, recovery.md
- **docs/storage/**: New directory for S3/MinIO, local, and network storage docs
- **docs/networking/**: Added dns.md, netcat.md; expanded firewall.md
- **docs/proxmox/**: Added backup.md, storage.md, templates.md
- **docs/docker/**: containers.md, compose.md, security.md
- **docs/security/**: ssh.md, gpg.md
- **docs/development/**: neovim.md, terminals.md
- **docs/virtualization/**: gpu-passthrough.md
- **docs/arch/**: aur.md, pacman.md, mirrors.md, troubleshooting.md
- **docs/uefi/README.md**: UEFI Secure Boot setup guide
- **docs/index.md**: Updated with links to all topic directories

#### Fixes
- **README.md**: Fixed non-existent CLI flags (`--dev`, `--docker`, `--pve`, `--system`), corrected `ghostctl proxmox menu` to `ghostctl pve menu`, fixed typos
- **docs/deployment/INSTALL.md**: Updated install URLs to use `ghostctl.cktech.sh`
- **docs/reference/COMMANDS.md**: Removed version clutter, fixed inaccurate command references

### Changed

#### Project Organization
- **dev/tests/**: Moved test scripts from `test_scripts/` to `dev/tests/`
- **packaging/**: Reorganized to `packaging/arch/`, `packaging/fedora/`, `packaging/debian/`

### Removed

- **demo_working_features.py**: Outdated v1.0.0 demo script
- **test_scripts/**: Moved to dev/tests/
- **install/**: Duplicate install directory

### Dependencies

- Uses `virt-firmware` package (`virt-fw-vars`) for key enrollment
- Uses `which` crate for tool detection

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
- `install.sh` - Checksum verification
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

For command reference, see [docs/reference/COMMANDS.md](docs/reference/COMMANDS.md).
For full documentation, see [docs/index.md](docs/index.md).
