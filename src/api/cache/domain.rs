use addr::parse_domain_name;
use mabe::{Context, Result, bail};
use rkyv::{Archive, Deserialize, Serialize};
use std::fmt;

#[derive(Archive, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Domain {
    pub(crate) root: String,
    pub(crate) host: String,
}

impl Domain {
    pub(crate) fn from<D: ToString>(domain: D) -> Result<Domain> {
        let domain: String = domain.to_string().to_lowercase();
        match parse_domain_name(domain.as_str()) {
            Ok(d) => {
                // Extracts the root domain.
                let root = d
                    .root()
                    .context(format!("This domain is invalid: '{}'. A domain must at least contain a TLD and an SLD.", domain))?
                    .to_string();

                // Validates the TLD.
                if !d.has_known_suffix() {
                    bail!("Invalid domain name: '{}'. Unknown TLD.", domain);
                }

                // Extracts the subdomains if there are any.
                let sub = match d.prefix() {
                    Some(s) => s.to_string(),
                    None => String::new(),
                };

                Ok(Domain { root: root, host: sub })
            }
            Err(e) => bail!("{}", e), // Fails RFC syntax validation.
        }
    }
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.host, self.root)
    }
}

pub(crate) trait DomainJoinExt {
    fn join(&self, separator: &str) -> String;
}

impl DomainJoinExt for Vec<Domain> {
    fn join(&self, separator: &str) -> String {
        let domains: Vec<String> = self.iter().map(|d| d.to_string()).collect();
        domains.join(separator)
    }
}
