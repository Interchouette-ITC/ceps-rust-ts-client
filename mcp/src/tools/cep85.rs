//! CEP-85 MCP tool bodies.

use crate::format;
use crate::tools::params;
use ceps_client::cep85::ChangeSecurityArgs;
use mcpkit::prelude::ToolOutput;

pub const TOOL_NAMES: &[&str] = &[
    "ceps85_install",
    "ceps85_upgrade",
    "ceps85_mint",
    "ceps85_batch_mint",
    "ceps85_burn",
    "ceps85_batch_burn",
    "ceps85_transfer",
    "ceps85_batch_transfer",
    "ceps85_set_approval_for_all",
    "ceps85_set_uri",
    "ceps85_set_total_supply_of",
    "ceps85_set_total_supply_of_batch",
    "ceps85_change_security",
    "ceps85_set_modalities",
    "ceps85_collection_name",
    "ceps85_collection_uri",
    "ceps85_balance_of",
    "ceps85_balance_of_batch",
    "ceps85_is_approved_for_all",
    "ceps85_supply_of",
    "ceps85_supply_of_batch",
    "ceps85_total_supply_of",
    "ceps85_total_supply_of_batch",
    "ceps85_total_fungible_supply",
    "ceps85_uri",
    "ceps85_is_non_fungible",
    "ceps85_enable_burn",
    "ceps85_events_mode",
    "ceps85_number_of_minted_tokens",
    "ceps85_transfer_filter_contract",
    "ceps85_transfer_filter_method",
    "ceps85_security_badge",
];

macro_rules! need_client {
    ($hash:expr, $pkg:expr) => {
        match params::cep85_client(Some($hash.as_str()), $pkg.as_deref()) {
            Ok(c) => c,
            Err(e) => return format::err(e),
        }
    };
}

fn refs(v: &[String]) -> Vec<&str> {
    v.iter().map(String::as_str).collect()
}

