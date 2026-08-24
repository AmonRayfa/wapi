// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module stores the API keys of the tokens in the operating system's keychain (i.e., the macOS Keychain, the Windows
//! Credential Manager, or the Linux secret service), so that they never touch the disk in plain text.

use keyring::Entry;
use mabe::{Context, Result};
use serde::{Deserialize, Serialize};

/// The service name under which all the client's credentials are registered in the OS keychain.
const KEYCHAIN_SERVICE: &str = "wapi";

/// The API keys of a token, as stored in the OS keychain.
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Credentials {
    pub(crate) api_key: String,
    pub(crate) secret_api_key: Option<String>,
}

/// Returns the keychain entry of a token.
fn entry(token_name: &str) -> Result<Entry> {
    Entry::new(KEYCHAIN_SERVICE, token_name)
        .context(format!("Failed to access the OS keychain entry of token '{}'.", token_name))
}

/// Stores the API keys of a token in the OS keychain.
pub(crate) fn store(token_name: &str, credentials: &Credentials) -> Result<()> {
    let payload = serde_json::to_string(credentials).context("Failed to serialize the API keys.")?;
    entry(token_name)?
        .set_password(&payload)
        .context(format!("Failed to store the API keys of token '{}' in the OS keychain.", token_name))
}

/// Retrieves the API keys of a token from the OS keychain.
pub(crate) fn load(token_name: &str) -> Result<Credentials> {
    let payload = entry(token_name)?.get_password().context(format!(
        "Failed to retrieve the API keys of token '{}' from the OS keychain. If the token was created by an older version of the client, delete it and create it again.",
        token_name
    ))?;
    serde_json::from_str(&payload).context(format!("Failed to parse the API keys of token '{}'.", token_name))
}

/// Removes the API keys of a token from the OS keychain. Missing entries are not treated as errors, so that a token whose
/// keys were already removed can still be deleted from the cache.
pub(crate) fn delete(token_name: &str) -> Result<()> {
    match entry(token_name)?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e).context(format!("Failed to remove the API keys of token '{}' from the OS keychain.", token_name)),
    }
}
