//! CEP-18 MCP tool bodies.

use crate::format;
use crate::tools::params;
use ceps_client::cep18::ChangeSecurityArgs;
use ceps_client::EventsMode;
use mcpkit::prelude::ToolOutput;

pub const TOOL_NAMES: &[&str] = &[
    "ceps18_install",
    "ceps18_upgrade",
    "ceps18_transfer",
    "ceps18_transfer_from",
    "ceps18_approve",
    "ceps18_increase_allowance",
    "ceps18_decrease_allowance",
    "ceps18_mint",
    "ceps18_burn",
    "ceps18_change_security",
    "ceps18_change_events_mode",
    "ceps18_name",
    "ceps18_symbol",
    "ceps18_decimals",
    "ceps18_total_supply",
    "ceps18_events_mode",
    "ceps18_is_mint_and_burn_enabled",
    "ceps18_balance_of",
    "ceps18_allowances",
];

macro_rules! need_client {
    ($hash:expr, $pkg:expr) => {
        match params::cep18_client(Some($hash.as_str()), $pkg.as_deref()) {
            Ok(c) => c,
            Err(e) => return format::err(e),
        }
    };
}

pub async fn install(
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: String,
    secret_key_pem: Option<String>,
    payment_amount: String,
    wasm_path: Option<String>,
    wasm_base64: Option<String>,
    events_mode: Option<u8>,
    enable_mint_and_burn: Option<bool>,
    admin_list: Option<Vec<String>>,
    minter_list: Option<Vec<String>>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let args = match params::cep18_install_args(
        name,
        symbol,
        decimals,
        total_supply,
        events_mode,
        enable_mint_and_burn,
        admin_list,
        minter_list,
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
    let client = match params::cep18_client(None, None) {
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
    events_mode: Option<u8>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let args = match params::cep18_upgrade_args(name, events_mode) {
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
    let client = match params::cep18_client(None, None) {
        Ok(c) => c,
        Err(e) => return format::err(e),
    };
    params::map_call(client.upgrade(&args, &wasm, &tx).await)
}

pub async fn transfer(
    contract_hash: String,
    package_hash: Option<String>,
    recipient: String,
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
    params::map_call(client.transfer(&recipient, &amount, &tx).await)
}

pub async fn transfer_from(
    contract_hash: String,
    package_hash: Option<String>,
    owner: String,
    recipient: String,
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
    params::map_call(client.transfer_from(&owner, &recipient, &amount, &tx).await)
}

pub async fn approve(
    contract_hash: String,
    package_hash: Option<String>,
    spender: String,
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
    params::map_call(client.approve(&spender, &amount, &tx).await)
}

pub async fn increase_allowance(
    contract_hash: String,
    package_hash: Option<String>,
    spender: String,
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
    params::map_call(client.increase_allowance(&spender, &amount, &tx).await)
}

pub async fn decrease_allowance(
    contract_hash: String,
    package_hash: Option<String>,
    spender: String,
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
    params::map_call(client.decrease_allowance(&spender, &amount, &tx).await)
}

pub async fn mint(
    contract_hash: String,
    package_hash: Option<String>,
    owner: String,
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
    params::map_call(client.mint(&owner, &amount, &tx).await)
}

pub async fn burn(
    contract_hash: String,
    package_hash: Option<String>,
    owner: String,
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
    params::map_call(client.burn(&owner, &amount, &tx).await)
}

pub async fn change_security(
    contract_hash: String,
    package_hash: Option<String>,
    secret_key_pem: Option<String>,
    payment_amount: String,
    admin_list: Option<Vec<String>>,
    minter_list: Option<Vec<String>>,
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
        none_list,
    };
    params::map_call(client.change_security(&args, &tx).await)
}

pub async fn change_events_mode(
    contract_hash: String,
    package_hash: Option<String>,
    events_mode: u8,
    secret_key_pem: Option<String>,
    payment_amount: String,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let mode = match EventsMode::from_u8(events_mode) {
        Some(m) => m,
        None => return format::err(format!("invalid events_mode: {events_mode}")),
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
    params::map_call(client.change_events_mode(mode, &tx).await)
}

pub async fn name(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.name().await)
}

pub async fn symbol(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.symbol().await)
}

pub async fn decimals(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.decimals().await)
}

pub async fn total_supply(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.total_supply().await)
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

pub async fn is_mint_and_burn_enabled(
    contract_hash: String,
    package_hash: Option<String>,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.is_mint_and_burn_enabled().await)
}

pub async fn balance_of(
    contract_hash: String,
    package_hash: Option<String>,
    account: String,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.balance_of(&account).await)
}

pub async fn allowances(
    contract_hash: String,
    package_hash: Option<String>,
    owner: String,
    spender: String,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.allowances(&owner, &spender).await)
}
