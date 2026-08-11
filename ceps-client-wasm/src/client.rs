//! Thin wasm-bindgen surface over `ceps-client` (CEP APIs only).

use ceps_client::cep18::InstallArgs as CEP18InstallArgs;
use ceps_client::cep78::InstallArgs as CEP78InstallArgs;
use ceps_client::cep85::InstallArgs as CEP85InstallArgs;
use ceps_client::cep95::InstallArgs as CEP95InstallArgs;
use ceps_client::{
    CEP18Client, CEP78Client, CEP85Client, CEP95Client, CallResult, EventsMode, EventsMode78,
    TransactionParams, Verbosity,
};
use gloo_utils::format::JsValueSerdeExt;
use js_sys::Uint8Array;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

fn map_err(err: ceps_client::CEPError) -> JsValue {
    JsValue::from_str(&err.to_string())
}

fn verbosity_from_u8(v: Option<u8>) -> Option<Verbosity> {
    match v {
        Some(0) | None => Some(Verbosity::Low),
        Some(1) => Some(Verbosity::Medium),
        Some(2) => Some(Verbosity::High),
        _ => Some(Verbosity::Low),
    }
}

fn transaction_params(
    secret_key_pem: Option<&str>,
    payment_amount: &str,
    wait: bool,
    make_only: bool,
    initiator_addr: Option<&str>,
) -> Result<TransactionParams, JsValue> {
    let mut tx = match secret_key_pem.filter(|s| !s.trim().is_empty()) {
        Some(pem) => TransactionParams::new(pem, payment_amount),
        None => TransactionParams::for_make(payment_amount),
    };
    if make_only {
        tx = tx.make_only();
    }
    if !wait {
        tx = tx.without_wait();
    }
    if let Some(addr) = initiator_addr.filter(|s| !s.trim().is_empty()) {
        tx = tx.with_initiator_addr(addr);
    }
    tx.validate().map_err(|e| JsValue::from_str(&e))?;
    Ok(tx)
}

fn call_result_json(result: CallResult) -> Result<String, JsValue> {
    serde_json::to_string(&serde_json::json!({
        "transactionHash": result.transaction_hash,
        "hasExecutionResult": result.execution_result.is_some(),
        "cesEvents": result.ces_events,
        "putResult": result.put_result,
        "transaction": result.transaction,
    }))
    .map_err(|e| JsValue::from_str(&e.to_string()))
}

fn bytes_from_js(wasm: &Uint8Array) -> Vec<u8> {
    wasm.to_vec()
}

#[derive(Debug, Deserialize)]
struct Cep85InstallArgsJs {
    name: String,
    uri: String,
    #[serde(default)]
    events_mode: Option<u8>,
    #[serde(default)]
    enable_burn: Option<bool>,
    #[serde(default)]
    transfer_filter_contract: Option<String>,
    #[serde(default)]
    transfer_filter_method: Option<String>,
    #[serde(default)]
    secret_key_pem: Option<String>,
    payment_amount: String,
    #[serde(default)]
    wait: Option<bool>,
    #[serde(default)]
    make_only: Option<bool>,
    #[serde(default)]
    initiator_addr: Option<String>,
}

/// WASM wrapper for [`CEP18Client`].
#[wasm_bindgen(js_name = CEP18Client)]
pub struct WasmCEP18Client {
    inner: CEP18Client,
}

#[wasm_bindgen(js_class = CEP18Client)]
impl WasmCEP18Client {
    /// Create a CEP-18 client.
    #[wasm_bindgen(constructor)]
    pub fn new(
        rpc_url: String,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<u8>,
    ) -> Result<WasmCEP18Client, JsValue> {
        let inner = CEP18Client::new(rpc_url, sse_url, chain_name, verbosity_from_u8(verbosity))
            .map_err(map_err)?;
        Ok(Self { inner })
    }

    /// RPC URL.
    #[wasm_bindgen(js_name = rpcUrl)]
    pub fn rpc_url(&self) -> String {
        self.inner.rpc_url().to_string()
    }

    /// SSE URL when set.
    #[wasm_bindgen(js_name = SSEUrl)]
    pub fn sse_url(&self) -> Option<String> {
        self.inner.sse_url().map(str::to_string)
    }

