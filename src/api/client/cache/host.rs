// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use addr::parse_domain_name;
use mabe::{Context, Result, bail};
use rkyv::{Archive, Deserialize, Serialize};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Archive, Clone, Debug, Default, Deserialize, Serialize)]
pub struct Host {
    name: String,
    dom: String,
    sub: String,
}

impl Host {
    pub fn new(name: impl Into<String>) -> Result<Self> {
        let name = name.into().to_lowercase();
        match parse_domain_name(name.as_str()) {
            Ok(d) => {
                // Validates the hostname's format and extracts its domain name.
                let dom = d
                    .root()
                    .context(format!(
                        "This hostname is invalid: '{}'. A hostname must at least contain a TLD and an SLD.",
                        name
                    ))?
                    .to_string();

                // Validates the host's TLD.
                if !d.has_known_suffix() {
                    bail!("Invalid hostname: '{}'. Unknown TLD.", name);
                }

                // Extracts the host's subdomain.
                let sub = match d.prefix() {
                    Some(s) => s.to_string(),
                    None => String::new(),
                };

                Ok(Self { name, dom, sub })
            }
            Err(e) => bail!("{}", e), // Fails RFC syntax validation.
        }
    }

    pub fn dom(&self) -> &str {
        &self.dom
    }

    pub fn sub(&self) -> &str {
        &self.sub
    }

    pub fn as_str(&self) -> &str {
        &self.name
    }
}

/// Only the `name` field is used when displaying a [`Host`].
impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// [`Host`] instances are considered equal only if they have the same `name` field.
impl PartialEq for Host {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl PartialEq for rkyv::Archived<Host> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Host {}
impl Eq for rkyv::Archived<Host> {}

/// Only the `name` field is used when hashing a [`Host`].
impl Hash for Host {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl Hash for rkyv::Archived<Host> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

/// Allows hostnames to be passed as strings for lookups.
impl Borrow<str> for Host {
    fn borrow(&self) -> &str {
        &self.name
    }
}

pub trait HostJoinExt {
    fn join(&self, separator: &str) -> String;
}

impl HostJoinExt for HashMap<Host, AddrData> {
    fn join(&self, separator: &str) -> String {
        let hostnames = self.keys().map(|h| h.to_string()).collect::<Vec<String>>();
        hostnames.join(separator)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AddrType {
    A,
    AAAA,
}

impl AddrType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::A => "A",
            Self::AAAA => "AAAA",
        }
    }
}

impl fmt::Display for AddrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Archive, Clone, Debug, Default, Deserialize, Serialize)]
pub struct AddrData {
    ipv4: Option<String>,
    ipv6: Option<String>,
}

impl AddrData {
    pub fn update_ip(&mut self, addr_type: AddrType, ip: impl Into<String>) -> Result<()> {
        let ip = ip.into();
        match addr_type {
            AddrType::A => {
                if self.ipv4.is_some() {
                    self.ipv4 = Some(
                        ip.parse::<Ipv4Addr>().context(format!("The provided IPv4 address is invalid: '{}'.", ip))?.to_string(),
                    )
                }
            }
            AddrType::AAAA => {
                if self.ipv6.is_some() {
                    self.ipv6 = Some(
                        ip.parse::<Ipv6Addr>().context(format!("The provided IPv6 address is invalid: '{}'.", ip))?.to_string(),
                    )
                }
            }
        };
        Ok(())
    }

    pub fn ipv4(&self) -> Option<String> {
        self.ipv4.clone()
    }

    pub fn ipv6(&self) -> Option<String> {
        self.ipv6.clone()
    }
}
