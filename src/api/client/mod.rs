// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module contains the struct and methods used to retrieve the public IP address of the client and update the DNS records.

pub(crate) mod cache;
pub(crate) mod providers;

use cache::{AddrType, Cache};
use providers::porkbun;

use mabe::{Context, Result};
use reqwest::blocking::Client as ClientHandle;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::thread;
use std::time::Duration;

// Define this somewhere in your file or a macro utility module
macro_rules! match_provider {
    (
        $looped:expr,
        $save:expr,
        $client_handle:expr,
        $token:expr,
        $host:expr,
        $addr_data:expr,
        $no_ipv4:expr,
        $no_ipv6:expr,
        $ipv4:expr,
        $ipv6:expr,
        [ $( $provider_mod:ident ),* $(,)? ]
    ) => {
        match $token.provider.as_str() {
            $(
                stringify!($provider_mod) => {
                    if !$no_ipv4 && let Some(cached_ipv4) = $addr_data.ipv4() && $ipv4.as_str() != cached_ipv4 {
                        let addr_type = AddrType::A;
                        $addr_data.update_ip(addr_type, $ipv4)?;
                        $provider_mod::update_address_record(
                            &$client_handle,
                            $token,
                            $host,
                            &addr_type,
                            $addr_data,
                        )?;
                        $save = true;
                    }
                    if !$no_ipv6 && let Some(cached_ipv6) = $addr_data.ipv6() && $ipv6.as_str() != cached_ipv6 {
                        let addr_type = AddrType::AAAA;
                        $addr_data.update_ip(addr_type, $ipv6)?;
                        $provider_mod::update_address_record(
                            &$client_handle,
                            $token,
                            $host,
                            &addr_type,
                            $addr_data,
                        )?;
                        $save = true;
                    }
                }
            )*
            p => {
                if !$looped {
                    println!("The following DNS provider is not supported and will be skipped: '{}'.", p);
                }
            }
        }
    };
}

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

    /// Retrieves the current IPv4 address.
    pub(crate) fn get_ipv4_address(&self) -> Result<String> {
        self.handle
            .get("https://api.ipify.org")
            .send()
            .context("Failed to retrieve the IPv4 address.")?
            .text()
            .context("Failed to convert the IPv4 address to a UTF-8 string.")
    }

    /// Retrieves the current IPv6 address.
    pub(crate) fn get_ipv6_address(&self) -> Result<String> {
        self.handle
            .get("https://api6.ipify.org")
            .send()
            .context("Failed to retrieve the IPv6 address.")?
            .text()
            .context("Failed to convert the IPv6 address to a UTF-8 string.")
    }

    /// Updates the IPv4 (A) and IPv6 (AAAA) address records of the hosts of the provided tokens.
    pub(crate) fn update_address_records(
        &mut self,
        token_names: impl IntoIterator<Item = impl Into<String>>,
        no_ipv4: bool,
        no_ipv6: bool,
        ipv4: Option<String>,
        ipv6: Option<String>,
        interval: Option<u64>,
    ) -> Result<()> {
        let mut looped = false;
        let mut save = false;

        // Extracts the tokens that will be used.
        let target_tokens = self.cache.get_tokens(token_names)?;

        // Validates the IPv4 address.
        let mut ipv4 = if !no_ipv4 {
            match ipv4 {
                Some(ip) => {
                    ip.parse::<Ipv4Addr>().context(format!("The provided IPv4 address is invalid: '{}'.", ip))?.to_string()
                }
                None => self.get_ipv4_address()?,
            }
        } else {
            String::new()
        };

        // Validates the IPv6 address.
        let mut ipv6 = if !no_ipv6 {
            match ipv6 {
                Some(ip) => {
                    ip.parse::<Ipv6Addr>().context(format!("The provided IPv6 address is invalid: '{}'.", ip))?.to_string()
                }
                None => self.get_ipv6_address()?,
            }
        } else {
            String::new()
        };

        loop {
            // Updates the address records.
            for token in &target_tokens {
                if let Some(dns_maps) = self.cache.get_mut_maps(token.as_str()) {
                    for (host, addr_data) in dns_maps.iter_mut() {
                        match_provider!(
                            looped,
                            save,
                            self.handle,
                            token,
                            host,
                            addr_data,
                            no_ipv4,
                            no_ipv6,
                            ipv4.clone(),
                            ipv6.clone(),
                            [porkbun]
                        );
                    }
                }
            }

            match interval {
                Some(v) => thread::sleep(Duration::from_secs(v)),
                None => break, // Breaks the loop after the first iteration if no interval was provided.
            }

            if !no_ipv4 {
                ipv4 = self.get_ipv4_address()?;
            }

            if !no_ipv6 {
                ipv6 = self.get_ipv6_address()?;
            }

            // Saves the cache to the disk if changes were made to it.
            if save {
                self.cache.save()?;
                save = false;
            }

            looped = true;
        }

        Ok(())
    }
}
