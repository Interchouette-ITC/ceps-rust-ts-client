//! Dictionary item key helpers for CEP-95 (Odra / JS storage encodings).

use super::entity::prefixed_key;
use crate::error::{CEPError, Result};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;
use casper_rust_wasm_sdk::helpers::{
    get_base64_key_from_account_hash, get_base64_key_from_key_hash, make_dictionary_item_key,
};
use casper_rust_wasm_sdk::types::key::Key;
use casper_types::bytesrepr::ToBytes;
use casper_types::U256;
use std::str::FromStr;

/// Balance dictionary item key: Base64(Key.to_bytes()) under dict `balances`.
pub fn balance_dictionary_key(owner: &str) -> Result<String> {
    let prefixed = prefixed_key(owner)?;
    if prefixed.starts_with("account-hash-") {
        get_base64_key_from_account_hash(&prefixed)
            .map_err(|e| CEPError::InvalidHash(format!("balance key: {e}")))
    } else {
        get_base64_key_from_key_hash(&prefixed)
            .map_err(|e| CEPError::InvalidHash(format!("balance key: {e}")))
    }
}

/// Token-id dictionary item key (owners / approvals / token_metadata):
/// Base64(U256.to_bytes()) as used by Odra `base64_encoded_key_value_storage`.
pub fn token_id_dictionary_key(token_id: &str) -> Result<String> {
    let id = U256::from_str(token_id.trim())
        .map_err(|e| CEPError::InvalidArgument(format!("token_id U256: {e}")))?;
    let bytes = id
        .to_bytes()
        .map_err(|e| CEPError::InvalidArgument(format!("token_id bytes: {e}")))?;
    Ok(BASE64.encode(bytes))
}

/// Operator dictionary item key: hex(blake2b-256(owner.bytes ‖ operator.bytes)).
pub fn operator_dictionary_key(owner: &str, operator: &str) -> Result<String> {
    let owner_key = Key::from_formatted_str(&prefixed_key(owner)?)
        .map_err(|e| CEPError::InvalidHash(format!("owner key: {e}")))?;
    let operator_key = Key::from_formatted_str(&prefixed_key(operator)?)
        .map_err(|e| CEPError::InvalidHash(format!("operator key: {e}")))?;
    Ok(make_dictionary_item_key(&owner_key, &operator_key))
}

/// Odra `state` dictionary item key for a small-index Var path (all indices <= 15).
///
/// Odra module field indices are 1-based. Tip `OwnedCep95` stores Ownable owner at
/// path `[ownable=1, owner=1]` → packed index `0x11`, then blake2b-256 hex (ASCII)
/// under dict `state`.
pub fn odra_state_var_key(path: &[u8]) -> Result<String> {
    if path.is_empty() {
        return Err(CEPError::InvalidArgument("odra state path empty".into()));
    }
    if path.iter().any(|&idx| idx > 15) {
        return Err(CEPError::InvalidArgument(
            "odra state path requires indices <= 15 for small encoding".into(),
        ));
    }
    let index: u32 = path
        .iter()
        .fold(0u32, |acc, &idx| (acc << 4) + u32::from(idx));
    let index_bytes = index.to_be_bytes();
    let mut hasher = Blake2bVar::new(32)
        .map_err(|_| CEPError::InvalidArgument("blake2b-256 hasher init failed".into()))?;
    hasher.update(&index_bytes);
    let mut hash = [0u8; 32];
    hasher
        .finalize_variable(&mut hash)
        .map_err(|_| CEPError::InvalidArgument("blake2b-256 finalize failed".into()))?;
    Ok(hex::encode(hash))
}

/// Ownable owner Var key for tip `OwnedCep95` (`ownable` field 1, `owner` field 1).
pub fn ownable_owner_state_key() -> Result<String> {
    odra_state_var_key(&[1, 1])
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
    fn token_id_key_is_stable_base64() {
        let a = token_id_dictionary_key("42").unwrap();
        let b = token_id_dictionary_key("42").unwrap();
        assert_eq!(a, b);
        assert!(!a.is_empty());
    }

    #[test]
    fn operator_key_is_hex() {
        let owner = "account-hash-b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f";
        let operator =
            "account-hash-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let key = operator_dictionary_key(owner, operator).unwrap();
        assert_eq!(key.len(), 64);
        assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn ownable_owner_state_key_stable_64_hex() {
        let key = ownable_owner_state_key().unwrap();
        assert_eq!(key.len(), 64);
        assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(key, ownable_owner_state_key().unwrap());
        // Path [1,1] → packed 0x11; must not collide with legacy wrong [0,0] key.
        assert_ne!(key, odra_state_var_key(&[0, 0]).unwrap());
        assert_eq!(key, odra_state_var_key(&[1, 1]).unwrap());
    }
}
