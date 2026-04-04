# Network Storage

## Overview

GhostCTL provides tools for managing network-attached storage including NFS and CIFS/SMB mounts.

## Access

```bash
ghostctl storage menu
# Select: Network Storage (NFS/CIFS)
```

## NFS Mounts

### Mount NFS Share
```bash
sudo mount -t nfs server:/export/path /local/mount
```

### Persistent NFS Mount
Add to `/etc/fstab`:
```
server:/export/path  /local/mount  nfs  defaults,_netdev  0  0
```

### NFS Options
- `rw` - Read-write access
- `ro` - Read-only access
- `sync` - Synchronous writes
- `async` - Asynchronous writes (faster, less safe)
- `hard` - Retry NFS requests indefinitely
- `soft` - Return error if server unavailable
- `noatime` - Don't update access times

## CIFS/SMB Mounts

### Mount SMB Share
```bash
sudo mount -t cifs //server/share /local/mount -o username=user,password=pass
```

### Persistent SMB Mount
Add to `/etc/fstab`:
```
//server/share  /local/mount  cifs  credentials=/etc/samba/creds,_netdev  0  0
```

Create credentials file `/etc/samba/creds`:
```
username=your_username
password=your_password
domain=WORKGROUP
```

Secure the file:
```bash
sudo chmod 600 /etc/samba/creds
```

### SMB Options
- `uid=1000` - Local user ownership
- `gid=1000` - Local group ownership
- `file_mode=0644` - File permissions
- `dir_mode=0755` - Directory permissions
- `vers=3.0` - SMB protocol version

## Best Practices

- Use `_netdev` option for network mounts in fstab
- Store credentials securely with restricted permissions
- Consider autofs for on-demand mounting
- Test mounts before adding to fstab
- Use NFS v4 when possible for better security
