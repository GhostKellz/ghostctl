# Arch Linux Package Signing

Arch Linux packages and repository databases are signed with detached OpenPGP binary signatures -- the same format `gpg --detach-sign` produces.

## Supported File Types

Auto-detected by compound extension:

| Pattern | Description |
|---------|-------------|
| `*.pkg.tar.zst` | Standard Arch package (zstd) |
| `*.pkg.tar.xz` | Arch package (xz) |
| `*.pkg.tar.gz` | Arch package (gzip) |
| `*.db.tar.gz` | Repository database |
| `*.db.tar.zst` | Repository database (zstd) |
| `*.files.tar.gz` | Repository files index |
| `*.files.tar.zst` | Repository files index (zstd) |

## Usage

```bash
# Sign a package
ghostctl sign file package-1.0-1-x86_64.pkg.tar.zst

# Sign a repo database
ghostctl sign file myrepo.db.tar.gz

# Dry run
ghostctl sign file package.pkg.tar.zst --dry-run
```

Output: `package-1.0-1-x86_64.pkg.tar.zst.sig` (raw OpenPGP v4 binary signature packet)

## Setting Up pacman-key

```bash
# Export the signing public key
ghostctl sign export-key --format pgp --output signing-key.asc

# Add to pacman keyring
sudo pacman-key --add signing-key.asc

# Get the key fingerprint
ghostctl sign export-key 2>&1 | grep Fingerprint

# Sign the key locally
sudo pacman-key --lsign-key <FINGERPRINT>
```

## Verification

```bash
# Verify with pacman-key
pacman-key --verify package.pkg.tar.zst.sig package.pkg.tar.zst

# Verify with ghostctl
ghostctl sign verify package.pkg.tar.zst
```

## Limitations

- RSA only (RS256/RS384/RS512)
- Binary signatures only (not ASCII-armored)
