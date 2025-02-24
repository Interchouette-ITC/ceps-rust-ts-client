use casper_rust_wasm_sdk::types::verbosity::Verbosity;
use common::{Cep as _Cep, Client};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default)]
pub struct TSClient {
    client: Client,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cep {
    inner: _Cep,
}

#[wasm_bindgen]
impl TSClient {
    #[wasm_bindgen(constructor)]
    pub fn new(
        rpc_address: Option<String>,
        node_address: Option<String>,
        verbosity: Option<Verbosity>,
        cep: Option<Cep>,
    ) -> TSClient {
        TSClient {
            client: Client::new(rpc_address, node_address, verbosity, cep.map(|c| c.inner)),
        }
    }

    #[wasm_bindgen(js_name = "getRPCAddress")]
    pub fn get_rpc_address(&self) -> String {
        self.client.get_rpc_address()
    }

    #[wasm_bindgen(js_name = "setRPCAddress")]
    pub fn set_rpc_address(&mut self, rpc_address: String) {
        self.client.set_rpc_address(rpc_address);
    }

    #[wasm_bindgen(js_name = "getNodeAddress")]
    pub fn get_node_address(&self) -> String {
        self.client.get_node_address()
    }

    #[wasm_bindgen(js_name = "setNodeAddress")]
    pub fn set_node_address(&mut self, node_address: String) {
        self.client.set_node_address(node_address);
    }

    #[wasm_bindgen(js_name = "getVerbosity")]
    pub fn get_verbosity(&self) -> Verbosity {
        self.client.get_verbosity()
    }

    #[wasm_bindgen(js_name = "setVerbosity")]
    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.client.set_verbosity(verbosity);
    }

    #[wasm_bindgen(js_name = "getCEP")]
    pub fn get_cep(&self) -> Cep {
        Cep {
            inner: self.client.get_cep(),
        }
    }
    #[wasm_bindgen(js_name = "setCEP")]
    pub fn set_cep(&mut self, cep: Cep) {
        self.client.set_cep(cep.inner);
    }
}
