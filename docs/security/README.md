# Security Management

GhostCTL provides tools for SSH, GPG, credential management, code signing,
package auditing, and threat intelligence.

## Documentation

- [SSH](ssh.md) - SSH key generation and management
- [GPG](gpg.md) - GPG key management and operations
- [Code Signing](../signing/README.md) - Azure Key Vault-backed code signing
- [Package Audit](package-audit.md) - CVE checks and AUR PKGBUILD scanning
- [CrowdSec](crowdsec.md) - Threat feed, LAPI metrics, DNS checks

## Quick Commands

```bash
ghostctl security menu            # Security management menu
ghostctl ssh                      # Interactive SSH menu
ghostctl gpg                      # Interactive GPG menu
ghostctl gpg list                 # List GPG keys (non-interactive)
ghostctl sign status              # Check signing readiness
```

## SSH Key Management

```bash
ghostctl ssh                      # Interactive SSH menu
ghostctl ssh generate             # Generate new key pair
ghostctl ssh list                 # List SSH keys
ghostctl ssh copy-id user@host    # Copy key to remote host
ghostctl ssh config               # SSH config management
```

## GPG Key Management

```bash
ghostctl gpg                      # Interactive GPG menu
ghostctl gpg list                 # List all keys
ghostctl gpg info <KEY_ID>        # Show key details
ghostctl gpg export <KEY_ID>      # Export public key
ghostctl gpg renew <KEY_ID>       # Extend key expiration
```

## Code Signing

```bash
ghostctl sign config --init       # Configure Azure Key Vault signing
ghostctl sign file <FILE>         # Sign a file
ghostctl sign verify <FILE>       # Verify a signature
ghostctl sign export-key          # Export signing public key
ghostctl sign list-keys           # List vault certificates
ghostctl sign status              # Check dependencies
```

## Package Audit

```bash
ghostctl audit summary               # Installed / foreign / orphan counts
ghostctl audit cve                   # Installed packages vs. Arch Security Tracker
ghostctl audit aur                   # Scan installed AUR/foreign PKGBUILDs
ghostctl audit pkgbuild ./PKGBUILD   # Scan a local PKGBUILD
ghostctl audit pkgbuild yay          # Fetch + scan an AUR package's PKGBUILD
```

## CrowdSec & Threat Intel

```bash
ghostctl crowdsec feed               # Inspect the public threat feed
ghostctl crowdsec metrics            # CrowdSec LAPI Prometheus metrics summary
ghostctl crowdsec cli ...            # Passthrough to local cscli (LAPI host)
ghostctl crowdsec dns                # DNS resolver reachability and DNSSEC
```

## Credential Storage

The credential management system provides secure storage for sensitive data:
- Encrypted credential store
- Interactive credential management
- Support for various credential types

## Features

- SSH key generation (ED25519, RSA)
- SSH agent integration
- SSH config file management
- GPG key generation, import, export, trust management
- Key renewal and keyserver operations
- Azure Key Vault code signing (PE, RPM, DEB, Pacman)
- Secure credential storage
