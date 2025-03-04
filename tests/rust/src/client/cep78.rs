#[cfg(test)]
mod tests {
    use common::{ClientCEP78, Verbosity};

    #[test]
    fn test_clientcep78_new_valid_urls() {
        let client = ClientCEP78::new(
            "http://valid-rpc-url".to_string(),
            Some("http://valid-sse-url".to_string()),
            Some(Verbosity::Low),
        );

        assert!(client.is_ok(), "Expected ClientCEP78 creation to succeed.");
    }

    #[test]
    fn test_clientcep78_new_invalid_rpc_url() {
        let client = ClientCEP78::new(
            "".to_string(), // Invalid RPC URL
            Some("http://valid-sse-url".to_string()),
            Some(Verbosity::Low),
        );

        assert!(
            client.is_err(),
            "Expected ClientCEP78 creation to fail due to invalid RPC URL."
        );
    }

    #[test]
    fn test_clientcep78_get_rpc_url() {
        let client = ClientCEP78::new(
            "http://valid-rpc-url".to_string(),
            Some("http://valid-sse-url".to_string()),
            Some(Verbosity::Low),
        )
        .unwrap();

        assert_eq!(client.get_rpc_url(), "http://valid-rpc-url/rpc");
    }

    #[test]
    fn test_clientcep78_set_rpc_url() {
        let mut client = ClientCEP78::new(
            "http://valid-rpc-url".to_string(),
            Some("http://valid-sse-url".to_string()),
            Some(Verbosity::Low),
        )
        .unwrap();

        let result = client.set_rpc_url("http://new-rpc-url".to_string());
        assert!(result.is_ok());
        assert_eq!(client.get_rpc_url(), "http://new-rpc-url/rpc");
    }

    #[test]
    fn test_clientcep78_get_sse_url() {
        let client = ClientCEP78::new(
            "http://valid-rpc-url".to_string(),
            Some("http://valid-sse-url".to_string()),
            Some(Verbosity::Low),
        )
        .unwrap();

        assert_eq!(client.get_sse_url(), "http://valid-sse-url/events");
    }

    #[test]
    fn test_clientcep78_set_sse_url() {
        let mut client = ClientCEP78::new(
            "http://valid-rpc-url".to_string(),
            Some("http://valid-sse-url".to_string()),
            Some(Verbosity::Low),
        )
        .unwrap();

        let result = client.set_sse_url("http://new-sse-url".to_string());
        assert!(result.is_ok());
        assert_eq!(client.get_sse_url(), "http://new-sse-url/events");
    }

    #[test]
    fn test_clientcep78_get_verbosity() {
        let client = ClientCEP78::new(
            "http://valid-rpc-url".to_string(),
            Some("http://valid-sse-url".to_string()),
            Some(Verbosity::High),
        )
        .unwrap();

        assert_eq!(client.get_verbosity(), Verbosity::High);
    }

    #[test]
    fn test_clientcep78_set_verbosity() {
        let mut client = ClientCEP78::new(
            "http://valid-rpc-url".to_string(),
            Some("http://valid-sse-url".to_string()),
            Some(Verbosity::Low),
        )
        .unwrap();

        client.set_verbosity(Verbosity::Medium);
        assert_eq!(client.get_verbosity(), Verbosity::Medium);
    }
}
