// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

mod cache;
mod client;
pub(crate) use cache::{Cache, Domain, DomainJoinExt, Token};
pub(crate) use client::Client;

use clap::{Parser, Subcommand};

use client::porkbun;

/// Represents a DNS provider with its identifier and homepage.
pub(crate) struct DnsProvider {
    mod_path: &'static str,
    pub(crate) url: &'static str,
}

impl DnsProvider {
    pub(crate) fn id(&self) -> &str {
        self.mod_path.rsplit("::").next().unwrap_or(self.mod_path)
    }
}

/// List of the supported DNS service providers.
pub(crate) const SUPPORTED_DNS_PROVIDERS: &[DnsProvider] = &[
    //DnsProvider { mod_path: alibabacloud::PATH, url: "https://www.alibabacloud.com" },
    //DnsProvider { mod_path: bluehost::PATH,     url: "https://www.bluehost.com" },
    //DnsProvider { mod_path: cloudflare::PATH,   url: "https://www.cloudflare.com" },
    //DnsProvider { mod_path: dnspod::PATH,       url: "https://www.dnspod.com" },
    //DnsProvider { mod_path: dreamhost::PATH,    url: "https://www.dreamhost.com" },
    //DnsProvider { mod_path: dynadot::PATH,      url: "https://www.dynadot.com" },
    //DnsProvider { mod_path: enom::PATH,         url: "https://www.enom.com" },
    //DnsProvider { mod_path: epik::PATH,         url: "https://www.epik.com" },
    //DnsProvider { mod_path: gandi::PATH,        url: "https://www.gandi.net" },
    //DnsProvider { mod_path: godaddy::PATH,      url: "https://www.godaddy.com" },
    //DnsProvider { mod_path: hover::PATH,        url: "https://www.hover.com" },
    //DnsProvider { mod_path: ionos::PATH,        url: "https://www.ionos.com" },
    //DnsProvider { mod_path: namecheap::PATH,    url: "https://www.namecheap.com" },
    //DnsProvider { mod_path: namesilo::PATH,     url: "https://www.namesilo.com" },
    //DnsProvider { mod_path: opensrs::PATH,      url: "https://opensrs.com" },
    //DnsProvider { mod_path: ovh::PATH,          url: "https://www.ovhcloud.com" },
    DnsProvider { mod_path: porkbun::PATH, url: "https://porkbun.com" },
    //DnsProvider { mod_path: resellerclub::PATH, url: "https://www.resellerclub.com" },
];

#[derive(Parser)]
#[command(name = "wapi")]
#[command(about = "A cross-platform DDNS client that automatically updates your DNS records when your IP address changes.", long_about = None)]
pub(crate) struct Cli {
    /// Returns the current version of the program.
    #[arg(short, long, exclusive = true)]
    pub(crate) version: bool,

    /// Returns the list of all the supported DNS service providers and their IDs.
    #[arg(short, long, exclusive = true)]
    pub(crate) providers: bool,

    /// The subcommand to execute.
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum TokenCommands {
    /// Adds domain names to tokens.
    Add {
        /// The domain names to add.
        domains: Vec<String>,

        /// The specific tokens to which the domain names are added; it defaults to all tokens if not specified.
        #[arg(short, long)]
        tokens: Option<Vec<String>>,
    },

    /// Removes domain names from tokens.
    Remove {
        /// The domain names to remove.
        domains: Vec<String>,

        /// The specific tokens from which the domain names are removed; it defaults to all tokens if not specified.
        #[arg(short, long)]
        tokens: Option<Vec<String>>,
    },
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Returns your current IPv4 address.
    Ipv4,

    /// Returns your current IPv6 address.
    Ipv6,

    /// Creates a new token in the cache.
    Create {
        /// The name to identify this token locally.
        name: String,

        /// The DNS service provider ID (run **wapi --providers** to get a list of all the supported providers and their IDs).
        provider: String,

        /// The primary API key.
        api_key: String,

        /// Optional secret API key.
        secret_api_key: Option<String>,
    },

    /// Deletes a token from the cache.
    Delete {
        /// The local name of the token to remove.
        name: String,
    },

    /// Manages the tokens in the cache.
    Token {
        /// The subcommand to execute under **token**.
        #[command(subcommand)]
        command: Option<TokenCommands>,
    },

    /// Prints a table of all the tokens stored in the cache.
    Show,

    /// Binds your current (or provided) IPv4/IPv6 addresses to the domain names associated with the specified tokens.
    Bind {
        /// The names of the tokens whose associated domains will be updated; it defaults to all tokens in the cache if non are provided.
        tokens: Option<Vec<String>>,

        // Excludes the IPv4 address from the binding process.
        #[arg(long)]
        no_ipv4: bool,

        /// Excludes IPv6 address from the binding process.
        #[arg(long)]
        no_ipv6: bool,

        /// The IPv4 address that will be bound to the domain names; it defaults to the current IPv4 address if not specified.
        #[arg(long, conflicts_with = "no_ipv4")]
        ipv4: Option<String>,

        /// The IPv6 address that will be bound to the domain names; it defaults to the current IPv6 address if not specified.
        #[arg(long, conflicts_with = "no_ipv6")]
        ipv6: Option<String>,

        /// The time it takes (in seconds) to repeat the whole binding operation; if not provided, wapi will run once and exit.
        #[arg(short, long)]
        interval: Option<u64>,
    },
}
