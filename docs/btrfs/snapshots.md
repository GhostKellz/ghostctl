# Btrfs Snapshot Management

## Overview

GhostCTL provides comprehensive Btrfs snapshot management for system backup and recovery.

## Commands

### Create Snapshot
```bash
ghostctl btrfs create NAME              # Create snapshot with name
ghostctl btrfs create NAME -s /home     # Snapshot specific subvolume
```

### List Snapshots
```bash
ghostctl btrfs list                     # List all snapshots
```

### Delete Snapshots
```bash
ghostctl btrfs delete NAME              # Delete specific snapshot
ghostctl btrfs cleanup --days 30        # Remove snapshots older than 30 days
ghostctl btrfs cleanup --range 1-100    # Remove snapshot range
ghostctl btrfs cleanup --emergency      # Remove ALL snapshots (dangerous)
```

### Restore Snapshot
```bash
ghostctl btrfs restore NAME PATH        # Restore snapshot to path
```

## Interactive Menu

```bash
ghostctl btrfs menu
```

The snapshot management menu provides:
- List all snapshots
- Create manual snapshot
- Delete snapshots (by number, age, or range)
- Rollback to snapshot
- Configure snapper

## Snapshot Cleanup

### By Age
Remove snapshots older than a specified number of days:
```bash
ghostctl btrfs cleanup --days 30
```

### By Range
Remove a range of snapshot numbers:
```bash
ghostctl btrfs cleanup --range 1-50
```

### Emergency Cleanup
When disk space is critically low, remove all snapshots:
```bash
ghostctl btrfs cleanup --emergency
```

This command:
1. Attempts snapper-based cleanup first
2. Falls back to direct btrfs subvolume deletion
3. Cleans up snapshot directories
4. Reports disk space recovery

## Best Practices

- Create snapshots before system updates
- Use meaningful snapshot descriptions
- Regularly prune old snapshots to save space
- Keep at least one known-good snapshot
- Test restore procedures periodically
