//! CEP-85 entity key prefixes (`entity-account-` / `entity-contract-`).

use crate::error::{CEPError, Result};
use crate::types::strip_hash_prefix;

/// Normalize an identity into CEP-85 entity-prefixed form.
///
/// - `account-hash-X` → `entity-account-X`
/// - `hash-X` / bare hex → `entity-contract-X`
/// - already `entity-*` → unchanged
pub fn prefixed_key(input: &str) -> Result<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(CEPError::InvalidHash("empty key".into()));
    }
    if trimmed.starts_with("entity-") {
        return Ok(trimmed.to_string());
    }
    if let Some(hex) = trimmed.strip_prefix("account-hash-") {
        return Ok(format!("entity-account-{hex}"));
    }
    if let Some(hex) = trimmed.strip_prefix("hash-") {
        return Ok(format!("entity-contract-{hex}"));
    }
    if let Some(hex) = trimmed.strip_prefix("entity-contract-") {
        return Ok(format!("entity-contract-{hex}"));
    }
    let hex = strip_hash_prefix(trimmed);
    if hex.len() == 64 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Ok(format!("entity-contract-{hex}"));
    }
    Err(CEPError::InvalidHash(format!(
        "unsupported CEP-85 key form: {input}"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_account_and_contract() {
        let a = prefixed_key(
            "account-hash-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        )
        .unwrap();
        assert_eq!(
            a,
            "entity-account-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        );
        let c =
            prefixed_key("hash-bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
                .unwrap();
        assert_eq!(
            c,
            "entity-contract-bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
        );
    }
}
