// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! The request logic of the [Porkbun v3 API](https://porkbun.com/api/json/v3/documentation). All the endpoints are POST
//! requests carrying the API keys in the JSON payload.

use crate::api::client::cache::{AddrType, Host};
use crate::api::client::keystore::Credentials;
use mabe::{Context, Result, bail};
use reqwest::blocking::Client as ClientHandle;
use serde::Deserialize;
use serde_json::json;

const API_BASE: &str = "https://api.porkbun.com/api/json/v3";

#[derive(Debug, Deserialize)]
struct PorkbunRecord {
    content: String,
}

#[derive(Debug, Deserialize)]
struct PorkbunResponse {
    status: String,
    message: Option<String>,
    records: Option<Vec<PorkbunRecord>>,
}

/// Builds the endpoint of a `*ByNameType` action; the subdomain segment is omitted for root domains.
fn by_name_type_endpoint(action: &str, host: &Host, addr_type: AddrType) -> String {
    let endpoint = format!("{}/dns/{}/{}/{}", API_BASE, action, host.dom(), addr_type);
    if host.sub().is_empty() { endpoint } else { format!("{}/{}", endpoint, host.sub()) }
}

/// Sends a request to Porkbun and validates the status of its response.
fn send(client_handle: &ClientHandle, endpoint: String, payload: serde_json::Value, action: &str) -> Result<PorkbunResponse> {
    let response: PorkbunResponse = client_handle
        .post(endpoint)
        .json(&payload)
        .send()
        .context(format!("Failed to send the request to {} on Porkbun.", action))?
        .json()
        .context(format!("Failed to parse the response of the request to {} on Porkbun.", action))?;

    if response.status != "SUCCESS" {
        let err = format!("Failed to {} on Porkbun.", action);
        match response.message {
            Some(cause) => bail!("{}\n{}", err, cause),
            None => bail!("{}\nNo cause was provided.", err),
        };
    }

    Ok(response)
}

/// Returns the current content of the address record of a host (or [`None`] if the record doesn't exist).
pub(crate) fn get_address_record(
    client_handle: &ClientHandle,
    credentials: &Credentials,
    host: &Host,
    addr_type: AddrType,
) -> Result<Option<String>> {
    let endpoint = by_name_type_endpoint("retrieveByNameType", host, addr_type);
    let payload = json!({
        "apikey": credentials.api_key,
        "secretapikey": credentials.secret_api_key,
    });

    let action = format!("retrieve the '{}' record of host '{}'", addr_type, host);
    let response = send(client_handle, endpoint, payload, &action)?;

    let mut records = response.records.unwrap_or_default();
    match records.len() {
        0 => Ok(None),
        1 => Ok(Some(records.remove(0).content)),
        n => bail!("Porkbun returned {} '{}' records for host '{}', but at most 1 was expected.", n, addr_type, host),
    }
}

/// Creates the address record of a host with the provided IP address.
pub(crate) fn create_address_record(
    client_handle: &ClientHandle,
    credentials: &Credentials,
    host: &Host,
    addr_type: AddrType,
    ip: &str,
) -> Result<()> {
    let endpoint = format!("{}/dns/create/{}", API_BASE, host.dom());
    let payload = json!({
        "apikey": credentials.api_key,
        "secretapikey": credentials.secret_api_key,
        "name": host.sub(),
        "type": addr_type.as_str(),
        "content": ip,
    });

    let action = format!("create the '{}' record of host '{}'", addr_type, host);
    send(client_handle, endpoint, payload, &action).map(|_| ())
}

/// Updates the address record of a host with the provided IP address.
pub(crate) fn update_address_record(
    client_handle: &ClientHandle,
    credentials: &Credentials,
    host: &Host,
    addr_type: AddrType,
    ip: &str,
) -> Result<()> {
    let endpoint = by_name_type_endpoint("editByNameType", host, addr_type);
    let payload = json!({
        "apikey": credentials.api_key,
        "secretapikey": credentials.secret_api_key,
        "content": ip,
    });

    let action = format!("update the '{}' record of host '{}'", addr_type, host);
    send(client_handle, endpoint, payload, &action).map(|_| ())
}
