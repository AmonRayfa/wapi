// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module contains the structs and methods used to manipulate the client's cache.

mod domain;

use crate::api::SUPPORTED_DNS_PROVIDERS;
use directories::BaseDirs;
pub(crate) use domain::{Domain, DomainJoinExt};
use mabe::{Context, Result, bail};
use rkyv::{Archive, Deserialize, Serialize};
use std::collections::{HashMap, HashSet, hash_map::Entry};
use std::fs;
use std::path::PathBuf;

#[derive(Archive, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TokenData {
    pub(crate) provider: String,
    pub(crate) api_key: String,
    pub(crate) secret_api_key: String,
    pub(crate) domains: HashSet<Domain>,
}

/// The struct used to manipulate the client's cache.
#[derive(Archive, Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct Cache(HashMap<String, TokenData>);

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

    /// Gives a read-only access to the cache's content.
    pub(crate) fn content(&self) -> &HashMap<String, TokenData> {
        &self.0
    }

    /// Creates a token in the cache.
    pub(crate) fn create_token(
        &mut self,
        name: String,
        provider: String,
        api_key: String,
        secret_api_key: Option<String>,
    ) -> Result<()> {
        // Ensures that the provider is valid.
        let provider_ids: Vec<&str> = SUPPORTED_DNS_PROVIDERS.iter().map(|provider| provider.id()).collect();
        if !provider_ids.contains(&provider.as_str()) {
            bail!("Unsupported provider: '{}'.\nRun 'wapi -p' to get a list of the supported DNS service providers.", provider);
        }

        // Adds the token to the cache while ensuring the token name is unique.
        match self.0.entry(name.clone()) {
            Entry::Occupied(_) => bail!("Token name '{}' already exists.", name),
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(TokenData {
                    provider,
                    api_key,
                    secret_api_key: secret_api_key.unwrap_or_default(),
                    domains: HashSet::new(),
                });
                Ok(())
            }
        }
    }

    /// Deletes a token from the cache.
    pub(crate) fn delete_token(&mut self, name: String) -> Result<()> {
        match self.0.remove(&name) {
            Some(_) => Ok(()),
            None => bail!("Token name '{}' does not exist.", name),
        }
    }

    /// Adds domain names to cached tokens.
    pub(crate) fn add_domains(&mut self, domains: Vec<String>, tokens: Option<Vec<String>>) -> Result<()> {
        // Validates the domain names.
        let domains: Vec<Domain> = domains.iter().map(|d| Domain::from(d)).collect::<Result<Vec<Domain>>>()?;

        // Defines the tokens to which the domains will be added; defaults to all tokens if none where provided.
        let target_tokens = match tokens {
            Some(v) => v,
            None => self.0.keys().cloned().collect(),
        };

        // Adds the domain names.
        for token in &target_tokens {
            for domain in &domains {
                if let Some(t) = self.0.get_mut(token) {
                    t.domains.insert(domain.clone());
                }
            }
        }

        Ok(())
    }

    /// Removes domain names from cached tokens.
    pub(crate) fn remove_domains(&mut self, domains: Vec<String>, tokens: Option<Vec<String>>) -> Result<()> {
        // Validates the domain names.
        let domains: Vec<Domain> = domains.iter().map(|d| Domain::from(d)).collect::<Result<Vec<Domain>>>()?;

        // Defines the tokens from which the domains will be removed; defaults to all tokens if none where provided.
        let target_tokens = match tokens {
            Some(v) => v,
            None => self.0.keys().cloned().collect(),
        };

        // Removes the domain names.
        for token in &target_tokens {
            for domain in &domains {
                if let Some(t) = self.0.get_mut(token) {
                    t.domains.remove(domain);
                }
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
