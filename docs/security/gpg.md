# GPG Key Management

Manage GPG keys through `ghostctl gpg` commands.

## Interactive Menu

```bash
ghostctl gpg              # Launch interactive GPG menu
ghostctl security gpg     # Same as above
```

Menu options:
- List GPG keys
- Generate new GPG key
- Export public key
- Import public key
- Change key passphrase
- Delete GPG key
- GPG configuration
- Refresh keys from keyserver
- Key trust management
- Custom GPG generation (batch mode with full control)

## Non-Interactive Commands

```bash
ghostctl gpg list                    # List all keys
ghostctl gpg info <KEY_ID>           # Show key details
ghostctl gpg export <KEY_ID>         # Export public key to stdout
ghostctl gpg renew <KEY_ID> -d 2y   # Extend key expiration
```

### Key ID Formats

Accepted formats for `<KEY_ID>`:
- Short hex ID: `ABCD1234`
- Long hex ID: `ABCD1234EFGH5678`
- Full fingerprint: 40-character hex string
- Email address: `user@example.com`
- Name: `John Doe`

## Key Generation

```bash
# Interactive (prompts for name, email, key type)
ghostctl gpg
# Then select "Generate new GPG key"

# Custom generation (full control over parameters)
ghostctl gpg
# Then select "Custom GPG generation"
```

Both methods validate input to prevent shell injection.

## Key Renewal

Extend expiration of an existing key:

```bash
ghostctl gpg renew ABCD1234 --duration 1y
```

Available durations: `1y`, `2y`, `3y`, `5y`, `10y`

## Git Integration

```bash
# List key IDs
ghostctl gpg list

# Configure Git
git config --global user.signingkey <KEY_ID>
git config --global commit.gpgsign true

# Sign commits
git commit -S -m "Signed commit"
```

## Keyserver Operations

Refresh keys from a keyserver (interactive, with default `hkps://keys.openpgp.org`):

```bash
ghostctl gpg
# Then select "Refresh keys from keyserver"
```

## Trust Management

Edit key trust levels (launches gpg's trust editor):

```bash
ghostctl gpg
# Then select "Key trust management"
```

## Local GPG Keys vs Azure Key Vault Signing

For automated code signing of release artifacts, see [Code Signing](../signing/README.md).

| Aspect | Local GPG | Azure Key Vault |
|--------|-----------|-----------------|
| Key storage | Local disk (encrypted) | HSM-backed cloud |
| Use case | Personal signing, git commits | Release artifacts, packages |
| CI/CD | Requires key export | Service principal auth |
| Management | `ghostctl gpg` | `ghostctl sign` |

Both can coexist -- use GPG for personal identity and Key Vault for organizational signing.
