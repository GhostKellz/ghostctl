# Btrfs Disaster Recovery

## Overview

GhostCTL provides disaster recovery tools for Btrfs filesystems including file-level restore, system rollback, and snapshot verification.

## Recovery Menu

```bash
ghostctl btrfs menu
# Select: Disaster Recovery
```

Options:
- Browse Snapshots (File Recovery)
- Compare Snapshots
- Rollback System
- Verify Snapshot Health
- Emergency Recovery Tools
- View Recovery History

## File-Level Recovery

### Browse Snapshots
1. Select snapper configuration (root, home, etc.)
2. Choose snapshot to browse
3. Available actions:
   - List files in snapshot root
   - Compare file with current system
   - Restore specific file/directory
   - Open shell in snapshot (read-only)

### Restore Single File
```bash
# Through menu:
ghostctl btrfs menu > Disaster Recovery > Browse Snapshots
# Navigate to file, select "Restore specific file/directory"
```

The restore process:
1. Prompts for file path (e.g., `/etc/fstab`)
2. Optionally creates backup of current file
3. Copies file from snapshot to current system

## System Rollback

### Full System Rollback
```bash
ghostctl btrfs menu
# Select: Disaster Recovery > Rollback System
```

Rollback process:
1. Lists available snapshots
2. Shows preview of changes
3. Creates pre-rollback snapshot
4. Performs rollback using `snapper undochange`
5. Prompts for reboot

### Important Considerations
- Rollback reverts root filesystem to previous state
- Changes after snapshot will be lost
- Requires system reboot to complete
- Pre-rollback snapshot allows recovery if needed

## Compare Snapshots

Compare differences between two snapshots:
1. Select configuration
2. Choose first (older) snapshot
3. Choose second (newer) snapshot
4. View list of changed files

## Verify Snapshot Health

Checks:
- Snapshot path existence
- Snapper configuration validity
- Btrfs device statistics
- Total snapshot count per configuration

## Emergency Tools

### Check/Repair Filesystem
Runs btrfs scrub on mounted filesystem. For unmounted repair, use recovery media.

### Mount Snapshot Read-Write
Temporarily mount a snapshot for direct access. Use with caution.

### Export Snapshot
Archive a snapshot to tar.gz for off-system backup:
```bash
# Creates /tmp/snapshot_N.tar.gz
```

### Clear Lock Files
Removes stuck snapper lock files if snapshots become unresponsive.

### Recovery Shell
Opens root shell for manual recovery operations.

## Backup Integration

### Restic Integration
Backup snapshots to external storage:
```bash
ghostctl btrfs menu
# Select: Backup Integration
```

Options:
- Backup latest snapshot to restic
- Backup specific snapshot
- Backup multiple recent snapshots
- Configure automated backups

### Automated Backup Workflows
Create systemd timers for scheduled snapshot backups:
- Daily, twice daily, or weekly schedules
- Automatic retention policy
- Integration with restic repository

## Best Practices

- Test recovery procedures before emergencies
- Keep recent known-good snapshots
- Document custom configurations
- Monitor snapshot disk usage
- Verify backups periodically
