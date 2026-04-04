# Btrfs Filesystem Maintenance

## Overview

Regular maintenance keeps Btrfs filesystems healthy and performing optimally.

## Commands

### Filesystem Status
```bash
ghostctl btrfs status
```

Shows:
- Filesystem information
- Subvolume list
- Device statistics

### Usage Statistics
```bash
ghostctl btrfs usage [PATH]
ghostctl btrfs usage /
```

Shows detailed space usage including metadata and data allocation.

### Scrub
```bash
ghostctl btrfs scrub [PATH]
ghostctl btrfs scrub /
```

Scrub verifies data integrity by reading all data and metadata, checking checksums, and repairing corrupted data when possible.

Run periodically (monthly recommended) to catch and fix silent data corruption.

### Balance
```bash
ghostctl btrfs balance [PATH]
ghostctl btrfs balance /
```

Balance redistributes data across devices and reclaims unused space. Run after:
- Large file deletions
- Adding/removing devices
- Changing RAID profiles

### Quotas
```bash
ghostctl btrfs quota [PATH]
ghostctl btrfs quota /
```

Shows quota group information. Quotas can limit subvolume size but may impact performance on large filesystems.

## Disk Space Monitoring

```bash
ghostctl btrfs menu
# Select: Filesystem Overview
```

The overview shows:
- Btrfs filesystem information
- Current disk usage
- Subvolume list

## Interactive Menu

```bash
ghostctl btrfs menu
```

Filesystem Overview options:
- Filesystem show
- Disk usage
- Subvolume list

## Maintenance Schedule

### Weekly
- Check device stats for errors: `btrfs device stats /`

### Monthly
- Run scrub: `btrfs scrub start /`
- Review disk usage

### As Needed
- Run balance after large deletions
- Defragment heavily fragmented files

## Troubleshooting

### Check Device Errors
```bash
sudo btrfs device stats /
```

Non-zero values indicate potential hardware issues.

### Filesystem Show
```bash
sudo btrfs filesystem show
```

Lists all Btrfs filesystems and their devices.

### Emergency Repair
For corrupted filesystems (unmounted):
```bash
sudo btrfs check /dev/sdX
sudo btrfs check --repair /dev/sdX  # Use with caution
```
