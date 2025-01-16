// Copyright 2025 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module contains the struct and methods used to manipulate the program's cache file.

use crate::error::api::{Error, Result};
use chrono::Local;
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
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
    /// Creates a new cache instance with empty values.
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

    /// Formats the cache file.
    fn fmt(&mut self) {
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
    }

    /// Retrieves the cache file's location. If the cache file cannot be located, an error is returned, and if the cache file
    /// does not exist, a new empty ache file is created.
    pub fn locate() -> Result<PathBuf> {
        // Retrieves the cache file location.
        let cache_location = match BaseDirs::new() {
            Some(base_dirs) => base_dirs.home_dir().join(Path::new("wapi")).join(Path::new("cache.json")),
            None => {
                return Err(Error::Cache(
                    String::from("locate"),
                    String::from("No valid user home directory path could be retrieved from the operating system."),
                ))
            }
        };

        // Ensures that the `wapi/` directory and its parent directories exist, and create them if they don't.
        if let Some(parent_dir) = cache_location.parent() {
            std::fs::create_dir_all(parent_dir).map_err(|err| Error::Cache(String::from("locate"), err.to_string()))?;
        }

        // Creates a new empty cache file if it doesn't exist.
        if !cache_location.exists() {
            let mut file =
                File::create(&cache_location).map_err(|err| Error::Cache(String::from("locate"), err.to_string()))?;
            file.write_all(b"{}").map_err(|err| Error::Cache(String::from("locate"), err.to_string()))?;
        }

        Ok(cache_location)
    }

    /// Loads the cache file (the location depends on the operating system), formats it, and returns it as a cache instance. If
    /// the cache file does not exist, or is corrupted and cannot be formatted, a new cache instance is created with default
    /// values. The cache file is formatted by ensuring that the metadata is correct, the IP addresses are valid, and the DNS
    /// credentials are in the correct format. If the IP addresses are not valid, they are replaced with default values
    /// (`0.0.0.0` and `0:0:0:0:0:0:0:0` for IPv4 and IPv6 respectively). If the provider of a DNS credential is not recognized,
    /// the credential is removed from the cache. And if the provider of DNS credential appears more than once, only the most
    /// recent one is kept. For a list of supported providers and their formatted names, see the [GitHub
    /// repository](https://github.com/AmonRayfa/wapi).
    pub fn load() -> Result<Cache> {
        // Retrieves the cache file location.
        let cache_location = Cache::locate()?;

        // Reads the cache file and returns an error if it fails.
        let cache_file =
            std::fs::read_to_string(&cache_location).map_err(|err| Error::Cache(String::from("load"), err.to_string()))?;

        // Deserializes the cache file and returns a new cache instance if it fails.
        let mut cache = match serde_json::from_str(&cache_file) {
            Ok(cache) => cache,
            Err(_) => Cache::new(),
        };

        // Formats the cache file.
        cache.fmt();

        // Returns the loaded cache instance.
        Ok(cache)
    }

    /// Saves the cache instance to `wapi/cache.json` in the user's home directory (the location depends on the operating
    /// system). An error is returned if the cache file's location cannot be retrieved. The cache file is saved as a JSON file,
    /// and an error is returned if the serialization fails. If a cache file already exists, it is overwritten with the new
    /// cache instance.
    pub fn save(&mut self) -> Result<()> {
        // Retrieves the cache file location.
        let cache_location = Cache::locate()?;

        // Saves the cache file and returns an error if it fails.
        let cache = serde_json::to_string(self).map_err(|err| Error::Cache(String::from("save"), err.to_string()))?;
        std::fs::write(cache_location, cache).map_err(|err| Error::Cache(String::from("save"), err.to_string()))?;

        Ok(())
    }
}
