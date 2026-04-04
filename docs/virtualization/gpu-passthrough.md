# GPU Passthrough

Pass GPUs (NVIDIA, AMD, Intel) to virtual machines using VFIO.

## Prerequisites

- CPU with IOMMU support (Intel VT-d / AMD-Vi)
- GPU in separate IOMMU group
- Second GPU or integrated graphics for host

## Commands

```bash
ghostctl iommu groups --gpu       # List GPU IOMMU groups
ghostctl iommu analyze 01:00.0    # Check passthrough viability
ghostctl vfio setup               # VFIO configuration wizard
ghostctl vfio bind 01:00.0        # Bind GPU to vfio-pci
ghostctl vfio status              # Check binding status
```

## Kernel Parameters

Add to bootloader config:

```bash
# Intel CPU
intel_iommu=on iommu=pt

# AMD CPU
amd_iommu=on iommu=pt
```

## VFIO Configuration

### /etc/modprobe.d/vfio.conf

**NVIDIA:**
```
options vfio-pci ids=10de:xxxx,10de:yyyy
softdep nvidia pre: vfio-pci
```

**AMD:**
```
options vfio-pci ids=1002:xxxx,1002:yyyy
softdep amdgpu pre: vfio-pci
```

Find your GPU IDs:
```bash
lspci -nn | grep -i vga
```

## ROM Dumping

Some GPUs need a ROM file for passthrough:

```bash
ghostctl vfio dump-rom 01:00.0 -o gpu.rom
ghostctl vfio rom-list            # List GPUs with ROM info
```

## Single GPU Passthrough

For systems with only one GPU:

```bash
ghostctl vfio single-gpu status   # Check status
ghostctl vfio single-gpu list     # List configured VMs
ghostctl vfio single-gpu remove <vm>  # Remove hooks
```

This uses hooks to unbind GPU from host when VM starts and rebind when VM stops.

## Troubleshooting

### GPU not in separate IOMMU group
- Enable ACS override patch
- Check BIOS settings for IOMMU/VT-d

### Black screen in VM
- Ensure ROM file is provided if needed
- Check GPU reset issues (AMD common)

### Reset bug
Some GPUs don't reset properly. Workarounds:
- Vendor reset patch
- Use newer kernel
- Avoid certain GPU models
