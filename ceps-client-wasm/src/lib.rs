//! Thin wasm-bindgen surface over `ceps-client` (CEP APIs only).

use ceps_client::cep18::InstallArgs as CEP18InstallArgs;
use ceps_client::cep78::InstallArgs as CEP78InstallArgs;
use ceps_client::cep85::InstallArgs as CEP85InstallArgs;
use ceps_client::cep95::InstallArgs as CEP95InstallArgs;
use ceps_client::{
    CEP18Client, CEP78Client, CEP85Client, CEP95Client, CallResult, EventsMode, EventsMode78,
    TransactionParams, Verbosity,
};
use js_sys::Uint8Array;
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

    /// Install with URI and optional CES events / burn flag.
    #[wasm_bindgen]
    #[allow(clippy::too_many_arguments)]
    pub async fn install(
        &self,
        name: String,
        uri: String,
        events_mode: Option<u8>,
        enable_burn: Option<bool>,
        wasm: Uint8Array,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wait: Option<bool>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> Result<String, JsValue> {
        let mut args = CEP85InstallArgs::new(name, uri);
        if let Some(mode) = events_mode {
            let mode = EventsMode::from_u8(mode)
                .ok_or_else(|| JsValue::from_str("invalid events_mode"))?;
            args = args.with_events_mode(mode);
        }
        if let Some(b) = enable_burn {
            args = args.with_enable_burn(b);
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

    /// Balance for account + token id.
    #[wasm_bindgen(js_name = balanceOf)]
    pub async fn balance_of(&self, account: String, id: String) -> Result<String, JsValue> {
        self.inner.balance_of(&account, &id).await.map_err(map_err)
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
