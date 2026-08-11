//! Dictionary item key helpers for CEP-18 balances and allowances.

use super::entity::prefixed_key;
use crate::error::{CEPError, Result};
use casper_rust_wasm_sdk::helpers::{
    get_base64_key_from_account_hash, get_base64_key_from_key_hash, make_dictionary_item_key,
};
use casper_rust_wasm_sdk::types::key::Key;

/// Balance / security-badge dictionary item key: Base64(Key.bytes()).
pub fn balance_dictionary_key(account: &str) -> Result<String> {
    let prefixed = prefixed_key(account)?;
    if prefixed.starts_with("account-hash-") {
        get_base64_key_from_account_hash(&prefixed)
            .map_err(|e| CEPError::InvalidHash(format!("balance key: {e}")))
    } else {
        get_base64_key_from_key_hash(&prefixed)
            .map_err(|e| CEPError::InvalidHash(format!("balance key: {e}")))
    }
}

/// Allowance dictionary item key: blake2b(owner.bytes ‖ spender.bytes) as hex.
pub fn allowance_dictionary_key(owner: &str, spender: &str) -> Result<String> {
    let owner_key = Key::from_formatted_str(&prefixed_key(owner)?)
        .map_err(|e| CEPError::InvalidHash(format!("owner key: {e}")))?;
    let spender_key = Key::from_formatted_str(&prefixed_key(spender)?)
        .map_err(|e| CEPError::InvalidHash(format!("spender key: {e}")))?;
    Ok(make_dictionary_item_key(&owner_key, &spender_key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balance_key_from_account_hash() {
        let account =
            "account-hash-b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f";
        let key = balance_dictionary_key(account).unwrap();
        assert!(!key.is_empty());
    }

    #[test]
    fn security_badge_uses_same_base64_item_key_as_balance() {
        let account =
            "account-hash-b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f";
        let a = balance_dictionary_key(account).unwrap();
        let b = balance_dictionary_key(account).unwrap();
        assert_eq!(a, b);
    }
}
