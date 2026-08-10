//! Unified Casper CEP client library (CEP-18, CEP-78, CEP-85, CEP-95).
//!
//! Builds on [`casper_rust_wasm_sdk`]. CEP-specific enums, dictionary-key
//! helpers, and contract error maps live here; RPC, transactions, and wait
//! helpers stay in the SDK.

#![deny(missing_docs)]

pub mod cep18;
pub mod cep78;
pub mod cep85;
pub mod cep95;
pub mod core;
pub mod error;
pub mod types;

pub use casper_rust_wasm_sdk::types::verbosity::Verbosity;
pub use casper_rust_wasm_sdk::SSE::{
    CESEvent, CESParseResult, CESParser, EventName, RawEvent, SSEClient,
};
pub use cep18::CEP18Client;
pub use cep78::CEP78Client;
pub use cep85::CEP85Client;
pub use cep95::CEP95Client;
pub use core::CEPClient;
pub use error::{CEPError, Result};
pub use types::{CallResult, EventsMode, EventsMode78, TransactionParams};
