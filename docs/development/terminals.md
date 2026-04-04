# Terminal Configuration

## Commands

```bash
ghostctl terminal menu            # Terminal configuration
ghostctl terminal ghostty         # Setup Ghostty
ghostctl terminal starship        # Install Starship prompt
```

## Ghostty

Fast, GPU-accelerated terminal emulator.

### Installation
```bash
ghostctl terminal ghostty
# or via AUR:
paru -S ghostty
```

### Configuration
`~/.config/ghostty/config`:
```
font-family = "JetBrainsMono Nerd Font"
font-size = 12
theme = catppuccin-mocha
```

## Alacritty

Cross-platform, GPU-accelerated terminal.

### Configuration
`~/.config/alacritty/alacritty.toml`:
```toml
[font]
normal.family = "JetBrainsMono Nerd Font"
size = 12

[window]
opacity = 0.95
```

## Starship

Cross-shell prompt with smart defaults.

### Installation
```bash
ghostctl terminal starship
# or:
curl -sS https://starship.rs/install.sh | sh
```

### Configuration
`~/.config/starship.toml`:
```toml
[character]
success_symbol = "[➜](bold green)"
error_symbol = "[✗](bold red)"

[directory]
truncation_length = 3
```

### Enable in Shell
```bash
# .bashrc or .zshrc
eval "$(starship init bash)"  # or zsh
```

## Nerd Fonts

Required for icons in terminals and Neovim.

```bash
# Arch
paru -S ttf-jetbrains-mono-nerd

# Manual
# Download from https://www.nerdfonts.com/
```
