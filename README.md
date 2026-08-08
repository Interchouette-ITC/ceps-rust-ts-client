# ceps-rust-ts-client

Unified **Rust** (+ thin WASM / CLI) client for Casper **CEP-18**, **CEP-78**, and **CEP-85**, built on [`casper-rust-wasm-sdk`](https://github.com/casper-ecosystem/casper-rust-wasm-sdk).

This repository does **not** vendor or embed SDK WASM packs (that is what [casper-deployer](https://github.com/Interchouette-ITC/casper-deployer#sdk-upgrade-note) does under `casper-rust-wasm-sdk/`). Here the SDK is a normal Cargo dependency: path checkout locally, pinned git tag in CI. See [docs/sdk.md](docs/sdk.md).

## What it does

- Install and drive **CEP-18** (fungible), **CEP-78** (enhanced NFT), **CEP-85** (multi-token) contracts
- Share one transport core (`CepCore`) for RPC/SSE, install, entrypoint call, query, wait, CES-friendly errors
- Ship a clap CLI (`ceps`) for status and common queries
- Ship thin `ceps-wasm` bindings for Node/browser CEP APIs (not a full SDK re-export)
- Run live integration against NCTL; publish CLI images to Hub + GHCR and GitHub Release artefacts

## Stack

| Layer | Tech |
| --- | --- |
| Library | `ceps-client` (Rust, edition 2021) |
| CLI | `ceps` (`cli` crate, clap) |
| WASM | `ceps-wasm` (`wasm-bindgen` / `wasm-pack`) |
| Casper | `casper-rust-wasm-sdk` features `transaction`, `contract`, `helpers`, `watcher`, `SSE` |
| Tests | Rust unit + NCTL live; Vitest on `pkg-nodejs` |
| Images | `interchouette/ceps-rust-ts-client` + GHCR (see [docs/docker.md](docs/docker.md)) |

## Repository layout

| Path | Role |
| --- | --- |
| `ceps-client/` | Native CEP library (`Cep18Client`, `Cep78Client`, `Cep85Client`) |
| `cli/` | Clap binary `ceps` |
| `ceps-wasm/` | Thin CEP-only WASM exports |
| `docker/` | Runtime Dockerfile for the CLI binary |
| `tests/rust/` | Integration tests (NCTL live cases) |
| `tests/ts/` | Vitest smoke against `pkg-nodejs` |
| `tests/wasm/` | Staged contract WASMs from tip builds |
| `docs/` | Hub, CEP closets, CI, security, SDK notes |

## Quick start

```bash
make help
make prepare
make build
make check-lint
make unit-test
cargo run -p cli -- --help
cargo run -p cli -- status
```

Library sketch:

```rust
use ceps_client::{Cep18Client, Verbosity};

let client = Cep18Client::new(
    "http://127.0.0.1:11101",
    Some("http://127.0.0.1:18101/events".into()),
    Some("casper-net-1".into()),
    Some(Verbosity::Low),
)?;
```

Defaults match NCTL `dev` (RPC `11101`, SSE `18101/events`, chain `casper-net-1`). Full walkthrough: [docs/getting-started.md](docs/getting-started.md).

## Contract tips

Develop against tip branch **`ceps-client-test`** on the CEP forks, build contracts there, then `make wasm-from-ceps`. Details: [docs/contributing.md](docs/contributing.md).

## Make

| Target | Purpose |
| --- | --- |
| `make build` / `check` / `check-lint` | Native workspace |
| `make unit-test` / `integration-test` / `e2e-test` | Tests |
| `make pack` / `nodejs` / `web` | wasm-pack |
| `make run-cli` | `ceps` binary |
| `make nctl-start` / `nctl-status` | Local NCTL (`dev` profile) |
| `make wasm-from-ceps` | Stage tip WASMs |
| `make release-cli-bin` / `docker-build` | Stripped CLI + image |
| `make doc` | rustdoc → `docs/api-rust/` |

Agents driving NCTL/SDK should use Cursor MCP (`nctl_*` / `sdk_*`).

## Docker

```bash
make release-cli-bin
make docker-build IMAGE_TAG=local
docker run --rm ceps-rust-ts-client:local --help

# published (after CI secrets + push):
docker pull interchouette/ceps-rust-ts-client:dev
```

Tags, Hub, and GHCR: [docs/docker.md](docs/docker.md). Release strategy: [docs/ci.md](docs/ci.md).

## Documentation

| Doc | Description |
| --- | --- |
| [docs/README.md](docs/README.md) | Documentation hub |
| [Getting started](docs/getting-started.md) | Build, NCTL defaults, first client |
| [Architecture](docs/architecture.md) | lib ↔ CLI ↔ WASM ↔ SDK |
| [SDK dependency](docs/sdk.md) | Path dep vs deployer vendoring; upgrade pin |
| [CLI](docs/cli.md) | Global flags and subcommands |
| [WASM / TS](docs/wasm-ts.md) | `ceps-wasm` pack and Vitest |
| [Testing](docs/testing.md) | Unit / live / e2e / examples |
| [CI / CD](docs/ci.md) | Gates, Hub/GHCR, GitHub Releases |
| [Docker](docs/docker.md) | CLI image tags and registries |
| [Contributing](docs/contributing.md) | Tip WASMs, lint, SDK pin |
| [SECURITY.md](docs/SECURITY.md) | Keys, trust boundary, reporting |
| [CHANGELOG.md](docs/CHANGELOG.md) | Semver notes |
| [docs/cep18/](docs/cep18/) | CEP-18 closet |
| [docs/cep78/](docs/cep78/) | CEP-78 closet |
| [docs/cep85/](docs/cep85/) | CEP-85 closet |
| `make doc` | Generated rustdoc under `docs/api-rust/` |

## License / security

GPL-3.0. See [LICENSE](LICENSE) and [docs/SECURITY.md](docs/SECURITY.md).
