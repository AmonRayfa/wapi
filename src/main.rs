// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! [**Wapi**](https://github.com/AmonRayfa/wapi) is a cross-platform command-line DDNS (Dynamic Domain Name System) client that
//! keeps your DNS records up to date by automatically adjusting them whenever your public IP address changes. This is
//! especially useful for users running services on home or private networks with dynamic IP addresses, ensuring their domain
//! names always resolve to the correct IP address.
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

use api::{Cli, Client, Commands, SUPPORTED_DNS_PROVIDERS, TokenCommands};
use clap::Parser;
use comfy_table::Table;
use mabe::Result;

#[mabe::main]
fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.version {
        println!("dev");
        return Ok(());
    }

    if cli.providers {
        for provider in SUPPORTED_DNS_PROVIDERS {
            println!("{}\t[{}]", provider.id, provider.url);
        }
        return Ok(());
    }

    let mut client = Client::init()?;
    match cli.command {
        Some(Commands::Token { list, command }) => {
            if list {
                let mut table = Table::new();
                table.set_header(vec!["TOKEN NAME", "PROVIDER", "DOMAINS"]);

                for token in &client.cache.tokens {
                    table.add_row(vec![&token.name, &token.provider, &token.domains.join(", ")]);
                }

                println!("{}", table);
                return Ok(());
            }

            match command {
                Some(TokenCommands::Create { name, provider, api_key, secret_api_key }) => {
                    client.cache.create_token(name, provider, api_key, secret_api_key)?;
                    client.cache.save()
                }
                Some(TokenCommands::Delete { name }) => {
                    client.cache.delete_token(name)?;
                    client.cache.save()
                }
                Some(TokenCommands::Add { domains, tokens }) => {
                    client.cache.add_domains(domains, tokens)?;
                    client.cache.save()
                }
                Some(TokenCommands::Remove { domains, tokens }) => {
                    client.cache.remove_domains(domains, tokens)?;
                    client.cache.save()
                }
                None => {
                    println!(
                        "\x1b[1;31merror:\x1b[0m no subcommand or flag was provided\n\n\x1b[1;4mUsage:\x1b[0m \x1b[1mwapi\x1b[0m token [COMMAND]\n\nFor more information, try '\x1b[1m--help\x1b[0m'."
                    );
                    Ok(())
                }
            }
        }
        Some(Commands::Ipv4) => {
            println!("{}", client.get_ipv4_address()?);
            Ok(())
        }
        Some(Commands::Ipv6) => {
            println!("{}", client.get_ipv6_address()?);
            Ok(())
        }
        Some(Commands::Bind { tokens, no_ipv4, no_ipv6, ipv4, ipv6, interval }) => {
            client.update_dns_records(tokens, no_ipv4, no_ipv6, ipv4, ipv6, interval)
        }
        None => {
            println!(
                "\x1b[1;31merror:\x1b[0m no subcommand or flag was provided\n\n\x1b[1;4mUsage:\x1b[0m \x1b[1mwapi\x1b[0m [COMMAND]\n\nFor more information, try '\x1b[1m--help\x1b[0m'."
            );
            Ok(())
        }
    }
}
