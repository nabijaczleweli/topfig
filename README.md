# topfig [![Licence](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)
Assorted config/tooling/lube from the nabtop


### [`src/adjust-screen-brightness.rs`](src/adjust-screen-brightness.rs)

Change `/sys/class/backlight/acpi_video0/brightness` by the amount specified as the first argument, clamped to `0..=max_brightness`.

Needs to be installed suid, see toplevel doc-comment.


### [`src/getxkblayout.c`](src/getxkblayout.c)

Get the current X keyboard layout in `localectl set-x11-keymap LAYOUT`/`setxkbmap -layout` format ("pl", "ru")

Based on https://unix.stackexchange.com/a/422493/162189, see [`src/deps`](src/deps) for build dependencies.
