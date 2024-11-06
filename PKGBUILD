pkgname=convers_prompt
pkgver="0.1.7"
pkgrel=1
pkgdesc="prompt quick access gui app, that use convers magic_convert"
arch=('x86_64')
license=('MIT')
depends=('rust' 'cargo')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/veaquer/convers_prompt/archive/refs/tags/$pkgver.tar.gz")
sha256sums=('cd3deb4d1cc09717397748b2db48760c0d46b68f37dc018ad0396e1e7c9fe2b5')

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
