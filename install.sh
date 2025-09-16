#!/bin/bash
# GhostCTL Universal Installation Script
# Supports: Arch Linux, Ubuntu/Debian, Fedora/RHEL/CentOS, openSUSE, Alpine, macOS
# Usage: curl -sSL https://ghostctl.io | bash

set -e

# Configuration
REPO="ghostkellz/ghostctl"
INSTALL_DIR="/usr/local/bin"
TEMP_DIR="/tmp/ghostctl-install"
VERSION="latest"
BINARY_NAME="ghostctl"
FORCE_METHOD=""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# ASCII Art Header
print_header() {
    echo -e "${PURPLE}${BOLD}"
    cat << 'EOF'
   ______ __               __   ______ _______ __
  / ____// /_   ____   ___/ /_ / ____//_  __/ / /
 / / __ / __ \ / __ \ / ___/ __// /     / /  / /
/ /_/ // / / // /_/ /(__  ) /_ / /___  / /  / /___
\____//_/ /_/ \____//____/ \__/ \____//_/  /_____/

EOF
    echo -e "${NC}${CYAN}👻 GhostCTL - Universal System Administration Toolkit${NC}"
    echo -e "${BLUE}🔗 https://github.com/$REPO${NC}"
    echo -e "${YELLOW}🌐 https://ghostctl.io${NC}"
    echo
}

# Logging functions
log_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }
log_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
log_error() { echo -e "${RED}❌ $1${NC}"; }
log_step() { echo -e "${CYAN}🔧 $1${NC}"; }

# Detect OS and Architecture
detect_system() {
    log_step "Detecting system information..."

    # Detect OS
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if command -v lsb_release &> /dev/null; then
            DISTRO=$(lsb_release -si | tr '[:upper:]' '[:lower:]')
            VERSION_ID=$(lsb_release -sr)
        elif [[ -f /etc/os-release ]]; then
            . /etc/os-release
            DISTRO=$(echo "$ID" | tr '[:upper:]' '[:lower:]')
            VERSION_ID="$VERSION_ID"
        elif [[ -f /etc/arch-release ]]; then
            DISTRO="arch"
        elif [[ -f /etc/debian_version ]]; then
            DISTRO="debian"
        elif [[ -f /etc/redhat-release ]]; then
            DISTRO="rhel"
        elif [[ -f /etc/alpine-release ]]; then
            DISTRO="alpine"
        else
            DISTRO="unknown"
        fi
        OS="linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
        DISTRO="macos"
    else
        OS="unknown"
        DISTRO="unknown"
    fi

    # Detect architecture
    ARCH=$(uname -m)
    case $ARCH in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        armv7l)
            ARCH="armv7"
            ;;
        *)
            log_error "Unsupported architecture: $ARCH"
            exit 1
            ;;
    esac

    # Check if running in container
    if [[ -f /.dockerenv ]] || grep -q 'docker\|lxc' /proc/1/cgroup 2>/dev/null; then
        CONTAINER="true"
    else
        CONTAINER="false"
    fi

    log_info "OS: $OS"
    log_info "Distribution: $DISTRO"
    log_info "Architecture: $ARCH"
    log_info "Container: $CONTAINER"
    echo
}

# Check if already installed
check_existing_installation() {
    if command -v ghostctl &> /dev/null; then
        CURRENT_VERSION=$(ghostctl --version 2>/dev/null | grep -o 'v[0-9.]*' || echo 'unknown')
        log_warning "GhostCTL is already installed (${CURRENT_VERSION})"
        echo
        if [[ "$FORCE_METHOD" != "force" ]]; then
            read -p "🤔 Reinstall/update? [y/N]: " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                log_info "Installation cancelled"
                exit 0
            fi
        fi
    fi
}

