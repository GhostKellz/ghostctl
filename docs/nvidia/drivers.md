# NVIDIA Driver Installation

## Commands

```bash
ghostctl nvidia install           # Interactive driver installation
ghostctl nvidia wayland           # Configure Wayland support
```

## Driver Variants

| Variant | Package | Description |
|---------|---------|-------------|
| Proprietary | `nvidia` | Stable, recommended for most users |
| Open | `nvidia-open` | Open kernel modules (Turing+) |
| Beta | `nvidia-beta` | Latest features, may be unstable |

## DKMS Management

DKMS automatically rebuilds kernel modules on kernel updates.

```bash
ghostctl nvidia dkms-status       # Check DKMS module status
ghostctl nvidia dkms-cleanup      # Remove old DKMS entries
ghostctl nvidia build-source      # Build from source (advanced)
ghostctl nvidia build-source --all-kernels  # Build for all kernels
```

## Wayland Support

For Wayland compositors (Sway, Hyprland, etc.):

```bash
ghostctl nvidia wayland           # Configure Wayland environment
```

This sets up:
- `nvidia-drm.modeset=1` kernel parameter
- GBM backend for EGL
- Required environment variables

## Troubleshooting

### Module not loading
```bash
# Check if module is loaded
lsmod | grep nvidia

# Rebuild DKMS modules
sudo dkms autoinstall
```

### Black screen after update
```bash
# Boot to TTY and rebuild modules
ghostctl nvidia build-source
```
