//! Schema field and CEP schema value types.

use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

/// One runtime / install argument descriptor (`session_args_json` CLType shape).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArgField {
    /// Argument name as expected by the contract / installer.
    pub name: &'static str,
    /// Whether the helper omits the arg when unset.
    pub optional: bool,
    /// Static JSON fragment for the CLType (`"String"`, `{"List":"Key"}`, …).
    type_json: &'static str,
}

impl ArgField {
    /// Create a field descriptor.
    pub const fn new(name: &'static str, type_json: &'static str, optional: bool) -> Self {
        Self {
            name,
            optional,
            type_json,
        }
    }

    /// Required field.
    pub const fn req(name: &'static str, type_json: &'static str) -> Self {
        Self::new(name, type_json, false)
    }

    /// Optional field.
    pub const fn opt(name: &'static str, type_json: &'static str) -> Self {
        Self::new(name, type_json, true)
    }

    /// CLType as `serde_json::Value` (casper-client `session_args_json` shape).
    pub fn cl_type(&self) -> Value {
        serde_json::from_str(self.type_json).unwrap_or_else(|e| {
            panic!(
                "invalid static CLType JSON for {}: {e}: {}",
                self.name, self.type_json
            )
        })
    }
}

impl Serialize for ArgField {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("ArgField", 3)?;
        state.serialize_field("name", self.name)?;
        state.serialize_field("type", &self.cl_type())?;
        state.serialize_field("optional", &self.optional)?;
        state.end()
    }
}

/// Full schema for one CEP (install + mutate entrypoints).
#[derive(Debug, Clone, Serialize)]
pub struct CepSchema {
    /// Stable id (`cep18`, `cep78`, `cep85`, `cep95`).
    pub cep: &'static str,
    /// Installer session args.
    pub install: Vec<ArgField>,
    /// Entrypoint name → runtime args.
    pub entrypoints: BTreeMap<&'static str, Vec<ArgField>>,
}

impl CepSchema {
    pub(crate) fn from_parts(
        cep: &'static str,
        install: &'static [ArgField],
        entrypoints: &'static [(&'static str, &'static [ArgField])],
    ) -> Self {
        let mut map = BTreeMap::new();
        for (name, fields) in entrypoints {
            map.insert(*name, fields.to_vec());
        }
        Self {
            cep,
            install: install.to_vec(),
            entrypoints: map,
        }
    }
}

/// Common CLType JSON fragments (static strings for [`ArgField`]).
pub mod ty {
    /// `String`
    pub const STRING: &str = r#""String""#;
    /// `U8`
    pub const U8: &str = r#""U8""#;
    /// `U64`
    pub const U64: &str = r#""U64""#;
    /// `U256`
    pub const U256: &str = r#""U256""#;
    /// `Bool`
    pub const BOOL: &str = r#""Bool""#;
    /// `Key`
    pub const KEY: &str = r#""Key""#;
    /// `List (Key)`
    pub const LIST_KEY: &str = r#"{"List":"Key"}"#;
    /// `List (U256)`
    pub const LIST_U256: &str = r#"{"List":"U256"}"#;
    /// `List (U8)` / Bytes
    pub const LIST_U8: &str = r#"{"List":"U8"}"#;
    /// `Option (List (U8))`
    pub const OPTION_LIST_U8: &str = r#"{"Option":{"List":"U8"}}"#;
    /// `List (Tuple2 (String, String))`
    pub const LIST_STRING_PAIR: &str = r#"{"List":{"Tuple2":["String","String"]}}"#;
}
