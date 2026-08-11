//! Feature-gated schema bindgen (no CEP clients / RPC).

use ceps_client::schema::{
    cep18_schema_json, cep78_schema_json, cep85_schema_json, cep95_schema_json, schema_json,
    supported_ceps, CepId,
};
use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;

/// JSON schema for a CEP id (`cep18`, `cep78`, `cep85`, `cep95`).
#[wasm_bindgen(js_name = schemaJson)]
pub fn schema_json_js(cep: &str) -> Result<String, JsValue> {
    let id = CepId::parse(cep).ok_or_else(|| JsValue::from_str("unknown CEP id"))?;
    Ok(schema_json(id))
}

/// Supported CEP id strings.
#[wasm_bindgen(js_name = supportedCeps)]
pub fn supported_ceps_js() -> Result<JsValue, JsValue> {
    let ids: Vec<&str> = supported_ceps().iter().map(|c| c.as_str()).collect();
    JsValue::from_serde(&ids).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// CEP-18 schema JSON.
#[wasm_bindgen(js_name = cep18SchemaJson)]
pub fn cep18_schema_json_js() -> String {
    cep18_schema_json()
}

/// CEP-78 schema JSON.
#[wasm_bindgen(js_name = cep78SchemaJson)]
pub fn cep78_schema_json_js() -> String {
    cep78_schema_json()
}

/// CEP-85 schema JSON.
#[wasm_bindgen(js_name = cep85SchemaJson)]
pub fn cep85_schema_json_js() -> String {
    cep85_schema_json()
}

/// CEP-95 schema JSON.
#[wasm_bindgen(js_name = cep95SchemaJson)]
pub fn cep95_schema_json_js() -> String {
    cep95_schema_json()
}
