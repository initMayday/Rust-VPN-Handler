# Maintainer: initMayday@protonmail.com

_pkgname=vpn_handler
pkgname="vpn-handler-git"
pkgver=r2.ff85586
pkgrel=1
pkgdesc="Tool to quickly bring vpns up and down via wireguard"
arch=('any')
url="https://github.com/initMayday/Rust-VPN-Handler"
makedepends=('git' 'rust' 'cargo')
depends=('wireguard-tools' 'openresolv')
license=('CC-BY-NC-SA-4.0')
source=("$_pkgname::git+$url")
sha256sums=('SKIP')
provides=("$_pkgname")
conflicts=("$_pkgname")

build() {
    cd "$_pkgname"
	export CARGO_TARGET_DIR=target
	cargo build --release --all-features
}

pkgver() {
	cd "$_pkgname"
	printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

package() {
    cd "$_pkgname"
	install -Dm0755 -t "$pkgdir/usr/bin" "target/release/$_pkgname"
}
