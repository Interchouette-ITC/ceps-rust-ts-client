//! Process-wide CEP endpoint config (NCTL-friendly defaults).

use std::sync::{Mutex, OnceLock};

use ceps_client::core::{DEFAULT_CHAIN_NAME, DEFAULT_RPC_URL, DEFAULT_SSE_URL};
use ceps_client::Verbosity;

/// Env: JSON-RPC endpoint.
pub const ENV_RPC_URL: &str = "CEPS_RPC_URL";
/// Env: SSE events endpoint.
pub const ENV_SSE_URL: &str = "CEPS_SSE_URL";
/// Env: chain name.
pub const ENV_CHAIN_NAME: &str = "CEPS_CHAIN_NAME";
/// Env: verbosity (`low` | `medium` | `high` | `0` | `1` | `2`).
pub const ENV_VERBOSITY: &str = "CEPS_VERBOSITY";
/// Env: root for demo contract WASMs (defaults to `tests/wasm` under cwd).
pub use ceps_client::wasm::ENV_WASM_ROOT;

#[derive(Debug, Clone)]
pub struct Endpoints {
    pub rpc_url: String,
    pub sse_url: String,
    pub chain_name: String,
    pub verbosity: Verbosity,
}

impl Endpoints {
    pub fn from_env() -> Self {
        Self {
            rpc_url: std::env::var(ENV_RPC_URL).unwrap_or_else(|_| DEFAULT_RPC_URL.to_string()),
            sse_url: std::env::var(ENV_SSE_URL).unwrap_or_else(|_| DEFAULT_SSE_URL.to_string()),
            chain_name: std::env::var(ENV_CHAIN_NAME)
                .unwrap_or_else(|_| DEFAULT_CHAIN_NAME.to_string()),
            verbosity: parse_verbosity(
                std::env::var(ENV_VERBOSITY)
                    .unwrap_or_else(|_| "low".to_string())
                    .as_str(),
            ),
        }
    }
}

static SHARED: OnceLock<Mutex<Endpoints>> = OnceLock::new();

fn shared_mutex() -> &'static Mutex<Endpoints> {
    SHARED.get_or_init(|| Mutex::new(Endpoints::from_env()))
}

/// Snapshot of current endpoints (safe across `.await`).
pub fn snapshot() -> Endpoints {
    shared_mutex()
        .lock()
        .expect("endpoints mutex poisoned")
        .clone()
}

/// Mutate shared endpoints.
pub fn update(
    rpc_url: Option<String>,
    sse_url: Option<String>,
    chain_name: Option<String>,
    verbosity: Option<String>,
) -> Result<Endpoints, String> {
    let mut guard = shared_mutex()
        .lock()
        .map_err(|e| format!("endpoints mutex poisoned: {e}"))?;
    if let Some(rpc) = rpc_url {
        guard.rpc_url = rpc;
    }
    if let Some(sse) = sse_url {
        guard.sse_url = sse;
    }
    if let Some(chain) = chain_name {
        guard.chain_name = chain;
    }
    if let Some(raw) = verbosity {
        guard.verbosity = parse_verbosity(&raw);
    }
    Ok(guard.clone())
}

/// Parse verbosity without panicking.
pub fn parse_verbosity(raw: &str) -> Verbosity {
    match raw.trim().to_lowercase().as_str() {
        "low" | "0" => Verbosity::Low,
        "medium" | "1" => Verbosity::Medium,
        "high" | "2" => Verbosity::High,
        _ => Verbosity::Low,
    }
}

/// Resolve demo WASM root directory.
pub fn wasm_root() -> std::path::PathBuf {
    ceps_client::wasm::wasm_root()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_verbosity_accepts_aliases() {
        assert!(matches!(parse_verbosity("LOW"), Verbosity::Low));
        assert!(matches!(parse_verbosity("1"), Verbosity::Medium));
        assert!(matches!(parse_verbosity("high"), Verbosity::High));
    }
}
