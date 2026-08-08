# Getting started

## Prerequisites

- Rust stable (edition 2021)
- [`casper-rust-wasm-sdk`](https://github.com/casper-ecosystem/casper-rust-wasm-sdk) available for the workspace path dependency (see root `Cargo.toml`)
- Optional: a local Casper NCTL `dev` network for live calls

## Build

```bash
make prepare
make build
make unit-test
```

## First CLI call

```bash
cargo run -p cli -- status
cargo run -p cli -- cep18 info --json
```

Defaults match NCTL `dev`:

| Setting | Default                         |
| ------- | ------------------------------- |
| RPC     | `http://127.0.0.1:11101`        |
| SSE     | `http://127.0.0.1:18101/events` |
| Chain   | `casper-net-1`                  |

## First Rust snippet

```rust
use ceps_client::{Cep18Client, Verbosity};

let client = Cep18Client::new(
    "http://127.0.0.1:11101",
    Some("http://127.0.0.1:18101/events".into()),
    Some("casper-net-1".into()),
    Some(Verbosity::Low),
)?;
```

Install / transfer flows are documented in [cep18/](cep18/) once you stage tip WASMs with `make wasm-from-ceps`.
