# Backup Management

## Backup Rotation

GhostCTL provides backup rotation and retention policy management.

### Features
- Automated backup scheduling
- Retention policies (daily, weekly, monthly)
- Pruning old backups
- Integrity verification

### Commands
Through the PVE menu:
```bash
ghostctl pve menu  # Select backup options
```

## Proxmox Backup Server (PBS)

Integration with PBS for enterprise backup.

### Features
- Datastore management
- Backup job configuration
- Restore operations
- Garbage collection
- Verification jobs

### PBS Menu
```bash
ghostctl pve menu  # PBS submenu
```

## Backup Best Practices

### Retention Strategy
```
# Example retention
keep-daily=7      # Keep 7 daily backups
keep-weekly=4     # Keep 4 weekly backups
keep-monthly=6    # Keep 6 monthly backups
```

### Storage Planning
- Use dedicated storage for backups
- Consider off-site replication
- Monitor storage usage
- Regular integrity checks
