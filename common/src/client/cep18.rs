use super::{Cep, Client, ClientTrait};
use casper_rust_wasm_sdk::types::verbosity::Verbosity;
use std::error::Error;

pub struct ClientCEP18 {
    client: Client,
}

impl ClientCEP18 {
    pub fn new(
        rpc_url: String,
        sse_url: Option<String>,
        verbosity: Option<Verbosity>,
    ) -> Result<Self, Box<dyn Error>> {
        let client = Client::new(rpc_url, sse_url, verbosity, Cep::CEP18)?;
        Ok(Self { client })
    }

    pub fn get_rpc_url(&self) -> String {
        self.client.get_rpc_url()
    }

    pub fn set_rpc_url(&mut self, rpc_url: String) -> Result<(), Box<dyn Error>> {
        self.client.set_rpc_url(rpc_url)
    }

    pub fn get_sse_url(&self) -> String {
        self.client.get_sse_url()
    }

    pub fn set_sse_url(&mut self, sse_url: String) -> Result<(), Box<dyn Error>> {
        self.client.set_sse_url(sse_url)
    }

    pub fn get_verbosity(&self) -> Verbosity {
        self.client.get_verbosity()
    }

    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.client.set_verbosity(verbosity);
    }
}
