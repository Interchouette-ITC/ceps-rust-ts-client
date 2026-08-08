//! Helpers for building `session_args_json` payloads.

use serde::Serialize;
use serde_json::{json, Value};

/// One runtime argument in casper-client JSON args form.
#[derive(Debug, Clone, Serialize)]
pub struct JsonArg {
    name: String,
    #[serde(rename = "type")]
    arg_type: String,
    value: Value,
}

impl JsonArg {
    /// Create a typed JSON arg.
    pub fn new(name: impl Into<String>, arg_type: impl Into<String>, value: Value) -> Self {
        Self {
            name: name.into(),
            arg_type: arg_type.into(),
            value,
        }
    }
}

/// Build a typed JSON arg (same fields as [`JsonArg::new`]).
pub fn json_arg(name: &str, arg_type: &str, value: Value) -> JsonArg {
    JsonArg::new(name, arg_type, value)
}

/// Serialize args to the JSON array string expected by `set_session_args_json`.
pub fn json_args(args: &[JsonArg]) -> String {
    serde_json::to_string(args).expect("JsonArg serialization is infallible")
}

/// `String` CL value.
pub fn string_arg(name: &str, value: &str) -> JsonArg {
    json_arg(name, "String", Value::String(value.to_string()))
}

/// `U8` CL value.
pub fn u8_arg(name: &str, value: u8) -> JsonArg {
    json_arg(name, "U8", json!(value))
}

/// `U64` CL value.
pub fn u64_arg(name: &str, value: u64) -> JsonArg {
    json_arg(name, "U64", json!(value))
}

/// `U256` from a decimal string (not hex).
pub fn u256_arg(name: &str, value: &str) -> JsonArg {
    json_arg(name, "U256", Value::String(value.to_string()))
}

/// `Bool` CL value.
pub fn bool_arg(name: &str, value: bool) -> JsonArg {
    json_arg(name, "Bool", json!(value))
}

/// `Key` from a prefixed key string (`account-hash-…`, `hash-…`, `entity-…`).
pub fn key_arg(name: &str, prefixed_key: &str) -> JsonArg {
    json_arg(name, "Key", Value::String(prefixed_key.to_string()))
}

/// `List (Key)` values.
pub fn key_list_arg(name: &str, keys: &[String]) -> JsonArg {
    json_arg(name, "List (Key)", json!(keys))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_args_json() {
        let s = json_args(&[string_arg("name", "TOKEN"), u8_arg("decimals", 9)]);
        assert!(s.contains("\"name\":\"name\""));
        assert!(s.contains("TOKEN"));
    }

    #[test]
    fn u256_and_key_list_shapes() {
        let s = json_args(&[
            u256_arg("amount", "1000"),
            key_list_arg(
                "admin_list",
                &[
                    "account-hash-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                        .into(),
                ],
            ),
            bool_arg("flag", true),
        ]);
        assert!(s.contains("U256"));
        assert!(s.contains("List (Key)"));
        assert!(s.contains("Bool"));
        assert!(s.contains("1000"));
    }
}
