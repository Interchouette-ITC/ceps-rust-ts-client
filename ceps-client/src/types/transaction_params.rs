//! Shared transaction envelope for mutate / install calls.

/// Payment, signing, put/make, and wait options shared by all CEP mutate methods.
#[derive(Debug, Clone)]
pub struct TransactionParams {
    /// PEM-encoded secret key used to sign the transaction (required when `put` is true).
    pub secret_key_pem: Option<String>,
    /// Payment amount in motes.
    pub payment_amount: String,
    /// Optional chain name override (falls back to client default).
    pub chain_name: Option<String>,
    /// When true (default), wait on SSE until the transaction is processed.
    /// Ignored when [`Self::put`] is false.
    pub wait: bool,
    /// Optional wait timeout in milliseconds (SDK default when `None`).
    pub wait_timeout_ms: Option<u64>,
    /// When true (default), put the transaction to the node. When false, only make it.
    pub put: bool,
    /// Initiator account address (public key hex). Required for unsigned make-only.
    pub initiator_addr: Option<String>,
}

impl TransactionParams {
    /// Build put-path params with a secret key and wait enabled.
    pub fn new(secret_key_pem: impl Into<String>, payment_amount: impl Into<String>) -> Self {
        Self {
            secret_key_pem: Some(secret_key_pem.into()),
            payment_amount: payment_amount.into(),
            chain_name: None,
            wait: true,
            wait_timeout_ms: None,
            put: true,
            initiator_addr: None,
        }
    }

    /// Build make-only params (no put, wait ignored). Supply PEM and/or initiator separately.
    pub fn for_make(payment_amount: impl Into<String>) -> Self {
        Self {
            secret_key_pem: None,
            payment_amount: payment_amount.into(),
            chain_name: None,
            wait: false,
            wait_timeout_ms: None,
            put: false,
            initiator_addr: None,
        }
    }

    /// Disable put: build Transaction JSON only (wait ignored).
    pub fn make_only(mut self) -> Self {
        self.put = false;
        self
    }

    /// Disable SSE wait (put-only).
    pub fn without_wait(mut self) -> Self {
        self.wait = false;
        self
    }

    /// Override chain name for this call.
    pub fn with_chain_name(mut self, chain_name: impl Into<String>) -> Self {
        self.chain_name = Some(chain_name.into());
        self
    }

    /// Set wait timeout in milliseconds.
    pub fn with_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.wait_timeout_ms = Some(timeout_ms);
        self
    }

    /// Set or clear the PEM secret key.
    pub fn with_secret_key_pem(mut self, secret_key_pem: Option<impl Into<String>>) -> Self {
        self.secret_key_pem = secret_key_pem.map(Into::into);
        self
    }

    /// Set initiator address (public key hex) for unsigned make-only.
    pub fn with_initiator_addr(mut self, initiator_addr: impl Into<String>) -> Self {
        self.initiator_addr = Some(initiator_addr.into());
        self
    }

    /// Validate put vs make signing requirements.
    pub fn validate(&self) -> Result<(), String> {
        if self.put {
            match &self.secret_key_pem {
                Some(s) if !s.trim().is_empty() => Ok(()),
                _ => Err("put requires secret_key_pem".into()),
            }
        } else {
            let has_secret = self
                .secret_key_pem
                .as_ref()
                .is_some_and(|s| !s.trim().is_empty());
            let has_initiator = self
                .initiator_addr
                .as_ref()
                .is_some_and(|s| !s.trim().is_empty());
            if has_secret || has_initiator {
                Ok(())
            } else {
                Err("make-only requires secret_key_pem or initiator_addr".into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_requires_secret() {
        let mut put = TransactionParams::new("pem", "1");
        put.secret_key_pem = None;
        assert!(put.validate().unwrap_err().contains("secret_key_pem"));
        assert!(TransactionParams::new("pem", "1").validate().is_ok());
    }

    #[test]
    fn make_only_needs_secret_or_initiator() {
        let bare = TransactionParams::for_make("1");
        assert!(bare.validate().unwrap_err().contains("initiator_addr"));
        let with_init = bare.with_initiator_addr("01ab");
        assert!(with_init.validate().is_ok());
        let with_pem = TransactionParams::for_make("1").with_secret_key_pem(Some("pem"));
        assert!(with_pem.validate().is_ok());
    }

    #[test]
    fn make_only_flag_disables_put() {
        let tx = TransactionParams::new("pem", "1").make_only();
        assert!(!tx.put);
    }

    #[test]
    fn make_only_ignores_wait_semantics() {
        let tx = TransactionParams::new("pem", "1").make_only();
        // put false; wait flag is ignored by core when !put
        assert!(!tx.put);
        assert!(tx.wait);
        let unsigned = TransactionParams::for_make("1")
            .with_initiator_addr("01ab")
            .without_wait();
        assert!(!unsigned.put);
        assert!(!unsigned.wait);
        assert!(unsigned.validate().is_ok());
    }
}
