// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use crate::api::client::cache::{AddrData, AddrType, Host, Token};
use mabe::{Result, bail};
use reqwest::blocking::Client as ClientHandle;
use serde::Deserialize;
use serde_json::json;

/*
#[derive(Debug, Deserialize)]
struct PorkbunRecord {
    content: String,
    ttl: String,
}
*/

#[derive(Debug, Deserialize)]
struct PorkbunResponse {
    status: String,
    message: Option<String>,
    //records: Option<Vec<PorkbunRecord>>,
}

/*
pub(crate) fn get_address_records(
    client_handle: &ClientHandle,
    token: &Token,
    host: &Host,
    addr_data: &mut AddrData,
    addr_type: AddrType,
) -> Result<()> {
    let endpoint =
        format!("https://api.porkbun.com/api/json/v3/dns/retrieveByNameType/{}/{}/{}", host.dom, addr_type, host.sub);

    let payload = json!({
        "apikey": token.api_key,
        "secretapikey": token.secret_api_key,
    });

    let response: PorkbunResponse = client_handle.get(endpoint).json(&payload).send()?.json()?;

    if response.status == "SUCCESS" {
        match response.records {
            Some(recs) => {
                if recs.is_empty() {
                    bail!(
                        "No records were provided in the record list of the request response when trying to retrieve the content of the '{}' record for hostname '{}' on Porkbun.",
                        addr_type,
                        host
                    )
                }

                if recs.len() > 1 {
                    bail!(
                        "The record list of the request response of Prokbun contains more than 1 '{}' record for hostname '{}'.",
                        addr_type,
                        host
                    )
                }

                addr_data.update_ip(addr_type, recs[0].content.clone()).context(format!("The IP address '{}' returned by Porkbun for the '{}' record of the '{}' domain is invalid.", recs[0].ttl, addr_type, domain_name))?;
                addr_data.update_ttl(recs[0].ttl.clone()).context(format!("The TTL value '{}' returned by Porkbun for the '{}' record of the '{}' domain could not be parsed to a 'u64' type.", recs[0].ttl, addr_type, domain_name))?;
            }
            None => bail!(
                "No record list was provided in the request response when trying to retrieve the content of the '{}' record for domain name '{}' on Porkbun.",
                addr_type,
                addr_data
            ),
        }
    } else {
        let err = format!("Failed to retrieve the data in the '{}' record for domain name '{}' on Porkbun.", addr_type, domain_name);
        match response.message {
            Some(cause) => bail!("{}\nCause by: {}", err, cause),
            None => bail!("{} No cause was provided.", err),
        }
    }

    Ok(())
}
*/

pub(crate) fn update_address_record(
    client_handle: &ClientHandle,
    token: &Token,
    host: &Host,
    addr_type: &AddrType,
    addr_data: &AddrData,
) -> Result<()> {
    let endpoint =
        format!("https://api.porkbun.com/api/json/v3/dns/editByNameType/{}/{}/{}", host.dom(), addr_type, host.sub());

    let payload = json!({
        "apikey": token.api_key,
        "secretapikey": token.secret_api_key,
        "content": match addr_type {
            AddrType::A => addr_data.ipv4(),
            AddrType::AAAA => addr_data.ipv6(),
        },
        "ttl": null,
        "notes": null,
    });

    let response: PorkbunResponse = client_handle.post(endpoint).json(&payload).send()?.json()?;

    if response.status == "ERROR" {
        let err = format!("Failed to update the '{}' record of host '{}' on Porkbun.", addr_type, host);
        match response.message {
            Some(cause) => bail!("{}\n{}", err, cause),
            None => bail!("{}\nNo cause was provided.", err),
        };
    }

    Ok(())
}
