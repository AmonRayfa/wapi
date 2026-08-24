<p align="center">
  <h1 align="center">Wapi</h1>
  <p align="center">
    Contributions, corrections, and requests can be made through GitHub, and the documentation is available <a href="https://wapi.readthedocs.io">here</a>.
  </p>
  <p align="center">Thank you for your interest in the project, enjoy your reading! 🚀</p>
</p>

<div align="center">
  <a href="https://phased-versioning.koseka.net"><img src="https://img.shields.io/badge/Versioning-Phased-304CD3?style=flat&color=12398D" alt="Phased Versioning" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-Apache%202.0-723179?style=flat" alt="License" /></a>
  <br>
  <a href="https://github.com/AmonRayfa/wapi/releases"><img src="https://img.shields.io/github/v/tag/AmonRayfa/wapi?label=version&logo=github&color=579D52" alt="version" /></a>
  <a href="https://github.com/AmonRayfa/wapi"><img src="https://img.shields.io/github/created-at/AmonRayfa/wapi?logo=github&label=created&color=C9443C" alt="created" /></a>
  <a href="https://github.com/AmonRayfa/wapi/commits/dev"><img src="https://img.shields.io/github/last-commit/AmonRayfa/wapi?display_timestamp=committer&logo=github&color=438240" alt="last commit" /></a>
  <a href="https://github.com/AmonRayfa/wapi/milestones"><img src="https://img.shields.io/github/milestones/all/AmonRayfa/wapi?logo=github&color=5288DF" alt="milestones" /></a>
  <a href="https://github.com/AmonRayfa/wapi/stargazers"><img src="https://img.shields.io/github/stars/AmonRayfa/wapi?style=flat&logo=github&color=DCB456" alt="stars" /></a>
  <br>
  <a href="Cargo.toml"><img src="https://img.shields.io/badge/Dependencies-9-black?style=flat&logo=rust&logoColor=black" alt="Dependencies" /></a>
  <a href="Cargo.toml"><img src="https://img.shields.io/badge/Size-x.ykB-black?style=flat&logo=rust&logoColor=black" alt="Size" /></a>
</div>

---

**Wapi** is a cross-platform command-line DDNS (Dynamic Domain Name System) client that keeps your DNS mappings up to date by automatically adjusting them whenever your public IP address changes. This is especially useful for users running services on home or private networks with dynamic IP addresses, ensuring their hostnames always resolve to the correct IP address.

The client supports a wide range of DNS service providers (see [DNS Providers List](DNS-PROVIDERS.md)), making it a versatile solution for managing your DNS records. It provides a user-friendly command-line, perfect for workflows involving external scripts or automation tools, as well as a flexible Rust library for developers who want to integrate the client into their own applications.

<h2><img height="20" alt="branches" src="./img/branches.svg">&nbsp;&nbsp;Branches</h2>

| Branch | Description                |
| :----- | :------------------------- |
| `dev`  | Active development branch. |

<h2><img height="20" alt="installation" src="./img/installation.svg">&nbsp;&nbsp;Installation</h2>

The project is still in active development so, there is no fully stable version yet. To use the **nightly version** (tracking the latest commits on the `dev` branch), add the repository link to your `Cargo.toml` file:

```sh
cargo install --git https://github.com/AmonRayfa/wapi --branch dev
```

If you want to install `cargo`, you can do so by following the instructions on the [Rust website](https://www.rust-lang.org/tools/install/).

You can now...

<h2><img height="20" alt="usage" src="./img/usage.svg">&nbsp;&nbsp;Usage</h2>

-> This section will be updated soon.

For further details on how to use the project, please refer to the [documentation](https://wapi.readthedocs.io).

<h2><img height="20" alt="security" src="./img/security.svg">&nbsp;&nbsp;Security</h2>

Vulnerabilities and sensitive information should not be reported via public GitHub issues. Please refer to the [Security Policy](SECURITY.md) for details on supported versions and instructions on how to responsibly disclose security concerns.

<h2><img height="20" alt="contributing" src="./img/contributing.svg">&nbsp;&nbsp;Contributing</h2>

This project is open to contributions and suggestions, and any help or feedback is highly appreciated. There is no code of conduct, but please be respectful and considerate when engaging with the community.

This project uses [Phased Versioning](https://phased-versioning.koseka.net), which defines the versioning, branching, and release rules, and commit messages follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification. So, make sure to read both first before contributing to the project in any way. Additionally, please refer to the [Contribution Guide](CONTRIBUTING.md) for setup instructions and guidance on how to contribute the project.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, shall be licensed as below, without any additional terms or conditions.

<h2><img height="20" alt="license" src="./img/license.svg">&nbsp;&nbsp;License</h2>

Copyright 2026 Amon Rayfa.

This project is licensed under the [Apache License (Version 2.0)](LICENSE).
