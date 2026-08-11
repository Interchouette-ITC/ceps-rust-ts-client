//! CEP-85 multi-token client.

mod entity;
mod error;
mod keys;
mod types;

pub use entity::prefixed_key;
pub use error::CEP85Error;
pub use types::{ChangeSecurityArgs, InstallArgs, SecurityBadge85, UpgradeArgs};

use crate::core::CEPClient;
use crate::core::{
    bool_arg, json_args, key_arg, key_list_arg, option_byte_list_arg, string_arg, u256_arg,
    u256_list_arg, u8_arg, JsonArg,
};
use crate::error::{CEPError, CEPKind, Result};
use crate::types::{CallResult, EventsMode, TransactionParams};
use casper_rust_wasm_sdk::types::verbosity::Verbosity;
use casper_types::U256;
use keys::{balance_dictionary_key, operator_dictionary_key, security_badge_dictionary_key};
use serde_json::Value;
use std::str::FromStr;

/// Client for CEP-85 multi-token contracts.
pub struct CEP85Client {
    core: CEPClient,
}

impl CEP85Client {
    /// Create a CEP-85 client.
    pub fn new(
        rpc_url: impl Into<String>,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<Verbosity>,
    ) -> Result<Self> {
        let core =
            CEPClient::new(rpc_url, sse_url, chain_name, verbosity)?.with_cep_kind(CEPKind::CEP85);
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
    ///
    /// Typical post-install: `cep85_contract_hash_{name}` / `cep85_contract_package_{name}`.
    pub async fn get_account_named_key(
        &self,
        account_identifier: &str,
        named_key: &str,
    ) -> Result<String> {
        self.core
            .get_account_named_key(account_identifier, named_key)
            .await
    }

    /// Install a CEP-85 contract.
    pub async fn install(
        &self,
        args: &InstallArgs,
        wasm: &[u8],
        tx: &TransactionParams,
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
        push_transfer_filter(
            &mut v,
            args.transfer_filter_contract.as_deref(),
            args.transfer_filter_method.as_deref(),
        )?;
        self.core.install_wasm(wasm, tx, &json_args(&v)).await
    }

    /// Upgrade with `upgrade: true`.
    pub async fn upgrade(
        &self,
        args: &UpgradeArgs,
        wasm: &[u8],
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut v = vec![string_arg("name", &args.name), bool_arg("upgrade", true)];
        push_transfer_filter(
            &mut v,
            args.transfer_filter_contract.as_deref(),
            args.transfer_filter_method.as_deref(),
        )?;
        self.core.install_wasm(wasm, tx, &json_args(&v)).await
    }

    /// Mint one token id.
    pub async fn mint(
        &self,
        recipient: &str,
        id: &str,
        amount: &str,
        uri: Option<&str>,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut v = vec![
            key_arg("recipient", &prefixed_key(recipient)?),
            u256_arg("id", id),
            u256_arg("amount", amount),
        ];
        if let Some(uri) = uri {
            v.push(string_arg("uri", uri));
        }
        self.core.call_entrypoint("mint", tx, &json_args(&v)).await
    }

    /// Batch mint.
    pub async fn batch_mint(
        &self,
        recipient: &str,
        ids: &[&str],
        amounts: &[&str],
        uri: Option<&str>,
        tx: &TransactionParams,
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
            .call_entrypoint("batch_mint", tx, &json_args(&v))
            .await
    }

    /// Burn one id.
    pub async fn burn(
        &self,
        owner: &str,
        id: &str,
        amount: &str,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let v = vec![
            key_arg("owner", &prefixed_key(owner)?),
            u256_arg("id", id),
            u256_arg("amount", amount),
        ];
        self.core.call_entrypoint("burn", tx, &json_args(&v)).await
    }

    /// Batch burn.
    pub async fn batch_burn(
        &self,
        owner: &str,
        ids: &[&str],
        amounts: &[&str],
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let v = vec![
            key_arg("owner", &prefixed_key(owner)?),
            u256_list_arg("ids", ids),
            u256_list_arg("amounts", amounts),
        ];
        self.core
            .call_entrypoint("batch_burn", tx, &json_args(&v))
            .await
    }

    /// Transfer (on-chain `transfer_from`), optional receiver `data`.
    pub async fn transfer(
        &self,
        from: &str,
        to: &str,
        id: &str,
        amount: &str,
        data: Option<&[u8]>,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let v = vec![
            key_arg("from", &prefixed_key(from)?),
            key_arg("to", &prefixed_key(to)?),
            u256_arg("id", id),
            u256_arg("amount", amount),
            option_byte_list_arg("data", data),
        ];
        self.core
            .call_entrypoint("transfer_from", tx, &json_args(&v))
            .await
    }

    /// Batch transfer (on-chain `batch_transfer_from`), optional receiver `data`.
    pub async fn batch_transfer(
        &self,
        from: &str,
        to: &str,
        ids: &[&str],
        amounts: &[&str],
        data: Option<&[u8]>,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let v = vec![
            key_arg("from", &prefixed_key(from)?),
            key_arg("to", &prefixed_key(to)?),
            u256_list_arg("ids", ids),
            u256_list_arg("amounts", amounts),
            option_byte_list_arg("data", data),
        ];
        self.core
            .call_entrypoint("batch_transfer_from", tx, &json_args(&v))
            .await
    }

    /// Set approval for all.
    pub async fn set_approval_for_all(
        &self,
        operator: &str,
        approved: bool,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let v = vec![
            key_arg("operator", &prefixed_key(operator)?),
            bool_arg("approved", approved),
        ];
        self.core
            .call_entrypoint("set_approval_for_all", tx, &json_args(&v))
            .await
    }

    /// Set URI (optionally per id).
    pub async fn set_uri(
        &self,
        uri: &str,
        id: Option<&str>,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut v = vec![string_arg("uri", uri)];
        if let Some(id) = id {
            v.push(u256_arg("id", id));
        }
        self.core
            .call_entrypoint("set_uri", tx, &json_args(&v))
            .await
    }

    /// Set total supply cap for one id.
    pub async fn set_total_supply_of(
        &self,
        id: &str,
        total_supply: &str,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let v = vec![u256_arg("id", id), u256_arg("total_supply", total_supply)];
        self.core
            .call_entrypoint("set_total_supply_of", tx, &json_args(&v))
            .await
    }

    /// Set total supply caps in batch.
    pub async fn set_total_supply_of_batch(
        &self,
        ids: &[&str],
        total_supplies: &[&str],
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let v = vec![
            u256_list_arg("ids", ids),
            u256_list_arg("total_supplies", total_supplies),
        ];
        self.core
            .call_entrypoint("set_total_supply_of_batch", tx, &json_args(&v))
            .await
    }

    /// Change security lists.
    pub async fn change_security(
        &self,
        args: &ChangeSecurityArgs,
        tx: &TransactionParams,
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
            return Err(CEPError::MissingArgument(
                "change_security requires at least one list".into(),
            ));
        }
        self.core
            .call_entrypoint("change_security", tx, &json_args(&v))
            .await
    }

