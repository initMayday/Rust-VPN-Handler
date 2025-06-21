pkgname=vpn_handler
pkgver=1.0.1
pkgrel=2
makedepends=('rust' 'cargo')
depends=('wireguard-tools' 'openresolv')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
license=('GPL3')

build() {
	export RUST_TOOLCHAIN=nightly
	export CARGO_TARGET_DIR=target
	cargo build --release --all-features
}

package() {
	install -Dm0755 -t "$pkgdir/usr/bin" "target/release/$pkgname"
}
