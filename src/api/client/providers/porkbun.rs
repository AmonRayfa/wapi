use super::AddressRecordData;
use crate::api::{Domain, TokenData};
use mabe::{Context, Result, bail};
use reqwest::blocking::Client as ClientHandle;
use serde::Deserialize;
use serde_json::json;

pub const PATH: &str = module_path!();

#[derive(Debug, Deserialize)]
struct PorkbunRecord {
    content: String,
    ttl: String,
}

#[derive(Debug, Deserialize)]
struct PorkbunResponse {
    status: String,
    message: Option<String>,
    records: Option<Vec<PorkbunRecord>>,
}

pub(crate) fn get_address_records(
    client_handle: &ClientHandle,
    token: &Token,
    domain: &Domain,
    rec_type: &str,
    rec_data: &mut AddressRecordData,
) -> Result<()> {
    let endpoint =
        format!("https://api.porkbun.com/api/json/v3/dns/retrieveByNameType/{}/{}/{}", domain.root, rec_type, domain.host);

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
                        "No records were provided in the record list of the request response when trying to retrieve the content of the '{}' record for domain name '{}' on Porkbun.",
                        rec_type,
                        domain
                    )
                }

                if recs.len() > 1 {
                    bail!(
                        "The record list of the request response of Prokbun contains more than 1 '{}' record for domain name '{}'.",
                        rec_type,
                        domain
                    )
                }

                rec_data.ip = recs[0].content.clone();
                rec_data.ttl = recs[0].ttl.parse::<u64>().context(format!("The TTL value '{}' returned by Porkbun for the '{}' record of the '{}' domain could not be parsed to a 'u64' type.", recs[0].ttl, rec_type, domain))?;
            }
            None => bail!(
                "No record list was provided in the request response when trying to retrieve the content of the '{}' record for domain name '{}' on Porkbun.",
                rec_type,
                domain
            ),
        }
    } else {
        let err = format!("Failed to retrieve the data in the '{}' record for domain name '{}' on Porkbun.", rec_type, domain);
        match response.message {
            Some(cause) => bail!("{}\nCause by: {}", err, cause),
            None => bail!("{} No cause was provided.", err),
        }
    }

    Ok(())
}

pub(crate) fn update_address_records(
    client_handle: &ClientHandle,
    token: &Token,
    domain: &Domain,
    rec_type: &str,
    rec_data: &AddressRecordData,
) -> Result<()> {
    let endpoint =
        format!("https://api.porkbun.com/api/json/v3/dns/editByNameType/{}/{}/{}", domain.root, rec_type, domain.host);

    let payload = json!({
        "apikey": token.api_key,
        "secretapikey": token.secret_api_key,
        "content": rec_data.ip,
        "ttl": rec_data.ttl.to_string(),
        "notes": null,
    });

    let response: PorkbunResponse = client_handle.post(endpoint).json(&payload).send()?.json()?;

    if response.status == "ERROR" {
        match response.message {
            Some(msg) => bail!("Failed to update the '{}' record of domain '{}' on Porkbun.\n{}", rec_type, domain, msg),
            None => {
                bail!("Failed to update the '{}' record of domain '{}' on Porkbun.\nNo cause was provided.", rec_type, domain)
            }
        };
    }

    Ok(())
}
