//! Unified Casper CEP client library (CEP-18, CEP-78, CEP-85, CEP-95).
//!
//! Builds on [`casper_rust_wasm_sdk`] when the `client` feature is enabled.
//! The `schema` feature exports declarative install / entrypoint argument
//! metadata without pulling the RPC client.

#![deny(missing_docs)]

#[cfg(feature = "schema")]
pub mod schema;

#[cfg(feature = "client")]
pub mod cep18;
#[cfg(feature = "client")]
pub mod cep78;
#[cfg(feature = "client")]
pub mod cep85;
#[cfg(feature = "client")]
pub mod cep95;
#[cfg(feature = "client")]
pub mod core;
#[cfg(feature = "client")]
pub mod error;
#[cfg(feature = "client")]
pub mod types;

#[cfg(feature = "client")]
pub use casper_rust_wasm_sdk::types::verbosity::Verbosity;
#[cfg(feature = "client")]
pub use casper_rust_wasm_sdk::SSE::{
    CESEvent, CESParseResult, CESParser, EventName, RawEvent, SSEClient,
};
#[cfg(feature = "client")]
pub use cep18::CEP18Client;
#[cfg(feature = "client")]
pub use cep78::CEP78Client;
#[cfg(feature = "client")]
pub use cep85::CEP85Client;
#[cfg(feature = "client")]
pub use cep95::CEP95Client;
#[cfg(feature = "client")]
pub use core::CEPClient;
#[cfg(feature = "client")]
pub use error::{CEPError, Result};
#[cfg(feature = "client")]
pub use types::{CallResult, EventsMode, EventsMode78, TransactionParams};
