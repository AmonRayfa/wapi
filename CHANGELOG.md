# Changelog

This project is licensed under the [Apache License (Version 2.0)](LICENSE). The format of this file (and the project as a whole) follows [Phased Versioning](https://phased-versioning.koseka.net).

## v1-alpha.2 (2026-08-24)

- Upgraded the comfy-table and keyring dependencies to their next major versions.
- Replaced the vendored D-Bus build by a pure-Rust secret service backend, so that Linux builds no longer require any system libraries.

## v1-alpha.1 (2026-08-24)

- Fixed the Linux build by vendoring the D-Bus library, so that the system libdbus headers are no longer required.

## v1-alpha.0 (2026-08-24)

First stable version.
