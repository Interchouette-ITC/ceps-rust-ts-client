//! CEP-95 MCP tool bodies.

use crate::format;
use crate::tools::params;
use rmcp::model::CallToolResult;

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

pub async fn install(a: crate::tool_args::Ceps95InstallArgs) -> CallToolResult {
    let args = params::cep95_install_args(
        a.name,
        a.symbol,
        a.package_hash_key_name,
        a.allow_key_override,
        a.is_upgradable,
        a.is_upgrade,
    );
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
    let client = match params::cep95_client(None, None) {
        Ok(c) => c,
        Err(e) => return format::err(e),
    };
    params::map_call(client.install(&args, &wasm, &tx).await)
}

pub async fn bind_odra_install(
    installer_public_key: String,
    package_hash_key_name: String,
) -> CallToolResult {
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

pub async fn mint(a: crate::tool_args::Ceps95MintArgs) -> CallToolResult {
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
    params::map_call(client.mint(&a.to, &a.token_id, None, &tx).await)
}

pub async fn burn(a: crate::tool_args::Ceps95BurnArgs) -> CallToolResult {
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
    params::map_call(client.burn(&a.token_id, &tx).await)
}

pub async fn transfer_from(a: crate::tool_args::Ceps95TransferFromArgs) -> CallToolResult {
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
    params::map_call(client.transfer_from(&a.from, &a.to, &a.token_id, &tx).await)
}

pub async fn safe_transfer_from(a: crate::tool_args::Ceps95SafeTransferFromArgs) -> CallToolResult {
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
            .safe_transfer_from(&a.from, &a.to, &a.token_id, None, &tx)
            .await,
    )
}

pub async fn approve(a: crate::tool_args::Ceps95ApproveArgs) -> CallToolResult {
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
    params::map_call(client.approve(&a.spender, &a.token_id, &tx).await)
}

pub async fn revoke_approval(a: crate::tool_args::Ceps95RevokeApprovalArgs) -> CallToolResult {
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
    params::map_call(client.revoke_approval(&a.token_id, &tx).await)
}

pub async fn approve_for_all(a: crate::tool_args::Ceps95ApproveForAllArgs) -> CallToolResult {
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
    params::map_call(client.approve_for_all(&a.operator, &tx).await)
}

pub async fn revoke_approval_for_all(
    a: crate::tool_args::Ceps95RevokeApprovalForAllArgs,
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
    params::map_call(client.revoke_approval_for_all(&a.operator, &tx).await)
}

pub async fn name(contract_hash: String, package_hash: Option<String>) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.name().await)
}

pub async fn symbol(contract_hash: String, package_hash: Option<String>) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.symbol().await)
}

pub async fn total_supply(contract_hash: String, package_hash: Option<String>) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.total_supply().await)
}

pub async fn balance_of(
    contract_hash: String,
    package_hash: Option<String>,
    owner: String,
) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.balance_of(&owner).await)
}

pub async fn owner_of(
    contract_hash: String,
    package_hash: Option<String>,
    token_id: String,
) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.owner_of(&token_id).await)
}

pub async fn get_approved(
    contract_hash: String,
    package_hash: Option<String>,
    token_id: String,
) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.get_approved(&token_id).await)
}

pub async fn is_approved_for_all(
    contract_hash: String,
    package_hash: Option<String>,
    owner: String,
    operator: String,
) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.is_approved_for_all(&owner, &operator).await)
}

pub async fn token_metadata(
    contract_hash: String,
    package_hash: Option<String>,
    token_id: String,
) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.token_metadata(&token_id).await)
}

pub async fn get_owner(contract_hash: String, package_hash: Option<String>) -> CallToolResult {
    let client = need_client!(contract_hash, package_hash);
    params::map_query(client.get_owner().await)
}

pub async fn transfer_ownership(
    a: crate::tool_args::Ceps95TransferOwnershipArgs,
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
    params::map_call(client.transfer_ownership(&a.new_owner, &tx).await)
}
