# topfig [![Licence](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)
Assorted config/tooling/lube from the nabtop


## `src/adjust-screen-brightness.rs`

Change `/sys/class/backlight/acpi_video0/brightness` by the amount specified as the first argument.

Needs to be installed suid, see toplevel doc-comment.
