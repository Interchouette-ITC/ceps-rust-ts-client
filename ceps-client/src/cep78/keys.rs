//! Prefixed keys and dictionary item keys for CEP-78.

use crate::error::{CepError, Result};
use crate::types::strip_hash_prefix;
use casper_rust_wasm_sdk::helpers::make_dictionary_item_key;
use casper_rust_wasm_sdk::types::key::Key;

/// Normalize to a CL Key prefixed string (`account-hash-…` / `hash-…` / `entity-…`).
pub fn prefixed_key(input: &str) -> Result<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(CepError::InvalidHash("empty key".into()));
    }
    if trimmed.contains('-') {
        return Ok(trimmed.to_string());
    }
    let hex = strip_hash_prefix(trimmed);
    if hex.len() == 64 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Ok(format!("hash-{hex}"));
    }
    Err(CepError::InvalidHash(format!(
        "unsupported CEP-78 key form: {input}"
    )))
}

/// Dictionary item key for balances / acl_whitelist: hex body without prefix.
pub fn key_hex_body(input: &str) -> Result<String> {
    let prefixed = prefixed_key(input)?;
    Ok(strip_hash_prefix(&prefixed))
}

/// Operator dictionary item: blake2b(owner_key_bytes ‖ operator_key_bytes).
pub fn operator_dictionary_key(owner: &str, operator: &str) -> Result<String> {
    let owner_key = Key::from_formatted_str(&prefixed_key(owner)?)
        .map_err(|e| CepError::InvalidHash(format!("owner key: {e}")))?;
    let operator_key = Key::from_formatted_str(&prefixed_key(operator)?)
        .map_err(|e| CepError::InvalidHash(format!("operator key: {e}")))?;
    Ok(make_dictionary_item_key(&owner_key, &operator_key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_account_hash_prefix() {
        let body = key_hex_body(
            "account-hash-b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f",
        )
        .unwrap();
        assert_eq!(
            body,
            "b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f"
        );
    }

    #[test]
    fn operator_key_len() {
        let owner =
            "account-hash-b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f";
        let op = "account-hash-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        assert_eq!(operator_dictionary_key(owner, op).unwrap().len(), 64);
    }
}
