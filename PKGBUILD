# Maintainer: Christopher Kelley <ckelley@ghostctl.sh>
# Contributor: CK Technology LLC

pkgname=ghostctl
pkgver=0.9.5
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
source=("$pkgname-$pkgver.tar.gz::https://github.com/ghostkellz/ghostctl/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')  # This will be updated by CI/CD

prepare() {
    cd "$pkgname-$pkgver"
    # Update Cargo.lock if needed
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$pkgname-$pkgver/ghostctl"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    cd "$pkgname-$pkgver/ghostctl"
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    cd "$pkgname-$pkgver"
    
    # Install binary
    install -Dm755 "ghostctl/target/release/ghostctl" "$pkgdir/usr/bin/ghostctl"
    
    # Install documentation
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 DOCS.md "$pkgdir/usr/share/doc/$pkgname/DOCS.md"
    install -Dm644 COMMANDS.md "$pkgdir/usr/share/doc/$pkgname/COMMANDS.md"

    # Install desktop entry and icon
    install -Dm644 packaging/ghostctl.desktop "$pkgdir/usr/share/applications/ghostctl.desktop"
    install -Dm644 assets/icons/png/ghostctl-icon-48.png "$pkgdir/usr/share/pixmaps/ghostctl.png"
    
    # Install scripts directory (optional)
    if [ -d "scripts" ]; then
        cp -r scripts "$pkgdir/usr/share/doc/$pkgname/"
    fi
}
