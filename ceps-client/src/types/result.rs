//! Result of a mutate / install call.

use casper_rust_wasm_sdk::SSE::CESParseResult;
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
    /// Soft-fail CES decode rows when wait attached execution and a contract hash was bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ces_events: Option<Vec<CESParseResult>>,
}

impl CallResult {
    /// Construct from a transaction hash and put response.
    pub fn new(transaction_hash: impl Into<String>, put_result: Value) -> Self {
        Self {
            transaction_hash: transaction_hash.into(),
            put_result,
            execution_result: None,
            ces_events: None,
        }
    }

    /// Attach an execution result after waiting.
    pub fn with_execution(mut self, execution_result: Value) -> Self {
        self.execution_result = Some(execution_result);
        self
    }

    /// Attach CES parse rows (soft-fail list from the SDK parser).
    pub fn with_ces_events(mut self, ces_events: Vec<CESParseResult>) -> Self {
        self.ces_events = Some(ces_events);
        self
    }
}
