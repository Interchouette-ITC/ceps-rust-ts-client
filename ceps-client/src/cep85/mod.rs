//! CEP-85 multi-token client (facade shell; full parity in later work).

mod entity;
mod types;

pub use entity::prefixed_key;
pub use types::InstallArgs;

use crate::core::CepCore;
use crate::error::{CepKind, Result};
use casper_rust_wasm_sdk::types::verbosity::Verbosity;

/// Client for CEP-85 multi-token contracts.
pub struct Cep85Client {
    core: CepCore,
}

impl Cep85Client {
    /// Create a CEP-85 client.
    pub fn new(
        rpc_url: impl Into<String>,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<Verbosity>,
    ) -> Result<Self> {
        let core =
            CepCore::new(rpc_url, sse_url, chain_name, verbosity)?.with_cep_kind(CepKind::Cep85);
        Ok(Self { core })
    }

    /// Borrow the shared core.
    pub fn core(&self) -> &CepCore {
        &self.core
    }

    /// Mutable core access.
    pub fn core_mut(&mut self) -> &mut CepCore {
        &mut self.core
    }

    /// RPC URL.
    pub fn rpc_url(&self) -> &str {
        self.core.rpc_url()
    }

    /// SSE URL when set.
    pub fn sse_url(&self) -> Option<&str> {
        self.core.sse_url()
    }

    /// Chain name.
    pub fn chain_name(&self) -> &str {
        self.core.chain_name()
    }

    /// Verbosity.
    pub fn verbosity(&self) -> Verbosity {
        self.core.verbosity()
    }

    /// Set RPC URL.
    pub fn set_rpc_url(&mut self, rpc_url: impl Into<String>) -> Result<()> {
        self.core.set_rpc_url(rpc_url)
    }

    /// Set SSE URL.
    pub fn set_sse_url(&mut self, sse_url: impl Into<String>) -> Result<()> {
        self.core.set_sse_url(sse_url)
    }

    /// Set chain name.
    pub fn set_chain_name(&mut self, chain_name: impl Into<String>) {
        self.core.set_chain_name(chain_name);
    }

    /// Set verbosity.
    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.core.set_verbosity(verbosity);
    }

    /// Bind contract and optional package hash.
    pub fn set_contract_hash(
        &mut self,
        contract_hash: impl AsRef<str>,
        package_hash: Option<impl AsRef<str>>,
    ) -> Result<()> {
        self.core.set_contract_hash(contract_hash, package_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_client() {
        let client =
            Cep85Client::new("http://127.0.0.1:11101", None, None, Some(Verbosity::Low)).unwrap();
        assert_eq!(client.rpc_url(), "http://127.0.0.1:11101/rpc");
    }

    #[test]
    fn entity_prefix_for_account() {
        let key = prefixed_key(
            "account-hash-b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f",
        )
        .unwrap();
        assert!(key.starts_with("entity-account-"));
    }
}
