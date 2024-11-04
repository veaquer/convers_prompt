pkgname=convers_prompt
pkgver="0.1.6"
pkgrel=1
pkgdesc="prompt quick access gui app, that use convers magic_convert"
arch=('x86_64')
license=('MIT')
depends=('rust' 'cargo' 'translate-shell')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/veaquer/convers_prompt/archive/refs/tags/$pkgver.tar.gz")
sha256sums=('ff57c2a7bcfbe3690845aabef7a3df2d1e349f3d0b8c8419c3cd42fb66f293ff')

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
