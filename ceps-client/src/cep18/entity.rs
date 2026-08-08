//! Prefixed key encoding for CEP-18 (account-hash- / hash-).

use crate::error::{CepError, Result};
use crate::types::strip_hash_prefix;
use casper_rust_wasm_sdk::types::public_key::PublicKey;

/// Normalize an identity string into a CL Key prefixed form for CEP-18.
///
/// Accepts:
/// - already-prefixed keys (`account-hash-…`, `hash-…`, `uref-…`, `entity-…`)
/// - `public-key-<hex>` or raw public-key hex (algo tag + key) → `account-hash-…`
/// - bare 64-hex (treated as `hash-…`)
pub fn prefixed_key(input: &str) -> Result<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(CepError::InvalidHash("empty key".into()));
    }
    if let Some(hex) = trimmed.strip_prefix("public-key-") {
        return account_hash_from_public_key_hex(hex);
    }
    if trimmed.contains('-') {
        return Ok(trimmed.to_string());
    }
    // Public keys include an algo tag byte (typically 66 hex chars).
    if trimmed.len() != 64 && trimmed.len() >= 66 && trimmed.chars().all(|c| c.is_ascii_hexdigit())
    {
        if let Ok(account) = account_hash_from_public_key_hex(trimmed) {
            return Ok(account);
        }
    }
    let hex = strip_hash_prefix(trimmed);
    if hex.len() == 64 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Ok(format!("hash-{hex}"));
    }
    Err(CepError::InvalidHash(format!(
        "unsupported CEP-18 key form: {input}"
    )))
}

fn account_hash_from_public_key_hex(public_key_hex: &str) -> Result<String> {
    let pk = PublicKey::new(public_key_hex)
        .map_err(|e| CepError::InvalidHash(format!("public key: {e}")))?;
    Ok(pk.to_account_hash().to_formatted_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_hex_becomes_hash() {
        let hex = "b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f";
        assert_eq!(prefixed_key(hex).unwrap(), format!("hash-{hex}"));
    }

    #[test]
    fn empty_rejected() {
        assert!(prefixed_key("").is_err());
    }

    #[test]
    fn keeps_account_hash_prefix() {
        let key = "account-hash-b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f";
        assert_eq!(prefixed_key(key).unwrap(), key);
    }
}
