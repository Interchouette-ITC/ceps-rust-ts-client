use casper_rust_wasm_sdk::types::verbosity::Verbosity;
use ts_client::Client;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_default_client() {
    let client = Client::default();
    assert_eq!(client.get_rpc_address(), "");
    assert_eq!(client.get_node_address(), "");
    assert_eq!(client.get_verbosity(), Verbosity::Low);
}

#[wasm_bindgen_test]
fn test_rpc_address() {
    let mut client = Client::default();
    assert_eq!(client.get_rpc_address(), "");

    client
        .set_rpc_address(Some("http://127.0.0.1:11101".to_string()))
        .unwrap();
    assert_eq!(client.get_rpc_address(), "http://127.0.0.1:11101");
}

#[wasm_bindgen_test]
fn test_node_address() {
    let mut client = Client::default();
    assert_eq!(client.get_node_address(), "");

    client
        .set_node_address(Some("http://127.0.0.1:28101".to_string()))
        .unwrap();
    assert_eq!(client.get_node_address(), "http://127.0.0.1:28101");
}

#[wasm_bindgen_test]
fn test_verbosity() {
    let mut client = Client::default();
    assert_eq!(client.get_verbosity(), Verbosity::Low);

    client.set_verbosity(Some(Verbosity::High)).unwrap();
    assert_eq!(client.get_verbosity(), Verbosity::High);
}
