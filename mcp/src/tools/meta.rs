//! Meta tools: help, endpoints, inventory.

use crate::format;
use crate::handle;
use crate::tools;
use rmcp::model::CallToolResult;

pub const TOOL_NAMES: &[&str] = &[
    "ceps_help",
    "ceps_get_endpoints",
    "ceps_set_endpoints",
    "ceps_list_tools",
];

pub fn help_text() -> String {
    let groups = tools::tool_groups()
        .iter()
        .map(|(g, d)| format!("  - {g}: {d}"))
        .collect::<Vec<_>>()
        .join("\n");
    let names = tools::registered_tool_names().join(", ");
    let snap = handle::snapshot();
    format!(
        r#"ceps-rust-ts-client-mcp {version}

CEP domain MCP for ceps-rust-ts-client. Use sdk_* (casper-rust-wasm-sdk-mcp) for raw RPC
and nctl_* for local testnet lifecycle. This server exposes ceps_* only.

Endpoints (process-wide):
  rpc_url={rpc}
  sse_url={sse}
  chain_name={chain}
  verbosity={verbosity:?}
  wasm_root={wasm}

Env: CEPS_RPC_URL, CEPS_SSE_URL, CEPS_CHAIN_NAME, CEPS_VERBOSITY, CEPS_WASM_ROOT
HTTP: CEPS_MCP_HTTP=1 CEPS_MCP_ADDR=0.0.0.0:6790

Write tools need secret_key_pem + payment_amount (motes). Prefer wasm_path under
tests/wasm (or CEPS_WASM_ROOT) for install/session bytes; wasm_base64 also accepted.

Groups:
{groups}

Tools ({count}): {names}
"#,
        version = crate::VERSION,
        rpc = snap.rpc_url,
        sse = snap.sse_url,
        chain = snap.chain_name,
        verbosity = snap.verbosity,
        wasm = handle::wasm_root().display(),
        groups = groups,
        count = tools::registered_tool_names().len(),
        names = names,
    )
}

pub fn get_endpoints() -> CallToolResult {
    let snap = handle::snapshot();
    format::json_ok(&serde_json::json!({
        "rpc_url": snap.rpc_url,
        "sse_url": snap.sse_url,
        "chain_name": snap.chain_name,
        "verbosity": format!("{:?}", snap.verbosity),
        "wasm_root": handle::wasm_root().display().to_string(),
        "env": {
            "CEPS_RPC_URL": handle::ENV_RPC_URL,
            "CEPS_SSE_URL": handle::ENV_SSE_URL,
            "CEPS_CHAIN_NAME": handle::ENV_CHAIN_NAME,
            "CEPS_VERBOSITY": handle::ENV_VERBOSITY,
            "CEPS_WASM_ROOT": handle::ENV_WASM_ROOT,
        }
    }))
}

pub fn set_endpoints(
    rpc_url: Option<String>,
    sse_url: Option<String>,
    chain_name: Option<String>,
    verbosity: Option<String>,
) -> CallToolResult {
    match handle::update(rpc_url, sse_url, chain_name, verbosity) {
        Ok(snap) => format::json_ok(&serde_json::json!({
            "rpc_url": snap.rpc_url,
            "sse_url": snap.sse_url,
            "chain_name": snap.chain_name,
            "verbosity": format!("{:?}", snap.verbosity),
        })),
        Err(e) => format::err(e),
    }
}

pub fn list_tools() -> CallToolResult {
    format::json_ok(&serde_json::json!({
        "groups": tools::tool_groups(),
        "tools": tools::registered_tool_names(),
    }))
}
