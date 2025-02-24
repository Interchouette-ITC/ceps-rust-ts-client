use casper_rust_wasm_sdk::types::verbosity::Verbosity;

pub struct Client {
    rpc_address: Option<String>,
    node_address: Option<String>,
    verbosity: Option<Verbosity>,
    cep: Option<Cep>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Cep {
    #[default]
    CEP18 = 18,
    CEP78 = 78,
    CEP85 = 85,
}

impl Default for Client {
    fn default() -> Self {
        Self::new(None, None, None, Some(Cep::default()))
    }
}

impl Client {
    pub fn new(
        rpc_address: Option<String>,
        node_address: Option<String>,
        verbosity: Option<Verbosity>,
        cep: Option<Cep>,
    ) -> Self {
        Client {
            rpc_address,
            node_address,
            verbosity,
            cep: Some(cep.unwrap_or_default()),
        }
    }

    pub fn get_rpc_address(&self) -> String {
        self.rpc_address
            .as_ref()
            .map(String::to_owned)
            .unwrap_or_default()
    }

    pub fn set_rpc_address(&mut self, rpc_address: String) {
        self.rpc_address = Some(rpc_address);
    }

    pub fn get_node_address(&self) -> String {
        self.node_address
            .as_ref()
            .map(String::to_owned)
            .unwrap_or_default()
    }

    pub fn set_node_address(&mut self, node_address: String) {
        self.node_address = Some(node_address);
    }

    pub fn get_verbosity(&self) -> Verbosity {
        self.verbosity.unwrap_or(Verbosity::Low)
    }

    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.verbosity = Some(verbosity);
    }

    pub fn get_cep(&self) -> Cep {
        self.cep.unwrap_or_default()
    }

    pub fn set_cep(&mut self, cep: Cep) {
        self.cep = Some(cep);
    }
}
