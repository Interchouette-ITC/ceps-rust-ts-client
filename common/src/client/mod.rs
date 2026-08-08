use casper_rust_wasm_sdk::types::verbosity::Verbosity;
use std::error::Error;
use url::Url;

pub mod cep18;
pub mod cep78;

pub trait ClientTrait {
    fn get_rpc_url(&self) -> String;
    fn set_rpc_url(&mut self, new_rpc_url: String) -> Result<(), Box<dyn Error>>;
    fn get_sse_url(&self) -> String;
    fn set_sse_url(&mut self, sse_url: String) -> Result<(), Box<dyn Error>>;
    fn get_verbosity(&self) -> Verbosity;
    fn set_verbosity(&mut self, verbosity: Verbosity);
}

pub(crate) struct Client {
    rpc_url: String,
    sse_url: Option<String>,
    chain_name: Option<String>,
    verbosity: Option<Verbosity>,
    cep: Cep,
}

impl Client {
    pub(crate) fn new(
        rpc_url: String,
        sse_url: Option<String>,
        verbosity: Option<Verbosity>,
        cep: Cep,
    ) -> Result<Self, Box<dyn Error>> {
        let mut client = Self {
            rpc_url: String::from(""),
            sse_url: None,
            chain_name: None,
            verbosity,
            cep,
        };

        client.set_rpc_url(rpc_url)?;

        if let Some(sse_url) = sse_url {
            if !sse_url.is_empty() {
                client.set_sse_url(sse_url)?;
            }
        }

        Ok(client)
    }
}

impl ClientTrait for Client {
    fn get_rpc_url(&self) -> String {
        self.rpc_url.clone()
    }

    fn set_rpc_url(&mut self, new_rpc_url: String) -> Result<(), Box<dyn Error>> {
        let trimmed_url = new_rpc_url.trim().to_string();
        if trimmed_url.is_empty() {
            return Err("RPC URL could not be initialized due to invalid URL.".into());
        }
        let parsed_url = Url::parse(&trimmed_url).map_err(|e| {
            format!(
                "RPC URL could not be initialized due to invalid URL.\n{}",
                e
            )
        })?;
        if !["http", "https"].contains(&parsed_url.scheme()) {
            return Err("RPC URL could not be initialized due to invalid URL protocol. Only http and https are allowed.".into());
        }
        let mut final_url = trimmed_url.clone();
        if !final_url.ends_with("/rpc") {
            final_url.push_str("/rpc");
        }
        self.rpc_url = final_url;
        Ok(())
    }

    fn get_sse_url(&self) -> String {
        self.sse_url.clone().unwrap_or_default()
    }

    fn set_sse_url(&mut self, new_sse_url: String) -> Result<(), Box<dyn Error>> {
        let trimmed_url = new_sse_url.trim().to_string();
        if trimmed_url.is_empty() {
            return Err("SSE URL could not be initialized due to invalid URL.".into());
        }
        let parsed_url = Url::parse(&trimmed_url).map_err(|e| {
            format!(
                "SSE URL could not be initialized due to invalid URL.\n{}",
                e
            )
        })?;
        if !["http", "https"].contains(&parsed_url.scheme()) {
            return Err("SSE URL could not be initialized due to invalid URL protocol. Only http and https are allowed.".into());
        }
        let mut final_url = trimmed_url.clone();
        if !final_url.ends_with("/events") {
            final_url.push_str("/events");
        }
        self.sse_url = Some(final_url);
        Ok(())
    }

    fn get_verbosity(&self) -> Verbosity {
        self.verbosity.unwrap_or(Verbosity::Low)
    }

    fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.verbosity = Some(verbosity);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Cep {
    #[default]
    CEP18 = 18,
    CEP78 = 78,
    CEP85 = 85,
}
