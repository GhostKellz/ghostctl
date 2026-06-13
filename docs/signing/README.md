# Code Signing via Azure Key Vault

> **Status: Experimental** -- This feature is under active development.

Sign files using Azure Key Vault-backed keys. The private key never leaves Key Vault -- files are hashed locally and the digest is sent to Key Vault for signing via its REST API.

## Prerequisites

- Azure subscription with an Azure Key Vault (Premium tier recommended for HSM-backed keys)
- A signing certificate/key uploaded to the Key Vault
- One of:
  - **Azure CLI** installed and authenticated (`az login`)
  - **Service Principal** with Key Vault access (for CI/CD)

### Key Vault Access Policy

| Area        | Permission |
|-------------|------------|
| Key         | Sign       |
| Secret      | Get        |
| Certificate | Get        |

## Quick Start

```bash
# Configure signing
ghostctl sign config --init

# Sign a file (auto-detects format)
ghostctl sign file my-artifact.tar.gz

# Check status
ghostctl sign status

# Verify a signature
ghostctl sign verify my-artifact.tar.gz

# Export public key for distribution
ghostctl sign export-key --format pgp

# List certificates in vault
ghostctl sign list-keys
```

## Authentication

### Azure CLI (default)

Uses your existing `az login` session:

```bash
az login
ghostctl sign file artifact.tar.gz
```

### Service Principal

For CI/CD pipelines. Set `AZURE_CLIENT_SECRET` as environment variable:

```bash
export AZURE_CLIENT_SECRET="your-secret-here"
ghostctl sign file artifact.tar.gz --auth sp
```

## Supported Formats

| Format  | Detection | Signature Type |
|---------|-----------|----------------|
| [Generic](generic.md) | Fallback | Detached `.sig` + `.sig.json` |
| [PE/Authenticode](pe.md) | `.exe`, `.dll`, `.sys` | Embedded WIN_CERTIFICATE |
| [RPM](rpm.md) | `.rpm` magic bytes | Detached or native (`--native`) |
| [DEB](deb.md) | `!<arch>` + debian-binary | Detached or native (`--native`) |
| [Pacman](pacman.md) | `.pkg.tar.*`, `.db.tar.*` | Detached OpenPGP `.sig` |

## Security Model

- **Private key isolation**: The signing key never leaves Azure Key Vault
- **No secrets in config**: `AZURE_CLIENT_SECRET` read from environment only
- **Token lifecycle**: In-memory only, auto-refreshed before expiry
- **TLS enforcement**: All API calls use HTTPS with cert verification
- **Input validation**: Key Vault names validated before API calls

## Further Reading

- [PE Authenticode Signing](pe.md)
- [RPM Package Signing](rpm.md)
- [DEB Package Signing](deb.md)
- [Arch Linux Package Signing](pacman.md)
- [Key Management](key-management.md)
- [Signature Verification](verification.md)

## Configuration

Signing config lives in `~/.config/ghostctl/config.toml` under `[signing]`:

```toml
[signing]
vault_url = "https://my-vault.vault.azure.net"
cert_name = "code-signing-cert"
algorithm = "RS256"
auth_method = "cli"
tsa_url = "http://timestamp.digicert.com"
```
