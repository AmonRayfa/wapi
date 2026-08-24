// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! [**Wapi**](https://github.com/AmonRayfa/wapi) is a cross-platform command-line DDNS (Dynamic Domain Name System) client that
//! keeps your DNS records up to date by automatically adjusting them whenever your public IP address changes. This is
//! especially useful for users running services on home or private networks with dynamic IP addresses, ensuring their hostnames
//! always resolve to the correct IP address.
//!
//! The client is designed to support a wide range of DNS service providers, making it a versatile solution for managing your
//! DNS records. It provides a user-friendly command-line interface, perfect for workflows involving external scripts or
//! automation tools.
//!
//! # Cargo Features
//!
//! This crate has no public
//! [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/features.html#the-features-section).
//!
//! # Installation
//!
//! The client is distributed as a Git repository, by branch, following
//! [Phased Versioning](https://phased-versioning.koseka.net). To install the latest stable version of the current generation,
//! run:
//!
//! ```sh
//! cargo install --git https://github.com/AmonRayfa/wapi --branch v1
//! ```
//!
//! To install the nightly version instead (tracking the latest commits), use the `dev` branch.
//!
//! # Usage
//!
//! ```sh
//! wapi providers                                # Lists the supported DNS service providers and their IDs.
//! wapi token porkbun-main                       # Creates a token (the provider ID and API keys are prompted).
//! wapi track example.com www.example.com @porkbun-main    # Tracks hostnames by associating them with the token.
//! wapi bind                                     # Binds your current IP addresses to all the tracked hostnames.
//! wapi bind --interval 300                      # Keeps the DNS records up to date, checking every 5 minutes.
//! ```
//!
//! Run `wapi --help` for the full list of commands and options.

mod api;

use api::{Cli, Client, Commands, Credentials, HostJoinExt, PROVIDERS, collect_mappings, keystore};
use clap::Parser;
use comfy_table::Table;
use mabe::{Context, Result};
use std::io::{self, Write};

/// The full [Phased Versioning](https://phased-versioning.koseka.net) version of the program; it must be set to the release
/// tag (e.g., *v1-alpha.0*) before a release is cut, and stays *dev* while no release exists.
const VERSION: &str = "v1-alpha.0";

#[mabe::main]
fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.version {
        println!("{}", VERSION);
        return Ok(());
    }

    let mut client = Client::init()?;
    match cli.command {
        Some(Commands::Ipv4) => {
            println!("{}", client.get_ipv4_address()?);
            Ok(())
        }
        Some(Commands::Ipv6) => {
            println!("{}", client.get_ipv6_address()?);
            Ok(())
        }
        Some(Commands::Providers) => {
            let mut table = Table::new();
            table.set_header(vec!["ID", "URL"]);

            for (id, url) in PROVIDERS {
                table.add_row(vec![id, url]);
            }

            println!("{}", table);
            Ok(())
        }
        Some(Commands::Token { name, provider, delete }) => match name {
            Some(token_name) => {
                if !delete {
                    // Prompts the user for the provider ID and API keys.
                    let provider_id = match provider {
                        Some(id) => id,
                        None => {
                            let mut id = String::new();
                            print!("Provider ID: ");

                            io::stdout().flush().context("Failed to flush the terminal.")?;

                            io::stdin().read_line(&mut id).context("Failed to read the provider ID from the terminal.")?;

                            id.trim().to_string()
                        }
                    };

                    let api_key =
                        rpassword::prompt_password("API key: ").context("Failed to read the API key from the terminal.")?;
                    let secret_api_key_input = rpassword::prompt_password("Secret API key (optional, press enter if none): ")
                        .context("Failed to read the secret API key from the terminal.")?;
                    let secret_api_key = if secret_api_key_input.is_empty() { None } else { Some(secret_api_key_input) };

                    // Validates the token before touching the OS keychain, and stores the API keys before saving the cache, so
                    // that a keychain failure never leaves a token without keys.
                    client.cache.create_token(&token_name, provider_id)?;
                    keystore::store(&token_name, &Credentials { api_key, secret_api_key })?;
                } else {
                    client.cache.delete_token(&token_name)?;
                    keystore::delete(&token_name)?;
                }
                client.cache.save()
            }
            None => {
                let mut table = Table::new();
                table.set_header(vec!["TOKEN NAME", "PROVIDER ID", "HOSTNAMES"]);

                for (token, dns_maps) in client.cache.content().iter() {
                    table.add_row(vec![&token.to_string(), &token.provider.to_string(), &dns_maps.join(", ")]);
                }

                println!("{}", table);
                Ok(())
            }
        },
        Some(Commands::Track { targets }) => {
            for (hostnames, token_names) in collect_mappings(targets)? {
                let token_names = match token_names {
                    Some(n) => n,
                    None => client.cache.content().keys().map(|token| token.to_string()).collect(),
                };

                client.cache.add_hosts(hostnames, token_names)?;
            }
            client.cache.save()
        }
        Some(Commands::Untrack { targets }) => {
            for (hostnames, token_names) in collect_mappings(targets)? {
                let token_names = match token_names {
                    Some(n) => n,
                    None => client.cache.content().keys().map(|token| token.to_string()).collect(),
                };

                client.cache.remove_hosts(hostnames, token_names)?;
            }
            client.cache.save()
        }
        Some(Commands::Bind { tokens, no_ipv4, no_ipv6, ipv4, ipv6, interval }) => {
            let token_names: Vec<String> = match tokens {
                // Token names are accepted with or without the `@` prefix used by the track/untrack commands.
                Some(n) => n.into_iter().map(|t| t.trim_start_matches('@').to_string()).collect(),
                None => client.cache.content().keys().map(|token| token.to_string()).collect(),
            };
            client.update_address_records(token_names, no_ipv4, no_ipv6, ipv4, ipv6, interval)
        }
        None => {
            println!(
                "\x1b[1;31merror:\x1b[0m no subcommand or flag was provided\n\n\x1b[1;4mUsage:\x1b[0m \x1b[1mwapi\x1b[0m [COMMAND]\n\nFor more information, try '\x1b[1m--help\x1b[0m'."
            );
            Ok(())
        }
    }
}
