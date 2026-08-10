//! Named-key and dictionary queries against the bound contract.

use super::CEPClient;
use crate::error::{CEPError, Result};
use casper_rust_wasm_sdk::helpers::contract_hash_key_for_global_state;
use casper_rust_wasm_sdk::rpcs::get_dictionary_item::DictionaryItemInput;
use casper_rust_wasm_sdk::rpcs::query_global_state::PathIdentifierInput;
use casper_rust_wasm_sdk::types::identifier::dictionary_item_identifier::DictionaryItemIdentifier;
use serde_json::Value;

pub(super) async fn query_contract_key(core: &CEPClient, path: &[&str]) -> Result<Value> {
    let target = core.require_target()?;
    let key = contract_hash_key_for_global_state(&target.query_key());
    // Pass the hash as a string so SDK falls back to classic `hash-…` global-state
    // queries when addressable entities are disabled (EntityIdentifier rejects `hash-`).
    let path_input = if path.len() == 1 {
        PathIdentifierInput::String(path[0].to_string())
    } else {
        PathIdentifierInput::String(path.join("/"))
    };
    let response = core
        .sdk()
        .query_contract_key(
            None,
            Some(key),
            path_input,
            None,
            Some(core.verbosity()),
            Some(core.rpc_url().to_string()),
        )
        .await?;
    serde_json::to_value(&response.result)
        .map_err(|e| CEPError::Decode(format!("query_contract_key: {e}")))
}

pub(super) async fn query_dictionary(
    core: &CEPClient,
    dictionary_name: &str,
    item_key: &str,
) -> Result<Value> {
    let target = core.require_target()?;
    let key = contract_hash_key_for_global_state(&target.query_key());
    let identifier =
        DictionaryItemIdentifier::new_from_contract_info(&key, dictionary_name, item_key)
            .map_err(|e| CEPError::InvalidHash(format!("dictionary item: {e}")))?;
    let input = DictionaryItemInput::Identifier(identifier);
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
        .map_err(|e| CEPError::Decode(format!("query_dictionary: {e}")))
}

pub(super) async fn get_account_named_key(
    core: &CEPClient,
    account_identifier: &str,
    named_key: &str,
) -> Result<String> {
    #[allow(deprecated)]
    let response = core
        .sdk()
        .get_account(
            None,
            Some(account_identifier.to_string()),
            None,
            Some(core.verbosity()),
            Some(core.rpc_url().to_string()),
        )
        .await?;

    let keys = response.result.account.named_keys();
    let key = keys.get(named_key).ok_or_else(|| {
        let names: Vec<String> = keys.names().cloned().collect();
        CEPError::EmptyQuery(format!("account named key '{named_key}' (have: {names:?})"))
    })?;
    Ok(key.to_formatted_string())
}
