//! Shared CES MCP tools (wrap CEPClient / SDK CESParser).

use crate::format;
use crate::handle;
use ceps_client::CEPClient;
use mcpkit::prelude::ToolOutput;

pub const TOOL_NAMES: &[&str] = &[
    "ceps_ces_parse_execution",
    "ceps_ces_parse_transaction",
    "ceps_ces_collect",
];

pub async fn parse_execution(contract_hash: String, execution_result_json: String) -> ToolOutput {
    let client = match shared_client() {
        Ok(c) => c,
        Err(e) => return format::err(e),
    };
    let exec: serde_json::Value = match serde_json::from_str(&execution_result_json) {
        Ok(v) => v,
        Err(e) => return format::err(e.to_string()),
    };
    let hash = normalize_hash(&contract_hash);
    match client.parse_ces_execution(&[hash], &exec).await {
        Ok(rows) => match serde_json::to_value(&rows) {
            Ok(v) => format::json_ok(&v),
            Err(e) => format::err(e.to_string()),
        },
        Err(e) => format::err(e),
    }
}

pub async fn parse_transaction(contract_hash: String, transaction_hash: String) -> ToolOutput {
    let client = match shared_client() {
        Ok(c) => c,
        Err(e) => return format::err(e),
    };
    let hash = normalize_hash(&contract_hash);
    match client
        .parse_ces_transaction(&[hash], &transaction_hash)
        .await
    {
        Ok(rows) => match serde_json::to_value(&rows) {
            Ok(v) => format::json_ok(&v),
            Err(e) => format::err(e.to_string()),
        },
        Err(e) => format::err(e),
    }
}

pub async fn collect(
    contract_hash: String,
    package_hash: Option<String>,
    event_names: Option<Vec<String>>,
    max_transactions: Option<u64>,
    timeout_ms: Option<u64>,
) -> ToolOutput {
    let mut client = match shared_client() {
        Ok(c) => c,
        Err(e) => return format::err(e),
    };
    if let Err(e) = client.set_contract_hash(&contract_hash, package_hash.as_deref()) {
        return format::err(e);
    }
    let names_owned = event_names.unwrap_or_default();
    let names: Vec<&str> = names_owned.iter().map(String::as_str).collect();
    let max_tx = usize::try_from(max_transactions.unwrap_or(8)).unwrap_or(8);
    let timeout = timeout_ms.unwrap_or(120_000);
    match client.collect_ces_events(&names, max_tx, timeout).await {
        Ok(rows) => match serde_json::to_value(&rows) {
            Ok(v) => format::json_ok(&v),
            Err(e) => format::err(e.to_string()),
        },
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

fn normalize_hash(contract_hash: &str) -> String {
    let t = contract_hash.trim();
    if t.starts_with("hash-") || t.starts_with("entity-") {
        t.to_string()
    } else {
        format!("hash-{t}")
    }
}
