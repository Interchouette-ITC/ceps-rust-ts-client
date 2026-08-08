# ceps-rust-ts-client

One **CEP client** for Casper **CEP-18**, **CEP-78**, and **CEP-85**: a **Rust library**, a **`ceps` CLI**, and a **thin WASM pack** of that same library for JavaScript.

It replaces the separate TypeScript **`client-js`** packages that lived next to each CEP contract. Same CEP verbs (`install` → bind hash → query / mint / transfer), one implementation.

```text
                    ┌─→  Rust apps:   ceps-client (native crate)
CEP client-js ×3 ──┼─→  Shell:       ceps CLI
                    └─→  JS apps:     ceps-wasm
                                      ├─ make nodejs → pkg-nodejs  (Node)
                                      └─ make web    → pkg        (browser)
```

## What “thin WASM” means

Yes: the library can be loaded from JS.

| Pack | Command | Load in |
| --- | --- | --- |
| `ceps-wasm/pkg-nodejs/` | `make nodejs` | **Node.js** (`import { Cep18Client } from "…"` ) |
| `ceps-wasm/pkg/` | `make web` | **Browsers** (bundler / webpack / vite) |

That pack is **`ceps-client` compiled with `wasm-bindgen`**. Same class names (`Cep18Client`, …). It is **not** how you pull latest CEP **contracts**.

| Name | Meaning |
| --- | --- |
| **`ceps-wasm`** | Off-chain **client** for Node/web |
| **`make wasm-from-ceps`** | Copies on-chain **contract** `.wasm` from demo tip repos into `tests/wasm/` for `install(...)` |

## Surfaces

| Surface | What it is | Who uses it |
| --- | --- | --- |
| **`ceps-client`** | Native Rust library (full CEP API) | Rust apps, examples, tests |
| **`ceps` CLI** | Binary on top of `ceps-client` | Shell / scripts |
| **`ceps-wasm`** | Same CEP API as WASM + JS glue | Node (`pkg-nodejs`) and web (`pkg`) |

## What you can do

| CEP | Standard | Typical flow |
| --- | --- | --- |
| **18** | Fungible token | install → bind hash → `name` / `balance_of` → `transfer` / `mint` / `burn` |
| **78** | Enhanced NFT | install → bind hash → `mint` → `owner_of` / `balance_of` |
| **85** | Multi-token | install → bind hash → `mint` / `burn` → `balance_of(account, id)` |

Defaults talk to local NCTL (`http://127.0.0.1:11101`, SSE `…:18101/events`, chain `casper-net-1`).

## Usage (Rust)

Needs a running node, a secret-key PEM, and **contract** WASM bytes (your own build, or the demo tips via `make wasm-from-ceps`).

### CEP-18 - fungible

```text
install contract wasm → named keys cep18_contract_hash_* / package_*
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
install contract wasm → named keys cep78_contract_hash_* / package_*
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
install contract wasm → named keys cep85_contract_hash_* / package_*
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

### Try an install end-to-end (Rust)

```bash
make prepare && make build
make wasm-from-ceps          # copy demo tip contract WASMs into tests/wasm/
# NCTL running + SECRET_KEY_USER_1 set to a PEM:
cargo run -p ceps-client --example cep18_install
```

More setup: [docs/getting-started.md](docs/getting-started.md).

## Usage (Node / `ceps-wasm`)

Build the client pack, then import the same CEP class names from JS.

```bash
make nodejs    # → ceps-wasm/pkg-nodejs
# or: make web → ceps-wasm/pkg  (browsers)
# or: make pack  (both)
```

### What `ceps-wasm` exposes today

| Client | Bound methods (JS names) |
| --- | --- |
| CEP-18 | `setContractHash`, `install`, `name`, `symbol`, `balanceOf`, URL getters |
| CEP-78 | `setContractHash`, `install`, `collectionName`, `balanceOf`, … |
| CEP-85 | `setContractHash`, `install`, `collectionName`, `balanceOf`, … |

Full mutate/query parity is on Rust `ceps-client`; the WASM surface is the thin JS entry for the same clients.

### Node.js - query after bind

```js
import { Cep18Client } from "./ceps-wasm/pkg-nodejs/ceps_wasm.js";
// after `make nodejs`; or depend on the packed package name `ceps-wasm`

const client = new Cep18Client(
  "http://127.0.0.1:11101",
  "http://127.0.0.1:18101/events",
  "casper-net-1",
  0, // verbosity: 0=low
);

client.setContractHash(contractHash, packageHash);
const name = await client.name();
const bal = await client.balanceOf("account-hash-…");
console.log({ name, bal });
```

### Node.js - install (contract bytes in, tx hash out)

```js
import { readFileSync } from "node:fs";
import { Cep18Client } from "./ceps-wasm/pkg-nodejs/ceps_wasm.js";

const client = new Cep18Client("http://127.0.0.1:11101", "http://127.0.0.1:18101/events", "casper-net-1", 0);
const contractWasm = new Uint8Array(readFileSync("tests/wasm/cep18/cep18.wasm"));
const secretPem = readFileSync("secret_key.pem", "utf8");

// install(name, symbol, decimals, totalSupply, eventsMode?, contractWasm, secretPem, payment, wait?)
const resultJson = await client.install(
  "MyToken",
  "MTK",
  9,
  "1000000000",
  2, // CES events mode
  contractWasm,
  secretPem,
  "400000000000",
  true,
);
const { transactionHash } = JSON.parse(resultJson);
// then resolve installer named keys and client.setContractHash(...)
```

Smoke test: `make ts-test`. Full notes: [docs/wasm-ts.md](docs/wasm-ts.md).

## Contract WASMs (on-chain demos)

This client is **not** a contract repo. You pass on-chain `.wasm` into `install`. For local demos and CI, we stage fresh builds from short-lived **demo tip** forks (branch `ceps-client-test`). They exist so examples and tests have current entity-era contracts; they are not a claim of “the” upstream CEP tip forever.

This client (and these demo tips) are headed to the **Interchouette-ITC** org; URLs below are the sources **today**:

| CEP | Demo tip repo (now) | Branch | What you get |
| --- | --- | --- | --- |
| 18 | [gRoussac/cep18](https://github.com/gRoussac/cep18) | `ceps-client-test` | Fungible contract WASM |
| 78 | [gRoussac/cep-78-enhanced-nft](https://github.com/gRoussac/cep-78-enhanced-nft) | `ceps-client-test` | NFT + session WASMs |
| 85 | [gRoussac/cep-85](https://github.com/gRoussac/cep-85) | `ceps-client-test` | Multi-token WASM |

```bash
# after checking out those tips and building contracts there:
make wasm-from-ceps   # → tests/wasm/{cep18,cep78,cep85}/
```

Override checkout roots with `CEP18_PRODUCT` / `CEP78_PRODUCT` / `CEP85_PRODUCT`. Pins and SHAs: [docs/contributing.md](docs/contributing.md).

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
