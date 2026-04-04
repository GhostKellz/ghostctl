# UEFI Secure Boot Management

Create OVMF VARS files with Secure Boot keys enrolled for Windows 11 VMs.

## Dependencies

| Package | Purpose |
|---------|---------|
| `edk2-ovmf` | OVMF firmware |
| `virt-firmware` | Key enrollment (`virt-fw-vars`) |
| `swtpm` | TPM 2.0 emulator |

```bash
sudo pacman -S edk2-ovmf virt-firmware swtpm
```

## Commands

### Check Status

```bash
ghostctl uefi status
```

### Enroll Keys

```bash
ghostctl uefi enroll -o /path/to/output.fd
```

Creates OVMF VARS with Microsoft Secure Boot keys. Enrolls:
- PK (Platform Key)
- KEK (Key Exchange Key) with Microsoft certs
- db (Signature Database) with Microsoft Windows + UEFI certs

**Options:**

| Flag | Description |
|------|-------------|
| `-o, --output` | Output path (required) |
| `--template` | OVMF_VARS template (default: `/usr/share/edk2/x64/OVMF_VARS.4m.fd`) |
| `-v, --verbose` | Show detailed output |

### Verify VARS

```bash
ghostctl uefi verify /path/to/vars.fd
```

Best-effort check that VARS has required keys enrolled. Note: this verifies the VARS file contents only, not your VM XML configuration.

## Usage with libvirt

1. Create enrolled VARS:
   ```bash
   ghostctl uefi enroll -o /var/lib/libvirt/qemu/nvram/win11_VARS.fd
   sudo chown libvirt-qemu:libvirt-qemu /var/lib/libvirt/qemu/nvram/win11_VARS.fd
   ```

2. VM XML config:
   ```xml
   <os>
     <type arch='x86_64' machine='q35'>hvm</type>
     <loader readonly='yes' secure='yes' type='pflash'>/usr/share/edk2/x64/OVMF_CODE.secboot.4m.fd</loader>
     <nvram>/var/lib/libvirt/qemu/nvram/win11_VARS.fd</nvram>
   </os>
   <features>
     <smm state='on'/>
   </features>
   <tpm model='tpm-crb'>
     <backend type='emulator' version='2.0'/>
   </tpm>
   ```

## Requirements

- **SMM enabled** (`<smm state='on'/>`)
- **Q35 machine type**
- **TPM 2.0** for Windows 11

## Troubleshooting

**Secure Boot shows "Setup Mode"**
VARS doesn't have keys. Re-run `ghostctl uefi enroll`.

**Permission denied**
```bash
sudo chown libvirt-qemu:libvirt-qemu /path/to/vars.fd
```

**Windows 11 "doesn't meet requirements"**
Check both Secure Boot (enrolled VARS) and TPM 2.0 (`<tpm>` element).

## Future Improvements

Potential enhancements tracked for future releases:

- **Structured parsing**: `virt-fw-vars` supports `--output-json` which could replace current string-based verification for more robust parsing
- **End-to-end validation**: Optional VM boot verification to confirm Secure Boot is active (beyond varstore inspection)
