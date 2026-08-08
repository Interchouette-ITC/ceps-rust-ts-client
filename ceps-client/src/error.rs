//! Library error type wrapping SDK failures and CEP contract user errors.

use casper_rust_wasm_sdk::types::sdk_error::SdkError;
use std::fmt;
use thiserror::Error;

/// Alias for `Result<T, CepError>`.
pub type Result<T> = std::result::Result<T, CepError>;

/// Errors produced by `ceps-client`.
#[derive(Debug, Error)]
pub enum CepError {
    /// Invalid or empty endpoint URL.
    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    /// Contract hash has not been set on the client.
    #[error("contract hash is not set")]
    ContractHashMissing,

    /// Failed to parse a hash or key string.
    #[error("invalid hash or key: {0}")]
    InvalidHash(String),

    /// Missing required argument for a mutate or install call.
    #[error("missing argument: {0}")]
    MissingArgument(String),

    /// Invalid argument combination or value.
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    /// Underlying SDK error.
    #[error(transparent)]
    Sdk(Box<SdkError>),

    /// Wait-for-processed timed out or SSE failed.
    #[error("wait for transaction failed: {0}")]
    WaitFailed(String),

    /// Transaction executed with an on-chain failure.
    #[error("execution error: {message}")]
    Execution {
        /// Human-readable execution error message from the node.
        message: String,
        /// Parsed `User error: N` code when present.
        user_error: Option<u16>,
        /// CEP family that owns the user-error map, when known.
        cep: Option<CepKind>,
    },

    /// Query returned no stored value.
    #[error("query returned empty result for {0}")]
    EmptyQuery(String),

    /// Failed to decode a CL value from global state / dictionary.
    #[error("failed to decode value: {0}")]
    Decode(String),

    /// I/O error (for example reading a WASM file).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Catch-all for unexpected failures.
    #[error("{0}")]
    Other(String),
}

/// Which CEP owns a user-error code space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CepKind {
    /// CEP-18 fungible token (user errors `60000+`).
    Cep18,
    /// CEP-78 enhanced NFT (user errors `1..=180`).
    Cep78,
    /// CEP-85 multi-token (user errors `1..=91`).
    Cep85,
}

impl fmt::Display for CepKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cep18 => write!(f, "CEP-18"),
            Self::Cep78 => write!(f, "CEP-78"),
            Self::Cep85 => write!(f, "CEP-85"),
        }
    }
}

impl From<SdkError> for CepError {
    fn from(value: SdkError) -> Self {
        CepError::Sdk(Box::new(value))
    }
}

impl From<Box<SdkError>> for CepError {
    fn from(value: Box<SdkError>) -> Self {
        CepError::Sdk(value)
    }
}

impl CepError {
    /// Build an [`CepError::Execution`] from a node error string.
    pub fn from_execution_message(message: impl Into<String>, cep: Option<CepKind>) -> Self {
        let message = message.into();
        let user_error = parse_user_error(&message);
        Self::Execution {
            message,
            user_error,
            cep,
        }
    }

    /// Return the parsed user-error code when this is an execution failure.
    pub fn user_error_code(&self) -> Option<u16> {
        match self {
            Self::Execution { user_error, .. } => *user_error,
            _ => None,
        }
    }
}

/// Extract `N` from a string containing `User error: N`.
pub fn parse_user_error(message: &str) -> Option<u16> {
    const MARKER: &str = "User error: ";
    let start = message.find(MARKER)? + MARKER.len();
    let rest = message[start..].trim_start();
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_user_error_code() {
        assert_eq!(parse_user_error("ApiError::User error: 60001"), Some(60001));
        assert_eq!(parse_user_error("User error: 42 at entry"), Some(42));
        assert_eq!(parse_user_error("no code here"), None);
    }
}
