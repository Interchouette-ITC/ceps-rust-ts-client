//! CEP-85 multi-token client.

mod entity;
mod error;
mod keys;
mod types;

pub use entity::prefixed_key;
pub use error::Cep85Error;
pub use types::{ChangeSecurityArgs, InstallArgs, UpgradeArgs};

use crate::core::CepCore;
use crate::core::{
    bool_arg, json_args, key_arg, key_list_arg, string_arg, u256_arg, u256_list_arg, u8_arg,
    JsonArg,
};
use crate::error::{CepError, CepKind, Result};
use crate::types::{CallResult, DeployParams, EventsMode};
use casper_rust_wasm_sdk::types::verbosity::Verbosity;
use keys::{balance_dictionary_key, operator_dictionary_key};
use serde_json::Value;

/// Client for CEP-85 multi-token contracts.
pub struct Cep85Client {
    core: CepCore,
}

impl Cep85Client {
    /// Create a CEP-85 client.
    pub fn new(
        rpc_url: impl Into<String>,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<Verbosity>,
    ) -> Result<Self> {
        let core =
            CepCore::new(rpc_url, sse_url, chain_name, verbosity)?.with_cep_kind(CepKind::Cep85);
        Ok(Self { core })
    }

    /// Borrow the shared core.
    pub fn core(&self) -> &CepCore {
        &self.core
    }

