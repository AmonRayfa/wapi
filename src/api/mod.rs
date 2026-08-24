// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

mod client;
mod parser;

pub(crate) use client::Client;
pub(crate) use client::cache::{Cache, HostJoinExt};
pub(crate) use client::providers::PROVIDERS;
pub(crate) use parser::collect_mappings;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wapi")]
#[command(about = "A cross-platform DDNS client that automatically updates your DNS records when your IP address changes.", long_about = None)]
pub(crate) struct Cli {
    /// The subcommand to execute.
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,

    /// Returns the current version of the program.
    #[arg(short, long, exclusive = true)]
    pub(crate) version: bool,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Returns your current IPv4 address.
    Ipv4,

    /// Returns your current IPv6 address.
    Ipv6,

    /// Returns all the supported DNS service providers and their IDs.
    Providers,

    /// Creates and deletes tokens. When no arguments are provided, it returns all the tokens stored in the cache with their
    /// associated DNS service provider and hostnames.
    Token {
        /// The name of the token to create or delete (prefixed with `@`, e.g., *@porkbun*).
        name: Option<String>,

        /// The DNS service provider ID of the token that will be created (run **wapi providers** to get all the
        /// supported providers and their IDs).
        #[arg(short, long, requires = "names", conflicts_with = "delete")]
        provider: Option<String>,

        /// Deletes provided token from the cache.
        #[arg(short, long, requires = "names", conflicts_with = "provider")]
        delete: bool,
    },

    /// Tracks hostnames by associating them with the provided tokens, or all tokens if none were given.
    Track {
        /// An interleaved list of hostnames and `@` prefixed tokens.
        #[arg(required = true)]
        targets: Vec<String>,
    },

    /// Untracks hostnames by removing them from the provided tokens, or all tokens if none were given.
    Untrack {
        /// An interleaved list of hostnames and `@` prefixed tokens.
        #[arg(required = true)]
        targets: Vec<String>,
    },

    /// Binds your current (or provided) IPv4/IPv6 addresses to the hostnames associated with the specified tokens.
    Bind {
        /// The tokens to update, prefixed with `@` (e.g., *@cloudflare* *@porkbun*).
        /// Defaults to all tokens in the cache if none are provided.
        tokens: Option<Vec<String>>,

        // Excludes the IPv4 address from the binding process.
        #[arg(long)]
        no_ipv4: bool,

        /// Excludes IPv6 address from the binding process.
        #[arg(long)]
        no_ipv6: bool,

        /// The IPv4 address to bind. Defaults to the current machine's IPv4 address.
        #[arg(long, conflicts_with = "no_ipv4")]
        ipv4: Option<String>,

        /// The IPv6 address to bind. Defaults to the current machine's IPv6 address.
        #[arg(long, conflicts_with = "no_ipv6")]
        ipv6: Option<String>,

        /// The interval (in seconds) between each iteration of the binding process (the binding runs until the user stops it).
        /// If omitted, the binding process stops after one iteration.
        #[arg(short, long)]
        interval: Option<u64>,
    },
}
