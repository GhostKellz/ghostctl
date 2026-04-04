# SSH Key Management

## Commands

```bash
ghostctl ssh                      # Interactive SSH menu
ghostctl ssh generate             # Generate new key pair
ghostctl ssh list                 # List SSH keys
ghostctl ssh copy-id user@host    # Copy key to remote host
ghostctl ssh config               # SSH config management
```

## Key Generation

### ED25519 (Recommended)
```bash
ssh-keygen -t ed25519 -C "your@email.com"
```

### RSA (Legacy compatibility)
```bash
ssh-keygen -t rsa -b 4096 -C "your@email.com"
```

## Key Management

### List Keys
```bash
ls -la ~/.ssh/
ghostctl ssh list
```

### Copy to Remote
```bash
ghostctl ssh copy-id user@host
# or:
ssh-copy-id -i ~/.ssh/id_ed25519.pub user@host
```

## SSH Config

`~/.ssh/config`:
```
Host myserver
    HostName 192.168.1.100
    User admin
    Port 22
    IdentityFile ~/.ssh/id_ed25519

Host *
    AddKeysToAgent yes
    IdentitiesOnly yes
```

## SSH Agent

```bash
# Start agent
eval "$(ssh-agent -s)"

# Add key
ssh-add ~/.ssh/id_ed25519

# List loaded keys
ssh-add -l
```

## Security Best Practices

- Use ED25519 keys (faster, more secure)
- Always use a passphrase
- Disable password authentication on servers
- Use SSH agent for convenience
- Regularly rotate keys
