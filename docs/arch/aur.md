# AUR Package Management

## Supported Helpers

| Helper | Language | Description |
|--------|----------|-------------|
| reaper | Rust | Fast, minimal (recommended) |
| paru | Rust | Feature-rich, colorful |
| yay | Go | Popular, well-maintained |

## Commands

```bash
ghostctl arch aur                 # AUR management menu
```

## Helper Detection

GhostCTL detects installed helpers in order: reaper → paru → yay

## Installing Helpers

### Reaper
```bash
ghostctl ghost reaper
```

### Paru
```bash
sudo pacman -S --needed base-devel
git clone https://aur.archlinux.org/paru.git
cd paru && makepkg -si
```

### Yay
```bash
sudo pacman -S --needed base-devel git
git clone https://aur.archlinux.org/yay.git
cd yay && makepkg -si
```

## Common Operations

```bash
# Search
paru -Ss package

# Install
paru -S package

# Update all (including AUR)
paru -Syu

# Clean build cache
paru -Sc
```

## Best Practices

- Review PKGBUILDs before installing
- Check comments on AUR pages
- Keep helpers updated
- Clean build cache periodically
