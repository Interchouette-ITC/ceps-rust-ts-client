pub mod helpers;
pub mod integration;
#[cfg(test)]
pub mod integration_tests;
use crate::{config::DEFAULT_EVENTS_ADDRESS, tests::helpers::get_event_handler_fn};
use casper_rust_wasm_sdk::{
    helpers::public_key_from_secret_key,
    types::verbosity::Verbosity,
    watcher::{EventHandlerFn, EventParseResult, Subscription},
    SDK,
};
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub async fn run_tests() {
    let _ = _run_example_1().await;
}

// get_transaction
pub async fn _run_example_1() {
    let sdk = SDK::new(
        Some("http://localhost:11101".to_string()),
        None,
        Some(Verbosity::High),
    );
    use casper_rust_wasm_sdk::types::hash::transaction_hash::TransactionHash;

    let transaction_hash =
        TransactionHash::new("27d81df41801602f47cdb4618a814407daf38d0c39be32c7f6c109d7e39a3f4b")
            .unwrap();

    let finalized_approvals = true;
    let get_transaction = sdk
        .get_transaction(transaction_hash, Some(finalized_approvals), None, None)
        .await;

    let transaction = get_transaction.unwrap().result.transaction;
    let timestamp = transaction.timestamp();
    let hash = transaction.hash().to_hex_string();
    println!("{timestamp} {hash}");
}
