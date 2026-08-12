//! Ignored NCTL live smokes for MCP tool wrappers (install + query per CEP).

#![cfg(test)]

use crate::tools::{cep18, cep78, cep85, cep95};
use casper_rust_wasm_sdk::helpers::public_key_from_secret_key;
use ceps_client::{CEP18Client, CEP78Client, CEP85Client, CEP95Client, Verbosity};
use rmcp::model::CallToolResult;
use std::env;
use std::fs;
use std::net::TcpStream;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const INSTALL_PAYMENT_18: &str = "400000000000";
const INSTALL_PAYMENT_78: &str = "600000000000";
const INSTALL_PAYMENT_85: &str = "550000000000";
const INSTALL_PAYMENT_95: &str = "600000000000";

fn nctl_available() -> bool {
    TcpStream::connect_timeout(
        &"127.0.0.1:11101".parse().expect("addr"),
        Duration::from_secs(1),
    )
    .is_ok()
}

fn user1_secret_pem() -> Option<String> {
    if let Ok(pem) = env::var("SECRET_KEY_USER_1") {
        if !pem.trim().is_empty() {
            return Some(pem);
        }
    }
    let path = env::var("SECRET_KEY_NCTL_PATH").unwrap_or_else(|_| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../casper-nctl-2-docker/assets/users/user-1")
            .to_string_lossy()
            .into_owned()
    });
    let file = {
        let p = PathBuf::from(&path);
        if p.is_dir() {
            p.join("secret_key.pem")
        } else {
            p
        }
    };
    fs::read_to_string(file).ok()
}

fn ensure_wasm_root() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../tests/wasm");
    env::set_var("CEPS_WASM_ROOT", root);
}

