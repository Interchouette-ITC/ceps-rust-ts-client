//! Declarative CEP install / entrypoint argument schema.
//!
//! Source of truth for `session_args_json` argument names and CLTypes used by
//! install and mutate helpers. Enable with the `schema` feature (pulled in by
//! default `client`).

pub mod cep18;
pub mod cep78;
pub mod cep85;
pub mod cep95;
mod types;

pub use types::{ArgField, CepSchema};

use serde_json::Value;

/// Supported CEP identifiers for schema export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CepId {
    /// CEP-18 fungible token.
    Cep18,
    /// CEP-78 enhanced NFT.
    Cep78,
    /// CEP-85 multi-token.
    Cep85,
    /// CEP-95 NFT (Odra).
    Cep95,
}

impl CepId {
    /// Stable string id (`cep18`, …).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cep18 => "cep18",
            Self::Cep78 => "cep78",
            Self::Cep85 => "cep85",
            Self::Cep95 => "cep95",
        }
    }

    /// Parse a stable id string.
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "cep18" | "18" => Some(Self::Cep18),
            "cep78" | "78" => Some(Self::Cep78),
            "cep85" | "85" | "cep1155" => Some(Self::Cep85),
            "cep95" | "95" => Some(Self::Cep95),
            _ => None,
        }
    }
}

/// All CEPs with a published schema.
pub const fn supported_ceps() -> &'static [CepId] {
    &[CepId::Cep18, CepId::Cep78, CepId::Cep85, CepId::Cep95]
}

/// Full schema for `cep`.
pub fn schema(cep: CepId) -> CepSchema {
    match cep {
        CepId::Cep18 => CepSchema::from_parts(cep.as_str(), cep18::INSTALL, cep18::ENTRYPOINTS),
        CepId::Cep78 => CepSchema::from_parts(cep.as_str(), cep78::INSTALL, cep78::ENTRYPOINTS),
        CepId::Cep85 => CepSchema::from_parts(cep.as_str(), cep85::INSTALL, cep85::ENTRYPOINTS),
        CepId::Cep95 => CepSchema::from_parts(cep.as_str(), cep95::INSTALL, cep95::ENTRYPOINTS),
    }
}

/// Schema as `serde_json::Value`.
pub fn schema_value(cep: CepId) -> Value {
    serde_json::to_value(schema(cep)).expect("CepSchema serialization is infallible")
}

/// Schema as a JSON string.
pub fn schema_json(cep: CepId) -> String {
    serde_json::to_string(&schema(cep)).expect("CepSchema serialization is infallible")
}

/// CEP-18 schema JSON.
pub fn cep18_schema_json() -> String {
    schema_json(CepId::Cep18)
}

/// CEP-78 schema JSON.
pub fn cep78_schema_json() -> String {
    schema_json(CepId::Cep78)
}

/// CEP-85 schema JSON.
pub fn cep85_schema_json() -> String {
    schema_json(CepId::Cep85)
}

/// CEP-95 schema JSON.
pub fn cep95_schema_json() -> String {
    schema_json(CepId::Cep95)
}

pub use cep18 as cep18_fields;
pub use cep78 as cep78_fields;
pub use cep85 as cep85_fields;
pub use cep95 as cep95_fields;

/// Build a [`crate::core::JsonArg`] from a schema field and value (client builds).
#[cfg(feature = "client")]
pub fn arg(field: &ArgField, value: Value) -> crate::core::JsonArg {
    crate::core::JsonArg::new(field.name, field.cl_type(), value)
}

