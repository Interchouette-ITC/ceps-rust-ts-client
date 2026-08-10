//! Shared helpers for NCTL-backed integration tests.

use std::env;
use std::fs;
use std::net::TcpStream;
use std::path::PathBuf;
use std::time::Duration;

use casper_rust_wasm_sdk::helpers::public_key_from_secret_key;
use casper_rust_wasm_sdk::types::public_key::PublicKey;
use casper_rust_wasm_sdk::types::verbosity::Verbosity;
use ceps_client::CEP18Client;

pub const DEFAULT_RPC: &str = "http://127.0.0.1:11101";
pub const DEFAULT_SSE: &str = "http://127.0.0.1:18101/events";
pub const DEFAULT_CHAIN: &str = "casper-net-1";
pub const INSTALL_PAYMENT: &str = "400000000000";
pub const CALL_PAYMENT: &str = "5000000000";

/// True when TCP `127.0.0.1:11101` accepts connections.
pub fn nctl_available() -> bool {
    TcpStream::connect_timeout(
        &"127.0.0.1:11101".parse().expect("addr"),
        Duration::from_secs(1),
    )
    .is_ok()
}

/// Load user-1 secret key PEM (env override or sibling NCTL assets).
pub fn user1_secret_pem() -> Option<String> {
    if let Ok(pem) = env::var("SECRET_KEY_USER_1") {
        if !pem.trim().is_empty() {
            return Some(pem);
        }
    }
    if let Ok(path) = env::var("SECRET_KEY_NCTL_PATH") {
        let file = {
            let p = PathBuf::from(&path);
            if p.is_dir() {
                p.join("secret_key.pem")
            } else {
                p
            }
        };
        if let Ok(pem) = fs::read_to_string(file) {
            return Some(pem);
        }
    }
    user_secret_pem(1, "SECRET_KEY_USER_1")
}

/// Load user-N secret key PEM from sibling NCTL assets (or `env_key`).
pub fn user_secret_pem(user_n: u8, env_key: &str) -> Option<String> {
    if let Ok(pem) = env::var(env_key) {
        if !pem.trim().is_empty() {
            return Some(pem);
        }
    }
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
        "../../../casper-nctl-2-docker/assets/users/user-{user_n}/secret_key.pem"
    ));
    fs::read_to_string(path).ok()
}

pub fn user1_account_hash(secret_pem: &str) -> String {
    account_hash_from_secret(secret_pem)
}

pub fn account_hash_from_secret(secret_pem: &str) -> String {
    let pk_hex = public_key_from_secret_key(secret_pem).expect("public key from secret");
    let public = PublicKey::new(&pk_hex).expect("parse public key");
    public.to_account_hash().to_formatted_string()
}

pub fn user1_public_key_hex(secret_pem: &str) -> String {
    public_key_from_secret_key(secret_pem).expect("public key")
}

pub fn cep18_client() -> CEP18Client {
    let rpc = env::var("CEPS_RPC_URL").unwrap_or_else(|_| DEFAULT_RPC.to_string());
    let sse = env::var("CEPS_SSE_URL").unwrap_or_else(|_| DEFAULT_SSE.to_string());
    let chain = env::var("CEPS_CHAIN_NAME").unwrap_or_else(|_| DEFAULT_CHAIN.to_string());
    CEP18Client::new(rpc, Some(sse), Some(chain), Some(Verbosity::Low)).expect("client")
}

pub fn wasm_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/wasm/cep18")
        .join(name)
}
