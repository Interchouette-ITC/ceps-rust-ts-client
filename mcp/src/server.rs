//! MCP server (`mcpkit`) for `ceps-rust-ts-client-mcp` (stdio or Streamable HTTP).

#![allow(clippy::unused_async)]

use mcpkit::prelude::*;
use mcpkit::transport::stdio::StdioTransport;
use mcpkit_axum::McpRouter;

use crate::{format, tools};

/// MCP server handle exposing CEP client tools.
pub struct CepsClientMcp;

/// Default HTTP bind address for Streamable MCP.
pub const DEFAULT_HTTP_LISTEN: &str = "0.0.0.0:6790";

#[mcp_server(name = "ceps-rust-ts-client-mcp", version = "1.0.0")]
impl CepsClientMcp {
    #[tool(description = "Help: groups, env, endpoints, and available ceps_* tools")]
    async fn ceps_help(&self) -> ToolOutput {
        format::text_ok(tools::meta::help_text())
    }

    #[tool(description = "Show CEPS_RPC_URL / SSE / chain / verbosity / wasm_root")]
    async fn ceps_get_endpoints(&self) -> ToolOutput {
        tools::meta::get_endpoints()
    }

    #[tool(description = "Update process-wide CEP endpoints")]
    async fn ceps_set_endpoints(
        &self,
        rpc_url: Option<String>,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<String>,
    ) -> ToolOutput {
        tools::meta::set_endpoints(rpc_url, sse_url, chain_name, verbosity)
    }

    #[tool(description = "List registered ceps_* tool names and groups")]
    async fn ceps_list_tools(&self) -> ToolOutput {
        tools::meta::list_tools()
    }

    #[tool(description = "List demo contract .wasm files under CEPS_WASM_ROOT")]
    async fn ceps_list_contract_wasms(&self) -> ToolOutput {
        tools::wasm::list_contract_wasms()
    }

    #[tool(description = "Read a relative wasm path as base64")]
    async fn ceps_read_contract_wasm(&self, path: String) -> ToolOutput {
        tools::wasm::read_contract_wasm(path)
    }

    #[tool(description = "Recommended install/session wasm paths per CEP")]
    async fn ceps_canonical_wasm_paths(&self) -> ToolOutput {
        tools::wasm::canonical_wasm_paths()
    }

