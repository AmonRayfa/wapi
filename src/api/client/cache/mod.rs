// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module contains the structs and methods used to manipulate the client's cache.

mod host;
mod token;

pub(crate) use host::{AddrData, AddrType, Host, HostJoinExt};
pub(crate) use token::Token;

use super::providers::Provider;
use directories::BaseDirs;
use mabe::{Context, Result, bail};
use rkyv::{Archive, Deserialize, Serialize};
use std::collections::{HashMap, hash_map::Entry};
use std::fs;
use std::path::PathBuf;

/// The struct used to manipulate the client's cache.
#[derive(Archive, Clone, Debug, Default, Deserialize, Serialize)]
pub struct Cache(HashMap<Token, HashMap<Host, AddrData>>);

impl Cache {
    /// Returns the path to the cache file (if it exists).
    fn get_path() -> Result<PathBuf> {
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
    pub fn load() -> Result<Self> {
        let cache_path = Self::get_path()?;

        let cache = if !cache_path.exists() {
            Self::default()
        } else {
            // Reads the file from the disk.
            let bytes = fs::read(&cache_path).context("Failed to read the client's cache file from the disk.")?;

            // Deserializes the bytes vector into a `Cache` struct.
            rkyv::from_bytes::<Self, rkyv::rancor::Error>(&bytes)
                .map_err(|e| std::io::Error::other(e.to_string()))
                .context("Failed to deserialize the client's cache data.")?
        };

        Ok(cache)
    }

    /// Returns an immutable reference to the cache's underlying [`HashMap`].
    pub fn content(&self) -> &HashMap<Token, HashMap<Host, AddrData>> {
        &self.0
    }

    /// Returns tokens from the cache based on token names.
    pub fn get_tokens(&self, names: impl IntoIterator<Item = impl Into<String>>) -> Result<Vec<Token>> {
        let token_names = names.into_iter().map(Into::into).collect::<Vec<String>>();
        let mut tokens = Vec::new();

        for name in &token_names {
            match self.0.get_key_value(name.as_str()).map(|(token, _)| token) {
                Some(t) => tokens.push(t.clone()),
                None => bail!("Token '{}' does not exist in the cache.", name),
            }
        }

        Ok(tokens)
    }

    /// Returns an immutable reference to the DNS mappings of a token.
    pub fn get_maps(&self, token_name: impl Into<String>) -> Option<&HashMap<Host, AddrData>> {
        self.0.get(token_name.into().as_str())
    }

    /// Returns a mutable reference to the DNS mappings of a token.
    pub fn get_mut_maps(&mut self, token_name: impl Into<String>) -> Option<&mut HashMap<Host, AddrData>> {
        self.0.get_mut(token_name.into().as_str())
    }

    /// Creates a token in the cache.
    pub fn create_token(
        &mut self,
        name: impl Into<String>,
        provider_id: impl Into<String>,
        api_key: impl Into<String>,
        secret_api_key: Option<impl Into<String>>,
    ) -> Result<()> {
        let name = name.into();
        let token = match secret_api_key {
            Some(k) => Token::new(name.clone(), provider_id, api_key)?.with_secret(k),
            None => Token::new(name.clone(), provider_id, api_key)?,
        };

        match self.0.entry(token) {
            Entry::Occupied(_) => {
                bail!("A token with the name '{}' already exists in the client's cache.", name);
            }
            Entry::Vacant(token_entry) => {
                token_entry.insert(HashMap::new());
                Ok(())
            }
        }
    }

    /// Deletes a token from the cache.
    pub fn delete_token(&mut self, name: impl Into<String>) -> Result<()> {
        let name = name.into();
        if self.0.remove(name.as_str()).is_some() {
            Ok(())
        } else {
            bail!("There is no token named '{}' the client's cache.", name);
        }
    }

    /// Adds hostnames to cached tokens.
    pub fn add_hosts(
        &mut self,
        hostnames: impl IntoIterator<Item = impl Into<String>>,
        token_names: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<()> {
        // Extracts the tokens that will be used.
        let target_tokens = self.get_tokens(token_names)?;

        // Validates the hostnames.
        let hosts = hostnames.into_iter().map(|h| Host::new(h)).collect::<Result<Vec<Host>>>()?;

        // Adds the hostnames.
        for token in target_tokens {
            if let Some(dns_maps) = self.0.get_mut(&token) {
                for host in &hosts {
                    dns_maps.entry(host.clone()).or_insert_with(AddrData::default);
                }
            }
        }

        Ok(())
    }

    /// Removes hostnames from cached tokens.
    pub fn remove_hosts(
        &mut self,
        hostnames: impl IntoIterator<Item = impl Into<String>>,
        token_names: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<()> {
        // Extracts the tokens that will be used.
        let target_tokens = self.get_tokens(token_names)?;

        // Validates the hostnames.
        let hosts = hostnames.into_iter().map(|h| Host::new(h)).collect::<Result<Vec<Host>>>()?;

        // Removes the hostnames.
        for token in target_tokens {
            if let Some(dns_maps) = self.0.get_mut(&token) {
                for host in &hosts {
                    dns_maps.remove(host);
                }
            }
        }

        Ok(())
    }

    /// Saves the [`Cache`] instance to the cache file (the location of the file depends on the operating system).
    pub fn save(&mut self) -> Result<()> {
        let cache_path = Self::get_path()?;

        // Serializes the struct into a byte vector.
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map_err(|e| std::io::Error::other(e.to_string()))
            .context("Failed to serialize the client's cache data.")?;

        // Writes the bytes vector to the disk.
        fs::write(&cache_path, bytes).context("Failed to write the client's cache file to the disk.")?;

        Ok(())
    }
}
