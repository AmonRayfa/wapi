// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module contains the struct and methods used to retrieve the public IP address of the client and update the DNS records.

use crate::api::cache::Cache;
use addr::parse_domain_name;
use mabe::{Context, Result, bail};
use reqwest::blocking::Client as ClientHandle;
use serde::Deserialize;
use serde_json::json;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::thread;
use std::time::Duration;

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

    /// Updates the `A` and `AAAA` DNS records for a given domain and IP address on the specified provider.
    pub(crate) fn update_dns_records(
        self,
        tokens: Option<Vec<String>>,
        no_ipv4: bool,
        no_ipv6: bool,
        ipv4: Option<String>,
        ipv6: Option<String>,
        interval: Option<u64>,
    ) -> Result<()> {
        // Unwraps the tokens.
        let tokens = tokens.unwrap_or_default();

        // Validates the IPv4 address.
        let ipv4 = match !no_ipv4 {
            true => match ipv4 {
                Some(ip) => {
                    ip.parse::<Ipv4Addr>().context(format!("The provided IPv4 address is invalid '{}'.", ip))?.to_string()
                }
                None => self.get_ipv4_address()?,
            },
            false => String::new(),
        };

        // Validates the IPv6 address.
        let ipv6 = match !no_ipv6 {
            true => match ipv6 {
                Some(ip) => {
                    ip.parse::<Ipv6Addr>().context(format!("The provided IPv6 address is invalid '{}'.", ip))?.to_string()
                }
                None => self.get_ipv6_address()?,
            },
            false => String::new(),
        };

        loop {
            // Updates the DNS records.
            for token in &self.cache.tokens {
                if tokens.contains(&token.name) || tokens.is_empty() {
                    match token.provider.as_str() {
                        "alibabacloud" => {
                            //TODO: Implementation for Alibaba Cloud.
                        }
                        "bluehost" => {
                            //TODO: Implementation for Bluehost.
                        }
                        "cloudflare" => {
                            //TODO: Implementation for Cloudflare.
                        }
                        "dnspod" => {
                            //TODO: Implementation for DNSPod.
                        }
                        "dreamhost" => {
                            //TODO: Implementation for Dreamhost.
                        }
                        "dynadot" => {
                            //TODO: Implementation for Dynadot.
                        }
                        "enom" => {
                            //TODO: Implementation for Enom.
                        }
                        "epik" => {
                            //TODO: Implementation for Epik.
                        }
                        "gandi" => {
                            //TODO: Implementation for Gandi.
                        }
                        "godaddy" => {
                            //TODO: Implementation for GoDaddy.
                        }
                        "hover" => {
                            //TODO: Implementation for Hover.
                        }
                        "ionos" => {
                            //TODO: Implementation for Ionos.
                        }
                        "namecheap" => {
                            //TODO: Implementation for Namecheap.
                        }
                        "namesilo" => {
                            //TODO: Implementation for Namesilo.
                        }
                        "opensrs" => {
                            //TODO: Implementation for OpenSRS.
                        }
                        "ovh" => {
                            //TODO: Implementation for OVH.
                        }
                        "porkbun" => {
                            #[derive(Debug, Deserialize)]
                            struct PorkbunResponse {
                                status: String,
                                message: Option<String>,
                            }

                            let mut payload = json!({
                                "apikey": token.api_key,
                                "secretapikey": token.secret_api_key,
                            });

                            for domain in &token.domains {
                                match parse_domain_name(domain.as_str()) {
                                    Ok(d) => {
                                        // Extracts the subdomain if there is one.
                                        let subdomain = d.prefix().unwrap_or_default();

                                        // Updates the 'A' record of the domain if an IPv4 address was provided.
                                        if !ipv4.is_empty() {
                                            payload["content"] = json!(ipv4);
                                            let response: PorkbunResponse = self
                                                .inner
                                                .post(format!(
                                                    "https://api.porkbun.com/api/json/v3/dns/editByNameType/{}/A/{}",
                                                    domain, subdomain
                                                ))
                                                .json(&payload)
                                                .send()?
                                                .json()?;

                                            if &response.status == "ERROR" {
                                                match response.message {
                                                    Some(msg) => bail!(
                                                        "Failed to update the 'A' record of domain '{}' on Porkbun.\n{}",
                                                        domain,
                                                        msg
                                                    ),
                                                    None => bail!(
                                                        "Failed to update the 'A' record of domain '{}' on Porkbun.\nNo cause was provided.",
                                                        domain
                                                    ),
                                                };
                                            }
                                        }

                                        // Updates the 'AAAA' record of the domain if an IPv6 address was provided.
                                        if !ipv6.is_empty() {
                                            payload["content"] = json!(ipv6);
                                            let response: PorkbunResponse = self
                                                .inner
                                                .post(format!(
                                                    "https://api.porkbun.com/api/json/v3/dns/editByNameType/{}/AAAA/{}",
                                                    domain, subdomain
                                                ))
                                                .json(&payload)
                                                .send()?
                                                .json()?;

                                            if &response.status == "ERROR" {
                                                match response.message {
                                                    Some(msg) => bail!(
                                                        "Failed to update the 'AAAA' record of domain '{}' on Porkbun.\n{}",
                                                        domain,
                                                        msg
                                                    ),
                                                    None => bail!(
                                                        "Failed to update the 'AAAA' record of domain '{}' on Porkbun.\nNo cause was provided.",
                                                        domain
                                                    ),
                                                };
                                            }
                                        }
                                    }
                                    Err(e) => bail!("{}", e), // Fails RFC syntax validation.
                                };
                            }
                        }
                        "resellerclub" => {
                            //TODO: Implementation for ResellerClub.
                        }
                        p => println!("The following provider is not supported and will be skipped: '{}'.", p),
                    };
                }
            }

            match interval {
                Some(v) => thread::sleep(Duration::from_secs(v)),
                None => break,
            }
        }

        Ok(())
    }
}
