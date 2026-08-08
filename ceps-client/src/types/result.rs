//! Result of a mutate / install call.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Outcome of putting (and optionally waiting for) a transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallResult {
    /// Transaction hash as hex.
    pub transaction_hash: String,
    /// Raw put-transaction API response (JSON).
    pub put_result: Value,
    /// Execution result JSON when wait succeeded and a result was available.
    pub execution_result: Option<Value>,
}

impl CallResult {
    /// Construct from a transaction hash and put response.
    pub fn new(transaction_hash: impl Into<String>, put_result: Value) -> Self {
        Self {
            transaction_hash: transaction_hash.into(),
            put_result,
            execution_result: None,
        }
    }

    /// Attach an execution result after waiting.
    pub fn with_execution(mut self, execution_result: Value) -> Self {
        self.execution_result = Some(execution_result);
        self
    }
}
