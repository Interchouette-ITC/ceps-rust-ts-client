//! Install an Odra OwnedCEP95 on NCTL, mint one token, print owner.
//!
//! ```bash
//! CEPS_WASM_ROOT=tests/wasm \
//! SECRET_KEY_USER_1="$(cat ../casper-nctl-2-docker/assets/users/user-1/secret_key.pem)" \
//!   cargo run -p ceps-client --example cep95_install
//! ```
//!
//! `ceps_client::wasm::load` reads under `CEPS_WASM_ROOT` (default `tests/wasm`
//! when that directory exists relative to the process cwd).

use ceps_client::cep95::InstallArgs;
use ceps_client::{CEP95Client, TransactionParams, Verbosity};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = env::var("SECRET_KEY_USER_1").unwrap_or_else(|_| {
        let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../casper-nctl-2-docker/assets/users/user-1/secret_key.pem");
        fs::read_to_string(p).expect("read user-1 secret or set SECRET_KEY_USER_1")
    });
    let wasm = match env::var("CEPS_WASM_ROOT") {
        Ok(root) if !root.trim().is_empty() => {
            ceps_client::wasm::load_from(PathBuf::from(root).as_path(), "cep95")?
        }
        _ => {
            let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../tests/wasm");
            ceps_client::wasm::load_from(&root, "cep95")?
        }
    };

    let mut client = CEP95Client::new(
        "http://127.0.0.1:11101",
        Some("http://127.0.0.1:18101/events".into()),
        Some("casper-net-1".into()),
        Some(Verbosity::Low),
    )?;

    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let name = format!("Example95{nonce}");
    let package_key = format!("cep95_pkg_{nonce}");
    let args = InstallArgs::new(&name, "EX95", &package_key);
    let tx = TransactionParams::new(&secret, "600000000000");
    let put = client.install(&args, &wasm, &tx).await?;
    println!("installed tx={}", put.transaction_hash);

    let pk = casper_rust_wasm_sdk::helpers::public_key_from_secret_key(&secret)?;
    let (contract, package) = client.bind_odra_install(&pk, &package_key).await?;
    println!("contract={contract}");
    println!("package={package}");

    println!("name={}", client.name().await?);
    println!("symbol={}", client.symbol().await?);

    let owner = {
        let public = casper_rust_wasm_sdk::types::public_key::PublicKey::new(&pk)?;
        public.to_account_hash().to_formatted_string()
    };
    let mint_tx = TransactionParams::new(&secret, "5000000000");
    client.mint(&owner, "1", None, &mint_tx).await?;
    println!("balance={}", client.balance_of(&owner).await?);
    println!("owner_of_1={}", client.owner_of("1").await?);
    Ok(())
}
