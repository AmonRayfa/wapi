// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module contains the struct and methods used to retrieve the public IP address of the client and update the DNS records.

pub(crate) mod porkbun;

use crate::api::{Cache, Token};
use mabe::{Context, Result};
use reqwest::blocking::Client as ClientHandle;
use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::thread;
use std::time::Duration;

#[derive(Default)]
pub(super) struct AddressRecordData {
    pub(crate) ip: String,
    pub(crate) ttl: u64,
    //pub(crate) notes: Option<String>,
}

#[derive(Default)]
struct AddressRecords {
    pub(crate) a: AddressRecordData,  // 'A' record.
    pub(crate) qa: AddressRecordData, // 'AAAA' (or 'Quad-A') record.
}

// Define this somewhere in your file or a macro utility module
macro_rules! match_provider {
    (
        $client_handle:expr,
        $token:expr,
        $domain:expr,
        $tracker:expr,
        $looped:expr,
        $no_ipv4:expr,
        $no_ipv6:expr,
        $ipv4:expr,
        $ipv6:expr,
        [ $( $provider_mod:ident ),* $(,)? ]
    ) => {
        match $token.provider.as_str() {
            $(
                stringify!($provider_mod) => {
                    if !$looped {
                        if !$no_ipv4 {
                            $provider_mod::get_address_records(
                                &$client_handle,
                                $token,
                                $domain,
                                "A",
                                &mut $tracker.get_mut(stringify!($provider_mod)).unwrap().get_mut(&$domain.to_string()).unwrap().a,
                            )?
                        }
                        if !$no_ipv6 {
                            $provider_mod::get_address_records(
                                &$client_handle,
                                $token,
                                $domain,
                                "AAAA",
                                &mut $tracker.get_mut(stringify!($provider_mod)).unwrap().get_mut(&$domain.to_string()).unwrap().qa,
                            )?
                        }
                    } else {
                        if !$no_ipv4 {
                            $provider_mod::update_address_records(
                                &$client_handle,
                                $token,
                                $domain,
                                "A",
                                &$tracker.get_mut(stringify!($provider_mod)).unwrap().get_mut(&$domain.to_string()).unwrap().a,
                            )?;

                            $tracker.get_mut(stringify!($provider_mod)).unwrap().get_mut(&$domain.to_string()).unwrap().a.ip = $ipv4.clone();
                        }
                        if !$no_ipv6 {
                            $provider_mod::update_address_records(
                                &$client_handle,
                                $token,
                                $domain,
                                "AAAA",
                                &$tracker.get_mut(stringify!($provider_mod)).unwrap().get_mut(&$domain.to_string()).unwrap().qa,
                            )?;

                            $tracker.get_mut(stringify!($provider_mod)).unwrap().get_mut(&$domain.to_string()).unwrap().qa.ip = $ipv6.clone();
                        }
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
    inner: ClientHandle,
    pub(crate) cache: Cache,
}

impl Client {
    /// Creates a [`Client`] instance.
    pub(crate) fn init() -> Result<Client> {
        Ok(Client { inner: ClientHandle::new(), cache: Cache::load()? })
    }

    /// Retrieves the current IPv4 address.
    pub(crate) fn get_ipv4_address(&self) -> Result<String> {
        self.inner
            .get("https://api.ipify.org")
            .send()
            .context("Failed to retrieve the IPv4 address.")?
            .text()
            .context("Failed to convert the IPv4 address to a UTF-8 string.")
    }

    /// Retrieves the current IPv6 address.
    pub(crate) fn get_ipv6_address(&self) -> Result<String> {
        self.inner
            .get("https://api6.ipify.org")
            .send()
            .context("Failed to retrieve the IPv6 address.")?
            .text()
            .context("Failed to convert the IPv6 address to a UTF-8 string.")
    }

    /// Updates the IPv4 (A) and IPv6 (AAAA) address records for the given provider's domains.
    pub(crate) fn update_address_records(
        &self,
        tokens: Option<Vec<String>>,
        no_ipv4: bool,
        no_ipv6: bool,
        ipv4: Option<String>,
        ipv6: Option<String>,
        interval: Option<u64>,
    ) -> Result<()> {
        let mut looped = false;

        // Retrieves the tokens that will be used; defaults to all tokens if none where provided.
        let tokens: Vec<&Token> = match tokens {
            Some(v) => self.cache.tokens.iter().filter(|token| v.contains(&token.name)).collect(),
            None => self.cache.tokens.iter().collect(),
        };

        // Validates the IPv4 addresse.
        let mut ipv4 = if !no_ipv4 {
            match ipv4 {
                Some(ip) => {
                    ip.parse::<Ipv4Addr>().context(format!("The provided IPv4 address is invalid '{}'.", ip))?.to_string()
                }
                None => self.get_ipv4_address()?,
            }
        } else {
            String::new()
        };

        // Validates the IPv6 addresse.
        let mut ipv6 = if !no_ipv6 {
            match ipv6 {
                Some(ip) => {
                    ip.parse::<Ipv6Addr>().context(format!("The provided IPv6 address is invalid '{}'.", ip))?.to_string()
                }
                None => self.get_ipv6_address()?,
            }
        } else {
            String::new()
        };

        // Tracks the address records on the different DNS providers.
        let mut tracker: HashMap<String, HashMap<String, AddressRecords>> = HashMap::new();
        for token in &tokens {
            tracker.entry(token.provider.clone()).or_insert(HashMap::new());
            for domain in &token.domains {
                tracker.get_mut(&token.provider).unwrap().entry(domain.to_string()).or_insert(AddressRecords::default());
            }
        }

        loop {
            // Updates the address records.
            for token in &tokens {
                for domain in &token.domains {
                    match_provider!(self.inner, token, domain, tracker, looped, no_ipv4, no_ipv6, ipv4, ipv6, [porkbun]);
                }
            }

            match interval {
                Some(v) => thread::sleep(Duration::from_secs(v)),
                None => break,
            }

            if !no_ipv4 {
                ipv4 = self.get_ipv4_address()?;
            }

            if !no_ipv6 {
                ipv6 = self.get_ipv6_address()?;
            }

            looped = true;
        }

        Ok(())
    }
}
