// Copyright 2025 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module contains the struct and methods used to manipulate the program's cache file.

use serde::{Deserialize, Serialize};

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
