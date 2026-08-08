# Getting started

## Prerequisites

- Rust stable (edition 2021)
- [`casper-rust-wasm-sdk`](https://github.com/casper-ecosystem/casper-rust-wasm-sdk) as the workspace Cargo dependency (path locally; see [sdk.md](sdk.md))
- Optional: local Casper NCTL `dev` for live install/query
- Optional: `wasm-pack` + Binaryen for `make pack` (Makefile pins Binaryen via `ensure-binaryen`)

## Build and verify

```bash
make prepare
make build
make check-lint
make unit-test
make doc-check
```

With NCTL up and tip WASMs staged:

```bash
make wasm-from-ceps
make integration-test
make e2e-test
```

## First CLI call

```bash
cargo run -p ceps-client-cli -- status
cargo run -p ceps-client-cli -- cep18 info --json
# or
make run-cli CLI_ARGS='status'
```

Defaults match NCTL `dev`:

| Setting | Default |
| --- | --- |
| RPC | `http://127.0.0.1:11101` |
| SSE | `http://127.0.0.1:18101/events` |
| Chain | `casper-net-1` |

Override with `--rpc-url` / `CEPS_RPC_URL` (and SSE / chain counterparts). Full flag table: [cli.md](cli.md).

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

Install / transfer flows: stage demo tip WASMs with `make wasm-from-ceps` (see [contributing.md](contributing.md)), then follow [cep18/](cep18/), [cep78/](cep78/), or [cep85/](cep85/). Examples:

```bash
SECRET_KEY_USER_1="$(cat path/to/user-1/secret_key.pem)" \
  cargo run -p ceps-client --example cep18_install
```

## WASM / TypeScript

```bash
make nodejs
make ts-test
```

See [wasm-ts.md](wasm-ts.md).

## Docker CLI

```bash
make release-cli-bin && make docker-build IMAGE_TAG=local
docker run --rm ceps-rust-ts-client:local --help
```

See [docker.md](docker.md).
