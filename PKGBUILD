pkgname=convers_prompt
pkgver="latest"
pkgrel=1
pkgdesc="prompt quick access gui app, that use convers magic_convert"
arch=('x86_64')
license=('MIT')
depends=('rust' 'cargo' 'translate-shell')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/veaquer/convers_prompt/archive/refs/tags/$pkgver.tar.gz")
sha256sums=('cf6bcef9611ce5cad2bc4c7dfc4e397784a33b5fe10a8dfdcb4a7dd804f2c2e1')

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
