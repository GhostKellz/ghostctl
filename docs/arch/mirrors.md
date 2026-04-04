# Mirror Management

## Commands

```bash
ghostctl arch mirrors             # Update mirrors with reflector
ghostctl arch clean mirrors       # Clean and optimize
```

## Reflector

### Basic Usage
```bash
# Fastest mirrors in your country
sudo reflector --country US --age 12 --protocol https --sort rate --save /etc/pacman.d/mirrorlist

# Top 10 fastest worldwide
sudo reflector --latest 10 --protocol https --sort rate --save /etc/pacman.d/mirrorlist
```

### Automated Updates
```bash
sudo systemctl enable --now reflector.timer
```

### Configuration
`/etc/xdg/reflector/reflector.conf`:
```
--save /etc/pacman.d/mirrorlist
--protocol https
--country US,CA
--latest 10
--sort rate
```

## Manual Mirror Selection

Edit `/etc/pacman.d/mirrorlist`:
```
Server = https://mirror.example.com/archlinux/$repo/os/$arch
```

## Testing Mirrors

```bash
# Test download speed
curl -o /dev/null -w "%{speed_download}\n" \
  https://mirror.example.com/archlinux/core/os/x86_64/core.db

# Force refresh after changes
sudo pacman -Syy
```

## Troubleshooting

### Slow downloads
```bash
ghostctl arch mirrors
```

### 404 errors
Mirror may be out of sync. Update mirror list or wait.
