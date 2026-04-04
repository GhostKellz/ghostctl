# Arch Linux Management

GhostCTL provides comprehensive Arch Linux system management.

## Documentation

- [Pacman](pacman.md) - Package management, fixes, cache
- [AUR](aur.md) - AUR helpers and packages
- [Mirrors](mirrors.md) - Mirror optimization with reflector
- [Troubleshooting](troubleshooting.md) - Common issues and fixes

## Quick Commands

```bash
ghostctl arch fix                 # Fix common issues
ghostctl arch clean <target>      # Clean: orphans, mirrors, locks, all
ghostctl arch bouncer [target]    # Recovery: pacman, keyring, mirrors, all
ghostctl arch aur                 # AUR management
ghostctl arch mirrors             # Update mirrors
ghostctl arch health              # System health check
ghostctl arch optimize            # Optimize zram/zswap
```

## System Maintenance

```bash
# Full system update
sudo pacman -Syu

# Clean orphans
ghostctl arch clean orphans

# Fix issues and update
ghostctl arch bouncer all
```
