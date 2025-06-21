# GhostCTL TODO & Development Roadmap

## 🎯 Current Status
**Build Status**: ✅ Compiles successfully  
**Dead Code Warnings**: 50 remaining (down from 121)  
**Tests**: 0 tests implemented  
**Production Ready**: ❌ Not yet

## 🚀 Immediate Priorities (High Priority)

### 1. Core Function Wiring
- [x] **Restic CLI Integration** - Fully wired up with backup subcommands
- [x] **Proxmox Script Generation** - Wired up with CLI handlers
- [ ] **Backup Management Functions** - Need to connect backup module functions
- [ ] **Btrfs Management** - Wire up snapshot functions to CLI
- [ ] **Arch Utilities** - Wire arch module functions to system commands

### 2. Dead Code Elimination (50 warnings remaining)
**Backup Module** (1 warning):
- [ ] `backup::cleanup::run()` - Wire to cleanup command

**CLI Module** (1 warning):
- [ ] `ssl_management_menu()` - Connect to SSL commands or remove

**Docker DevOps Module** (10 warnings):
- [ ] `container_security_scanning()` - Add to docker security menu
- [ ] `scan_local_image()` - Add to docker security commands
- [ ] `compose_stack_manager()` - Wire to docker compose commands
- [ ] `list_compose_stacks()` - Connect to docker list functionality
- [ ] `deploy_new_stack()` - Add to docker deployment commands
- [ ] `registry_tools()` - Add to docker registry management
- [ ] `kubernetes_tools()` - Add kubernetes subcommand module
- [ ] `generate_github_workflow()` - Add to CI/CD generators
- [ ] `docker_build_optimizer()` - Add to docker optimization menu
- [ ] `environment_manager()` - Add to docker environment commands
- [ ] `search_registry()` - Connect to docker search functionality

**Logging Module** (3 warnings):
- [ ] `GhostLogger::log_action()` - Integrate throughout command execution
- [ ] `GhostLogger::show_recent_logs()` - Add logs subcommand
- [ ] `execute_with_logging()` - Replace manual command execution
- [ ] `safe_command()` - Use for all system command execution

**Network Module** (1 warning):
- [ ] `ssh_key_management()` - Wire to security ssh commands

**Nginx Module** (6 warnings):
- [ ] `generate_config()` - Add to nginx config commands
- [ ] `ssl_management()` - Wire to nginx SSL functionality
- [ ] `proxy_config()` - Add proxy configuration commands
- [ ] `test_config()` - Add nginx test functionality
- [ ] `reload_service()` - Wire to nginx restart commands
- [ ] `get_certificate_paths()` - Use in SSL functions
- [ ] `ensure_custom_cert_structure()` - Integrate in SSL setup

**NVIDIA Module** (3 warnings):
- [ ] `install_proprietary()` - Wire to nvidia drivers commands
- [ ] `install_open()` - Add to nvidia driver options
- [ ] `install_open_beta()` - Add beta driver installation

**Neovim Module** (2 warnings):
- [ ] `get_aur_helper()` - Use in AUR package installation
- [ ] `install_aur_package()` - Wire to nvim package management

**Plugins Module** (3 warnings):
- [ ] `list_plugins()` - Add plugins list command
- [ ] `install_from_url()` - Add plugin installation from URL
- [ ] `run_user_script_menu()` - Wire to plugins menu
- [ ] `run_lua_script()` - Add Lua script execution

**Proxmox Enhanced Module** (9 warnings):
- [ ] `proxmox_backup_management()` - Add to pve backup commands
- [ ] `list_backup_jobs()` - Wire to pve status commands
- [ ] `run_backup_now()` - Add immediate backup functionality
- [ ] `schedule_backup()` - Wire to pve schedule commands
- [ ] `verify_backups()` - Add backup verification
- [ ] `proxmox_cluster_management()` - Add cluster management menu
- [ ] `show_cluster_status()` - Wire to pve status
- [ ] `join_cluster()` - Add cluster join functionality
- [ ] `add_cluster_node()` - Add cluster node management
- [ ] `remove_cluster_node()` - Add cluster node removal

**Restic Module** (1 warning):
- [ ] `setup()` - Wire to backup setup commands

**Scripts Module** (1 warning):
- [ ] `check_certificate_file()` - Use in SSL script generation

