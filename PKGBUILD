pkgname=convers_prompt
pkgver="0.1.9"
pkgrel=1
pkgdesc="prompt quick access gui app, that use convers magic_convert"
arch=('x86_64')
license=('MIT')
depends=('rust' 'cargo')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/veaquer/convers_prompt/archive/refs/tags/$pkgver.tar.gz")
sha256sums=('f3183b3570c9e57007ece419e9bdff7d225790ede5e46e6286a8959a0db7fe1f')

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
