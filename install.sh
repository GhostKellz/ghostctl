#!/bin/bash
# GhostCTL Installation Script
# Supports: Arch Linux, Ubuntu/Debian, Fedora/RHEL, and manual builds

set -e

REPO="ghostkellz/ghostctl"
INSTALL_DIR="/usr/local/bin"
TEMP_DIR="/tmp/ghostctl-install"
VERSION="latest"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${BLUE}"
    echo "████████  ██     ██  ████████  ████████  ██████████  ████████  ██████████  ██"
    echo "██        ██     ██  ██    ██  ██           ██       ██           ██      ██"
    echo "██  ████  ███████    ██    ██  ████████     ██       ██           ██      ██"
    echo "██    ██  ██     ██  ██    ██        ██     ██       ██           ██      ██"
    echo "████████  ██     ██  ████████  ████████     ██       ████████     ██      ████████"
    echo -e "${NC}"
    echo "👻 GhostCTL - System Administration Toolkit"
    echo "🔗 https://github.com/$REPO"
    echo
}

detect_os() {
    if [[ -f /etc/arch-release ]]; then
        OS="arch"
    elif [[ -f /etc/debian_version ]]; then
        OS="debian"
    elif [[ -f /etc/redhat-release ]]; then
        OS="redhat"
    else
        OS="unknown"
    fi
    echo -e "${BLUE}ℹ️  Detected OS: $OS${NC}"
}

install_dependencies() {
    echo -e "${YELLOW}📦 Installing dependencies...${NC}"
    
    case $OS in
        "arch")
            sudo pacman -S --needed --noconfirm git rust cargo curl
            ;;
        "debian")
            sudo apt update
            sudo apt install -y git build-essential curl
            # Install Rust via rustup if not present
            if ! command -v cargo &> /dev/null; then
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                source ~/.cargo/env
            fi
            ;;
        "redhat")
            sudo dnf install -y git rust cargo curl gcc
            ;;
        *)
            echo -e "${YELLOW}⚠️  Unknown OS. Please install: git, rust, cargo manually${NC}"
            ;;
    esac
}

download_and_build() {
    echo -e "${YELLOW}🔧 Building GhostCTL from source...${NC}"
    
    # Clean temp directory
    rm -rf "$TEMP_DIR"
    mkdir -p "$TEMP_DIR"
    cd "$TEMP_DIR"
    
    # Clone repository
    echo "📥 Cloning repository..."
    git clone "https://github.com/$REPO.git" .
    
    # Build
    echo "🔨 Building (this may take a few minutes)..."
    cd ghostctl
    cargo build --release
    
    # Install
    echo "📦 Installing to $INSTALL_DIR..."
    sudo install -Dm755 target/release/ghostctl "$INSTALL_DIR/ghostctl"
    
    # Install completion scripts if available
    if [[ -d completions ]]; then
        echo "🔧 Installing shell completions..."
        sudo mkdir -p /usr/share/bash-completion/completions
        sudo mkdir -p /usr/share/zsh/site-functions
        sudo cp completions/ghostctl.bash /usr/share/bash-completion/completions/ghostctl 2>/dev/null || true
        sudo cp completions/_ghostctl /usr/share/zsh/site-functions/_ghostctl 2>/dev/null || true
    fi
}

create_desktop_entry() {
    echo "🖥️  Creating desktop entry..."
    sudo tee /usr/share/applications/ghostctl.desktop > /dev/null <<EOF
[Desktop Entry]
Name=GhostCTL
Comment=System Administration Toolkit
Exec=ghostctl menu
Icon=utilities-terminal
Terminal=true
Type=Application
Categories=System;Settings;
Keywords=system;admin;terminal;
EOF
}

setup_user_directories() {
    echo "📁 Setting up user directories..."
    mkdir -p ~/.config/ghostctl/{scripts,plugins}
    mkdir -p ~/.local/share/ghostctl
    
    # Create example local script
    cat > ~/.config/ghostctl/scripts/example.sh <<'EOF'
#!/bin/bash
# Example local script
echo "🎉 This is an example local script!"
echo "📁 Located in: ~/.config/ghostctl/scripts/"
echo "✏️  Edit this file to create your own scripts"
EOF
    chmod +x ~/.config/ghostctl/scripts/example.sh
}

install_aur_package() {
    if command -v yay &> /dev/null; then
        echo -e "${YELLOW}📦 Installing via AUR (yay)...${NC}"
        yay -S ghostctl
        return 0
    elif command -v paru &> /dev/null; then
        echo -e "${YELLOW}📦 Installing via AUR (paru)...${NC}"
        paru -S ghostctl
        return 0
    else
        echo -e "${YELLOW}⚠️  No AUR helper found${NC}"
        return 1
    fi
}

main() {
    print_header
    
    # Check if already installed
    if command -v ghostctl &> /dev/null; then
        echo -e "${GREEN}✅ GhostCTL is already installed!${NC}"
        echo "🔄 Current version: $(ghostctl --version 2>/dev/null || echo 'unknown')"
        echo
        read -p "🤔 Reinstall/update? [y/N]: " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "👋 Installation cancelled"
            exit 0
        fi
    fi
    
    detect_os
    
    # Try AUR first for Arch users
    if [[ $OS == "arch" ]]; then
        echo "🎯 Attempting AUR installation first..."
        if install_aur_package; then
            echo -e "${GREEN}✅ Installed via AUR!${NC}"
            setup_user_directories
            echo -e "${GREEN}🎉 Installation complete!${NC}"
            echo "🚀 Run 'ghostctl menu' to get started"
            exit 0
        else
            echo "⚠️  AUR installation failed, falling back to source build..."
        fi
    fi
    
    # Source installation
    install_dependencies
    download_and_build
    create_desktop_entry
    setup_user_directories
    
    # Cleanup
    rm -rf "$TEMP_DIR"
    
    echo -e "${GREEN}✅ GhostCTL installation complete!${NC}"
    echo
    echo "🚀 Getting Started:"
    echo "  • Run: ghostctl menu"
    echo "  • Config: ghostctl config show"  
    echo "  • Help: ghostctl --help"
    echo
    echo "📚 Documentation: https://github.com/$REPO"
    echo "🐛 Issues: https://github.com/$REPO/issues"
    
    # Verify installation
    if command -v ghostctl &> /dev/null; then
        echo -e "${GREEN}✅ Installation verified!${NC}"
        ghostctl --version
    else
        echo -e "${RED}❌ Installation may have failed${NC}"
        exit 1
    fi
}

# Check for help flag
if [[ $1 == "--help" || $1 == "-h" ]]; then
    echo "GhostCTL Installation Script"
    echo
    echo "Usage: $0 [options]"
    echo
    echo "Options:"
    echo "  --help, -h     Show this help message"
    echo "  --version VER  Install specific version (default: latest)"
    echo
    echo "Environment Variables:"
    echo "  INSTALL_DIR    Installation directory (default: /usr/local/bin)"
    echo
    exit 0
fi

# Parse version if provided
if [[ $1 == "--version" ]]; then
    VERSION="$2"
fi

main "$@"