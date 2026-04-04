# Neovim Setup

## Commands

```bash
ghostctl nvim menu                # Neovim management
ghostctl nvim install             # Install Neovim
ghostctl nvim lazyvim             # Install LazyVim config
```

## LazyVim

Modern Neovim configuration with sensible defaults.

### Installation
```bash
ghostctl nvim lazyvim
```

### Configuration Location
```
~/.config/nvim/
├── init.lua
├── lua/
│   ├── config/
│   └── plugins/
└── lazy-lock.json
```

## Mason

LSP server and tool manager.

```vim
:Mason              " Open Mason UI
:MasonInstall <pkg>
:MasonUpdate
:MasonLog
```

### Common Packages

| Language | LSP | Formatter |
|----------|-----|-----------|
| Rust | rust-analyzer | rustfmt |
| Python | pyright | ruff, black |
| Go | gopls | gofumpt |
| Zig | zls | - |
| TypeScript | typescript-language-server | prettier |
| Lua | lua-language-server | stylua |

## Health Check

```vim
:checkhealth
:LspInfo
:Mason
```

## Useful Keybindings (LazyVim)

| Key | Action |
|-----|--------|
| `<space>` | Leader key |
| `<leader>e` | File explorer |
| `<leader>ff` | Find files |
| `<leader>fg` | Live grep |
| `gd` | Go to definition |
| `K` | Hover docs |
