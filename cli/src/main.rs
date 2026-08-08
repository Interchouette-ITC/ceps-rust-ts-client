//! `ceps` CLI - thin clap wrapper over `ceps-client`.

use anyhow::{bail, Context, Result};
use casper_rust_wasm_sdk::types::verbosity::Verbosity;
use ceps_client::{Cep18Client, Cep78Client, Cep85Client};
use clap::{Parser, Subcommand, ValueEnum};
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(
    name = "ceps",
    version,
    about = "Unified Casper CEP client (18 / 78 / 85)"
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
        let cli = Cli::try_parse_from(["ceps", "status"]).expect("parse");
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
}
