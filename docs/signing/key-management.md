# Key Management

Managing signing keys across Azure Key Vault, package managers, and local GPG.

## Export Public Key

Export the signing certificate's public key for distribution:

```bash
# OpenPGP format (for rpm --import, pacman-key --add)
ghostctl sign export-key --format pgp

# PEM format (X.509 certificate)
ghostctl sign export-key --format pem --output cert.pem

# Raw DER format
ghostctl sign export-key --format der --output cert.der
```

## OpenPGP Fingerprint Stability

OpenPGP v4 fingerprints include the public-key creation timestamp. GhostCTL uses the `[signing].pgp_key_created_at` value for OpenPGP exports and native package signatures so a key imported into pacman, rpm, or gpg keeps the same fingerprint across later signing runs.

Set `pgp_key_created_at` once before distributing a production OpenPGP key. Existing configs default to `0`, which is stable but less descriptive than a real issuance timestamp.

## Import into Package Managers

### RPM

```bash
ghostctl sign export-key --format pgp --output signing-key.asc
rpm --import signing-key.asc
```

### Pacman

```bash
ghostctl sign export-key --format pgp --output signing-key.asc
sudo pacman-key --add signing-key.asc
sudo pacman-key --lsign-key <FINGERPRINT>
```

### APT (Debian/Ubuntu)

```bash
ghostctl sign export-key --format pgp --output signing-key.asc
sudo cp signing-key.asc /etc/apt/trusted.gpg.d/
```

## List Certificates

View all certificates in the configured Key Vault:

```bash
ghostctl sign list-keys
```

Shows name, enabled status, and expiration date for each certificate.

## Key Lifecycle

1. Generate a certificate in Azure Key Vault (via Azure Portal or CLI)
2. Configure ghostctl: `ghostctl sign config --init`
3. Export and distribute the public key to consumers
4. Sign artifacts as part of build/release pipeline
5. Monitor expiration with `ghostctl sign list-keys`
6. Rotate: upload new cert, update config, redistribute public key

## Local GPG Keys vs Azure Key Vault

| Aspect | Local GPG | Azure Key Vault |
|--------|-----------|-----------------|
| Key storage | Local disk (encrypted) | HSM-backed cloud |
| Access control | File permissions | Azure RBAC + policies |
| Audit trail | None | Azure Activity Log |
| CI/CD | Requires key export | Service principal auth |
| Key rotation | Manual | Portal/CLI managed |
| Use case | Personal signing, git | Release artifacts, packages |

Both can coexist. Use `ghostctl gpg` for personal GPG keys and `ghostctl sign` for automated release signing.