    /// Mutable core access.
    pub fn core_mut(&mut self) -> &mut CepCore {
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

    /// Install a CEP-85 contract.
    pub async fn install(
        &self,
        args: &InstallArgs,
        wasm: &[u8],
        deploy: &DeployParams,
    ) -> Result<CallResult> {
        let mut v = vec![string_arg("name", &args.name), string_arg("uri", &args.uri)];
        if let Some(mode) = args.events_mode {
            v.push(u8_arg("events_mode", mode.into()));
        }
        if let Some(enable) = args.enable_burn {
            v.push(bool_arg("enable_burn", enable));
        }
        if let Some(list) = &args.admin_list {
            v.push(key_list_arg("admin_list", &map_keys(list)?));
        }
        if let Some(list) = &args.minter_list {
            v.push(key_list_arg("minter_list", &map_keys(list)?));
        }
        if let Some(list) = &args.burner_list {
            v.push(key_list_arg("burner_list", &map_keys(list)?));
        }
        if let Some(list) = &args.meta_list {
            v.push(key_list_arg("meta_list", &map_keys(list)?));
        }
        self.core.install_wasm(wasm, deploy, &json_args(&v)).await
    }

    /// Upgrade with `upgrade: true`.
    pub async fn upgrade(
        &self,
        args: &UpgradeArgs,
        wasm: &[u8],
        deploy: &DeployParams,
    ) -> Result<CallResult> {
        let v = vec![string_arg("name", &args.name), bool_arg("upgrade", true)];
        self.core.install_wasm(wasm, deploy, &json_args(&v)).await
    }

    /// Mint one token id.
    pub async fn mint(
        &self,
        recipient: &str,
        id: &str,
        amount: &str,
        uri: Option<&str>,
        deploy: &DeployParams,
    ) -> Result<CallResult> {
        let mut v = vec![
            key_arg("recipient", &prefixed_key(recipient)?),
            u256_arg("id", id),
            u256_arg("amount", amount),
        ];
        if let Some(uri) = uri {
            v.push(string_arg("uri", uri));
        }
        self.core
            .call_entrypoint("mint", deploy, &json_args(&v))
            .await
    }

    /// Batch mint.
    pub async fn batch_mint(
        &self,
        recipient: &str,
        ids: &[&str],
        amounts: &[&str],
        uri: Option<&str>,
        deploy: &DeployParams,
    ) -> Result<CallResult> {
        let mut v = vec![
            key_arg("recipient", &prefixed_key(recipient)?),
            u256_list_arg("ids", ids),
            u256_list_arg("amounts", amounts),
        ];
        if let Some(uri) = uri {
            v.push(string_arg("uri", uri));
        }
        self.core
            .call_entrypoint("batch_mint", deploy, &json_args(&v))
            .await
    }

    /// Burn one id.
    pub async fn burn(
        &self,
        owner: &str,
        id: &str,
        amount: &str,
        deploy: &DeployParams,
    ) -> Result<CallResult> {
        let v = vec![
            key_arg("owner", &prefixed_key(owner)?),
            u256_arg("id", id),
            u256_arg("amount", amount),
        ];
        self.core
            .call_entrypoint("burn", deploy, &json_args(&v))
            .await
    }

    /// Batch burn.
    pub async fn batch_burn(
        &self,
        owner: &str,
        ids: &[&str],
        amounts: &[&str],
        deploy: &DeployParams,
    ) -> Result<CallResult> {
        let v = vec![
            key_arg("owner", &prefixed_key(owner)?),
            u256_list_arg("ids", ids),
            u256_list_arg("amounts", amounts),
        ];
        self.core
            .call_entrypoint("batch_burn", deploy, &json_args(&v))
            .await
    }

    /// Transfer (on-chain `transfer_from`).
    pub async fn transfer(
        &self,
        from: &str,
        to: &str,
        id: &str,
        amount: &str,
        deploy: &DeployParams,
    ) -> Result<CallResult> {
        let v = vec![
            key_arg("from", &prefixed_key(from)?),
            key_arg("to", &prefixed_key(to)?),
            u256_arg("id", id),
            u256_arg("amount", amount),
        ];
        self.core
            .call_entrypoint("transfer_from", deploy, &json_args(&v))
            .await
    }

    /// Batch transfer (on-chain `batch_transfer_from`).
    pub async fn batch_transfer(
        &self,
        from: &str,
        to: &str,
        ids: &[&str],
        amounts: &[&str],
        deploy: &DeployParams,
    ) -> Result<CallResult> {
        let v = vec![
            key_arg("from", &prefixed_key(from)?),
            key_arg("to", &prefixed_key(to)?),
            u256_list_arg("ids", ids),
            u256_list_arg("amounts", amounts),
        ];
        self.core
            .call_entrypoint("batch_transfer_from", deploy, &json_args(&v))
            .await
    }

    /// Set approval for all.
    pub async fn set_approval_for_all(
        &self,
        operator: &str,
        approved: bool,
        deploy: &DeployParams,
    ) -> Result<CallResult> {
        let v = vec![
            key_arg("operator", &prefixed_key(operator)?),
            bool_arg("approved", approved),
        ];
        self.core
            .call_entrypoint("set_approval_for_all", deploy, &json_args(&v))
            .await
    }

    /// Set URI (optionally per id).
    pub async fn set_uri(
        &self,
        uri: &str,
        id: Option<&str>,
        deploy: &DeployParams,
    ) -> Result<CallResult> {
        let mut v = vec![string_arg("uri", uri)];
        if let Some(id) = id {
            v.push(u256_arg("id", id));
        }
        self.core
            .call_entrypoint("set_uri", deploy, &json_args(&v))
            .await
    }

    /// Set total supply cap for one id.
    pub async fn set_total_supply_of(
        &self,
        id: &str,
        total_supply: &str,
        deploy: &DeployParams,
    ) -> Result<CallResult> {
        let v = vec![u256_arg("id", id), u256_arg("total_supply", total_supply)];
        self.core
            .call_entrypoint("set_total_supply_of", deploy, &json_args(&v))
            .await
    }

    /// Set total supply caps in batch.
    pub async fn set_total_supply_of_batch(
        &self,
        ids: &[&str],
        total_supplies: &[&str],
        deploy: &DeployParams,
    ) -> Result<CallResult> {
        let v = vec![
            u256_list_arg("ids", ids),
            u256_list_arg("total_supplies", total_supplies),
        ];
        self.core
            .call_entrypoint("set_total_supply_of_batch", deploy, &json_args(&v))
            .await
    }

    /// Change security lists.
    pub async fn change_security(
        &self,
        args: &ChangeSecurityArgs,
        deploy: &DeployParams,
    ) -> Result<CallResult> {
        let mut v: Vec<JsonArg> = Vec::new();
        if let Some(list) = &args.admin_list {
            v.push(key_list_arg("admin_list", &map_keys(list)?));
        }
        if let Some(list) = &args.minter_list {
            v.push(key_list_arg("minter_list", &map_keys(list)?));
        }
        if let Some(list) = &args.burner_list {
            v.push(key_list_arg("burner_list", &map_keys(list)?));
        }
        if let Some(list) = &args.meta_list {
            v.push(key_list_arg("meta_list", &map_keys(list)?));
        }
        if let Some(list) = &args.none_list {
            v.push(key_list_arg("none_list", &map_keys(list)?));
        }
        if v.is_empty() {
            return Err(CepError::MissingArgument(
                "change_security requires at least one list".into(),
            ));
        }
        self.core
            .call_entrypoint("change_security", deploy, &json_args(&v))
            .await
    }

    /// Set modalities (burn / events).
    pub async fn set_modalities(
        &self,
        enable_burn: Option<bool>,
        events_mode: Option<EventsMode>,
        deploy: &DeployParams,
    ) -> Result<CallResult> {
        let mut v = Vec::new();
        if let Some(b) = enable_burn {
            v.push(bool_arg("enable_burn", b));
        }
        if let Some(m) = events_mode {
            v.push(u8_arg("events_mode", m.into()));
        }
        if v.is_empty() {
            return Err(CepError::MissingArgument(
                "set_modalities requires enable_burn and/or events_mode".into(),
            ));
        }
        self.core
            .call_entrypoint("set_modalities", deploy, &json_args(&v))
            .await
    }

    /// Collection name.
    pub async fn collection_name(&self) -> Result<String> {
        decode_string_cl(self.core.query_contract_key(&["name"]).await?)
    }

    /// Collection URI template.
    pub async fn collection_uri(&self) -> Result<String> {
        decode_string_cl(self.core.query_contract_key(&["uri"]).await?)
    }

    /// Balance of `account` for token `id`.
    pub async fn balance_of(&self, account: &str, id: &str) -> Result<String> {
        let item = balance_dictionary_key(account, id)?;
        decode_u256_cl(self.core.query_dictionary("balances", &item).await?)
    }

    /// Whether `operator` is approved for all of `owner`.
    pub async fn is_approved_for_all(&self, owner: &str, operator: &str) -> Result<bool> {
        let item = operator_dictionary_key(owner, operator)?;
        let raw = self.core.query_dictionary("operators", &item).await?;
        decode_bool_cl(raw)
    }

    /// Circulating supply for `id`.
    pub async fn supply_of(&self, id: &str) -> Result<String> {
        decode_u256_cl(self.core.query_dictionary("supply", id).await?)
    }

    /// Total supply cap for `id`.
    pub async fn total_supply_of(&self, id: &str) -> Result<String> {
        decode_u256_cl(self.core.query_dictionary("total_supply", id).await?)
    }

    /// Token URI for `id` (falls back to collection URI template when unset).
    pub async fn uri(&self, id: Option<&str>) -> Result<String> {
        match id {
            Some(id) => match self.core.query_dictionary("token_uri", id).await {
                Ok(raw) => decode_string_cl(raw),
                Err(_) => {
                    let template = self.collection_uri().await?;
                    Ok(template.replace("{id}", id))
                }
            },
            None => self.collection_uri().await,
        }
    }

    /// Whether `id` is non-fungible (total supply cap == 1).
    pub async fn is_non_fungible(&self, id: &str) -> Result<bool> {
        let cap = self.total_supply_of(id).await?;
        Ok(cap == "1")
    }
}

fn map_keys(list: &[String]) -> Result<Vec<String>> {
    list.iter().map(|k| prefixed_key(k)).collect()
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
    Err(CepError::Decode(format!("expected string, got {value}")))
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
        other => Err(CepError::Decode(format!("expected U256, got {other}"))),
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
        Value::Number(n) => Ok(n.as_u64().unwrap_or(0) != 0),
        other => Err(CepError::Decode(format!("expected bool, got {other}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_client() {
        let client =
            Cep85Client::new("http://127.0.0.1:11101", None, None, Some(Verbosity::Low)).unwrap();
        assert_eq!(client.rpc_url(), "http://127.0.0.1:11101/rpc");
    }

    #[test]
    fn entity_prefix_for_account() {
        let key = prefixed_key(
            "account-hash-b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f",
        )
        .unwrap();
        assert!(key.starts_with("entity-account-"));
    }

    #[test]
    fn install_args_include_uri_and_burn() {
        let args = InstallArgs::new("Bag", "https://x/{id}.json")
            .with_events_mode(EventsMode::Ces)
            .with_enable_burn(true);
        let mut v = vec![string_arg("name", &args.name), string_arg("uri", &args.uri)];
        if let Some(mode) = args.events_mode {
            v.push(u8_arg("events_mode", mode.into()));
        }
        if let Some(enable) = args.enable_burn {
            v.push(bool_arg("enable_burn", enable));
        }
        let s = json_args(&v);
        assert!(s.contains("Bag"));
        assert!(s.contains("enable_burn"));
        assert!(s.contains("events_mode"));
    }

    #[test]
    fn decode_bool_from_stored_value() {
        let v = serde_json::json!({
            "stored_value": { "CLValue": { "parsed": true } }
        });
        assert!(decode_bool_cl(v).unwrap());
    }
}
