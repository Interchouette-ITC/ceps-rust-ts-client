//! Shared transport, install, call, query, and wait helpers.

mod args;
mod call;
mod install;
mod query;
mod urls;

pub use args::{
    bool_arg, json_arg, json_args, key_arg, key_list_arg, string_arg, u256_arg, u64_arg, u8_arg,
    JsonArg,
};
pub use urls::{normalize_rpc_url, normalize_sse_url};

use crate::error::{CepError, CepKind, Result};
use crate::types::{CallResult, ContractTarget, DeployParams};
use casper_rust_wasm_sdk::types::cl::bytes::Bytes;
use casper_rust_wasm_sdk::types::hash::addressable_entity_hash::AddressableEntityHash;
use casper_rust_wasm_sdk::types::hash::package_hash::PackageHash;
use casper_rust_wasm_sdk::types::hash::transaction_hash::TransactionHash;
use casper_rust_wasm_sdk::types::transaction_params::transaction_builder_params::TransactionBuilderParams;
use casper_rust_wasm_sdk::types::transaction_params::transaction_str_params::TransactionStrParams;
use casper_rust_wasm_sdk::types::verbosity::Verbosity;
use casper_rust_wasm_sdk::SDK;
use serde_json::Value;

/// Default NCTL RPC endpoint.
pub const DEFAULT_RPC_URL: &str = "http://127.0.0.1:11101";
/// Default NCTL SSE events endpoint.
pub const DEFAULT_SSE_URL: &str = "http://127.0.0.1:18101/events";
/// Default NCTL chain name.
pub const DEFAULT_CHAIN_NAME: &str = "casper-net-1";
/// Default wait timeout (matches JS clients): 120 seconds.
pub const DEFAULT_WAIT_TIMEOUT_MS: u64 = 120_000;

/// Shared CEP client core: endpoints, SDK handle, and contract targeting.
pub struct CepCore {
    sdk: SDK,
    rpc_url: String,
    sse_url: Option<String>,
    chain_name: String,
    verbosity: Verbosity,
    target: Option<ContractTarget>,
    /// Use VmCasperV2 session runtime when `Some(true)`.
    runtime_v2: Option<bool>,
    cep_kind: Option<CepKind>,
}

impl CepCore {
    /// Create a core client.
    ///
    /// `rpc_url` is required. Empty `sse_url` is treated as unset.
    pub fn new(
        rpc_url: impl Into<String>,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<Verbosity>,
    ) -> Result<Self> {
        let rpc_url = normalize_rpc_url(&rpc_url.into())?;
        let sse_url = match sse_url {
            Some(s) if !s.trim().is_empty() => Some(normalize_sse_url(&s)?),
            _ => None,
        };
        let verbosity = verbosity.unwrap_or(Verbosity::Low);
        let sdk = SDK::new(Some(rpc_url.clone()), None, Some(verbosity));
        Ok(Self {
            sdk,
            rpc_url,
            sse_url,
            chain_name: chain_name.unwrap_or_else(|| DEFAULT_CHAIN_NAME.to_string()),
            verbosity,
            target: None,
            runtime_v2: Some(false),
            cep_kind: None,
        })
    }

    /// Tag this core with a CEP kind (used when mapping user errors).
    pub fn with_cep_kind(mut self, kind: CepKind) -> Self {
        self.cep_kind = Some(kind);
        self
    }

    /// Set session runtime to VmCasperV2 (`true`) or VmCasperV1 (`false`).
    pub fn set_runtime_v2(&mut self, runtime_v2: bool) {
        self.runtime_v2 = Some(runtime_v2);
    }

    /// Borrow the underlying SDK.
    pub fn sdk(&self) -> &SDK {
        &self.sdk
    }

