# ceps-rust-ts-client

One **Rust** client for Casper **CEP-18**, **CEP-78**, and **CEP-85**, with a **`ceps` CLI** and **WASM packs** so the same client can run from Node or the browser.

It replaces the separate TypeScript **`client-js`** packages that lived next to each CEP contract. Instead of three JS clients, you use one library: `Cep18Client` / `Cep78Client` / `Cep85Client`, on top of [`casper-rust-wasm-sdk`](https://github.com/casper-ecosystem/casper-rust-wasm-sdk).

```text
CEP-18 client-js  ─┐
CEP-78 client-js  ─┼─→  ceps-client (Rust)  +  ceps CLI  +  ceps-wasm
CEP-85 client-js  ─┘
```

## What you get

| Piece | Role |
| --- | --- |
| **Rust library** (`ceps-client`) | Full CEP API for native apps |
| **CLI** (`ceps`) | Status and common queries from the shell |
| **WASM packs** (`ceps-wasm`) | Same CEP classes for JavaScript (Node and web) |

## What you can do

| CEP | Standard | Typical flow |
| --- | --- | --- |
| **18** | Fungible token | install → bind hash → `name` / `balance_of` → `transfer` / `mint` / `burn` |
| **78** | Enhanced NFT | install → bind hash → `mint` → `owner_of` / `balance_of` |
| **85** | Multi-token | install → bind hash → `mint` / `burn` → `balance_of(account, id)` |

Defaults talk to local NCTL (`http://127.0.0.1:11101`, SSE `…:18101/events`, chain `casper-net-1`).

## Usage

Needs a running node, a secret-key PEM, and on-chain contract `.wasm` bytes (your own build, or the [demo tips](#contract-wasms-demos) via `make wasm-from-ceps`).

### CEP-18 - fungible

```text
install → named keys cep18_contract_hash_* / package_*
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
        &contract_wasm_bytes,
        &DeployParams::new(&secret_pem, "400000000000"),
    )
    .await?;
client.set_contract_hash(&contract_hash, Some(&package_hash))?;
let bal = client.balance_of("account-hash-…").await?;
```

Details: [docs/cep18/](docs/cep18/) · example: `cargo run -p ceps-client --example cep18_install`

### CEP-78 - NFT

```text
install → named keys cep78_contract_hash_* / package_*
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
        &contract_wasm_bytes,
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
install → named keys cep85_contract_hash_* / package_*
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
        &contract_wasm_bytes,
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
make wasm-from-ceps          # copy demo tip contract WASMs into tests/wasm/
# NCTL running + SECRET_KEY_USER_1 set to a PEM:
cargo run -p ceps-client --example cep18_install
```

More setup: [docs/getting-started.md](docs/getting-started.md).

## WASM packs (JavaScript)

`ceps-wasm` is the **Rust CEP client compiled for JavaScript**. Use it when your app is Node or browser and you want the same `Cep18Client` / `Cep78Client` / `Cep85Client` surface instead of a per-CEP `client-js`.

It is **not** how you fetch on-chain contracts (see [Contract WASMs](#contract-wasms-demos) and release `ceps-contracts-*.tgz`).

| Target | Output | Typical use |
| --- | --- | --- |
| Node | `pkg-nodejs/` | Backend / scripts / Vitest |
| Web | `pkg/` | Bundled frontends |

**Fetch a published pack** (no local `wasm-pack`):

```bash
TAG=v1.0.0   # or dev-preview
LABEL=${TAG#v}
curl -fsSL -o ceps-wasm-nodejs.tgz \
  "https://github.com/gRoussac/ceps-rust-ts-client/releases/download/${TAG}/ceps-wasm-nodejs-${LABEL}.tgz"
mkdir -p ceps-wasm && tar -xzf ceps-wasm-nodejs.tgz -C ceps-wasm
# → ceps-wasm/pkg-nodejs/
```

Or build locally: `make nodejs` / `make web` / `make pack`. Details: [docs/wasm-ts.md](docs/wasm-ts.md) · [docs/releases.md](docs/releases.md).

```js
import { Cep18Client } from "ceps-wasm"; // file:./ceps-wasm/pkg-nodejs after unpack

const client = new Cep18Client(
  "http://127.0.0.1:11101",
  "http://127.0.0.1:18101/events",
  "casper-net-1",
  0,
);
client.setContractHash(contractHash, packageHash);
const name = await client.name();
const bal = await client.balanceOf("account-hash-…");
```

Install from JS takes contract bytes as `Uint8Array` and returns JSON `{ transactionHash, hasExecutionResult }`. Bound methods today are a subset of the Rust API (see [docs/wasm-ts.md](docs/wasm-ts.md)); full parity is on `ceps-client`.

## Contract WASMs (demos)

This client is **not** a contract repo. You pass on-chain `.wasm` into `install`.

**Easiest:** download the release bundle (no tip checkout / no contract build):

```bash
TAG=v1.0.0
LABEL=${TAG#v}
curl -fsSL -o ceps-contracts.tgz \
  "https://github.com/gRoussac/ceps-rust-ts-client/releases/download/${TAG}/ceps-contracts-${LABEL}.tgz"
mkdir -p tests/wasm && tar -xzf ceps-contracts.tgz -C tests/wasm
```

**Or** stage from demo tip forks yourself (`make wasm-from-ceps`). Tips are short-lived entity-era builds for demos/CI, not a claim of “the” upstream CEP tip forever. This client (and these tips) are headed to **Interchouette-ITC**; sources **today**:

| CEP | Demo tip repo (now) | Branch | What you get |
| --- | --- | --- | --- |
| 18 | [gRoussac/cep18](https://github.com/gRoussac/cep18) | `ceps-client-test` | Fungible contract WASM |
| 78 | [gRoussac/cep-78-enhanced-nft](https://github.com/gRoussac/cep-78-enhanced-nft) | `ceps-client-test` | NFT + session WASMs |
| 85 | [gRoussac/cep-85](https://github.com/gRoussac/cep-85) | `ceps-client-test` | Multi-token WASM |

```bash
# after checking out those tips and building contracts there:
make wasm-from-ceps   # → tests/wasm/{cep18,cep78,cep85}/
```

Override checkout roots with `CEP18_PRODUCT` / `CEP78_PRODUCT` / `CEP85_PRODUCT`. Pins and SHAs: [docs/contributing.md](docs/contributing.md). All release downloads: [docs/releases.md](docs/releases.md).

## Documentation

| Doc | Description |
| --- | --- |
| [Getting started](docs/getting-started.md) | Build, NCTL defaults, first run |
| [Architecture](docs/architecture.md) | How lib / CLI / WASM sit on the SDK |
| [docs/cep18/](docs/cep18/) · [cep78/](docs/cep78/) · [cep85/](docs/cep85/) | Per-CEP guides |
| [CLI](docs/cli.md) · [WASM / TS](docs/wasm-ts.md) | Surfaces |
| [Testing](docs/testing.md) · [CI / CD](docs/ci.md) · [Releases](docs/releases.md) · [Docker](docs/docker.md) | Verify and ship |
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
