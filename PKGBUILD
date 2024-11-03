pkgname=convers_prompt
pkgver="0.1.5"
pkgrel=1
pkgdesc="prompt quick access gui app, that use convers magic_convert"
arch=('x86_64')
license=('MIT')
depends=('rust' 'cargo' 'translate-shell')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/veaquer/convers_prompt/archive/refs/tags/$pkgver.tar.gz")
sha256sums=('7196cacfad80067e1c7c734b3472db8ec75a510043b9b2b793f89f86dc666e5b')

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
