//! CEP-78 MCP tool bodies.

use crate::format;
use crate::tools::params;
use ceps_client::cep78::SetVariablesArgs;
use mcpkit::prelude::ToolOutput;

pub const TOOL_NAMES: &[&str] = &[
    "ceps78_install",
    "ceps78_upgrade",
    "ceps78_mint",
    "ceps78_mint_session",
    "ceps78_burn",
    "ceps78_transfer",
    "ceps78_transfer_session",
    "ceps78_register_owner",
    "ceps78_approve",
    "ceps78_revoke",
    "ceps78_set_approval_for_all",
    "ceps78_set_token_metadata",
    "ceps78_set_variables",
    "ceps78_updated_receipts",
    "ceps78_collection_name",
    "ceps78_collection_symbol",
    "ceps78_total_token_supply",
    "ceps78_number_of_minted_tokens",
    "ceps78_events_mode",
    "ceps78_allow_minting",
    "ceps78_minting_mode",
    "ceps78_whitelist_mode",
    "ceps78_reporting_mode",
    "ceps78_burn_mode",
    "ceps78_operator_burn_mode",
    "ceps78_holder_mode",
    "ceps78_identifier_mode",
    "ceps78_metadata_mutability",
    "ceps78_nft_kind",
    "ceps78_nft_metadata_kind",
    "ceps78_ownership_mode",
    "ceps78_package_operator_mode",
    "ceps78_acl_package_mode",
    "ceps78_json_schema",
    "ceps78_is_acl_whitelisted",
    "ceps78_owner_of_session",
    "ceps78_balance_of_session",
    "ceps78_get_approved_session",
    "ceps78_is_approved_for_all_session",
    "ceps78_owner_of",
    "ceps78_balance_of",
    "ceps78_get_approved",
    "ceps78_is_approved_for_all",
    "ceps78_metadata",
];

macro_rules! need_client {
    ($hash:expr, $pkg:expr) => {
        match params::cep78_client(Some($hash.as_str()), $pkg.as_deref()) {
            Ok(c) => c,
            Err(e) => return format::err(e),
        }
    };
}

pub async fn install(
    collection_name: String,
    collection_symbol: String,
    total_token_supply: u64,
    secret_key_pem: String,
    payment_amount: String,
    wasm_path: Option<String>,
    wasm_base64: Option<String>,
    events_mode: Option<u8>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let args = match params::cep78_install_args_basic(
        collection_name,
        collection_symbol,
        total_token_supply,
        events_mode,
    ) {
        Ok(a) => a,
        Err(e) => return format::err(e),
    };
    let wasm = match params::load_wasm(wasm_base64, wasm_path) {
        Ok(w) => w,
        Err(e) => return format::err(e),
    };
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    let client = match params::cep78_client(None, None) {
        Ok(c) => c,
        Err(e) => return format::err(e),
    };
    params::map_call(client.install(&args, &wasm, &deploy).await)
}

pub async fn upgrade(
    collection_name: String,
    secret_key_pem: String,
    payment_amount: String,
    wasm_path: Option<String>,
    wasm_base64: Option<String>,
    total_token_supply: Option<u64>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let args = params::cep78_upgrade_args(collection_name, total_token_supply);
    let wasm = match params::load_wasm(wasm_base64, wasm_path) {
        Ok(w) => w,
        Err(e) => return format::err(e),
    };
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    let client = match params::cep78_client(None, None) {
        Ok(c) => c,
        Err(e) => return format::err(e),
    };
    params::map_call(client.upgrade(&args, &wasm, &deploy).await)
}

pub async fn mint(
    contract_hash: String,
    package_hash: Option<String>,
    token_owner: String,
    token_meta_data: String,
    token_hash: Option<String>,
    secret_key_pem: String,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    params::map_call(
        client
            .mint(
                &token_owner,
                &token_meta_data,
                token_hash.as_deref(),
                &deploy,
            )
            .await,
    )
}

pub async fn mint_session(
    contract_hash: String,
    package_hash: Option<String>,
    token_owner: String,
    token_meta_data: String,
    token_hash: Option<String>,
    secret_key_pem: String,
    payment_amount: String,
    session_wasm_path: Option<String>,
    session_wasm_base64: Option<String>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let session = match params::load_wasm(session_wasm_base64, session_wasm_path) {
        Ok(w) => w,
        Err(e) => return format::err(e),
    };
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    params::map_call(
        client
            .mint_session(
                &token_owner,
                &token_meta_data,
                token_hash.as_deref(),
                &session,
                &deploy,
            )
            .await,
    )
}

pub async fn burn(
    contract_hash: String,
    package_hash: Option<String>,
    token_id: Option<u64>,
    token_hash: Option<String>,
    secret_key_pem: String,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let token = match params::parse_token_id(token_id, token_hash) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let client = need_client!(contract_hash, package_hash);
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    params::map_call(client.burn(&token, &deploy).await)
}

