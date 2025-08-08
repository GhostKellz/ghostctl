# PVE.md — ghostctl Proxmox Enhancements (VFIO + Upgrade)

**Owner:** GhostKellz
**Agent:** Claude
**Repo:** `github.com/ghostkellz/ghostctl`
## ✅ v1.0.0 COMPLETED MODULES:

1. **Template Management** — Complete lifecycle management for LXC/VM templates ✅
2. **Storage Migration** — VM/Container storage migration with bulk operations ✅  
3. **Backup Rotation & Pruning** — Advanced backup management with retention policies ✅
4. **Firewall Automation** — Firewall management with gscan security scanning integration ✅

## 🚧 PLANNED MODULES (Future Releases):

1. `pve vfio` — safe-mode + rescue + NVIDIA/AMD passthrough helpers
2. `pve upgrade` — cluster-aware PVE 8→9 orchestrator

---

## 1) `ghostctl pve vfio` — Safe Mode + Rescue + NVIDIA Support

### Problem

Hosts can go headless after VFIO/GRUB/kernel changes. We need idempotent enable/disable, an SSH-only **rescue** that restores a TTY, runtime (un)bind, and built-ins for **AMD + NVIDIA** GPUs.

### CLI

```bash
ghostctl pve vfio enable --gpu 0000:0d:00.0 [--ids 1002:67df,1002:aaf0] [--persist]
ghostctl pve vfio disable [--persist]
ghostctl pve vfio rescue [--persist]
ghostctl pve vfio bind --device 0000:0d:00.0
ghostctl pve vfio status

# NVIDIA helpers
ghostctl pve vfio nvidia-prepare --gpu 0000:0a:00.0 [--with-audio] [--force]
ghostctl pve vfio nvidia-clean
```

> **`--persist`** ⇒ triggers `update-initramfs`, `proxmox-boot-tool refresh`, and reboots. Without it, commands apply runtime-only when possible.

### Behaviors

* **enable**

  * Detect PCI vendor\:device IDs (via `lspci -n -s <gpu>`). If `--ids` omitted, auto-fill from detection. For dGPU with HDA audio function, include both functions (ex: `10de:1b80,10de:10f0` or `1002:67df,1002:aaf0`).
  * Write `/etc/modprobe.d/vfio.conf` (idempotent):

    ```
    blacklist amdgpu
    blacklist nouveau
    options vfio-pci ids=<comma-separated-ids>
    options vfio-pci disable_vga=1
    ```
  * Ensure initramfs has `vfio`, `vfio_iommu_type1`, `vfio_pci`.
  * Ensure GRUB has IOMMU on: for AMD `amd_iommu=on iommu=pt`, for Intel `intel_iommu=on iommu=pt` (auto-detect CPU vendor).
  * If `--persist`: rebuild initramfs + refresh boot entries + reboot.

* **disable**

  * Remove `/etc/modprobe.d/vfio.conf` and any `vfio-pci.ids` GRUB appends made by this module.
  * Unblacklist amdgpu/nouveau if we added them.
  * `update-initramfs`, `proxmox-boot-tool refresh`, reboot if `--persist`.

* **rescue** (SSH-only rollback to visible console)

  * Force console boot flags (append or ensure): `video=efifb:force fbcon=map:1 console=tty1`.
  * Remove any amdgpu/nouveau blacklist and `vfio-pci.ids` so host driver can claim GPU.
  * Rebuild & refresh; reboot if `--persist`.

* **bind**

  * Runtime unbind/rebind a device without reboot:

    ```
    echo 0000:0d:00.0 > /sys/bus/pci/devices/0000:0d:00.0/driver/unbind
    echo <vendor> <device> > /sys/bus/pci/drivers/vfio-pci/new_id
    echo 0000:0d:00.0 > /sys/bus/pci/drivers/vfio-pci/bind
    ```

* **status**

  * Show per-GPU: PCI address, IOMMU group, current driver, candidate IDs, blacklist state, active GRUB flags.

### NVIDIA Passthrough Helpers

#### `nvidia-prepare`

Sets up the host cleanly for NVIDIA passthrough while avoiding Code 43 / binding issues.

* Detects paired HDA audio function (same slot `xx:yy.z` → `.1`).
* Writes `/etc/modprobe.d/blacklist-nouveau.conf`:

  ```
  blacklist nouveau
  options nouveau modeset=0
  ```
