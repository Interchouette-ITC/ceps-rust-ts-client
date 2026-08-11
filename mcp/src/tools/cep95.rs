//! CEP-95 MCP tool bodies.

use crate::format;
use crate::tools::params;
use mcpkit::prelude::ToolOutput;

pub const TOOL_NAMES: &[&str] = &[
    "ceps95_install",
    "ceps95_bind_odra_install",
    "ceps95_mint",
    "ceps95_burn",
    "ceps95_transfer_from",
    "ceps95_safe_transfer_from",
    "ceps95_approve",
    "ceps95_revoke_approval",
    "ceps95_approve_for_all",
    "ceps95_revoke_approval_for_all",
    "ceps95_name",
    "ceps95_symbol",
    "ceps95_total_supply",
    "ceps95_balance_of",
    "ceps95_owner_of",
    "ceps95_get_approved",
    "ceps95_is_approved_for_all",
    "ceps95_token_metadata",
    "ceps95_get_owner",
    "ceps95_transfer_ownership",
];

macro_rules! need_client {
    ($hash:expr, $pkg:expr) => {
        match params::cep95_client(Some($hash.as_str()), $pkg.as_deref()) {
            Ok(c) => c,
            Err(e) => return format::err(e),
        }
    };
}

pub async fn install(
    name: String,
    symbol: String,
    package_hash_key_name: String,
    secret_key_pem: Option<String>,
    payment_amount: String,
    wasm_path: Option<String>,
    wasm_base64: Option<String>,
    allow_key_override: Option<bool>,
    is_upgradable: Option<bool>,
    is_upgrade: Option<bool>,
    wait: Option<bool>,
    wait_timeout_ms: Option<u64>,
    make_only: Option<bool>,
    initiator_addr: Option<String>,
) -> ToolOutput {
    let args = params::cep95_install_args(
        name,
        symbol,
        package_hash_key_name,
        allow_key_override,
        is_upgradable,
        is_upgrade,
    );
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
    let client = match params::cep95_client(None, None) {
        Ok(c) => c,
        Err(e) => return format::err(e),
    };
    params::map_call(client.install(&args, &wasm, &tx).await)
}

pub async fn bind_odra_install(
    installer_public_key: String,
    package_hash_key_name: String,
) -> ToolOutput {
    let mut client = match params::cep95_client(None, None) {
        Ok(c) => c,
        Err(e) => return format::err(e),
    };
    match client
        .bind_odra_install(&installer_public_key, &package_hash_key_name)
        .await
    {
        Ok((contract, package)) => format::json_ok(&serde_json::json!({
            "contractHash": contract,
            "packageHash": package,
        })),
        Err(e) => format::err(e),
    }
}

pub async fn mint(
    contract_hash: String,
    package_hash: Option<String>,
    to: String,
    token_id: String,
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
    params::map_call(client.mint(&to, &token_id, None, &tx).await)
}

pub async fn burn(
    contract_hash: String,
    package_hash: Option<String>,
    token_id: String,
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
    params::map_call(client.burn(&token_id, &tx).await)
}

pub async fn transfer_from(
    contract_hash: String,
    package_hash: Option<String>,
    from: String,
    to: String,
    token_id: String,
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
    params::map_call(client.transfer_from(&from, &to, &token_id, &tx).await)
}

pub async fn safe_transfer_from(
    contract_hash: String,
    package_hash: Option<String>,
    from: String,
    to: String,
    token_id: String,
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
            .safe_transfer_from(&from, &to, &token_id, None, &tx)
            .await,
    )
}

pub async fn approve(
    contract_hash: String,
    package_hash: Option<String>,
    spender: String,
    token_id: String,
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
    params::map_call(client.approve(&spender, &token_id, &tx).await)
}

pub async fn revoke_approval(
    contract_hash: String,
    package_hash: Option<String>,
    token_id: String,
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
    params::map_call(client.revoke_approval(&token_id, &tx).await)
}

pub async fn approve_for_all(
    contract_hash: String,
    package_hash: Option<String>,
    operator: String,
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
    params::map_call(client.approve_for_all(&operator, &tx).await)
}

pub async fn revoke_approval_for_all(
    contract_hash: String,
    package_hash: Option<String>,
    operator: String,
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
    params::map_call(client.revoke_approval_for_all(&operator, &tx).await)
}

pub async fn name(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.name().await)
}

pub async fn symbol(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.symbol().await)
}

pub async fn total_supply(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.total_supply().await)
}

pub async fn balance_of(
    contract_hash: String,
    package_hash: Option<String>,
    owner: String,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.balance_of(&owner).await)
}

pub async fn owner_of(
    contract_hash: String,
    package_hash: Option<String>,
    token_id: String,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.owner_of(&token_id).await)
}

pub async fn get_approved(
    contract_hash: String,
    package_hash: Option<String>,
    token_id: String,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.get_approved(&token_id).await)
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

pub async fn token_metadata(
    contract_hash: String,
    package_hash: Option<String>,
    token_id: String,
) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.token_metadata(&token_id).await)
}

pub async fn get_owner(contract_hash: String, package_hash: Option<String>) -> ToolOutput {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.get_owner().await)
}

pub async fn transfer_ownership(
    contract_hash: String,
    package_hash: Option<String>,
    new_owner: String,
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
    params::map_call(client.transfer_ownership(&new_owner, &tx).await)
}
