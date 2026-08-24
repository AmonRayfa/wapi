// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! [**Wapi**](https://github.com/AmonRayfa/wapi) is a cross-platform command-line DDNS (Dynamic Domain Name System) client that
//! keeps your DNS records up to date by automatically adjusting them whenever your public IP address changes. This is
//! especially useful for users running services on home or private networks with dynamic IP addresses, ensuring their hostnames
//! always resolve to the correct IP address.
//!
//! The client supports a wide range of DNS service providers, making it a versatile solution for managing your DNS records. It
//! provides a user-friendly command-line, perfect for workflows involving external scripts or automation tools, as well as a
//! flexible Rust library for developers who want to integrate the client into their own applications.
//!
//! # Cargo Features
//!
//! This crate has no public
//! [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/features.html#the-features-section).
//!
//! # Installation
//!
//! -> TODO
//!
//! # Usage
//!
//! -> TODO

mod api;

use api::{Cache, Cli, Client, Commands, HostJoinExt, PROVIDERS, collect_mappings};
use clap::Parser;
use comfy_table::Table;
use mabe::{Context, Result};
use std::io::{self, Write};

#[mabe::main]
fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.version {
        println!("dev");
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
                    // TODO: Prompt the user to type in the provider ID and API keys.
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

                    client.cache.create_token(token_name, provider_id, api_key, secret_api_key)?;
                } else {
                    client.cache.delete_token(token_name)?;
                }
                client.cache.save()
            }
            None => {
                let cache = Cache::load()?;
                let mut table = Table::new();
                table.set_header(vec!["TOKEN NAME", "PROVIDER ID", "HOSTNAMES"]);

                for (token, dns_maps) in cache.content().iter() {
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
            let token_names = match tokens {
                Some(n) => n,
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
