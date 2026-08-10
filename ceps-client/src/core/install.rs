//! Session install / upgrade / companion session calls.

use super::CEPClient;
use crate::error::{CEPError, Result};
use crate::types::{CallResult, TransactionParams};
use casper_rust_wasm_sdk::types::hash::transaction_hash::TransactionHash;
use casper_rust_wasm_sdk::types::transaction_params::transaction_builder_params::TransactionBuilderParams;

pub(super) async fn install_wasm(
    core: &CEPClient,
    wasm: &[u8],
    tx: &TransactionParams,
    args_json: &str,
) -> Result<CallResult> {
    let params = core.build_tx_params(tx, args_json)?;
    let bytes = CEPClient::bytes_from_slice(wasm);
    if !tx.put {
        let mut builder = TransactionBuilderParams::new_session(Some(bytes), Some(true));
        apply_runtime_v2(&mut builder, core.runtime_v2());
        return core.make_only_result(builder, params);
    }
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
        .map_err(|e| CEPError::Other(format!("serialize put result: {e}")))?;
    let result = CallResult::new(tx_hash, put_json);
    core.maybe_wait(tx, result).await
}

/// Run a session WASM that is not an install/upgrade (companion session).
pub(super) async fn call_session(
    core: &CEPClient,
    wasm: &[u8],
    tx: &TransactionParams,
    args_json: &str,
) -> Result<CallResult> {
    let params = core.build_tx_params(tx, args_json)?;
    let bytes = CEPClient::bytes_from_slice(wasm);
    let mut builder = TransactionBuilderParams::new_session(Some(bytes), Some(false));
    apply_runtime_v2(&mut builder, core.runtime_v2());
    if !tx.put {
        return core.make_only_result(builder, params);
    }
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
        .map_err(|e| CEPError::Other(format!("serialize put result: {e}")))?;
    let result = CallResult::new(tx_hash, put_json);
    core.maybe_wait(tx, result).await
}

fn apply_runtime_v2(builder: &mut TransactionBuilderParams, runtime_v2: Option<bool>) {
    match runtime_v2 {
        None | Some(true) => {}
        Some(false) => builder.set_runtime_v1(),
    }
}
