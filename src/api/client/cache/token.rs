// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use super::Provider;
use mabe::{Result, bail};
use rkyv::{Archive, Deserialize, Serialize};
use std::borrow::Borrow;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Archive, Clone, Debug, Default, Deserialize, Serialize)]
pub struct Token {
    pub(crate) name: String,
    pub(crate) provider: Provider,
    pub(crate) api_key: String,
    pub(crate) secret_api_key: Option<String>,
}

impl Token {
    pub(crate) fn new(name: impl Into<String>, provider: impl Into<String>, api_key: impl Into<String>) -> Result<Self> {
        let name = name.into();
        if name.is_empty() || !name.as_str().chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            bail!(
                "Invalid token name: '{}'.\nToken names must only contain alphanumeric characters, hyphens, and underscores.",
                name
            );
        } else {
            Ok(Self { name, provider: Provider::new(provider)?, api_key: api_key.into(), secret_api_key: None })
        }
    }

    pub(crate) fn with_secret(mut self, secret: impl Into<String>) -> Self {
        self.secret_api_key = Some(secret.into());
        self
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.name
    }
}

/// Only the `name` field is used when displaying a [`Token`].
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// [`Token`] instances are considered equal only if they have the same `name` field.
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl PartialEq for rkyv::Archived<Token> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Token {}
impl Eq for rkyv::Archived<Token> {}

/// Only the `name` field is used when hashing a [`Token`].
impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl Hash for rkyv::Archived<Token> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

/// Allows token names to be passed as strings for lookups.
impl Borrow<str> for Token {
    fn borrow(&self) -> &str {
        &self.name
    }
}
