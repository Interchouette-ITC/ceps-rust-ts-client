//! Install a CEP-85 multi-token on NCTL, mint, and print balance.
//!
//! ```bash
//! SECRET_KEY_USER_1="$(cat ../casper-nctl-2-docker/assets/users/user-1/secret_key.pem)" \
//!   cargo run -p ceps-client --example cep85_install
//! ```

use ceps_client::cep85::InstallArgs;
use ceps_client::{Cep85Client, DeployParams, EventsMode, Verbosity};
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
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../tests/wasm/cep85/cep85.wasm");
    let wasm = fs::read(&wasm_path)?;

    let mut client = Cep85Client::new(
        "http://127.0.0.1:11101",
        Some("http://127.0.0.1:18101/events".into()),
        Some("casper-net-1".into()),
        Some(Verbosity::Low),
    )?;

    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let name = format!("Example85{nonce}");
    let args = InstallArgs::new(&name, "https://example.com/metadata/{id}.json")
        .with_events_mode(EventsMode::Ces)
        .with_enable_burn(true);
    let deploy = DeployParams::new(&secret, "550000000000");
    let put = client.install(&args, &wasm, &deploy).await?;
    println!("installed tx={}", put.transaction_hash);

    let pk = casper_rust_wasm_sdk::helpers::public_key_from_secret_key(&secret)?;
    let contract = client
        .core()
        .get_account_named_key(&pk, &format!("cep85_contract_hash_{name}"))
        .await?;
    let package = client
        .core()
        .get_account_named_key(&pk, &format!("cep85_contract_package_{name}"))
        .await?;
    client.set_contract_hash(&contract, Some(&package))?;

    println!("name={}", client.collection_name().await?);
    println!("uri={}", client.collection_uri().await?);

    let owner = {
        let public = casper_rust_wasm_sdk::types::public_key::PublicKey::new(&pk)?;
        public.to_account_hash().to_formatted_string()
    };
    let mint_deploy = DeployParams::new(&secret, "5000000000");
    client.mint(&owner, "1", "10", None, &mint_deploy).await?;
    println!("balance={}", client.balance_of(&owner, "1").await?);
    Ok(())
}
