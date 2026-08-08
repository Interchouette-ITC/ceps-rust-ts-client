//! Shared MCP arg parsing helpers.

use crate::format;
use crate::handle;
use base64::Engine;
use ceps_client::cep18::{
    Cep18Client, InstallArgs as Cep18InstallArgs, UpgradeArgs as Cep18UpgradeArgs,
};
use ceps_client::cep78::{
    Cep78Client, InstallArgs as Cep78InstallArgs, NftMetadataKind, TokenIdentifier,
    UpgradeArgs as Cep78UpgradeArgs,
};
use ceps_client::cep85::{
    Cep85Client, InstallArgs as Cep85InstallArgs, UpgradeArgs as Cep85UpgradeArgs,
};
use ceps_client::cep95::{Cep95Client, InstallArgs as Cep95InstallArgs};
use ceps_client::{DeployParams, EventsMode, EventsMode78, Result as CepResult};
use mcpkit::prelude::ToolOutput;
use std::path::{Component, Path, PathBuf};

/// Build `DeployParams` from MCP args.
pub fn deploy_params(
    secret_key_pem: String,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    chain_name: Option<String>,
) -> DeployParams {
    let mut d = DeployParams::new(secret_key_pem, payment_amount);
    if wait == Some(false) {
        d = d.without_wait();
    }
    if let Some(ms) = wait_timeout_ms {
        d = d.with_timeout_ms(ms);
    }
    if let Some(chain) = chain_name {
        d = d.with_chain_name(chain);
    }
    d
}

fn resolve_under_root(root: &Path, relative: &str) -> Result<PathBuf, String> {
    let rel = Path::new(relative);
    if rel.is_absolute() {
        return Err("path must be relative to CEPS_WASM_ROOT (e.g. cep18/cep18.wasm)".into());
    }
    if rel.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err("path must not contain '..'".into());
    }
    let full = root.join(rel);
    let canon_root = root.canonicalize().map_err(|e| format!("wasm root: {e}"))?;
    let canon = full
        .canonicalize()
        .map_err(|e| format!("wasm path {relative}: {e}"))?;
    if !canon.starts_with(&canon_root) {
        return Err("path escapes CEPS_WASM_ROOT".into());
    }
    Ok(canon)
}

/// Load WASM bytes from base64 and/or a path under the wasm root.
pub fn load_wasm(
    wasm_base64: Option<String>,
    wasm_path: Option<String>,
) -> Result<Vec<u8>, String> {
    if let Some(b64) = wasm_base64 {
        return base64::engine::general_purpose::STANDARD
            .decode(b64.trim())
            .map_err(|e| format!("wasm_base64 decode: {e}"));
    }
    let Some(rel) = wasm_path else {
        return Err("provide wasm_base64 or wasm_path".into());
    };
    let path = resolve_under_root(&handle::wasm_root(), &rel)?;
    std::fs::read(&path).map_err(|e| format!("read {}: {e}", path.display()))
}

pub fn parse_events_mode(raw: Option<u8>) -> Result<Option<EventsMode>, String> {
    match raw {
        None => Ok(None),
        Some(v) => EventsMode::from_u8(v)
            .map(Some)
            .ok_or_else(|| format!("invalid events_mode u8: {v}")),
    }
}

pub fn parse_events_mode78(raw: Option<u8>) -> Result<Option<EventsMode78>, String> {
    match raw {
        None => Ok(None),
        Some(v) => EventsMode78::from_u8(v)
            .map(Some)
            .ok_or_else(|| format!("invalid events_mode u8: {v}")),
    }
}

pub fn parse_token_id(
    token_id: Option<u64>,
    token_hash: Option<String>,
) -> Result<TokenIdentifier, String> {
    match (token_id, token_hash) {
        (Some(id), None) => Ok(TokenIdentifier::id(id)),
        (None, Some(h)) => Ok(TokenIdentifier::hash(h)),
        _ => Err("provide exactly one of token_id or token_hash".into()),
    }
}

pub fn parse_nft_metadata_kind(raw: u8) -> Result<NftMetadataKind, String> {
    match raw {
        0 => Ok(NftMetadataKind::Cep78),
        1 => Ok(NftMetadataKind::Nft721),
        2 => Ok(NftMetadataKind::Raw),
        3 => Ok(NftMetadataKind::CustomValidated),
        _ => Err(format!("invalid nft_metadata_kind: {raw}")),
    }
}

pub fn call_result_json(r: &ceps_client::CallResult) -> serde_json::Value {
    serde_json::json!({
        "transactionHash": r.transaction_hash,
        "hasExecutionResult": r.execution_result.is_some(),
        "putResult": r.put_result,
        "executionResult": r.execution_result,
    })
}

pub fn map_call(result: CepResult<ceps_client::CallResult>) -> ToolOutput {
    match result {
        Ok(r) => format::json_ok(&call_result_json(&r)),
        Err(e) => format::err(e),
    }
}

pub fn map_query<T: serde::Serialize>(result: CepResult<T>) -> ToolOutput {
    format::from_result(result)
}

pub fn cep18_client(
    contract_hash: Option<&str>,
    package_hash: Option<&str>,
) -> Result<Cep18Client, String> {
    let ep = handle::snapshot();
    let mut c = Cep18Client::new(
        &ep.rpc_url,
        Some(ep.sse_url.clone()),
        Some(ep.chain_name.clone()),
        Some(ep.verbosity),
    )
    .map_err(|e| e.to_string())?;
    if let Some(h) = contract_hash {
        c.set_contract_hash(h, package_hash)
            .map_err(|e| e.to_string())?;
    }
    Ok(c)
}

