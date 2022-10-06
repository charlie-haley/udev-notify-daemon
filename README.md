# udev-notify-daemon

`udev-notify-daemon` is a simple daemon that listens for `udev` USB events and sends out a notification to `dbus` using the [freedesktop notification specification.](https://specifications.freedesktop.org/notification-spec/notification-spec-latest.html)

## Installation

There's a [binary release on GitHub](https://github.com/charlie-haley/udev-notify-daemon/releases) or you can install through the AUR:
```bash
yay -S udev-notify-daemon-bin
systemctl --user enable --now udev-notify-daemon.service
```
