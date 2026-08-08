# ceps-rust-ts-client

One **Rust** client for Casper **CEP-18**, **CEP-78**, and **CEP-85**, with a **`ceps-client-cli`** and **WASM packs** so the same client can run from Node or the browser.

It replaces the separate TypeScript **`client-js`** packages that lived next to each CEP contract. Instead of three JS clients, you use one library: `Cep18Client` / `Cep78Client` / `Cep85Client`, on top of [`casper-rust-wasm-sdk`](https://github.com/casper-ecosystem/casper-rust-wasm-sdk).

```text
CEP-18 client-js  ─┐
CEP-78 client-js  ─┼─→  ceps-client (Rust)  +  ceps-client-cli  +  ceps-client-wasm
CEP-85 client-js  ─┘
```

## What you get

<table>
<thead>
<tr>
<th>Piece</th>
<th>Name</th>
<th>Role</th>
<th>In-tree path</th>
</tr>
</thead>
<tbody>
<tr>
<td>Rust library</td>
<td><code>ceps‑client</code></td>
<td>CEP API for native Rust apps</td>
<td><code>ceps-client/</code></td>
</tr>
<tr>
<td>CLI</td>
<td><code>ceps‑client‑cli</code></td>
<td>Status and common queries from the shell</td>
<td><code>ceps-client-cli/</code></td>
</tr>
<tr>
<td>Client JS packs</td>
<td><code>ceps‑client‑wasm</code></td>
<td>Same CEP classes for Node and browsers (replaces per-CEP <code>client-js</code>)</td>
<td><code>ceps-client-wasm/pkg</code> / <code>pkg-nodejs</code></td>
</tr>
<tr>
<td>Demo contract WASMs</td>
<td><code>ceps‑contracts</code></td>
<td>On-chain bytecode for <code>install</code> (demo tips)</td>
<td><code>tests/wasm/{cep18,cep78,cep85}/</code></td>
</tr>
</tbody>
</table>

Release downloads use the same names: `ceps-client-cli-*-linux-x86_64`, `ceps-client-wasm-*.tgz`, and `ceps-contracts-*.tgz`. The first three rows are this client; the last row is sample contract bytecode, not the JS library.

## What you can do

| CEP    | Standard       | Typical flow                                                               |
| ------ | -------------- | -------------------------------------------------------------------------- |
| **18** | Fungible token | install → bind hash → `name` / `balance_of` → `transfer` / `mint` / `burn` |
| **78** | Enhanced NFT   | install → bind hash → `mint` → `owner_of` / `balance_of`                   |
| **85** | Multi-token    | install → bind hash → `mint` / `burn` → `balance_of(account, id)`          |

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
cargo run -p ceps-client-cli -- status
cargo run -p ceps-client-cli -- cep18 info
cargo run -p ceps-client-cli -- cep78 balance --contract-hash <hash> --account <account-hash-…>
cargo run -p ceps-client-cli -- cep85 balance --contract-hash <hash> --account <…> --id 1
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

## Client JS packs (`ceps-client-wasm`)

`ceps-client-wasm` is the **Rust CEP client compiled for JavaScript** (the replacement for per-CEP `client-js`). Use it from Node or the browser for the same `Cep18Client` / `Cep78Client` / `Cep85Client` surface.

It is **not** on-chain contract bytecode (that is `tests/wasm/` / `ceps-contracts-*.tgz`).

| Target | In-tree (committed)            | Typical use                |
| ------ | ------------------------------ | -------------------------- |
| Node   | `ceps-client-wasm/pkg-nodejs/` | Backend / scripts / Vitest |
| Web    | `ceps-client-wasm/pkg/`        | Bundled frontends          |

Rebuild locally with `make pack` (or `make nodejs` / `make web`). Details: [docs/wasm-ts.md](docs/wasm-ts.md) · [docs/releases.md](docs/releases.md).

**Or** fetch a published pack (no local `wasm-pack`):

```bash
TAG=v1.0.0   # or dev-preview
LABEL=${TAG#v}
curl -fsSL -o ceps-client-wasm-nodejs.tgz \
  "https://github.com/gRoussac/ceps-rust-ts-client/releases/download/${TAG}/ceps-client-wasm-nodejs-${LABEL}.tgz"
mkdir -p ceps-client-wasm && tar -xzf ceps-client-wasm-nodejs.tgz -C ceps-client-wasm
# -> ceps-client-wasm/pkg-nodejs/
```

```js
import { Cep18Client } from "ceps-client-wasm"; // file:./ceps-client-wasm/pkg-nodejs after unpack

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

**In-tree:** the last staged demo tip builds ship under `tests/wasm/{cep18,cep78,cep85}/` (same idea as committing `pkg` / `pkg-nodejs`). Use those bytes directly, or refresh with `make wasm-from-ceps`.

**Or** download the release bundle (no tip checkout / no contract build):

```bash
TAG=v1.0.0
LABEL=${TAG#v}
curl -fsSL -o ceps-contracts.tgz \
  "https://github.com/gRoussac/ceps-rust-ts-client/releases/download/${TAG}/ceps-contracts-${LABEL}.tgz"
mkdir -p tests/wasm && tar -xzf ceps-contracts.tgz -C tests/wasm
```

Tips are short-lived entity-era builds for demos/CI, not a claim of "the" upstream CEP tip forever. This client (and these tips) are headed to **Interchouette-ITC**; sources **today**:

| CEP | Demo tip repo (now)                                                             | Branch             | What you get           |
| --- | ------------------------------------------------------------------------------- | ------------------ | ---------------------- |
| 18  | [gRoussac/cep18](https://github.com/gRoussac/cep18)                             | `ceps-client-test` | Fungible contract WASM |
| 78  | [gRoussac/cep-78-enhanced-nft](https://github.com/gRoussac/cep-78-enhanced-nft) | `ceps-client-test` | NFT + session WASMs    |
| 85  | [gRoussac/cep-85](https://github.com/gRoussac/cep-85)                           | `ceps-client-test` | Multi-token WASM       |

```bash
# after checking out those tips and building contracts there:
make wasm-from-ceps   # → tests/wasm/{cep18,cep78,cep85}/
```

Override checkout roots with `CEP18_PRODUCT` / `CEP78_PRODUCT` / `CEP85_PRODUCT`. Pins and SHAs: [docs/contributing.md](docs/contributing.md). All release downloads: [docs/releases.md](docs/releases.md).

## Documentation

| Doc                                                                                                          | Description                         |
| ------------------------------------------------------------------------------------------------------------ | ----------------------------------- |
| [Getting started](docs/getting-started.md)                                                                   | Build, NCTL defaults, first run     |
| [Architecture](docs/architecture.md)                                                                         | How lib / CLI / WASM sit on the SDK |
| [docs/cep18/](docs/cep18/) · [cep78/](docs/cep78/) · [cep85/](docs/cep85/)                                   | Per-CEP guides                      |
| [CLI](docs/cli.md) · [WASM / TS](docs/wasm-ts.md)                                                            | Surfaces                            |
| [Testing](docs/testing.md) · [CI / CD](docs/ci.md) · [Releases](docs/releases.md) · [Docker](docs/docker.md) | Verify and ship                     |
| [Contributing](docs/contributing.md) · [SDK](docs/sdk.md)                                                    | Tips, pins, upgrades                |
| [SECURITY.md](docs/SECURITY.md)                                                                              | Keys and reporting                  |
| `make doc`                                                                                                   | rustdoc → `docs/api-rust/`          |

## Docker

```bash
make release-cli-bin && make docker-build IMAGE_TAG=local
docker run --rm ceps-rust-ts-client:local --help
docker pull interchouette/ceps-rust-ts-client:dev
```

See [docs/docker.md](docs/docker.md).

## License / security

GPL-3.0. See [LICENSE](LICENSE) and [docs/SECURITY.md](docs/SECURITY.md).
