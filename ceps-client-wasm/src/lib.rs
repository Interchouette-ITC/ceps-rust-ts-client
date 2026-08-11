//! Thin wasm-bindgen surface over `ceps-client`.

#![deny(missing_docs)]

#[cfg(feature = "client")]
mod client;

#[cfg(feature = "schema")]
mod schema_api;

#[cfg(feature = "client")]
pub use client::*;

#[cfg(feature = "schema")]
pub use schema_api::*;
