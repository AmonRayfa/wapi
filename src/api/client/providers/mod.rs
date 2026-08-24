// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

pub(crate) mod porkbun;

use mabe::{Result, bail};
use rkyv::{Archive, Deserialize, Serialize};
use std::fmt;

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
