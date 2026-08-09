//! Contract entrypoint calls.

use super::CepCore;
use crate::error::{CepError, Result};
use crate::types::{CallResult, TransactionParams};
use casper_rust_wasm_sdk::types::hash::transaction_hash::TransactionHash;

pub(super) async fn call_entrypoint(
    core: &CepCore,
    entry_point: &str,
    tx: &TransactionParams,
    args_json: &str,
) -> Result<CallResult> {
    let builder = core.builder_for_entrypoint(entry_point)?;
    let params = core.build_tx_params(tx, args_json)?;
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
        .map_err(|e| CepError::Other(format!("serialize put result: {e}")))?;
    let result = CallResult::new(tx_hash, put_json);
    core.maybe_wait(tx, result).await
}
