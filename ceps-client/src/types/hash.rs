//! Contract hash / package hash targeting helpers.

use crate::error::{CEPError, Result};

/// Bound contract identity used for entrypoint calls and state queries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractTarget {
    /// Hex contract (entity) hash without prefix.
    pub contract_hash: String,
    /// Hex package hash without prefix, when known.
    pub package_hash: Option<String>,
}

impl ContractTarget {
    /// Build a target from hex or prefixed hash strings.
    pub fn new(
        contract_hash: impl AsRef<str>,
        package_hash: Option<impl AsRef<str>>,
    ) -> Result<Self> {
        let contract_hash = strip_hash_prefix(contract_hash.as_ref());
        if contract_hash.is_empty() || !is_hex64(&contract_hash) {
            return Err(CEPError::InvalidHash(
                "contract hash must be 64 hex chars (optional prefix allowed)".into(),
            ));
        }
        let package_hash = match package_hash {
            Some(p) => {
                let hex = strip_hash_prefix(p.as_ref());
                if hex.is_empty() {
                    None
                } else if !is_hex64(&hex) {
                    return Err(CEPError::InvalidHash(
                        "package hash must be 64 hex chars (optional prefix allowed)".into(),
                    ));
                } else {
                    Some(hex)
                }
            }
            None => None,
        };
        Ok(Self {
            contract_hash,
            package_hash,
        })
    }

    /// `hash-{contract}` form used for named-key / dictionary queries.
    pub fn query_key(&self) -> String {
        format!("hash-{}", self.contract_hash)
    }

    /// Prefixed package key when present (`package-{hex}`).
    pub fn package_key(&self) -> Option<String> {
        self.package_hash.as_ref().map(|h| format!("package-{h}"))
    }
}

/// Strip `hash-`, `contract-`, `package-`, `entity-contract-`, or any `*-` prefix.
pub fn strip_hash_prefix(value: &str) -> String {
    let trimmed = value.trim();
    if let Some(idx) = trimmed.rfind('-') {
        // Keep only the trailing hex segment after the last dash.
        let candidate = &trimmed[idx + 1..];
        if is_hex64(candidate) {
            return candidate.to_ascii_lowercase();
        }
    }
    trimmed.to_ascii_lowercase()
}

fn is_hex64(s: &str) -> bool {
    s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_common_prefixes() {
        let hex = "b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f";
        assert_eq!(strip_hash_prefix(&format!("hash-{hex}")), hex);
        assert_eq!(strip_hash_prefix(&format!("entity-contract-{hex}")), hex);
        assert_eq!(strip_hash_prefix(&format!("package-{hex}")), hex);
        assert_eq!(strip_hash_prefix(hex), hex);
    }

    #[test]
    fn contract_target_rejects_bad_hash() {
        assert!(ContractTarget::new("deadbeef", None::<&str>).is_err());
    }
}
