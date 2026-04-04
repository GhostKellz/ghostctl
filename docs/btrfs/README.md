# Btrfs Filesystem Management

GhostCTL provides comprehensive Btrfs snapshot and filesystem management.

## Documentation

- [Snapshots](snapshots.md) - Snapshot creation, listing, deletion, and restoration
- [Snapper](snapper.md) - Snapper integration and configuration
- [Maintenance](maintenance.md) - Scrub, balance, quotas, and filesystem health
- [Recovery](recovery.md) - Disaster recovery, rollback, and file restoration

## Quick Commands

```bash
ghostctl btrfs menu               # Interactive menu
ghostctl btrfs list               # List snapshots
ghostctl btrfs create NAME        # Create snapshot
ghostctl btrfs delete NAME        # Delete snapshot
ghostctl btrfs restore NAME PATH  # Restore snapshot
ghostctl btrfs status             # Filesystem status
ghostctl btrfs scrub /            # Start scrub
ghostctl btrfs balance /          # Start balance
```

## Features

- Snapshot lifecycle management
- Snapper integration for automated snapshots
- Filesystem maintenance (scrub, balance, quotas)
- Disaster recovery and system rollback
- File-level restore from snapshots
- Restic backup integration
- Emergency cleanup tools

## Menu Structure

```
Btrfs Management
├── Filesystem Overview
│   ├── Filesystem show
│   ├── Disk usage
│   └── Subvolume list
├── Snapshot Management
│   ├── List all snapshots
│   ├── Create manual snapshot
│   ├── Delete snapshots
│   ├── Rollback to snapshot
│   └── Configure snapper
├── Backup Integration
│   ├── Backup snapshots to restic
│   ├── Setup automated workflows
│   └── Backup status
└── Disaster Recovery
    ├── Browse Snapshots
    ├── Compare Snapshots
    ├── Rollback System
    ├── Verify Snapshot Health
    └── Emergency Recovery Tools
```
