// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe::{Result, bail};

pub(crate) fn collect_mappings(targets: Vec<String>) -> Result<Vec<(Vec<String>, Option<Vec<String>>)>> {
    if targets.is_empty() {
        bail!("No hostnames were provided.");
    }
    if targets[0].strip_prefix('@').is_some() {
        bail!("The first argument must be a hostname, not a token name.");
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
            // Moves the full vectors out and replacse them with empty ones.
            map.push((std::mem::take(&mut hostnames), Some(std::mem::take(&mut token_names))));
        }
    }

    // Checks for trailing hostnames. Those will be added globally (i.e., to all tokens).
    if !hostnames.is_empty() {
        map.push((hostnames, None))
    }

    Ok(map)
}