# Get latest version from GitHub API
get_latest_version() {
    if [[ "$VERSION" == "latest" ]]; then
        log_step "Fetching latest version..."
        if command -v curl &> /dev/null; then
            VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/' 2>/dev/null)
        elif command -v wget &> /dev/null; then
            VERSION=$(wget -qO- "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/' 2>/dev/null)
        fi

        if [[ -z "$VERSION" || "$VERSION" == "null" ]]; then
            log_warning "Failed to fetch latest version, using fallback"
            VERSION="v1.0.1"
        fi
        log_success "Latest version: $VERSION"
    fi
}

# Install via package manager (preferred method)
install_via_package_manager() {
    log_step "Attempting package manager installation..."

    case $DISTRO in
        arch|manjaro|endeavouros)
            if install_arch_package; then return 0; fi
            ;;
        ubuntu|debian|linuxmint|pop|elementary)
            if install_debian_package; then return 0; fi
            ;;
        fedora|rhel|centos|rocky|almalinux)
            if install_fedora_package; then return 0; fi
            ;;
        opensuse*|sles)
            if install_opensuse_package; then return 0; fi
            ;;
        alpine)
            if install_alpine_package; then return 0; fi
            ;;
        macos)
            if install_macos_package; then return 0; fi
            ;;
    esac

    return 1
}

# Arch Linux package installation
install_arch_package() {
    log_step "Trying Arch Linux package installation..."

    # Try AUR helpers first
    for helper in yay paru; do
        if command -v $helper &> /dev/null; then
            log_info "Installing via AUR ($helper)..."
            if $helper -S --noconfirm ghostctl 2>/dev/null; then
                log_success "Installed via AUR ($helper)"
                return 0
            fi
        fi
    done

    # Try official repos
    if sudo pacman -S ghostctl --noconfirm 2>/dev/null; then
        log_success "Installed via pacman"
        return 0
    fi

    log_warning "Package installation failed, trying binary download..."
    return 1
}

# Debian/Ubuntu package installation
install_debian_package() {
    log_step "Trying Debian/Ubuntu package installation..."

    # Check if our repository is available
    if [[ -f /etc/apt/sources.list.d/ghostctl.list ]]; then
        sudo apt update
        if sudo apt install -y ghostctl 2>/dev/null; then
            log_success "Installed via APT"
            return 0
        fi
    fi

    log_warning "Package installation failed, trying binary download..."
    return 1
}

# Fedora/RHEL package installation
install_fedora_package() {
    log_step "Trying Fedora/RHEL package installation..."

    # Try DNF/YUM with our repository
    if command -v dnf &> /dev/null; then
        PKG_MGR="dnf"
    else
        PKG_MGR="yum"
    fi

    if sudo $PKG_MGR install -y ghostctl 2>/dev/null; then
        log_success "Installed via $PKG_MGR"
        return 0
    fi

    log_warning "Package installation failed, trying binary download..."
    return 1
}

# openSUSE package installation
install_opensuse_package() {
    log_step "Trying openSUSE package installation..."

    if sudo zypper install -y ghostctl 2>/dev/null; then
        log_success "Installed via zypper"
        return 0
    fi

    log_warning "Package installation failed, trying binary download..."
    return 1
}

# Alpine package installation
install_alpine_package() {
    log_step "Trying Alpine package installation..."

    if sudo apk add ghostctl 2>/dev/null; then
        log_success "Installed via apk"
        return 0
    fi

    log_warning "Package installation failed, trying binary download..."
    return 1
}

# macOS package installation
install_macos_package() {
    log_step "Trying macOS package installation..."

    if command -v brew &> /dev/null; then
        # Try official tap first
        if brew install ghostkellz/tap/ghostctl 2>/dev/null; then
            log_success "Installed via Homebrew"
            return 0
        fi

        # Try cask
        if brew install --cask ghostctl 2>/dev/null; then
            log_success "Installed via Homebrew Cask"
            return 0
        fi
    fi

    log_warning "Package installation failed, trying binary download..."
    return 1
}

