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

/// The magic bytes identifying a wapi cache file.
const MAGIC: [u8; 4] = *b"WAPI";

/// The version of the cache file format. Bump it whenever the serialized layout of the cache changes, so that older files are
/// rejected with a clear message instead of failing to deserialize (or worse, deserializing into garbage).
const FORMAT_VERSION: u8 = 1;

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

    /// Serializes the [`Cache`] instance into a byte vector, prefixed with the format header.
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let payload = rkyv::to_bytes::<rkyv::rancor::Error>(self).context("Failed to serialize the client's cache data.")?;

        let mut bytes = Vec::with_capacity(MAGIC.len() + 1 + payload.len());
        bytes.extend_from_slice(&MAGIC);
        bytes.push(FORMAT_VERSION);
        bytes.extend_from_slice(&payload);

        Ok(bytes)
    }

    /// Deserializes a byte vector into a [`Cache`] instance, validating the format header first.
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < MAGIC.len() + 1 || bytes[..MAGIC.len()] != MAGIC {
            bail!(
                "The client's cache file is not in a recognized format (it was probably created by an older version of the client). Delete the file and recreate your tokens."
            );
        }

        let version = bytes[MAGIC.len()];
        if version != FORMAT_VERSION {
            bail!(
                "The client's cache file uses an unsupported format version ('{}' instead of '{}'). Delete the file and recreate your tokens.",
                version,
                FORMAT_VERSION
            );
        }

        // Copies the payload into an aligned buffer, since rkyv requires aligned bytes and the format header offsets the
        // payload out of alignment.
        let mut payload = rkyv::util::AlignedVec::<16>::new();
        payload.extend_from_slice(&bytes[MAGIC.len() + 1..]);

        rkyv::from_bytes::<Self, rkyv::rancor::Error>(&payload).context("Failed to deserialize the client's cache data.")
    }

    /// Loads the cache file (the location depends on the operating system), and returns it as a [`Cache`] instance; returns a
    /// default [`Cache`] instance if the cache file doesn't exist.
    pub fn load() -> Result<Self> {
        let cache_path = Self::get_path()?;

        if !cache_path.exists() {
            return Ok(Self::default());
        }

        let bytes = fs::read(&cache_path).context("Failed to read the client's cache file from the disk.")?;
        Self::from_bytes(&bytes)
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

    /// Creates a token in the cache. The API keys of the token are not handled here: they belong in the OS keychain, managed
    /// by the [`keystore`](crate::api::client::keystore) module.
    pub fn create_token(&mut self, name: impl Into<String>, provider_id: impl Into<String>) -> Result<()> {
        let name = name.into();
        let token = Token::new(name.clone(), provider_id)?;

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
            bail!("There is no token named '{}' in the client's cache.", name);
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

        fs::write(&cache_path, self.to_bytes()?).context("Failed to write the client's cache file to the disk.")?;

        // Restricts the cache file to the current user.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&cache_path, fs::Permissions::from_mode(0o600))
                .context("Failed to restrict the permissions of the client's cache file.")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_cache() -> Cache {
        let mut cache = Cache::default();
        cache.create_token("test-token", "porkbun").unwrap();
        cache.add_hosts(["sub.example.com"], ["test-token"]).unwrap();
        cache
    }

    #[test]
    fn serialization_round_trips() {
        let cache = sample_cache();
        let restored = Cache::from_bytes(&cache.to_bytes().unwrap()).unwrap();

        let maps = restored.get_maps("test-token").unwrap();
        assert_eq!(maps.len(), 1);
        assert!(maps.contains_key("sub.example.com"));
    }

    #[test]
    fn unrecognized_headers_are_rejected() {
        assert!(Cache::from_bytes(b"not a wapi cache file").is_err());
        assert!(Cache::from_bytes(b"WAP").is_err());
    }

    #[test]
    fn unsupported_format_versions_are_rejected() {
        let mut bytes = sample_cache().to_bytes().unwrap();
        bytes[MAGIC.len()] = FORMAT_VERSION + 1;
        assert!(Cache::from_bytes(&bytes).is_err());
    }

    #[test]
    fn duplicate_tokens_are_rejected() {
        let mut cache = sample_cache();
        assert!(cache.create_token("test-token", "porkbun").is_err());
    }

    #[test]
    fn deleting_a_token_removes_it() {
        let mut cache = sample_cache();
        cache.delete_token("test-token").unwrap();
        assert!(cache.delete_token("test-token").is_err());
        assert!(cache.get_maps("test-token").is_none());
    }

    #[test]
    fn removing_hosts_updates_the_maps() {
        let mut cache = sample_cache();
        cache.remove_hosts(["sub.example.com"], ["test-token"]).unwrap();
        assert!(cache.get_maps("test-token").unwrap().is_empty());
    }

    #[test]
    fn unknown_tokens_fail_lookups() {
        assert!(sample_cache().get_tokens(["missing"]).is_err());
    }
}
