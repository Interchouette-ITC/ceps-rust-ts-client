//! `ceps-client-cli` - clap wrapper over `ceps-client`.

use anyhow::{bail, Context, Result};
use casper_rust_wasm_sdk::types::verbosity::Verbosity;
use ceps_client::cep18::InstallArgs as CEP18InstallArgs;
use ceps_client::cep78::{InstallArgs as CEP78InstallArgs, TokenIdentifier};
use ceps_client::cep85::InstallArgs as CEP85InstallArgs;
use ceps_client::cep95::InstallArgs as CEP95InstallArgs;
use ceps_client::{
    CEP18Client, CEP78Client, CEP85Client, CEP95Client, CEPClient, EventsMode, EventsMode78,
    TransactionParams,
};
use clap::{Parser, Subcommand, ValueEnum};
use serde_json::Value;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(
    name = "ceps-client-cli",
    version,
    about = "Unified Casper CEP client (18 / 78 / 85 / 95)"
)]
struct Cli {
    /// JSON-RPC URL (NCTL default: http://127.0.0.1:11101).
    #[arg(long, env = "CEPS_RPC_URL", default_value = "http://127.0.0.1:11101")]
    rpc_url: String,

    /// SSE events URL (NCTL default: http://127.0.0.1:18101/events).
    #[arg(
        long,
        env = "CEPS_SSE_URL",
        default_value = "http://127.0.0.1:18101/events"
    )]
    sse_url: String,

    /// Chain name (NCTL default: casper-net-1).
    #[arg(long, env = "CEPS_CHAIN_NAME", default_value = "casper-net-1")]
    chain_name: String,

    /// SDK verbosity.
    #[arg(long, env = "CEPS_VERBOSITY", value_enum, default_value = "low")]
    verbosity: VerbosityArg,

    /// Emit JSON on stdout where applicable.
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum VerbosityArg {
    Low,
    Medium,
    High,
}