    /// RPC URL (always ends with `/rpc`).
    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }

    /// SSE URL when configured.
    pub fn sse_url(&self) -> Option<&str> {
        self.sse_url.as_deref()
    }

    /// Chain name used for transactions.
    pub fn chain_name(&self) -> &str {
        &self.chain_name
    }

    /// Current verbosity.
    pub fn verbosity(&self) -> Verbosity {
        self.verbosity
    }

    /// Bound contract target, if any.
    pub fn target(&self) -> Option<&ContractTarget> {
        self.target.as_ref()
    }

    /// Update RPC URL.
    pub fn set_rpc_url(&mut self, rpc_url: impl Into<String>) -> Result<()> {
        let rpc_url = normalize_rpc_url(&rpc_url.into())?;
        self.sdk
            .set_rpc_address(Some(rpc_url.clone()))
            .map_err(CepError::Other)?;
        self.rpc_url = rpc_url;
        Ok(())
    }

    /// Update SSE URL.
    pub fn set_sse_url(&mut self, sse_url: impl Into<String>) -> Result<()> {
        self.sse_url = Some(normalize_sse_url(&sse_url.into())?);
        Ok(())
    }

    /// Update chain name.
    pub fn set_chain_name(&mut self, chain_name: impl Into<String>) {
        self.chain_name = chain_name.into();
    }

    /// Update verbosity on the core and SDK handle.
    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.verbosity = verbosity;
        let _ = self.sdk.set_verbosity(Some(verbosity));
    }

    /// Bind contract (and optional package) hashes.
    pub fn set_contract_hash(
        &mut self,
        contract_hash: impl AsRef<str>,
        package_hash: Option<impl AsRef<str>>,
    ) -> Result<()> {
        self.target = Some(ContractTarget::new(contract_hash, package_hash)?);
        Ok(())
    }

    /// Require a bound contract target.
    pub fn require_target(&self) -> Result<&ContractTarget> {
        self.target.as_ref().ok_or(CepError::ContractHashMissing)
    }

    /// Build [`TransactionStrParams`] from deploy options and JSON session args.
    pub fn build_tx_params(&self, deploy: &DeployParams, args_json: &str) -> TransactionStrParams {
        let params = TransactionStrParams::default();
        let chain = deploy
            .chain_name
            .as_deref()
            .unwrap_or(self.chain_name.as_str());
        params.set_chain_name(chain);
        params.set_secret_key(&deploy.secret_key_pem);
        params.set_payment_amount(&deploy.payment_amount);
        if !args_json.is_empty() {
            params.set_session_args_json(args_json);
        }
        params
    }

    /// Install a session WASM (install or upgrade path).
    pub async fn install_wasm(
        &self,
        wasm: &[u8],
        deploy: &DeployParams,
        args_json: &str,
    ) -> Result<CallResult> {
        install::install_wasm(self, wasm, deploy, args_json).await
    }

    /// Call a contract entrypoint by package hash (preferred) or entity hash.
    pub async fn call_entrypoint(
        &self,
        entry_point: &str,
        deploy: &DeployParams,
        args_json: &str,
    ) -> Result<CallResult> {
        call::call_entrypoint(self, entry_point, deploy, args_json).await
    }

    /// Call a companion session WASM (CEP-78 session helpers).
    pub async fn call_session(
        &self,
        wasm: &[u8],
        deploy: &DeployParams,
        args_json: &str,
    ) -> Result<CallResult> {
        install::call_session(self, wasm, deploy, args_json).await
    }

    /// Query a named key under the bound contract (`hash-{contract}`).
    pub async fn query_contract_key(&self, path: &[&str]) -> Result<Value> {
        query::query_contract_key(self, path).await
    }

    /// Query a dictionary item under a named dictionary on the bound contract.
    pub async fn query_dictionary(&self, dictionary_name: &str, item_key: &str) -> Result<Value> {
        query::query_dictionary(self, dictionary_name, item_key).await
    }

    /// Read a named key from an account (by public key hex or account-hash-…).
    pub async fn get_account_named_key(
        &self,
        account_identifier: &str,
        named_key: &str,
    ) -> Result<String> {
        query::get_account_named_key(self, account_identifier, named_key).await
    }

    /// Wait for a transaction hash on the configured SSE endpoint.
    pub async fn wait_transaction(
        &self,
        transaction_hash: &str,
        timeout_ms: Option<u64>,
    ) -> Result<Value> {
        let sse = self
            .sse_url
            .as_deref()
            .ok_or_else(|| CepError::WaitFailed("SSE URL is not configured".into()))?;
        let timeout = timeout_ms.or(Some(DEFAULT_WAIT_TIMEOUT_MS));
        let event = self
            .sdk
            .wait_transaction(sse, transaction_hash, timeout)
            .await
            .map_err(CepError::WaitFailed)?;
        serde_json::to_value(event).map_err(|e| CepError::Other(e.to_string()))
    }

    pub(crate) fn builder_for_entrypoint(
        &self,
        entry_point: &str,
    ) -> Result<TransactionBuilderParams> {
        let target = self.require_target()?;
        if let Some(package_hex) = &target.package_hash {
            let package = PackageHash::new(package_hex)
                .map_err(|e| CepError::InvalidHash(format!("package hash: {e}")))?;
            Ok(TransactionBuilderParams::new_package(
                package,
                entry_point,
                None,
            ))
        } else {
            let entity = AddressableEntityHash::new(&target.contract_hash)
                .map_err(|e| CepError::InvalidHash(format!("contract hash: {e}")))?;
            Ok(TransactionBuilderParams::new_invocable_entity(
                entity,
                entry_point,
            ))
        }
    }

    pub(crate) fn bytes_from_slice(wasm: &[u8]) -> Bytes {
        Bytes::from(wasm.to_vec())
    }

    pub(crate) async fn maybe_wait(
        &self,
        deploy: &DeployParams,
        mut result: CallResult,
    ) -> Result<CallResult> {
        if !deploy.wait {
            return Ok(result);
        }
        let _event = self
            .wait_transaction(&result.transaction_hash, deploy.wait_timeout_ms)
            .await?;
        let tx_hash = TransactionHash::new(&result.transaction_hash)
            .map_err(|e| CepError::InvalidHash(format!("transaction hash: {e}")))?;
        if let Ok(get_tx) = self
            .sdk
            .get_transaction(
                tx_hash,
                Some(false),
                Some(self.verbosity),
                Some(self.rpc_url.clone()),
            )
            .await
        {
            if let Ok(json) = serde_json::to_value(&get_tx.result) {
                if let Some(err) = extract_execution_error(&json) {
                    return Err(CepError::from_execution_message(err, self.cep_kind));
                }
                result = result.with_execution(json);
            }
        }
        Ok(result)
    }

    pub(crate) fn runtime_v2(&self) -> Option<bool> {
        self.runtime_v2
    }
}

fn extract_execution_error(json: &Value) -> Option<String> {
    // Walk common shapes for execution failure messages.
    let candidates = [
        json.pointer("/execution_info/execution_result/Failure/error_message"),
        json.pointer("/execution_result/Failure/error_message"),
        json.pointer("/execution_info/execution_result/error_message"),
    ];
    for c in candidates.into_iter().flatten() {
        if let Some(s) = c.as_str() {
            return Some(s.to_string());
        }
    }
    // String-search in serialized JSON as a last resort.
    let text = json.to_string();
    if text.contains("User error:") || text.contains("\"Failure\"") {
        if let Some(idx) = text.find("User error:") {
            let slice: String = text[idx..].chars().take(64).collect();
            return Some(slice);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_new_normalizes_urls() {
        let core = CepCore::new("http://127.0.0.1:11101", None, None, None).unwrap();
        assert_eq!(core.rpc_url(), "http://127.0.0.1:11101/rpc");
        assert!(core.sse_url().is_none());
    }

    #[test]
    fn core_rejects_empty_rpc() {
        assert!(CepCore::new("", None, None, None).is_err());
    }
}
