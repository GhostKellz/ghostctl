# Security Management

GhostCTL provides tools for SSH, GPG, and credential management.

## Documentation

- [SSH](ssh.md) - SSH key generation and management
- [GPG](gpg.md) - GPG key management and operations

## Quick Commands

```bash
ghostctl security menu            # Security management menu
ghostctl ssh                      # Interactive SSH menu
ghostctl gpg                      # Interactive GPG menu
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
- GPG key generation and management
- Secure credential storage
