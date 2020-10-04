#
# PKGBUILD file for package czkawka
#
# Copyright (C) 2020  sp1rit
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# Maintainer: sp1rit <sp1ritCS@protonmail.com>

_basename=czkawka
_releasever=1.0.0
pkgname=${_basename}-git
pkgver=1.0.0.8d263fd
pkgrel=3
pkgdesc="Multi functional app to find duplicates, empty folders etc."
arch=(any)
url="https://github.com/qarmin/czkawka"
license=('MIT')
depends=("gtk3>=3.22")
makedepends=("git" "rust" "cargo")
provides=("czkawka")
source=("${_basename}::git+https://github.com/qarmin/czkawka.git" "com.github.qarmin.czkawka.desktop")
md5sums=("SKIP" "bbc480c6106a7bd7dc73c3cc49a76961")

pkgver() {
	cd "$srcdir/${_basename}"
	echo "${_releasever}.$(git rev-parse --short HEAD)"
}

build() {
	cd "${_basename}/"
	cargo build --release --bin czkawka_gui
	cargo build --release --bin czkawka_gui_orbtk
	cargo build --release --bin czkawka_cli
}

check() {
	cd "${_basename}/czkawka_core"
	cargo test
}

package() {
	cd "${_basename}"
	#ls -lah target/release/
	install -Dm755 target/release/czkawka_gui $pkgdir/usr/bin/${_basename}_gui
	install -Dm755 target/release/czkawka_gui_orbtk $pkgdir/usr/bin/${_basename}_gui_orbtk
	install -Dm755 target/release/czkawka_cli $pkgdir/usr/bin/${_basename}_cli

	ln -s $pkgdir/usr/bin/${_basename}_gui $pkgdir/usr/bin/${_basename}

	install -Dm644 LICENSE $pkgdir/usr/share/licenses/${_basename}/LICENSE
	install -Dm644 $srcdir/com.github.qarmin.czkawka.desktop -t $pkgdir/usr/share/applications/
}
