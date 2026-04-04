# Storage Management

GhostCTL provides comprehensive storage management including S3-compatible cloud storage, local storage tools, and network storage.

## Documentation

- [S3 Cloud Storage](s3.md) - AWS, MinIO, Azure, Backblaze, Wasabi, DigitalOcean
- [Local Storage](local.md) - Local disk management and tools
- [Network Storage](network.md) - NFS and CIFS/SMB mounts

## Quick Start

```bash
ghostctl storage menu             # Storage management menu
```

## Menu Structure

```
Storage Management
├── S3/MinIO Storage Management
│   ├── Configure S3 Provider
│   ├── Bucket Operations
│   ├── File Operations
│   ├── Sync Operations
│   ├── Restic Integration
│   ├── Test Connection
│   └── Manage Profiles
├── Local Storage Tools
└── Network Storage (NFS/CIFS)
```

## Supported S3 Providers

- **Amazon S3** (AWS) - Native AWS integration
- **MinIO** - Self-hosted S3-compatible storage
- **Azure Blob Storage** - Microsoft Azure
- **Backblaze B2** - Cost-effective cloud storage
- **Wasabi** - Hot cloud storage
- **DigitalOcean Spaces** - DO's object storage
- **Custom** - Any S3-compatible endpoint

## Features

- Multi-provider S3 configuration
- Profile management for multiple accounts
- Bucket lifecycle and versioning
- File upload/download/sync operations
- Restic backup integration
- Presigned URL generation
- Local and network storage tools
