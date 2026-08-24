// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe::{Result, bail};

/// The hostname-to-token mappings extracted from a list of CLI targets: each entry associates a group of hostnames with a group
/// of token names, where [`None`] means "all the tokens in the cache".
pub(crate) type Mappings = Vec<(Vec<String>, Option<Vec<String>>)>;

pub(crate) fn collect_mappings(targets: Vec<String>) -> Result<Mappings> {
    match targets.first() {
        None => bail!("No hostnames were provided."),
        Some(first) if first.starts_with('@') => bail!("The first argument must be a hostname, not a token name."),
        _ => {}
    }

    let mut hostnames = Vec::new();
    let mut token_names = Vec::new();
    let mut map = Vec::new();

    for arg in targets {
        // Checks if the argument is a token name.
        if let Some(name) = arg.strip_prefix('@') {
            token_names.push(name.to_string());
        } else if token_names.is_empty() {
            hostnames.push(arg);
        } else {
            // A hostname right after a token group closes the group and starts a new one.
            map.push((std::mem::take(&mut hostnames), Some(std::mem::take(&mut token_names))));
            hostnames.push(arg);
        }
    }

    // Flushes the last group. Trailing hostnames without tokens will be added globally (i.e., to all tokens).
    if !token_names.is_empty() {
        map.push((hostnames, Some(token_names)));
    } else if !hostnames.is_empty() {
        map.push((hostnames, None));
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn global_hostnames_map_to_all_tokens() {
        let map = collect_mappings(args(&["a.com", "b.org"])).unwrap();
        assert_eq!(map, vec![(args(&["a.com", "b.org"]), None)]);
    }

    #[test]
    fn hostnames_map_to_their_token_group() {
        let map = collect_mappings(args(&["a.com", "b.org", "@t1", "@t2"])).unwrap();
        assert_eq!(map, vec![(args(&["a.com", "b.org"]), Some(args(&["t1", "t2"])))]);
    }

    #[test]
    fn hostname_after_token_group_starts_a_new_group() {
        let map = collect_mappings(args(&["a.com", "@t1", "b.org", "@t2"])).unwrap();
        assert_eq!(map, vec![(args(&["a.com"]), Some(args(&["t1"]))), (args(&["b.org"]), Some(args(&["t2"]))),]);
    }

    #[test]
    fn trailing_hostnames_map_to_all_tokens() {
        let map = collect_mappings(args(&["a.com", "@t1", "b.org", "c.net"])).unwrap();
        assert_eq!(map, vec![(args(&["a.com"]), Some(args(&["t1"]))), (args(&["b.org", "c.net"]), None)]);
    }

    #[test]
    fn leading_token_is_rejected() {
        assert!(collect_mappings(args(&["@t1", "a.com"])).is_err());
    }

    #[test]
    fn empty_targets_are_rejected() {
        assert!(collect_mappings(Vec::new()).is_err());
    }
}
