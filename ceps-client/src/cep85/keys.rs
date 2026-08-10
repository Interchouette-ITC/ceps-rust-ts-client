//! Dictionary item keys for CEP-85.

use super::entity::prefixed_key;
use crate::error::{CEPError, Result};
use casper_rust_wasm_sdk::helpers::make_dictionary_item_key;
use casper_rust_wasm_sdk::types::key::Key;

/// Balance dict key: blake2b(account_key ‖ id_u256) hex.
pub fn balance_dictionary_key(account: &str, id: &str) -> Result<String> {
    let account_key = Key::from_formatted_str(&prefixed_key(account)?)
        .map_err(|e| CEPError::InvalidHash(format!("account key: {e}")))?;
    // Encode id as U256 bytesrepr via casper_types if available through SDK Key helper pattern.
    // make_dictionary_item_key hashes Key ‖ V::to_bytes(); use the decimal string's U256.
    let id_u256 = parse_u256(id)?;
    Ok(make_dictionary_item_key(&account_key, &id_u256))
}

/// Operator dict key: blake2b(owner_key ‖ operator_key) hex.
pub fn operator_dictionary_key(owner: &str, operator: &str) -> Result<String> {
    let owner_key = Key::from_formatted_str(&prefixed_key(owner)?)
        .map_err(|e| CEPError::InvalidHash(format!("owner key: {e}")))?;
    let operator_key = Key::from_formatted_str(&prefixed_key(operator)?)
        .map_err(|e| CEPError::InvalidHash(format!("operator key: {e}")))?;
    Ok(make_dictionary_item_key(&owner_key, &operator_key))
}

fn parse_u256(s: &str) -> Result<casper_types::U256> {
    casper_types::U256::from_dec_str(s).map_err(|e| CEPError::InvalidArgument(format!("U256: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balance_key_stable() {
        let account =
            "account-hash-b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f";
        let key = balance_dictionary_key(account, "1").unwrap();
        assert_eq!(key.len(), 64);
    }
}
