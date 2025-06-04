# Maintainer: Christopher Kelley <ckelley@ghostkellz.sh>
# Contributor: CK Technology LLC

pkgname=ghostctl
pkgver=0.1.0
pkgrel=1
pkgdesc="Modular CLI toolkit for Linux sysadmins, homelabbers, and power users."
arch=('x86_64')
url="https://github.com/ghostkellz/ghostctl"
license=('MIT')
depends=('rust' 'pkgconf' 'git')
makedepends=('cargo')
source=()
noextract=()
b2sums=()

build() {
  cd "$startdir/ghostctl"
  cargo build --release --locked
}

package() {
  cd "$startdir/ghostctl"
  install -Dm755 target/release/ghostctl "$pkgdir/usr/bin/ghostctl"
  install -Dm644 "$startdir/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 "$startdir/README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
}
