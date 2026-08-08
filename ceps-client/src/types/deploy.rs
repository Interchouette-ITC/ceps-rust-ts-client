//! Shared transaction envelope for mutate / install calls.

/// Payment, signing, and wait options shared by all CEP mutate methods.
#[derive(Debug, Clone)]
pub struct DeployParams {
    /// PEM-encoded secret key used to sign the transaction.
    pub secret_key_pem: String,
    /// Payment amount in motes.
    pub payment_amount: String,
    /// Optional chain name override (falls back to client default).
    pub chain_name: Option<String>,
    /// When true (default), wait on SSE until the transaction is processed.
    pub wait: bool,
    /// Optional wait timeout in milliseconds (SDK default when `None`).
    pub wait_timeout_ms: Option<u64>,
}

impl DeployParams {
    /// Build deploy params with wait enabled.
    pub fn new(secret_key_pem: impl Into<String>, payment_amount: impl Into<String>) -> Self {
        Self {
            secret_key_pem: secret_key_pem.into(),
            payment_amount: payment_amount.into(),
            chain_name: None,
            wait: true,
            wait_timeout_ms: None,
        }
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
}