**Security Module** (6 warnings):
- [ ] `SecureCredentialManager::list_credentials()` - Add to security commands
- [ ] `SecureCredentialManager::delete_credential()` - Add credential management
- [ ] `GpgError` variants - Implement proper error handling
- [ ] `SshError` variants - Implement proper error handling
- [ ] `validate_hostname()` - Use in SSH configuration
- [ ] `validate_username()` - Use in SSH user management

## 🏗️ Architecture & Infrastructure

### 3. Testing Framework
- [ ] **Unit Tests** - Add comprehensive test suite (currently 0 tests)
- [ ] **Integration Tests** - Test CLI command integration
- [ ] **Mock Framework** - Mock external commands for testing
- [ ] **CI/CD Testing** - Automated testing in GitHub Actions

### 4. Code Quality
- [ ] **Clippy Compliance** - Fix all clippy warnings with `-D warnings`
- [ ] **Documentation** - Add rustdoc comments to public functions
- [ ] **Error Handling** - Implement proper error types and handling
- [ ] **Logging Integration** - Use GhostLogger throughout application

### 5. Configuration Management
- [ ] **Config File Integration** - Wire up GhostConfig save/load functions
- [ ] **Environment Variables** - Support for environment-based configuration
- [ ] **User Preferences** - Persistent user preference storage

## 🚢 Feature Development

### 6. Core Features
- [ ] **Complete Btrfs Integration** - Snapshot management, rollback functionality
- [ ] **Advanced Backup Scheduling** - Cron integration, automated backups
- [ ] **System Health Monitoring** - Regular system health checks
- [ ] **Package Management** - AUR helper integration, package cleanup

### 7. Developer Experience
- [ ] **Development Environment Setup** - Complete dev stack automation
- [ ] **Language Support** - Full Rust, Go, Python, Zig, Node.js support
- [ ] **Tool Integration** - LSP servers, debugging tools, profilers

### 8. Infrastructure Management
- [ ] **Docker Orchestration** - Complete container lifecycle management
- [ ] **Kubernetes Support** - K8s cluster management and deployment
- [ ] **Cloud Integration** - AWS, Azure, GCP tool integration
- [ ] **Proxmox Automation** - VM/LXC template generation and deployment

### 9. Security & Compliance
- [ ] **Security Auditing** - Automated security scanning and reporting
- [ ] **Key Management** - Complete SSH/GPG key lifecycle management
- [ ] **Credential Security** - Encrypted credential storage and rotation
- [ ] **Compliance Reporting** - Security compliance checks and reports

## 📋 Maintenance & Operations

### 10. System Integration
- [ ] **Systemd Integration** - Service management and monitoring
- [ ] **Network Management** - Advanced network configuration and diagnostics
- [ ] **SSL/TLS Management** - Automated certificate management and renewal
- [ ] **Nginx Optimization** - Performance tuning and configuration management

### 11. Monitoring & Observability
- [ ] **Metrics Collection** - System and application metrics
- [ ] **Log Aggregation** - Centralized logging and analysis
- [ ] **Alerting** - Proactive system monitoring and notifications
- [ ] **Performance Monitoring** - Resource usage and optimization recommendations

## 🎯 Release Milestones

### v0.7.1 - Core Stability
- [ ] Fix all dead code warnings
- [ ] Add basic test suite
- [ ] Complete function wiring for existing features
- [ ] Improve error handling

### v0.8.0 - Feature Completeness
- [ ] Complete all module integrations
- [ ] Advanced backup and recovery features
- [ ] Comprehensive system management
- [ ] Full developer environment automation

### v1.0.0 - Production Ready
- [ ] Comprehensive test coverage (>80%)
- [ ] Complete documentation
- [ ] Security audit and compliance
- [ ] Performance optimization
- [ ] Stable API

## 🐛 Known Issues
1. **Function Isolation** - Many functions defined but not integrated into CLI
2. **Error Propagation** - Inconsistent error handling across modules
3. **Test Coverage** - No automated testing currently implemented
4. **Configuration** - Config system defined but not fully utilized
5. **Logging** - Logging framework exists but not used consistently

## 📝 Development Guidelines
- **Wire up functions** instead of using `#[allow(dead_code)]`
- **Write tests** for all new functionality
- **Use the logging framework** for all operations
- **Follow CLI command patterns** for consistency
- **Document public APIs** with rustdoc
- **Handle errors properly** with Result types