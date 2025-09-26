# GhostCTL Installation Guide

## Quick Install (Recommended)

```bash
# One-line installation (will be available at ghostctl.sh)
curl -sSL https://raw.githubusercontent.com/ghostkellz/ghostctl/main/install.sh | bash

# Or with wget
wget -qO- https://raw.githubusercontent.com/ghostkellz/ghostctl/main/install.sh | bash
```

## Installation Options

### 🔧 Installation Methods

The installer automatically detects your distribution and tries methods in this order:

1. **Package Manager** (preferred) - Uses AUR, APT, DNF, etc.
2. **Binary Download** - Downloads pre-built binaries from GitHub releases
3. **Source Build** - Compiles from source as fallback

### 📦 Package Manager Installation

#### Arch Linux (AUR)
```bash
# With yay
yay -S ghostctl

# With paru
paru -S ghostctl

# Manual PKGBUILD
git clone https://aur.archlinux.org/ghostctl.git
cd ghostctl
makepkg -si
```

#### Debian/Ubuntu
```bash
# Will be available via APT repository (coming soon)
# For now, use the universal installer or build from source
```

#### Fedora/RHEL/CentOS
```bash
# Will be available via DNF/YUM repository (coming soon)
# For now, use the universal installer or build from source
```

### 💾 Binary Installation

Download pre-built binaries from [GitHub releases](https://github.com/ghostkellz/ghostctl/releases):

```bash
# Download and install manually
curl -L https://github.com/ghostkellz/ghostctl/releases/latest/download/ghostctl-latest-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv ghostctl /usr/local/bin/
```

### 🛠️ Build from Source

```bash
# Clone repository
git clone https://github.com/ghostkellz/ghostctl.git
cd ghostctl/ghostctl

# Build with Cargo
cargo build --release

# Install
sudo cp target/release/ghostctl /usr/local/bin/
```

## Advanced Installation Options

### Custom Installation Directory

```bash
# Install to custom directory
curl -sSL https://raw.githubusercontent.com/ghostkellz/ghostctl/main/install.sh | bash -s -- --dir ~/.local/bin
```

### Specific Version

```bash
# Install specific version
curl -sSL https://raw.githubusercontent.com/ghostkellz/ghostctl/main/install.sh | bash -s -- --version v1.0.0
```

### Force Installation Method

```bash
# Force binary installation
curl -sSL https://raw.githubusercontent.com/ghostkellz/ghostctl/main/install.sh | bash -s -- --method binary

# Force source build
curl -sSL https://raw.githubusercontent.com/ghostkellz/ghostctl/main/install.sh | bash -s -- --method source
```

### Skip Optional Features

```bash
# Skip shell completions and desktop entry
curl -sSL https://raw.githubusercontent.com/ghostkellz/ghostctl/main/install.sh | bash -s -- --no-completions --no-desktop
```

## Environment Variables

Set these environment variables to customize installation:

```bash
export GHOSTCTL_INSTALL_DIR="/usr/local/bin"  # Installation directory
export GHOSTCTL_VERSION="v1.0.1"             # Specific version
export GHOSTCTL_METHOD="binary"              # Installation method
```

## Post-Installation

### Quick Start
```bash
# Launch interactive menu
ghostctl

# Show help
ghostctl --help

# Show version
ghostctl version
```

### Configuration
```bash
# User configuration directory
~/.config/ghostctl/
├── scripts/     # Custom scripts
├── plugins/     # Plugin files
└── profiles/    # Configuration profiles

# User data directory
~/.local/share/ghostctl/
├── logs/        # Application logs
├── cache/       # Cache files
└── data/        # Application data
```

## Building Packages

For maintainers and contributors:

```bash
# Build package for current distribution
./packaging/build-packages.sh

# Build specific package type
./packaging/build-packages.sh arch     # Arch Linux
./packaging/build-packages.sh debian   # Debian/Ubuntu
./packaging/build-packages.sh fedora   # Fedora/RHEL

# Install build dependencies
./packaging/build-packages.sh deps
```

## Supported Platforms

### Operating Systems
- ✅ **Linux** (All major distributions)
  - Arch Linux, Manjaro, EndeavourOS
  - Ubuntu, Debian, Linux Mint, Pop!_OS
  - Fedora, RHEL, CentOS, Rocky Linux, AlmaLinux
  - openSUSE, SLES
  - Alpine Linux
- ✅ **macOS** (via Homebrew)

### Architectures
- ✅ **x86_64** (Intel/AMD 64-bit)
- ✅ **aarch64** (ARM 64-bit)
- ⚠️ **armv7** (ARM 32-bit) - Limited support

## Troubleshooting

### Permission Issues
```bash
# If installation fails due to permissions
sudo mkdir -p /usr/local/bin
sudo chown $USER:$USER /usr/local/bin

# Or install to user directory
curl -sSL https://raw.githubusercontent.com/ghostkellz/ghostctl/main/install.sh | bash -s -- --dir ~/.local/bin
```

### PATH Issues
```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
export PATH="/usr/local/bin:$PATH"

# Or for user installation
export PATH="$HOME/.local/bin:$PATH"
```

### Build Issues
```bash
# Update Rust toolchain
rustup update stable

# Clear cargo cache
cargo clean

# Install system dependencies (Debian/Ubuntu)
sudo apt install build-essential pkg-config libssl-dev
```

## Uninstallation

```bash
# Remove binary
sudo rm /usr/local/bin/ghostctl

# Remove user data (optional)
rm -rf ~/.config/ghostctl
rm -rf ~/.local/share/ghostctl

# Remove desktop entry and completions
sudo rm /usr/share/applications/ghostctl.desktop
sudo rm /usr/share/bash-completion/completions/ghostctl
sudo rm /usr/share/zsh/site-functions/_ghostctl
```

## Support

- 📚 **Documentation**: [GitHub Repository](https://github.com/ghostkellz/ghostctl)
- 🐛 **Issues**: [GitHub Issues](https://github.com/ghostkellz/ghostctl/issues)
- 🌐 **Website**: [ghostctl.sh](https://ghostctl.sh)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/ghostkellz/ghostctl/discussions)

---

For more information about using GhostCTL after installation, see the [main README](README.md) and [documentation](DOCS.md).