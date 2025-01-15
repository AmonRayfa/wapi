// Copyright 2025 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! [**Wapi**](https://github.com/AmonRayfa/wapi) is a cross-platform command-line DDNS (Dynamic Domain Name System) client that
//! keeps your DNS records up to date by automatically adjusting them whenever your public IP address changes. This is
//! especially useful for users running services on home or private networks with dynamic IP addresses, ensuring their domain
//! names always resolve to the correct IP address.
//!
//! The client supports a wide range of domain registrars and DNS service providers, making it a versatile solution for managing
//! your DNS records. It provides a user-friendly command-line, perfect for workflows involving external scripts or automation
//! tools, as well as a flexible Rust library for developers who want to integrate the client into their own applications.
//!
//! # Examples
//!
//! -> This section will be updated soon.
//!
//! # Cargo Features
//!
//! This crate has no public
//! [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/features.html#the-features-section).

mod api;
mod error;

#[cfg(debug_assertions)]
mod utils;
