use common::{ClientCEP18 as CommonClientCEP78, Verbosity};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = ClientCEP18)]
pub struct ClientCEP18 {
    client: CommonClientCEP78,
}

#[wasm_bindgen]
impl ClientCEP18 {
    #[wasm_bindgen(constructor)]
    pub fn new(
        rpc_url: String,
        sse_url: Option<String>,
        verbosity: Option<Verbosity>,
    ) -> Result<Self, JsError> {
        let client = CommonClientCEP78::new(rpc_url, sse_url, verbosity)
            .map_err(|err| JsError::new(&err.to_string()))?;
        Ok(Self { client })
    }

    #[wasm_bindgen(js_name = "getRPCUrl")]
    pub fn get_rpc_url(&self) -> String {
        self.client.get_rpc_url()
    }

    #[wasm_bindgen(js_name = "setRPCUrl")]
    pub fn set_rpc_url(&mut self, rpc_url: String) -> Result<(), JsError> {
        self.client
            .set_rpc_url(rpc_url)
            .map_err(|err| JsError::new(&err.to_string()))
    }

    #[wasm_bindgen(js_name = "getSSEUrl")]
    pub fn get_sse_url(&self) -> String {
        self.client.get_sse_url()
    }

    #[wasm_bindgen(js_name = "setSSEUrl")]
    pub fn set_sse_url(&mut self, sse_url: String) -> Result<(), JsError> {
        self.client
            .set_sse_url(sse_url)
            .map_err(|err| JsError::new(&err.to_string()))
    }

    #[wasm_bindgen(js_name = "getVerbosity")]
    pub fn get_verbosity(&self) -> Verbosity {
        self.client.get_verbosity()
    }

    #[wasm_bindgen(js_name = "setVerbosity")]
    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.client.set_verbosity(verbosity);
    }
}
