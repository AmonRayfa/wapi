use addr::parse_domain_name;
use mabe::{Context, Result, bail};
use rkyv::{Archive, Deserialize, Serialize};
use std::borrow::Borrow;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Archive, Clone, Debug, Default, Deserialize, Serialize)]
#[rkyv(attr(derive(Eq, Hash, PartialEq)))]
pub(crate) struct Domain {
    pub(crate) name: String,
    pub(crate) root: String,
    pub(crate) host: String,
    pub(crate) ipv4: String,
    pub(crate) ipv6: String,
    pub(crate) ttl: u64,
}

impl Domain {
    pub(crate) fn from<S: ToString>(domain_name: S) -> Result<Domain> {
        let domain_name: String = domain_name.to_string().to_lowercase();
        match parse_domain_name(domain_name.as_str()) {
            Ok(d) => {
                // Extracts the root domain.
                let root = d
                    .root()
                    .context(format!(
                        "This domain name is invalid: '{}'. A domain name must at least contain a TLD and an SLD.",
                        domain_name
                    ))?
                    .to_string();

                // Validates the TLD.
                if !d.has_known_suffix() {
                    bail!("Invalid domain name: '{}'. Unknown TLD.", domain_name);
                }

                // Extracts the host name if there are any.
                let host = match d.prefix() {
                    Some(s) => s.to_string(),
                    None => String::new(),
                };

                Ok(Domain { name: domain_name, root: root, host: host, ipv4: String::new(), ipv6: String::new(), ttl: 0 })
            }
            Err(e) => bail!("{}", e), // Fails RFC syntax validation.
        }
    }
}

// Only the `name` field is used when displaying a `Domain` instance.
impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// Two `Domain` instances are equal only if their `name` fields match (the others are ignored).
impl PartialEq for Domain {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Domain {}

// Only hashes the `name` field.
impl Hash for Domain {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

// Allows the struct to present itself as a `&str` for lookups.
impl Borrow<str> for Domain {
    fn borrow(&self) -> &str {
        &self.name
    }
}

/// Implements the `join()` method on `Domain`.
pub(crate) trait DomainJoinExt {
    fn join(&self, separator: &str) -> String;
}

impl DomainJoinExt for Vec<Domain> {
    fn join(&self, separator: &str) -> String {
        let domain_names: Vec<String> = self.iter().map(|d| d.to_string()).collect();
        domain_names.join(separator)
    }
}
