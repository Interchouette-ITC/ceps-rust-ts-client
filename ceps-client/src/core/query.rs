//! Named-key and dictionary queries against the bound contract.

use super::CepCore;
use crate::error::{CepError, Result};
use casper_rust_wasm_sdk::helpers::contract_hash_key_for_global_state;
use casper_rust_wasm_sdk::rpcs::get_dictionary_item::DictionaryItemInput;
use casper_rust_wasm_sdk::rpcs::query_global_state::PathIdentifierInput;
use casper_rust_wasm_sdk::types::deploy_params::dictionary_item_str_params::DictionaryItemStrParams;
use casper_rust_wasm_sdk::types::identifier::entity_identifier::EntityIdentifier;
use serde_json::Value;

pub(super) async fn query_contract_key(core: &CepCore, path: &[&str]) -> Result<Value> {
    let target = core.require_target()?;
    let key = contract_hash_key_for_global_state(&target.query_key());
    let entity = EntityIdentifier::from_formatted_str(&key)
        .map_err(|e| CepError::InvalidHash(format!("entity identifier: {e}")))?;
    let path_input = if path.len() == 1 {
        PathIdentifierInput::String(path[0].to_string())
    } else {
        PathIdentifierInput::String(path.join("/"))
    };
    let response = core
        .sdk()
        .query_contract_key(
            Some(entity),
            None,
            path_input,
            None,
            Some(core.verbosity()),
            Some(core.rpc_url().to_string()),
        )
        .await?;
    serde_json::to_value(&response.result)
        .map_err(|e| CepError::Decode(format!("query_contract_key: {e}")))
}

pub(super) async fn query_dictionary(
    core: &CepCore,
    dictionary_name: &str,
    item_key: &str,
) -> Result<Value> {
    let target = core.require_target()?;
    let key = contract_hash_key_for_global_state(&target.query_key());
    let mut params = DictionaryItemStrParams::new();
    params.set_contract_named_key(&key, dictionary_name, item_key);
    let input = DictionaryItemInput::Params(Box::new(params));
    let response = core
        .sdk()
        .query_contract_dict(
            input,
            None::<&str>,
            Some(core.verbosity()),
            Some(core.rpc_url().to_string()),
        )
        .await?;
    serde_json::to_value(&response.result)
        .map_err(|e| CepError::Decode(format!("query_dictionary: {e}")))
}
