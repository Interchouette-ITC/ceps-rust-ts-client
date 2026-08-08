//! Session install / upgrade / companion session calls.

use super::CepCore;
use crate::error::{CepError, Result};
use crate::types::{CallResult, DeployParams};
use casper_rust_wasm_sdk::types::hash::transaction_hash::TransactionHash;
use casper_rust_wasm_sdk::types::transaction_params::transaction_builder_params::TransactionBuilderParams;

pub(super) async fn install_wasm(
    core: &CepCore,
    wasm: &[u8],
    deploy: &DeployParams,
    args_json: &str,
) -> Result<CallResult> {
    let params = core.build_tx_params(deploy, args_json);
    let bytes = CepCore::bytes_from_slice(wasm);
    let put = core
        .sdk()
        .install(
            params,
            bytes,
            Some(core.rpc_url().to_string()),
            core.runtime_v2(),
        )
        .await?;
    let tx_hash = TransactionHash::from(put.result.transaction_hash).to_string();
    let put_json = serde_json::to_value(&put.result)
        .map_err(|e| CepError::Other(format!("serialize put result: {e}")))?;
    let result = CallResult::new(tx_hash, put_json);
    core.maybe_wait(deploy, result).await
}

/// Run a session WASM that is not an install/upgrade (companion session).
pub(super) async fn call_session(
    core: &CepCore,
    wasm: &[u8],
    deploy: &DeployParams,
    args_json: &str,
) -> Result<CallResult> {
    let params = core.build_tx_params(deploy, args_json);
    let bytes = CepCore::bytes_from_slice(wasm);
    let builder = TransactionBuilderParams::new_session(Some(bytes), Some(false));
    let put = core
        .sdk()
        .call_entrypoint(
            builder,
            params,
            Some(core.rpc_url().to_string()),
            core.runtime_v2(),
        )
        .await?;
    let tx_hash = TransactionHash::from(put.result.transaction_hash).to_string();
    let put_json = serde_json::to_value(&put.result)
        .map_err(|e| CepError::Other(format!("serialize put result: {e}")))?;
    let result = CallResult::new(tx_hash, put_json);
    core.maybe_wait(deploy, result).await
}
