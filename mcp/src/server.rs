//! MCP server (`rmcp`) for `ceps-rust-ts-client-mcp` (stdio or Streamable HTTP).

#![allow(clippy::unused_async)]

use std::sync::Arc;

use rmcp::{
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
    transport::stdio,
    ErrorData as McpError, ServerHandler, ServiceExt,
};

use crate::tool_args::*;
use crate::{format, tools};

/// MCP server handle.
#[derive(Clone, Default)]
pub struct CepsClientMcp;

/// Default HTTP bind address for Streamable MCP.
pub const DEFAULT_HTTP_LISTEN: &str = "0.0.0.0:6790";

#[tool_router]
impl CepsClientMcp {
    #[tool(description = "Help: groups, env, endpoints, and available ceps_* tools")]
    async fn ceps_help(&self) -> Result<CallToolResult, McpError> {
        Ok(format::text_ok(tools::meta::help_text()))
    }

    #[tool(description = "Show CEPS_RPC_URL / SSE / chain / verbosity / wasm_root")]
    async fn ceps_get_endpoints(&self) -> Result<CallToolResult, McpError> {
        Ok(tools::meta::get_endpoints())
    }

    #[tool(description = "Update process-wide CEP endpoints")]
    async fn ceps_set_endpoints(
        &self,
        Parameters(CepsSetEndpointsArgs {
            rpc_url,
            sse_url,
            chain_name,
            verbosity,
        }): Parameters<CepsSetEndpointsArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::meta::set_endpoints(
            rpc_url, sse_url, chain_name, verbosity,
        ))
    }

    #[tool(description = "List registered ceps_* tool names and groups")]
    async fn ceps_list_tools(&self) -> Result<CallToolResult, McpError> {
        Ok(tools::meta::list_tools())
    }

    #[tool(description = "List demo contract .wasm files under CEPS_WASM_ROOT")]
    async fn ceps_list_contract_wasms(&self) -> Result<CallToolResult, McpError> {
        Ok(tools::wasm::list_contract_wasms())
    }

    #[tool(description = "Read a relative wasm path as base64")]
    async fn ceps_read_contract_wasm(
        &self,
        Parameters(CepsReadContractWasmArgs { path }): Parameters<CepsReadContractWasmArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::wasm::read_contract_wasm(path))
    }

    #[tool(description = "Recommended install/session wasm paths per CEP")]
    async fn ceps_canonical_wasm_paths(&self) -> Result<CallToolResult, McpError> {
        Ok(tools::wasm::canonical_wasm_paths())
    }

    #[tool(description = "CEPS18 install")]
    async fn ceps18_install(
        &self,
        Parameters(args): Parameters<Ceps18InstallArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::install(args).await)
    }

    #[tool(description = "CEPS18 upgrade")]
    async fn ceps18_upgrade(
        &self,
        Parameters(args): Parameters<Ceps18UpgradeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::upgrade(args).await)
    }

    #[tool(description = "CEPS18 transfer")]
    async fn ceps18_transfer(
        &self,
        Parameters(args): Parameters<Ceps18TransferArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::transfer(args).await)
    }

    #[tool(description = "CEPS18 transfer_from")]
    async fn ceps18_transfer_from(
        &self,
        Parameters(args): Parameters<Ceps18TransferFromArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::transfer_from(args).await)
    }

    #[tool(description = "CEPS18 approve")]
    async fn ceps18_approve(
        &self,
        Parameters(args): Parameters<Ceps18ApproveArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::approve(args).await)
    }

    #[tool(description = "CEPS18 increase_allowance")]
    async fn ceps18_increase_allowance(
        &self,
        Parameters(args): Parameters<Ceps18IncreaseAllowanceArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::increase_allowance(args).await)
    }

    #[tool(description = "CEPS18 decrease_allowance")]
    async fn ceps18_decrease_allowance(
        &self,
        Parameters(args): Parameters<Ceps18DecreaseAllowanceArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::decrease_allowance(args).await)
    }

    #[tool(description = "CEPS18 mint")]
    async fn ceps18_mint(
        &self,
        Parameters(args): Parameters<Ceps18MintArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::mint(args).await)
    }

    #[tool(description = "CEPS18 burn")]
    async fn ceps18_burn(
        &self,
        Parameters(args): Parameters<Ceps18BurnArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::burn(args).await)
    }

    #[tool(description = "CEPS18 change_security")]
    async fn ceps18_change_security(
        &self,
        Parameters(args): Parameters<Ceps18ChangeSecurityArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::change_security(args).await)
    }

    #[tool(description = "CEPS18 change_events_mode")]
    async fn ceps18_change_events_mode(
        &self,
        Parameters(args): Parameters<Ceps18ChangeEventsModeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::change_events_mode(args).await)
    }

    #[tool(description = "CEPS18 name")]
    async fn ceps18_name(
        &self,
        Parameters(Ceps18NameArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps18NameArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::name(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS18 symbol")]
    async fn ceps18_symbol(
        &self,
        Parameters(Ceps18SymbolArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps18SymbolArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::symbol(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS18 decimals")]
    async fn ceps18_decimals(
        &self,
        Parameters(Ceps18DecimalsArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps18DecimalsArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::decimals(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS18 total_supply")]
    async fn ceps18_total_supply(
        &self,
        Parameters(Ceps18TotalSupplyArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps18TotalSupplyArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::total_supply(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS18 events_mode")]
    async fn ceps18_events_mode(
        &self,
        Parameters(Ceps18EventsModeArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps18EventsModeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::events_mode(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS18 is_mint_and_burn_enabled")]
    async fn ceps18_is_mint_and_burn_enabled(
        &self,
        Parameters(Ceps18IsMintAndBurnEnabledArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps18IsMintAndBurnEnabledArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::is_mint_and_burn_enabled(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS18 balance_of")]
    async fn ceps18_balance_of(
        &self,
        Parameters(Ceps18BalanceOfArgs {
            contract_hash,
            package_hash,
            account,
        }): Parameters<Ceps18BalanceOfArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::balance_of(contract_hash, package_hash, account).await)
    }

    #[tool(description = "CEPS18 allowances")]
    async fn ceps18_allowances(
        &self,
        Parameters(Ceps18AllowancesArgs {
            contract_hash,
            package_hash,
            owner,
            spender,
        }): Parameters<Ceps18AllowancesArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::allowances(contract_hash, package_hash, owner, spender).await)
    }

    #[tool(description = "CEPS18 security_badge")]
    async fn ceps18_security_badge(
        &self,
        Parameters(Ceps18SecurityBadgeArgs {
            contract_hash,
            package_hash,
            account,
        }): Parameters<Ceps18SecurityBadgeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep18::security_badge(contract_hash, package_hash, account).await)
    }

    #[tool(description = "CEPS78 install")]
    async fn ceps78_install(
        &self,
        Parameters(args): Parameters<Ceps78InstallArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::install(args).await)
    }

    #[tool(description = "CEPS78 upgrade")]
    async fn ceps78_upgrade(
        &self,
        Parameters(args): Parameters<Ceps78UpgradeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::upgrade(args).await)
    }

    #[tool(description = "CEPS78 mint")]
    async fn ceps78_mint(
        &self,
        Parameters(args): Parameters<Ceps78MintArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::mint(args).await)
    }

    #[tool(description = "CEPS78 mint_session")]
    async fn ceps78_mint_session(
        &self,
        Parameters(args): Parameters<Ceps78MintSessionArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::mint_session(args).await)
    }

    #[tool(description = "CEPS78 burn")]
    async fn ceps78_burn(
        &self,
        Parameters(args): Parameters<Ceps78BurnArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::burn(args).await)
    }

    #[tool(description = "CEPS78 transfer")]
    async fn ceps78_transfer(
        &self,
        Parameters(args): Parameters<Ceps78TransferArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::transfer(args).await)
    }

    #[tool(description = "CEPS78 transfer_session")]
    async fn ceps78_transfer_session(
        &self,
        Parameters(args): Parameters<Ceps78TransferSessionArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::transfer_session(args).await)
    }

    #[tool(description = "CEPS78 register_owner")]
    async fn ceps78_register_owner(
        &self,
        Parameters(args): Parameters<Ceps78RegisterOwnerArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::register_owner(args).await)
    }

    #[tool(description = "CEPS78 approve")]
    async fn ceps78_approve(
        &self,
        Parameters(args): Parameters<Ceps78ApproveArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::approve(args).await)
    }

    #[tool(description = "CEPS78 revoke")]
    async fn ceps78_revoke(
        &self,
        Parameters(args): Parameters<Ceps78RevokeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::revoke(args).await)
    }

    #[tool(description = "CEPS78 set_approval_for_all")]
    async fn ceps78_set_approval_for_all(
        &self,
        Parameters(args): Parameters<Ceps78SetApprovalForAllArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::set_approval_for_all(args).await)
    }

    #[tool(description = "CEPS78 set_token_metadata")]
    async fn ceps78_set_token_metadata(
        &self,
        Parameters(args): Parameters<Ceps78SetTokenMetadataArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::set_token_metadata(args).await)
    }

    #[tool(description = "CEPS78 set_variables")]
    async fn ceps78_set_variables(
        &self,
        Parameters(args): Parameters<Ceps78SetVariablesArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::set_variables(args).await)
    }

    #[tool(description = "CEPS78 updated_receipts")]
    async fn ceps78_updated_receipts(
        &self,
        Parameters(args): Parameters<Ceps78UpdatedReceiptsArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::updated_receipts(args).await)
    }

    #[tool(description = "CEPS78 collection_name")]
    async fn ceps78_collection_name(
        &self,
        Parameters(Ceps78CollectionNameArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78CollectionNameArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::collection_name(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 collection_symbol")]
    async fn ceps78_collection_symbol(
        &self,
        Parameters(Ceps78CollectionSymbolArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78CollectionSymbolArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::collection_symbol(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 total_token_supply")]
    async fn ceps78_total_token_supply(
        &self,
        Parameters(Ceps78TotalTokenSupplyArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78TotalTokenSupplyArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::total_token_supply(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 number_of_minted_tokens")]
    async fn ceps78_number_of_minted_tokens(
        &self,
        Parameters(Ceps78NumberOfMintedTokensArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78NumberOfMintedTokensArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::number_of_minted_tokens(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 events_mode")]
    async fn ceps78_events_mode(
        &self,
        Parameters(Ceps78EventsModeArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78EventsModeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::events_mode(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 allow_minting")]
    async fn ceps78_allow_minting(
        &self,
        Parameters(Ceps78AllowMintingArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78AllowMintingArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::allow_minting(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 minting_mode")]
    async fn ceps78_minting_mode(
        &self,
        Parameters(Ceps78MintingModeArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78MintingModeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::minting_mode(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 whitelist_mode")]
    async fn ceps78_whitelist_mode(
        &self,
        Parameters(Ceps78WhitelistModeArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78WhitelistModeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::whitelist_mode(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 reporting_mode (owner reverse lookup)")]
    async fn ceps78_reporting_mode(
        &self,
        Parameters(Ceps78ReportingModeArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78ReportingModeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::reporting_mode(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 burn_mode")]
    async fn ceps78_burn_mode(
        &self,
        Parameters(Ceps78BurnModeArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78BurnModeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::burn_mode(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 operator_burn_mode")]
    async fn ceps78_operator_burn_mode(
        &self,
        Parameters(Ceps78OperatorBurnModeArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78OperatorBurnModeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::operator_burn_mode(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 holder_mode")]
    async fn ceps78_holder_mode(
        &self,
        Parameters(Ceps78HolderModeArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78HolderModeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::holder_mode(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 identifier_mode")]
    async fn ceps78_identifier_mode(
        &self,
        Parameters(Ceps78IdentifierModeArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78IdentifierModeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::identifier_mode(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 metadata_mutability")]
    async fn ceps78_metadata_mutability(
        &self,
        Parameters(Ceps78MetadataMutabilityArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78MetadataMutabilityArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::metadata_mutability(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 nft_kind")]
    async fn ceps78_nft_kind(
        &self,
        Parameters(Ceps78NftKindArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78NftKindArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::nft_kind(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 nft_metadata_kind")]
    async fn ceps78_nft_metadata_kind(
        &self,
        Parameters(Ceps78NftMetadataKindArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78NftMetadataKindArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::nft_metadata_kind(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 ownership_mode")]
    async fn ceps78_ownership_mode(
        &self,
        Parameters(Ceps78OwnershipModeArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78OwnershipModeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::ownership_mode(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 package_operator_mode")]
    async fn ceps78_package_operator_mode(
        &self,
        Parameters(Ceps78PackageOperatorModeArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78PackageOperatorModeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::package_operator_mode(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 acl_package_mode")]
    async fn ceps78_acl_package_mode(
        &self,
        Parameters(Ceps78AclPackageModeArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78AclPackageModeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::acl_package_mode(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 json_schema")]
    async fn ceps78_json_schema(
        &self,
        Parameters(Ceps78JsonSchemaArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps78JsonSchemaArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::json_schema(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS78 is_acl_whitelisted")]
    async fn ceps78_is_acl_whitelisted(
        &self,
        Parameters(Ceps78IsAclWhitelistedArgs {
            contract_hash,
            package_hash,
            entity,
        }): Parameters<Ceps78IsAclWhitelistedArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::is_acl_whitelisted(contract_hash, package_hash, entity).await)
    }

    #[tool(description = "CEPS78 owner_of_session")]
    async fn ceps78_owner_of_session(
        &self,
        Parameters(args): Parameters<Ceps78OwnerOfSessionArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::owner_of_session(args).await)
    }

    #[tool(description = "CEPS78 balance_of_session")]
    async fn ceps78_balance_of_session(
        &self,
        Parameters(args): Parameters<Ceps78BalanceOfSessionArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::balance_of_session(args).await)
    }

    #[tool(description = "CEPS78 get_approved_session")]
    async fn ceps78_get_approved_session(
        &self,
        Parameters(args): Parameters<Ceps78GetApprovedSessionArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::get_approved_session(args).await)
    }

    #[tool(description = "CEPS78 is_approved_for_all_session")]
    async fn ceps78_is_approved_for_all_session(
        &self,
        Parameters(args): Parameters<Ceps78IsApprovedForAllSessionArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::is_approved_for_all_session(args).await)
    }

    #[tool(description = "Put a signed Transaction JSON (CEPClient::put_transaction)")]
    async fn ceps_put_transaction(
        &self,
        Parameters(args): Parameters<CepsPutTransactionArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::client::put_transaction(args).await)
    }

    #[tool(description = "Wait for a transaction hash on SSE (CEPClient::wait_transaction)")]
    async fn ceps_wait_transaction(
        &self,
        Parameters(CepsWaitTransactionArgs {
            transaction_hash,
            wait_timeout_ms,
        }): Parameters<CepsWaitTransactionArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::client::wait_transaction(transaction_hash, wait_timeout_ms).await)
    }

    #[tool(description = "Parse CES events from execution JSON for a contract hash")]
    async fn ceps_ces_parse_execution(
        &self,
        Parameters(CepsCesParseExecutionArgs {
            contract_hash,
            execution_result_json,
        }): Parameters<CepsCesParseExecutionArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::ces::parse_execution(contract_hash, execution_result_json).await)
    }

    #[tool(description = "Fetch a transaction and parse CES events for a contract hash")]
    async fn ceps_ces_parse_transaction(
        &self,
        Parameters(CepsCesParseTransactionArgs {
            contract_hash,
            transaction_hash,
        }): Parameters<CepsCesParseTransactionArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::ces::parse_transaction(contract_hash, transaction_hash).await)
    }

    #[tool(
        description = "Collect SSE TransactionProcessed frames and decode CES for a bound contract"
    )]
    async fn ceps_ces_collect(
        &self,
        Parameters(args): Parameters<CepsCesCollectArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::ces::collect(args).await)
    }

    #[tool(description = "CEPS78 owner_of")]
    async fn ceps78_owner_of(
        &self,
        Parameters(Ceps78OwnerOfArgs {
            contract_hash,
            package_hash,
            token_id,
            token_hash,
        }): Parameters<Ceps78OwnerOfArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::owner_of(contract_hash, package_hash, token_id, token_hash).await)
    }

    #[tool(description = "CEPS78 balance_of")]
    async fn ceps78_balance_of(
        &self,
        Parameters(Ceps78BalanceOfArgs {
            contract_hash,
            package_hash,
            owner,
        }): Parameters<Ceps78BalanceOfArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::balance_of(contract_hash, package_hash, owner).await)
    }

    #[tool(description = "CEPS78 get_approved")]
    async fn ceps78_get_approved(
        &self,
        Parameters(Ceps78GetApprovedArgs {
            contract_hash,
            package_hash,
            token_id,
            token_hash,
        }): Parameters<Ceps78GetApprovedArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::get_approved(contract_hash, package_hash, token_id, token_hash).await)
    }

    #[tool(description = "CEPS78 is_approved_for_all")]
    async fn ceps78_is_approved_for_all(
        &self,
        Parameters(Ceps78IsApprovedForAllArgs {
            contract_hash,
            package_hash,
            owner,
            operator,
        }): Parameters<Ceps78IsApprovedForAllArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::is_approved_for_all(contract_hash, package_hash, owner, operator).await)
    }

    #[tool(description = "CEPS78 metadata")]
    async fn ceps78_metadata(
        &self,
        Parameters(args): Parameters<Ceps78MetadataArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep78::metadata(args).await)
    }

    #[tool(description = "CEPS85 install")]
    async fn ceps85_install(
        &self,
        Parameters(args): Parameters<Ceps85InstallArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::install(args).await)
    }

    #[tool(description = "CEPS85 upgrade")]
    async fn ceps85_upgrade(
        &self,
        Parameters(args): Parameters<Ceps85UpgradeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::upgrade(args).await)
    }

    #[tool(description = "CEPS85 mint")]
    async fn ceps85_mint(
        &self,
        Parameters(args): Parameters<Ceps85MintArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::mint(args).await)
    }

    #[tool(description = "CEPS85 batch_mint")]
    async fn ceps85_batch_mint(
        &self,
        Parameters(args): Parameters<Ceps85BatchMintArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::batch_mint(args).await)
    }

    #[tool(description = "CEPS85 burn")]
    async fn ceps85_burn(
        &self,
        Parameters(args): Parameters<Ceps85BurnArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::burn(args).await)
    }

    #[tool(description = "CEPS85 batch_burn")]
    async fn ceps85_batch_burn(
        &self,
        Parameters(args): Parameters<Ceps85BatchBurnArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::batch_burn(args).await)
    }

    #[tool(description = "CEPS85 transfer")]
    async fn ceps85_transfer(
        &self,
        Parameters(args): Parameters<Ceps85TransferArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::transfer(args).await)
    }

    #[tool(description = "CEPS85 batch_transfer")]
    async fn ceps85_batch_transfer(
        &self,
        Parameters(args): Parameters<Ceps85BatchTransferArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::batch_transfer(args).await)
    }

    #[tool(description = "CEPS85 set_approval_for_all")]
    async fn ceps85_set_approval_for_all(
        &self,
        Parameters(args): Parameters<Ceps85SetApprovalForAllArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::set_approval_for_all(args).await)
    }

    #[tool(description = "CEPS85 set_uri")]
    async fn ceps85_set_uri(
        &self,
        Parameters(args): Parameters<Ceps85SetUriArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::set_uri(args).await)
    }

    #[tool(description = "CEPS85 set_total_supply_of")]
    async fn ceps85_set_total_supply_of(
        &self,
        Parameters(args): Parameters<Ceps85SetTotalSupplyOfArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::set_total_supply_of(args).await)
    }

    #[tool(description = "CEPS85 set_total_supply_of_batch")]
    async fn ceps85_set_total_supply_of_batch(
        &self,
        Parameters(args): Parameters<Ceps85SetTotalSupplyOfBatchArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::set_total_supply_of_batch(args).await)
    }

    #[tool(description = "CEPS85 change_security")]
    async fn ceps85_change_security(
        &self,
        Parameters(args): Parameters<Ceps85ChangeSecurityArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::change_security(args).await)
    }

    #[tool(description = "CEPS85 set_modalities")]
    async fn ceps85_set_modalities(
        &self,
        Parameters(args): Parameters<Ceps85SetModalitiesArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::set_modalities(args).await)
    }

    #[tool(description = "CEPS85 collection_name")]
    async fn ceps85_collection_name(
        &self,
        Parameters(Ceps85CollectionNameArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps85CollectionNameArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::collection_name(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS85 collection_uri")]
    async fn ceps85_collection_uri(
        &self,
        Parameters(Ceps85CollectionUriArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps85CollectionUriArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::collection_uri(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS85 balance_of")]
    async fn ceps85_balance_of(
        &self,
        Parameters(Ceps85BalanceOfArgs {
            contract_hash,
            package_hash,
            account,
            id,
        }): Parameters<Ceps85BalanceOfArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::balance_of(contract_hash, package_hash, account, id).await)
    }

    #[tool(description = "CEPS85 is_approved_for_all")]
    async fn ceps85_is_approved_for_all(
        &self,
        Parameters(Ceps85IsApprovedForAllArgs {
            contract_hash,
            package_hash,
            owner,
            operator,
        }): Parameters<Ceps85IsApprovedForAllArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::is_approved_for_all(contract_hash, package_hash, owner, operator).await)
    }

    #[tool(description = "CEPS85 supply_of")]
    async fn ceps85_supply_of(
        &self,
        Parameters(Ceps85SupplyOfArgs {
            contract_hash,
            package_hash,
            id,
        }): Parameters<Ceps85SupplyOfArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::supply_of(contract_hash, package_hash, id).await)
    }

    #[tool(description = "CEPS85 total_supply_of")]
    async fn ceps85_total_supply_of(
        &self,
        Parameters(Ceps85TotalSupplyOfArgs {
            contract_hash,
            package_hash,
            id,
        }): Parameters<Ceps85TotalSupplyOfArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::total_supply_of(contract_hash, package_hash, id).await)
    }

    #[tool(description = "CEPS85 uri")]
    async fn ceps85_uri(
        &self,
        Parameters(Ceps85UriArgs {
            contract_hash,
            package_hash,
            id,
        }): Parameters<Ceps85UriArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::uri(contract_hash, package_hash, id).await)
    }

    #[tool(description = "CEPS85 is_non_fungible")]
    async fn ceps85_is_non_fungible(
        &self,
        Parameters(Ceps85IsNonFungibleArgs {
            contract_hash,
            package_hash,
            id,
        }): Parameters<Ceps85IsNonFungibleArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::is_non_fungible(contract_hash, package_hash, id).await)
    }

    #[tool(description = "CEPS85 balance_of_batch")]
    async fn ceps85_balance_of_batch(
        &self,
        Parameters(Ceps85BalanceOfBatchArgs {
            contract_hash,
            package_hash,
            accounts,
            ids,
        }): Parameters<Ceps85BalanceOfBatchArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::balance_of_batch(contract_hash, package_hash, accounts, ids).await)
    }

    #[tool(description = "CEPS85 supply_of_batch")]
    async fn ceps85_supply_of_batch(
        &self,
        Parameters(Ceps85SupplyOfBatchArgs {
            contract_hash,
            package_hash,
            ids,
        }): Parameters<Ceps85SupplyOfBatchArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::supply_of_batch(contract_hash, package_hash, ids).await)
    }

    #[tool(description = "CEPS85 total_supply_of_batch")]
    async fn ceps85_total_supply_of_batch(
        &self,
        Parameters(Ceps85TotalSupplyOfBatchArgs {
            contract_hash,
            package_hash,
            ids,
        }): Parameters<Ceps85TotalSupplyOfBatchArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::total_supply_of_batch(contract_hash, package_hash, ids).await)
    }

    #[tool(description = "CEPS85 total_fungible_supply")]
    async fn ceps85_total_fungible_supply(
        &self,
        Parameters(Ceps85TotalFungibleSupplyArgs {
            contract_hash,
            package_hash,
            id,
        }): Parameters<Ceps85TotalFungibleSupplyArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::total_fungible_supply(contract_hash, package_hash, id).await)
    }

    #[tool(description = "CEPS85 enable_burn")]
    async fn ceps85_enable_burn(
        &self,
        Parameters(Ceps85EnableBurnArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps85EnableBurnArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::enable_burn(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS85 events_mode")]
    async fn ceps85_events_mode(
        &self,
        Parameters(Ceps85EventsModeArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps85EventsModeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::events_mode(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS85 number_of_minted_tokens")]
    async fn ceps85_number_of_minted_tokens(
        &self,
        Parameters(Ceps85NumberOfMintedTokensArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps85NumberOfMintedTokensArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::number_of_minted_tokens(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS85 transfer_filter_contract")]
    async fn ceps85_transfer_filter_contract(
        &self,
        Parameters(Ceps85TransferFilterContractArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps85TransferFilterContractArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::transfer_filter_contract(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS85 transfer_filter_method")]
    async fn ceps85_transfer_filter_method(
        &self,
        Parameters(Ceps85TransferFilterMethodArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps85TransferFilterMethodArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::transfer_filter_method(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS85 security_badge")]
    async fn ceps85_security_badge(
        &self,
        Parameters(Ceps85SecurityBadgeArgs {
            contract_hash,
            package_hash,
            entity,
        }): Parameters<Ceps85SecurityBadgeArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep85::security_badge(contract_hash, package_hash, entity).await)
    }

    #[tool(description = "CEPS95 Odra install")]
    async fn ceps95_install(
        &self,
        Parameters(args): Parameters<Ceps95InstallArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::install(args).await)
    }

    #[tool(description = "CEPS95 bind Odra install (package named key → contract+package)")]
    async fn ceps95_bind_odra_install(
        &self,
        Parameters(Ceps95BindOdraInstallArgs {
            installer_public_key,
            package_hash_key_name,
        }): Parameters<Ceps95BindOdraInstallArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::bind_odra_install(installer_public_key, package_hash_key_name).await)
    }

    #[tool(description = "CEPS95 mint")]
    async fn ceps95_mint(
        &self,
        Parameters(args): Parameters<Ceps95MintArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::mint(args).await)
    }

    #[tool(description = "CEPS95 burn")]
    async fn ceps95_burn(
        &self,
        Parameters(args): Parameters<Ceps95BurnArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::burn(args).await)
    }

    #[tool(description = "CEPS95 transfer_from")]
    async fn ceps95_transfer_from(
        &self,
        Parameters(args): Parameters<Ceps95TransferFromArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::transfer_from(args).await)
    }

    #[tool(description = "CEPS95 safe_transfer_from")]
    async fn ceps95_safe_transfer_from(
        &self,
        Parameters(args): Parameters<Ceps95SafeTransferFromArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::safe_transfer_from(args).await)
    }

    #[tool(description = "CEPS95 approve")]
    async fn ceps95_approve(
        &self,
        Parameters(args): Parameters<Ceps95ApproveArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::approve(args).await)
    }

    #[tool(description = "CEPS95 revoke_approval")]
    async fn ceps95_revoke_approval(
        &self,
        Parameters(args): Parameters<Ceps95RevokeApprovalArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::revoke_approval(args).await)
    }

    #[tool(description = "CEPS95 approve_for_all")]
    async fn ceps95_approve_for_all(
        &self,
        Parameters(args): Parameters<Ceps95ApproveForAllArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::approve_for_all(args).await)
    }

    #[tool(description = "CEPS95 revoke_approval_for_all")]
    async fn ceps95_revoke_approval_for_all(
        &self,
        Parameters(args): Parameters<Ceps95RevokeApprovalForAllArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::revoke_approval_for_all(args).await)
    }

    #[tool(description = "CEPS95 name")]
    async fn ceps95_name(
        &self,
        Parameters(Ceps95NameArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps95NameArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::name(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS95 symbol")]
    async fn ceps95_symbol(
        &self,
        Parameters(Ceps95SymbolArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps95SymbolArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::symbol(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS95 total_supply")]
    async fn ceps95_total_supply(
        &self,
        Parameters(Ceps95TotalSupplyArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps95TotalSupplyArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::total_supply(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS95 balance_of")]
    async fn ceps95_balance_of(
        &self,
        Parameters(Ceps95BalanceOfArgs {
            contract_hash,
            package_hash,
            owner,
        }): Parameters<Ceps95BalanceOfArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::balance_of(contract_hash, package_hash, owner).await)
    }

    #[tool(description = "CEPS95 owner_of")]
    async fn ceps95_owner_of(
        &self,
        Parameters(Ceps95OwnerOfArgs {
            contract_hash,
            package_hash,
            token_id,
        }): Parameters<Ceps95OwnerOfArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::owner_of(contract_hash, package_hash, token_id).await)
    }

    #[tool(description = "CEPS95 get_approved")]
    async fn ceps95_get_approved(
        &self,
        Parameters(Ceps95GetApprovedArgs {
            contract_hash,
            package_hash,
            token_id,
        }): Parameters<Ceps95GetApprovedArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::get_approved(contract_hash, package_hash, token_id).await)
    }

    #[tool(description = "CEPS95 is_approved_for_all")]
    async fn ceps95_is_approved_for_all(
        &self,
        Parameters(Ceps95IsApprovedForAllArgs {
            contract_hash,
            package_hash,
            owner,
            operator,
        }): Parameters<Ceps95IsApprovedForAllArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::is_approved_for_all(contract_hash, package_hash, owner, operator).await)
    }

    #[tool(description = "CEPS95 token_metadata")]
    async fn ceps95_token_metadata(
        &self,
        Parameters(Ceps95TokenMetadataArgs {
            contract_hash,
            package_hash,
            token_id,
        }): Parameters<Ceps95TokenMetadataArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::token_metadata(contract_hash, package_hash, token_id).await)
    }

    #[tool(description = "CEPS95 get_owner")]
    async fn ceps95_get_owner(
        &self,
        Parameters(Ceps95GetOwnerArgs {
            contract_hash,
            package_hash,
        }): Parameters<Ceps95GetOwnerArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::get_owner(contract_hash, package_hash).await)
    }

    #[tool(description = "CEPS95 transfer_ownership")]
    async fn ceps95_transfer_ownership(
        &self,
        Parameters(args): Parameters<Ceps95TransferOwnershipArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::cep95::transfer_ownership(args).await)
    }
}

/// Serves MCP over stdio until the client disconnects.
pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let server = CepsClientMcp;
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}

/// Serves MCP over Streamable HTTP until the process is stopped.
pub async fn run_http(addr: &str) -> std::io::Result<()> {
    let config =
        rmcp::transport::streamable_http_server::tower::StreamableHttpServerConfig::default();
    let service = rmcp::transport::streamable_http_server::tower::StreamableHttpService::new(
        || Ok(CepsClientMcp),
        Arc::new(
            rmcp::transport::streamable_http_server::session::local::LocalSessionManager::default(),
        ),
        config,
    );
    let method_router = axum::routing::any_service(service);
    let app = axum::Router::new()
        .route("/mcp", method_router.clone())
        .route("/mcp/", method_router);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "ceps-rust-ts-client-mcp HTTP listening");
    axum::serve(listener, app).await?;
    Ok(())
}

#[tool_handler]
impl ServerHandler for CepsClientMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(rmcp::model::Implementation::new(
                "ceps-rust-ts-client-mcp",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions("MCP tools for ceps-rust-ts-client: CEP-18/78/85/95 install/query/mutate plus wasm helpers. Set CEPS_RPC_URL / CEPS_SSE_URL / CEPS_WASM_ROOT.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mcp_server_version_matches_crate() {
        let info = CepsClientMcp.get_info();
        assert_eq!(info.server_info.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(info.server_info.name.as_str(), "ceps-rust-ts-client-mcp");
    }
}
