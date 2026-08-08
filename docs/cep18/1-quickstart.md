# CEP-18 quickstart

## Prerequisites

- NCTL `dev` running (RPC `http://127.0.0.1:11101`, SSE `http://127.0.0.1:18101/events`)
- Tip WASM staged: `make wasm-from-ceps` (needs `../cep-18` on `ceps-client-test`)
- Installer secret key PEM (NCTL `user-1`)

## Rust

```rust
use ceps_client::cep18::InstallArgs;
use ceps_client::{Cep18Client, DeployParams, EventsMode, Verbosity};
use std::fs;

let mut client = Cep18Client::new(
    "http://127.0.0.1:11101",
    Some("http://127.0.0.1:18101/events".into()),
    Some("casper-net-1".into()),
    Some(Verbosity::Low),
)?;

let secret = fs::read_to_string("secret_key.pem")?;
let wasm = fs::read("tests/wasm/cep18/cep18.wasm")?;
let args = InstallArgs::new("MyToken", "MTK", 9, "1000000000")
    .with_events_mode(EventsMode::Ces)
    .with_mint_and_burn(true);
let deploy = DeployParams::new(&secret, "400000000000");
let result = client.install(&args, &wasm, &deploy).await?;

// Bind hashes from installer named keys:
//   cep18_contract_hash_MyToken
//   cep18_contract_package_MyToken
client.set_contract_hash(contract_hash, Some(package_hash))?;
let name = client.name().await?;
let balance = client.balance_of("account-hash-…").await?;
```

## CLI

```bash
cargo run -p cli -- status
cargo run -p cli -- cep18 info
# Mutating subcommands: see 7-cli.md
```
