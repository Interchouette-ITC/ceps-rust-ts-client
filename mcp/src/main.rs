//! `ceps-client-mcp` - MCP server (stdio by default, optional Streamable HTTP).

use anyhow::Result;
use ceps_client_mcp::server::{run, run_http, DEFAULT_HTTP_LISTEN};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "ceps-client-mcp",
    about = "ceps-client MCP server (stdio or Streamable HTTP)",
    version
)]
struct Cli {
    /// Serve Streamable HTTP instead of stdio.
    #[arg(
        long,
        env = "CEPS_MCP_HTTP",
        value_parser = clap::builder::BoolishValueParser::new()
    )]
    http: bool,

    /// HTTP bind address when `--http` is set (also: `CEPS_MCP_ADDR`).
    #[arg(long, env = "CEPS_MCP_ADDR", default_value = DEFAULT_HTTP_LISTEN)]
    listen: String,
}

fn init_logging() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    let cli = Cli::parse();

    if cli.http {
        tracing::info!(addr = %cli.listen, "ceps-client-mcp starting (HTTP)");
        run_http(&cli.listen)
            .await
            .map_err(|err| anyhow::anyhow!("{err}"))?;
    } else {
        tracing::info!("ceps-client-mcp starting (stdio)");
        run().await.map_err(|err| anyhow::anyhow!("{err}"))?;
    }
    Ok(())
}
