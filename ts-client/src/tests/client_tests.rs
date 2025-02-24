#[cfg(test)]
mod tests {
    use crate::TSClient;
    use casper_rust_wasm_sdk::types::verbosity::Verbosity;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_default_client_native() {
        let client = TSClient::default();
        assert_eq!(client.get_rpc_address(), "");
        assert_eq!(client.get_node_address(), "");
        assert_eq!(client.get_verbosity(), Verbosity::Low);
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test]
    fn test_default_client() {
        let client = TSClient::default();
        assert_eq!(client.get_rpc_address(), "");
        assert_eq!(client.get_node_address(), "");
        assert_eq!(client.get_verbosity(), Verbosity::Low);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_rpc_address() {
        let mut client = TSClient::default();
        assert_eq!(client.get_rpc_address(), "");

        client.set_rpc_address("http://127.0.0.1:11101".to_string());
        assert_eq!(client.get_rpc_address(), "http://127.0.0.1:11101");
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test]
    fn test_rpc_address() {
        let mut client = TSClient::default();
        assert_eq!(client.get_rpc_address(), "");

        client.set_rpc_address("http://127.0.0.1:11101".to_string());
        assert_eq!(client.get_rpc_address(), "http://127.0.0.1:11101");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_node_address() {
        let mut client = TSClient::default();
        assert_eq!(client.get_node_address(), "");

        client.set_node_address("http://127.0.0.1:28101".to_string());
        assert_eq!(client.get_node_address(), "http://127.0.0.1:28101");
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test]
    fn test_node_address() {
        let mut client = TSClient::default();
        assert_eq!(client.get_node_address(), "");

        client.set_node_address("http://127.0.0.1:28101".to_string());
        assert_eq!(client.get_node_address(), "http://127.0.0.1:28101");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_verbosity() {
        let mut client = TSClient::default();
        assert_eq!(client.get_verbosity(), Verbosity::Low);

        client.set_verbosity(Verbosity::High);
        assert_eq!(client.get_verbosity(), Verbosity::High);
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test]
    fn test_verbosity() {
        let mut client = TSClient::default();
        assert_eq!(client.get_verbosity(), Verbosity::Low);

        client.set_verbosity(Verbosity::High);
        assert_eq!(client.get_verbosity(), Verbosity::High);
    }
}
