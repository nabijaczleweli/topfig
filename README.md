# topfig [![Licence](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)
Assorted config/tooling/lube from the nabtop


### [`src/adjust-screen-brightness.rs`](src/adjust-screen-brightness.rs)

Change `/sys/class/backlight/acpi_video0/brightness` by the amount specified as the first argument, clamped to `0..=max_brightness`.

Needs to be installed suid, see toplevel doc-comment.


### [`src/getxkblayout.c`](src/getxkblayout.c)

Get the current X keyboard layout in `localectl set-x11-keymap LAYOUT`/`setxkbmap -layout` format ("pl", "ru", &c.).

Based on https://unix.stackexchange.com/a/422493/162189, see [`src/deps`](src/deps) for build dependencies.


### [`src/i3status`](src/i3status)

The following patches to [`i3status`](https://github.com/i3/i3status):

1. [`0001-Don-t-pad-CPU-usage-to-2-characters.patch`](src/i3status/0001-Don-t-pad-CPU-usage-to-2-characters.patch)
   the `cpu_usage` metric is padded with `%02d` by default,
   this replaces that behaviour with `%d`.
2. [`0002-Count-all-CPUs-for-general-cpu_usage.patch`](src/i3status/0002-Count-all-CPUs-for-general-cpu_usage.patch)
   `cpu_usage` of all CPUs is scaled between 0-100 by default,
   this replaces that behaviour with summing each CPU's load together instead for a scaling of 0-ncpus\*100.
