<p align="center">
  <h1 align="center">Wapi</h1>
  <p align="center">Contributions, corrections, and requests can be made through GitHub, and the documentation is available on the platforms linked bellow.</p>
  <p align="center">Thank you for your interest in the project, enjoy your reading! 🚀</p>
</p>

<p align="center">
  <a href="https://github.com/AmonRayfa/wapi"><img alt="GitHub: created in" src="https://img.shields.io/github/created-at/AmonRayfa/wapi?logo=github&label=created%20in&color=red"/></a>
  <a href="https://github.com/AmonRayfa/wapi"><img alt="GitHub: last commit" src="https://img.shields.io/github/last-commit/AmonRayfa/wapi?display_timestamp=committer&logo=github&color=yellow"/></a>
  <a href="https://github.com/AmonRayfa/wapi"><img alt="GitHub: milestones" src="https://img.shields.io/github/milestones/all/AmonRayfa/wapi?logo=github&color=blue"/></a>
  <a href="https://github.com/AmonRayfa/wapi"><img alt="GitHub: CI/CD" src="https://img.shields.io/github/actions/workflow/status/AmonRayfa/wapi/ci-cd.yaml?branch=main&logo=github&label=CI%2FCD"/></a>
  <br/>
  <a href="https://crates.io/crates/wapi"><img alt="Crates.io: size" src="https://img.shields.io/crates/size/wapi?logo=rust&logoColor=black&color=black"/></a>
  <a href="https://crates.io/crates/wpai"><img alt="Crates.io: dependents" src="https://img.shields.io/crates/dependents/wapi?logo=rust&logoColor=black&color=black"/></a>
</p>

## Introduction

**Wapi** is a cross-platform command-line DDNS (Dynamic Domain Name System) client that keeps your DNS records up to date by
automatically adjusting them whenever your public IP address changes. This is especially useful for users running services on
home or private networks with dynamic IP addresses, ensuring their domain names always resolve to the correct IP address.

The client supports a wide range of domain registrars and DNS service providers (see [DNS-PROVIDERS](DNS-PROVIDERS.md)), making
it a versatile solution for managing your DNS records. It provides a user-friendly command-line, perfect for workflows involving
external scripts or automation tools, as well as a flexible Rust library for developers who want to integrate the client into
their own applications.

---

**Other DDNSs:**

- [Duck DNS](https://www.home-assistant.io/integrations/duckdns)
- [Dynu](https://www.dynu.com)
- [No-IP](https://www.noip.com)

## Getting Started

In order to use **Wapi** in your project, you need to add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
wapi = "0.1"
```

If you want to use the command-line client, you can install it using `cargo`:

```sh
cargo install wapi
```

If you want to install `cargo`, you can do so by following the instructions on the
[Rust website](https://www.rust-lang.org/tools/install).

## Examples

-> This section will be updated soon.

## Contributing

This project is open to contributions and suggestions, and any help or feedback is highly appreciated. There is no code of
conduct, but please be respectful and considerate when engaging with the community.

The project follows the [Koseka Project Guidelines](https://koseka.org/project-guidelines), which provide standardized rules and
recommendations for project development. Make sure to read these guidelines first before contributing to the project in any way.
Additionally, please refer to the [DEVELOPMENT](DEVELOPMENT.md) file for setup instructions and guidance on developing, testing,
and building the project.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, shall be
licensed as bellow, without any additional terms or conditions.

## License

Copyright 2025 Amon Rayfa.

This project is licensed under the [Apache License (Version 2.0)](LICENSE).
