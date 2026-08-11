//! CEP-95 NFT client (JS-client API parity, Odra tip install).

mod entity;
mod error;
mod keys;
mod types;

pub use error::CEP95Error;
pub use types::InstallArgs;

use crate::core::json_args;
use crate::core::CEPClient;
use crate::error::{CEPError, CEPKind, Result};
use crate::schema::arg;
use crate::schema::cep95_fields as sch;
use crate::types::{CallResult, TransactionParams};
use casper_rust_wasm_sdk::types::verbosity::Verbosity;
use entity::prefixed_key;
use keys::{
    balance_dictionary_key, operator_dictionary_key, ownable_owner_state_key,
    token_id_dictionary_key,
};
use serde_json::{json, Value};

/// Client for CEP-95 NFT contracts (Odra OwnedCEP95 tip and compatible ABIs).
pub struct CEP95Client {
    core: CEPClient,
}

impl CEP95Client {
    /// Create a CEP-95 client.
    pub fn new(
        rpc_url: impl Into<String>,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<Verbosity>,
    ) -> Result<Self> {
        let core =
            CEPClient::new(rpc_url, sse_url, chain_name, verbosity)?.with_cep_kind(CEPKind::CEP95);
        Ok(Self { core })
    }

    /// Borrow the shared core.
    pub fn core(&self) -> &CEPClient {
        &self.core
    }

    /// Mutable core access.
    pub fn core_mut(&mut self) -> &mut CEPClient {
        &mut self.core
    }

    /// RPC URL.
    pub fn rpc_url(&self) -> &str {
        self.core.rpc_url()
    }

    /// SSE URL when set.
    pub fn sse_url(&self) -> Option<&str> {
        self.core.sse_url()
    }

    /// Chain name.
    pub fn chain_name(&self) -> &str {
        self.core.chain_name()
    }

    /// Verbosity.
    pub fn verbosity(&self) -> Verbosity {
        self.core.verbosity()
    }

    /// Set RPC URL.
    pub fn set_rpc_url(&mut self, rpc_url: impl Into<String>) -> Result<()> {
        self.core.set_rpc_url(rpc_url)
    }

    /// Set SSE URL.
    pub fn set_sse_url(&mut self, sse_url: impl Into<String>) -> Result<()> {
        self.core.set_sse_url(sse_url)
    }

    /// Set chain name.
    pub fn set_chain_name(&mut self, chain_name: impl Into<String>) {
        self.core.set_chain_name(chain_name);
    }

