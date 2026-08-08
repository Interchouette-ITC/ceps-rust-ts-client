//! `ceps-client-cli` - clap wrapper over `ceps-client`.

use anyhow::{bail, Context, Result};
use casper_rust_wasm_sdk::types::verbosity::Verbosity;
use ceps_client::cep95::InstallArgs as Cep95InstallArgs;
use ceps_client::{Cep18Client, Cep78Client, Cep85Client, Cep95Client, DeployParams};
use clap::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::path::PathBuf;
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
    /// CEP-18 fungible token commands.
    Cep18 {
        #[command(subcommand)]
        command: Cep18Commands,
    },
    /// CEP-78 NFT commands.
    Cep78 {
        #[command(subcommand)]
        command: Cep78Commands,
    },
    /// CEP-85 multi-token commands.
    Cep85 {
        #[command(subcommand)]
        command: Cep85Commands,
    },
    /// CEP-95 NFT commands (supported simpler API).
    Cep95 {
        #[command(subcommand)]
        command: Cep95Commands,
    },
    /// CES parse helpers (SDK CESParser).
    Ces {
        #[command(subcommand)]
        command: CesCommands,
    },
}

#[derive(Debug, Subcommand)]
enum Cep18Commands {
    /// Print client endpoint configuration.
    Info,
}

#[derive(Debug, Subcommand)]
enum Cep78Commands {
    /// Print client endpoint configuration.
    Info,
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
enum CesCommands {
    /// Parse CES events for a transaction against a contract hash.
    Parse {
        #[arg(long)]
        contract_hash: String,
        #[arg(long)]
        transaction_hash: String,
    },
}

#[derive(Debug, Subcommand)]
enum Cep85Commands {
    /// Print client endpoint configuration.
    Info,
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
}

#[derive(Debug, Subcommand)]
enum Cep95Commands {
    /// Print client endpoint configuration.
    Info,
    /// Install Odra OwnedCep95 WASM (`--wasm`, `--secret-key`).
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
        secret_key: PathBuf,
        #[arg(long, default_value = "600000000000")]
        payment: String,
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
        secret_key: PathBuf,
        #[arg(long, default_value = "5000000000")]
        payment: String,
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
        secret_key: PathBuf,
        #[arg(long, default_value = "5000000000")]
        payment: String,
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
        secret_key: PathBuf,
        #[arg(long, default_value = "5000000000")]
        payment: String,
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
        secret_key: PathBuf,
        #[arg(long, default_value = "5000000000")]
        payment: String,
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
        secret_key: PathBuf,
        #[arg(long, default_value = "5000000000")]
        payment: String,
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
        secret_key: PathBuf,
        #[arg(long, default_value = "5000000000")]
        payment: String,
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
        secret_key: PathBuf,
        #[arg(long, default_value = "5000000000")]
        payment: String,
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
}

fn read_secret(path: &PathBuf) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("read secret key {}", path.display()))
}

