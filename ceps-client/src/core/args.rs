//! Helpers for building `session_args_json` payloads.

use serde::Serialize;
use serde_json::{json, Value};

/// One runtime argument in casper-client JSON args form.
///
/// `type` is a [`serde_json::Value`]: a string for simple CLTypes (`"String"`,
/// `"U256"`, …) or an object for nested ones (`{"List":"Key"}`,
/// `{"List":{"Tuple2":["String","String"]}}`, …).
#[derive(Debug, Clone, Serialize)]
pub struct JsonArg {
    name: String,
    #[serde(rename = "type")]
    arg_type: Value,
    value: Value,
}

impl JsonArg {
    /// Create a typed JSON arg (`arg_type` is a CLType JSON fragment).
    pub fn new(name: impl Into<String>, arg_type: Value, value: Value) -> Self {
        Self {
            name: name.into(),
            arg_type,
            value,
        }
    }
}

/// Build a typed JSON arg (same fields as [`JsonArg::new`]).
pub fn json_arg(name: &str, arg_type: Value, value: Value) -> JsonArg {
    JsonArg::new(name, arg_type, value)
}

/// Simple (non-nested) CLType name as a JSON string.
fn simple_type(name: &str) -> Value {
    Value::String(name.to_string())
}

/// Serialize args to the JSON array string expected by `set_session_args_json`.
pub fn json_args(args: &[JsonArg]) -> String {
    serde_json::to_string(args).expect("JsonArg serialization is infallible")
}

/// `String` CL value.
pub fn string_arg(name: &str, value: &str) -> JsonArg {
    json_arg(
        name,
        simple_type("String"),
        Value::String(value.to_string()),
    )
}

/// `U8` CL value.
pub fn u8_arg(name: &str, value: u8) -> JsonArg {
    json_arg(name, simple_type("U8"), json!(value))
}

/// `U64` CL value.
pub fn u64_arg(name: &str, value: u64) -> JsonArg {
    json_arg(name, simple_type("U64"), json!(value))
}

/// `U256` from a decimal string (not hex).
pub fn u256_arg(name: &str, value: &str) -> JsonArg {
    json_arg(name, simple_type("U256"), Value::String(value.to_string()))
}

/// `Bool` CL value.
pub fn bool_arg(name: &str, value: bool) -> JsonArg {
    json_arg(name, simple_type("Bool"), json!(value))
}

/// `Key` from a prefixed key string (`account-hash-…`, `hash-…`, `entity-…`).
pub fn key_arg(name: &str, prefixed_key: &str) -> JsonArg {
    json_arg(
        name,
        simple_type("Key"),
        Value::String(prefixed_key.to_string()),
    )
}

/// `List (Key)` values (`{"List":"Key"}`).
pub fn key_list_arg(name: &str, keys: &[String]) -> JsonArg {
    json_arg(name, json!({"List": "Key"}), json!(keys))
}

/// `List (U256)` from decimal strings.
pub fn u256_list_arg(name: &str, values: &[&str]) -> JsonArg {
    json_arg(name, json!({"List": "U256"}), json!(values))
}

/// `Option (List (U8))` - null or byte array.
pub fn option_byte_list_arg(name: &str, bytes: Option<&[u8]>) -> JsonArg {
    let value = match bytes {
        Some(b) if !b.is_empty() => json!(b),
        _ => Value::Null,
    };
    json_arg(name, json!({"Option": {"List": "U8"}}), value)
}

/// `List (Tuple2 (String, String))` metadata pairs.
pub fn string_pair_list_arg(name: &str, pairs: &[(String, String)]) -> JsonArg {
    let values: Vec<Value> = pairs.iter().map(|(k, v)| json!([k, v])).collect();
    json_arg(
        name,
        json!({"List": {"Tuple2": ["String", "String"]}}),
        json!(values),
    )
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
        assert!(s.contains(r#""List":"Key""#) || s.contains(r#""List": "Key""#));
        assert!(s.contains("Bool"));
        assert!(s.contains("1000"));
    }

    #[test]
    fn nested_types_are_json_objects() {
        let s = json_args(&[
            string_pair_list_arg("metadata", &[]),
            option_byte_list_arg("data", None),
            u256_list_arg("ids", &["1", "2"]),
        ]);
        let v: Value = serde_json::from_str(&s).unwrap();
        assert_eq!(
            v[0]["type"],
            json!({"List": {"Tuple2": ["String", "String"]}})
        );
        assert_eq!(v[1]["type"], json!({"Option": {"List": "U8"}}));
        assert_eq!(v[2]["type"], json!({"List": "U256"}));
        // Round-trip through casper_types::CLType.
        let _: casper_types::CLType = serde_json::from_value(v[0]["type"].clone()).unwrap();
        let _: casper_types::CLType = serde_json::from_value(v[1]["type"].clone()).unwrap();
        let _: casper_types::CLType = serde_json::from_value(v[2]["type"].clone()).unwrap();
    }
}
