//! JSON-schema parameter structs for rmcp `Parameters<T>` tool handlers.

use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct CepsSetEndpointsArgs {
    #[serde(default)]
    pub rpc_url: Option<String>,
    #[serde(default)]
    pub sse_url: Option<String>,
    #[serde(default)]
    pub chain_name: Option<String>,
    #[serde(default)]
    pub verbosity: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CepsReadContractWasmArgs {
    pub path: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18InstallArgs {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wasm_path: Option<String>,
    #[serde(default)]
    pub wasm_base64: Option<String>,
    #[serde(default)]
    pub events_mode: Option<u8>,
    #[serde(default)]
    pub enable_mint_and_burn: Option<bool>,
    #[serde(default)]
    pub admin_list: Option<Vec<String>>,
    #[serde(default)]
    pub minter_list: Option<Vec<String>>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18UpgradeArgs {
    pub name: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wasm_path: Option<String>,
    #[serde(default)]
    pub wasm_base64: Option<String>,
    #[serde(default)]
    pub events_mode: Option<u8>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18TransferArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub recipient: String,
    pub amount: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18TransferFromArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub owner: String,
    pub recipient: String,
    pub amount: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18ApproveArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub spender: String,
    pub amount: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18IncreaseAllowanceArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub spender: String,
    pub amount: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18DecreaseAllowanceArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub spender: String,
    pub amount: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18MintArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub owner: String,
    pub amount: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18BurnArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub owner: String,
    pub amount: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18ChangeSecurityArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub admin_list: Option<Vec<String>>,
    #[serde(default)]
    pub minter_list: Option<Vec<String>>,
    #[serde(default)]
    pub none_list: Option<Vec<String>>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18ChangeEventsModeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub events_mode: u8,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18NameArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18SymbolArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18DecimalsArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18TotalSupplyArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18EventsModeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18IsMintAndBurnEnabledArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18BalanceOfArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub account: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18AllowancesArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub owner: String,
    pub spender: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps18SecurityBadgeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub account: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78InstallArgs {
    pub collection_name: String,
    pub collection_symbol: String,
    pub total_token_supply: u64,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wasm_path: Option<String>,
    #[serde(default)]
    pub wasm_base64: Option<String>,
    #[serde(default)]
    pub events_mode: Option<u8>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78UpgradeArgs {
    pub collection_name: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wasm_path: Option<String>,
    #[serde(default)]
    pub wasm_base64: Option<String>,
    #[serde(default)]
    pub total_token_supply: Option<u64>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78MintArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub token_owner: String,
    pub token_meta_data: String,
    #[serde(default)]
    pub token_hash: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78MintSessionArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub token_owner: String,
    pub token_meta_data: String,
    #[serde(default)]
    pub token_hash: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub session_wasm_path: Option<String>,
    #[serde(default)]
    pub session_wasm_base64: Option<String>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78BurnArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    #[serde(default)]
    pub token_id: Option<u64>,
    #[serde(default)]
    pub token_hash: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78TransferArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub token_id: Option<u64>,
    #[serde(default)]
    pub token_hash: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78TransferSessionArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub token_id: Option<u64>,
    #[serde(default)]
    pub token_hash: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub session_wasm_path: Option<String>,
    #[serde(default)]
    pub session_wasm_base64: Option<String>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78RegisterOwnerArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub token_owner: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78ApproveArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub operator: String,
    #[serde(default)]
    pub token_id: Option<u64>,
    #[serde(default)]
    pub token_hash: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78RevokeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub operator: String,
    #[serde(default)]
    pub token_id: Option<u64>,
    #[serde(default)]
    pub token_hash: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78SetApprovalForAllArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub operator: String,
    pub approve_all: bool,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78SetTokenMetadataArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub token_meta_data: String,
    #[serde(default)]
    pub token_id: Option<u64>,
    #[serde(default)]
    pub token_hash: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78SetVariablesArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub allow_minting: Option<bool>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78UpdatedReceiptsArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub session_wasm_path: Option<String>,
    #[serde(default)]
    pub session_wasm_base64: Option<String>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78CollectionNameArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78CollectionSymbolArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78TotalTokenSupplyArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78NumberOfMintedTokensArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78EventsModeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78AllowMintingArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78MintingModeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78WhitelistModeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78ReportingModeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78BurnModeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78OperatorBurnModeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78HolderModeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78IdentifierModeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78MetadataMutabilityArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78NftKindArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78NftMetadataKindArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78OwnershipModeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78PackageOperatorModeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78AclPackageModeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78JsonSchemaArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78IsAclWhitelistedArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub entity: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78OwnerOfSessionArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    #[serde(default)]
    pub token_id: Option<u64>,
    #[serde(default)]
    pub token_hash: Option<String>,
    pub key_name: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub session_wasm_path: Option<String>,
    #[serde(default)]
    pub session_wasm_base64: Option<String>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78BalanceOfSessionArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub token_owner: String,
    pub key_name: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub session_wasm_path: Option<String>,
    #[serde(default)]
    pub session_wasm_base64: Option<String>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78GetApprovedSessionArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    #[serde(default)]
    pub token_id: Option<u64>,
    #[serde(default)]
    pub token_hash: Option<String>,
    pub key_name: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub session_wasm_path: Option<String>,
    #[serde(default)]
    pub session_wasm_base64: Option<String>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78IsApprovedForAllSessionArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub token_owner: String,
    pub operator: String,
    pub key_name: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub session_wasm_path: Option<String>,
    #[serde(default)]
    pub session_wasm_base64: Option<String>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CepsPutTransactionArgs {
    pub transaction_json: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub contract_hash: Option<String>,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CepsWaitTransactionArgs {
    pub transaction_hash: String,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CepsCesParseExecutionArgs {
    pub contract_hash: String,
    pub execution_result_json: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CepsCesParseTransactionArgs {
    pub contract_hash: String,
    pub transaction_hash: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CepsCesCollectArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    #[serde(default)]
    pub event_names: Option<Vec<String>>,
    #[serde(default)]
    pub max_transactions: Option<u64>,
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78OwnerOfArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    #[serde(default)]
    pub token_id: Option<u64>,
    #[serde(default)]
    pub token_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78BalanceOfArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub owner: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78GetApprovedArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    #[serde(default)]
    pub token_id: Option<u64>,
    #[serde(default)]
    pub token_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78IsApprovedForAllArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub owner: String,
    pub operator: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps78MetadataArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    #[serde(default)]
    pub token_id: Option<u64>,
    #[serde(default)]
    pub token_hash: Option<String>,
    pub nft_metadata_kind: u8,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85InstallArgs {
    pub name: String,
    pub uri: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wasm_path: Option<String>,
    #[serde(default)]
    pub wasm_base64: Option<String>,
    #[serde(default)]
    pub events_mode: Option<u8>,
    #[serde(default)]
    pub enable_burn: Option<bool>,
    #[serde(default)]
    pub admin_list: Option<Vec<String>>,
    #[serde(default)]
    pub minter_list: Option<Vec<String>>,
    #[serde(default)]
    pub burner_list: Option<Vec<String>>,
    #[serde(default)]
    pub meta_list: Option<Vec<String>>,
    #[serde(default)]
    pub transfer_filter_contract: Option<String>,
    #[serde(default)]
    pub transfer_filter_method: Option<String>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85UpgradeArgs {
    pub name: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wasm_path: Option<String>,
    #[serde(default)]
    pub wasm_base64: Option<String>,
    #[serde(default)]
    pub transfer_filter_contract: Option<String>,
    #[serde(default)]
    pub transfer_filter_method: Option<String>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85MintArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub recipient: String,
    pub id: String,
    pub amount: String,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85BatchMintArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub recipient: String,
    pub ids: Vec<String>,
    pub amounts: Vec<String>,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85BurnArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub owner: String,
    pub id: String,
    pub amount: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85BatchBurnArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub owner: String,
    pub ids: Vec<String>,
    pub amounts: Vec<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85TransferArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub from: String,
    pub to: String,
    pub id: String,
    pub amount: String,
    #[serde(default)]
    pub data_hex: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85BatchTransferArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub from: String,
    pub to: String,
    pub ids: Vec<String>,
    pub amounts: Vec<String>,
    #[serde(default)]
    pub data_hex: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85SetApprovalForAllArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub operator: String,
    pub approved: bool,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85SetUriArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub uri: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85SetTotalSupplyOfArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub id: String,
    pub total_supply: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85SetTotalSupplyOfBatchArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub ids: Vec<String>,
    pub total_supplies: Vec<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85ChangeSecurityArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub admin_list: Option<Vec<String>>,
    #[serde(default)]
    pub minter_list: Option<Vec<String>>,
    #[serde(default)]
    pub burner_list: Option<Vec<String>>,
    #[serde(default)]
    pub meta_list: Option<Vec<String>>,
    #[serde(default)]
    pub none_list: Option<Vec<String>>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85SetModalitiesArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub enable_burn: Option<bool>,
    #[serde(default)]
    pub events_mode: Option<u8>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85CollectionNameArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85CollectionUriArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85BalanceOfArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub account: String,
    pub id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85IsApprovedForAllArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub owner: String,
    pub operator: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85SupplyOfArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85TotalSupplyOfArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85UriArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85IsNonFungibleArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85BalanceOfBatchArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub accounts: Vec<String>,
    pub ids: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85SupplyOfBatchArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub ids: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85TotalSupplyOfBatchArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub ids: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85TotalFungibleSupplyArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85EnableBurnArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85EventsModeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85NumberOfMintedTokensArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85TransferFilterContractArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85TransferFilterMethodArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps85SecurityBadgeArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub entity: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95InstallArgs {
    pub name: String,
    pub symbol: String,
    pub package_hash_key_name: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wasm_path: Option<String>,
    #[serde(default)]
    pub wasm_base64: Option<String>,
    #[serde(default)]
    pub allow_key_override: Option<bool>,
    #[serde(default)]
    pub is_upgradable: Option<bool>,
    #[serde(default)]
    pub is_upgrade: Option<bool>,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95BindOdraInstallArgs {
    pub installer_public_key: String,
    pub package_hash_key_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95MintArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub to: String,
    pub token_id: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95BurnArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub token_id: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95TransferFromArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub from: String,
    pub to: String,
    pub token_id: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95SafeTransferFromArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub from: String,
    pub to: String,
    pub token_id: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95ApproveArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub spender: String,
    pub token_id: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95RevokeApprovalArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub token_id: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95ApproveForAllArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub operator: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95RevokeApprovalForAllArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub operator: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95NameArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95SymbolArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95TotalSupplyArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95BalanceOfArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub owner: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95OwnerOfArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub token_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95GetApprovedArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub token_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95IsApprovedForAllArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub owner: String,
    pub operator: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95TokenMetadataArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub token_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95GetOwnerArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Ceps95TransferOwnershipArgs {
    pub contract_hash: String,
    #[serde(default)]
    pub package_hash: Option<String>,
    pub new_owner: String,
    #[serde(default)]
    pub secret_key_pem: Option<String>,
    pub payment_amount: String,
    #[serde(default)]
    pub wait: Option<bool>,
    #[serde(default)]
    pub wait_timeout_ms: Option<u64>,
    #[serde(default)]
    pub make_only: Option<bool>,
    #[serde(default)]
    pub initiator_addr: Option<String>,
}