/// Assert every emitted `session_args_json` arg name/type appears in `schema(cep).install`.
#[cfg(all(test, feature = "client"))]
pub fn assert_install_json_matches_schema(cep: CepId, args_json: &str) {
    let emitted: Vec<Value> =
        serde_json::from_str(args_json).expect("install args must be JSON array");
    let s = schema(cep);
    let by_name: std::collections::HashMap<&str, &ArgField> =
        s.install.iter().map(|f| (f.name, f)).collect();
    for arg_v in &emitted {
        let name = arg_v["name"].as_str().expect("arg name");
        let field = by_name
            .get(name)
            .unwrap_or_else(|| panic!("{} install: unexpected arg {name}", cep.as_str()));
        assert_eq!(
            arg_v["type"],
            field.cl_type(),
            "{} install {name} type drift",
            cep.as_str()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const EXPECTED_ENTRYPOINTS: &[(CepId, &[&str])] = &[
        (
            CepId::Cep18,
            &[
                "upgrade",
                "transfer",
                "transfer_from",
                "approve",
                "increase_allowance",
                "decrease_allowance",
                "mint",
                "burn",
                "change_security",
                "change_events_mode",
            ],
        ),
        (
            CepId::Cep78,
            &[
                "upgrade",
                "mint",
                "burn",
                "transfer",
                "register_owner",
                "approve",
                "revoke",
                "set_approval_for_all",
                "set_token_metadata",
                "set_variables",
            ],
        ),
        (
            CepId::Cep85,
            &[
                "upgrade",
                "mint",
                "batch_mint",
                "burn",
                "batch_burn",
                "transfer_from",
                "batch_transfer_from",
                "set_approval_for_all",
                "set_uri",
                "set_total_supply_of",
                "set_total_supply_of_batch",
                "change_security",
                "set_modalities",
            ],
        ),
        (
            CepId::Cep95,
            &[
                "transfer_from",
                "safe_transfer_from",
                "approve",
                "revoke_approval",
                "approve_for_all",
                "revoke_approval_for_all",
                "mint",
                "burn",
                "transfer_ownership",
            ],
        ),
    ];

    #[test]
    fn supported_ceps_lists_four() {
        assert_eq!(supported_ceps().len(), 4);
        assert_eq!(CepId::parse("cep18"), Some(CepId::Cep18));
        assert_eq!(CepId::parse("85"), Some(CepId::Cep85));
    }

    #[test]
    fn schemas_cover_install_and_entrypoints() {
        for &(cep, eps) in EXPECTED_ENTRYPOINTS {
            let s = schema(cep);
            assert!(!s.install.is_empty(), "{cep:?} install empty");
            for name in eps {
                assert!(
                    s.entrypoints.contains_key(name),
                    "{cep:?} missing entrypoint {name}"
                );
                assert!(
                    !s.entrypoints[*name].is_empty(),
                    "{cep:?} {name} has no args"
                );
            }
        }
    }

    #[test]
    fn cl_types_round_trip_casper_types() {
        for cep in supported_ceps() {
            let s = schema(*cep);
            for field in s.install.iter().chain(s.entrypoints.values().flatten()) {
                let _: casper_types::CLType = serde_json::from_value(field.cl_type())
                    .unwrap_or_else(|e| panic!("{} {}: {e}", cep.as_str(), field.name));
            }
        }
    }

    #[test]
    fn schema_json_round_trip_stable() {
        for cep in supported_ceps() {
            let raw = schema_json(*cep);
            let v1: Value = serde_json::from_str(&raw).unwrap();
            let raw2 = serde_json::to_string(&v1).unwrap();
            let v2: Value = serde_json::from_str(&raw2).unwrap();
            assert_eq!(v1, v2, "{}", cep.as_str());
            assert_eq!(v1["cep"], json!(cep.as_str()));
            assert!(v1["install"].is_array());
            assert!(v1["entrypoints"].is_object());
        }
    }

    #[test]
    fn per_cep_shortcuts_match_schema_json() {
        assert_eq!(cep18_schema_json(), schema_json(CepId::Cep18));
        assert_eq!(cep78_schema_json(), schema_json(CepId::Cep78));
        assert_eq!(cep85_schema_json(), schema_json(CepId::Cep85));
        assert_eq!(cep95_schema_json(), schema_json(CepId::Cep95));
    }
}
