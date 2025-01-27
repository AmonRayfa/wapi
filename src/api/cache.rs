// Copyright 2025 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module contains the struct and methods used to manipulate the program's cache.

use crate::error::api::{Error, Result};
use chrono::Local;
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
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
struct DNSProvider {
    id: String,
    api_key: String,
    secret_api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Data {
    ipv4_address: String,
    ipv6_address: String,
    dns_providers: Vec<DNSProvider>,
}

/// The struct used to manipulate the program's cache file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    #[serde(rename = "METADATA")]
    metadata: Metadata,
    #[serde(rename = "DATA")]
    data: Data,
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
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
            data: Data { ipv4_address: String::new(), ipv6_address: String::new(), dns_providers: Vec::new() },
        };

        cache.fmt();
        cache
    }

    /// Formats and timestamps the [`Cache`](wapi::Cache) instance (this method will be called after each change made to cache's
    /// content). The is done by ensuring that the metadata is correct, the IP addresses are valid, and the DNS providers are in
    /// the correct format. If the IP addresses are not valid, they are replaced with default values (`0.0.0.0` and
    /// `0:0:0:0:0:0:0:0` for IPv4 and IPv6 respectively). If the ID of a DNS provider is not recognized, the DNS provider is
    /// removed from the cache. And if the ID of a DNS provider appears more than once, only the most recent one is kept. For a
    /// list of the supported DNS providers and their ID, see the [GitHub repository](https://github.com/AmonRayfa/wapi).
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

        // Removes duplicate DNS providers and ensures that only the most recent one is kept.
        let mut filtered_providers = HashSet::new();
        self.data.dns_providers.reverse();
        self.data.dns_providers.retain(|p| match p.id.as_str() {
            "alibabacloud" => filtered_providers.insert(p.id.clone()),
            "bluehost" => filtered_providers.insert(p.id.clone()),
            "cloudflare" => filtered_providers.insert(p.id.clone()),
            "dnspod" => filtered_providers.insert(p.id.clone()),
            "dreamhost" => filtered_providers.insert(p.id.clone()),
            "dynadot" => filtered_providers.insert(p.id.clone()),
            "enom" => filtered_providers.insert(p.id.clone()),
            "epik" => filtered_providers.insert(p.id.clone()),
            "gandi" => filtered_providers.insert(p.id.clone()),
            "godaddy" => filtered_providers.insert(p.id.clone()),
            "hover" => filtered_providers.insert(p.id.clone()),
            "ionos" => filtered_providers.insert(p.id.clone()),
            "namecheap" => filtered_providers.insert(p.id.clone()),
            "namesilo" => filtered_providers.insert(p.id.clone()),
            "opensrs" => filtered_providers.insert(p.id.clone()),
            "ovh" => filtered_providers.insert(p.id.clone()),
            "porkbun" => filtered_providers.insert(p.id.clone()),
            "resellerclub" => filtered_providers.insert(p.id.clone()),
            _ => false,
        });
        self.data.dns_providers.reverse();

        // Timestamps the cache.
        self.metadata.timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    }

    /// Retrieves the cache file's path. A `None` value is returned if the user's home directory path cannot be retrieved from
    /// the operating system.
    pub fn get_path() -> Option<PathBuf> {
        BaseDirs::new().map(|base_dirs| base_dirs.home_dir().join(Path::new("wapi")).join(Path::new("cache.json")))
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

    /// Adds a DNS provider to the cache. If the DNS provider already exists in the cache, it is replaced with the new one.
    pub fn add_dns_provider(&mut self, id: String, api_key: String, secret_api_key: String) {
        self.fmt();
        self.data.dns_providers.push(DNSProvider { id, api_key, secret_api_key });
        self.fmt();
    }

    /// Removes a DNS provider from the cache. If the DNS provider does not exist in the cache, nothing happens.
    pub fn remove_dns_provider(&mut self, id: String) {
        self.fmt();
        self.data.dns_providers.retain(|provider| provider.id != id);
        self.fmt();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cache() {
        let mut cache = Cache::new();
        assert_eq!(cache.metadata.name, "wapi-cache");
        assert_eq!(cache.metadata.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(cache.metadata.description, "The cache file for the Wapi client.");
        assert_eq!(cache.metadata.homepage, "https://github.com/AmonRayfa/wapi");
        assert_eq!(cache.data.ipv4_address, "0.0.0.0");
        assert_eq!(cache.data.ipv6_address, "0:0:0:0:0:0:0:0");
        assert_eq!(cache.data.dns_providers.len(), 0);

        cache.add_dns_provider("cloudflare".to_string(), "SOME_API_KEY_1".to_string(), "SOME_SECRET_API_KEY_1".to_string());
        assert_eq!(cache.data.dns_providers.len(), 1);
        assert_eq!(cache.data.dns_providers[0].api_key, "SOME_API_KEY_1");
        assert_eq!(cache.data.dns_providers[0].secret_api_key, "SOME_SECRET_API_KEY_1");

        cache.add_dns_provider("namesilo".to_string(), "SOME_API_KEY".to_string(), "SOME_SECRET_API_KEY".to_string());
        cache.add_dns_provider("bluehost".to_string(), "SOME_API_KEY".to_string(), "SOME_SECRET_API_KEY".to_string());
        cache.add_dns_provider("porkbun".to_string(), "SOME_API_KEY_1".to_string(), "SOME_SECRET_API_KEY_1".to_string());
        cache.add_dns_provider("namecheap".to_string(), "SOME_API_KEY".to_string(), "SOME_SECRET_API_KEY".to_string());
        cache.add_dns_provider("alibabacloud".to_string(), "SOME_API_KEY".to_string(), "SOME_SECRET_API_KEY".to_string());
        cache.add_dns_provider("some_random_name".to_string(), "SOME_API_KEY".to_string(), "SOME_SECRET_API_KEY".to_string());
        cache.add_dns_provider("dreamhost".to_string(), "SOME_API_KEY".to_string(), "SOME_SECRET_API_KEY".to_string());

        cache.add_dns_provider("cloudflare".to_string(), "SOME_API_KEY_2".to_string(), "SOME_SECRET_API_KEY_2".to_string());
        assert_eq!(cache.data.dns_providers.len(), 7);
        assert_eq!(cache.data.dns_providers[6].api_key, "SOME_API_KEY_2");
        assert_eq!(cache.data.dns_providers[cache.data.dns_providers.len() - 1].secret_api_key, "SOME_SECRET_API_KEY_2");

        cache.add_dns_provider("porkbun".to_string(), "SOME_API_KEY_2".to_string(), "SOME_SECRET_API_KEY_2".to_string());
        assert_eq!(cache.data.dns_providers.len(), 7);
        assert_eq!(cache.data.dns_providers[6].api_key, "SOME_API_KEY_2");
        assert_eq!(cache.data.dns_providers[cache.data.dns_providers.len() - 1].secret_api_key, "SOME_SECRET_API_KEY_2");

        cache.remove_dns_provider("cloudflare".to_string());
        assert_eq!(cache.data.dns_providers.len(), 6);
        assert_eq!(cache.data.dns_providers[0].id, "namesilo");
        assert_eq!(cache.data.dns_providers[1].id, "bluehost");
        assert_eq!(cache.data.dns_providers[2].id, "namecheap");
        assert_eq!(cache.data.dns_providers[3].id, "alibabacloud");
        assert_eq!(cache.data.dns_providers[4].id, "dreamhost");
        assert_eq!(cache.data.dns_providers[5].id, "porkbun");

        cache.remove_dns_provider("dreamhost".to_string());
        assert_eq!(cache.data.dns_providers.len(), 5);
        assert_eq!(cache.data.dns_providers[0].id, "namesilo");
        assert_eq!(cache.data.dns_providers[1].id, "bluehost");
        assert_eq!(cache.data.dns_providers[2].id, "namecheap");
        assert_eq!(cache.data.dns_providers[3].id, "alibabacloud");
        assert_eq!(cache.data.dns_providers[4].id, "porkbun");

        cache.remove_dns_provider("namesilo".to_string());
        assert_eq!(cache.data.dns_providers.len(), 4);
        assert_eq!(cache.data.dns_providers[0].id, "bluehost");
        assert_eq!(cache.data.dns_providers[1].id, "namecheap");
        assert_eq!(cache.data.dns_providers[2].id, "alibabacloud");
        assert_eq!(cache.data.dns_providers[3].id, "porkbun");
    }
}
