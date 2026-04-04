# Storage Migration

## Features

- Live VM/CT storage migration
- Bulk migration operations
- Progress tracking
- Validation checks

## Storage Migration Menu

```bash
ghostctl pve menu  # Select storage migration
```

### Operations
- Migrate VM disk to different storage
- Migrate container rootfs
- Bulk migrate multiple VMs/CTs
- Clone with storage change

## Supported Storage Types

- Local (directory)
- LVM
- LVM-thin
- ZFS
- Ceph/RBD
- NFS
- CIFS/SMB
- iSCSI

## Migration Considerations

### Live Migration
- VM continues running during migration
- Requires shared storage or local-to-local
- May impact performance temporarily

### Offline Migration
- VM must be stopped
- Faster for large disks
- No impact during migration

## Best Practices

- Verify target storage has sufficient space
- Test migration on non-critical VMs first
- Schedule migrations during low-usage periods
- Monitor migration progress
