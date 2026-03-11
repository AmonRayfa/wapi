// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module contains the structs and methods used to manipulate the program's cache.

use crate::api::SUPPORTED_DNS_PROVIDERS;
use addr::parse_domain_name;
use directories::BaseDirs;
use mabe::{Context, Result, bail};
use rkyv::{Archive, Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Archive, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Token {
    pub(crate) name: String,
    pub(crate) domains: Vec<String>,
    pub(crate) provider: String,
    pub(crate) api_key: String,
    pub(crate) secret_api_key: String,
}

/// The struct used to manipulate the client's cache.
#[derive(Archive, Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct Cache {
    pub(crate) ipv4: String,
    pub(crate) ipv6: String,
    pub(crate) tokens: Vec<Token>,
}

impl Cache {
    /// Returns the path to the cache file (if it exists).
    fn get() -> Result<PathBuf> {
        let path = BaseDirs::new()
            .context("No valid home directory could be retrieved from the operating system.")?
            .home_dir()
            .join(".wapi")
            .join("cache");

        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        Ok(path)
    }

    /// Loads the cache file (the location depends on the operating system), and returns it as a [`Cache`] instance; returns a
    /// default [`Cache`] instance if the cache file doesn't exist.
    pub(crate) fn load() -> Result<Cache> {
        let cache_path = Cache::get()?;

        let cache = if !cache_path.exists() {
            Cache::default()
        } else {
            // Reads the file from the disk.
            let bytes = fs::read(&cache_path).context("Failed to read cache file from the disk.")?;

            // Deserializes the bytes vector into a [`Cache`] struct.
            rkyv::from_bytes::<Cache, rkyv::rancor::Error>(&bytes)
                .map_err(|e| std::io::Error::other(e.to_string()))
                .context("Failed to deserialize the cache data.")?
        };

        Ok(cache)
    }

    /// Creates a token in the cache.
    pub(crate) fn create_token(
        &mut self,
        name: String,
        provider: String,
        api_key: String,
        secret_api_key: Option<String>,
    ) -> Result<()> {
        // Ensures that the token name is unique.
        let token_names: Vec<&str> = self.tokens.iter().map(|token| token.name.as_str()).collect();
        if token_names.contains(&name.as_str()) {
            bail!("Token name '{}' already exists.", name);
        }

        // Ensures that the provider is valid.
        let provider_ids: Vec<&str> = SUPPORTED_DNS_PROVIDERS.iter().map(|provider| provider.id).collect();
        if !provider_ids.contains(&provider.as_str()) {
            bail!("Unsupported provider: '{}'.", provider);
        }

        // Adds the token to the cache.
        self.tokens.push(Token {
            name,
            domains: Vec::new(),
            provider,
            api_key,
            secret_api_key: secret_api_key.unwrap_or_default(),
        });
        Ok(())
    }

    /// Deletes a token from the cache.
    pub(crate) fn delete_token(&mut self, name: String) -> Result<()> {
        for (index, token) in self.tokens.iter().enumerate() {
            if name == token.name {
                self.tokens.remove(index);
                return Ok(());
            }
        }

        bail!("The token name you provided does not exist in the cache: '{}'.", name);
    }

    /// Adds domain names to cached tokens.
    pub(crate) fn add_domains(&mut self, domains: Vec<String>, tokens: Option<Vec<String>>) -> Result<()> {
        // Validates the domain names.
        for domain in &domains {
            match parse_domain_name(domain.as_str()) {
                Ok(d) => {
                    // Validates the TLD.
                    if !d.has_known_suffix() {
                        bail!("Invalid domain: '{}'. Unknown TLD.", domain);
                    }

                    // Ensures the domanin name does not contain more than 1 subdomain.
                    if let Some(prefix) = d.prefix()
                        && prefix.contains('.')
                    {
                        bail!("Invalid domain: '{}'. Contains more than 1 subdomain.", domain);
                    }
                }
                Err(e) => bail!("{}", e), // Fails RFC syntax validation.
            };
        }

        // Retrieves the tokens to which the domain names will be added; defaults to all tokens if none where provided.
        let token_names = match tokens {
            Some(v) => v,
            None => self.tokens.iter().map(|t| t.name.clone()).collect(),
        };

        // Adds the domain names.
        for token in &mut self.tokens {
            if token_names.contains(&token.name) {
                // Checks for duplicates before adding the domains.
                for domain in &domains {
                    if !token.domains.contains(domain) {
                        token.domains.push(domain.clone());
                    }
                }
            }
        }

        Ok(())
    }

    /// Removes domain names from cached tokens.
    pub(crate) fn remove_domains(&mut self, domains: Vec<String>, tokens: Option<Vec<String>>) -> Result<()> {
        // Retrieves the tokens from which the domain names will be removed; defaults to all tokens if none where provided.
        let token_names = match tokens {
            Some(v) => v,
            None => self.tokens.iter().map(|t| t.name.clone()).collect(),
        };

        // Removes the domain names.
        for token in &mut self.tokens {
            if token_names.contains(&token.name) {
                token.domains.retain(|d| !domains.contains(d));
            }
        }

        Ok(())
    }

    /// Saves the [`Cache`] instance to the cache file (the location of the file depends on the operating system).
    pub(crate) fn save(&mut self) -> Result<()> {
        let cache_path = BaseDirs::new()
            .context("No valid home directory could be retrieved from the operating system.")?
            .home_dir()
            .join(".wapi")
            .join("cache");

        // Serializes the struct into a byte vector.
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map_err(|e| std::io::Error::other(e.to_string()))
            .context("Failed to serialize the cache data.")?;

        // Writes the bytes vector to the disk.
        fs::write(&cache_path, bytes).context("Failed to write the cache file to the disk.")?;

        Ok(())
    }
}
