//! Shared CES MCP tools (wrap CepCore / SDK CESParser).

use crate::format;
use crate::tools::params;
use mcpkit::prelude::ToolOutput;

pub const TOOL_NAMES: &[&str] = &["ceps_ces_parse_execution", "ceps_ces_parse_transaction"];

pub async fn parse_execution(contract_hash: String, execution_result_json: String) -> ToolOutput {
    let client = match params::cep18_client(None, None) {
        Ok(c) => c,
        Err(e) => return format::err(e),
    };
    let exec: serde_json::Value = match serde_json::from_str(&execution_result_json) {
        Ok(v) => v,
        Err(e) => return format::err(e.to_string()),
    };
    let hash = normalize_hash(&contract_hash);
    match client.core().parse_ces_execution(&[hash], &exec).await {
        Ok(rows) => match serde_json::to_value(&rows) {
            Ok(v) => format::json_ok(&v),
            Err(e) => format::err(e.to_string()),
        },
        Err(e) => format::err(e),
    }
}

pub async fn parse_transaction(contract_hash: String, transaction_hash: String) -> ToolOutput {
    let client = match params::cep18_client(None, None) {
        Ok(c) => c,
        Err(e) => return format::err(e),
    };
    let hash = normalize_hash(&contract_hash);
    match client
        .core()
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

fn normalize_hash(contract_hash: &str) -> String {
    let t = contract_hash.trim();
    if t.starts_with("hash-") || t.starts_with("entity-") {
        t.to_string()
    } else {
        format!("hash-{t}")
    }
}
