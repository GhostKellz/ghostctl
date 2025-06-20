#!/bin/bash
# GhostCTL Universal Installation Script
# Supports: Arch Linux, Ubuntu/Debian, Fedora/RHEL, macOS, and manual builds

set -e

REPO="ghostkellz/ghostctl"
INSTALL_DIR="/usr/local/bin"
TEMP_DIR="/tmp/ghostctl-install"
VERSION="latest"
BINARY_NAME="ghostctl"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${PURPLE}"
    echo "████████  ██     ██  ████████  ████████  ██████████  ████████  ██████████  ██"
    echo "██        ██     ██  ██    ██  ██           ██       ██           ██      ██"
    echo "██  ████  ███████    ██    ██  ████████     ██       ██           ██      ██"
    echo "██    ██  ██     ██  ██    ██        ██     ██       ██           ██      ██"
    echo "████████  ██     ██  ████████  ████████     ██       ████████     ██      ████████"
    echo -e "${NC}"
    echo -e "${CYAN}👻 GhostCTL v0.5.0 - System Administration Toolkit${NC}"
    echo -e "${BLUE}🔗 https://github.com/$REPO${NC}"
    echo
}

detect_os_arch() {
    # Detect OS
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if [[ -f /etc/arch-release ]]; then
            OS="arch"
        elif [[ -f /etc/debian_version ]]; then
            OS="debian"
        elif [[ -f /etc/redhat-release ]] || [[ -f /etc/centos-release ]]; then
            OS="redhat"
        else
            OS="linux"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
    else
        OS="unknown"
    fi

    # Detect architecture
    ARCH=$(uname -m)
    case $ARCH in
        x86_64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            echo -e "${RED}❌ Unsupported architecture: $ARCH${NC}"
            exit 1
            ;;
    esac

    echo -e "${BLUE}ℹ️  Detected OS: $OS${NC}"
    echo -e "${BLUE}ℹ️  Detected Architecture: $ARCH${NC}"
}

get_latest_version() {
    if [[ "$VERSION" == "latest" ]]; then
        echo -e "${YELLOW}🔍 Fetching latest version...${NC}"
        VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
        if [[ -z "$VERSION" ]]; then
            echo -e "${RED}❌ Failed to fetch latest version${NC}"
            exit 1
        fi
        echo -e "${GREEN}✅ Latest version: $VERSION${NC}"
    fi
}

download_binary() {
    echo -e "${YELLOW}📥 Downloading GhostCTL binary...${NC}"
    
    # Determine the target triple based on OS and architecture
    if [[ "$OS" == "linux" || "$OS" == "arch" || "$OS" == "debian" || "$OS" == "redhat" ]]; then
        if [[ "$ARCH" == "x86_64" ]]; then
            TARGET="x86_64-unknown-linux-gnu"
        else
            TARGET="aarch64-unknown-linux-gnu"
        fi
    elif [[ "$OS" == "macos" ]]; then
        if [[ "$ARCH" == "x86_64" ]]; then
            TARGET="x86_64-apple-darwin"
        else
            TARGET="aarch64-apple-darwin"
        fi
    else
        echo -e "${RED}❌ Unsupported OS: $OS${NC}"
        exit 1
    fi

    # Create temp directory
    mkdir -p "$TEMP_DIR"
    cd "$TEMP_DIR"

    # Download the archive
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/ghostctl-$VERSION-$TARGET.tar.gz"
    echo -e "${BLUE}🔗 Download URL: $DOWNLOAD_URL${NC}"
    
    if ! curl -L -o "ghostctl.tar.gz" "$DOWNLOAD_URL"; then
        echo -e "${RED}❌ Failed to download binary${NC}"
        echo -e "${YELLOW}💡 Falling back to building from source...${NC}"
        build_from_source
        return
    fi

    # Extract the archive
    tar -xzf "ghostctl.tar.gz"
    
    if [[ ! -f "$BINARY_NAME" ]]; then
        echo -e "${RED}❌ Binary not found in archive${NC}"
        echo -e "${YELLOW}💡 Falling back to building from source...${NC}"
        build_from_source
        return
    fi

    echo -e "${GREEN}✅ Binary downloaded successfully${NC}"
}

build_from_source() {
    echo -e "${YELLOW}🔨 Building GhostCTL from source...${NC}"
    
    # Check for required tools
    if ! command -v git &> /dev/null; then
        echo -e "${RED}❌ git is required but not installed${NC}"
        install_dependencies
    fi
    
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ Rust/Cargo is required but not installed${NC}"
        install_dependencies
    fi

    # Clone the repository
    cd "$TEMP_DIR"
    if [[ "$VERSION" == "latest" ]]; then
        git clone "https://github.com/$REPO.git" ghostctl-repo
    else
        git clone --branch "$VERSION" "https://github.com/$REPO.git" ghostctl-repo
    fi
    
    cd ghostctl-repo/ghostctl
    
    # Build the project
    cargo build --release
    
    # Copy the binary
    cp target/release/ghostctl "../$BINARY_NAME"
    cd ..
    
    echo -e "${GREEN}✅ Built from source successfully${NC}"
}