pub fn cep78_client(
    contract_hash: Option<&str>,
    package_hash: Option<&str>,
) -> Result<Cep78Client, String> {
    let ep = handle::snapshot();
    let mut c = Cep78Client::new(
        &ep.rpc_url,
        Some(ep.sse_url.clone()),
        Some(ep.chain_name.clone()),
        Some(ep.verbosity),
    )
    .map_err(|e| e.to_string())?;
    if let Some(h) = contract_hash {
        c.set_contract_hash(h, package_hash)
            .map_err(|e| e.to_string())?;
    }
    Ok(c)
}

pub fn cep85_client(
    contract_hash: Option<&str>,
    package_hash: Option<&str>,
) -> Result<Cep85Client, String> {
    let ep = handle::snapshot();
    let mut c = Cep85Client::new(
        &ep.rpc_url,
        Some(ep.sse_url.clone()),
        Some(ep.chain_name.clone()),
        Some(ep.verbosity),
    )
    .map_err(|e| e.to_string())?;
    if let Some(h) = contract_hash {
        c.set_contract_hash(h, package_hash)
            .map_err(|e| e.to_string())?;
    }
    Ok(c)
}

pub fn cep95_client(
    contract_hash: Option<&str>,
    package_hash: Option<&str>,
) -> Result<Cep95Client, String> {
    let ep = handle::snapshot();
    let mut c = Cep95Client::new(
        &ep.rpc_url,
        Some(ep.sse_url.clone()),
        Some(ep.chain_name.clone()),
        Some(ep.verbosity),
    )
    .map_err(|e| e.to_string())?;
    if let Some(h) = contract_hash {
        c.set_contract_hash(h, package_hash)
            .map_err(|e| e.to_string())?;
    }
    Ok(c)
}

pub fn cep95_install_args(
    name: String,
    symbol: String,
    package_hash_key_name: String,
    allow_key_override: Option<bool>,
    is_upgradable: Option<bool>,
    is_upgrade: Option<bool>,
) -> Cep95InstallArgs {
    let mut args = Cep95InstallArgs::new(name, symbol, package_hash_key_name);
    if let Some(v) = allow_key_override {
        args = args.with_allow_key_override(v);
    }
    if let Some(v) = is_upgradable {
        args = args.with_upgradable(v);
    }
    if let Some(v) = is_upgrade {
        args = args.with_upgrade(v);
    }
    args
}

pub fn cep18_install_args(
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: String,
    events_mode: Option<u8>,
    enable_mint_and_burn: Option<bool>,
    admin_list: Option<Vec<String>>,
    minter_list: Option<Vec<String>>,
) -> Result<Cep18InstallArgs, String> {
    let mut args = Cep18InstallArgs::new(name, symbol, decimals, total_supply);
    if let Some(mode) = parse_events_mode(events_mode)? {
        args = args.with_events_mode(mode);
    }
    if let Some(v) = enable_mint_and_burn {
        args = args.with_mint_and_burn(v);
    }
    args.admin_list = admin_list;
    args.minter_list = minter_list;
    Ok(args)
}

pub fn cep18_upgrade_args(
    name: String,
    events_mode: Option<u8>,
) -> Result<Cep18UpgradeArgs, String> {
    let mut args = Cep18UpgradeArgs::new(name);
    if let Some(mode) = parse_events_mode(events_mode)? {
        args.events_mode = Some(mode);
    }
    Ok(args)
}

pub fn cep85_install_args(
    name: String,
    uri: String,
    events_mode: Option<u8>,
    enable_burn: Option<bool>,
    admin_list: Option<Vec<String>>,
    minter_list: Option<Vec<String>>,
    burner_list: Option<Vec<String>>,
    meta_list: Option<Vec<String>>,
) -> Result<Cep85InstallArgs, String> {
    let mut args = Cep85InstallArgs::new(name, uri);
    if let Some(mode) = parse_events_mode(events_mode)? {
        args = args.with_events_mode(mode);
    }
    if let Some(v) = enable_burn {
        args = args.with_enable_burn(v);
    }
    args.admin_list = admin_list;
    args.minter_list = minter_list;
    args.burner_list = burner_list;
    args.meta_list = meta_list;
    Ok(args)
}

pub fn cep85_upgrade_args(name: String) -> Cep85UpgradeArgs {
    Cep85UpgradeArgs::new(name)
}

pub fn cep78_install_args_basic(
    collection_name: String,
    collection_symbol: String,
    total_token_supply: u64,
    events_mode: Option<u8>,
) -> Result<Cep78InstallArgs, String> {
    let mut args = Cep78InstallArgs::new(collection_name, collection_symbol, total_token_supply);
    if let Some(mode) = parse_events_mode78(events_mode)? {
        args.events_mode = Some(mode);
    }
    Ok(args)
}

pub fn cep78_upgrade_args(
    collection_name: String,
    total_token_supply: Option<u64>,
) -> Cep78UpgradeArgs {
    let mut args = Cep78UpgradeArgs::new(collection_name);
    args.total_token_supply = total_token_supply;
    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deploy_params_wait_flag() {
        let d = deploy_params("pem".into(), "1".into(), Some(false), None, None);
        assert!(!d.wait);
    }

    #[test]
    fn token_id_xor() {
        assert!(parse_token_id(Some(1), None).is_ok());
        assert!(parse_token_id(None, Some("ab".into())).is_ok());
        assert!(parse_token_id(Some(1), Some("ab".into())).is_err());
    }
}