fn print_tx(hash: &str, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::json!({ "transactionHash": hash }));
    } else {
        println!("{hash}");
    }
    Ok(())
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
        Commands::Cep18 {
            command: Cep18Commands::Info,
        } => {
            let client = Cep18Client::new(&cli.rpc_url, sse, chain, verbosity)
                .context("create CEP-18 client")?;
            print_info(
                "cep18",
                client.rpc_url(),
                client.sse_url(),
                client.chain_name(),
                cli.json,
            )?;
        }
        Commands::Cep78 { command } => match command {
            Cep78Commands::Info => {
                let client = Cep78Client::new(&cli.rpc_url, sse, chain, verbosity)
                    .context("create CEP-78 client")?;
                print_info(
                    "cep78",
                    client.rpc_url(),
                    client.sse_url(),
                    client.chain_name(),
                    cli.json,
                )?;
            }
            Cep78Commands::Name {
                contract_hash,
                package_hash,
            } => {
                let mut client = Cep78Client::new(&cli.rpc_url, sse, chain, verbosity)
                    .context("create CEP-78 client")?;
                client
                    .set_contract_hash(&contract_hash, package_hash.as_deref())
                    .context("set contract")?;
                let name = client.collection_name().await.context("collection name")?;
                if cli.json {
                    println!("{}", serde_json::json!({ "name": name }));
                } else {
                    println!("{name}");
                }
            }
            Cep78Commands::Balance {
                contract_hash,
                package_hash,
                account,
            } => {
                let mut client = Cep78Client::new(&cli.rpc_url, sse, chain, verbosity)
                    .context("create CEP-78 client")?;
                client
                    .set_contract_hash(&contract_hash, package_hash.as_deref())
                    .context("set contract")?;
                let bal = client.balance_of(&account).await.context("balance_of")?;
                if cli.json {
                    println!(
                        "{}",
                        serde_json::json!({ "account": account, "balance": bal })
                    );
                } else {
                    println!("{bal}");
                }
            }
            Cep78Commands::OwnershipMode {
                contract_hash,
                package_hash,
            } => {
                let mut client = Cep78Client::new(&cli.rpc_url, sse, chain, verbosity)
                    .context("create CEP-78 client")?;
                client
                    .set_contract_hash(&contract_hash, package_hash.as_deref())
                    .context("set contract")?;
                let mode = client.ownership_mode().await.context("ownership_mode")?;
                if cli.json {
                    println!(
                        "{}",
                        serde_json::json!({ "ownership_mode": format!("{:?}", mode), "value": u8::from(mode) })
                    );
                } else {
                    println!("{:?} ({})", mode, u8::from(mode));
                }
            }
        },
        Commands::Ces { command } => match command {
            CesCommands::Parse {
                contract_hash,
                transaction_hash,
            } => {
                let client = Cep18Client::new(&cli.rpc_url, sse, chain, verbosity)
                    .context("create client for CES")?;
                let hash =
                    if contract_hash.starts_with("hash-") || contract_hash.starts_with("entity-") {
                        contract_hash.clone()
                    } else {
                        format!("hash-{contract_hash}")
                    };
                let rows = client
                    .core()
                    .parse_ces_transaction(&[hash], &transaction_hash)
                    .await
                    .context("parse CES")?;
                println!("{}", serde_json::to_string_pretty(&rows)?);
            }
        },
        Commands::Cep85 { command } => match command {
            Cep85Commands::Info => {
                let client = Cep85Client::new(&cli.rpc_url, sse, chain, verbosity)
                    .context("create CEP-85 client")?;
                print_info(
                    "cep85",
                    client.rpc_url(),
                    client.sse_url(),
                    client.chain_name(),
                    cli.json,
                )?;
            }
            Cep85Commands::Name {
                contract_hash,
                package_hash,
            } => {
                let mut client = Cep85Client::new(&cli.rpc_url, sse, chain, verbosity)
                    .context("create CEP-85 client")?;
                client
                    .set_contract_hash(&contract_hash, package_hash.as_deref())
                    .context("set contract")?;
                let name = client.collection_name().await.context("collection name")?;
                if cli.json {
                    println!("{}", serde_json::json!({ "name": name }));
                } else {
                    println!("{name}");
                }
            }
            Cep85Commands::Balance {
                contract_hash,
                package_hash,
                account,
                id,
            } => {
                let mut client = Cep85Client::new(&cli.rpc_url, sse, chain, verbosity)
                    .context("create CEP-85 client")?;
                client
                    .set_contract_hash(&contract_hash, package_hash.as_deref())
                    .context("set contract")?;
                let bal = client
                    .balance_of(&account, &id)
                    .await
                    .context("balance_of")?;
                if cli.json {
                    println!(
                        "{}",
                        serde_json::json!({ "account": account, "id": id, "balance": bal })
                    );
                } else {
                    println!("{bal}");
                }
            }
        },
        Commands::Cep95 { command } => {
            run_cep95(&cli.rpc_url, sse, chain, verbosity, cli.json, command).await?
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
    command: Cep95Commands,
) -> Result<()> {
    match command {
        Cep95Commands::Info => {
            let client =
                Cep95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            print_info(
                "cep95",
                client.rpc_url(),
                client.sse_url(),
                client.chain_name(),
                json,
            )?;
        }
        Cep95Commands::Install {
            name,
            symbol,
            package_key_name,
            wasm,
            secret_key,
            payment,
        } => {
            let secret = read_secret(&secret_key)?;
            let bytes = fs::read(&wasm).with_context(|| format!("read wasm {}", wasm.display()))?;
            let mut client =
                Cep95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            let args = Cep95InstallArgs::new(&name, &symbol, &package_key_name);
            let deploy = DeployParams::new(&secret, &payment);
            let put = client
                .install(&args, &bytes, &deploy)
                .await
                .context("install")?;
            let pk = casper_rust_wasm_sdk::helpers::public_key_from_secret_key(&secret)
                .context("public key")?;
            let (contract, package) = client
                .bind_odra_install(&pk, &package_key_name)
                .await
                .context("bind_odra_install")?;
            if json {
                println!(
                    "{}",
                    serde_json::json!({
                        "transactionHash": put.transaction_hash,
                        "contractHash": contract,
                        "packageHash": package,
                    })
                );
            } else {
                println!("tx={}", put.transaction_hash);
                println!("contract={contract}");
                println!("package={package}");
            }
        }
        Cep95Commands::Mint {
            contract_hash,
            package_hash,
            to,
            token_id,
            secret_key,
            payment,
        } => {
            let secret = read_secret(&secret_key)?;
            let mut client =
                Cep95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .mint(&to, &token_id, None, &DeployParams::new(&secret, &payment))
                .await
                .context("mint")?;
            print_tx(&put.transaction_hash, json)?;
        }
        Cep95Commands::Burn {
            contract_hash,
            package_hash,
            token_id,
            secret_key,
            payment,
        } => {
            let secret = read_secret(&secret_key)?;
            let mut client =
                Cep95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .burn(&token_id, &DeployParams::new(&secret, &payment))
                .await
                .context("burn")?;
            print_tx(&put.transaction_hash, json)?;
        }
        Cep95Commands::TransferFrom {
            contract_hash,
            package_hash,
            from,
            to,
            token_id,
            secret_key,
            payment,
        } => {
            let secret = read_secret(&secret_key)?;
            let mut client =
                Cep95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .transfer_from(&from, &to, &token_id, &DeployParams::new(&secret, &payment))
                .await
                .context("transfer_from")?;
            print_tx(&put.transaction_hash, json)?;
        }
        Cep95Commands::Approve {
            contract_hash,
            package_hash,
            spender,
            token_id,
            secret_key,
            payment,
        } => {
            let secret = read_secret(&secret_key)?;
            let mut client =
                Cep95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .approve(&spender, &token_id, &DeployParams::new(&secret, &payment))
                .await
                .context("approve")?;
            print_tx(&put.transaction_hash, json)?;
        }
        Cep95Commands::RevokeApproval {
            contract_hash,
            package_hash,
            token_id,
            secret_key,
            payment,
        } => {
            let secret = read_secret(&secret_key)?;
            let mut client =
                Cep95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .revoke_approval(&token_id, &DeployParams::new(&secret, &payment))
                .await
                .context("revoke_approval")?;
            print_tx(&put.transaction_hash, json)?;
        }
        Cep95Commands::ApproveForAll {
            contract_hash,
            package_hash,
            operator,
            secret_key,
            payment,
        } => {
            let secret = read_secret(&secret_key)?;
            let mut client =
                Cep95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .approve_for_all(&operator, &DeployParams::new(&secret, &payment))
                .await
                .context("approve_for_all")?;
            print_tx(&put.transaction_hash, json)?;
        }
        Cep95Commands::RevokeApprovalForAll {
            contract_hash,
            package_hash,
            operator,
            secret_key,
            payment,
        } => {
            let secret = read_secret(&secret_key)?;
            let mut client =
                Cep95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
            client
                .set_contract_hash(&contract_hash, package_hash.as_deref())
                .context("set contract")?;
            let put = client
                .revoke_approval_for_all(&operator, &DeployParams::new(&secret, &payment))
                .await
                .context("revoke_approval_for_all")?;
            print_tx(&put.transaction_hash, json)?;
        }
        Cep95Commands::Name {
            contract_hash,
            package_hash,
        } => {
            let mut client =
                Cep95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
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
        Cep95Commands::Symbol {
            contract_hash,
            package_hash,
        } => {
            let mut client =
                Cep95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
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
        Cep95Commands::Balance {
            contract_hash,
            package_hash,
            account,
        } => {
            let mut client =
                Cep95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
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
        Cep95Commands::OwnerOf {
            contract_hash,
            package_hash,
            token_id,
        } => {
            let mut client =
                Cep95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
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
        Cep95Commands::GetApproved {
            contract_hash,
            package_hash,
            token_id,
        } => {
            let mut client =
                Cep95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
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
        Cep95Commands::IsApprovedForAll {
            contract_hash,
            package_hash,
            owner,
            operator,
        } => {
            let mut client =
                Cep95Client::new(rpc_url, sse, chain, verbosity).context("create CEP-95 client")?;
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
    }
    Ok(())
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
            Commands::Cep85 {
                command: Cep85Commands::Balance { id, .. },
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
            Commands::Cep78 {
                command: Cep78Commands::Name { .. }
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
            Commands::Cep95 {
                command: Cep95Commands::OwnerOf { token_id, .. },
            } => assert_eq!(token_id, "1"),
            other => panic!("unexpected {other:?}"),
        }
    }
}