    /// Set modalities (burn / events).
    pub async fn set_modalities(
        &self,
        enable_burn: Option<bool>,
        events_mode: Option<EventsMode>,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut v = Vec::new();
        if let Some(b) = enable_burn {
            v.push(bool_arg("enable_burn", b));
        }
        if let Some(m) = events_mode {
            v.push(u8_arg("events_mode", m.into()));
        }
        if v.is_empty() {
            return Err(CEPError::MissingArgument(
                "set_modalities requires enable_burn and/or events_mode".into(),
            ));
        }
        self.core
            .call_entrypoint("set_modalities", tx, &json_args(&v))
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
        match self.core.query_dictionary("balances", &item).await {
            Ok(raw) => decode_u256_cl(raw),
            Err(CEPError::EmptyQuery(_)) | Err(CEPError::Sdk(_)) => Ok("0".into()),
            Err(e) => Err(e),
        }
    }

    /// Batch balances: parallel dict reads for each `(account, id)` pair (same length).
    pub async fn balance_of_batch(&self, accounts: &[&str], ids: &[&str]) -> Result<Vec<String>> {
        if accounts.len() != ids.len() {
            return Err(CEPError::InvalidArgument(
                "balance_of_batch: accounts and ids length mismatch".into(),
            ));
        }
        let mut out = Vec::with_capacity(accounts.len());
        for (account, id) in accounts.iter().zip(ids.iter()) {
            out.push(self.balance_of(account, id).await?);
        }
        Ok(out)
    }