* Ensures **no** host `nvidia` modules will claim the device by binding to `vfio-pci` via ids. (No need to uninstall drivers.)
* Adds recommended GRUB flags for stability (only if not present):

  * `kvm.ignore_msrs=1` (guest stability)
  * `pci=noaer` (optional, silence spammy AER if needed)
* Proxmox VM hints (printed after run):

  * Use `BIOS: OVMF (UEFI)` and `Machine: q35`.
  * In VM hardware: `hostpci0: 0000:0a:00,pcie=1,x-vga=1,rombar=0` (add `,multifunction=on` and `hostpci1` for audio function).
  * Set **hidden** KVM flag in Proxmox: `args: -cpu host,kvm=off` **or** `hostpciX: ...,hidden=1` (newer Proxmox supports `hidden=1`).
  * Optionally supply a clean NVIDIA VBIOS ROM (`romfile=...`) if the card requires it.

#### `nvidia-clean`

* Removes nouveau blacklist and extra GRUB flags added by helper (leaves vfio config unless `disable` used).

### Troubleshooting Commands Added by Module

* `ghostctl pve vfio diag iommu` — verify IOMMU enabled (dmesg + `/proc/cmdline`).
* `ghostctl pve vfio diag map` — list `lspci -nnk` mapping and IOMMU groups.
* `ghostctl pve vfio diag conflicts` — check if `amdgpu`, `nouveau`, or `nvidia` currently bind the target device.
* `ghostctl pve vfio diag grub` — show and diff GRUB flags managed by ghostctl; offer one-click restore.

### Idempotency & Safety

* All file writes are **marker-guarded** (comments `# ghostctl-managed`) and reversible.
* GRUB edits only touch the `ghostctl-managed` segment appended to `GRUB_CMDLINE_LINUX_DEFAULT`.
* No destructive changes outside these markers.

---

## 2) `ghostctl pve upgrade` — Cluster-Aware 8→9 Orchestrator

### Problem

Manual cluster upgrades are error-prone. This module automates prechecks, draining, repo flips, upgrades, and reboots while preserving quorum.

### CLI

```bash
ghostctl pve upgrade precheck
ghostctl pve upgrade node <name> --no-sub
ghostctl pve drain <name> [--with-local-disks] [--offline]
ghostctl pve wave --nodes n2,n3,n4 [--no-sub]
```

### Behaviors

* **precheck**

  * Run `pve8to9 --full` and surface remedations.
  * Check cluster quorum, storage shared flags, HA state, PBS reachability.
  * Warn/abort if Ceph packages found (require explicit `--ack-ceph`).

* **drain**

  * Migrate running VMs/CTs off node. If disks on local-only storage and `--with-local-disks` not set, fall back to **shutdown** and **offline migrate**.

* **upgrade node**

  * Flip APT to Debian **Trixie** + PVE9 **no-subscription** repos.
  * `apt update && apt -y dist-upgrade`.
  * `proxmox-boot-tool refresh` when present.
  * Reboot and wait for node to rejoin quorum.

* **wave**

  * Perform `precheck`.
  * For each node in order: `drain` → `upgrade node` → wait for healthy cluster before continuing.

### Implementation Notes

* Uses `pvesh` for cluster introspection.
* Supports dry-run (`--dry-run`) for all steps.
* Logs to `/var/log/ghostctl/pve-upgrade-<date>.log`.

---

## Acceptance Criteria

* `ghostctl pve vfio enable/disable/rescue/bind/status` work on AMD RX 470-class and NVIDIA 10/20/30/40-series.
* Idempotent runs do not duplicate config.
* Rescue guarantees a visible TTY at boot on next reboot.
* `nvidia-prepare` emits exact Proxmox VM config hints (hostpci lines & args) based on detected PCI functions.
* `pve upgrade wave` upgrades a 3+ node cluster one-by-one, maintaining quorum.

## Out of Scope (for MVP)

* Automated VBIOS extraction/injection.
* Ceph repo/version management.
* Windows guest driver install.

## Future

* `ghostctl pve vfio profiles` (save/restore per-host VFIO configs).
* `ghostctl pve vm passthrough-wizard --vmid <id>` (writes hostpci lines + validates topology).
* `ghostctl pve upgrade rollback` (flip back to Bookworm/PVE8 in chroot).

[https://ghostctl.sh/](https://ghostctl.sh/)