# Download and install binary
install_via_binary() {
    log_step "Installing via binary download..."

    # Create temp directory
    mkdir -p "$TEMP_DIR"
    cd "$TEMP_DIR"

    # Determine target triple
    case "$OS-$ARCH" in
        linux-x86_64)
            TARGET="x86_64-unknown-linux-gnu"
            ;;
        linux-aarch64)
            TARGET="aarch64-unknown-linux-gnu"
            ;;
        macos-x86_64)
            TARGET="x86_64-apple-darwin"
            ;;
        macos-aarch64)
            TARGET="aarch64-apple-darwin"
            ;;
        *)
            log_error "No binary available for $OS-$ARCH"
            return 1
            ;;
    esac

    # Download binary
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/ghostctl-$VERSION-$TARGET.tar.gz"
    log_info "Download URL: $DOWNLOAD_URL"

    if command -v curl &> /dev/null; then
        if ! curl -L -o "ghostctl.tar.gz" "$DOWNLOAD_URL"; then
            log_error "Failed to download binary"
            return 1
        fi
    elif command -v wget &> /dev/null; then
        if ! wget -O "ghostctl.tar.gz" "$DOWNLOAD_URL"; then
            log_error "Failed to download binary"
            return 1
        fi
    else
        log_error "No download tool available (curl or wget)"
        return 1
    fi

    # Extract and install
    tar -xzf "ghostctl.tar.gz"
    if [[ ! -f "$BINARY_NAME" ]]; then
        log_error "Binary not found in archive"
        return 1
    fi

    # Install binary
    if [[ -w "$INSTALL_DIR" ]]; then
        cp "$BINARY_NAME" "$INSTALL_DIR/"
        chmod +x "$INSTALL_DIR/$BINARY_NAME"
    else
        sudo cp "$BINARY_NAME" "$INSTALL_DIR/"
        sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
    fi

    log_success "Binary installed to $INSTALL_DIR/$BINARY_NAME"
    return 0
}

# Install dependencies for source build
install_build_dependencies() {
    log_step "Installing build dependencies..."

    case $DISTRO in
        arch|manjaro|endeavouros)
            sudo pacman -S --needed --noconfirm git rust cargo
            ;;
        ubuntu|debian|linuxmint|pop|elementary)
            sudo apt update
            sudo apt install -y git build-essential curl
            # Install Rust if not present
            if ! command -v cargo &> /dev/null; then
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                source ~/.cargo/env
            fi
            ;;
        fedora|rhel|centos|rocky|almalinux)
            if command -v dnf &> /dev/null; then
                sudo dnf install -y git rust cargo gcc
            else
                sudo yum install -y git rust cargo gcc
            fi
            ;;
        opensuse*|sles)
            sudo zypper install -y git rust cargo gcc
            ;;
        alpine)
            sudo apk add git rust cargo build-base
            ;;
        macos)
            if ! command -v brew &> /dev/null; then
                log_error "Homebrew required for macOS"
                return 1
            fi
            brew install rust git
            ;;
        *)
            log_error "Unknown distribution for dependency installation"
            return 1
            ;;
    esac
}

# Build from source (fallback method)
install_via_source() {
    log_step "Building from source..."

    # Install dependencies
    if ! install_build_dependencies; then
        log_error "Failed to install build dependencies"
        return 1
    fi

    # Clone repository
    cd "$TEMP_DIR"
    if [[ "$VERSION" == "latest" ]]; then
        git clone "https://github.com/$REPO.git" ghostctl-repo
    else
        git clone --branch "$VERSION" "https://github.com/$REPO.git" ghostctl-repo
    fi

    cd ghostctl-repo/ghostctl

    # Build
    log_info "Building (this may take several minutes)..."
    if ! cargo build --release; then
        log_error "Build failed"
        return 1
    fi

    # Install
    if [[ -w "$INSTALL_DIR" ]]; then
        cp target/release/ghostctl "$INSTALL_DIR/"
        chmod +x "$INSTALL_DIR/ghostctl"
    else
        sudo cp target/release/ghostctl "$INSTALL_DIR/"
        sudo chmod +x "$INSTALL_DIR/ghostctl"
    fi

    log_success "Built and installed from source"
    return 0
}

# Setup shell completions
setup_completions() {
    log_step "Setting up shell completions..."

    # Create completion directories
    sudo mkdir -p /usr/share/bash-completion/completions
    sudo mkdir -p /usr/share/zsh/site-functions
    sudo mkdir -p /usr/share/fish/vendor_completions.d

    # Generate completions if binary supports it
    if ghostctl --help | grep -q completion 2>/dev/null; then
        ghostctl completion bash 2>/dev/null | sudo tee /usr/share/bash-completion/completions/ghostctl > /dev/null
        ghostctl completion zsh 2>/dev/null | sudo tee /usr/share/zsh/site-functions/_ghostctl > /dev/null
        ghostctl completion fish 2>/dev/null | sudo tee /usr/share/fish/vendor_completions.d/ghostctl.fish > /dev/null
        log_success "Shell completions installed"
    fi
}