    #[tool(description = "CEPS18 install")]
    async fn ceps18_install(
        &self,
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
        tools::cep18::install(
            name,
            symbol,
            decimals,
            total_supply,
            secret_key_pem,
            payment_amount,
            wasm_path,
            wasm_base64,
            events_mode,
            enable_mint_and_burn,
            admin_list,
            minter_list,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS18 upgrade")]
    async fn ceps18_upgrade(
        &self,
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
        tools::cep18::upgrade(
            name,
            secret_key_pem,
            payment_amount,
            wasm_path,
            wasm_base64,
            events_mode,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS18 transfer")]
    async fn ceps18_transfer(
        &self,
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
        tools::cep18::transfer(
            contract_hash,
            package_hash,
            recipient,
            amount,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS18 transfer_from")]
    async fn ceps18_transfer_from(
        &self,
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
        tools::cep18::transfer_from(
            contract_hash,
            package_hash,
            owner,
            recipient,
            amount,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS18 approve")]
    async fn ceps18_approve(
        &self,
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
        tools::cep18::approve(
            contract_hash,
            package_hash,
            spender,
            amount,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS18 increase_allowance")]
    async fn ceps18_increase_allowance(
        &self,
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
        tools::cep18::increase_allowance(
            contract_hash,
            package_hash,
            spender,
            amount,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS18 decrease_allowance")]
    async fn ceps18_decrease_allowance(
        &self,
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
        tools::cep18::decrease_allowance(
            contract_hash,
            package_hash,
            spender,
            amount,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS18 mint")]
    async fn ceps18_mint(
        &self,
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
        tools::cep18::mint(
            contract_hash,
            package_hash,
            owner,
            amount,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS18 burn")]
    async fn ceps18_burn(
        &self,
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
        tools::cep18::burn(
            contract_hash,
            package_hash,
            owner,
            amount,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS18 change_security")]
    async fn ceps18_change_security(
        &self,
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
        tools::cep18::change_security(
            contract_hash,
            package_hash,
            secret_key_pem,
            payment_amount,
            admin_list,
            minter_list,
            none_list,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS18 change_events_mode")]
    async fn ceps18_change_events_mode(
        &self,
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
        tools::cep18::change_events_mode(
            contract_hash,
            package_hash,
            events_mode,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS18 name")]
    async fn ceps18_name(&self, contract_hash: String, package_hash: Option<String>) -> ToolOutput {
        tools::cep18::name(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS18 symbol")]
    async fn ceps18_symbol(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep18::symbol(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS18 decimals")]
    async fn ceps18_decimals(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep18::decimals(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS18 total_supply")]
    async fn ceps18_total_supply(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep18::total_supply(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS18 events_mode")]
    async fn ceps18_events_mode(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep18::events_mode(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS18 is_mint_and_burn_enabled")]
    async fn ceps18_is_mint_and_burn_enabled(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep18::is_mint_and_burn_enabled(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS18 balance_of")]
    async fn ceps18_balance_of(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        account: String,
    ) -> ToolOutput {
        tools::cep18::balance_of(contract_hash, package_hash, account).await
    }

    #[tool(description = "CEPS18 allowances")]
    async fn ceps18_allowances(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        owner: String,
        spender: String,
    ) -> ToolOutput {
        tools::cep18::allowances(contract_hash, package_hash, owner, spender).await
    }

    #[tool(description = "CEPS78 install")]
    async fn ceps78_install(
        &self,
        collection_name: String,
        collection_symbol: String,
        total_token_supply: u64,
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
        tools::cep78::install(
            collection_name,
            collection_symbol,
            total_token_supply,
            secret_key_pem,
            payment_amount,
            wasm_path,
            wasm_base64,
            events_mode,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 upgrade")]
    async fn ceps78_upgrade(
        &self,
        collection_name: String,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wasm_path: Option<String>,
        wasm_base64: Option<String>,
        total_token_supply: Option<u64>,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::upgrade(
            collection_name,
            secret_key_pem,
            payment_amount,
            wasm_path,
            wasm_base64,
            total_token_supply,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 mint")]
    async fn ceps78_mint(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        token_owner: String,
        token_meta_data: String,
        token_hash: Option<String>,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::mint(
            contract_hash,
            package_hash,
            token_owner,
            token_meta_data,
            token_hash,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 mint_session")]
    async fn ceps78_mint_session(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        token_owner: String,
        token_meta_data: String,
        token_hash: Option<String>,
        secret_key_pem: Option<String>,
        payment_amount: String,
        session_wasm_path: Option<String>,
        session_wasm_base64: Option<String>,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::mint_session(
            contract_hash,
            package_hash,
            token_owner,
            token_meta_data,
            token_hash,
            secret_key_pem,
            payment_amount,
            session_wasm_path,
            session_wasm_base64,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 burn")]
    async fn ceps78_burn(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        token_id: Option<u64>,
        token_hash: Option<String>,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::burn(
            contract_hash,
            package_hash,
            token_id,
            token_hash,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 transfer")]
    async fn ceps78_transfer(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        source: String,
        target: String,
        token_id: Option<u64>,
        token_hash: Option<String>,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::transfer(
            contract_hash,
            package_hash,
            source,
            target,
            token_id,
            token_hash,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 transfer_session")]
    async fn ceps78_transfer_session(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        source: String,
        target: String,
        token_id: Option<u64>,
        token_hash: Option<String>,
        secret_key_pem: Option<String>,
        payment_amount: String,
        session_wasm_path: Option<String>,
        session_wasm_base64: Option<String>,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::transfer_session(
            contract_hash,
            package_hash,
            source,
            target,
            token_id,
            token_hash,
            secret_key_pem,
            payment_amount,
            session_wasm_path,
            session_wasm_base64,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 register_owner")]
    async fn ceps78_register_owner(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        token_owner: String,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::register_owner(
            contract_hash,
            package_hash,
            token_owner,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 approve")]
    async fn ceps78_approve(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        operator: String,
        token_id: Option<u64>,
        token_hash: Option<String>,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::approve(
            contract_hash,
            package_hash,
            operator,
            token_id,
            token_hash,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 revoke")]
    async fn ceps78_revoke(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        operator: String,
        token_id: Option<u64>,
        token_hash: Option<String>,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::revoke(
            contract_hash,
            package_hash,
            operator,
            token_id,
            token_hash,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 set_approval_for_all")]
    async fn ceps78_set_approval_for_all(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        operator: String,
        approve_all: bool,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::set_approval_for_all(
            contract_hash,
            package_hash,
            operator,
            approve_all,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 set_token_metadata")]
    async fn ceps78_set_token_metadata(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        token_meta_data: String,
        token_id: Option<u64>,
        token_hash: Option<String>,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::set_token_metadata(
            contract_hash,
            package_hash,
            token_meta_data,
            token_id,
            token_hash,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 set_variables")]
    async fn ceps78_set_variables(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        secret_key_pem: Option<String>,
        payment_amount: String,
        allow_minting: Option<bool>,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::set_variables(
            contract_hash,
            package_hash,
            secret_key_pem,
            payment_amount,
            allow_minting,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 updated_receipts")]
    async fn ceps78_updated_receipts(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        secret_key_pem: Option<String>,
        payment_amount: String,
        session_wasm_path: Option<String>,
        session_wasm_base64: Option<String>,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::updated_receipts(
            contract_hash,
            package_hash,
            secret_key_pem,
            payment_amount,
            session_wasm_path,
            session_wasm_base64,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 collection_name")]
    async fn ceps78_collection_name(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::collection_name(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 collection_symbol")]
    async fn ceps78_collection_symbol(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::collection_symbol(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 total_token_supply")]
    async fn ceps78_total_token_supply(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::total_token_supply(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 number_of_minted_tokens")]
    async fn ceps78_number_of_minted_tokens(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::number_of_minted_tokens(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 events_mode")]
    async fn ceps78_events_mode(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::events_mode(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 allow_minting")]
    async fn ceps78_allow_minting(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::allow_minting(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 minting_mode")]
    async fn ceps78_minting_mode(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::minting_mode(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 whitelist_mode")]
    async fn ceps78_whitelist_mode(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::whitelist_mode(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 reporting_mode (owner reverse lookup)")]
    async fn ceps78_reporting_mode(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::reporting_mode(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 burn_mode")]
    async fn ceps78_burn_mode(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::burn_mode(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 operator_burn_mode")]
    async fn ceps78_operator_burn_mode(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::operator_burn_mode(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 holder_mode")]
    async fn ceps78_holder_mode(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::holder_mode(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 identifier_mode")]
    async fn ceps78_identifier_mode(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::identifier_mode(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 metadata_mutability")]
    async fn ceps78_metadata_mutability(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::metadata_mutability(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 nft_kind")]
    async fn ceps78_nft_kind(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::nft_kind(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 nft_metadata_kind")]
    async fn ceps78_nft_metadata_kind(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::nft_metadata_kind(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 ownership_mode")]
    async fn ceps78_ownership_mode(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::ownership_mode(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 package_operator_mode")]
    async fn ceps78_package_operator_mode(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::package_operator_mode(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 acl_package_mode")]
    async fn ceps78_acl_package_mode(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::acl_package_mode(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 json_schema")]
    async fn ceps78_json_schema(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::json_schema(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS78 is_acl_whitelisted")]
    async fn ceps78_is_acl_whitelisted(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        entity: String,
    ) -> ToolOutput {
        tools::cep78::is_acl_whitelisted(contract_hash, package_hash, entity).await
    }

    #[tool(description = "CEPS78 owner_of_session")]
    async fn ceps78_owner_of_session(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        token_id: Option<u64>,
        token_hash: Option<String>,
        key_name: String,
        secret_key_pem: Option<String>,
        payment_amount: String,
        session_wasm_path: Option<String>,
        session_wasm_base64: Option<String>,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::owner_of_session(
            contract_hash,
            package_hash,
            token_id,
            token_hash,
            key_name,
            secret_key_pem,
            payment_amount,
            session_wasm_path,
            session_wasm_base64,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 balance_of_session")]
    async fn ceps78_balance_of_session(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        token_owner: String,
        key_name: String,
        secret_key_pem: Option<String>,
        payment_amount: String,
        session_wasm_path: Option<String>,
        session_wasm_base64: Option<String>,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::balance_of_session(
            contract_hash,
            package_hash,
            token_owner,
            key_name,
            secret_key_pem,
            payment_amount,
            session_wasm_path,
            session_wasm_base64,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 get_approved_session")]
    async fn ceps78_get_approved_session(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        token_id: Option<u64>,
        token_hash: Option<String>,
        key_name: String,
        secret_key_pem: Option<String>,
        payment_amount: String,
        session_wasm_path: Option<String>,
        session_wasm_base64: Option<String>,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::get_approved_session(
            contract_hash,
            package_hash,
            token_id,
            token_hash,
            key_name,
            secret_key_pem,
            payment_amount,
            session_wasm_path,
            session_wasm_base64,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS78 is_approved_for_all_session")]
    async fn ceps78_is_approved_for_all_session(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        token_owner: String,
        operator: String,
        key_name: String,
        secret_key_pem: Option<String>,
        payment_amount: String,
        session_wasm_path: Option<String>,
        session_wasm_base64: Option<String>,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep78::is_approved_for_all_session(
            contract_hash,
            package_hash,
            token_owner,
            operator,
            key_name,
            secret_key_pem,
            payment_amount,
            session_wasm_path,
            session_wasm_base64,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "Parse CES events from execution JSON for a contract hash")]
    async fn ceps_ces_parse_execution(
        &self,
        contract_hash: String,
        execution_result_json: String,
    ) -> ToolOutput {
        tools::ces::parse_execution(contract_hash, execution_result_json).await
    }

    #[tool(description = "Fetch a transaction and parse CES events for a contract hash")]
    async fn ceps_ces_parse_transaction(
        &self,
        contract_hash: String,
        transaction_hash: String,
    ) -> ToolOutput {
        tools::ces::parse_transaction(contract_hash, transaction_hash).await
    }

    #[tool(description = "CEPS78 owner_of")]
    async fn ceps78_owner_of(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        token_id: Option<u64>,
        token_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::owner_of(contract_hash, package_hash, token_id, token_hash).await
    }

    #[tool(description = "CEPS78 balance_of")]
    async fn ceps78_balance_of(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        owner: String,
    ) -> ToolOutput {
        tools::cep78::balance_of(contract_hash, package_hash, owner).await
    }

    #[tool(description = "CEPS78 get_approved")]
    async fn ceps78_get_approved(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        token_id: Option<u64>,
        token_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep78::get_approved(contract_hash, package_hash, token_id, token_hash).await
    }

    #[tool(description = "CEPS78 is_approved_for_all")]
    async fn ceps78_is_approved_for_all(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        owner: String,
        operator: String,
    ) -> ToolOutput {
        tools::cep78::is_approved_for_all(contract_hash, package_hash, owner, operator).await
    }

    #[tool(description = "CEPS78 metadata")]
    async fn ceps78_metadata(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        token_id: Option<u64>,
        token_hash: Option<String>,
        nft_metadata_kind: u8,
    ) -> ToolOutput {
        tools::cep78::metadata(
            contract_hash,
            package_hash,
            token_id,
            token_hash,
            nft_metadata_kind,
        )
        .await
    }

    #[tool(description = "CEPS85 install")]
    async fn ceps85_install(
        &self,
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
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep85::install(
            name,
            uri,
            secret_key_pem,
            payment_amount,
            wasm_path,
            wasm_base64,
            events_mode,
            enable_burn,
            admin_list,
            minter_list,
            burner_list,
            meta_list,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS85 upgrade")]
    async fn ceps85_upgrade(
        &self,
        name: String,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wasm_path: Option<String>,
        wasm_base64: Option<String>,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep85::upgrade(
            name,
            secret_key_pem,
            payment_amount,
            wasm_path,
            wasm_base64,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS85 mint")]
    async fn ceps85_mint(
        &self,
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
        tools::cep85::mint(
            contract_hash,
            package_hash,
            recipient,
            id,
            amount,
            uri,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS85 batch_mint")]
    async fn ceps85_batch_mint(
        &self,
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
        tools::cep85::batch_mint(
            contract_hash,
            package_hash,
            recipient,
            ids,
            amounts,
            uri,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS85 burn")]
    async fn ceps85_burn(
        &self,
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
        tools::cep85::burn(
            contract_hash,
            package_hash,
            owner,
            id,
            amount,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS85 batch_burn")]
    async fn ceps85_batch_burn(
        &self,
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
        tools::cep85::batch_burn(
            contract_hash,
            package_hash,
            owner,
            ids,
            amounts,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS85 transfer")]
    async fn ceps85_transfer(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        from: String,
        to: String,
        id: String,
        amount: String,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep85::transfer(
            contract_hash,
            package_hash,
            from,
            to,
            id,
            amount,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS85 batch_transfer")]
    async fn ceps85_batch_transfer(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        from: String,
        to: String,
        ids: Vec<String>,
        amounts: Vec<String>,
        secret_key_pem: Option<String>,
        payment_amount: String,
        wait: Option<bool>,
        wait_timeout_ms: Option<u64>,
        make_only: Option<bool>,
        initiator_addr: Option<String>,
    ) -> ToolOutput {
        tools::cep85::batch_transfer(
            contract_hash,
            package_hash,
            from,
            to,
            ids,
            amounts,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS85 set_approval_for_all")]
    async fn ceps85_set_approval_for_all(
        &self,
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
        tools::cep85::set_approval_for_all(
            contract_hash,
            package_hash,
            operator,
            approved,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS85 set_uri")]
    async fn ceps85_set_uri(
        &self,
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
        tools::cep85::set_uri(
            contract_hash,
            package_hash,
            uri,
            id,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS85 set_total_supply_of")]
    async fn ceps85_set_total_supply_of(
        &self,
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
        tools::cep85::set_total_supply_of(
            contract_hash,
            package_hash,
            id,
            total_supply,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS85 set_total_supply_of_batch")]
    async fn ceps85_set_total_supply_of_batch(
        &self,
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
        tools::cep85::set_total_supply_of_batch(
            contract_hash,
            package_hash,
            ids,
            total_supplies,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS85 change_security")]
    async fn ceps85_change_security(
        &self,
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
        tools::cep85::change_security(
            contract_hash,
            package_hash,
            secret_key_pem,
            payment_amount,
            admin_list,
            minter_list,
            burner_list,
            meta_list,
            none_list,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS85 set_modalities")]
    async fn ceps85_set_modalities(
        &self,
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
        tools::cep85::set_modalities(
            contract_hash,
            package_hash,
            secret_key_pem,
            payment_amount,
            enable_burn,
            events_mode,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS85 collection_name")]
    async fn ceps85_collection_name(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep85::collection_name(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS85 collection_uri")]
    async fn ceps85_collection_uri(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep85::collection_uri(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS85 balance_of")]
    async fn ceps85_balance_of(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        account: String,
        id: String,
    ) -> ToolOutput {
        tools::cep85::balance_of(contract_hash, package_hash, account, id).await
    }

    #[tool(description = "CEPS85 is_approved_for_all")]
    async fn ceps85_is_approved_for_all(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        owner: String,
        operator: String,
    ) -> ToolOutput {
        tools::cep85::is_approved_for_all(contract_hash, package_hash, owner, operator).await
    }

    #[tool(description = "CEPS85 supply_of")]
    async fn ceps85_supply_of(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        id: String,
    ) -> ToolOutput {
        tools::cep85::supply_of(contract_hash, package_hash, id).await
    }

    #[tool(description = "CEPS85 total_supply_of")]
    async fn ceps85_total_supply_of(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        id: String,
    ) -> ToolOutput {
        tools::cep85::total_supply_of(contract_hash, package_hash, id).await
    }

    #[tool(description = "CEPS85 uri")]
    async fn ceps85_uri(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        id: Option<String>,
    ) -> ToolOutput {
        tools::cep85::uri(contract_hash, package_hash, id).await
    }

    #[tool(description = "CEPS85 is_non_fungible")]
    async fn ceps85_is_non_fungible(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        id: String,
    ) -> ToolOutput {
        tools::cep85::is_non_fungible(contract_hash, package_hash, id).await
    }

    #[tool(description = "CEPS95 Odra install")]
    async fn ceps95_install(
        &self,
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
        tools::cep95::install(
            name,
            symbol,
            package_hash_key_name,
            secret_key_pem,
            payment_amount,
            wasm_path,
            wasm_base64,
            allow_key_override,
            is_upgradable,
            is_upgrade,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS95 bind Odra install (package named key → contract+package)")]
    async fn ceps95_bind_odra_install(
        &self,
        installer_public_key: String,
        package_hash_key_name: String,
    ) -> ToolOutput {
        tools::cep95::bind_odra_install(installer_public_key, package_hash_key_name).await
    }

    #[tool(description = "CEPS95 mint")]
    async fn ceps95_mint(
        &self,
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
        tools::cep95::mint(
            contract_hash,
            package_hash,
            to,
            token_id,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS95 burn")]
    async fn ceps95_burn(
        &self,
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
        tools::cep95::burn(
            contract_hash,
            package_hash,
            token_id,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS95 transfer_from")]
    async fn ceps95_transfer_from(
        &self,
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
        tools::cep95::transfer_from(
            contract_hash,
            package_hash,
            from,
            to,
            token_id,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS95 safe_transfer_from")]
    async fn ceps95_safe_transfer_from(
        &self,
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
        tools::cep95::safe_transfer_from(
            contract_hash,
            package_hash,
            from,
            to,
            token_id,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS95 approve")]
    async fn ceps95_approve(
        &self,
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
        tools::cep95::approve(
            contract_hash,
            package_hash,
            spender,
            token_id,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS95 revoke_approval")]
    async fn ceps95_revoke_approval(
        &self,
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
        tools::cep95::revoke_approval(
            contract_hash,
            package_hash,
            token_id,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS95 approve_for_all")]
    async fn ceps95_approve_for_all(
        &self,
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
        tools::cep95::approve_for_all(
            contract_hash,
            package_hash,
            operator,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS95 revoke_approval_for_all")]
    async fn ceps95_revoke_approval_for_all(
        &self,
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
        tools::cep95::revoke_approval_for_all(
            contract_hash,
            package_hash,
            operator,
            secret_key_pem,
            payment_amount,
            wait,
            wait_timeout_ms,
            make_only,
            initiator_addr,
        )
        .await
    }

    #[tool(description = "CEPS95 name")]
    async fn ceps95_name(&self, contract_hash: String, package_hash: Option<String>) -> ToolOutput {
        tools::cep95::name(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS95 symbol")]
    async fn ceps95_symbol(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep95::symbol(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS95 total_supply")]
    async fn ceps95_total_supply(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
    ) -> ToolOutput {
        tools::cep95::total_supply(contract_hash, package_hash).await
    }

    #[tool(description = "CEPS95 balance_of")]
    async fn ceps95_balance_of(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        owner: String,
    ) -> ToolOutput {
        tools::cep95::balance_of(contract_hash, package_hash, owner).await
    }

    #[tool(description = "CEPS95 owner_of")]
    async fn ceps95_owner_of(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        token_id: String,
    ) -> ToolOutput {
        tools::cep95::owner_of(contract_hash, package_hash, token_id).await
    }

    #[tool(description = "CEPS95 get_approved")]
    async fn ceps95_get_approved(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        token_id: String,
    ) -> ToolOutput {
        tools::cep95::get_approved(contract_hash, package_hash, token_id).await
    }

    #[tool(description = "CEPS95 is_approved_for_all")]
    async fn ceps95_is_approved_for_all(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        owner: String,
        operator: String,
    ) -> ToolOutput {
        tools::cep95::is_approved_for_all(contract_hash, package_hash, owner, operator).await
    }

    #[tool(description = "CEPS95 token_metadata")]
    async fn ceps95_token_metadata(
        &self,
        contract_hash: String,
        package_hash: Option<String>,
        token_id: String,
    ) -> ToolOutput {
        tools::cep95::token_metadata(contract_hash, package_hash, token_id).await
    }
}

/// Serves MCP over stdio until the client disconnects.
pub async fn run() -> Result<(), McpError> {
    let transport = StdioTransport::new();
    let server = ServerBuilder::new(CepsClientMcp)
        .with_tools(CepsClientMcp)
        .build();
    server.serve(transport).await
}

/// Serves MCP over Streamable HTTP until the process is stopped.
pub async fn run_http(addr: &str) -> std::io::Result<()> {
    McpRouter::new(CepsClientMcp).serve(addr).await
}

impl ResourceHandler for CepsClientMcp {
    async fn list_resources(&self, _ctx: &Context<'_>) -> Result<Vec<Resource>, McpError> {
        Ok(Vec::new())
    }

    async fn read_resource(
        &self,
        uri: &str,
        _ctx: &Context<'_>,
    ) -> Result<Vec<ResourceContents>, McpError> {
        Err(McpError::invalid_params(
            "resources/read",
            format!("unknown resource: {uri}"),
        ))
    }
}

impl PromptHandler for CepsClientMcp {
    async fn list_prompts(&self, _ctx: &Context<'_>) -> Result<Vec<Prompt>, McpError> {
        Ok(Vec::new())
    }

    async fn get_prompt(
        &self,
        name: &str,
        _args: Option<serde_json::Map<String, serde_json::Value>>,
        _ctx: &Context<'_>,
    ) -> Result<GetPromptResult, McpError> {
        Err(McpError::invalid_params(
            "prompts/get",
            format!("unknown prompt: {name}"),
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_matches_cargo() {
        assert_eq!(env!("CARGO_PKG_VERSION"), "1.0.0");
    }
}
