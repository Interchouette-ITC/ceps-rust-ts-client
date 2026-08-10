//! Shared `CEPClient` MCP tools (put / wait).

use crate::format;
use crate::handle;
use crate::tools::params;
use ceps_client::CEPClient;
use mcpkit::prelude::ToolOutput;

pub const TOOL_NAMES: &[&str] = &["ceps_put_transaction", "ceps_wait_transaction"];

pub async fn put_transaction(
    transaction_json: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    contract_hash: Option<String>,
    package_hash: Option<String>,
) -> ToolOutput {
    let mut client = match shared_client() {
        Ok(c) => c,
        Err(e) => return format::err(e),
    };
    if let Some(h) = contract_hash.filter(|s| !s.trim().is_empty()) {
        if let Err(e) = client.set_contract_hash(h, package_hash.as_deref()) {
            return format::err(e);
        }
    }
    let tx: serde_json::Value = match serde_json::from_str(&transaction_json) {
        Ok(v) => v,
        Err(e) => return format::err(e.to_string()),
    };
    let do_wait = wait.unwrap_or(true);
    params::map_call(client.put_transaction(&tx, do_wait, wait_timeout_ms).await)
}

pub async fn wait_transaction(
    transaction_hash: String,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let client = match shared_client() {
        Ok(c) => c,
        Err(e) => return format::err(e),
    };
    match client
        .wait_transaction(&transaction_hash, wait_timeout_ms)
        .await
    {
        Ok(v) => format::json_ok(&v),
        Err(e) => format::err(e),
    }
}

fn shared_client() -> Result<CEPClient, String> {
    let ep = handle::snapshot();
    CEPClient::new(
        &ep.rpc_url,
        Some(ep.sse_url.clone()),
        Some(ep.chain_name.clone()),
        Some(ep.verbosity),
    )
    .map_err(|e| e.to_string())
}