# Create desktop entry (Linux only)
create_desktop_entry() {
    if [[ "$OS" == "linux" ]]; then
        log_step "Creating desktop entry..."

        # Install icon if available
        if command -v ghostctl &> /dev/null; then
            GHOSTCTL_DIR=$(dirname "$(which ghostctl)")
            if [[ -f "$GHOSTCTL_DIR/../share/pixmaps/ghostctl.png" ]]; then
                sudo mkdir -p /usr/share/pixmaps
                sudo cp "$GHOSTCTL_DIR/../share/pixmaps/ghostctl.png" /usr/share/pixmaps/
            fi
        fi

        sudo tee /usr/share/applications/ghostctl.desktop > /dev/null << EOF
[Desktop Entry]
Name=GhostCTL
Comment=Universal System Administration Toolkit
GenericName=System Administration Tool
Exec=ghostctl menu
Icon=ghostctl
Terminal=true
Type=Application
Categories=System;Settings;Administration;Monitor;
Keywords=system;admin;terminal;homelab;automation;docker;network;security;
StartupNotify=false
StartupWMClass=ghostctl
EOF
        log_success "Desktop entry created"
    fi
}

# Setup user directories and configuration
setup_user_environment() {
    log_step "Setting up user environment..."

    # Create config directories
    mkdir -p ~/.config/ghostctl/{scripts,plugins,profiles}
    mkdir -p ~/.local/share/ghostctl/{logs,cache,data}

    # Create example script
    if [[ ! -f ~/.config/ghostctl/scripts/example.sh ]]; then
        cat > ~/.config/ghostctl/scripts/example.sh << 'EOF'
#!/bin/bash
# GhostCTL Example Script
# This script demonstrates how to create custom GhostCTL scripts

echo "🎉 Welcome to GhostCTL!"
echo "📁 Script location: ~/.config/ghostctl/scripts/"
echo "✏️  Edit this file to create your own automation scripts"
echo ""
echo "🚀 Available GhostCTL commands:"
echo "  ghostctl menu       - Interactive menu"
echo "  ghostctl help       - Show help"
echo "  ghostctl version    - Show version"
echo ""
echo "📚 Documentation: https://github.com/ghostkellz/ghostctl"
EOF
        chmod +x ~/.config/ghostctl/scripts/example.sh
    fi

    log_success "User environment configured"
}

# Verify installation
verify_installation() {
    log_step "Verifying installation..."

    if command -v ghostctl &> /dev/null; then
        INSTALLED_VERSION=$(ghostctl --version 2>/dev/null | head -1 || echo "unknown")
        log_success "Installation verified!"
        echo
        echo -e "${CYAN}📋 Installation Summary:${NC}"
        echo -e "  ${BOLD}Version:${NC} $INSTALLED_VERSION"
        echo -e "  ${BOLD}Location:${NC} $(which ghostctl)"
        echo -e "  ${BOLD}Method:${NC} $INSTALL_METHOD"
        echo
        echo -e "${BLUE}🚀 Quick Start:${NC}"
        echo -e "  ${CYAN}ghostctl${NC}              - Launch interactive menu"
        echo -e "  ${CYAN}ghostctl --help${NC}       - Show help"
        echo -e "  ${CYAN}ghostctl version${NC}      - Show version info"
        echo -e "  ${CYAN}ghostctl menu${NC}         - Main navigation menu"
        echo
        echo -e "${BLUE}📚 Resources:${NC}"
        echo -e "  ${CYAN}Documentation:${NC} https://github.com/$REPO"
        echo -e "  ${CYAN}Issues:${NC}        https://github.com/$REPO/issues"
        echo -e "  ${CYAN}Website:${NC}       https://ghostctl.io"
        return 0
    else
        log_error "Installation verification failed"
        echo -e "${YELLOW}💡 You may need to add $INSTALL_DIR to your PATH${NC}"
        echo -e "${YELLOW}💡 Try running: export PATH=\"$INSTALL_DIR:\$PATH\"${NC}"
        return 1
    fi
}

