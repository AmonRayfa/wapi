// Copyright 2025 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module contains the struct and methods used to manipulate the program's cache file.

use chrono::Local;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Metadata {
    warning: String,
    name: String,
    version: String,
    description: String,
    homepage: String,
    timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DNSCredential {
    provider: String,
    api_key: String,
    secret_api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Data {
    ipv4_address: String,
    ipv6_address: String,
    dns_credentials: Vec<DNSCredential>,
}

/// The struct used to manipulate the program's cache file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    #[serde(rename = "METADATA")]
    metadata: Metadata,
    #[serde(rename = "DATA")]
    data: Data,
}

impl Cache {
    // Creates a new cache instance with empty values.
    fn new() -> Cache {
        Cache {
            metadata: Metadata {
                warning: String::new(),
                name: String::new(),
                version: String::new(),
                description: String::new(),
                homepage: String::new(),
                timestamp: String::new(),
            },
            data: Data { ipv4_address: String::new(), ipv6_address: String::new(), dns_credentials: Vec::new() },
        }
    }

    // Formats the cache file.
    fn fmt(&mut self) -> () {
        // Ensures the metadata is correct.
        self.metadata.warning = String::from("THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY. IF THE FILE IS TAMPERED WITH, IT WILL BE OVERWRITTEN WITH DEFAULT DATA, AND ALL PREVIOUS DATA WILL BE LOST.");
        self.metadata.name = String::from("wapi-cache");
        self.metadata.version = env!("CARGO_PKG_VERSION").to_string();
        self.metadata.description = String::from("The cache file for the Wapi client.");
        self.metadata.homepage = String::from("https://github.com/AmonRayfa/wapi");
        self.metadata.timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();

        // Ensures the IPv4 address is valid and replaces it with a default value if it is not.
        match self.data.ipv4_address.parse::<Ipv4Addr>() {
            Ok(_) => {}
            Err(_) => self.data.ipv4_address = String::from("0.0.0.0"),
        }

        // Ensures the IPv6 address is valid and replaces it with a default value if it is not.
        match self.data.ipv6_address.parse::<Ipv6Addr>() {
            Ok(_) => {}
            Err(_) => self.data.ipv6_address = String::from("0:0:0:0:0:0:0:0"),
        }

        self.data.dns_credentials = self
            .data
            .dns_credentials
            .iter()
            .rev()
            .filter(|credentials| match credentials.provider.as_str() {
                "alibabacloud" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "alibabacloud").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "bluehost" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "bluehost").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "cloudflare" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "cloudflare").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "dnspod" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "dnspod").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "dreamhost" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "dreamhost").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "dynadot" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "dynadot").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "enom" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "enom").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "epik" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "epik").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "gandi" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "gandi").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "godaddy" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "godaddy").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "hover" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "hover").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "ionos" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "ionos").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "namecheap" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "namecheap").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "namesilo" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "namesilo").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "porkbun" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "porkbun").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "resellerclub" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "resellerclub").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "opensrs" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "opensrs").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                "ovh" => {
                    if self.data.dns_credentials.iter().filter(|c| c.provider == "ovh").count() > 1 {
                        false
                    } else {
                        true
                    }
                }
                _ => false,
            })
            .cloned()
            .collect();
    }
}
