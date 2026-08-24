# Contribution Guide

This file is primarily intended for developers who wish to fork the project and potentially contribute to it. This project uses [Phased Versioning](https://phased-versioning.koseka.net), which defines the versioning, branching, and release rules, and commit messages follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification. So, make sure to read both first before contributing to the project in any way.

## Project Structure

Here are the main directories and files in the project:

```plaintext
.
├── src/
│   ├── api/
│   │   ├── client/
│   │   │   ├── cache/
│   │   │   ├── providers/
│   │   │   ├── keystore.rs
│   │   │   └── mod.rs
│   │   ├── mod.rs
│   │   └── parser.rs
│   └── main.rs
├── Cargo.toml
└── package.json
```

The `src/api/mod.rs` file defines the topology of the CLI, and the `src/api/parser.rs` file parses the interleaved hostname and token arguments of the track/untrack commands. The `src/api/client/` directory defines the structs and methods to manipulate the CLI's client: the `providers/` directory contains a submodule for each DNS service provider, housing the provider-specific request logic for retrieving and updating address records; the `cache/` directory handles the client's caching logic; and the `keystore.rs` file stores the API keys in the operating system's keychain. The rest of the structure is self-explanatory.

Additionally, the `package.json` file configures the [Node](https://nodejs.org) environment required to run the [Trunk CLI](https://docs.trunk.io/code-quality/overview) metalinter.

See the [API reference](https://wapi.readthedocs.io/en/stable/wapi/all.html) for a more detailed overview of the project structure.

## Setting Up the Development Environment

First, ensure that you have the latest version of **Rust** installed on your machine. You can install **Rust** by following the instructions on the official [Rust website](https://www.rust-lang.org/tools/install).

Second, this project uses the [**Trunk Code Quality CLI**](https://docs.trunk.io/code-quality/overview) as an npm package for formatting and linting the code, and **npm** as a package manager. So, make sure you have **node.js** and **npm** installed on your machine. You can install both of them from the official [node.js](https://nodejs.org) website.

Next, clone the `wapi` repository to your local machine and install the development dependencies:

```sh
git clone https://github.com/AmonRayfa/wapi.git             # Clones the repository.
cd wapi                                                     # Moves into the project directory.
npm install                                                 # Installs the development dependencies.
```

Since the **Trunk CLI** is used for formatting the code, it's best if you disable the _format on save_ option in your editor to avoid potential conflicts with the project's formatting configurations.

If you are using [**Zed**](https://zed.dev), you can locally disable the _format on save_ option of your editor for this project by adding the following line to the `.zed/settings.json` file at the root of the project directory:

```json
{
	"format_on_save": "off"
}
```

If you are using [**VSCode**](https://code.visualstudio.com), you can locally disable the _format on save_ option of your editor for this project by adding the following line to the `.vscode/settings.json` file at the root of the project directory:

```json
{
	"editor.formatOnSave": false
}
```

As for the linting, the project comes with its own linters and configurations, so if you have your own linters installed with custom configurations, you should make sure they don't conflict with the project's linters. You can check the list of linters (and formatters) along with their configurations in the `.trunk/trunk.yaml` file and the `.trunk/configs/` directory.

If you have followed all the steps correctly, you should now have a working development environment for the project. If you encounter any issues, feel free to open an issue on the project's [GitHub repository](https://github.com/AmonRayfa/name/issues).

## Linting and Formatting the Code

The linters and formatters work through git hooks, so they will run automatically when you commit changes. However, it's best to also run them manually before committing changes to avoid failing the commit hook.

To make sure the **Trunk CLI** is managing the git hooks, you can run the following command:

```sh
npm run trunk git-hooks sync
```

You can manually run the linters and formatters using the following commands:

```sh
npm run check                                       		# Runs linters and formatters on all the changed files.
npm run check --all                                 		# Runs linters and formatters on all the files in the repository.
```

You can manually format the code using the following commands:

```sh
npm run fmt                                         		# Formats all the changed files.
npm run fmt --all                                   		# Formats all the files in the repository.
```

## Testing and Building the Project

The tests live next to the code they cover, in `#[cfg(test)]` modules. Here are some useful test commands:

```sh
cargo test                                                  # Runs all the tests in the project.
cargo test --nocapture                                      # Runs all the tests in the project and displays their output.
cargo test --lib parent_mod::child_mod::tests               # Runs the tests in the `tests` module in `src/parent_mod/child_mod/mod.rs` (or in `src/parent_mod/child_mod.rs`).
cargo test --test some_test_module                          # Runs the tests in `tests/some_test_module.rs`.
cargo test --features colorize                              # Runs all the tests in the project for the `colorize` feature.
cargo test --all-features                                   # Runs all the tests in the project for all the features.
```

You can build the project using the following commands:

```sh
cargo build                                               	# Builds the project in debug mode.
cargo build --release                                     	# Builds the project in release mode.
cargo build --features some_feature                       	# Builds the project with the `some_feature` feature.
cargo build --all-features                                	# Builds the project with all the features.
```

## Building the Documentation

You can build the documentation using the following commands:

```sh
cargo doc --no-deps                                         # Builds the documentation at `target/doc/`.
cargo doc --no-deps --open                                  # Builds the documentation and opens it in the browser.
```

## License

Copyright 2026 Amon Rayfa.

This project is licensed under the [Apache License (Version 2.0)](LICENSE).
