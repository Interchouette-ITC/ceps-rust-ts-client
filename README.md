# ceps-rust-ts-client

One **Rust** client for Casper **CEP-18**, **CEP-78**, and **CEP-85** (plus a thin WASM pack and a `ceps` CLI).

It replaces the separate TypeScript **`client-js`** packages that lived next to each CEP contract. Instead of three JS clients, you use one library: `Cep18Client` / `Cep78Client` / `Cep85Client`, on top of [`casper-rust-wasm-sdk`](https://github.com/casper-ecosystem/casper-rust-wasm-sdk).

```text
CEP-18 client-js  ─┐
CEP-78 client-js  ─┼─→  ceps-client (Rust)  +  ceps CLI  +  ceps-wasm
CEP-85 client-js  ─┘
```

## What you can do

| CEP | Standard | Typical flow |
| --- | --- | --- |
| **18** | Fungible token | install → bind hash → `name` / `balance_of` → `transfer` / `mint` / `burn` |
| **78** | Enhanced NFT | install → bind hash → `mint` → `owner_of` / `balance_of` |
| **85** | Multi-token | install → bind hash → `mint` / `burn` → `balance_of(account, id)` |

Defaults talk to local NCTL (`http://127.0.0.1:11101`, SSE `…:18101/events`, chain `casper-net-1`).

## Usage

Needs a running node, a secret-key PEM, and contract WASM (stage tips with `make wasm-from-ceps`, or point at your own `.wasm`).

### CEP-18 - fungible

```text
install wasm → named keys cep18_contract_hash_* / package_*
  → set_contract_hash
  → name / symbol / balance_of
  → transfer | mint | burn
```

```rust
use ceps_client::cep18::InstallArgs;
use ceps_client::{Cep18Client, DeployParams, EventsMode, Verbosity};

let mut client = Cep18Client::new(
    "http://127.0.0.1:11101",
    Some("http://127.0.0.1:18101/events".into()),
    Some("casper-net-1".into()),
    Some(Verbosity::Low),
)?;

let put = client
    .install(
        &InstallArgs::new("MyToken", "MTK", 9, "1000000000")
            .with_events_mode(EventsMode::Ces)
            .with_mint_and_burn(true),
        &wasm_bytes,
        &DeployParams::new(&secret_pem, "400000000000"),
    )
    .await?;
// bind installer named keys, then:
client.set_contract_hash(&contract_hash, Some(&package_hash))?;
let bal = client.balance_of("account-hash-…").await?;
```

Details: [docs/cep18/](docs/cep18/) · example: `cargo run -p ceps-client --example cep18_install`

### CEP-78 - NFT

```text
install wasm → named keys cep78_contract_hash_* / package_*
  → set_contract_hash
  → mint → owner_of / balance_of
```

```rust
use ceps_client::cep78::InstallArgs;
use ceps_client::{Cep78Client, DeployParams, EventsMode78, Verbosity};

let mut client = Cep78Client::new(/* rpc, sse, chain, verbosity */)?;
client
    .install(
        &InstallArgs::new("MyNft", "NFT", 100).with_events_mode(EventsMode78::Ces),
        &wasm_bytes,
        &DeployParams::new(&secret_pem, "600000000000"),
    )
    .await?;
client.set_contract_hash(&contract_hash, Some(&package_hash))?;
client
    .mint(
        "account-hash-…",
        r#"{"name":"token-1"}"#,
        None,
        &DeployParams::new(&secret_pem, "5000000000"),
    )
    .await?;
let owner = client.owner_of(&token_id).await?;
```

Details: [docs/cep78/](docs/cep78/) · example: `cargo run -p ceps-client --example cep78_install`

### CEP-85 - multi-token

```text
install wasm → named keys cep85_contract_hash_* / package_*
  → set_contract_hash
  → mint / burn → balance_of(account, id)
```

```rust
use ceps_client::cep85::InstallArgs;
use ceps_client::{Cep85Client, DeployParams, EventsMode, Verbosity};

let mut client = Cep85Client::new(/* rpc, sse, chain, verbosity */)?;
client
    .install(
        &InstallArgs::new("MyMulti", "https://example.com/{id}.json")
            .with_events_mode(EventsMode::Ces)
            .with_enable_burn(true),
        &wasm_bytes,
        &DeployParams::new(&secret_pem, "550000000000"),
    )
    .await?;
client.set_contract_hash(&contract_hash, Some(&package_hash))?;
client.mint(&owner, "1", "10", None, &DeployParams::new(&secret_pem, "5000000000")).await?;
let bal = client.balance_of(&owner, "1").await?;
```

Details: [docs/cep85/](docs/cep85/) · example: `cargo run -p ceps-client --example cep85_install`

### CLI

```bash
cargo run -p cli -- status
cargo run -p cli -- cep18 info
cargo run -p cli -- cep78 balance --contract-hash <hash> --account <account-hash-…>
cargo run -p cli -- cep85 balance --contract-hash <hash> --account <…> --id 1
```

Mutations are on the library / examples today. Flags: [docs/cli.md](docs/cli.md).

### Try an install end-to-end

```bash
make prepare && make build
make wasm-from-ceps          # stage tip contract WASMs into tests/wasm/
# NCTL running + SECRET_KEY_USER_1 set to a PEM:
cargo run -p ceps-client --example cep18_install
```

More setup: [docs/getting-started.md](docs/getting-started.md).

## Package map

| Crate | For |
| --- | --- |
| `ceps-client` | Native Rust apps (full CEP API) |
| `cli` (`ceps`) | Shell queries / status |
| `ceps-wasm` | Node / browser CEP bindings ([docs/wasm-ts.md](docs/wasm-ts.md)) |

## Documentation

| Doc | Description |
| --- | --- |
| [Getting started](docs/getting-started.md) | Build, NCTL defaults, first run |
| [Architecture](docs/architecture.md) | How lib / CLI / WASM sit on the SDK |
| [docs/cep18/](docs/cep18/) · [cep78/](docs/cep78/) · [cep85/](docs/cep85/) | Per-CEP guides |
| [CLI](docs/cli.md) · [WASM / TS](docs/wasm-ts.md) | Surfaces |
| [Testing](docs/testing.md) · [CI / CD](docs/ci.md) · [Docker](docs/docker.md) | Verify and ship |
| [Contributing](docs/contributing.md) · [SDK](docs/sdk.md) | Tips, pins, upgrades |
| [SECURITY.md](docs/SECURITY.md) | Keys and reporting |
| `make doc` | rustdoc → `docs/api-rust/` |

## Docker

```bash
make release-cli-bin && make docker-build IMAGE_TAG=local
docker run --rm ceps-rust-ts-client:local --help
docker pull interchouette/ceps-rust-ts-client:dev
```

See [docs/docker.md](docs/docker.md).

## License / security

GPL-3.0. See [LICENSE](LICENSE) and [docs/SECURITY.md](docs/SECURITY.md).