fn tool_text(out: CallToolResult) -> String {
    assert_eq!(out.is_error, Some(false), "tool error: {out:?}");
    out.content
        .iter()
        .filter_map(|c| c.as_text().map(|t| t.text.clone()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn assert_tx_hash(text: &str) {
    assert!(
        text.contains("transactionHash"),
        "missing transactionHash: {text}"
    );
    assert!(
        !text.to_lowercase().contains("connection refused"),
        "RPC failed: {text}"
    );
}

fn pk_hex(secret: &str) -> String {
    public_key_from_secret_key(secret).expect("public key")
}

fn rpc_sse() -> (String, String) {
    let rpc = env::var("CEPS_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:11101".into());
    let sse = env::var("CEPS_SSE_URL").unwrap_or_else(|_| "http://127.0.0.1:18101/events".into());
    (rpc, sse)
}

#[tokio::test]
#[ignore = "requires live NCTL / RPC"]
async fn live_ceps18_install_query() {
    if !nctl_available() {
        panic!("NCTL RPC not reachable on 127.0.0.1:11101");
    }
    let secret = user1_secret_pem().expect("SECRET_KEY_USER_1 or NCTL user-1 pem");
    ensure_wasm_root();
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let name = format!("Mcp18{nonce}");

    let install = cep18::install(crate::tool_args::Ceps18InstallArgs {
        name: name.clone(),
        symbol: "M18".into(),
        decimals: 9,
        total_supply: "1000000000000".into(),
        secret_key_pem: Some(secret.clone()),
        payment_amount: INSTALL_PAYMENT_18.into(),
        wasm_path: Some("cep18/cep18.wasm".into()),
        wasm_base64: None,
        events_mode: Some(1),
        enable_mint_and_burn: Some(true),
        admin_list: None,
        minter_list: None,
        wait: Some(true),
        wait_timeout_ms: None,
        make_only: None,
        initiator_addr: None,
    })
    .await;
    let install_text = tool_text(install);
    assert_tx_hash(&install_text);

    let (rpc, sse) = rpc_sse();
    let client = CEP18Client::new(
        rpc,
        Some(sse),
        Some("casper-net-1".into()),
        Some(Verbosity::Low),
    )
    .expect("client");
    let pk = pk_hex(&secret);
    let contract = client
        .get_account_named_key(&pk, &format!("cep18_contract_hash_{name}"))
        .await
        .expect("contract hash");
    let package = client
        .get_account_named_key(&pk, &format!("cep18_contract_package_{name}"))
        .await
        .expect("package hash");

    let name_out = cep18::name(contract, Some(package)).await;
    let name_text = tool_text(name_out);
    assert!(
        name_text.contains(&name),
        "name query mismatch: {name_text}"
    );
}

#[tokio::test]
#[ignore = "requires live NCTL / RPC"]
async fn live_ceps78_install_query() {
    if !nctl_available() {
        panic!("NCTL RPC not reachable on 127.0.0.1:11101");
    }
    let secret = user1_secret_pem().expect("SECRET_KEY_USER_1 or NCTL user-1 pem");
    ensure_wasm_root();
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let name = format!("Mcp78{nonce}");

    let install = cep78::install(crate::tool_args::Ceps78InstallArgs {
        collection_name: name.clone(),
        collection_symbol: "M78".into(),
        total_token_supply: 100,
        secret_key_pem: Some(secret.clone()),
        payment_amount: INSTALL_PAYMENT_78.into(),
        wasm_path: Some("cep78/cep78.wasm".into()),
        wasm_base64: None,
        events_mode: Some(2),
        wait: Some(true),
        wait_timeout_ms: None,
        make_only: None,
        initiator_addr: None,
    })
    .await;
    let install_text = tool_text(install);
    assert_tx_hash(&install_text);

    let (rpc, sse) = rpc_sse();
    let client = CEP78Client::new(
        rpc,
        Some(sse),
        Some("casper-net-1".into()),
        Some(Verbosity::Low),
    )
    .expect("client");
    let pk = pk_hex(&secret);
    let contract = client
        .get_account_named_key(&pk, &format!("cep78_contract_hash_{name}"))
        .await
        .expect("contract hash");
    let package = client
        .get_account_named_key(&pk, &format!("cep78_contract_package_{name}"))
        .await
        .expect("package hash");

    let name_out = cep78::collection_name(contract, Some(package)).await;
    let name_text = tool_text(name_out);
    assert!(
        name_text.contains(&name),
        "collection_name mismatch: {name_text}"
    );
}

#[tokio::test]
#[ignore = "requires live NCTL / RPC"]
async fn live_ceps85_install_query() {
    if !nctl_available() {
        panic!("NCTL RPC not reachable on 127.0.0.1:11101");
    }
    let secret = user1_secret_pem().expect("SECRET_KEY_USER_1 or NCTL user-1 pem");
    ensure_wasm_root();
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let name = format!("Mcp85{nonce}");

    let install = cep85::install(crate::tool_args::Ceps85InstallArgs {
        name: name.clone(),
        uri: "https://example.com/metadata/{id}.json".into(),
        secret_key_pem: Some(secret.clone()),
        payment_amount: INSTALL_PAYMENT_85.into(),
        wasm_path: Some("cep85/cep85.wasm".into()),
        wasm_base64: None,
        events_mode: Some(1),
        enable_burn: Some(true),
        admin_list: None,
        minter_list: None,
        burner_list: None,
        meta_list: None,
        transfer_filter_contract: None,
        transfer_filter_method: None,
        wait: Some(true),
        wait_timeout_ms: None,
        make_only: None,
        initiator_addr: None,
    })
    .await;
    let install_text = tool_text(install);
    assert_tx_hash(&install_text);

    let (rpc, sse) = rpc_sse();
    let client = CEP85Client::new(
        rpc,
        Some(sse),
        Some("casper-net-1".into()),
        Some(Verbosity::Low),
    )
    .expect("client");
    let pk = pk_hex(&secret);
    let contract = client
        .get_account_named_key(&pk, &format!("cep85_contract_hash_{name}"))
        .await
        .expect("contract hash");
    let package = client
        .get_account_named_key(&pk, &format!("cep85_contract_package_{name}"))
        .await
        .expect("package hash");

    let name_out = cep85::collection_name(contract, Some(package)).await;
    let name_text = tool_text(name_out);
    assert!(
        name_text.contains(&name),
        "collection_name mismatch: {name_text}"
    );
}

#[tokio::test]
#[ignore = "requires live NCTL / RPC"]
async fn live_ceps95_install_query() {
    if !nctl_available() {
        panic!("NCTL RPC not reachable on 127.0.0.1:11101");
    }
    let secret = user1_secret_pem().expect("SECRET_KEY_USER_1 or NCTL user-1 pem");
    ensure_wasm_root();
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let name = format!("Mcp95{nonce}");
    let package_key = format!("cep95_pkg_{nonce}");

    let install = cep95::install(crate::tool_args::Ceps95InstallArgs {
        name: name.clone(),
        symbol: "M95".into(),
        package_hash_key_name: package_key.clone(),
        secret_key_pem: Some(secret.clone()),
        payment_amount: INSTALL_PAYMENT_95.into(),
        wasm_path: Some("cep95/cep95.wasm".into()),
        wasm_base64: None,
        allow_key_override: None,
        is_upgradable: None,
        is_upgrade: None,
        wait: Some(true),
        wait_timeout_ms: None,
        make_only: None,
        initiator_addr: None,
    })
    .await;
    let install_text = tool_text(install);
    assert_tx_hash(&install_text);

    let pk = pk_hex(&secret);
    let bind = cep95::bind_odra_install(pk.clone(), package_key.clone()).await;
    let bind_text = tool_text(bind);
    assert!(
        bind_text.contains("contractHash") || bind_text.contains("hash-"),
        "bind_odra_install: {bind_text}"
    );

    let (rpc, sse) = rpc_sse();
    let mut client = CEP95Client::new(
        rpc,
        Some(sse),
        Some("casper-net-1".into()),
        Some(Verbosity::Low),
    )
    .expect("client");
    let (contract, package) = client
        .bind_odra_install(&pk, &package_key)
        .await
        .expect("bind for query");

    let name_out = cep95::name(contract, Some(package)).await;
    let name_text = tool_text(name_out);
    assert!(name_text.contains(&name), "name mismatch: {name_text}");
}