    /// Chain name.
    #[wasm_bindgen(js_name = chainName)]
    pub fn chain_name(&self) -> String {
        self.inner.chain_name().to_string()
    }

    /// Bind contract hashes (hex or prefixed).
    #[wasm_bindgen(js_name = setContractHash)]
    pub fn set_contract_hash(
        &mut self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> Result<(), JsValue> {
        self.inner
            .set_contract_hash(contract_hash, package_hash)
            .map_err(map_err)
    }

    /// Install with required fields (+ optional CES events when `events_mode` is set).
    #[wasm_bindgen]
    #[allow(clippy::too_many_arguments)]
    pub async fn install(
        &self,
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: String,
        events_mode: Option<u8>,
        wasm: Uint8Array,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wait: Option<bool>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> Result<String, JsValue> {
        let mut args = CEP18InstallArgs::new(name, symbol, decimals, total_supply);
        if let Some(mode) = events_mode {
            let mode = EventsMode::from_u8(mode)
                .ok_or_else(|| JsValue::from_str("invalid events_mode"))?;
            args = args.with_events_mode(mode);
        }
        let tx = transaction_params(
            secret_key_pem.as_deref(),
            &payment_amount,
            wait.unwrap_or(true),
            make_only.unwrap_or(false),
            initiator_addr.as_deref(),
        )?;
        let put = self
            .inner
            .install(&args, &bytes_from_js(&wasm), &tx)
            .await
            .map_err(map_err)?;
        call_result_json(put)
    }

    /// Token name.
    #[wasm_bindgen]
    pub async fn name(&self) -> Result<String, JsValue> {
        self.inner.name().await.map_err(map_err)
    }

    /// Token symbol.
    #[wasm_bindgen]
    pub async fn symbol(&self) -> Result<String, JsValue> {
        self.inner.symbol().await.map_err(map_err)
    }

    /// Balance of `account` (`account-hash-…` or prefixed).
    #[wasm_bindgen(js_name = balanceOf)]
    pub async fn balance_of(&self, account: String) -> Result<String, JsValue> {
        self.inner.balance_of(&account).await.map_err(map_err)
    }

    /// Security badge name for `account`, or `undefined` when unset.
    #[wasm_bindgen(js_name = securityBadge)]
    pub async fn security_badge(&self, account: String) -> Result<Option<String>, JsValue> {
        Ok(self
            .inner
            .security_badge(&account)
            .await
            .map_err(map_err)?
            .map(|b| b.as_str().to_string()))
    }

    /// Put signed Transaction JSON (`CEPClient::put_transaction`).
    #[wasm_bindgen(js_name = putTransaction)]
    pub async fn put_transaction(
        &self,
        transaction_json: String,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
    ) -> Result<String, JsValue> {
        core_put_transaction(
            self.inner.core(),
            &transaction_json,
            wait.unwrap_or(true),
            wait_timeout_ms,
        )
        .await
    }

    /// Wait for a transaction hash on SSE.
    #[wasm_bindgen(js_name = waitTransaction)]
    pub async fn wait_transaction(
        &self,
        transaction_hash: String,
        timeout_ms: Option<u64>,
    ) -> Result<String, JsValue> {
        core_wait_transaction(self.inner.core(), &transaction_hash, timeout_ms).await
    }

    /// Parse CES events for a transaction against the bound contract.
    #[wasm_bindgen(js_name = parseCES)]
    pub async fn parse_ces(&self, transaction_hash: String) -> Result<String, JsValue> {
        core_parse_ces(self.inner.core(), &transaction_hash).await
    }

    /// Collect SSE processed frames and decode CES for the bound contract.
    #[wasm_bindgen(js_name = collectCESEvents)]
    pub async fn collect_ces_events(
        &self,
        event_names: Vec<String>,
        max_transactions: Option<u32>,
        timeout_ms: Option<u64>,
    ) -> Result<String, JsValue> {
        core_collect_ces(
            self.inner.core(),
            &event_names,
            max_transactions.unwrap_or(8) as usize,
            timeout_ms.unwrap_or(120_000),
        )
        .await
    }
}

/// WASM wrapper for [`CEP78Client`].
#[wasm_bindgen(js_name = CEP78Client)]
pub struct WasmCEP78Client {
    inner: CEP78Client,
}

#[wasm_bindgen(js_class = CEP78Client)]
impl WasmCEP78Client {
    /// Create a CEP-78 client.
    #[wasm_bindgen(constructor)]
    pub fn new(
        rpc_url: String,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<u8>,
    ) -> Result<WasmCEP78Client, JsValue> {
        let inner = CEP78Client::new(rpc_url, sse_url, chain_name, verbosity_from_u8(verbosity))
            .map_err(map_err)?;
        Ok(Self { inner })
    }

    /// RPC URL.
    #[wasm_bindgen(js_name = rpcUrl)]
    pub fn rpc_url(&self) -> String {
        self.inner.rpc_url().to_string()
    }

    /// SSE URL when set.
    #[wasm_bindgen(js_name = SSEUrl)]
    pub fn sse_url(&self) -> Option<String> {
        self.inner.sse_url().map(str::to_string)
    }

    /// Bind contract hashes.
    #[wasm_bindgen(js_name = setContractHash)]
    pub fn set_contract_hash(
        &mut self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> Result<(), JsValue> {
        self.inner
            .set_contract_hash(contract_hash, package_hash)
            .map_err(map_err)
    }

    /// Install with defaults (Transferable / Raw / Ordinal) and optional events mode.
    #[wasm_bindgen]
    #[allow(clippy::too_many_arguments)]
    pub async fn install(
        &self,
        collection_name: String,
        collection_symbol: String,
        total_token_supply: u64,
        events_mode: Option<u8>,
        wasm: Uint8Array,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wait: Option<bool>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> Result<String, JsValue> {
        let mut args =
            CEP78InstallArgs::new(collection_name, collection_symbol, total_token_supply);
        if let Some(mode) = events_mode {
            let mode = EventsMode78::from_u8(mode)
                .ok_or_else(|| JsValue::from_str("invalid events_mode"))?;
            args = args.with_events_mode(mode);
        }
        let tx = transaction_params(
            secret_key_pem.as_deref(),
            &payment_amount,
            wait.unwrap_or(true),
            make_only.unwrap_or(false),
            initiator_addr.as_deref(),
        )?;
        let put = self
            .inner
            .install(&args, &bytes_from_js(&wasm), &tx)
            .await
            .map_err(map_err)?;
        call_result_json(put)
    }

    /// Collection name.
    #[wasm_bindgen(js_name = collectionName)]
    pub async fn collection_name(&self) -> Result<String, JsValue> {
        self.inner.collection_name().await.map_err(map_err)
    }

    /// Balance of owner.
    #[wasm_bindgen(js_name = balanceOf)]
    pub async fn balance_of(&self, owner: String) -> Result<String, JsValue> {
        self.inner.balance_of(&owner).await.map_err(map_err)
    }

    /// Ownership mode (`u8`).
    #[wasm_bindgen(js_name = ownershipMode)]
    pub async fn ownership_mode(&self) -> Result<u8, JsValue> {
        self.inner
            .ownership_mode()
            .await
            .map(u8::from)
            .map_err(map_err)
    }

    /// Parse CES events for a transaction against the bound contract.
    #[wasm_bindgen(js_name = parseCES)]
    pub async fn parse_ces(&self, transaction_hash: String) -> Result<String, JsValue> {
        core_parse_ces(self.inner.core(), &transaction_hash).await
    }

    /// Put signed Transaction JSON (`CEPClient::put_transaction`).
    #[wasm_bindgen(js_name = putTransaction)]
    pub async fn put_transaction(
        &self,
        transaction_json: String,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
    ) -> Result<String, JsValue> {
        core_put_transaction(
            self.inner.core(),
            &transaction_json,
            wait.unwrap_or(true),
            wait_timeout_ms,
        )
        .await
    }

    /// Wait for a transaction hash on SSE.
    #[wasm_bindgen(js_name = waitTransaction)]
    pub async fn wait_transaction(
        &self,
        transaction_hash: String,
        timeout_ms: Option<u64>,
    ) -> Result<String, JsValue> {
        core_wait_transaction(self.inner.core(), &transaction_hash, timeout_ms).await
    }

    /// Collect SSE processed frames and decode CES for the bound contract.
    #[wasm_bindgen(js_name = collectCESEvents)]
    pub async fn collect_ces_events(
        &self,
        event_names: Vec<String>,
        max_transactions: Option<u32>,
        timeout_ms: Option<u64>,
    ) -> Result<String, JsValue> {
        core_collect_ces(
            self.inner.core(),
            &event_names,
            max_transactions.unwrap_or(8) as usize,
            timeout_ms.unwrap_or(120_000),
        )
        .await
    }
}

/// WASM wrapper for [`CEP85Client`].
#[wasm_bindgen(js_name = CEP85Client)]
pub struct WasmCEP85Client {
    inner: CEP85Client,
}

#[wasm_bindgen(js_class = CEP85Client)]
impl WasmCEP85Client {
    /// Create a CEP-85 client.
    #[wasm_bindgen(constructor)]
    pub fn new(
        rpc_url: String,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<u8>,
    ) -> Result<WasmCEP85Client, JsValue> {
        let inner = CEP85Client::new(rpc_url, sse_url, chain_name, verbosity_from_u8(verbosity))
            .map_err(map_err)?;
        Ok(Self { inner })
    }

    /// RPC URL.
    #[wasm_bindgen(js_name = rpcUrl)]
    pub fn rpc_url(&self) -> String {
        self.inner.rpc_url().to_string()
    }

    /// SSE URL when set.
    #[wasm_bindgen(js_name = SSEUrl)]
    pub fn sse_url(&self) -> Option<String> {
        self.inner.sse_url().map(str::to_string)
    }

    /// Bind contract hashes.
    #[wasm_bindgen(js_name = setContractHash)]
    pub fn set_contract_hash(
        &mut self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> Result<(), JsValue> {
        self.inner
            .set_contract_hash(contract_hash, package_hash)
            .map_err(map_err)
    }

    /// Install from a JSON object (`name`, `uri`, `payment_amount`, optional events/burn/filter
    /// and tx fields) plus contract WASM bytes.
    #[wasm_bindgen]
    pub async fn install(&self, args: JsValue, wasm: Uint8Array) -> Result<String, JsValue> {
        let parsed: Cep85InstallArgsJs = args
            .into_serde()
            .map_err(|e| JsValue::from_str(&format!("install args: {e}")))?;
        let mut install_args = CEP85InstallArgs::new(parsed.name, parsed.uri);
        if let Some(mode) = parsed.events_mode {
            let mode = EventsMode::from_u8(mode)
                .ok_or_else(|| JsValue::from_str("invalid events_mode"))?;
            install_args = install_args.with_events_mode(mode);
        }
        if let Some(b) = parsed.enable_burn {
            install_args = install_args.with_enable_burn(b);
        }
        match (
            parsed.transfer_filter_contract,
            parsed.transfer_filter_method,
        ) {
            (None, None) => {}
            (Some(c), Some(m)) => {
                install_args = install_args.with_transfer_filter(c, m);
            }
            _ => {
                return Err(JsValue::from_str(
                    "transfer_filter_contract and transfer_filter_method must both be set",
                ));
            }
        }
        let tx = transaction_params(
            parsed.secret_key_pem.as_deref(),
            &parsed.payment_amount,
            parsed.wait.unwrap_or(true),
            parsed.make_only.unwrap_or(false),
            parsed.initiator_addr.as_deref(),
        )?;
        let put = self
            .inner
            .install(&install_args, &bytes_from_js(&wasm), &tx)
            .await
            .map_err(map_err)?;
        call_result_json(put)
    }

    /// Collection name.
    #[wasm_bindgen(js_name = collectionName)]
    pub async fn collection_name(&self) -> Result<String, JsValue> {
        self.inner.collection_name().await.map_err(map_err)
    }

    /// Balance for account + token id.
    #[wasm_bindgen(js_name = balanceOf)]
    pub async fn balance_of(&self, account: String, id: String) -> Result<String, JsValue> {
        self.inner.balance_of(&account, &id).await.map_err(map_err)
    }

    /// Batch balances as JSON string array.
    #[wasm_bindgen(js_name = balanceOfBatch)]
    pub async fn balance_of_batch(
        &self,
        accounts: Vec<String>,
        ids: Vec<String>,
    ) -> Result<String, JsValue> {
        let acct_refs: Vec<&str> = accounts.iter().map(String::as_str).collect();
        let id_refs: Vec<&str> = ids.iter().map(String::as_str).collect();
        let bals = self
            .inner
            .balance_of_batch(&acct_refs, &id_refs)
            .await
            .map_err(map_err)?;
        serde_json::to_string(&bals).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Batch circulating supplies as JSON string array.
    #[wasm_bindgen(js_name = supplyOfBatch)]
    pub async fn supply_of_batch(&self, ids: Vec<String>) -> Result<String, JsValue> {
        let id_refs: Vec<&str> = ids.iter().map(String::as_str).collect();
        let vals = self
            .inner
            .supply_of_batch(&id_refs)
            .await
            .map_err(map_err)?;
        serde_json::to_string(&vals).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Batch total supply caps as JSON string array.
    #[wasm_bindgen(js_name = totalSupplyOfBatch)]
    pub async fn total_supply_of_batch(&self, ids: Vec<String>) -> Result<String, JsValue> {
        let id_refs: Vec<&str> = ids.iter().map(String::as_str).collect();
        let vals = self
            .inner
            .total_supply_of_batch(&id_refs)
            .await
            .map_err(map_err)?;
        serde_json::to_string(&vals).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Remaining fungible mintable amount, or `undefined` when cap unset/zero.
    #[wasm_bindgen(js_name = totalFungibleSupply)]
    pub async fn total_fungible_supply(&self, id: String) -> Result<Option<String>, JsValue> {
        self.inner.total_fungible_supply(&id).await.map_err(map_err)
    }

    /// Whether burn is enabled.
    #[wasm_bindgen(js_name = enableBurn)]
    pub async fn enable_burn(&self) -> Result<bool, JsValue> {
        self.inner.enable_burn().await.map_err(map_err)
    }

    /// Events mode as `u8`.
    #[wasm_bindgen(js_name = eventsMode)]
    pub async fn events_mode(&self) -> Result<u8, JsValue> {
        self.inner
            .events_mode()
            .await
            .map(u8::from)
            .map_err(map_err)
    }

    /// Number of minted token ids.
    #[wasm_bindgen(js_name = numberOfMintedTokens)]
    pub async fn number_of_minted_tokens(&self) -> Result<u64, JsValue> {
        self.inner.number_of_minted_tokens().await.map_err(map_err)
    }

    /// Transfer-filter contract key when set.
    #[wasm_bindgen(js_name = transferFilterContract)]
    pub async fn transfer_filter_contract(&self) -> Result<Option<String>, JsValue> {
        self.inner.transfer_filter_contract().await.map_err(map_err)
    }

    /// Transfer-filter method when set.
    #[wasm_bindgen(js_name = transferFilterMethod)]
    pub async fn transfer_filter_method(&self) -> Result<Option<String>, JsValue> {
        self.inner.transfer_filter_method().await.map_err(map_err)
    }

    /// Security badge name for `entity`, or `undefined` when unset.
    #[wasm_bindgen(js_name = securityBadge)]
    pub async fn security_badge(&self, entity: String) -> Result<Option<String>, JsValue> {
        Ok(self
            .inner
            .security_badge(&entity)
            .await
            .map_err(map_err)?
            .map(|b| b.as_str().to_string()))
    }

    /// Put signed Transaction JSON (`CEPClient::put_transaction`).
    #[wasm_bindgen(js_name = putTransaction)]
    pub async fn put_transaction(
        &self,
        transaction_json: String,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
    ) -> Result<String, JsValue> {
        core_put_transaction(
            self.inner.core(),
            &transaction_json,
            wait.unwrap_or(true),
            wait_timeout_ms,
        )
        .await
    }

    /// Wait for a transaction hash on SSE.
    #[wasm_bindgen(js_name = waitTransaction)]
    pub async fn wait_transaction(
        &self,
        transaction_hash: String,
        timeout_ms: Option<u64>,
    ) -> Result<String, JsValue> {
        core_wait_transaction(self.inner.core(), &transaction_hash, timeout_ms).await
    }

    /// Parse CES events for a transaction against the bound contract.
    #[wasm_bindgen(js_name = parseCES)]
    pub async fn parse_ces(&self, transaction_hash: String) -> Result<String, JsValue> {
        core_parse_ces(self.inner.core(), &transaction_hash).await
    }

    /// Collect SSE processed frames and decode CES for the bound contract.
    #[wasm_bindgen(js_name = collectCESEvents)]
    pub async fn collect_ces_events(
        &self,
        event_names: Vec<String>,
        max_transactions: Option<u32>,
        timeout_ms: Option<u64>,
    ) -> Result<String, JsValue> {
        core_collect_ces(
            self.inner.core(),
            &event_names,
            max_transactions.unwrap_or(8) as usize,
            timeout_ms.unwrap_or(120_000),
        )
        .await
    }
}

/// WASM wrapper for [`CEP95Client`].
#[wasm_bindgen(js_name = CEP95Client)]
pub struct WasmCEP95Client {
    inner: CEP95Client,
}

#[wasm_bindgen(js_class = CEP95Client)]
impl WasmCEP95Client {
    /// Create a CEP-95 client.
    #[wasm_bindgen(constructor)]
    pub fn new(
        rpc_url: String,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<u8>,
    ) -> Result<WasmCEP95Client, JsValue> {
        let inner = CEP95Client::new(rpc_url, sse_url, chain_name, verbosity_from_u8(verbosity))
            .map_err(map_err)?;
        Ok(Self { inner })
    }

    /// RPC URL.
    #[wasm_bindgen(js_name = rpcUrl)]
    pub fn rpc_url(&self) -> String {
        self.inner.rpc_url().to_string()
    }

    /// SSE URL when set.
    #[wasm_bindgen(js_name = SSEUrl)]
    pub fn sse_url(&self) -> Option<String> {
        self.inner.sse_url().map(str::to_string)
    }

    /// Bind contract hashes.
    #[wasm_bindgen(js_name = setContractHash)]
    pub fn set_contract_hash(
        &mut self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> Result<(), JsValue> {
        self.inner
            .set_contract_hash(contract_hash, package_hash)
            .map_err(map_err)
    }

    /// Install Odra OwnedCEP95 (or compatible) with package named-key name.
    #[wasm_bindgen]
    #[allow(clippy::too_many_arguments)]
    pub async fn install(
        &self,
        name: String,
        symbol: String,
        package_hash_key_name: String,
        wasm: Uint8Array,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wait: Option<bool>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> Result<String, JsValue> {
        let args = CEP95InstallArgs::new(name, symbol, package_hash_key_name);
        let tx = transaction_params(
            secret_key_pem.as_deref(),
            &payment_amount,
            wait.unwrap_or(true),
            make_only.unwrap_or(false),
            initiator_addr.as_deref(),
        )?;
        let put = self
            .inner
            .install(&args, &bytes_from_js(&wasm), &tx)
            .await
            .map_err(map_err)?;
        call_result_json(put)
    }

    /// Collection name.
    #[wasm_bindgen]
    pub async fn name(&self) -> Result<String, JsValue> {
        self.inner.name().await.map_err(map_err)
    }

    /// Collection symbol.
    #[wasm_bindgen]
    pub async fn symbol(&self) -> Result<String, JsValue> {
        self.inner.symbol().await.map_err(map_err)
    }

    /// Balance of owner.
    #[wasm_bindgen(js_name = balanceOf)]
    pub async fn balance_of(&self, owner: String) -> Result<String, JsValue> {
        self.inner.balance_of(&owner).await.map_err(map_err)
    }

    /// Owner of token id.
    #[wasm_bindgen(js_name = ownerOf)]
    pub async fn owner_of(&self, token_id: String) -> Result<String, JsValue> {
        self.inner.owner_of(&token_id).await.map_err(map_err)
    }

    /// Ownable contract owner.
    #[wasm_bindgen(js_name = getOwner)]
    pub async fn get_owner(&self) -> Result<String, JsValue> {
        self.inner.get_owner().await.map_err(map_err)
    }

    /// Transfer Ownable ownership.
    #[wasm_bindgen(js_name = transferOwnership)]
    pub async fn transfer_ownership(
        &self,
        new_owner: String,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wait: Option<bool>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> Result<String, JsValue> {
        let tx = transaction_params(
            secret_key_pem.as_deref(),
            &payment_amount,
            wait.unwrap_or(true),
            make_only.unwrap_or(false),
            initiator_addr.as_deref(),
        )?;
        let put = self
            .inner
            .transfer_ownership(&new_owner, &tx)
            .await
            .map_err(map_err)?;
        call_result_json(put)
    }

    /// Put signed Transaction JSON (`CEPClient::put_transaction`).
    #[wasm_bindgen(js_name = putTransaction)]
    pub async fn put_transaction(
        &self,
        transaction_json: String,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
    ) -> Result<String, JsValue> {
        core_put_transaction(
            self.inner.core(),
            &transaction_json,
            wait.unwrap_or(true),
            wait_timeout_ms,
        )
        .await
    }

    /// Wait for a transaction hash on SSE.
    #[wasm_bindgen(js_name = waitTransaction)]
    pub async fn wait_transaction(
        &self,
        transaction_hash: String,
        timeout_ms: Option<u64>,
    ) -> Result<String, JsValue> {
        core_wait_transaction(self.inner.core(), &transaction_hash, timeout_ms).await
    }

    /// Parse CES events for a transaction against the bound contract.
    #[wasm_bindgen(js_name = parseCES)]
    pub async fn parse_ces(&self, transaction_hash: String) -> Result<String, JsValue> {
        core_parse_ces(self.inner.core(), &transaction_hash).await
    }

    /// Collect SSE processed frames and decode CES for the bound contract.
    #[wasm_bindgen(js_name = collectCESEvents)]
    pub async fn collect_ces_events(
        &self,
        event_names: Vec<String>,
        max_transactions: Option<u32>,
        timeout_ms: Option<u64>,
    ) -> Result<String, JsValue> {
        core_collect_ces(
            self.inner.core(),
            &event_names,
            max_transactions.unwrap_or(8) as usize,
            timeout_ms.unwrap_or(120_000),
        )
        .await
    }
}

async fn core_put_transaction(
    core: &ceps_client::CEPClient,
    transaction_json: &str,
    wait: bool,
    wait_timeout_ms: Option<u64>,
) -> Result<String, JsValue> {
    let tx: serde_json::Value =
        serde_json::from_str(transaction_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let put = core
        .put_transaction(&tx, wait, wait_timeout_ms)
        .await
        .map_err(map_err)?;
    call_result_json(put)
}

async fn core_wait_transaction(
    core: &ceps_client::CEPClient,
    transaction_hash: &str,
    timeout_ms: Option<u64>,
) -> Result<String, JsValue> {
    let event = core
        .wait_transaction(transaction_hash, timeout_ms)
        .await
        .map_err(map_err)?;
    serde_json::to_string(&event).map_err(|e| JsValue::from_str(&e.to_string()))
}

async fn core_parse_ces(
    core: &ceps_client::CEPClient,
    transaction_hash: &str,
) -> Result<String, JsValue> {
    let hash = core
        .target()
        .ok_or_else(|| JsValue::from_str("contract hash is not set"))?
        .contract_hash
        .clone();
    let key = format!("hash-{hash}");
    let rows = core
        .parse_ces_transaction(&[key], transaction_hash)
        .await
        .map_err(map_err)?;
    serde_json::to_string(&rows).map_err(|e| JsValue::from_str(&e.to_string()))
}

async fn core_collect_ces(
    core: &ceps_client::CEPClient,
    event_names: &[String],
    max_transactions: usize,
    timeout_ms: u64,
) -> Result<String, JsValue> {
    let names: Vec<&str> = event_names.iter().map(String::as_str).collect();
    let rows = core
        .collect_ces_events(&names, max_transactions, timeout_ms)
        .await
        .map_err(map_err)?;
    serde_json::to_string(&rows).map_err(|e| JsValue::from_str(&e.to_string()))
}
