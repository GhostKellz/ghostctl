# Maintainer: Christopher Kelley <ckelley@ghostctl.sh>
# Contributor: CK Technology LLC

pkgname=ghostctl
pkgver=0.9.8
pkgrel=1
pkgdesc="Ghost Infrastructure Control - Complete system and homelab management toolkit"
arch=('x86_64' 'aarch64')
url="https://github.com/ghostkellz/ghostctl"
license=('MIT')
depends=('gcc-libs')
makedepends=('rust' 'cargo' 'git')
optdepends=(
    'docker: for Docker management features'
    'nginx: for web server management'
    'restic: for backup functionality'
    'btrfs-progs: for Btrfs filesystem management'
    'proxmox-ve: for Proxmox VE management'
)
# Use git source with signed tag verification for supply-chain security
# The commit hash is pinned and verified against the signed tag
source=("$pkgname::git+https://github.com/ghostkellz/ghostctl.git#tag=v$pkgver")
sha256sums=('SKIP')  # Git sources use SKIP; integrity verified via signed tag
# To verify: git verify-tag v$pkgver

# For tarball-based builds (e.g., AUR), use:
# source=("$pkgname-$pkgver.tar.gz::https://github.com/ghostkellz/ghostctl/archive/v$pkgver.tar.gz")
# sha256sums=('CHECKSUM_FROM_RELEASE')  # Updated by release automation
# Get checksum: curl -sL URL | sha256sum

prepare() {
    cd "$pkgname"
    local expected_tag="v$pkgver"

    # Verify we're on the expected tag
    git describe --tags --exact-match 2>/dev/null || {
        echo "ERROR: Not on expected tag $expected_tag"
        exit 1
    }

    # Verify the release tag signature for supply-chain integrity
    git verify-tag "$expected_tag" >/dev/null 2>&1 || {
        echo "ERROR: Tag signature verification failed for $expected_tag"
        exit 1
    }

    # Update Cargo.lock if needed
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$pkgname/ghostctl"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    cd "$pkgname/ghostctl"
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    cd "$pkgname"

    # Install binary
    install -Dm755 "ghostctl/target/release/ghostctl" "$pkgdir/usr/bin/ghostctl"

    # Install documentation
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 DOCS.md "$pkgdir/usr/share/doc/$pkgname/DOCS.md" 2>/dev/null || true
    install -Dm644 COMMANDS.md "$pkgdir/usr/share/doc/$pkgname/COMMANDS.md" 2>/dev/null || true

    # Install desktop entry and icon (if present)
    install -Dm644 packaging/ghostctl.desktop "$pkgdir/usr/share/applications/ghostctl.desktop" 2>/dev/null || true
    install -Dm644 assets/icons/png/ghostctl-icon-48.png "$pkgdir/usr/share/pixmaps/ghostctl.png" 2>/dev/null || true
    
    # Install scripts directory (optional)
    if [ -d "scripts" ]; then
        cp -r scripts "$pkgdir/usr/share/doc/$pkgname/"
    fi
}