pub async fn install(
    name: String,
    uri: String,
    secret_key_pem: Option<String>,
    payment_amount: String,
    wasm_path: Option<String>,
    wasm_base64: Option<String>,
    events_mode: Option<u8>,
    enable_burn: Option<bool>,
    admin_list: Option<Vec<String>>,
    minter_list: Option<Vec<String>>,
    burner_list: Option<Vec<String>>,
    meta_list: Option<Vec<String>>,
    transfer_filter_contract: Option<String>,
    transfer_filter_method: Option<String>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let args = match params::cep85_install_args(
        name,
        uri,
        events_mode,
        enable_burn,
        admin_list,
        minter_list,
        burner_list,
        meta_list,
        transfer_filter_contract,
        transfer_filter_method,
    ) {
        Ok(a) => a,
        Err(e) => return format::err(e),
    };
    let wasm = match params::load_wasm(wasm_base64, wasm_path) {
        Ok(w) => w,
        Err(e) => return format::err(e),
    };
    let tx = match params::transaction_params(
        secret_key_pem,
        payment_amount,
        wait,
        wait_timeout_ms,
        None,
        make_only,
        initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let client = match params::cep85_client(None, None) {
        Ok(c) => c,
        Err(e) => return format::err(e),
    };
    params::map_call(client.install(&args, &wasm, &tx).await)
}

pub async fn upgrade(
    name: String,
    secret_key_pem: Option<String>,
    payment_amount: String,
    wasm_path: Option<String>,
    wasm_base64: Option<String>,
    transfer_filter_contract: Option<String>,
    transfer_filter_method: Option<String>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let args =
        match params::cep85_upgrade_args(name, transfer_filter_contract, transfer_filter_method) {
            Ok(a) => a,
            Err(e) => return format::err(e),
        };
    let wasm = match params::load_wasm(wasm_base64, wasm_path) {
        Ok(w) => w,
        Err(e) => return format::err(e),
    };
    let tx = match params::transaction_params(
        secret_key_pem,
        payment_amount,
        wait,
        wait_timeout_ms,
        None,
        make_only,
        initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let client = match params::cep85_client(None, None) {
        Ok(c) => c,
        Err(e) => return format::err(e),
    };
    params::map_call(client.upgrade(&args, &wasm, &tx).await)
}

pub async fn mint(
    contract_hash: String,
    package_hash: Option<String>,
    recipient: String,
    id: String,
    amount: String,
    uri: Option<String>,
    secret_key_pem: Option<String>,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let tx = match params::transaction_params(
        secret_key_pem,
        payment_amount,
        wait,
        wait_timeout_ms,
        None,
        make_only,
        initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    params::map_call(
        client
            .mint(&recipient, &id, &amount, uri.as_deref(), &tx)
            .await,
    )
}

pub async fn batch_mint(
    contract_hash: String,
    package_hash: Option<String>,
    recipient: String,
    ids: Vec<String>,
    amounts: Vec<String>,
    uri: Option<String>,
    secret_key_pem: Option<String>,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let tx = match params::transaction_params(
        secret_key_pem,
        payment_amount,
        wait,
        wait_timeout_ms,
        None,
        make_only,
        initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let id_refs = refs(&ids);
    let amt_refs = refs(&amounts);
    params::map_call(
        client
            .batch_mint(&recipient, &id_refs, &amt_refs, uri.as_deref(), &tx)
            .await,
    )
}

pub async fn burn(
    contract_hash: String,
    package_hash: Option<String>,
    owner: String,
    id: String,
    amount: String,
    secret_key_pem: Option<String>,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let tx = match params::transaction_params(
        secret_key_pem,
        payment_amount,
        wait,
        wait_timeout_ms,
        None,
        make_only,
        initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    params::map_call(client.burn(&owner, &id, &amount, &tx).await)
}

pub async fn batch_burn(
    contract_hash: String,
    package_hash: Option<String>,
    owner: String,
    ids: Vec<String>,
    amounts: Vec<String>,
    secret_key_pem: Option<String>,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let tx = match params::transaction_params(
        secret_key_pem,
        payment_amount,
        wait,
        wait_timeout_ms,
        None,
        make_only,
        initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let id_refs = refs(&ids);
    let amt_refs = refs(&amounts);
    params::map_call(client.batch_burn(&owner, &id_refs, &amt_refs, &tx).await)
}

pub async fn transfer(
    contract_hash: String,
    package_hash: Option<String>,
    from: String,
    to: String,
    id: String,
    amount: String,
    data_hex: Option<String>,
    secret_key_pem: Option<String>,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let tx = match params::transaction_params(
        secret_key_pem,
        payment_amount,
        wait,
        wait_timeout_ms,
        None,
        make_only,
        initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let data = match decode_optional_hex(data_hex) {
        Ok(d) => d,
        Err(e) => return format::err(e),
    };
    params::map_call(
        client
            .transfer(&from, &to, &id, &amount, data.as_deref(), &tx)
            .await,
    )
}

pub async fn batch_transfer(
    contract_hash: String,
    package_hash: Option<String>,
    from: String,
    to: String,
    ids: Vec<String>,
    amounts: Vec<String>,
    data_hex: Option<String>,
    secret_key_pem: Option<String>,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let tx = match params::transaction_params(
        secret_key_pem,
        payment_amount,
        wait,
        wait_timeout_ms,
        None,
        make_only,
        initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let data = match decode_optional_hex(data_hex) {
        Ok(d) => d,
        Err(e) => return format::err(e),
    };
    let id_refs = refs(&ids);
    let amt_refs = refs(&amounts);
    params::map_call(
        client
            .batch_transfer(&from, &to, &id_refs, &amt_refs, data.as_deref(), &tx)
            .await,
    )
}

pub async fn set_approval_for_all(
    contract_hash: String,
    package_hash: Option<String>,
    operator: String,
    approved: bool,
    secret_key_pem: Option<String>,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let tx = match params::transaction_params(
        secret_key_pem,
        payment_amount,
        wait,
        wait_timeout_ms,
        None,
        make_only,
        initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    params::map_call(client.set_approval_for_all(&operator, approved, &tx).await)
}

pub async fn set_uri(
    contract_hash: String,
    package_hash: Option<String>,
    uri: String,
    id: Option<String>,
    secret_key_pem: Option<String>,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let tx = match params::transaction_params(
        secret_key_pem,
        payment_amount,
        wait,
        wait_timeout_ms,
        None,
        make_only,
        initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    params::map_call(client.set_uri(&uri, id.as_deref(), &tx).await)
}

pub async fn set_total_supply_of(
    contract_hash: String,
    package_hash: Option<String>,
    id: String,
    total_supply: String,
    secret_key_pem: Option<String>,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let tx = match params::transaction_params(
        secret_key_pem,
        payment_amount,
        wait,
        wait_timeout_ms,
        None,
        make_only,
        initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    params::map_call(client.set_total_supply_of(&id, &total_supply, &tx).await)
}

pub async fn set_total_supply_of_batch(
    contract_hash: String,
    package_hash: Option<String>,
    ids: Vec<String>,
    total_supplies: Vec<String>,
    secret_key_pem: Option<String>,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let tx = match params::transaction_params(
        secret_key_pem,
        payment_amount,
        wait,
        wait_timeout_ms,
        None,
        make_only,
        initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let id_refs = refs(&ids);
    let supply_refs = refs(&total_supplies);
    params::map_call(
        client
            .set_total_supply_of_batch(&id_refs, &supply_refs, &tx)
            .await,
    )
}

pub async fn change_security(
    contract_hash: String,
    package_hash: Option<String>,
    secret_key_pem: Option<String>,
    payment_amount: String,
    admin_list: Option<Vec<String>>,
    minter_list: Option<Vec<String>>,
    burner_list: Option<Vec<String>>,
    meta_list: Option<Vec<String>>,
    none_list: Option<Vec<String>>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let tx = match params::transaction_params(
        secret_key_pem,
        payment_amount,
        wait,
        wait_timeout_ms,
        None,
        make_only,
        initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let args = ChangeSecurityArgs {
        admin_list,
        minter_list,
        burner_list,
        meta_list,
        none_list,
    };
    params::map_call(client.change_security(&args, &tx).await)
}

pub async fn set_modalities(
    contract_hash: String,
    package_hash: Option<String>,
    secret_key_pem: Option<String>,
    payment_amount: String,
    enable_burn: Option<bool>,
    events_mode: Option<u8>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let mode = match params::parse_events_mode(events_mode) {
        Ok(m) => m,
        Err(e) => return format::err(e),
    };
    let client = need_client!(contract_hash, package_hash);
    let tx = match params::transaction_params(
        secret_key_pem,
        payment_amount,
        wait,
        wait_timeout_ms,
        None,
        make_only,
        initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    params::map_call(client.set_modalities(enable_burn, mode, &tx).await)
}

pub async fn collection_name(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.collection_name().await)
}

pub async fn collection_uri(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.collection_uri().await)
}

pub async fn balance_of(
    contract_hash: String,
    package_hash: Option<String>,
    account: String,
    id: String,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.balance_of(&account, &id).await)
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

pub async fn supply_of(
    contract_hash: String,
    package_hash: Option<String>,
    id: String,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.supply_of(&id).await)
}

pub async fn total_supply_of(
    contract_hash: String,
    package_hash: Option<String>,
    id: String,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.total_supply_of(&id).await)
}

pub async fn uri(
    contract_hash: String,
    package_hash: Option<String>,
    id: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.uri(id.as_deref()).await)
}

pub async fn is_non_fungible(
    contract_hash: String,
    package_hash: Option<String>,
    id: String,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.is_non_fungible(&id).await)
}

pub async fn balance_of_batch(
    contract_hash: String,
    package_hash: Option<String>,
    accounts: Vec<String>,
    ids: Vec<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let acct_refs = refs(&accounts);
    let id_refs = refs(&ids);
    params::map_query(client.balance_of_batch(&acct_refs, &id_refs).await)
}

pub async fn supply_of_batch(
    contract_hash: String,
    package_hash: Option<String>,
    ids: Vec<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let id_refs = refs(&ids);
    params::map_query(client.supply_of_batch(&id_refs).await)
}

pub async fn total_supply_of_batch(
    contract_hash: String,
    package_hash: Option<String>,
    ids: Vec<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    let id_refs = refs(&ids);
    params::map_query(client.total_supply_of_batch(&id_refs).await)
}

pub async fn total_fungible_supply(
    contract_hash: String,
    package_hash: Option<String>,
    id: String,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.total_fungible_supply(&id).await)
}

pub async fn enable_burn(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.enable_burn().await)
}

pub async fn events_mode(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    match client.events_mode().await {
        Ok(m) => {
            format::json_ok(&serde_json::json!({ "events_mode": m.as_str(), "value": u8::from(m) }))
        }
        Err(e) => format::err(e),
    }
}

pub async fn number_of_minted_tokens(
    contract_hash: String,
    package_hash: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.number_of_minted_tokens().await)
}

pub async fn transfer_filter_contract(
    contract_hash: String,
    package_hash: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.transfer_filter_contract().await)
}

pub async fn transfer_filter_method(
    contract_hash: String,
    package_hash: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.transfer_filter_method().await)
}

pub async fn security_badge(
    contract_hash: String,
    package_hash: Option<String>,
    entity: String,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    match client.security_badge(&entity).await {
        Ok(Some(b)) => format::json_ok(&serde_json::json!({
            "badge": b.as_str(),
            "value": b as u8
        })),
        Ok(None) => format::json_ok(&serde_json::json!({ "badge": null })),
        Err(e) => format::err(e),
    }
}

fn decode_optional_hex(data_hex: Option<String>) -> Result<Option<Vec<u8>>, String> {
    match data_hex {
        None => Ok(None),
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => {
            let bytes =
                hex::decode(s.trim_start_matches("0x")).map_err(|e| format!("data_hex: {e}"))?;
            Ok(Some(bytes))
        }
    }
}
