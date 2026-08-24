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

// `AAAA` is the canonical DNS record type name, so the acronym lint doesn't apply.
#[allow(clippy::upper_case_acronyms)]
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
    /// Validates the provided IP address and stores its normalized form.
    pub fn set_ip(&mut self, addr_type: AddrType, ip: impl Into<String>) -> Result<()> {
        let ip = ip.into();
        match addr_type {
            AddrType::A => {
                self.ipv4 = Some(
                    ip.parse::<Ipv4Addr>().context(format!("The provided IPv4 address is invalid: '{}'.", ip))?.to_string(),
                )
            }
            AddrType::AAAA => {
                self.ipv6 = Some(
                    ip.parse::<Ipv6Addr>().context(format!("The provided IPv6 address is invalid: '{}'.", ip))?.to_string(),
                )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hostnames_are_parsed_and_lowercased() {
        let host = Host::new("Sub.Example.COM").unwrap();
        assert_eq!(host.as_str(), "sub.example.com");
        assert_eq!(host.dom(), "example.com");
        assert_eq!(host.sub(), "sub");
    }

    #[test]
    fn root_domains_have_no_subdomain() {
        let host = Host::new("example.com").unwrap();
        assert_eq!(host.dom(), "example.com");
        assert_eq!(host.sub(), "");
    }

    #[test]
    fn invalid_hostnames_are_rejected() {
        for name in ["", "no-tld", "example.notarealtld", "spa ce.com"] {
            assert!(Host::new(name).is_err(), "'{}' should be an invalid hostname", name);
        }
    }

    #[test]
    fn set_ip_validates_and_normalizes() {
        let mut data = AddrData::default();

        assert!(data.set_ip(AddrType::A, "not-an-ip").is_err());
        assert!(data.set_ip(AddrType::AAAA, "999::g").is_err());

        data.set_ip(AddrType::A, "192.168.1.1").unwrap();
        data.set_ip(AddrType::AAAA, "2001:0db8:0000:0000:0000:0000:0000:0001").unwrap();
        assert_eq!(data.ipv4().as_deref(), Some("192.168.1.1"));
        assert_eq!(data.ipv6().as_deref(), Some("2001:db8::1"));
    }
}
