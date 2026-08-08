//! Prefixed key encoding for CEP-18 (account-hash- / hash-).

use crate::error::{CepError, Result};
use crate::types::strip_hash_prefix;

/// Normalize an identity string into a CL Key prefixed form for CEP-18.
///
/// Accepts:
/// - already-prefixed keys (`account-hash-…`, `hash-…`, `uref-…`, `entity-…`)
/// - bare 64-hex (treated as `hash-…`)
/// - `public-key-…` / raw public key hex → hashed to `account-hash-…` via SDK helpers when possible
pub fn prefixed_key(input: &str) -> Result<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(CepError::InvalidHash("empty key".into()));
    }
    if trimmed.contains('-') {
        // Already prefixed (account-hash-, hash-, uref-, entity-*, package-, …).
        return Ok(trimmed.to_string());
    }
    let hex = strip_hash_prefix(trimmed);
    if hex.len() == 64 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Ok(format!("hash-{hex}"));
    }
    Err(CepError::InvalidHash(format!(
        "unsupported CEP-18 key form: {input}"
    )))
}

/// Account-hash form from a public key hex (with optional algo tag byte in hex).
#[allow(dead_code)]
pub fn account_hash_from_public_key_hex(public_key_hex: &str) -> Result<String> {
    use casper_rust_wasm_sdk::types::public_key::PublicKey;
    let pk = PublicKey::new(public_key_hex)
        .map_err(|e| CepError::InvalidHash(format!("public key: {e}")))?;
    Ok(pk.to_account_hash().to_formatted_string())
}