    /// Set verbosity.
    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.core.set_verbosity(verbosity);
    }

    /// Bind contract and optional package hash.
    pub fn set_contract_hash(
        &mut self,
        contract_hash: impl AsRef<str>,
        package_hash: Option<impl AsRef<str>>,
    ) -> Result<()> {
        self.core.set_contract_hash(contract_hash, package_hash)
    }

    /// Read a named key from an account (public key hex or `account-hash-…`).
    pub async fn get_account_named_key(
        &self,
        account_identifier: &str,
        named_key: &str,
    ) -> Result<String> {
        self.core
            .get_account_named_key(account_identifier, named_key)
            .await
    }

    /// Install an Odra OwnedCEP95 (or compatible) WASM.
    pub async fn install(
        &self,
        args: &InstallArgs,
        wasm: &[u8],
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let args_json = install_args_json(args);
        self.core.install_wasm(wasm, tx, &args_json).await
    }

    /// Transfer without recipient check (`transfer_from`).
    pub async fn transfer_from(
        &self,
        from: &str,
        to: &str,
        token_id: &str,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let args = json_args(&[
            arg(&sch::ep::TF_FROM, Value::String(prefixed_key(from)?)),
            arg(&sch::ep::TF_TO, Value::String(prefixed_key(to)?)),
            arg(&sch::ep::TF_TOKEN_ID, Value::String(token_id.to_string())),
        ]);
        self.core.call_entrypoint("transfer_from", tx, &args).await
    }

    /// Safe transfer with optional receiver data.
    pub async fn safe_transfer_from(
        &self,
        from: &str,
        to: &str,
        token_id: &str,
        data: Option<&[u8]>,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let data_value = match data {
            Some(b) if !b.is_empty() => json!(b),
            _ => Value::Null,
        };
        let args = json_args(&[
            arg(&sch::ep::STF_FROM, Value::String(prefixed_key(from)?)),
            arg(&sch::ep::STF_TO, Value::String(prefixed_key(to)?)),
            arg(&sch::ep::STF_TOKEN_ID, Value::String(token_id.to_string())),
            arg(&sch::ep::STF_DATA, data_value),
        ]);
        self.core
            .call_entrypoint("safe_transfer_from", tx, &args)
            .await
    }

    /// Approve a spender for one token.
    pub async fn approve(
        &self,
        spender: &str,
        token_id: &str,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let args = json_args(&[
            arg(
                &sch::ep::APPROVE_SPENDER,
                Value::String(prefixed_key(spender)?),
            ),
            arg(
                &sch::ep::APPROVE_TOKEN_ID,
                Value::String(token_id.to_string()),
            ),
        ]);
        self.core.call_entrypoint("approve", tx, &args).await
    }

    /// Revoke single-token approval.
    pub async fn revoke_approval(
        &self,
        token_id: &str,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let args = json_args(&[arg(
            &sch::ep::REVOKE_TOKEN_ID,
            Value::String(token_id.to_string()),
        )]);
        self.core
            .call_entrypoint("revoke_approval", tx, &args)
            .await
    }

    /// Approve an operator for all of the caller's tokens.
    pub async fn approve_for_all(
        &self,
        operator: &str,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let args = json_args(&[arg(
            &sch::ep::OPERATOR,
            Value::String(prefixed_key(operator)?),
        )]);
        self.core
            .call_entrypoint("approve_for_all", tx, &args)
            .await
    }

    /// Revoke operator approval for all tokens.
    pub async fn revoke_approval_for_all(
        &self,
        operator: &str,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let args = json_args(&[arg(
            &sch::ep::OPERATOR,
            Value::String(prefixed_key(operator)?),
        )]);
        self.core
            .call_entrypoint("revoke_approval_for_all", tx, &args)
            .await
    }

    /// Mint a token (owner-gated on OwnedCEP95). Metadata defaults to empty.
    pub async fn mint(
        &self,
        to: &str,
        token_id: &str,
        metadata: Option<&[(String, String)]>,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let pairs: &[(String, String)] = metadata.unwrap_or(&[]);
        let values: Vec<Value> = pairs.iter().map(|(k, v)| json!([k, v])).collect();
        let args = json_args(&[
            arg(&sch::ep::MINT_TO, Value::String(prefixed_key(to)?)),
            arg(&sch::ep::MINT_TOKEN_ID, Value::String(token_id.to_string())),
            arg(&sch::ep::MINT_METADATA, json!(values)),
        ]);
        self.core.call_entrypoint("mint", tx, &args).await
    }

    /// Burn a token (token-owner gated on OwnedCEP95).
    pub async fn burn(&self, token_id: &str, tx: &TransactionParams) -> Result<CallResult> {
        let args = json_args(&[arg(
            &sch::ep::BURN_TOKEN_ID,
            Value::String(token_id.to_string()),
        )]);
        self.core.call_entrypoint("burn", tx, &args).await
    }

    /// Transfer Ownable contract ownership (`new_owner` arg).
    pub async fn transfer_ownership(
        &self,
        new_owner: &str,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let args = json_args(&[arg(
            &sch::ep::NEW_OWNER,
            Value::String(prefixed_key(new_owner)?),
        )]);
        self.core
            .call_entrypoint("transfer_ownership", tx, &args)
            .await
    }

    /// Collection name (named key).
    pub async fn name(&self) -> Result<String> {
        decode_string_cl(self.core.query_contract_key(&["name"]).await?)
    }

    /// Collection symbol (named key).
    pub async fn symbol(&self) -> Result<String> {
        decode_string_cl(self.core.query_contract_key(&["symbol"]).await?)
    }

    /// Total supply named key when present (spec / JS). Odra tip may omit this key.
    pub async fn total_supply(&self) -> Result<String> {
        decode_u256_cl(self.core.query_contract_key(&["total_supply"]).await?)
    }

    /// Contract Ownable owner (Odra `state` dict at Ownable owner Var path).
    pub async fn get_owner(&self) -> Result<String> {
        let item = ownable_owner_state_key()?;
        let raw = self.core.query_dictionary("state", &item).await?;
        decode_odra_option_address_cl(raw)
    }

    /// Balance of `owner`.
    pub async fn balance_of(&self, owner: &str) -> Result<String> {
        let item_key = balance_dictionary_key(owner)?;
        match self.core.query_dictionary("balances", &item_key).await {
            Ok(raw) => decode_u256_cl(raw),
            Err(CEPError::EmptyQuery(_)) | Err(CEPError::Sdk(_)) => Ok("0".into()),
            Err(e) => Err(e),
        }
    }

    /// Owner of `token_id` as a prefixed key string.
    pub async fn owner_of(&self, token_id: &str) -> Result<String> {
        let item_key = token_id_dictionary_key(token_id)?;
        let raw = self.core.query_dictionary("owners", &item_key).await?;
        decode_key_cl(raw)
    }

    /// Approved spender for `token_id`, or `None` when unset (JS `getApproved`).
    pub async fn get_approved(&self, token_id: &str) -> Result<Option<String>> {
        let item_key = token_id_dictionary_key(token_id)?;
        match self.core.query_dictionary("approvals", &item_key).await {
            Ok(raw) => Ok(Some(decode_key_cl(raw)?)),
            Err(CEPError::EmptyQuery(_)) | Err(CEPError::Sdk(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Whether `operator` is approved for all of `owner`'s tokens.
    pub async fn is_approved_for_all(&self, owner: &str, operator: &str) -> Result<bool> {
        let item_key = operator_dictionary_key(owner, operator)?;
        match self.core.query_dictionary("operators", &item_key).await {
            Ok(raw) => decode_bool_cl(raw),
            Err(CEPError::EmptyQuery(_)) | Err(CEPError::Sdk(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// On-chain token metadata as key/value pairs (empty when missing).
    pub async fn token_metadata(&self, token_id: &str) -> Result<Vec<(String, String)>> {
        let item_key = token_id_dictionary_key(token_id)?;
        match self
            .core
            .query_dictionary("token_metadata", &item_key)
            .await
        {
            Ok(raw) => decode_string_map_cl(raw),
            Err(CEPError::EmptyQuery(_)) | Err(CEPError::Sdk(_)) => Ok(Vec::new()),
            Err(e) => Err(e),
        }
    }

    /// After Odra install: read package named key from the installer account and
    /// resolve the latest contract hash from the package, then bind both.
    pub async fn bind_odra_install(
        &mut self,
        installer_public_key: &str,
        package_hash_key_name: &str,
    ) -> Result<(String, String)> {
        let package = self
            .core
            .get_account_named_key(installer_public_key, package_hash_key_name)
            .await?;
        let contract = resolve_latest_contract_hash(self.core(), &package).await?;
        self.set_contract_hash(&contract, Some(&package))?;
        Ok((contract, package))
    }
}

fn install_args_json(args: &InstallArgs) -> String {
    json_args(&[
        arg(
            &sch::install::PACKAGE_HASH_KEY_NAME,
            Value::String(args.package_hash_key_name.clone()),
        ),
        arg(
            &sch::install::ALLOW_KEY_OVERRIDE,
            json!(args.allow_key_override),
        ),
        arg(&sch::install::IS_UPGRADABLE, json!(args.is_upgradable)),
        arg(&sch::install::IS_UPGRADE, json!(args.is_upgrade)),
        arg(&sch::install::NAME, Value::String(args.name.clone())),
        arg(&sch::install::SYMBOL, Value::String(args.symbol.clone())),
    ])
}

async fn resolve_latest_contract_hash(core: &CEPClient, package_key: &str) -> Result<String> {
    use crate::types::strip_hash_prefix;
    use casper_rust_wasm_sdk::rpcs::query_global_state::{
        KeyIdentifierInput, QueryGlobalStateParams,
    };

    let hex = strip_hash_prefix(package_key);
    let candidates = [
        package_key.to_string(),
        format!("hash-{hex}"),
        format!("package-{hex}"),
    ];
    let mut last_err = None;
    for key in candidates {
        let params = QueryGlobalStateParams {
            key: KeyIdentifierInput::String(key.clone()),
            path: None,
            maybe_global_state_identifier: None,
            state_root_hash: None,
            maybe_block_id: None,
            verbosity: Some(core.verbosity()),
            rpc_address: Some(core.rpc_url().to_string()),
        };
        match core.sdk().query_global_state(params).await {
            Ok(response) => {
                let value = serde_json::to_value(&response.result)
                    .map_err(|e| CEPError::Decode(format!("package query serialize: {e}")))?;
                if let Some(hash) = extract_latest_contract_hash(&value) {
                    return Ok(hash);
                }
                last_err = Some(CEPError::Decode(format!(
                    "no contract version in package under {key}: {value}"
                )));
            }
            Err(e) => last_err = Some(CEPError::from(e)),
        }
    }
    Err(last_err.unwrap_or_else(|| {
        CEPError::EmptyQuery(format!("package contract hash for {package_key}"))
    }))
}

fn extract_latest_contract_hash(value: &Value) -> Option<String> {
    // Walk common Casper package JSON shapes for the highest contract_version.
    let mut best: Option<(u64, String)> = None;
    fn visit(v: &Value, best: &mut Option<(u64, String)>) {
        match v {
            Value::Object(map) => {
                if let (Some(ver), Some(hash)) = (
                    map.get("contract_version")
                        .and_then(|x| x.as_u64())
                        .or_else(|| {
                            map.get("contract_version")
                                .and_then(|x| x.as_str())
                                .and_then(|s| s.parse().ok())
                        }),
                    map.get("contract_hash")
                        .and_then(|x| x.as_str())
                        .map(str::to_string)
                        .or_else(|| {
                            map.get("contract_hash")
                                .and_then(|x| x.get("Hash"))
                                .and_then(|x| x.as_str())
                                .map(|s| format!("hash-{s}"))
                        }),
                ) {
                    let better = best.as_ref().map(|(v, _)| ver >= *v).unwrap_or(true);
                    if better {
                        let h = if hash.contains('-') {
                            hash
                        } else {
                            format!("hash-{hash}")
                        };
                        *best = Some((ver, h));
                    }
                }
                for child in map.values() {
                    visit(child, best);
                }
            }
            Value::Array(items) => {
                for child in items {
                    visit(child, best);
                }
            }
            _ => {}
        }
    }
    visit(value, &mut best);
    best.map(|(_, h)| h)
}

fn decode_string_cl(value: Value) -> Result<String> {
    if let Some(s) = value
        .pointer("/stored_value/CLValue/parsed")
        .and_then(|v| v.as_str())
    {
        return Ok(s.to_string());
    }
    if let Some(s) = value.pointer("/CLValue/parsed").and_then(|v| v.as_str()) {
        return Ok(s.to_string());
    }
    if let Some(s) = value.as_str() {
        return Ok(s.to_string());
    }
    Err(CEPError::Decode(format!(
        "expected string CLValue, got {value}"
    )))
}

fn decode_u256_cl(value: Value) -> Result<String> {
    let parsed = value
        .pointer("/stored_value/CLValue/parsed")
        .or_else(|| value.pointer("/CLValue/parsed"))
        .cloned()
        .unwrap_or(value);
    match parsed {
        Value::String(s) => Ok(s),
        Value::Number(n) => Ok(n.to_string()),
        other => Err(CEPError::Decode(format!("expected U256, got {other}"))),
    }
}

fn decode_bool_cl(value: Value) -> Result<bool> {
    let parsed = value
        .pointer("/stored_value/CLValue/parsed")
        .or_else(|| value.pointer("/CLValue/parsed"))
        .cloned()
        .unwrap_or(value);
    match parsed {
        Value::Bool(b) => Ok(b),
        Value::String(s) => Ok(s == "true" || s == "1"),
        Value::Number(n) => Ok(n.as_u64().unwrap_or(0) != 0),
        other => Err(CEPError::Decode(format!("expected Bool, got {other}"))),
    }
}

fn decode_key_cl(value: Value) -> Result<String> {
    let parsed = value
        .pointer("/stored_value/CLValue/parsed")
        .or_else(|| value.pointer("/CLValue/parsed"))
        .cloned()
        .unwrap_or(value.clone());
    if let Some(s) = parsed.as_str() {
        return Ok(s.to_string());
    }
    // Option::Some(Key) or nested map shapes.
    if let Some(s) = parsed.pointer("/Some").and_then(|v| v.as_str()) {
        return Ok(s.to_string());
    }
    if let Some(s) = parsed
        .as_object()
        .and_then(|o| o.values().next())
        .and_then(|v| v.as_str())
    {
        return Ok(s.to_string());
    }
    Err(CEPError::Decode(format!("expected Key, got {value}")))
}

/// Decode Odra `Var<Option<Address>>` stored under `state` as `CLValue(Vec<u8>)`
/// of `Option<Key>` bytesrepr, or already-parsed Option/Key JSON.
fn decode_odra_option_address_cl(value: Value) -> Result<String> {
    if let Ok(s) = decode_key_cl(value.clone()) {
        return Ok(s);
    }
    let parsed = value
        .pointer("/stored_value/CLValue/parsed")
        .or_else(|| value.pointer("/CLValue/parsed"))
        .cloned()
        .unwrap_or(value.clone());
    let bytes = match parsed {
        Value::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                let n = item.as_u64().ok_or_else(|| {
                    CEPError::Decode(format!("expected byte in Odra state array, got {item}"))
                })?;
                out.push(u8::try_from(n).map_err(|_| {
                    CEPError::Decode(format!("byte out of range in Odra state: {n}"))
                })?);
            }
            out
        }
        Value::String(s) => {
            if s.starts_with("account-hash-")
                || s.starts_with("hash-")
                || s.starts_with("entity-")
                || s.starts_with("uref-")
            {
                return Ok(s);
            }
            hex::decode(s.trim_start_matches("0x"))
                .map_err(|e| CEPError::Decode(format!("Odra state hex bytes: {e}")))?
        }
        other => {
            return Err(CEPError::Decode(format!(
                "expected Odra Option<Address> bytes, got {other}"
            )));
        }
    };
    use casper_types::bytesrepr::FromBytes;
    use casper_types::Key;
    let (opt, _) = Option::<Key>::from_bytes(&bytes)
        .map_err(|e| CEPError::Decode(format!("Option<Key> from Odra state: {e:?}")))?;
    opt.map(|k| k.to_formatted_string())
        .ok_or_else(|| CEPError::Decode("Ownable owner is unset".into()))
}

fn decode_string_map_cl(value: Value) -> Result<Vec<(String, String)>> {
    let parsed = value
        .pointer("/stored_value/CLValue/parsed")
        .or_else(|| value.pointer("/CLValue/parsed"))
        .cloned()
        .unwrap_or(value);
    let mut out = Vec::new();
    match parsed {
        Value::Object(map) => {
            for (k, v) in map {
                if let Some(s) = v.as_str() {
                    out.push((k, s.to_string()));
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                if let Some(arr) = item.as_array() {
                    if arr.len() >= 2 {
                        if let (Some(k), Some(v)) = (arr[0].as_str(), arr[1].as_str()) {
                            out.push((k.to_string(), v.to_string()));
                        }
                    }
                }
            }
        }
        _ => {
            return Err(CEPError::Decode(format!(
                "expected metadata map, got {parsed}"
            )));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_client() {
        let client = CEP95Client::new(
            "http://127.0.0.1:11101",
            Some("http://127.0.0.1:18101".into()),
            None,
            Some(Verbosity::Low),
        )
        .unwrap();
        assert_eq!(client.rpc_url(), "http://127.0.0.1:11101/rpc");
    }

    #[test]
    fn install_json_includes_odra_cfg() {
        let args = InstallArgs::new("N", "S", "cep95_demo");
        let s = install_args_json(&args);
        assert!(s.contains("odra_cfg_package_hash_key_name"));
        assert!(s.contains("cep95_demo"));
        assert!(s.contains("\"name\":\"name\""));
        assert!(s.contains("\"name\":\"symbol\""));
        crate::schema::assert_install_json_matches_schema(crate::schema::CepId::Cep95, &s);
    }
}
