use casper_rust_wasm_sdk::types::verbosity::Verbosity;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Client {
    rpc_address: Option<String>,
    node_address: Option<String>,
    verbosity: Option<Verbosity>,
}

impl Default for Client {
    fn default() -> Self {
        Self::new(None, None, None)
    }
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(
        rpc_address: Option<String>,
        node_address: Option<String>,
        verbosity: Option<Verbosity>,
    ) -> Self {
        Client {
            rpc_address,
            node_address,
            verbosity,
        }
    }

    #[wasm_bindgen(js_name = "getRPCAddress")]
    pub fn get_rpc_address(&self) -> String {
        self.rpc_address
            .as_ref()
            .map(String::to_owned)
            .unwrap_or_default()
    }

    #[wasm_bindgen(js_name = "setRPCAddress")]
    pub fn set_rpc_address(&mut self, rpc_address: Option<String>) -> Result<(), String> {
        self.rpc_address = rpc_address;
        Ok(())
    }

    #[wasm_bindgen(js_name = "getNodeAddress")]
    pub fn get_node_address(&self) -> String {
        self.node_address
            .as_ref()
            .map(String::to_owned)
            .unwrap_or_default()
    }

    #[wasm_bindgen(js_name = "setNodeAddress")]
    pub fn set_node_address(&mut self, node_address: Option<String>) -> Result<(), String> {
        self.node_address = node_address;
        Ok(())
    }

    #[wasm_bindgen(js_name = "getVerbosity")]
    pub fn get_verbosity(&self) -> Verbosity {
        self.verbosity.unwrap_or(Verbosity::Low)
    }

    #[wasm_bindgen(js_name = "setVerbosity")]
    pub fn set_verbosity(&mut self, verbosity: Option<Verbosity>) -> Result<(), String> {
        self.verbosity = verbosity;
        Ok(())
    }
}
