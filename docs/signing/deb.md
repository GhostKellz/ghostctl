# DEB Package Signing

Debian packages can be signed in two modes: detached signatures or native dpkg-sig format.

## Detached Signing (default)

Produces a `.sig` file alongside the DEB with extracted package metadata:

```bash
ghostctl sign file package.deb
```

Output:
- `package.deb.sig` -- raw signature bytes
- `package.deb.sig.json` -- metadata (package name, version, arch, algorithm, key ID)

## Native Signing (`--native`)

Adds a `_gpgbuilder` ar member to the `.deb` archive in dpkg-sig v4 format. The resulting DEB is verifiable with `dpkg-sig --verify`.

```bash
ghostctl sign file package.deb --native
```

### How It Works

1. The `.deb` AR archive is parsed into members
2. MD5 and SHA-1 hashes are computed for each existing ar member
3. A dpkg-sig v4 control text is generated listing all member hashes
4. A PGP-contextualized SHA-256 hash is computed over the control text
5. The hash is sent to Azure Key Vault for PKCS#1 v1.5 signing
6. An ASCII-armored OpenPGP signature is appended to the control text
7. The `_gpgbuilder` ar member is added to the archive

### Verification

```bash
# Import the public key
gpg --import <(ghostctl sign export-key --format pgp)

# Verify
dpkg-sig --verify package.deb
```

## Limitations

- RSA only (RS256/RS384/RS512)
- No key chain or web of trust
- Single signer per package
