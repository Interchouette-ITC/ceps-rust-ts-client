//! Thin wasm-bindgen surface over `ceps-client` (CEP APIs only).

use ceps_client::{Cep18Client, Cep78Client, Cep85Client, Verbosity};
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
}