install_dependencies() {
    echo -e "${YELLOW}📦 Installing dependencies...${NC}"
    
    case $OS in
        "arch")
            sudo pacman -S --needed --noconfirm git rust cargo curl
            ;;
        "debian")
            sudo apt update
            sudo apt install -y git cargo rustc curl build-essential
            ;;
        "redhat")
            if command -v dnf &> /dev/null; then
                sudo dnf install -y git cargo rust curl gcc
            else
                sudo yum install -y git cargo rust curl gcc
            fi
            ;;
        "macos")
            if ! command -v brew &> /dev/null; then
                echo -e "${RED}❌ Homebrew is required for macOS installation${NC}"
                echo -e "${BLUE}ℹ️  Install Homebrew: /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"${NC}"
                exit 1
            fi
            brew install rust git curl
            ;;
        *)
            echo -e "${RED}❌ Unsupported OS for automatic dependency installation${NC}"
            echo -e "${BLUE}ℹ️  Please install git, rust, and cargo manually${NC}"
            exit 1
            ;;
    esac
}

install_binary() {
    echo -e "${YELLOW}📦 Installing GhostCTL...${NC}"
    
    # Check if we need sudo
    if [[ -w "$INSTALL_DIR" ]]; then
        cp "$BINARY_NAME" "$INSTALL_DIR/"
    else
        sudo cp "$BINARY_NAME" "$INSTALL_DIR/"
    fi
    
    # Make it executable
    if [[ -w "$INSTALL_DIR/$BINARY_NAME" ]]; then
        chmod +x "$INSTALL_DIR/$BINARY_NAME"
    else
        sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
    fi
    
    echo -e "${GREEN}✅ GhostCTL installed to $INSTALL_DIR/$BINARY_NAME${NC}"
}

verify_installation() {
    echo -e "${YELLOW}🔍 Verifying installation...${NC}"
    
    if command -v ghostctl &> /dev/null; then
        echo -e "${GREEN}✅ Installation verified!${NC}"
        echo -e "${CYAN}Version information:${NC}"
        ghostctl version
        echo
        echo -e "${BLUE}🚀 Quick start:${NC}"
        echo -e "  ${CYAN}ghostctl${NC}           - Launch interactive menu"
        echo -e "  ${CYAN}ghostctl --help${NC}     - Show help"
        echo -e "  ${CYAN}ghostctl version${NC}    - Show version"
        echo -e "  ${CYAN}ghostctl dev menu${NC}   - Development environment"
        echo -e "  ${CYAN}ghostctl docker menu${NC} - Docker management"
    else
        echo -e "${RED}❌ Installation verification failed${NC}"
        echo -e "${YELLOW}💡 You may need to add $INSTALL_DIR to your PATH${NC}"
        exit 1
    fi
}

cleanup() {
    echo -e "${YELLOW}🧹 Cleaning up...${NC}"
    rm -rf "$TEMP_DIR"
}

main() {
    print_header
    
    # Check for help flag
    if [[ $1 == "--help" || $1 == "-h" ]]; then
        echo "GhostCTL Universal Installation Script"
        echo
        echo "Usage: $0 [options]"
        echo
        echo "Options:"
        echo "  --help, -h        Show this help message"
        echo "  --version VER     Install specific version (default: latest)"
        echo "  --install-dir DIR Installation directory (default: /usr/local/bin)"
        echo
        echo "Environment Variables:"
        echo "  INSTALL_DIR       Installation directory"
        echo
        echo "Examples:"
        echo "  $0                          # Install latest version"
        echo "  $0 --version v0.5.0         # Install specific version"
        echo "  $0 --install-dir ~/.local/bin # Install to custom directory"
        echo
        exit 0
    fi

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --version)
                VERSION="$2"
                shift 2
                ;;
            --install-dir)
                INSTALL_DIR="$2"
                shift 2
                ;;
            *)
                echo -e "${RED}❌ Unknown option: $1${NC}"
                exit 1
                ;;
        esac
    done

    detect_os_arch
    get_latest_version
    download_binary
    install_binary
    verify_installation
    cleanup
    
    echo -e "${GREEN}🎉 GhostCTL installation completed successfully!${NC}"
}

# Execute main function with all arguments
main "$@"
