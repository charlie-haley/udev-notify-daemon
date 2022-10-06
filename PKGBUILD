# Maintainer: Charlie Haley <charlie.haley@hotmail.com>

pkgname=udev-notify-daemon-bin
pkgver=<placeholder-version>
pkgrel=<release-number>
pkgdesc="A simple daemon that listens for udev USB events and sends out a notification to dbus"
url="https://github.com/charlie-haley/udev-notify-daemon"
arch=("x86_64")
license=("MIT")
source=(
  "udev-notify-daemon_amd64::https://github.com/charlie-haley/udev-notify-daemon/releases/download/v${pkgver}/udev-notify-daemon_amd64"
  "https://raw.githubusercontent.com/charlie-haley/udev-notify-daemon/v${pkgver}/udev-notify-daemon.service"
)
sha256sums=(
    '<sha256-binary>'
    '<sha256-unit>'
)
_srcname=udev-notify-daemon

package() {
	install -Dm755 udev-notify-daemon_amd64 ${pkgdir}/usr/bin/udev-notify-daemon_amd64
    install -Dm755 udev-notify-daemon.service ${pkgdir}/usr/lib/systemd/user/udev-notify-daemon.service
}