    /// Whether `operator` is approved for all of `owner`.
    pub async fn is_approved_for_all(&self, owner: &str, operator: &str) -> Result<bool> {
        let item = operator_dictionary_key(owner, operator)?;
        match self.core.query_dictionary("operators", &item).await {
            Ok(raw) => decode_bool_cl(raw),
            Err(CEPError::EmptyQuery(_)) | Err(CEPError::Sdk(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Circulating supply for `id`.
    pub async fn supply_of(&self, id: &str) -> Result<String> {
        match self.core.query_dictionary("supply", id).await {
            Ok(raw) => decode_u256_cl(raw),
            Err(CEPError::EmptyQuery(_)) | Err(CEPError::Sdk(_)) => Ok("0".into()),
            Err(e) => Err(e),
        }
    }

    /// Batch circulating supplies.
    pub async fn supply_of_batch(&self, ids: &[&str]) -> Result<Vec<String>> {
        let mut out = Vec::with_capacity(ids.len());
        for id in ids {
            out.push(self.supply_of(id).await?);
        }
        Ok(out)
    }

    /// Total supply cap for `id`.
    pub async fn total_supply_of(&self, id: &str) -> Result<String> {
        decode_u256_cl(self.core.query_dictionary("total_supply", id).await?)
    }

    /// Batch total supply caps.
    pub async fn total_supply_of_batch(&self, ids: &[&str]) -> Result<Vec<String>> {
        let mut out = Vec::with_capacity(ids.len());
        for id in ids {
            out.push(self.total_supply_of(id).await?);
        }
        Ok(out)
    }

    /// Remaining mintable fungible amount for `id` (`total_supply_of - supply_of`).
    ///
    /// Mirrors the on-chain `total_fungible_supply` view without calling the entrypoint.
    /// Returns `None` when the total-supply cap is unset or zero.
    pub async fn total_fungible_supply(&self, id: &str) -> Result<Option<String>> {
        let cap = match self.core.query_dictionary("total_supply", id).await {
            Ok(raw) => decode_u256_cl(raw)?,
            Err(CEPError::EmptyQuery(_)) | Err(CEPError::Sdk(_)) => return Ok(None),
            Err(e) => return Err(e),
        };
        let cap_u = U256::from_str(&cap)
            .map_err(|e| CEPError::Decode(format!("total_supply U256: {e}")))?;
        if cap_u.is_zero() {
            return Ok(None);
        }
        let circulating = self.supply_of(id).await?;
        let circ_u = U256::from_str(&circulating)
            .map_err(|e| CEPError::Decode(format!("supply U256: {e}")))?;
        let remaining = if cap_u >= circ_u {
            cap_u.checked_sub(circ_u).unwrap_or_else(U256::zero)
        } else {
            U256::zero()
        };
        Ok(Some(remaining.to_string()))
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

    /// Whether burn is enabled.
    pub async fn enable_burn(&self) -> Result<bool> {
        decode_bool_cl(self.core.query_contract_key(&["enable_burn"]).await?)
    }

    /// Events mode named key.
    pub async fn events_mode(&self) -> Result<EventsMode> {
        let v = decode_u8_cl(self.core.query_contract_key(&["events_mode"]).await?)?;
        EventsMode::from_u8(v).ok_or_else(|| CEPError::Decode(format!("unknown events_mode {v}")))
    }

    /// Number of minted token ids (named key).
    pub async fn number_of_minted_tokens(&self) -> Result<u64> {
        decode_u64_cl(
            self.core
                .query_contract_key(&["number_of_minted_tokens"])
                .await?,
        )
    }

    /// Transfer-filter contract key when set.
    pub async fn transfer_filter_contract(&self) -> Result<Option<String>> {
        match self
            .core
            .query_contract_key(&["transfer_filter_contract"])
            .await
        {
            Ok(raw) => decode_optional_key_cl(raw),
            Err(CEPError::EmptyQuery(_)) | Err(CEPError::Sdk(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Transfer-filter method name when set.
    pub async fn transfer_filter_method(&self) -> Result<Option<String>> {
        match self
            .core
            .query_contract_key(&["transfer_filter_method"])
            .await
        {
            Ok(raw) => decode_optional_string_cl(raw),
            Err(CEPError::EmptyQuery(_)) | Err(CEPError::Sdk(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Security badge for `entity`, if present.
    pub async fn security_badge(&self, entity: &str) -> Result<Option<SecurityBadge85>> {
        let item = security_badge_dictionary_key(entity)?;
        match self.core.query_dictionary("security_badges", &item).await {
            Ok(raw) => {
                let v = decode_u8_cl(raw)?;
                Ok(SecurityBadge85::from_u8(v))
            }
            Err(CEPError::EmptyQuery(_)) | Err(CEPError::Sdk(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

fn map_keys(list: &[String]) -> Result<Vec<String>> {
    list.iter().map(|k| prefixed_key(k)).collect()
}

fn push_transfer_filter(
    v: &mut Vec<JsonArg>,
    contract: Option<&str>,
    method: Option<&str>,
) -> Result<()> {
    match (contract, method) {
        (None, None) => Ok(()),
        (Some(c), Some(m)) if !m.is_empty() => {
            v.push(key_arg("transfer_filter_contract", &prefixed_key(c)?));
            v.push(string_arg("transfer_filter_method", m));
            Ok(())
        }
        (Some(_), _) => Err(CEPError::MissingArgument(
            "transfer_filter_method required when transfer_filter_contract is set".into(),
        )),
        (None, Some(_)) => Err(CEPError::MissingArgument(
            "transfer_filter_contract required when transfer_filter_method is set".into(),
        )),
    }
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
    Err(CEPError::Decode(format!("expected string, got {value}")))
}

fn decode_optional_string_cl(value: Value) -> Result<Option<String>> {
    let parsed = value
        .pointer("/stored_value/CLValue/parsed")
        .or_else(|| value.pointer("/CLValue/parsed"))
        .cloned()
        .unwrap_or(value);
    match parsed {
        Value::Null => Ok(None),
        Value::String(s) => Ok(Some(s)),
        other => Err(CEPError::Decode(format!(
            "expected Option<String>, got {other}"
        ))),
    }
}

fn decode_optional_key_cl(value: Value) -> Result<Option<String>> {
    let parsed = value
        .pointer("/stored_value/CLValue/parsed")
        .or_else(|| value.pointer("/CLValue/parsed"))
        .cloned()
        .unwrap_or(value);
    match parsed {
        Value::Null => Ok(None),
        Value::String(s) => Ok(Some(s)),
        Value::Object(map) => {
            if let Some(Value::String(s)) = map.get("Some").or_else(|| map.get("some")) {
                return Ok(Some(s.clone()));
            }
            if map.contains_key("None") || map.contains_key("none") {
                return Ok(None);
            }
            Err(CEPError::Decode(format!(
                "expected Option<Key> object, got {map:?}"
            )))
        }
        other => Err(CEPError::Decode(format!(
            "expected Option<Key>, got {other}"
        ))),
    }
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
        Value::Number(n) => Ok(n.as_u64().unwrap_or(0) != 0),
        other => Err(CEPError::Decode(format!("expected bool, got {other}"))),
    }
}

fn decode_u8_cl(value: Value) -> Result<u8> {
    let parsed = value
        .pointer("/stored_value/CLValue/parsed")
        .or_else(|| value.pointer("/CLValue/parsed"))
        .cloned()
        .unwrap_or(value);
    match parsed {
        Value::Number(n) => n
            .as_u64()
            .and_then(|v| u8::try_from(v).ok())
            .ok_or_else(|| CEPError::Decode(format!("invalid u8: {n}"))),
        Value::String(s) => s
            .parse()
            .map_err(|e| CEPError::Decode(format!("invalid u8 string: {e}"))),
        other => Err(CEPError::Decode(format!("expected u8, got {other}"))),
    }
}

fn decode_u64_cl(value: Value) -> Result<u64> {
    let parsed = value
        .pointer("/stored_value/CLValue/parsed")
        .or_else(|| value.pointer("/CLValue/parsed"))
        .cloned()
        .unwrap_or(value);
    match parsed {
        Value::Number(n) => n
            .as_u64()
            .ok_or_else(|| CEPError::Decode(format!("invalid u64: {n}"))),
        Value::String(s) => s
            .parse()
            .map_err(|e| CEPError::Decode(format!("invalid u64 string: {e}"))),
        other => Err(CEPError::Decode(format!("expected u64, got {other}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_client() {
        let client =
            CEP85Client::new("http://127.0.0.1:11101", None, None, Some(Verbosity::Low)).unwrap();
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
            .with_events_mode(EventsMode::CES)
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
    fn transfer_filter_requires_method() {
        let mut v = Vec::new();
        let err = push_transfer_filter(&mut v, Some("hash-aa"), None).unwrap_err();
        assert!(matches!(err, CEPError::MissingArgument(_)));
    }

    #[test]
    fn total_fungible_arithmetic_edges() {
        let cap = U256::from(10u64);
        let circ = U256::from(3u64);
        assert_eq!(cap.checked_sub(circ).unwrap().to_string(), "7");
        assert!(U256::zero().is_zero());
    }

    #[tokio::test]
    async fn make_only_install_returns_transaction_json() {
        let client = CEP85Client::new("http://127.0.0.1:11101", None, None, None).unwrap();
        let tx = TransactionParams::for_make("1000000000").with_initiator_addr(
            "010101010101010101010101010101010101010101010101010101010101010101",
        );
        let wasm = [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
        let args = InstallArgs::new("Bag", "https://x/{id}.json");
        let result = client
            .install(&args, &wasm, &tx)
            .await
            .expect("make-only install");
        assert!(result.put_result.is_null());
        assert!(result.transaction.is_some());
        assert!(!result.transaction_hash.is_empty());
    }

    #[tokio::test]
    async fn make_only_mint_returns_transaction_json() {
        let mut client = CEP85Client::new("http://127.0.0.1:11101", None, None, None).unwrap();
        client
            .set_contract_hash(
                "cfa781f5eb69c3eee952c2944ce9670a049f88c5e46b83fb5881ebe13fb98e6d",
                None::<&str>,
            )
            .unwrap();
        let tx = TransactionParams::for_make("1000000000").with_initiator_addr(
            "010101010101010101010101010101010101010101010101010101010101010101",
        );
        let result = client
            .mint(
                "account-hash-b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f",
                "1",
                "10",
                None,
                &tx,
            )
            .await
            .expect("make-only mint");
        assert!(result.put_result.is_null());
        assert!(result.transaction.expect("json").is_object());
    }

    #[test]
    fn decode_bool_from_stored_value() {
        let v = serde_json::json!({
            "stored_value": { "CLValue": { "parsed": true } }
        });
        assert!(decode_bool_cl(v).unwrap());
    }

    #[test]
    fn security_badge85_roundtrip() {
        assert_eq!(SecurityBadge85::from_u8(2).unwrap().as_str(), "Burner");
        assert!(SecurityBadge85::from_u8(9).is_none());
    }
}
