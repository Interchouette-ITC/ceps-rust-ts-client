//! Endpoint smoke tests (no NCTL).

#[cfg(test)]
mod tests {
    use ceps_client::{CEP18Client, CEP78Client, CEP85Client, CEP95Client, Verbosity};

    #[test]
    fn cep18_endpoints() {
        let client = CEP18Client::new(
            "http://127.0.0.1:11101",
            Some("http://127.0.0.1:18101".into()),
            Some("casper-net-1".into()),
            Some(Verbosity::High),
        )
        .expect("client");
        assert_eq!(client.rpc_url(), "http://127.0.0.1:11101/rpc");
        assert_eq!(client.sse_url(), Some("http://127.0.0.1:18101/events"));
        assert_eq!(client.verbosity(), Verbosity::High);
    }

    #[test]
    fn cep78_endpoints() {
        let client = CEP78Client::new("http://127.0.0.1:11101", None, None, None).expect("client");
        assert_eq!(client.rpc_url(), "http://127.0.0.1:11101/rpc");
        assert!(client.sse_url().is_none());
    }

    #[test]
    fn cep85_endpoints() {
        let client = CEP85Client::new("http://127.0.0.1:11101", None, None, None).expect("client");
        assert_eq!(client.rpc_url(), "http://127.0.0.1:11101/rpc");
    }

    #[test]
    fn cep95_endpoints() {
        let client = CEP95Client::new("http://127.0.0.1:11101", None, None, None).expect("client");
        assert_eq!(client.rpc_url(), "http://127.0.0.1:11101/rpc");
    }

    #[test]
    fn rejects_empty_rpc() {
        assert!(CEP18Client::new("", None, None, None).is_err());
    }
}
