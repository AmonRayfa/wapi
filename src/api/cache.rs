// Copyright 2025 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module contains the struct and methods used to manipulate the program's cache.

use crate::error::api::{Error, Result};
use chrono::Local;
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::{Path, PathBuf};

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
    /// Creates a new cache instance with default values.
    pub fn new() -> Cache {
        let mut cache = Cache {
            metadata: Metadata {
                warning: String::new(),
                name: String::new(),
                version: String::new(),
                description: String::new(),
                homepage: String::new(),
                timestamp: String::new(),
            },
            data: Data { ipv4_address: String::new(), ipv6_address: String::new(), dns_credentials: Vec::new() },
        };

        cache.fmt();
        cache
    }

    /// Formats and timestamps the [`Cache`](wapi::Cache) instance (this method will be called after each change made to cache's
    /// content). The is done by ensuring that the metadata is correct, the IP addresses are valid, and the DNS providers are in
    /// the correct format. If the IP addresses are not valid, they are replaced with default values (`0.0.0.0` and
    /// `0:0:0:0:0:0:0:0` for IPv4 and IPv6 respectively). If the name of a DNS provider is not recognized, the DNS provider is
    /// removed from the cache. And if the name of a DNS provider appears more than once, only the most recent one is kept. For
    /// a list of the supported DNS providers and their formatted names, see the [GitHub
    /// repository](https://github.com/AmonRayfa/wapi).
    pub fn fmt(&mut self) {
        // Ensures the metadata is correct.
        self.metadata.warning = String::from("THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY. IF THE FILE IS TAMPERED WITH, IT WILL BE OVERWRITTEN WITH DEFAULT DATA, AND ALL PREVIOUS DATA WILL BE LOST.");
        self.metadata.name = String::from("wapi-cache");
        self.metadata.version = env!("CARGO_PKG_VERSION").to_string();
        self.metadata.description = String::from("The cache file for the Wapi client.");
        self.metadata.homepage = String::from("https://github.com/AmonRayfa/wapi");

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
                "alibabacloud" => self.data.dns_credentials.iter().filter(|c| c.provider == "alibabacloud").count() > 1,
                "bluehost" => self.data.dns_credentials.iter().filter(|c| c.provider == "bluehost").count() > 1,
                "cloudflare" => self.data.dns_credentials.iter().filter(|c| c.provider == "cloudflare").count() > 1,
                "dnspod" => self.data.dns_credentials.iter().filter(|c| c.provider == "dnspod").count() > 1,
                "dreamhost" => self.data.dns_credentials.iter().filter(|c| c.provider == "dreamhost").count() > 1,
                "dynadot" => self.data.dns_credentials.iter().filter(|c| c.provider == "dynadot").count() > 1,
                "enom" => self.data.dns_credentials.iter().filter(|c| c.provider == "enom").count() > 1,
                "epik" => self.data.dns_credentials.iter().filter(|c| c.provider == "epik").count() > 1,
                "gandi" => self.data.dns_credentials.iter().filter(|c| c.provider == "gandi").count() > 1,
                "godaddy" => self.data.dns_credentials.iter().filter(|c| c.provider == "godaddy").count() > 1,
                "hover" => self.data.dns_credentials.iter().filter(|c| c.provider == "hover").count() > 1,
                "ionos" => self.data.dns_credentials.iter().filter(|c| c.provider == "ionos").count() > 1,
                "namecheap" => self.data.dns_credentials.iter().filter(|c| c.provider == "namecheap").count() > 1,
                "namesilo" => self.data.dns_credentials.iter().filter(|c| c.provider == "namesilo").count() > 1,
                "opensrs" => self.data.dns_credentials.iter().filter(|c| c.provider == "opensrs").count() > 1,
                "ovh" => self.data.dns_credentials.iter().filter(|c| c.provider == "ovh").count() > 1,
                "porkbun" => self.data.dns_credentials.iter().filter(|c| c.provider == "porkbun").count() > 1,
                "resellerclub" => self.data.dns_credentials.iter().filter(|c| c.provider == "resellerclub").count() > 1,
                _ => false,
            })
            .cloned()
            .collect();

        // Timestamps the cache.
        self.metadata.timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    }

    /// Retrieves the cache file's path. A `None` value is returned if the user's home directory path cannot be retrieved from
    /// the operating system.
    pub fn get_path() -> Option<PathBuf> {
        match BaseDirs::new() {
            Some(base_dirs) => Some(base_dirs.home_dir().join(Path::new("wapi")).join(Path::new("cache.json"))),
            None => None,
        }
    }

    /// Loads the cache file (the location depends on the operating system), and returns it as a [`Cache`](wapi::Cache)
    /// instance. An error is returned if the cache file: does not exist, cannot be read to a string, or is corrupted and cannot
    /// be deserialized.
    pub fn load() -> Result<Cache> {
        // Retrieves the cache file's path and returns an error if it fails.
        let cache_path = match Cache::get_path() {
            Some(p) => p,
            None => {
                return Err(Error::Cache(
                    String::from("locate"),
                    String::from("No valid user home directory path could be retrieved from the operating system."),
                ))
            }
        };

        // Reads the cache file to a string and returns an error if it fails.
        let cache_file =
            std::fs::read_to_string(&cache_path).map_err(|err| Error::Cache(String::from("load"), err.to_string()))?;

        // Deserializes the cache file and returns an error if it fails.
        let cache = match serde_json::from_str(&cache_file) {
            Ok(c) => c,
            Err(e) => return Err(Error::Cache(String::from("load"), e.to_string())),
        };

        Ok(cache)
    }

    /// Saves the [`Cache`](wapi::Cache) instance to a JSON file (the location of the file depends on the operating system). An
    /// error is returned if the cache file's path is invalid, or if the [`Cache`](wapi::Cache) instance cannot be serialized.
    /// If a cache file already exists, it is overwritten with the new cache.
    pub fn save(&mut self) -> Result<()> {
        // Retrieves the cache file's path and returns an error if it fails.
        let cache_path = match Cache::get_path() {
            Some(p) => p,
            None => {
                return Err(Error::Cache(
                    String::from("locate"),
                    String::from("No valid user home directory path could be retrieved from the operating system."),
                ))
            }
        };

        // Ensures that the parent directories of the cache file exist, and creates them if they don't.
        if let Some(parent_dir) = cache_path.parent() {
            std::fs::create_dir_all(parent_dir).map_err(|err| Error::Cache(String::from("locate"), err.to_string()))?;
        } else {
            return Err(Error::Cache(
                String::from("locate"),
                String::from("No valid parent directory path could be retrieved from the cache file path."),
            ));
        }

        // Serializes the cache instance and returns an error if it fails.
        let cache = serde_json::to_string(self).map_err(|err| Error::Cache(String::from("save"), err.to_string()))?;
        std::fs::write(cache_path, cache).map_err(|err| Error::Cache(String::from("save"), err.to_string()))?;

        Ok(())
    }
}
