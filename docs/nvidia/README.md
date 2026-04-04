# NVIDIA GPU Management

GhostCTL provides tools for NVIDIA driver installation, optimization, and container GPU support.

## Documentation

- [Driver Installation](drivers.md) - Install and manage NVIDIA drivers, DKMS
- [Container Runtime](container.md) - Docker/Podman GPU support

## Quick Commands

```bash
ghostctl nvidia                   # Interactive menu
ghostctl nvidia install           # Install drivers
ghostctl nvidia optimize          # Optimize GPU
ghostctl nvidia wayland           # Configure Wayland
ghostctl nvidia dkms-status       # DKMS module status
ghostctl nvidia dkms-cleanup      # Clean old DKMS entries
```

## GPU Passthrough

For passing GPUs to virtual machines, see [Virtualization > GPU Passthrough](../virtualization/gpu-passthrough.md).
