# Snapper Integration

## Overview

GhostCTL integrates with Snapper for automated snapshot management on Btrfs filesystems.

## Setup

### Initial Configuration
```bash
ghostctl btrfs snapper setup
```

This command:
1. Installs snapper if not present
2. Creates root config for `/`
3. Creates home config for `/home`

### Manual Setup
```bash
sudo snapper -c root create-config /
sudo snapper -c home create-config /home
```

## Commands

### List Configurations
```bash
ghostctl btrfs snapper list
```

### Edit Configuration
```bash
ghostctl btrfs snapper edit CONFIG
ghostctl btrfs snapper edit root
ghostctl btrfs snapper edit home
```

### Cleanup
```bash
ghostctl btrfs snapper cleanup
```

## Configuration Options

Edit `/etc/snapper/configs/root` or `/etc/snapper/configs/home`:

```ini
# Snapshot retention
TIMELINE_LIMIT_HOURLY="5"
TIMELINE_LIMIT_DAILY="7"
TIMELINE_LIMIT_WEEKLY="4"
TIMELINE_LIMIT_MONTHLY="6"
TIMELINE_LIMIT_YEARLY="0"

# Cleanup algorithm
CLEANUP_ALGORITHM="number"
NUMBER_LIMIT="50"

# Pre/post snapshots
SYNC_ACL="yes"
```

## Interactive Menu

```bash
ghostctl btrfs menu
# Select: Snapshot Management > Configure snapper
```

Menu options:
- Deploy Base Config
- Edit Config
- List Configs

## Automated Snapshots

Snapper can create automatic snapshots:

### Timeline Snapshots
Hourly snapshots with automatic cleanup based on retention settings.

### Pre/Post Snapshots
When integrated with package managers, snapshots before and after system changes.

## Best Practices

- Configure retention policies based on disk space
- Enable timeline snapshots for critical subvolumes
- Regularly verify snapper configs are working
- Monitor snapshot disk usage with `btrfs filesystem usage /`