# Cleanup temporary files
cleanup() {
    if [[ -d "$TEMP_DIR" ]]; then
        log_step "Cleaning up temporary files..."
        rm -rf "$TEMP_DIR"
    fi
}

# Show help
show_help() {
    cat << EOF
GhostCTL Universal Installation Script

USAGE:
    $0 [OPTIONS]

OPTIONS:
    -h, --help              Show this help message
    -v, --version VERSION   Install specific version (default: latest)
    -d, --dir DIRECTORY     Installation directory (default: /usr/local/bin)
    -m, --method METHOD     Installation method: auto, package, binary, source
    -f, --force             Force reinstallation without prompting
    --no-completions        Skip shell completion setup
    --no-desktop            Skip desktop entry creation

EXAMPLES:
    # Standard installation
    curl -sSL https://ghostctl.io | bash

    # Install specific version
    curl -sSL https://ghostctl.io | bash -s -- --version v1.0.0

    # Install to custom directory
    curl -sSL https://ghostctl.io | bash -s -- --dir ~/.local/bin

    # Force binary installation
    curl -sSL https://ghostctl.io | bash -s -- --method binary

ENVIRONMENT VARIABLES:
    GHOSTCTL_INSTALL_DIR    Installation directory
    GHOSTCTL_VERSION        Version to install
    GHOSTCTL_METHOD         Installation method

For more information, visit: https://github.com/$REPO
EOF
}

# Main installation function
main() {
    local setup_completions_flag=true
    local setup_desktop_flag=true

    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -v|--version)
                VERSION="$2"
                shift 2
                ;;
            -d|--dir)
                INSTALL_DIR="$2"
                shift 2
                ;;
            -m|--method)
                FORCE_METHOD="$2"
                shift 2
                ;;
            -f|--force)
                FORCE_METHOD="force"
                shift
                ;;
            --no-completions)
                setup_completions_flag=false
                shift
                ;;
            --no-desktop)
                setup_desktop_flag=false
                shift
                ;;
            *)
                log_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done

    # Override with environment variables
    INSTALL_DIR="${GHOSTCTL_INSTALL_DIR:-$INSTALL_DIR}"
    VERSION="${GHOSTCTL_VERSION:-$VERSION}"
    FORCE_METHOD="${GHOSTCTL_METHOD:-$FORCE_METHOD}"

    # Start installation
    print_header
    detect_system
    check_existing_installation
    get_latest_version

    # Try installation methods in order
    INSTALL_METHOD="unknown"

    case "$FORCE_METHOD" in
        package)
            if install_via_package_manager; then
                INSTALL_METHOD="package"
            else
                log_error "Package installation failed"
                exit 1
            fi
            ;;
        binary)
            if install_via_binary; then
                INSTALL_METHOD="binary"
            else
                log_error "Binary installation failed"
                exit 1
            fi
            ;;
        source)
            if install_via_source; then
                INSTALL_METHOD="source"
            else
                log_error "Source installation failed"
                exit 1
            fi
            ;;
        *)
            # Auto-detect best method
            if install_via_package_manager; then
                INSTALL_METHOD="package"
            elif install_via_binary; then
                INSTALL_METHOD="binary"
            elif install_via_source; then
                INSTALL_METHOD="source"
            else
                log_error "All installation methods failed"
                exit 1
            fi
            ;;
    esac

    # Post-installation setup
    if [[ "$setup_completions_flag" == true ]]; then
        setup_completions
    fi

    if [[ "$setup_desktop_flag" == true ]]; then
        create_desktop_entry
    fi

    setup_user_environment

    # Verify and cleanup
    if verify_installation; then
        cleanup
        log_success "🎉 GhostCTL installation completed successfully!"
        echo
    else
        cleanup
        exit 1
    fi
}

# Handle script interruption
trap cleanup EXIT

# Execute main function with all arguments
main "$@"