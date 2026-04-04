# Pacman Management

## Commands

```bash
ghostctl arch fix                 # Fix common issues
ghostctl arch clean orphans       # Remove orphaned packages
ghostctl arch clean locks         # Clear database locks
ghostctl arch bouncer pacman      # Full pacman recovery
ghostctl arch bouncer keyring     # Fix keyring issues
```

## Common Fixes

### Database Lock
```bash
ghostctl arch clean locks
# or:
sudo rm /var/lib/pacman/db.lck
```

### Corrupted Database
```bash
ghostctl arch bouncer pacman
# or:
sudo rm -rf /var/lib/pacman/sync
sudo pacman -Syy
```

### Keyring Issues
```bash
ghostctl arch bouncer keyring
# or:
sudo pacman-key --init
sudo pacman-key --populate archlinux
sudo pacman -Sy archlinux-keyring
```

## Orphan Packages

```bash
# List orphans
pacman -Qdt

# Remove orphans
ghostctl arch clean orphans
# or:
sudo pacman -Rns $(pacman -Qdtq)
```

## Cache Management

```bash
# Remove old versions (keep 3)
sudo paccache -r

# Keep only latest
sudo paccache -rk1

# Clear all
sudo pacman -Scc
```

## Useful Queries

```bash
pacman -Ss keyword        # Search
pacman -Si package        # Package info
pacman -Q                 # List installed
pacman -Ql package        # List files
pacman -Qo /path/file     # Who owns file
pacman -Qe                # Explicitly installed
```
