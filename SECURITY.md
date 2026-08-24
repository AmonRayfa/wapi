# Security Policy

## Supported Versions

Security updates are provided for the following versions of the project:

| Version |     Supported      |
| :------ | :----------------: |
| `dev`   | :white_check_mark: |

## Credential Storage

The API keys of your DNS service provider accounts are stored in the operating system's keychain (i.e., the macOS Keychain, the Windows Credential Manager, or the Linux secret service), and never touch the disk in plain text. The rest of the client's state (tokens, hostnames, and cached IP addresses) lives in `~/.wapi/cache`, which is restricted to your user account on Unix systems.

## Reporting a Vulnerability

The disclosure of security vulnerabilities helps ensure the safety of the community. If a vulnerability is discovered, please do **not** open a public issue on GitHub, as this allows the flaw to be exploited before a fix is available.

Instead, please report suspected security issues through one of the following private channels:

1.  **GitHub Security Advisories:** Use the "Report a vulnerability" button located in the [**Security**](https://github.com/AmonRayfa/wapi/security) tab of the repository.
2.  **Email:** Send a detailed report to [**amon@koseka.net**](mailto:amon@koseka.net).

Reports are reviewed promptly, and a fix will be released as soon as possible. Thank you for practicing responsible disclosure.

## License

Copyright 2026 Amon Rayfa.

This project is licensed under the [Apache License (Version 2.0)](LICENSE).
