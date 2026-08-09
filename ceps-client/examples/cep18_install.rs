//! Install a CEP-18 token on NCTL and print name + balance.
//!
//! ```bash
//! SECRET_KEY_USER_1="$(cat ../casper-nctl-2-docker/assets/users/user-1/secret_key.pem)" \
//!   cargo run -p ceps-client --example cep18_install
//! ```

use ceps_client::cep18::InstallArgs;
use ceps_client::{Cep18Client, EventsMode, TransactionParams, Verbosity};
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
    let wasm_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../tests/wasm/cep18/cep18.wasm");
    let wasm = fs::read(&wasm_path)?;

    let mut client = Cep18Client::new(
        "http://127.0.0.1:11101",
        Some("http://127.0.0.1:18101/events".into()),
        Some("casper-net-1".into()),
        Some(Verbosity::Low),
    )?;

    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let name = format!("Example{nonce}");
    let args = InstallArgs::new(&name, "EX", 9, "1000")
        .with_events_mode(EventsMode::Ces)
        .with_mint_and_burn(true);
    let tx = TransactionParams::new(&secret, "400000000000");
    let put = client.install(&args, &wasm, &tx).await?;
    println!("installed tx={}", put.transaction_hash);

    let pk = casper_rust_wasm_sdk::helpers::public_key_from_secret_key(&secret)?;
    let contract = client
        .core()
        .get_account_named_key(&pk, &format!("cep18_contract_hash_{name}"))
        .await?;
    let package = client
        .core()
        .get_account_named_key(&pk, &format!("cep18_contract_package_{name}"))
        .await?;
    client.set_contract_hash(&contract, Some(&package))?;

    println!("name={}", client.name().await?);
    println!("symbol={}", client.symbol().await?);
    println!("decimals={}", client.decimals().await?);
    Ok(())
}