pub async fn transfer(
    contract_hash: String,
    package_hash: Option<String>,
    source: String,
    target: String,
    token_id: Option<u64>,
    token_hash: Option<String>,
    secret_key_pem: String,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let token = match params::parse_token_id(token_id, token_hash) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let client = need_client!(contract_hash, package_hash);
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    params::map_call(client.transfer(&source, &target, &token, &deploy).await)
}

pub async fn transfer_session(
    contract_hash: String,
    package_hash: Option<String>,
    source: String,
    target: String,
    token_id: Option<u64>,
    token_hash: Option<String>,
    secret_key_pem: String,
    payment_amount: String,
    session_wasm_path: Option<String>,
    session_wasm_base64: Option<String>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let token = match params::parse_token_id(token_id, token_hash) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let session = match params::load_wasm(session_wasm_base64, session_wasm_path) {
        Ok(w) => w,
        Err(e) => return format::err(e),
    };
    let client = need_client!(contract_hash, package_hash);
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    params::map_call(
        client
            .transfer_session(&source, &target, &token, &session, &deploy)
            .await,
    )
}

pub async fn register_owner(
    contract_hash: String,
    package_hash: Option<String>,
    token_owner: String,
    secret_key_pem: String,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    params::map_call(client.register_owner(&token_owner, &deploy).await)
}

pub async fn approve(
    contract_hash: String,
    package_hash: Option<String>,
    operator: String,
    token_id: Option<u64>,
    token_hash: Option<String>,
    secret_key_pem: String,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let token = match params::parse_token_id(token_id, token_hash) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let client = need_client!(contract_hash, package_hash);
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    params::map_call(client.approve(&operator, &token, &deploy).await)
}

pub async fn revoke(
    contract_hash: String,
    package_hash: Option<String>,
    operator: String,
    token_id: Option<u64>,
    token_hash: Option<String>,
    secret_key_pem: String,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let token = match params::parse_token_id(token_id, token_hash) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let client = need_client!(contract_hash, package_hash);
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    params::map_call(client.revoke(&operator, &token, &deploy).await)
}

pub async fn set_approval_for_all(
    contract_hash: String,
    package_hash: Option<String>,
    operator: String,
    approve_all: bool,
    secret_key_pem: String,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    params::map_call(
        client
            .set_approval_for_all(&operator, approve_all, &deploy)
            .await,
    )
}

pub async fn set_token_metadata(
    contract_hash: String,
    package_hash: Option<String>,
    token_meta_data: String,
    token_id: Option<u64>,
    token_hash: Option<String>,
    secret_key_pem: String,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let token = match params::parse_token_id(token_id, token_hash) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let client = need_client!(contract_hash, package_hash);
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    params::map_call(
        client
            .set_token_metadata(&token_meta_data, &token, &deploy)
            .await,
    )
}

pub async fn set_variables(
    contract_hash: String,
    package_hash: Option<String>,
    secret_key_pem: String,
    payment_amount: String,
    allow_minting: Option<bool>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    let args = SetVariablesArgs {
        allow_minting,
        ..Default::default()
    };
    params::map_call(client.set_variables(&args, &deploy).await)
}

pub async fn updated_receipts(
    contract_hash: String,
    package_hash: Option<String>,
    secret_key_pem: String,
    payment_amount: String,
    session_wasm_path: Option<String>,
    session_wasm_base64: Option<String>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let session = match params::load_wasm(session_wasm_base64, session_wasm_path) {
        Ok(w) => w,
        Err(e) => return format::err(e),
    };
    let client = need_client!(contract_hash, package_hash);
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    params::map_call(client.updated_receipts(&session, &deploy).await)
}

pub async fn collection_name(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.collection_name().await)
}

pub async fn collection_symbol(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.collection_symbol().await)
}

pub async fn total_token_supply(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.total_token_supply().await)
}

pub async fn number_of_minted_tokens(
    contract_hash: String,
    package_hash: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.number_of_minted_tokens().await)
}

pub async fn events_mode(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    match client.events_mode().await {
        Ok(m) => format::json_ok(&serde_json::json!({
            "events_mode": m.as_str(),
            "value": u8::from(m)
        })),
        Err(e) => format::err(e),
    }
}

pub async fn owner_of(
    contract_hash: String,
    package_hash: Option<String>,
    token_id: Option<u64>,
    token_hash: Option<String>,
) -> ToolOutput {
    let token = match params::parse_token_id(token_id, token_hash) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.owner_of(&token).await)
}

pub async fn balance_of(
    contract_hash: String,
    package_hash: Option<String>,
    owner: String,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.balance_of(&owner).await)
}

pub async fn get_approved(
    contract_hash: String,
    package_hash: Option<String>,
    token_id: Option<u64>,
    token_hash: Option<String>,
) -> ToolOutput {
    let token = match params::parse_token_id(token_id, token_hash) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.get_approved(&token).await)
}

