# Maintainer: Your Name <your@email.com>
# Contributor: CK Technology LLC
pkgname=ghostctl
pkgver=0.1.0
pkgrel=1
pkgdesc="Modular CLI toolkit for Linux sysadmins, homelabbers, and power users."
arch=('x86_64')
url="https://github.com/ghostkellz/ghostctl"
license=('MIT')
depends=('lua54' 'pkgconf' 'git')
makedepends=('cargo' 'rust')
source=("$pkgname::git+https://github.com/ghostkellz/ghostctl.git#tag=v${pkgver}")
b2sums=('SKIP')

build() {
  cd "$srcdir/$pkgname"
  cargo build --release --locked
}

package() {
  cd "$srcdir/$pkgname"
  install -Dm755 target/release/ghostctl "$pkgdir/usr/bin/ghostctl"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
