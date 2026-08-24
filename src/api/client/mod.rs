// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module contains the struct and methods used to retrieve the public IP address of the client and update the DNS records.

pub(crate) mod cache;
pub(crate) mod keystore;
pub(crate) mod providers;

use cache::{AddrData, AddrType, Cache, Host, Token};
use keystore::Credentials;
use providers::ProviderApi;

use mabe::{Context, Result};
use reqwest::blocking::Client as ClientHandle;
use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::thread;
use std::time::Duration;

/// The core struct of the project.
pub(crate) struct Client {
    handle: ClientHandle,
    pub(crate) cache: Cache,
}

impl Client {
    /// Creates a [`Client`] instance.
    pub(crate) fn init() -> Result<Self> {
        Ok(Self { handle: ClientHandle::new(), cache: Cache::load()? })
    }

    /// Retrieves and validates the current IPv4 address.
    pub(crate) fn get_ipv4_address(&self) -> Result<String> {
        let ip = self
            .handle
            .get("https://api.ipify.org")
            .send()
            .context("Failed to retrieve the IPv4 address.")?
            .text()
            .context("Failed to convert the IPv4 address to a UTF-8 string.")?;

        Ok(ip.trim().parse::<Ipv4Addr>().context(format!("The retrieved IPv4 address is invalid: '{}'.", ip))?.to_string())
    }

    /// Retrieves and validates the current IPv6 address.
    pub(crate) fn get_ipv6_address(&self) -> Result<String> {
        let ip = self
            .handle
            .get("https://api6.ipify.org")
            .send()
            .context("Failed to retrieve the IPv6 address.")?
            .text()
            .context("Failed to convert the IPv6 address to a UTF-8 string.")?;

        Ok(ip.trim().parse::<Ipv6Addr>().context(format!("The retrieved IPv6 address is invalid: '{}'.", ip))?.to_string())
    }

    /// Updates the IPv4 (A) and IPv6 (AAAA) address records of the hosts of the provided tokens. When an interval is provided,
    /// the process repeats until the user stops it, and iteration errors (e.g., transient network failures) are reported
    /// without aborting the process; otherwise, the process stops after one iteration.
    pub(crate) fn update_address_records(
        &mut self,
        token_names: impl IntoIterator<Item = impl Into<String>>,
        no_ipv4: bool,
        no_ipv6: bool,
        ipv4: Option<String>,
        ipv6: Option<String>,
        interval: Option<u64>,
    ) -> Result<()> {
        // Extracts the tokens that will be used.
        let target_tokens = self.cache.get_tokens(token_names)?;

        // Retrieves the API keys of each token from the OS keychain.
        let credentials = target_tokens
            .iter()
            .map(|token| Ok((token.to_string(), keystore::load(token.as_str())?)))
            .collect::<Result<HashMap<String, Credentials>>>()?;

        // Validates the fixed IP addresses (when provided, they are used for every iteration).
        let fixed_ipv4 = match ipv4 {
            Some(ip) => {
                Some(ip.parse::<Ipv4Addr>().context(format!("The provided IPv4 address is invalid: '{}'.", ip))?.to_string())
            }
            None => None,
        };
        let fixed_ipv6 = match ipv6 {
            Some(ip) => {
                Some(ip.parse::<Ipv6Addr>().context(format!("The provided IPv6 address is invalid: '{}'.", ip))?.to_string())
            }
            None => None,
        };

        let mut first_iteration = true;
        loop {
            let outcome =
                self.run_iteration(&target_tokens, &credentials, no_ipv4, no_ipv6, &fixed_ipv4, &fixed_ipv6, first_iteration);

            match interval {
                None => return outcome, // Stops after the first iteration if no interval was provided.
                Some(secs) => {
                    if let Err(e) = outcome {
                        eprintln!("The binding iteration failed and will be retried after the interval: {}", e);
                    }
                    thread::sleep(Duration::from_secs(secs));
                }
            }

            first_iteration = false;
        }
    }

    /// Runs a single binding iteration: retrieves the current IP addresses, synchronizes the address records of every host of
    /// the target tokens, and saves the cache if changes were made to it.
    #[allow(clippy::too_many_arguments)]
    fn run_iteration(
        &mut self,
        target_tokens: &[Token],
        credentials: &HashMap<String, Credentials>,
        no_ipv4: bool,
        no_ipv6: bool,
        fixed_ipv4: &Option<String>,
        fixed_ipv6: &Option<String>,
        first_iteration: bool,
    ) -> Result<()> {
        let ipv4 = match (no_ipv4, fixed_ipv4) {
            (true, _) => None,
            (false, Some(ip)) => Some(ip.clone()),
            (false, None) => Some(self.get_ipv4_address()?),
        };
        let ipv6 = match (no_ipv6, fixed_ipv6) {
            (true, _) => None,
            (false, Some(ip)) => Some(ip.clone()),
            (false, None) => Some(self.get_ipv6_address()?),
        };

        let mut save = false;

        for token in target_tokens {
            let api = match providers::api_for(token.provider.as_str()) {
                Some(api) => api,
                None => {
                    if first_iteration {
                        println!("The following DNS provider is not supported and will be skipped: '{}'.", token.provider);
                    }
                    continue;
                }
            };
            let creds = &credentials[token.as_str()];

            if let Some(dns_maps) = self.cache.get_mut_maps(token.as_str()) {
                for (host, addr_data) in dns_maps.iter_mut() {
                    if let Some(ip) = &ipv4 {
                        save |= sync_record(&self.handle, &api, creds, host, AddrType::A, ip, addr_data)?;
                    }
                    if let Some(ip) = &ipv6 {
                        save |= sync_record(&self.handle, &api, creds, host, AddrType::AAAA, ip, addr_data)?;
                    }
                }
            }
        }

        // Saves the cache to the disk if changes were made to it.
        if save {
            self.cache.save()?;
        }

        Ok(())
    }
}

/// Synchronizes the address record of a host with the current IP address, and returns whether the cache was modified. The
/// record is only touched when the cached IP address is missing or outdated: the current record is then fetched from the
/// provider, and updated (or created if it doesn't exist) unless it already holds the current IP address.
fn sync_record(
    handle: &ClientHandle,
    api: &ProviderApi,
    credentials: &Credentials,
    host: &Host,
    addr_type: AddrType,
    current_ip: &str,
    addr_data: &mut AddrData,
) -> Result<bool> {
    let cached = match addr_type {
        AddrType::A => addr_data.ipv4(),
        AddrType::AAAA => addr_data.ipv6(),
    };
    if cached.as_deref() == Some(current_ip) {
        return Ok(false);
    }

    match (api.get)(handle, credentials, host, addr_type)? {
        Some(remote) if remote == current_ip => {} // The record is already up to date; only the cache needs to catch up.
        Some(_) => (api.update)(handle, credentials, host, addr_type, current_ip)?,
        None => (api.create)(handle, credentials, host, addr_type, current_ip)?,
    }

    addr_data.set_ip(addr_type, current_ip)?;
    Ok(true)
}
