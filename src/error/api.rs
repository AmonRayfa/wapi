// Copyright 2025 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module contains the custom `Error` and `Result` types for the `api` module.

use mabe::Mabe;

/// The custom `Error` type for the `api` module.
#[derive(Mabe)]
#[non_exhaustive]
pub enum Error {
    #[error("Cache manipulation failed: enable to {0} the cache.")]
    #[debug("{1}")]
    Cache(String, String),
}

/// The custom `Result` type for the `api` module.
pub type Result<T> = std::result::Result<T, Error>;
