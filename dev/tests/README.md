# GhostCTL Test Scripts

These test scripts are designed to safely verify all the new features added to GhostCTL without making any system modifications.

## 🧪 Test Scripts Overview

### `test_all_features.sh`
Comprehensive test suite that checks:
- Binary compilation and execution
- Core functionality access
- Dependencies and tools availability
- System permissions and directories
- Safety checks for dangerous patterns

### `test_gaming_features.sh`
Focuses on gaming-related features:
- Wine prefix management capabilities
- Lutris integration status
- Steam/Proton detection
- Gaming performance tools
- DXVK/VKD3D support
- Directory permissions and script storage

### `test_networking_features.sh`
Tests networking and firewall features:
- Firewall tools availability (iptables/nftables/ufw)
- Advanced networking tools
- Gaming network features
- QoS and traffic shaping support
- DDoS protection capabilities
- Template and backup systems

## 🚀 Usage

### On Your Backup VM (Arch Linux):

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd ghostctl
   ```

2. **Build the project:**
   ```bash
   cd ghostctl
   cargo build --release
   ```

3. **Make test scripts executable:**
   ```bash
   chmod +x dev/tests/*.sh
   ```

4. **Run the test suites:**
   ```bash
   # Run all tests
   ./dev/tests/test_all_features.sh

   # Test gaming features specifically
   ./dev/tests/test_gaming_features.sh

   # Test networking features specifically
   ./dev/tests/test_networking_features.sh
   ```

## 📊 Test Results Interpretation

### Status Indicators:
- ✅ **PASS**: Feature is working correctly
- ❌ **FAIL**: Critical error that prevents functionality
- ⚠️ **SKIP**: Feature unavailable due to missing dependencies (expected)

### Expected Results:
- Many **SKIP** results are normal - they indicate missing optional packages
- **FAIL** results should be investigated
- **PASS** results confirm the feature is ready to use

## 🎯 New Features Being Tested

### Gaming Features:
1. **Wine Prefix Management**
   - Create/delete/clone/backup prefixes
   - Game-specific templates (AAA, Classic, Indie)
   - Auto-detection of existing prefixes

2. **Lutris Integration**
   - Game library management
   - Wine runner management
   - Configuration import/export

3. **Advanced Wine Tools**
   - Winetricks deep integration
   - Automatic dependency resolution
   - Batch scripts and profiles

4. **Proton/DXVK Management**
   - Version management
   - Compatibility layer setup
   - Shader cache management

### Networking Features:
1. **Advanced nftables Management**
   - Rule builder GUI
   - Dynamic sets and rate limiting
   - Template library

2. **Gaming Network Optimization**
   - Port management for popular games
   - NAT type detection
   - QoS configuration

3. **Security Features**
   - DDoS protection setup
   - Port knocking configuration
   - Connection state analysis

4. **Backup/Restore System**
   - Automated rule backups
   - Safe rule testing sandbox
   - Migration tools (iptables → nftables)

## 🛡️ Safety Features

### What These Tests DON'T Do:
- ❌ Modify system firewall rules
- ❌ Create actual Wine prefixes
- ❌ Install or remove packages
- ❌ Change network configuration
- ❌ Modify system files

### What These Tests DO:
- ✅ Check tool availability
- ✅ Verify permissions
- ✅ Test directory creation
- ✅ Validate syntax
- ✅ Simulate dry-runs

## 🔧 Troubleshooting

### Common Issues:

1. **Compilation Errors:**
   ```bash
   # Check Rust version
   rustc --version

   # Update dependencies
   cargo update
   ```

2. **Permission Issues:**
   ```bash
   # Some tests require sudo access
   sudo -v
   ```

3. **Missing Dependencies:**
   ```bash
   # Install common gaming packages (Arch)
   sudo pacman -S wine winetricks lutris steam

   # Install networking tools
   sudo pacman -S nftables iptables-nft net-tools
   ```

## 📝 Test Reporting

After running tests, you can create a report:

```bash
# Run all tests and save output
./dev/tests/test_all_features.sh > test_results.txt 2>&1

# View summary
grep -E "(PASS|FAIL|SKIP)" test_results.txt | sort | uniq -c
```

## 🎮 Manual Testing After Automated Tests

Once automated tests pass, you can safely test interactive features:

1. **Wine Prefix Creation:**
   ```bash
   ./target/release/ghostctl
   # Navigate to: Gaming → Wine Prefix Management → Create New Prefix
   ```

2. **Firewall Rule Testing:**
   ```bash
   ./target/release/ghostctl
   # Navigate to: Networking → Advanced Firewall → Rule Testing Sandbox
   ```

3. **Gaming Optimization:**
   ```bash
   ./target/release/ghostctl
   # Navigate to: Gaming → Performance Optimization
   ```

Remember to test in the VM environment first before running on production systems!

## 🆘 Support

If tests fail or you encounter issues:

1. Check the detailed error messages
2. Verify dependencies are installed
3. Ensure proper permissions
4. Run individual test sections to isolate issues
5. Check the main GhostCTL documentation for troubleshooting

The test scripts are designed to be safe and informative - they'll tell you exactly what's working and what needs attention.