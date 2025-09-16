#!/bin/bash
# GhostCTL Package Building Script
# Builds packages for Arch (PKGBUILD), Debian/Ubuntu (deb), and Fedora/RHEL (rpm)

set -e

# Configuration
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="$PROJECT_ROOT/build"
PACKAGING_DIR="$PROJECT_ROOT/packaging"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }
log_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
log_error() { echo -e "${RED}❌ $1${NC}"; }

# Detect current distribution
detect_distro() {
    if [[ -f /etc/arch-release ]]; then
        DISTRO="arch"
    elif [[ -f /etc/debian_version ]]; then
        DISTRO="debian"
    elif [[ -f /etc/redhat-release ]] || [[ -f /etc/fedora-release ]]; then
        DISTRO="fedora"
    else
        DISTRO="unknown"
    fi
    log_info "Detected distribution: $DISTRO"
}

# Get version from Cargo.toml
get_version() {
    VERSION=$(grep '^version = ' "$PROJECT_ROOT/ghostctl/Cargo.toml" | sed 's/version = "\(.*\)"/\1/')
    log_info "Package version: $VERSION"
}

# Build Arch package
build_arch_package() {
    log_info "Building Arch Linux package..."

    if ! command -v makepkg &> /dev/null; then
        log_error "makepkg not found. Please install base-devel package."
        return 1
    fi

    cd "$PROJECT_ROOT"

    # Update PKGBUILD version
    sed -i "s/^pkgver=.*/pkgver=$VERSION/" PKGBUILD

    # Build package
    makepkg -sf --noconfirm

    # Move package to build directory
    mkdir -p "$BUILD_DIR/arch"
    mv ghostctl-*.pkg.tar.* "$BUILD_DIR/arch/" 2>/dev/null || true

    log_success "Arch package built successfully"
}

# Build Debian package
build_debian_package() {
    log_info "Building Debian package..."

    if ! command -v dpkg-buildpackage &> /dev/null; then
        log_error "dpkg-buildpackage not found. Please install build-essential and devscripts."
        return 1
    fi

    # Prepare build environment
    mkdir -p "$BUILD_DIR/debian"
    cd "$BUILD_DIR/debian"

    # Copy source
    cp -r "$PROJECT_ROOT" "ghostctl-$VERSION"
    cd "ghostctl-$VERSION"

    # Copy debian packaging files
    cp -r "$PACKAGING_DIR/debian" .

    # Update version in changelog
    sed -i "s/^ghostctl (.*)/ghostctl ($VERSION-1)/" debian/changelog

    # Build package
    dpkg-buildpackage -us -uc -b

    # Move packages
    cd ..
    mkdir -p "$BUILD_DIR/debian/packages"
    mv *.deb "$BUILD_DIR/debian/packages/" 2>/dev/null || true
    mv *.changes "$BUILD_DIR/debian/packages/" 2>/dev/null || true

    log_success "Debian package built successfully"
}

# Build RPM package
build_fedora_package() {
    log_info "Building RPM package..."

    if ! command -v rpmbuild &> /dev/null; then
        log_error "rpmbuild not found. Please install rpm-build and rpmdevtools."
        return 1
    fi

    # Setup RPM build environment
    mkdir -p "$BUILD_DIR/rpm"/{BUILD,BUILDROOT,RPMS,SOURCES,SPECS,SRPMS}

    # Create source tarball
    cd "$PROJECT_ROOT/.."
    tar -czf "$BUILD_DIR/rpm/SOURCES/ghostctl-$VERSION.tar.gz" \
        --transform "s|^ghostctl|ghostctl-$VERSION|" \
        --exclude='*.git*' \
        --exclude='build' \
        --exclude='target' \
        ghostctl/

    # Copy spec file and update version
    cp "$PACKAGING_DIR/ghostctl.spec" "$BUILD_DIR/rpm/SPECS/"
    sed -i "s/^Version:.*/Version:        $VERSION/" "$BUILD_DIR/rpm/SPECS/ghostctl.spec"

    # Build RPM
    cd "$BUILD_DIR/rpm"
    rpmbuild --define "_topdir $(pwd)" -ba SPECS/ghostctl.spec

    # Move packages
    mkdir -p "$BUILD_DIR/rpm/packages"
    mv RPMS/*/*.rpm "$BUILD_DIR/rpm/packages/" 2>/dev/null || true
    mv SRPMS/*.rpm "$BUILD_DIR/rpm/packages/" 2>/dev/null || true

    log_success "RPM package built successfully"
}

# Install dependencies for building
install_build_deps() {
    log_info "Installing build dependencies..."

    case $DISTRO in
        arch)
            sudo pacman -S --needed --noconfirm base-devel rust cargo git
            ;;
        debian)
            sudo apt update
            sudo apt install -y build-essential devscripts debhelper-compat cargo rustc git pkg-config libssl-dev
            ;;
        fedora)
            if command -v dnf &> /dev/null; then
                sudo dnf install -y rpm-build rpmdevtools rust cargo git gcc openssl-devel pkg-config
            else
                sudo yum install -y rpm-build rpmdevtools rust cargo git gcc openssl-devel pkg-config
            fi
            ;;
        *)
            log_error "Unknown distribution for dependency installation"
            return 1
            ;;
    esac
}

# Main function
main() {
    echo "🏗️  GhostCTL Package Builder"
    echo "=========================="
    echo

    detect_distro
    get_version

    # Parse arguments
    BUILD_TYPE="${1:-$DISTRO}"

    case "$BUILD_TYPE" in
        --help|-h)
            cat << EOF
GhostCTL Package Building Script

USAGE:
    $0 [PACKAGE_TYPE]

PACKAGE_TYPES:
    arch      Build Arch Linux package (PKGBUILD)
    debian    Build Debian/Ubuntu package (.deb)
    fedora    Build Fedora/RHEL package (.rpm)
    all       Build all package types
    deps      Install build dependencies

EXAMPLES:
    $0 arch       # Build only Arch package
    $0 debian     # Build only Debian package
    $0 fedora     # Build only RPM package
    $0 all        # Build all packages
    $0 deps       # Install dependencies

If no argument is provided, builds package for current distribution.
EOF
            exit 0
            ;;
        deps)
            install_build_deps
            exit 0
            ;;
        arch)
            install_build_deps
            build_arch_package
            ;;
        debian)
            install_build_deps
            build_debian_package
            ;;
        fedora|rpm)
            install_build_deps
            build_fedora_package
            ;;
        all)
            if [[ "$DISTRO" == "arch" ]]; then
                install_build_deps
                build_arch_package
            elif [[ "$DISTRO" == "debian" ]]; then
                install_build_deps
                build_debian_package
            elif [[ "$DISTRO" == "fedora" ]]; then
                install_build_deps
                build_fedora_package
            else
                log_error "Cannot build all packages on unknown distribution"
                exit 1
            fi
            ;;
        *)
            log_error "Unknown package type: $BUILD_TYPE"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac

    echo
    log_success "Package building completed!"
    log_info "Built packages are in: $BUILD_DIR/"
    echo
    echo "🚀 Next steps:"
    echo "  • Test installation: sudo pacman -U (Arch) / sudo dpkg -i (Debian) / sudo rpm -i (Fedora)"
    echo "  • Upload to repositories or distribute manually"
    echo "  • Update installer script to reference package repositories"
}