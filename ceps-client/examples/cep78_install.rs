//! Install a CEP-78 NFT collection on NCTL, mint one token, print owner.
//!
//! ```bash
//! SECRET_KEY_USER_1="$(cat ../casper-nctl-2-docker/assets/users/user-1/secret_key.pem)" \
//!   cargo run -p ceps-client --example cep78_install
//! ```

use ceps_client::cep78::{InstallArgs, TokenIdentifier};
use ceps_client::{Cep78Client, EventsMode78, TransactionParams, Verbosity};
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
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../tests/wasm/cep78/cep78.wasm");
    let wasm = fs::read(&wasm_path)?;

    let mut client = Cep78Client::new(
        "http://127.0.0.1:11101",
        Some("http://127.0.0.1:18101/events".into()),
        Some("casper-net-1".into()),
        Some(Verbosity::Low),
    )?;

    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let name = format!("Example78{nonce}");
    let args = InstallArgs::new(&name, "EX78", 50).with_events_mode(EventsMode78::Ces);
    let tx = TransactionParams::new(&secret, "600000000000");
    let put = client.install(&args, &wasm, &tx).await?;
    println!("installed tx={}", put.transaction_hash);

    let pk = casper_rust_wasm_sdk::helpers::public_key_from_secret_key(&secret)?;
    let contract = client
        .core()
        .get_account_named_key(&pk, &format!("cep78_contract_hash_{name}"))
        .await?;
    let package = client
        .core()
        .get_account_named_key(&pk, &format!("cep78_contract_package_{name}"))
        .await?;
    client.set_contract_hash(&contract, Some(&package))?;

    println!("name={}", client.collection_name().await?);
    println!("symbol={}", client.collection_symbol().await?);

    let owner = {
        let public = casper_rust_wasm_sdk::types::public_key::PublicKey::new(&pk)?;
        public.to_account_hash().to_formatted_string()
    };
    let mint_tx = TransactionParams::new(&secret, "5000000000");
    client.mint(&owner, "hello", None, &mint_tx).await?;
    println!("balance={}", client.balance_of(&owner).await?);
    println!(
        "owner_of_0={}",
        client.owner_of(&TokenIdentifier::id(0)).await?
    );
    Ok(())
}
