# Signature Verification

Verify signatures created by `ghostctl sign`.

## Built-in Verification

```bash
# Verify a signed file (looks for FILE.sig by default)
ghostctl sign verify my-artifact.tar.gz

# Specify signature path
ghostctl sign verify my-artifact.tar.gz --signature path/to/file.sig

# Verbose output (shows key ID, algorithm, signing time)
ghostctl sign verify my-artifact.tar.gz --verbose
```

The verify command:
1. Reads the file and its detached `.sig` signature
2. Authenticates to Azure Key Vault to fetch the signing certificate
3. Parses the OpenPGP v4 signature packet
4. Recomputes the PGP-contextualized hash
5. Verifies the RSA signature using the certificate's public key

## External Tool Verification

### RPM

```bash
rpm --import <(ghostctl sign export-key --format pgp)
rpm -K package.rpm
```

### DEB

```bash
gpg --import <(ghostctl sign export-key --format pgp)
dpkg-sig --verify package.deb
```

### Pacman

```bash
pacman-key --verify package.pkg.tar.zst.sig package.pkg.tar.zst
```

### PE/Authenticode

```bash
osslsigncode verify app.exe
# Or on Windows:
signtool verify /pa app.exe
```

## Troubleshooting

| Error | Cause | Fix |
|-------|-------|-----|
| "Signature file not found" | No `.sig` file next to the file | Use `--signature` to specify path |
| "Failed to parse signature packet" | Not an OpenPGP v4 signature | Check file is a valid `.sig` |
| "Hash prefix mismatch" | File was modified after signing | Re-sign the file |
| "RSA verification failed" | Wrong key or corrupted signature | Verify cert-name matches the signer |
| "Failed to fetch certificate" | Auth/network issue | Run `ghostctl sign status` |
