//! CEP-18 MCP tool bodies.

use crate::format;
use crate::tools::params;
use ceps_client::cep18::ChangeSecurityArgs;
use ceps_client::EventsMode;
use rmcp::model::CallToolResult;

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
    "ceps18_security_badge",
];

macro_rules! need_client {
    ($hash:expr, $pkg:expr) => {
        match params::cep18_client(Some($hash.as_str()), $pkg.as_deref()) {
            Ok(c) => c,
            Err(e) => return format::err(e),
        }
    };
}

pub async fn install(a: crate::tool_args::Ceps18InstallArgs) -> CallToolResult {
    let args = match params::cep18_install_args(&a) {
        Ok(a) => a,
        Err(e) => return format::err(e),
    };
    let wasm = match params::load_wasm(a.wasm_base64, a.wasm_path) {
        Ok(w) => w,
        Err(e) => return format::err(e),
    };
    let tx = match params::transaction_params(
        a.secret_key_pem,
        a.payment_amount,
        a.wait,
        a.wait_timeout_ms,
        None,
        a.make_only,
        a.initiator_addr,
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

pub async fn upgrade(a: crate::tool_args::Ceps18UpgradeArgs) -> CallToolResult {
    let args = match params::cep18_upgrade_args(a.name, a.events_mode) {
        Ok(a) => a,
        Err(e) => return format::err(e),
    };
    let wasm = match params::load_wasm(a.wasm_base64, a.wasm_path) {
        Ok(w) => w,
        Err(e) => return format::err(e),
    };
    let tx = match params::transaction_params(
        a.secret_key_pem,
        a.payment_amount,
        a.wait,
        a.wait_timeout_ms,
        None,
        a.make_only,
        a.initiator_addr,
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

pub async fn transfer(a: crate::tool_args::Ceps18TransferArgs) -> CallToolResult {
    let client = need_client!(a.contract_hash, a.package_hash);
    let tx = match params::transaction_params(
        a.secret_key_pem,
        a.payment_amount,
        a.wait,
        a.wait_timeout_ms,
        None,
        a.make_only,
        a.initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    params::map_call(client.transfer(&a.recipient, &a.amount, &tx).await)
}

pub async fn transfer_from(a: crate::tool_args::Ceps18TransferFromArgs) -> CallToolResult {
    let client = need_client!(a.contract_hash, a.package_hash);
    let tx = match params::transaction_params(
        a.secret_key_pem,
        a.payment_amount,
        a.wait,
        a.wait_timeout_ms,
        None,
        a.make_only,
        a.initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    params::map_call(
        client
            .transfer_from(&a.owner, &a.recipient, &a.amount, &tx)
            .await,
    )
}

pub async fn approve(a: crate::tool_args::Ceps18ApproveArgs) -> CallToolResult {
    let client = need_client!(a.contract_hash, a.package_hash);
    let tx = match params::transaction_params(
        a.secret_key_pem,
        a.payment_amount,
        a.wait,
        a.wait_timeout_ms,
        None,
        a.make_only,
        a.initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    params::map_call(client.approve(&a.spender, &a.amount, &tx).await)
}

pub async fn increase_allowance(
    a: crate::tool_args::Ceps18IncreaseAllowanceArgs,
) -> CallToolResult {
    let client = need_client!(a.contract_hash, a.package_hash);
    let tx = match params::transaction_params(
        a.secret_key_pem,
        a.payment_amount,
        a.wait,
        a.wait_timeout_ms,
        None,
        a.make_only,
        a.initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    params::map_call(client.increase_allowance(&a.spender, &a.amount, &tx).await)
}

pub async fn decrease_allowance(
    a: crate::tool_args::Ceps18DecreaseAllowanceArgs,
) -> CallToolResult {
    let client = need_client!(a.contract_hash, a.package_hash);
    let tx = match params::transaction_params(
        a.secret_key_pem,
        a.payment_amount,
        a.wait,
        a.wait_timeout_ms,
        None,
        a.make_only,
        a.initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    params::map_call(client.decrease_allowance(&a.spender, &a.amount, &tx).await)
}

pub async fn mint(a: crate::tool_args::Ceps18MintArgs) -> CallToolResult {
    let client = need_client!(a.contract_hash, a.package_hash);
    let tx = match params::transaction_params(
        a.secret_key_pem,
        a.payment_amount,
        a.wait,
        a.wait_timeout_ms,
        None,
        a.make_only,
        a.initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    params::map_call(client.mint(&a.owner, &a.amount, &tx).await)
}

pub async fn burn(a: crate::tool_args::Ceps18BurnArgs) -> CallToolResult {
    let client = need_client!(a.contract_hash, a.package_hash);
    let tx = match params::transaction_params(
        a.secret_key_pem,
        a.payment_amount,
        a.wait,
        a.wait_timeout_ms,
        None,
        a.make_only,
        a.initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    params::map_call(client.burn(&a.owner, &a.amount, &tx).await)
}

pub async fn change_security(a: crate::tool_args::Ceps18ChangeSecurityArgs) -> CallToolResult {
    let client = need_client!(a.contract_hash, a.package_hash);
    let tx = match params::transaction_params(
        a.secret_key_pem,
        a.payment_amount,
        a.wait,
        a.wait_timeout_ms,
        None,
        a.make_only,
        a.initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    let args = ChangeSecurityArgs {
        admin_list: a.admin_list,
        minter_list: a.minter_list,
        none_list: a.none_list,
    };
    params::map_call(client.change_security(&args, &tx).await)
}

pub async fn change_events_mode(a: crate::tool_args::Ceps18ChangeEventsModeArgs) -> CallToolResult {
    let mode = match EventsMode::from_u8(a.events_mode) {
        Some(m) => m,
        None => return format::err(format!("invalid events_mode: {}", a.events_mode)),
    };
    let client = need_client!(a.contract_hash, a.package_hash);
    let tx = match params::transaction_params(
        a.secret_key_pem,
        a.payment_amount,
        a.wait,
        a.wait_timeout_ms,
        None,
        a.make_only,
        a.initiator_addr,
    ) {
        Ok(t) => t,
        Err(e) => return format::err(e),
    };
    params::map_call(client.change_events_mode(mode, &tx).await)
}

pub async fn name(contract_hash: String, package_hash: Option<String>) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.name().await)
}

pub async fn symbol(contract_hash: String, package_hash: Option<String>) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.symbol().await)
}

pub async fn decimals(contract_hash: String, package_hash: Option<String>) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.decimals().await)
}

pub async fn total_supply(contract_hash: String, package_hash: Option<String>) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.total_supply().await)
}

pub async fn events_mode(contract_hash: String, package_hash: Option<String>) -> CallToolResult {
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
) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.is_mint_and_burn_enabled().await)
}

pub async fn balance_of(
    contract_hash: String,
    package_hash: Option<String>,
    account: String,
) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.balance_of(&account).await)
}

pub async fn allowances(
    contract_hash: String,
    package_hash: Option<String>,
    owner: String,
    spender: String,
) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.allowances(&owner, &spender).await)
}

pub async fn security_badge(
    contract_hash: String,
    package_hash: Option<String>,
    account: String,
) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    match client.security_badge(&account).await {
        Ok(Some(b)) => format::json_ok(&serde_json::json!({
            "badge": b.as_str(),
            "value": b as u8
        })),
        Ok(None) => format::json_ok(&serde_json::json!({ "badge": null })),
        Err(e) => format::err(e),
    }
}
