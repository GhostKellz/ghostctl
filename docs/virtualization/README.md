# Virtualization & Passthrough

GhostCTL provides tools for IOMMU management, VFIO configuration, and device passthrough.

## Documentation

- [GPU Passthrough](gpu-passthrough.md) - Pass GPUs to virtual machines

## IOMMU Commands

IOMMU groups determine which devices can be passed through together.

```bash
ghostctl iommu menu               # Interactive menu
ghostctl iommu status             # IOMMU status (enabled, mode)
ghostctl iommu groups             # List all IOMMU groups
ghostctl iommu groups --gpu       # GPU groups only
ghostctl iommu groups --json      # JSON output
ghostctl iommu analyze <device>   # Analyze passthrough viability
ghostctl iommu gpus               # List GPUs
ghostctl iommu usb                # List USB controllers
ghostctl iommu nvme               # List NVMe controllers
ghostctl iommu tree               # PCIe topology
ghostctl iommu acs                # ACS override status
```

## VFIO Commands

VFIO binds devices to the vfio-pci driver for passthrough.

```bash
ghostctl vfio menu                # Interactive menu
ghostctl vfio setup               # Setup wizard
ghostctl vfio status              # Bound devices
ghostctl vfio bind <device>       # Bind device (e.g., 01:00.0)
ghostctl vfio unbind <device>     # Unbind device
ghostctl vfio modules             # Check/load VFIO modules
ghostctl vfio config              # Configuration status
ghostctl vfio kernel-params       # Recommended kernel params
```

## Kernel Parameters

Enable IOMMU in bootloader:

```bash
# Intel CPU
intel_iommu=on iommu=pt

# AMD CPU
amd_iommu=on iommu=pt
```

## Related

- [NVIDIA documentation](../nvidia/README.md) - GPU drivers
- [Proxmox documentation](../proxmox/README.md) - VM management
