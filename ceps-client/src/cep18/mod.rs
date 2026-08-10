//! CEP-18 fungible token client.

mod entity;
mod error;
mod keys;
mod types;

pub use error::CEP18Error;
pub use types::{ChangeSecurityArgs, InstallArgs, UpgradeArgs};

use crate::core::CEPClient;
use crate::core::{json_args, key_arg, key_list_arg, string_arg, u256_arg, u8_arg, JsonArg};
use crate::error::{CEPError, CEPKind, Result};
use crate::types::{CallResult, EventsMode, TransactionParams};
use casper_rust_wasm_sdk::types::verbosity::Verbosity;
use entity::prefixed_key;
use keys::{allowance_dictionary_key, balance_dictionary_key};
use serde_json::Value;

/// Client for CEP-18 fungible token contracts.
pub struct CEP18Client {
    core: CEPClient,
}

impl CEP18Client {
    /// Create a CEP-18 client.
    pub fn new(
        rpc_url: impl Into<String>,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<Verbosity>,
    ) -> Result<Self> {
        let core =
            CEPClient::new(rpc_url, sse_url, chain_name, verbosity)?.with_cep_kind(CEPKind::CEP18);
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
    /// Typical post-install: `cep18_contract_hash_{name}` / `cep18_contract_package_{name}`.
    pub async fn get_account_named_key(
        &self,
        account_identifier: &str,
        named_key: &str,
    ) -> Result<String> {
        self.core
            .get_account_named_key(account_identifier, named_key)
            .await
    }

    /// Install a CEP-18 contract from WASM bytes.
    pub async fn install(
        &self,
        args: &InstallArgs,
        wasm: &[u8],
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let args_json = install_args_json(args)?;
        self.core.install_wasm(wasm, tx, &args_json).await
    }

    /// Upgrade an existing CEP-18 package.
    pub async fn upgrade(
        &self,
        args: &UpgradeArgs,
        wasm: &[u8],
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut json_args_vec = vec![string_arg("name", &args.name)];
        if let Some(mode) = args.events_mode {
            json_args_vec.push(u8_arg("events_mode", mode.into()));
        }
        let args_json = json_args(&json_args_vec);
        self.core.install_wasm(wasm, tx, &args_json).await
    }

    /// Transfer tokens to `recipient`.
    pub async fn transfer(
        &self,
        recipient: &str,
        amount: &str,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let args = json_args(&[
            key_arg("recipient", &prefixed_key(recipient)?),
            u256_arg("amount", amount),
        ]);
        self.core.call_entrypoint("transfer", tx, &args).await
    }

    /// Transfer tokens from `owner` to `recipient` using allowance.
    pub async fn transfer_from(
        &self,
        owner: &str,
        recipient: &str,
        amount: &str,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let args = json_args(&[
            key_arg("owner", &prefixed_key(owner)?),
            key_arg("recipient", &prefixed_key(recipient)?),
            u256_arg("amount", amount),
        ]);
        self.core.call_entrypoint("transfer_from", tx, &args).await
    }

    /// Approve `spender` for `amount`.
    pub async fn approve(
        &self,
        spender: &str,
        amount: &str,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let args = json_args(&[
            key_arg("spender", &prefixed_key(spender)?),
            u256_arg("amount", amount),
        ]);
        self.core.call_entrypoint("approve", tx, &args).await
    }

    /// Increase allowance for `spender`.
    pub async fn increase_allowance(
        &self,
        spender: &str,
        amount: &str,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let args = json_args(&[
            key_arg("spender", &prefixed_key(spender)?),
            u256_arg("amount", amount),
        ]);
        self.core
            .call_entrypoint("increase_allowance", tx, &args)
            .await
    }

    /// Decrease allowance for `spender`.
    pub async fn decrease_allowance(
        &self,
        spender: &str,
        amount: &str,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let args = json_args(&[
            key_arg("spender", &prefixed_key(spender)?),
            u256_arg("amount", amount),
        ]);
        self.core
            .call_entrypoint("decrease_allowance", tx, &args)
            .await
    }

    /// Mint tokens to `owner`.
    pub async fn mint(
        &self,
        owner: &str,
        amount: &str,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let args = json_args(&[
            key_arg("owner", &prefixed_key(owner)?),
            u256_arg("amount", amount),
        ]);
        self.core.call_entrypoint("mint", tx, &args).await
    }

    /// Burn tokens from `owner`.
    pub async fn burn(
        &self,
        owner: &str,
        amount: &str,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let args = json_args(&[
            key_arg("owner", &prefixed_key(owner)?),
            u256_arg("amount", amount),
        ]);
        self.core.call_entrypoint("burn", tx, &args).await
    }

    /// Change security lists (at least one list required).
    pub async fn change_security(
        &self,
        args: &ChangeSecurityArgs,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut json_args_vec: Vec<JsonArg> = Vec::new();
        if let Some(list) = &args.admin_list {
            let keys = map_keys(list)?;
            json_args_vec.push(key_list_arg("admin_list", &keys));
        }
        if let Some(list) = &args.minter_list {
            let keys = map_keys(list)?;
            json_args_vec.push(key_list_arg("minter_list", &keys));
        }
        if let Some(list) = &args.none_list {
            let keys = map_keys(list)?;
            json_args_vec.push(key_list_arg("none_list", &keys));
        }
        if json_args_vec.is_empty() {
            return Err(CEPError::MissingArgument(
                "change_security requires at least one list".into(),
            ));
        }
        let args_json = json_args(&json_args_vec);
        self.core
            .call_entrypoint("change_security", tx, &args_json)
            .await
    }

    /// Change events mode.
    pub async fn change_events_mode(
        &self,
        events_mode: EventsMode,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let args = json_args(&[u8_arg("events_mode", events_mode.into())]);
        self.core
            .call_entrypoint("change_events_mode", tx, &args)
            .await
    }

    /// Token name (named key).
    pub async fn name(&self) -> Result<String> {
        decode_string_cl(self.core.query_contract_key(&["name"]).await?)
    }

    /// Token symbol (named key).
    pub async fn symbol(&self) -> Result<String> {
        decode_string_cl(self.core.query_contract_key(&["symbol"]).await?)
    }

    /// Token decimals (named key).
    pub async fn decimals(&self) -> Result<u8> {
        decode_u8_cl(self.core.query_contract_key(&["decimals"]).await?)
    }

    /// Total supply (named key).
    pub async fn total_supply(&self) -> Result<String> {
        decode_u256_cl(self.core.query_contract_key(&["total_supply"]).await?)
    }

    /// Events mode (named key).
    pub async fn events_mode(&self) -> Result<EventsMode> {
        let v = decode_u8_cl(self.core.query_contract_key(&["events_mode"]).await?)?;
        EventsMode::from_u8(v).ok_or_else(|| CEPError::Decode(format!("unknown events_mode {v}")))
    }

    /// Whether mint/burn is enabled.
    pub async fn is_mint_and_burn_enabled(&self) -> Result<bool> {
        let v = decode_u8_cl(self.core.query_contract_key(&["enable_mint_burn"]).await?)?;
        Ok(v != 0)
    }

    /// Balance of `account` (`account-hash-…`, `hash-…`, `entity-…`, or bare 64-hex as `hash-`).
    pub async fn balance_of(&self, account: &str) -> Result<String> {
        let item_key = balance_dictionary_key(account)?;
        let raw = self.core.query_dictionary("balances", &item_key).await?;
        decode_u256_cl(raw)
    }

    /// Allowance from `owner` to `spender`.
    pub async fn allowances(&self, owner: &str, spender: &str) -> Result<String> {
        let item_key = allowance_dictionary_key(owner, spender)?;
        let raw = self.core.query_dictionary("allowances", &item_key).await?;
        decode_u256_cl(raw)
    }
}

fn install_args_json(args: &InstallArgs) -> Result<String> {
    let mut v = vec![
        string_arg("name", &args.name),
        string_arg("symbol", &args.symbol),
        u8_arg("decimals", args.decimals),
        u256_arg("total_supply", &args.total_supply),
    ];
    if let Some(mode) = args.events_mode {
        v.push(u8_arg("events_mode", mode.into()));
    }
    if let Some(enable) = args.enable_mint_and_burn {
        v.push(u8_arg("enable_mint_burn", u8::from(enable)));
    }
    if let Some(list) = &args.admin_list {
        v.push(key_list_arg("admin_list", &map_keys(list)?));
    }
    if let Some(list) = &args.minter_list {
        v.push(key_list_arg("minter_list", &map_keys(list)?));
    }
    Ok(json_args(&v))
}

fn map_keys(list: &[String]) -> Result<Vec<String>> {
    list.iter().map(|k| prefixed_key(k)).collect()
}

fn decode_string_cl(value: Value) -> Result<String> {
    // Prefer parsed CLValue JSON shapes from query_global_state.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_client() {
        let client = CEP18Client::new(
            "http://127.0.0.1:11101",
            Some("http://127.0.0.1:18101".into()),
            None,
            Some(Verbosity::Low),
        )
        .unwrap();
        assert_eq!(client.rpc_url(), "http://127.0.0.1:11101/rpc");
        assert_eq!(client.sse_url(), Some("http://127.0.0.1:18101/events"));
    }