pub async fn is_approved_for_all(
    contract_hash: String,
    package_hash: Option<String>,
    owner: String,
    operator: String,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.is_approved_for_all(&owner, &operator).await)
}

pub async fn metadata(
    contract_hash: String,
    package_hash: Option<String>,
    token_id: Option<u64>,
    token_hash: Option<String>,
    nft_metadata_kind: u8,
) -> ToolOutput {
    let token = match params::parse_token_id(token_id, token_hash) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let kind = match params::parse_nft_metadata_kind(nft_metadata_kind) {
        Ok(k) => k,
        Err(e) => return format::err(e),
    };
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.metadata(&token, kind).await)
}

macro_rules! mode_query {
    ($name:ident, $method:ident) => {
        pub async fn $name(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
            let client = need_client!(contract_hash, package_hash);
            match client.$method().await {
                Ok(m) => format::json_ok(&serde_json::json!({
                    "mode": format!("{:?}", m),
                    "value": u8::from(m)
                })),
                Err(e) => format::err(e),
            }
        }
    };
}

macro_rules! bool_query {
    ($name:ident, $method:ident) => {
        pub async fn $name(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
            let client = need_client!(contract_hash, package_hash);
            params::map_query(client.$method().await)
        }
    };
}

bool_query!(allow_minting, allow_minting);
mode_query!(minting_mode, minting_mode);
mode_query!(whitelist_mode, whitelist_mode);
mode_query!(reporting_mode, reporting_mode);
mode_query!(burn_mode, burn_mode);
bool_query!(operator_burn_mode, operator_burn_mode);
mode_query!(holder_mode, holder_mode);
mode_query!(identifier_mode, identifier_mode);
mode_query!(metadata_mutability, metadata_mutability);
mode_query!(nft_kind, nft_kind);
mode_query!(nft_metadata_kind, nft_metadata_kind);
mode_query!(ownership_mode, ownership_mode);
bool_query!(package_operator_mode, package_operator_mode);
bool_query!(acl_package_mode, acl_package_mode);

pub async fn json_schema(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.json_schema().await)
}

pub async fn is_acl_whitelisted(
    contract_hash: String,
    package_hash: Option<String>,
    entity: String,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.is_acl_whitelisted(&entity).await)
}

pub async fn owner_of_session(
    contract_hash: String,
    package_hash: Option<String>,
    token_id: Option<u64>,
    token_hash: Option<String>,
    key_name: String,
    secret_key_pem: String,
    payment_amount: String,
    session_wasm_path: Option<String>,
    session_wasm_base64: Option<String>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let token = match params::parse_token_id(token_id, token_hash) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let session = match params::load_wasm(session_wasm_base64, session_wasm_path) {
        Ok(w) => w,
        Err(e) => return format::err(e),
    };
    let client = need_client!(contract_hash, package_hash);
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    params::map_call(
        client
            .owner_of_session(&token, &key_name, &session, &deploy)
            .await,
    )
}

pub async fn balance_of_session(
    contract_hash: String,
    package_hash: Option<String>,
    token_owner: String,
    key_name: String,
    secret_key_pem: String,
    payment_amount: String,
    session_wasm_path: Option<String>,
    session_wasm_base64: Option<String>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let session = match params::load_wasm(session_wasm_base64, session_wasm_path) {
        Ok(w) => w,
        Err(e) => return format::err(e),
    };
    let client = need_client!(contract_hash, package_hash);
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    params::map_call(
        client
            .balance_of_session(&token_owner, &key_name, &session, &deploy)
            .await,
    )
}

pub async fn get_approved_session(
    contract_hash: String,
    package_hash: Option<String>,
    token_id: Option<u64>,
    token_hash: Option<String>,
    key_name: String,
    secret_key_pem: String,
    payment_amount: String,
    session_wasm_path: Option<String>,
    session_wasm_base64: Option<String>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let token = match params::parse_token_id(token_id, token_hash) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let session = match params::load_wasm(session_wasm_base64, session_wasm_path) {
        Ok(w) => w,
        Err(e) => return format::err(e),
    };
    let client = need_client!(contract_hash, package_hash);
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    params::map_call(
        client
            .get_approved_session(&token, &key_name, &session, &deploy)
            .await,
    )
}

pub async fn is_approved_for_all_session(
    contract_hash: String,
    package_hash: Option<String>,
    token_owner: String,
    operator: String,
    key_name: String,
    secret_key_pem: String,
    payment_amount: String,
    session_wasm_path: Option<String>,
    session_wasm_base64: Option<String>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
) -> ToolOutput {
    let session = match params::load_wasm(session_wasm_base64, session_wasm_path) {
        Ok(w) => w,
        Err(e) => return format::err(e),
    };
    let client = need_client!(contract_hash, package_hash);
    let deploy = params::deploy_params(secret_key_pem, payment_amount, wait, wait_timeout_ms, None);
    params::map_call(
        client
            .is_approved_for_all_session(&token_owner, &operator, &key_name, &session, &deploy)
            .await,
    )
}
