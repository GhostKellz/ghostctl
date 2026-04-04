# Development Environment

GhostCTL provides development environment setup for multiple languages and tools.

## Documentation

- [Neovim](neovim.md) - Editor setup, LazyVim, Mason
- [Terminals](terminals.md) - Ghostty, Alacritty, Starship

## Quick Commands

```bash
# Development menu
ghostctl dev menu

# Language environments
ghostctl dev rust
ghostctl dev python
ghostctl dev go
ghostctl dev zig

# Editor
ghostctl nvim menu
ghostctl nvim lazyvim

# Terminal
ghostctl terminal menu
ghostctl terminal ghostty
ghostctl terminal starship
```

## Ghost Tools

```bash
ghostctl ghost menu               # Ghost tools menu
ghostctl ghost install-all        # Install all
ghostctl ghost reaper             # Reaper AUR helper
ghostctl ghost oxygen             # Oxygen (Rust)
ghostctl ghost zion               # Zion (Zig)
ghostctl ghost status             # Check status
```

## Shell Configuration

```bash
ghostctl shell setup              # Shell environment
ghostctl shell zsh                # ZSH setup
```
