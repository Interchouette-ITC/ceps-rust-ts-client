//! Prefixed key encoding for CEP-95 (account-hash- / hash- / entity-).

use crate::error::{CEPError, Result};
use crate::types::strip_hash_prefix;
use casper_rust_wasm_sdk::types::public_key::PublicKey;

/// Normalize an identity string into a CL Key prefixed form for CEP-95.
pub fn prefixed_key(input: &str) -> Result<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(CEPError::InvalidHash("empty key".into()));
    }
    if let Some(hex) = trimmed.strip_prefix("public-key-") {
        return account_hash_from_public_key_hex(hex);
    }
    if trimmed.contains('-') {
        return Ok(trimmed.to_string());
    }
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
    Err(CEPError::InvalidHash(format!(
        "unsupported CEP-95 key form: {input}"
    )))
}

fn account_hash_from_public_key_hex(public_key_hex: &str) -> Result<String> {
    let pk = PublicKey::new(public_key_hex)
        .map_err(|e| CEPError::InvalidHash(format!("public key: {e}")))?;
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
    fn keeps_account_hash_prefix() {
        let key = "account-hash-b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f";
        assert_eq!(prefixed_key(key).unwrap(), key);
    }
}
