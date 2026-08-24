// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

pub(crate) mod porkbun;

use super::cache::{AddrType, Host};
use super::keystore::Credentials;
use mabe::{Result, bail};
use reqwest::blocking::Client as ClientHandle;
use rkyv::{Archive, Deserialize, Serialize};
use std::fmt;

type GetRecordFn = fn(&ClientHandle, &Credentials, &Host, AddrType) -> Result<Option<String>>;
type WriteRecordFn = fn(&ClientHandle, &Credentials, &Host, AddrType, &str) -> Result<()>;

/// The request functions a DNS service provider module must expose to support address record synchronization.
pub(crate) struct ProviderApi {
    /// Returns the current content of the address record of a host (or [`None`] if the record doesn't exist).
    pub(crate) get: GetRecordFn,
    /// Creates the address record of a host with the provided IP address.
    pub(crate) create: WriteRecordFn,
    /// Updates the address record of a host with the provided IP address.
    pub(crate) update: WriteRecordFn,
}

/// Returns the request functions of a DNS service provider (or [`None`] if the provider is not supported yet).
pub(crate) fn api_for(provider_id: &str) -> Option<ProviderApi> {
    match provider_id {
        "porkbun" => Some(ProviderApi {
            get: porkbun::get_address_record,
            create: porkbun::create_address_record,
            update: porkbun::update_address_record,
        }),
        _ => None,
    }
}

macro_rules! provider {
    ($mod_name:ident, $url:literal) => {
        (stringify!($mod_name), $url)
    };
}

/// A table containing the supported DNS service providers; the first column is the provider's ID, and the second one is the its URL.
/// The URLs are stored here instead of inside the [`Provider`] struct so that they don't take up space in the client's cache.
pub const PROVIDERS: &[(&str, &str)] = &[
    //provider!(alibabacloud, "https://www.alibabacloud.com"),
    //provider!(bluehost, "https://www.bluehost.com"),
    //provider!(cloudflare, "https://www.cloudflare.com"),
    //provider!(dnspod, "https://www.dnspod.com"),
    //provider!(dreamhost, "https://www.dreamhost.com"),
    //provider!(dynadot, "https://www.dynadot.com"),
    //provider!(enom, "https://www.enom.com"),
    //provider!(epik, "https://www.epik.com"),
    //provider!(gandi, "https://www.gandi.net"),
    //provider!(godaddy, "https://www.godaddy.com"),
    //provider!(hover, "https://www.hover.com"),
    //provider!(ionos, "https://www.ionos.com"),
    //provider!(namecheap, "https://www.namecheap.com"),
    //provider!(namesilo, "https://www.namesilo.com"),
    //provider!(opensrs, "https://opensrs.com"),
    //provider!(ovh, "https://www.ovhcloud.com"),
    provider!(porkbun, "https://porkbun.com"),
    //provider!(resellerclub, url: "https://www.resellerclub.com"),
];

#[derive(Archive, Clone, Debug, Default, Deserialize, Serialize)]
pub struct Provider(String);

impl Provider {
    pub fn new(id: impl Into<String>) -> Result<Self> {
        let provider_id = id.into();
        if PROVIDERS.iter().any(|(id, _)| *id == provider_id) {
            Ok(Self(provider_id))
        } else {
            bail!(
                "Provider ID '{}' is unknown.\nRun 'wapi providers' to get a list of all the supported DNS service providers and their IDs.",
                provider_id
            )
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
