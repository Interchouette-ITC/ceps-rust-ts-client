//! Thin wasm-bindgen surface over `ceps-client` (CEP APIs only).

use ceps_client::cep18::InstallArgs as Cep18InstallArgs;
use ceps_client::cep78::InstallArgs as Cep78InstallArgs;
use ceps_client::cep85::InstallArgs as Cep85InstallArgs;
use ceps_client::cep95::InstallArgs as Cep95InstallArgs;
use ceps_client::{
    CallResult, Cep18Client, Cep78Client, Cep85Client, Cep95Client, DeployParams, EventsMode,
    EventsMode78, Verbosity,
};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

fn map_err(err: ceps_client::CepError) -> JsValue {
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

fn deploy_params(secret_key_pem: &str, payment_amount: &str, wait: bool) -> DeployParams {
    let mut d = DeployParams::new(secret_key_pem, payment_amount);
    if !wait {
        d = d.without_wait();
    }
    d
}

fn call_result_json(result: CallResult) -> Result<String, JsValue> {
    serde_json::to_string(&serde_json::json!({
        "transactionHash": result.transaction_hash,
        "hasExecutionResult": result.execution_result.is_some(),
        "cesEvents": result.ces_events,
    }))
    .map_err(|e| JsValue::from_str(&e.to_string()))
}

fn bytes_from_js(wasm: &Uint8Array) -> Vec<u8> {
    wasm.to_vec()
}

/// WASM wrapper for [`Cep18Client`].
#[wasm_bindgen(js_name = Cep18Client)]
pub struct WasmCep18Client {
    inner: Cep18Client,
}

#[wasm_bindgen(js_class = Cep18Client)]
impl WasmCep18Client {
    /// Create a CEP-18 client.
    #[wasm_bindgen(constructor)]
    pub fn new(
        rpc_url: String,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<u8>,
    ) -> Result<WasmCep18Client, JsValue> {
        let inner = Cep18Client::new(rpc_url, sse_url, chain_name, verbosity_from_u8(verbosity))
            .map_err(map_err)?;
        Ok(Self { inner })
    }

    /// RPC URL.
    #[wasm_bindgen(js_name = rpcUrl)]
    pub fn rpc_url(&self) -> String {
        self.inner.rpc_url().to_string()
    }

    /// SSE URL when set.
    #[wasm_bindgen(js_name = sseUrl)]
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
        secret_key_pem: String,
        payment_amount: String,
        wait: Option<bool>,
    ) -> Result<String, JsValue> {
        let mut args = Cep18InstallArgs::new(name, symbol, decimals, total_supply);
        if let Some(mode) = events_mode {
            let mode = EventsMode::from_u8(mode)
                .ok_or_else(|| JsValue::from_str("invalid events_mode"))?;
            args = args.with_events_mode(mode);
        }
        let deploy = deploy_params(&secret_key_pem, &payment_amount, wait.unwrap_or(true));
        let put = self
            .inner
            .install(&args, &bytes_from_js(&wasm), &deploy)
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
}

/// WASM wrapper for [`Cep78Client`].
#[wasm_bindgen(js_name = Cep78Client)]
pub struct WasmCep78Client {
    inner: Cep78Client,
}

#[wasm_bindgen(js_class = Cep78Client)]
impl WasmCep78Client {
    /// Create a CEP-78 client.
    #[wasm_bindgen(constructor)]
    pub fn new(
        rpc_url: String,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<u8>,
    ) -> Result<WasmCep78Client, JsValue> {
        let inner = Cep78Client::new(rpc_url, sse_url, chain_name, verbosity_from_u8(verbosity))
            .map_err(map_err)?;
        Ok(Self { inner })
    }

    /// RPC URL.
    #[wasm_bindgen(js_name = rpcUrl)]
    pub fn rpc_url(&self) -> String {
        self.inner.rpc_url().to_string()
    }

    /// SSE URL when set.
    #[wasm_bindgen(js_name = sseUrl)]
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
        secret_key_pem: String,
        payment_amount: String,
        wait: Option<bool>,
    ) -> Result<String, JsValue> {
        let mut args =
            Cep78InstallArgs::new(collection_name, collection_symbol, total_token_supply);
        if let Some(mode) = events_mode {
            let mode = EventsMode78::from_u8(mode)
                .ok_or_else(|| JsValue::from_str("invalid events_mode"))?;
            args = args.with_events_mode(mode);
        }
        let deploy = deploy_params(&secret_key_pem, &payment_amount, wait.unwrap_or(true));
        let put = self
            .inner
            .install(&args, &bytes_from_js(&wasm), &deploy)
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
    #[wasm_bindgen(js_name = parseCes)]
    pub async fn parse_ces(&self, transaction_hash: String) -> Result<String, JsValue> {
        let hash = self
            .inner
            .core()
            .require_target()
            .map_err(map_err)?
            .contract_hash
            .clone();
        let key = format!("hash-{hash}");
        let rows = self
            .inner
            .core()
            .parse_ces_transaction(&[key], &transaction_hash)
            .await
            .map_err(map_err)?;
        serde_json::to_string(&rows).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

/// WASM wrapper for [`Cep85Client`].
#[wasm_bindgen(js_name = Cep85Client)]
pub struct WasmCep85Client {
    inner: Cep85Client,
}

#[wasm_bindgen(js_class = Cep85Client)]
impl WasmCep85Client {
    /// Create a CEP-85 client.
    #[wasm_bindgen(constructor)]
    pub fn new(
        rpc_url: String,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<u8>,
    ) -> Result<WasmCep85Client, JsValue> {
        let inner = Cep85Client::new(rpc_url, sse_url, chain_name, verbosity_from_u8(verbosity))
            .map_err(map_err)?;
        Ok(Self { inner })
    }

    /// RPC URL.
    #[wasm_bindgen(js_name = rpcUrl)]
    pub fn rpc_url(&self) -> String {
        self.inner.rpc_url().to_string()
    }

    /// SSE URL when set.
    #[wasm_bindgen(js_name = sseUrl)]
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
        secret_key_pem: String,
        payment_amount: String,
        wait: Option<bool>,
    ) -> Result<String, JsValue> {
        let mut args = Cep85InstallArgs::new(name, uri);
        if let Some(mode) = events_mode {
            let mode = EventsMode::from_u8(mode)
                .ok_or_else(|| JsValue::from_str("invalid events_mode"))?;
            args = args.with_events_mode(mode);
        }
        if let Some(b) = enable_burn {
            args = args.with_enable_burn(b);
        }
        let deploy = deploy_params(&secret_key_pem, &payment_amount, wait.unwrap_or(true));
        let put = self
            .inner
            .install(&args, &bytes_from_js(&wasm), &deploy)
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
}

/// WASM wrapper for [`Cep95Client`].
#[wasm_bindgen(js_name = Cep95Client)]
pub struct WasmCep95Client {
    inner: Cep95Client,
}

#[wasm_bindgen(js_class = Cep95Client)]
impl WasmCep95Client {
    /// Create a CEP-95 client.
    #[wasm_bindgen(constructor)]
    pub fn new(
        rpc_url: String,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<u8>,
    ) -> Result<WasmCep95Client, JsValue> {
        let inner = Cep95Client::new(rpc_url, sse_url, chain_name, verbosity_from_u8(verbosity))
            .map_err(map_err)?;
        Ok(Self { inner })
    }

    /// RPC URL.
    #[wasm_bindgen(js_name = rpcUrl)]
    pub fn rpc_url(&self) -> String {
        self.inner.rpc_url().to_string()
    }

    /// SSE URL when set.
    #[wasm_bindgen(js_name = sseUrl)]
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

    /// Install Odra OwnedCep95 (or compatible) with package named-key name.
    #[wasm_bindgen]
    #[allow(clippy::too_many_arguments)]
    pub async fn install(
        &self,
        name: String,
        symbol: String,
        package_hash_key_name: String,
        wasm: Uint8Array,
        secret_key_pem: String,
        payment_amount: String,
        wait: Option<bool>,
    ) -> Result<String, JsValue> {
        let args = Cep95InstallArgs::new(name, symbol, package_hash_key_name);
        let deploy = deploy_params(&secret_key_pem, &payment_amount, wait.unwrap_or(true));
        let put = self
            .inner
            .install(&args, &bytes_from_js(&wasm), &deploy)
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
}
