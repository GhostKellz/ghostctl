# GPG Key Management

## Commands

```bash
ghostctl gpg                      # Interactive GPG menu
ghostctl security gpg             # GPG management
```

## Key Generation

```bash
# Interactive generation
gpg --full-generate-key

# Quick generation
gpg --quick-generate-key "Name <email@example.com>"
```

## Key Management

### List Keys
```bash
# Public keys
gpg --list-keys

# Secret keys
gpg --list-secret-keys
```

### Export Keys
```bash
# Public key (for sharing)
gpg --armor --export email@example.com > public.asc

# Secret key (backup)
gpg --armor --export-secret-keys email@example.com > private.asc
```

### Import Keys
```bash
gpg --import public.asc
gpg --import private.asc
```

## Common Operations

### Sign a File
```bash
gpg --sign file.txt           # Creates file.txt.gpg
gpg --clearsign file.txt      # Creates file.txt.asc (readable)
gpg --detach-sign file.txt    # Creates file.txt.sig
```

### Verify Signature
```bash
gpg --verify file.txt.sig file.txt
```

### Encrypt/Decrypt
```bash
# Encrypt for recipient
gpg --encrypt --recipient email@example.com file.txt

# Decrypt
gpg --decrypt file.txt.gpg > file.txt
```

## Git Integration

```bash
# Configure Git to use GPG
git config --global user.signingkey KEYID
git config --global commit.gpgsign true

# Sign commits
git commit -S -m "Signed commit"
```