impl From<VerbosityArg> for Verbosity {
    fn from(value: VerbosityArg) -> Self {
        match value {
            VerbosityArg::Low => Verbosity::Low,
            VerbosityArg::Medium => Verbosity::Medium,
            VerbosityArg::High => Verbosity::High,
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Show configured network endpoints.
    Status,
    /// Put a signed Transaction JSON (`CEPClient::put_transaction`).
    #[command(name = "put-transaction")]
    PutTransaction {
        /// Path to Transaction JSON (`-` = stdin). Typically `CallResult.transaction` after make-only + external sign.
        #[arg(long)]
        transaction_file: PathBuf,
        /// Skip SSE wait after put (default is wait).
        #[arg(long, default_value_t = false)]
        no_wait: bool,
        #[arg(long)]
        wait_timeout_ms: Option<u64>,
        /// Optional bind for CES soft-attach when waiting.
        #[arg(long)]
        contract_hash: Option<String>,
        #[arg(long)]
        package_hash: Option<String>,
    },
    /// Wait for a transaction hash on SSE (`CEPClient::wait_transaction`).
    #[command(name = "wait-transaction")]
    WaitTransaction {
        #[arg(long)]
        transaction_hash: String,
        #[arg(long)]
        wait_timeout_ms: Option<u64>,
    },
    /// CEP-18 fungible token commands.
    #[command(name = "cep18")]
    CEP18 {
        #[command(subcommand)]
        command: CEP18Commands,
    },
    /// CEP-78 NFT commands.
    #[command(name = "cep78")]
    CEP78 {
        #[command(subcommand)]
        command: CEP78Commands,
    },
    /// CEP-85 multi-token commands.
    #[command(name = "cep85")]
    CEP85 {
        #[command(subcommand)]
        command: CEP85Commands,
    },
    /// CEP-95 NFT commands (supported simpler API).
    #[command(name = "cep95")]
    CEP95 {
        #[command(subcommand)]
        command: CEP95Commands,
    },
    /// CES helpers (`parse_ces_*` / `collect_ces_events`).
    #[command(name = "ces")]
    #[allow(clippy::upper_case_acronyms)]
    CES {
        #[command(subcommand)]
        command: CESCommands,
    },
}

#[derive(Debug, Subcommand)]
enum CEP18Commands {
    /// Print client endpoint configuration.
    Info,
    /// Install CEP-18 WASM (`--wasm`, `--secret-key` or `--make-only`).
    Install {
        #[arg(long)]
        name: String,
        #[arg(long)]
        symbol: String,
        #[arg(long, default_value_t = 9)]
        decimals: u8,
        #[arg(long)]
        total_supply: String,
        #[arg(long)]
        wasm: PathBuf,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "400000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
        /// Events mode u8 (0=NoEvents, 1=CES, …).
        #[arg(long)]
        events_mode: Option<u8>,
        #[arg(long, default_value_t = false)]
        enable_mint_and_burn: bool,
    },
    /// Transfer tokens.
    Transfer {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        recipient: String,
        #[arg(long)]
        amount: String,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Transfer from owner using allowance.
    TransferFrom {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        owner: String,
        #[arg(long)]
        recipient: String,
        #[arg(long)]
        amount: String,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Approve spender.
    Approve {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        spender: String,
        #[arg(long)]
        amount: String,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Mint tokens.
    Mint {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        owner: String,
        #[arg(long)]
        amount: String,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Burn tokens.
    Burn {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        owner: String,
        #[arg(long)]
        amount: String,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Query token name.
    Name {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
    },
    /// Query balance.
    Balance {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        account: String,
    },
    /// Query security badge for an account.
    SecurityBadge {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        account: String,
    },
}

#[derive(Debug, Subcommand)]
enum CEP78Commands {
    /// Print client endpoint configuration.
    Info,
    /// Install CEP-78 WASM (`--wasm`, `--secret-key` or `--make-only`).
    Install {
        #[arg(long)]
        name: String,
        #[arg(long)]
        symbol: String,
        #[arg(long, default_value_t = 100)]
        total_token_supply: u64,
        #[arg(long)]
        wasm: PathBuf,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "600000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
        /// Events mode u8 (0=NoEvents, 1=CEP47, 2=CES, …).
        #[arg(long)]
        events_mode: Option<u8>,
    },
    /// Mint a token.
    Mint {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        token_owner: String,
        #[arg(long, default_value = "")]
        token_meta_data: String,
        #[arg(long)]
        token_hash: Option<String>,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Burn a token (`--token-id` or `--token-hash`).
    Burn {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        token_id: Option<u64>,
        #[arg(long)]
        token_hash: Option<String>,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Transfer a token (`--token-id` or `--token-hash`).
    Transfer {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        source: String,
        #[arg(long)]
        target: String,
        #[arg(long)]
        token_id: Option<u64>,
        #[arg(long)]
        token_hash: Option<String>,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Approve an operator for one token.
    Approve {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        operator: String,
        #[arg(long)]
        token_id: Option<u64>,
        #[arg(long)]
        token_hash: Option<String>,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Query collection name.
    Name {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
    },
    /// Query balance for an account.
    Balance {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        account: String,
    },
    /// Query ownership mode (`u8` ABI).
    OwnershipMode {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
enum CESCommands {
    /// Parse CES events for a transaction against a contract hash.
    Parse {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        transaction_hash: String,
    },
    /// Parse CES events from execution-result JSON (`--execution-file`, `-` = stdin).
    #[command(name = "parse-execution")]
    ParseExecution {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        execution_file: PathBuf,
    },
    /// Collect SSE `TransactionProcessed` frames and decode CES for a bound contract.
    Collect {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        /// CES event names to keep (repeatable). Empty = keep all decoded events.
        #[arg(long = "event-name")]
        event_names: Vec<String>,
        #[arg(long, default_value_t = 8)]
        max_transactions: usize,
        #[arg(long, default_value_t = 120_000)]
        timeout_ms: u64,
    },
}

#[derive(Debug, Subcommand)]
enum CEP85Commands {
    /// Print client endpoint configuration.
    Info,
    /// Install CEP-85 WASM (`--wasm`, `--secret-key` or `--make-only`).
    Install {
        #[arg(long)]
        name: String,
        #[arg(long)]
        uri: String,
        #[arg(long)]
        wasm: PathBuf,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "550000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
        #[arg(long)]
        events_mode: Option<u8>,
        #[arg(long, default_value_t = false)]
        enable_burn: bool,
        #[arg(long)]
        transfer_filter_contract: Option<String>,
        #[arg(long)]
        transfer_filter_method: Option<String>,
    },
    /// Mint one token id.
    Mint {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        recipient: String,
        #[arg(long)]
        id: String,
        #[arg(long)]
        amount: String,
        #[arg(long)]
        uri: Option<String>,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Burn one token id.
    Burn {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        owner: String,
        #[arg(long)]
        id: String,
        #[arg(long)]
        amount: String,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Transfer (`transfer_from` on-chain).
    Transfer {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        id: String,
        #[arg(long)]
        amount: String,
        /// Optional transfer data as hex (optional `0x` prefix).
        #[arg(long)]
        data_hex: Option<String>,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Set approval for all.
    SetApprovalForAll {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        operator: String,
        #[arg(long)]
        approved: bool,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Query collection name (requires `--contract-hash`).
    Name {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
    },
    /// Query balance for an account and token id.
    Balance {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        account: String,
        #[arg(long)]
        id: String,
    },
    /// Batch balances (`--accounts` and `--ids` comma-separated, same length).
    BalanceOfBatch {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        accounts: String,
        #[arg(long)]
        ids: String,
    },
    /// Batch circulating supplies (`--ids` comma-separated).
    SupplyOfBatch {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        ids: String,
    },
    /// Batch total supply caps (`--ids` comma-separated).
    TotalSupplyOfBatch {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        ids: String,
    },
    /// Remaining fungible mintable amount for an id.
    TotalFungibleSupply {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        id: String,
    },
    /// Whether burn is enabled.
    EnableBurn {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
    },
    /// Events mode.
    EventsMode {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
    },
    /// Number of minted token ids.
    NumberOfMintedTokens {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
    },
    /// Transfer-filter contract key when set.
    TransferFilterContract {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
    },
    /// Transfer-filter method when set.
    TransferFilterMethod {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
    },
    /// Security badge for an entity.
    SecurityBadge {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        entity: String,
    },
}

#[derive(Debug, Subcommand)]
enum CEP95Commands {
    /// Print client endpoint configuration.
    Info,
    /// Install Odra OwnedCEP95 WASM (`--wasm`, `--secret-key` or `--make-only`).
    Install {
        #[arg(long)]
        name: String,
        #[arg(long)]
        symbol: String,
        #[arg(long)]
        package_key_name: String,
        #[arg(long)]
        wasm: PathBuf,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "600000000000")]
        payment: String,
        /// Build Transaction JSON only (no put).
        #[arg(long, default_value_t = false)]
        make_only: bool,
        /// Public key hex required for unsigned `--make-only`.
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Mint a token (owner-gated on tip).
    Mint {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        to: String,
        #[arg(long)]
        token_id: String,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Burn a token.
    Burn {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        token_id: String,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// `transfer_from`.
    TransferFrom {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        token_id: String,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Approve a spender for one token.
    Approve {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        spender: String,
        #[arg(long)]
        token_id: String,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Revoke single-token approval.
    RevokeApproval {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        token_id: String,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Approve operator for all tokens.
    ApproveForAll {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        operator: String,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Revoke operator for all tokens.
    RevokeApprovalForAll {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        operator: String,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
    /// Query collection name.
    Name {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
    },
    /// Query collection symbol.
    Symbol {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
    },
    /// Query balance for an account.
    Balance {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        account: String,
    },
    /// Query owner of a token id.
    OwnerOf {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        token_id: String,
    },
    /// Query approved spender for a token id.
    GetApproved {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        token_id: String,
    },
    /// Query operator approval.
    IsApprovedForAll {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        owner: String,
        #[arg(long)]
        operator: String,
    },
    /// Contract Ownable owner.
    GetOwner {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
    },
    /// Transfer Ownable ownership.
    TransferOwnership {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        package_hash: Option<String>,
        #[arg(long)]
        new_owner: String,
        #[arg(long)]
        secret_key: Option<PathBuf>,
        #[arg(long, default_value = "5000000000")]
        payment: String,
        #[arg(long, default_value_t = false)]
        make_only: bool,
        #[arg(long)]
        initiator_addr: Option<String>,
    },
}

fn read_secret(path: &PathBuf) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("read secret key {}", path.display()))
}

fn shared_client(
    rpc_url: &str,
    sse: Option<String>,
    chain: Option<String>,
    verbosity: Option<Verbosity>,
) -> Result<CEPClient> {
    CEPClient::new(rpc_url, sse, chain, verbosity).context("create CEPClient")
}

fn normalize_contract_hash_key(contract_hash: &str) -> String {
    let t = contract_hash.trim();
    if t.starts_with("hash-") || t.starts_with("entity-") {
        t.to_string()
    } else {
        format!("hash-{t}")
    }
}

fn read_json_value(path: &Path) -> Result<Value> {
    let raw = if path.as_os_str() == "-" {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .context("read stdin JSON")?;
        buf
    } else {
        fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?
    };
    serde_json::from_str(&raw).context("parse JSON")
}

fn build_tx_params(
    secret_key: &Option<PathBuf>,
    payment: &str,
    make_only: bool,
    initiator_addr: &Option<String>,
) -> Result<TransactionParams> {
    let mut tx = match secret_key {
        Some(path) => TransactionParams::new(read_secret(path)?, payment),
        None => TransactionParams::for_make(payment),
    };
    if make_only {
        tx = tx.make_only();
    }
    if let Some(addr) = initiator_addr.as_ref().filter(|s| !s.trim().is_empty()) {
        tx = tx.with_initiator_addr(addr);
    }
    tx.validate().map_err(|e| anyhow::anyhow!(e))?;
    Ok(tx)
}

fn print_tx(hash: &str, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::json!({ "transactionHash": hash }));
    } else {
        println!("{hash}");
    }
    Ok(())
}

fn print_call_result(result: &ceps_client::CallResult, json: bool) -> Result<()> {
    if json || result.transaction.is_some() {
        println!("{}", serde_json::to_string_pretty(result)?);
    } else {
        print_tx(&result.transaction_hash, json)?;
    }
    Ok(())
}

fn print_install_put(
    put: &ceps_client::CallResult,
    contract: &str,
    package: &str,
    json: bool,
) -> Result<()> {
    if json {
        println!(
            "{}",
            serde_json::json!({
                "transactionHash": put.transaction_hash,
                "contractHash": contract,
                "packageHash": package,
                "transaction": put.transaction,
            })
        );
    } else {
        println!("tx={}", put.transaction_hash);
        println!("contract={contract}");
        println!("package={package}");
    }
    Ok(())
}

fn cep78_token(token_id: Option<u64>, token_hash: Option<String>) -> Result<TokenIdentifier> {
    match (token_id, token_hash) {
        (Some(id), None) => Ok(TokenIdentifier::id(id)),
        (None, Some(hash)) => Ok(TokenIdentifier::hash(hash)),
        _ => bail!("provide exactly one of --token-id or --token-hash"),
    }
}

fn events_mode18(v: Option<u8>) -> Result<Option<EventsMode>> {
    match v {
        None => Ok(None),
        Some(n) => EventsMode::from_u8(n)
            .map(Some)
            .ok_or_else(|| anyhow::anyhow!("invalid CEP-18/85 events_mode {n}")),
    }
}

fn events_mode78(v: Option<u8>) -> Result<Option<EventsMode78>> {
    match v {
        None => Ok(None),
        Some(n) => EventsMode78::from_u8(n)
            .map(Some)
            .ok_or_else(|| anyhow::anyhow!("invalid CEP-78 events_mode {n}")),
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();
    let verbosity = Some(cli.verbosity.into());
    let sse = Some(cli.sse_url.clone());
    let chain = Some(cli.chain_name.clone());

    match cli.command {
        Commands::Status => {
            if cli.json {
                println!(
                    "{}",
                    serde_json::json!({
                        "rpc_url": format_rpc(&cli.rpc_url),
                        "sse_url": cli.sse_url,
                        "chain_name": cli.chain_name,
                        "verbosity": format!("{:?}", Verbosity::from(cli.verbosity)),
                    })
                );
            } else {
                println!("rpc_url     {}", format_rpc(&cli.rpc_url));
                println!("sse_url     {}", cli.sse_url);
                println!("chain_name  {}", cli.chain_name);
                println!("verbosity   {:?}", Verbosity::from(cli.verbosity));
            }
        }
        Commands::PutTransaction {
            transaction_file,
            no_wait,
            wait_timeout_ms,
            contract_hash,
            package_hash,
        } => {
            let mut client = shared_client(&cli.rpc_url, sse, chain, verbosity)?;
            if let Some(h) = contract_hash.as_ref().filter(|s| !s.trim().is_empty()) {
                client
                    .set_contract_hash(h, package_hash.as_deref())
                    .context("set_contract_hash")?;
            }
            let tx_json = read_json_value(&transaction_file).context("read transaction JSON")?;
            let result = client
                .put_transaction(&tx_json, !no_wait, wait_timeout_ms)
                .await
                .context("put_transaction")?;
            print_call_result(&result, cli.json)?;
        }
        Commands::WaitTransaction {
            transaction_hash,
            wait_timeout_ms,
        } => {
            let client = shared_client(&cli.rpc_url, sse, chain, verbosity)?;
            let event = client
                .wait_transaction(&transaction_hash, wait_timeout_ms)
                .await
                .context("wait_transaction")?;
            println!("{}", serde_json::to_string_pretty(&event)?);
        }
        Commands::CEP18 { command } => {
            run_cep18(&cli.rpc_url, sse, chain, verbosity, cli.json, command).await?
        }
        Commands::CEP78 { command } => {
            run_cep78(&cli.rpc_url, sse, chain, verbosity, cli.json, command).await?
        }
        Commands::CES { command } => match command {
            CESCommands::Parse {
                contract_hash,
                transaction_hash,
            } => {
                let client = shared_client(&cli.rpc_url, sse, chain, verbosity)?;
                let hash = normalize_contract_hash_key(&contract_hash);
                let rows = client
                    .parse_ces_transaction(&[hash], &transaction_hash)
                    .await
                    .context("parse CES")?;
                println!("{}", serde_json::to_string_pretty(&rows)?);
            }
            CESCommands::ParseExecution {
                contract_hash,
                execution_file,
            } => {
                let client = shared_client(&cli.rpc_url, sse, chain, verbosity)?;
                let hash = normalize_contract_hash_key(&contract_hash);
                let exec = read_json_value(&execution_file).context("read execution JSON")?;
                let rows = client
                    .parse_ces_execution(&[hash], &exec)
                    .await
                    .context("parse CES execution")?;
                println!("{}", serde_json::to_string_pretty(&rows)?);
            }
            CESCommands::Collect {
                contract_hash,
                package_hash,
                event_names,
                max_transactions,
                timeout_ms,
            } => {
                let mut client = shared_client(&cli.rpc_url, sse, chain, verbosity)?;
                client
                    .set_contract_hash(&contract_hash, package_hash.as_deref())
                    .context("set_contract_hash")?;
                let names: Vec<&str> = event_names.iter().map(String::as_str).collect();
                let rows = client
                    .collect_ces_events(&names, max_transactions, timeout_ms)
                    .await
                    .context("collect CES")?;
                println!("{}", serde_json::to_string_pretty(&rows)?);
            }
        },
        Commands::CEP85 { command } => {
            run_cep85(&cli.rpc_url, sse, chain, verbosity, cli.json, command).await?
        }
        Commands::CEP95 { command } => {
            run_cep95(&cli.rpc_url, sse, chain, verbosity, cli.json, command).await?
        }
    }
    Ok(())
}

async fn run_cep18(
    rpc_url: &str,
    sse: Option<String>,
    chain: Option<String>,
    verbosity: Option<Verbosity>,
    json: bool,
    command: CEP18Commands,
) -> Result<()> {
    match command {
        CEP18Commands::Info => {
            let client =
                CEP18Client::new(rpc_url, sse, chain, verbosity).context("create CEP-18 client")?;
            print_info(
                "cep18",
                client.rpc_url(),
                client.sse_url(),
                client.chain_name(),
                json,
            )?;
        }
        CEP18Commands::Install {
            name,
            symbol,
            decimals,
            total_supply,
            wasm,
            secret_key,
            payment,
            make_only,
            initiator_addr,
            events_mode,
            enable_mint_and_burn,
        } => {
            let bytes = fs::read(&wasm).with_context(|| format!("read wasm {}", wasm.display()))?;
            let client =
                CEP18Client::new(rpc_url, sse, chain, verbosity).context("create CEP-18 client")?;
            let mut args = CEP18InstallArgs::new(&name, &symbol, decimals, &total_supply);
            if let Some(mode) = events_mode18(events_mode)? {
                args = args.with_events_mode(mode);
            }
            if enable_mint_and_burn {
                args = args.with_mint_and_burn(true);
            }
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let put = client
                .install(&args, &bytes, &tx)
                .await
                .context("install")?;
            if make_only {
                print_call_result(&put, json)?;
            } else {
                let secret =
                    read_secret(secret_key.as_ref().context("secret_key required for put")?)?;
                let pk = casper_rust_wasm_sdk::helpers::public_key_from_secret_key(&secret)
                    .context("public key")?;
                let contract = client
                    .get_account_named_key(&pk, &format!("cep18_contract_hash_{name}"))
                    .await
                    .context("contract named key")?;
                let package = client
                    .get_account_named_key(&pk, &format!("cep18_contract_package_{name}"))
                    .await
                    .context("package named key")?;
                print_install_put(&put, &contract, &package, json)?;
            }
        }
        CEP18Commands::Transfer {
            contract_hash,
            package_hash,
            recipient,
            amount,
            secret_key,
            payment,
            make_only,
            initiator_addr,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP18Client::new(rpc_url, sse, chain, verbosity).context("create CEP-18 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .transfer(&recipient, &amount, &tx)
                .await
                .context("transfer")?;
            print_call_result(&put, json)?;
        }
        CEP18Commands::TransferFrom {
            contract_hash,
            package_hash,
            owner,
            recipient,
            amount,
            secret_key,
            payment,
            make_only,
            initiator_addr,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP18Client::new(rpc_url, sse, chain, verbosity).context("create CEP-18 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .transfer_from(&owner, &recipient, &amount, &tx)
                .await
                .context("transfer_from")?;
            print_call_result(&put, json)?;
        }
        CEP18Commands::Approve {
            contract_hash,
            package_hash,
            spender,
            amount,
            secret_key,
            payment,
            make_only,
            initiator_addr,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP18Client::new(rpc_url, sse, chain, verbosity).context("create CEP-18 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .approve(&spender, &amount, &tx)
                .await
                .context("approve")?;
            print_call_result(&put, json)?;
        }
        CEP18Commands::Mint {
            contract_hash,
            package_hash,
            owner,
            amount,
            secret_key,
            payment,
            make_only,
            initiator_addr,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP18Client::new(rpc_url, sse, chain, verbosity).context("create CEP-18 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client.mint(&owner, &amount, &tx).await.context("mint")?;
            print_call_result(&put, json)?;
        }
        CEP18Commands::Burn {
            contract_hash,
            package_hash,
            owner,
            amount,
            secret_key,
            payment,
            make_only,
            initiator_addr,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP18Client::new(rpc_url, sse, chain, verbosity).context("create CEP-18 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client.burn(&owner, &amount, &tx).await.context("burn")?;
            print_call_result(&put, json)?;
        }
        CEP18Commands::Name {
            contract_hash,
            package_hash,
        } => {
            let mut client =
                CEP18Client::new(rpc_url, sse, chain, verbosity).context("create CEP-18 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let name = client.name().await.context("name")?;
            if json {
                println!("{}", serde_json::json!({ "name": name }));
            } else {
                println!("{name}");
            }
        }
        CEP18Commands::Balance {
            contract_hash,
            package_hash,
            account,
        } => {
            let mut client =
                CEP18Client::new(rpc_url, sse, chain, verbosity).context("create CEP-18 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let bal = client.balance_of(&account).await.context("balance_of")?;
            if json {
                println!(
                    "{}",
                    serde_json::json!({ "account": account, "balance": bal })
                );
            } else {
                println!("{bal}");
            }
        }
        CEP18Commands::SecurityBadge {
            contract_hash,
            package_hash,
            account,
        } => {
            let mut client =
                CEP18Client::new(rpc_url, sse, chain, verbosity).context("create CEP-18 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let badge = client
                .security_badge(&account)
                .await
                .context("security_badge")?;
            if json {
                match badge {
                    Some(b) => println!(
                        "{}",
                        serde_json::json!({ "badge": b.as_str(), "value": b as u8 })
                    ),
                    None => println!("{}", serde_json::json!({ "badge": null })),
                }
            } else {
                match badge {
                    Some(b) => println!("{}", b.as_str()),
                    None => println!(),
                }
            }
        }
    }
    Ok(())
}

async fn run_cep78(
    rpc_url: &str,
    sse: Option<String>,
    chain: Option<String>,
    verbosity: Option<Verbosity>,
    json: bool,
    command: CEP78Commands,
) -> Result<()> {
    match command {
        CEP78Commands::Info => {
            let client =
                CEP78Client::new(rpc_url, sse, chain, verbosity).context("create CEP-78 client")?;
            print_info(
                "cep78",
                client.rpc_url(),
                client.sse_url(),
                client.chain_name(),
                json,
            )?;
        }
        CEP78Commands::Install {
            name,
            symbol,
            total_token_supply,
            wasm,
            secret_key,
            payment,
            make_only,
            initiator_addr,
            events_mode,
        } => {
            let bytes = fs::read(&wasm).with_context(|| format!("read wasm {}", wasm.display()))?;
            let client =
                CEP78Client::new(rpc_url, sse, chain, verbosity).context("create CEP-78 client")?;
            let mut args = CEP78InstallArgs::new(&name, &symbol, total_token_supply);
            if let Some(mode) = events_mode78(events_mode)? {
                args = args.with_events_mode(mode);
            }
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let put = client
                .install(&args, &bytes, &tx)
                .await
                .context("install")?;
            if make_only {
                print_call_result(&put, json)?;
            } else {
                let secret =
                    read_secret(secret_key.as_ref().context("secret_key required for put")?)?;
                let pk = casper_rust_wasm_sdk::helpers::public_key_from_secret_key(&secret)
                    .context("public key")?;
                let contract = client
                    .get_account_named_key(&pk, &format!("cep78_contract_hash_{name}"))
                    .await
                    .context("contract named key")?;
                let package = client
                    .get_account_named_key(&pk, &format!("cep78_contract_package_{name}"))
                    .await
                    .context("package named key")?;
                print_install_put(&put, &contract, &package, json)?;
            }
        }
        CEP78Commands::Mint {
            contract_hash,
            package_hash,
            token_owner,
            token_meta_data,
            token_hash,
            secret_key,
            payment,
            make_only,
            initiator_addr,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP78Client::new(rpc_url, sse, chain, verbosity).context("create CEP-78 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .mint(&token_owner, &token_meta_data, token_hash.as_deref(), &tx)
                .await
                .context("mint")?;
            print_call_result(&put, json)?;
        }
        CEP78Commands::Burn {
            contract_hash,
            package_hash,
            token_id,
            token_hash,
            secret_key,
            payment,
            make_only,
            initiator_addr,
        } => {
            let token = cep78_token(token_id, token_hash)?;
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP78Client::new(rpc_url, sse, chain, verbosity).context("create CEP-78 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client.burn(&token, &tx).await.context("burn")?;
            print_call_result(&put, json)?;
        }
        CEP78Commands::Transfer {
            contract_hash,
            package_hash,
            source,
            target,
            token_id,
            token_hash,
            secret_key,
            payment,
            make_only,
            initiator_addr,
        } => {
            let token = cep78_token(token_id, token_hash)?;
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP78Client::new(rpc_url, sse, chain, verbosity).context("create CEP-78 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .transfer(&source, &target, &token, &tx)
                .await
                .context("transfer")?;
            print_call_result(&put, json)?;
        }
        CEP78Commands::Approve {
            contract_hash,
            package_hash,
            operator,
            token_id,
            token_hash,
            secret_key,
            payment,
            make_only,
            initiator_addr,
        } => {
            let token = cep78_token(token_id, token_hash)?;
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP78Client::new(rpc_url, sse, chain, verbosity).context("create CEP-78 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .approve(&operator, &token, &tx)
                .await
                .context("approve")?;
            print_call_result(&put, json)?;
        }
        CEP78Commands::Name {
            contract_hash,
            package_hash,
        } => {
            let mut client =
                CEP78Client::new(rpc_url, sse, chain, verbosity).context("create CEP-78 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let name = client.collection_name().await.context("collection name")?;
            if json {
                println!("{}", serde_json::json!({ "name": name }));
            } else {
                println!("{name}");
            }
        }
        CEP78Commands::Balance {
            contract_hash,
            package_hash,
            account,
        } => {
            let mut client =
                CEP78Client::new(rpc_url, sse, chain, verbosity).context("create CEP-78 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let bal = client.balance_of(&account).await.context("balance_of")?;
            if json {
                println!(
                    "{}",
                    serde_json::json!({ "account": account, "balance": bal })
                );
            } else {
                println!("{bal}");
            }
        }
        CEP78Commands::OwnershipMode {
            contract_hash,
            package_hash,
        } => {
            let mut client =
                CEP78Client::new(rpc_url, sse, chain, verbosity).context("create CEP-78 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let mode = client.ownership_mode().await.context("ownership_mode")?;
            if json {
                println!(
                    "{}",
                    serde_json::json!({ "ownership_mode": format!("{:?}", mode), "value": u8::from(mode) })
                );
            } else {
                println!("{:?} ({})", mode, u8::from(mode));
            }
        }
    }
    Ok(())
}

async fn run_cep85(
    rpc_url: &str,
    sse: Option<String>,
    chain: Option<String>,
    verbosity: Option<Verbosity>,
    json: bool,
    command: CEP85Commands,
) -> Result<()> {
    match command {
        CEP85Commands::Info => {
            let client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            print_info(
                "cep85",
                client.rpc_url(),
                client.sse_url(),
                client.chain_name(),
                json,
            )?;
        }
        CEP85Commands::Install {
            name,
            uri,
            wasm,
            secret_key,
            payment,
            make_only,
            initiator_addr,
            events_mode,
            enable_burn,
            transfer_filter_contract,
            transfer_filter_method,
        } => {
            let bytes = fs::read(&wasm).with_context(|| format!("read wasm {}", wasm.display()))?;
            let client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            let mut args = CEP85InstallArgs::new(&name, &uri);
            if let Some(mode) = events_mode18(events_mode)? {
                args = args.with_events_mode(mode);
            }
            if enable_burn {
                args = args.with_enable_burn(true);
            }
            match (transfer_filter_contract, transfer_filter_method) {
                (None, None) => {}
                (Some(c), Some(m)) => {
                    args = args.with_transfer_filter(c, m);
                }
                _ => bail!("transfer_filter_contract and transfer_filter_method must both be set"),
            }
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let put = client
                .install(&args, &bytes, &tx)
                .await
                .context("install")?;
            if make_only {
                print_call_result(&put, json)?;
            } else {
                let secret =
                    read_secret(secret_key.as_ref().context("secret_key required for put")?)?;
                let pk = casper_rust_wasm_sdk::helpers::public_key_from_secret_key(&secret)
                    .context("public key")?;
                let contract = client
                    .get_account_named_key(&pk, &format!("cep85_contract_hash_{name}"))
                    .await
                    .context("contract named key")?;
                let package = client
                    .get_account_named_key(&pk, &format!("cep85_contract_package_{name}"))
                    .await
                    .context("package named key")?;
                print_install_put(&put, &contract, &package, json)?;
            }
        }
        CEP85Commands::Mint {
            contract_hash,
            package_hash,
            recipient,
            id,
            amount,
            uri,
            secret_key,
            payment,
            make_only,
            initiator_addr,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .mint(&recipient, &id, &amount, uri.as_deref(), &tx)
                .await
                .context("mint")?;
            print_call_result(&put, json)?;
        }
        CEP85Commands::Burn {
            contract_hash,
            package_hash,
            owner,
            id,
            amount,
            secret_key,
            payment,
            make_only,
            initiator_addr,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .burn(&owner, &id, &amount, &tx)
                .await
                .context("burn")?;
            print_call_result(&put, json)?;
        }
        CEP85Commands::Transfer {
            contract_hash,
            package_hash,
            from,
            to,
            id,
            amount,
            data_hex,
            secret_key,
            payment,
            make_only,
            initiator_addr,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let data = decode_optional_hex(data_hex.as_deref())?;
            let mut client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .transfer(&from, &to, &id, &amount, data.as_deref(), &tx)
                .await
                .context("transfer")?;
            print_call_result(&put, json)?;
        }
        CEP85Commands::SetApprovalForAll {
            contract_hash,
            package_hash,
            operator,
            approved,
            secret_key,
            payment,
            make_only,
            initiator_addr,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .set_approval_for_all(&operator, approved, &tx)
                .await
                .context("set_approval_for_all")?;
            print_call_result(&put, json)?;
        }
        CEP85Commands::Name {
            contract_hash,
            package_hash,
        } => {
            let mut client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let name = client.collection_name().await.context("collection name")?;
            if json {
                println!("{}", serde_json::json!({ "name": name }));
            } else {
                println!("{name}");
            }
        }
        CEP85Commands::Balance {
            contract_hash,
            package_hash,
            account,
            id,
        } => {
            let mut client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let bal = client
                .balance_of(&account, &id)
                .await
                .context("balance_of")?;
            if json {
                println!(
                    "{}",
                    serde_json::json!({ "account": account, "id": id, "balance": bal })
                );
            } else {
                println!("{bal}");
            }
        }
        CEP85Commands::BalanceOfBatch {
            contract_hash,
            package_hash,
            accounts,
            ids,
        } => {
            let mut client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let accounts_v = split_csv(&accounts);
            let ids_v = split_csv(&ids);
            let acct_refs: Vec<&str> = accounts_v.iter().map(String::as_str).collect();
            let id_refs: Vec<&str> = ids_v.iter().map(String::as_str).collect();
            let bals = client
                .balance_of_batch(&acct_refs, &id_refs)
                .await
                .context("balance_of_batch")?;
            if json {
                println!("{}", serde_json::json!({ "balances": bals }));
            } else {
                println!("{}", bals.join(","));
            }
        }
        CEP85Commands::SupplyOfBatch {
            contract_hash,
            package_hash,
            ids,
        } => {
            let mut client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let ids_v = split_csv(&ids);
            let id_refs: Vec<&str> = ids_v.iter().map(String::as_str).collect();
            let supplies = client
                .supply_of_batch(&id_refs)
                .await
                .context("supply_of_batch")?;
            if json {
                println!("{}", serde_json::json!({ "supplies": supplies }));
            } else {
                println!("{}", supplies.join(","));
            }
        }
        CEP85Commands::TotalSupplyOfBatch {
            contract_hash,
            package_hash,
            ids,
        } => {
            let mut client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let ids_v = split_csv(&ids);
            let id_refs: Vec<&str> = ids_v.iter().map(String::as_str).collect();
            let caps = client
                .total_supply_of_batch(&id_refs)
                .await
                .context("total_supply_of_batch")?;
            if json {
                println!("{}", serde_json::json!({ "total_supplies": caps }));
            } else {
                println!("{}", caps.join(","));
            }
        }
        CEP85Commands::TotalFungibleSupply {
            contract_hash,
            package_hash,
            id,
        } => {
            let mut client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let rem = client
                .total_fungible_supply(&id)
                .await
                .context("total_fungible_supply")?;
            if json {
                println!(
                    "{}",
                    serde_json::json!({ "id": id, "total_fungible_supply": rem })
                );
            } else {
                match rem {
                    Some(v) => println!("{v}"),
                    None => println!(),
                }
            }
        }
        CEP85Commands::EnableBurn {
            contract_hash,
            package_hash,
        } => {
            let mut client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let ok = client.enable_burn().await.context("enable_burn")?;
            if json {
                println!("{}", serde_json::json!({ "enable_burn": ok }));
            } else {
                println!("{ok}");
            }
        }
        CEP85Commands::EventsMode {
            contract_hash,
            package_hash,
        } => {
            let mut client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let mode = client.events_mode().await.context("events_mode")?;
            if json {
                println!(
                    "{}",
                    serde_json::json!({ "events_mode": mode.as_str(), "value": u8::from(mode) })
                );
            } else {
                println!("{}", mode.as_str());
            }
        }
        CEP85Commands::NumberOfMintedTokens {
            contract_hash,
            package_hash,
        } => {
            let mut client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let n = client
                .number_of_minted_tokens()
                .await
                .context("number_of_minted_tokens")?;
            if json {
                println!("{}", serde_json::json!({ "number_of_minted_tokens": n }));
            } else {
                println!("{n}");
            }
        }
        CEP85Commands::TransferFilterContract {
            contract_hash,
            package_hash,
        } => {
            let mut client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let v = client
                .transfer_filter_contract()
                .await
                .context("transfer_filter_contract")?;
            if json {
                println!("{}", serde_json::json!({ "transfer_filter_contract": v }));
            } else {
                match v {
                    Some(s) => println!("{s}"),
                    None => println!(),
                }
            }
        }
        CEP85Commands::TransferFilterMethod {
            contract_hash,
            package_hash,
        } => {
            let mut client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let v = client
                .transfer_filter_method()
                .await
                .context("transfer_filter_method")?;
            if json {
                println!("{}", serde_json::json!({ "transfer_filter_method": v }));
            } else {
                match v {
                    Some(s) => println!("{s}"),
                    None => println!(),
                }
            }
        }
        CEP85Commands::SecurityBadge {
            contract_hash,
            package_hash,
            entity,
        } => {
            let mut client =
                CEP85Client::new(rpc_url, sse, chain, verbosity).context("create CEP-85 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let badge = client
                .security_badge(&entity)
                .await
                .context("security_badge")?;
            if json {
                match badge {
                    Some(b) => println!(
                        "{}",
                        serde_json::json!({ "badge": b.as_str(), "value": b as u8 })
                    ),
                    None => println!("{}", serde_json::json!({ "badge": null })),
                }
            } else {
                match badge {
                    Some(b) => println!("{}", b.as_str()),
                    None => println!(),
                }
            }
        }
    }
    Ok(())
}

async fn run_cep95(
    rpc_url: &str,
    sse: Option<String>,
    chain: Option<String>,
    verbosity: Option<Verbosity>,
    json: bool,
    command: CEP95Commands,
) -> Result<()> {
    match command {
        CEP95Commands::Info => {
            let client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            print_info(
                "cep95",
                client.rpc_url(),
                client.sse_url(),
                client.chain_name(),
                json,
            )?;
        }
        CEP95Commands::Install {
            name,
            symbol,
            package_key_name,
            wasm,
            secret_key,
            make_only,
            initiator_addr,
            payment,
        } => {
            let bytes = fs::read(&wasm).with_context(|| format!("read wasm {}", wasm.display()))?;
            let mut client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            let args = CEP95InstallArgs::new(&name, &symbol, &package_key_name);
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let put = client
                .install(&args, &bytes, &tx)
                .await
                .context("install")?;
            if make_only {
                print_call_result(&put, json)?;
            } else {
                let secret =
                    read_secret(secret_key.as_ref().context("secret_key required for put")?)?;
                let pk = casper_rust_wasm_sdk::helpers::public_key_from_secret_key(&secret)
                    .context("public key")?;
                let (contract, package) = client
                    .bind_odra_install(&pk, &package_key_name)
                    .await
                    .context("bind_odra_install")?;
                print_install_put(&put, &contract, &package, json)?;
            }
        }
        CEP95Commands::Mint {
            contract_hash,
            package_hash,
            to,
            token_id,
            secret_key,
            make_only,
            initiator_addr,
            payment,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .mint(&to, &token_id, None, &tx)
                .await
                .context("mint")?;
            print_call_result(&put, json)?;
        }
        CEP95Commands::Burn {
            contract_hash,
            package_hash,
            token_id,
            secret_key,
            make_only,
            initiator_addr,
            payment,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client.burn(&token_id, &tx).await.context("burn")?;
            print_call_result(&put, json)?;
        }
        CEP95Commands::TransferFrom {
            contract_hash,
            package_hash,
            from,
            to,
            token_id,
            secret_key,
            make_only,
            initiator_addr,
            payment,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .transfer_from(&from, &to, &token_id, &tx)
                .await
                .context("transfer_from")?;
            print_call_result(&put, json)?;
        }
        CEP95Commands::Approve {
            contract_hash,
            package_hash,
            spender,
            token_id,
            secret_key,
            make_only,
            initiator_addr,
            payment,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .approve(&spender, &token_id, &tx)
                .await
                .context("approve")?;
            print_call_result(&put, json)?;
        }
        CEP95Commands::RevokeApproval {
            contract_hash,
            package_hash,
            token_id,
            secret_key,
            make_only,
            initiator_addr,
            payment,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .revoke_approval(&token_id, &tx)
                .await
                .context("revoke_approval")?;
            print_call_result(&put, json)?;
        }
        CEP95Commands::ApproveForAll {
            contract_hash,
            package_hash,
            operator,
            secret_key,
            make_only,
            initiator_addr,
            payment,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .approve_for_all(&operator, &tx)
                .await
                .context("approve_for_all")?;
            print_call_result(&put, json)?;
        }
        CEP95Commands::RevokeApprovalForAll {
            contract_hash,
            package_hash,
            operator,
            secret_key,
            make_only,
            initiator_addr,
            payment,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .revoke_approval_for_all(&operator, &tx)
                .await
                .context("revoke_approval_for_all")?;
            print_call_result(&put, json)?;
        }
        CEP95Commands::Name {
            contract_hash,
            package_hash,
        } => {
            let mut client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let name = client.name().await.context("name")?;
            if json {
                println!("{}", serde_json::json!({ "name": name }));
            } else {
                println!("{name}");
            }
        }
        CEP95Commands::Symbol {
            contract_hash,
            package_hash,
        } => {
            let mut client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let symbol = client.symbol().await.context("symbol")?;
            if json {
                println!("{}", serde_json::json!({ "symbol": symbol }));
            } else {
                println!("{symbol}");
            }
        }
        CEP95Commands::Balance {
            contract_hash,
            package_hash,
            account,
        } => {
            let mut client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let bal = client.balance_of(&account).await.context("balance_of")?;
            if json {
                println!(
                    "{}",
                    serde_json::json!({ "account": account, "balance": bal })
                );
            } else {
                println!("{bal}");
            }
        }
        CEP95Commands::OwnerOf {
            contract_hash,
            package_hash,
            token_id,
        } => {
            let mut client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let owner = client.owner_of(&token_id).await.context("owner_of")?;
            if json {
                println!(
                    "{}",
                    serde_json::json!({ "token_id": token_id, "owner": owner })
                );
            } else {
                println!("{owner}");
            }
        }
        CEP95Commands::GetApproved {
            contract_hash,
            package_hash,
            token_id,
        } => {
            let mut client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let approved = client
                .get_approved(&token_id)
                .await
                .context("get_approved")?;
            if json {
                println!(
                    "{}",
                    serde_json::json!({ "token_id": token_id, "approved": approved })
                );
            } else {
                match approved {
                    Some(a) => println!("{a}"),
                    None => println!(),
                }
            }
        }
        CEP95Commands::IsApprovedForAll {
            contract_hash,
            package_hash,
            owner,
            operator,
        } => {
            let mut client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let ok = client
                .is_approved_for_all(&owner, &operator)
                .await
                .context("is_approved_for_all")?;
            if json {
                println!("{}", serde_json::json!({ "approved": ok }));
            } else {
                println!("{ok}");
            }
        }
        CEP95Commands::GetOwner {
            contract_hash,
            package_hash,
        } => {
            let mut client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let owner = client.get_owner().await.context("get_owner")?;
            if json {
                println!("{}", serde_json::json!({ "owner": owner }));
            } else {
                println!("{owner}");
            }
        }
        CEP95Commands::TransferOwnership {
            contract_hash,
            package_hash,
            new_owner,
            secret_key,
            payment,
            make_only,
            initiator_addr,
        } => {
            let tx = build_tx_params(&secret_key, &payment, make_only, &initiator_addr)?;
            let mut client =
                CEP95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .transfer_ownership(&new_owner, &tx)
                .await
                .context("transfer_ownership")?;
            print_call_result(&put, json)?;
        }
    }
    Ok(())
}

fn split_csv(s: &str) -> Vec<String> {
    s.split(',')
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .map(str::to_string)
        .collect()
}

fn decode_optional_hex(data_hex: Option<&str>) -> Result<Option<Vec<u8>>> {
    match data_hex {
        None => Ok(None),
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => {
            let s = s.trim().trim_start_matches("0x");
            if s.len() % 2 != 0 {
                bail!("data_hex must have even length");
            }
            let mut out = Vec::with_capacity(s.len() / 2);
            let bytes = s.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                let hi = (bytes[i] as char)
                    .to_digit(16)
                    .ok_or_else(|| anyhow::anyhow!("data_hex: invalid hex"))?;
                let lo = (bytes[i + 1] as char)
                    .to_digit(16)
                    .ok_or_else(|| anyhow::anyhow!("data_hex: invalid hex"))?;
                out.push(((hi << 4) | lo) as u8);
                i += 2;
            }
            Ok(Some(out))
        }
    }
}

fn format_rpc(rpc: &str) -> String {
    ceps_client::core::normalize_rpc_url(rpc).unwrap_or_else(|_| rpc.to_string())
}

fn print_info(cep: &str, rpc: &str, sse: Option<&str>, chain: &str, json: bool) -> Result<()> {
    if json {
        println!(
            "{}",
            serde_json::json!({
                "cep": cep,
                "rpc_url": rpc,
                "sse_url": sse,
                "chain_name": chain,
            })
        );
    } else {
        println!("{cep}");
        println!("  rpc_url     {rpc}");
        println!("  sse_url     {}", sse.unwrap_or(""));
        println!("  chain_name  {chain}");
    }
    if rpc.is_empty() {
        bail!("rpc url empty");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn parses_status() {
        let cli = Cli::try_parse_from(["ceps-client-cli", "status"]).expect("parse");
        assert!(matches!(cli.command, Commands::Status));
    }

    #[test]
    fn parses_put_and_wait_transaction() {
        let cli = Cli::try_parse_from([
            "ceps",
            "put-transaction",
            "--transaction-file",
            "tx.json",
            "--no-wait",
            "--contract-hash",
            "aabb",
        ])
        .expect("parse put");
        match cli.command {
            Commands::PutTransaction {
                transaction_file,
                no_wait,
                contract_hash,
                ..
            } => {
                assert_eq!(transaction_file, PathBuf::from("tx.json"));
                assert!(no_wait);
                assert_eq!(contract_hash.as_deref(), Some("aabb"));
            }
            other => panic!("unexpected {other:?}"),
        }

        let cli = Cli::try_parse_from([
            "ceps",
            "wait-transaction",
            "--transaction-hash",
            "transaction-deadbeef",
            "--wait-timeout-ms",
            "5000",
        ])
        .expect("parse wait");
        match cli.command {
            Commands::WaitTransaction {
                transaction_hash,
                wait_timeout_ms,
            } => {
                assert_eq!(transaction_hash, "transaction-deadbeef");
                assert_eq!(wait_timeout_ms, Some(5000));
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn parses_ces_parse_execution_and_collect() {
        let cli = Cli::try_parse_from([
            "ceps",
            "ces",
            "parse-execution",
            "--contract-hash",
            "hash-aa",
            "--execution-file",
            "-",
        ])
        .expect("parse-execution");
        assert!(matches!(
            cli.command,
            Commands::CES {
                command: CESCommands::ParseExecution { .. }
            }
        ));

        let cli = Cli::try_parse_from([
            "ceps",
            "ces",
            "collect",
            "--contract-hash",
            "hash-aa",
            "--event-name",
            "Mint",
            "--event-name",
            "Burn",
            "--max-transactions",
            "3",
        ])
        .expect("collect");
        match cli.command {
            Commands::CES {
                command:
                    CESCommands::Collect {
                        event_names,
                        max_transactions,
                        ..
                    },
            } => {
                assert_eq!(event_names, vec!["Mint".to_string(), "Burn".to_string()]);
                assert_eq!(max_transactions, 3);
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn parses_cep18_transfer_make_only() {
        let cli = Cli::try_parse_from([
            "ceps",
            "cep18",
            "transfer",
            "--contract-hash",
            "b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f",
            "--recipient",
            "account-hash-b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f",
            "--amount",
            "10",
            "--make-only",
            "--initiator-addr",
            "010101010101010101010101010101010101010101010101010101010101010101",
        ])
        .expect("parse");
        match cli.command {
            Commands::CEP18 {
                command:
                    CEP18Commands::Transfer {
                        make_only,
                        amount,
                        secret_key,
                        ..
                    },
            } => {
                assert!(make_only);
                assert_eq!(amount, "10");
                assert!(secret_key.is_none());
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn parses_cep78_install_make_only() {
        let cli = Cli::try_parse_from([
            "ceps",
            "cep78",
            "install",
            "--name",
            "N",
            "--symbol",
            "S",
            "--wasm",
            "tests/wasm/cep78/cep78.wasm",
            "--make-only",
            "--initiator-addr",
            "010101010101010101010101010101010101010101010101010101010101010101",
        ])
        .expect("parse");
        assert!(matches!(
            cli.command,
            Commands::CEP78 {
                command: CEP78Commands::Install {
                    make_only: true,
                    ..
                }
            }
        ));
    }

    #[test]
    fn parses_cep85_mint_make_only() {
        let cli = Cli::try_parse_from([
            "ceps",
            "cep85",
            "mint",
            "--contract-hash",
            "b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f",
            "--recipient",
            "account-hash-b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f",
            "--id",
            "1",
            "--amount",
            "5",
            "--make-only",
            "--initiator-addr",
            "010101010101010101010101010101010101010101010101010101010101010101",
        ])
        .expect("parse");
        match cli.command {
            Commands::CEP85 {
                command: CEP85Commands::Mint { make_only, id, .. },
            } => {
                assert!(make_only);
                assert_eq!(id, "1");
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn parses_cep85_balance() {
        let cli = Cli::try_parse_from([
            "ceps",
            "cep85",
            "balance",
            "--contract-hash",
            "b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f",
            "--account",
            "account-hash-b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f",
            "--id",
            "1",
        ])
        .expect("parse");
        match cli.command {
            Commands::CEP85 {
                command: CEP85Commands::Balance { id, .. },
            } => assert_eq!(id, "1"),
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn parses_cep78_name() {
        let cli = Cli::try_parse_from([
            "ceps",
            "cep78",
            "name",
            "--contract-hash",
            "b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f",
        ])
        .expect("parse");
        assert!(matches!(
            cli.command,
            Commands::CEP78 {
                command: CEP78Commands::Name { .. }
            }
        ));
    }

    #[test]
    fn parses_cep95_owner_of() {
        let cli = Cli::try_parse_from([
            "ceps",
            "cep95",
            "owner-of",
            "--contract-hash",
            "b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f",
            "--token-id",
            "1",
        ])
        .expect("parse");
        match cli.command {
            Commands::CEP95 {
                command: CEP95Commands::OwnerOf { token_id, .. },
            } => assert_eq!(token_id, "1"),
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn build_tx_params_make_only_without_secret() {
        let tx = build_tx_params(
            &None,
            "1000",
            true,
            &Some("010101010101010101010101010101010101010101010101010101010101010101".into()),
        )
        .expect("tx");
        assert!(!tx.put);
        assert!(tx.secret_key_pem.is_none());
    }
}
