pkgname=lia
pkgver=0.2.2
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')

source=("$pkgname-git::git+https://github.com/jaspwr/LiA#branch=main")
sha256sums=('SKIP')

prepare() {
    cd "$pkgname-git"
    cargo fetch --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$pkgname-git"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --release
}

package() {
    cd "$pkgname-git"
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
}
