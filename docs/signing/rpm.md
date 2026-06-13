# RPM Package Signing

RPM packages can be signed in two modes: detached signatures or native OpenPGP embedding.

## Detached Signing (default)

Produces a `.sig` file alongside the RPM with extracted package metadata:

```bash
ghostctl sign file package.rpm
```

Output:
- `package.rpm.sig` -- raw signature bytes
- `package.rpm.sig.json` -- metadata (package name, version, arch, algorithm, key ID)

## Native Signing (`--native`)

Embeds an OpenPGP v4 signature packet directly in the RPM signature header as `RPMSIGTAG_RSA` (tag 268). The resulting RPM is verifiable with standard RPM tools.

```bash
ghostctl sign file package.rpm --native
```

### How It Works

1. The RPM is parsed into: lead, signature header, main header + payload
2. A PGP-contextualized SHA-256 hash is computed over the main header + payload
3. The hash digest is sent to Azure Key Vault for PKCS#1 v1.5 signing
4. The raw RSA signature is wrapped in an OpenPGP v4 signature packet
5. The signature header is rebuilt with the new `RPMSIGTAG_RSA` tag injected
6. The RPM is written with the updated signature header

### Verification

```bash
# Import the public key
rpm --import <(ghostctl sign export-key --format pgp)

# Verify
rpm -K package.rpm
```

## Limitations

- RSA only (RS256/RS384/RS512)
- No key chain or web of trust
- Replaces any existing header signature