    #[test]
    fn install_json_includes_required_and_flags() {
        let args = InstallArgs::new("Tok", "TOK", 9, "1000")
            .with_events_mode(EventsMode::CES)
            .with_mint_and_burn(true);
        let s = install_args_json(&args).unwrap();
        assert!(s.contains("Tok"));
        assert!(s.contains("events_mode"));
        assert!(s.contains("enable_mint_burn"));
    }

    #[test]
    fn decode_string_from_stored_value() {
        let v = serde_json::json!({
            "stored_value": { "CLValue": { "parsed": "hello" } }
        });
        assert_eq!(decode_string_cl(v).unwrap(), "hello");
    }

    #[test]
    fn decode_u256_from_number() {
        let v = serde_json::json!({ "CLValue": { "parsed": 42 } });
        assert_eq!(decode_u256_cl(v).unwrap(), "42");
    }

    #[tokio::test]
    async fn make_only_install_returns_transaction_json() {
        let client = CEP18Client::new("http://127.0.0.1:11101", None, None, None).unwrap();
        let tx = TransactionParams::for_make("1000000000").with_initiator_addr(
            "010101010101010101010101010101010101010101010101010101010101010101",
        );
        let wasm = [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
        let args = InstallArgs::new("Tok", "TOK", 9, "1000");
        let result = client
            .install(&args, &wasm, &tx)
            .await
            .expect("make-only install");
        assert!(result.put_result.is_null());
        assert!(result.transaction.is_some());
        assert!(!result.transaction_hash.is_empty());
        assert!(result.execution_result.is_none());
    }

    #[tokio::test]
    async fn make_only_transfer_returns_transaction_json() {
        let mut client = CEP18Client::new("http://127.0.0.1:11101", None, None, None).unwrap();
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
            .transfer(
                "account-hash-b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f",
                "1",
                &tx,
            )
            .await
            .expect("make-only transfer");
        assert!(result.put_result.is_null());
        assert!(result.transaction.expect("json").is_object());
    }
}
