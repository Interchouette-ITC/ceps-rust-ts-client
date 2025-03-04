#[cfg(test)]
#[allow(dead_code)]
mod tests_cep18 {
    use common::{ClientCEP18, Verbosity};

    #[test]
    fn test_new_client() {
        let client = ClientCEP18::new(
            "http://127.0.0.1:11101".to_string(),
            Some("http://127.0.0.1:28101".to_string()),
            Some(Verbosity::High),
        )
        .expect("Failed to create client");
        assert_eq!(client.get_rpc_url(), "http://127.0.0.1:11101/rpc");
        assert_eq!(client.get_sse_url(), "http://127.0.0.1:28101/events");
        assert_eq!(client.get_verbosity(), Verbosity::High);
    }

    #[test]
    fn test_new_client_without_sse_url() {
        let client = ClientCEP18::new("http://127.0.0.1:11101".to_string(), None, None)
            .expect("Failed to create client");
        assert_eq!(client.get_rpc_url(), "http://127.0.0.1:11101/rpc");
        assert_eq!(client.get_sse_url(), "");
        assert_eq!(client.get_verbosity(), Verbosity::Low);
    }

    #[test]
    fn test_set_rpc_url() {
        let mut client = ClientCEP18::new(
            "http://127.0.0.1:11101".to_string(),
            Some("http://127.0.0.1:28101".to_string()),
            Some(Verbosity::Low),
        )
        .expect("Failed to create client");
        assert_eq!(client.get_rpc_url(), "http://127.0.0.1:11101/rpc");

        client
            .set_rpc_url("http://127.0.0.1:12345".to_string())
            .expect("Failed to set RPC URL");
        assert_eq!(client.get_rpc_url(), "http://127.0.0.1:12345/rpc");
    }

    #[test]
    fn test_set_sse_url() {
        let mut client = ClientCEP18::new(
            "http://127.0.0.1:11101".to_string(),
            Some("http://127.0.0.1:28101".to_string()),
            Some(Verbosity::Low),
        )
        .expect("Failed to create client");
        assert_eq!(client.get_sse_url(), "http://127.0.0.1:28101/events");

        client
            .set_sse_url("http://127.0.0.1:55555".to_string())
            .expect("Failed to set SSE URL");
        assert_eq!(client.get_sse_url(), "http://127.0.0.1:55555/events");
    }

    #[test]
    fn test_verbosity() {
        let mut client = ClientCEP18::new(
            "http://127.0.0.1:11101".to_string(),
            Some("http://127.0.0.1:28101".to_string()),
            Some(Verbosity::Low),
        )
        .expect("Failed to create client");
        assert_eq!(client.get_verbosity(), Verbosity::Low);

        client.set_verbosity(Verbosity::High);
        assert_eq!(client.get_verbosity(), Verbosity::High);
    }

    fn test_empty_urls() {
        let client = ClientCEP18::new("".to_string(), Some("".to_string()), Some(Verbosity::Low));
        assert!(
            client.is_err(),
            "Expected an error for empty URLs, but got Ok"
        );
    }

    fn test_maformed_rpc_url() {
        let client = ClientCEP18::new(
            "test".to_string(),
            Some("".to_string()),
            Some(Verbosity::Low),
        );
        assert!(
            client.is_err(),
            "Expected an error for empty URLs, but got Ok"
        );
    }

    fn test_maformed_sse_url() {
        let client = ClientCEP18::new(
            "http://127.0.0.1:11101".to_string(),
            Some("test".to_string()),
            Some(Verbosity::Low),
        );
        assert!(
            client.is_err(),
            "Expected an error for empty URLs, but got Ok"
        );
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests_cep78 {
    use common::{ClientCEP78, Verbosity};

    #[test]
    fn test_new_client() {
        let client = ClientCEP78::new(
            "http://127.0.0.1:11101".to_string(),
            Some("http://127.0.0.1:28101".to_string()),
            Some(Verbosity::High),
        )
        .expect("Failed to create client");
        assert_eq!(client.get_rpc_url(), "http://127.0.0.1:11101/rpc");
        assert_eq!(client.get_sse_url(), "http://127.0.0.1:28101/events");
        assert_eq!(client.get_verbosity(), Verbosity::High);
    }

    #[test]
    fn test_new_client_without_sse_url() {
        let client = ClientCEP78::new("http://127.0.0.1:11101".to_string(), None, None)
            .expect("Failed to create client");
        assert_eq!(client.get_rpc_url(), "http://127.0.0.1:11101/rpc");
        assert_eq!(client.get_sse_url(), "");
        assert_eq!(client.get_verbosity(), Verbosity::Low);
    }

    #[test]
    fn test_set_rpc_url() {
        let mut client = ClientCEP78::new(
            "http://127.0.0.1:11101".to_string(),
            Some("http://127.0.0.1:28101".to_string()),
            Some(Verbosity::Low),
        )
        .expect("Failed to create client");
        assert_eq!(client.get_rpc_url(), "http://127.0.0.1:11101/rpc");

        client
            .set_rpc_url("http://127.0.0.1:12345".to_string())
            .expect("Failed to set RPC URL");
        assert_eq!(client.get_rpc_url(), "http://127.0.0.1:12345/rpc");
    }

    #[test]
    fn test_set_sse_url() {
        let mut client = ClientCEP78::new(
            "http://127.0.0.1:11101".to_string(),
            Some("http://127.0.0.1:28101".to_string()),
            Some(Verbosity::Low),
        )
        .expect("Failed to create client");
        assert_eq!(client.get_sse_url(), "http://127.0.0.1:28101/events");

        client
            .set_sse_url("http://127.0.0.1:55555".to_string())
            .expect("Failed to set SSE URL");
        assert_eq!(client.get_sse_url(), "http://127.0.0.1:55555/events");
    }

    #[test]
    fn test_verbosity() {
        let mut client = ClientCEP78::new(
            "http://127.0.0.1:11101".to_string(),
            Some("http://127.0.0.1:28101".to_string()),
            Some(Verbosity::Low),
        )
        .expect("Failed to create client");
        assert_eq!(client.get_verbosity(), Verbosity::Low);

        client.set_verbosity(Verbosity::High);
        assert_eq!(client.get_verbosity(), Verbosity::High);
    }
    fn test_empty_urls() {
        let client = ClientCEP78::new("".to_string(), Some("".to_string()), Some(Verbosity::Low));
        assert!(
            client.is_err(),
            "Expected an error for empty URLs, but got Ok"
        );
    }

    fn test_maformed_rpc_url() {
        let client = ClientCEP78::new(
            "test".to_string(),
            Some("".to_string()),
            Some(Verbosity::Low),
        );
        assert!(
            client.is_err(),
            "Expected an error for empty URLs, but got Ok"
        );
    }

    fn test_maformed_sse_url() {
        let client = ClientCEP78::new(
            "http://127.0.0.1:11101".to_string(),
            Some("test".to_string()),
            Some(Verbosity::Low),
        );
        assert!(
            client.is_err(),
            "Expected an error for empty URLs, but got Ok"
        );
    }
}
