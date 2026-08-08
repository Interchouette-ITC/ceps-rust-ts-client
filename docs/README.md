# Documentation hub

Unified **Rust** CEP client for Casper (**CEP-18**, **CEP-78**, **CEP-85**), with a thin WASM/TS surface and a clap CLI (`ceps`).

## Quick links

| Page | Content |
| --- | --- |
| [Getting started](getting-started.md) | Install, NCTL defaults, first client |
| [Architecture](architecture.md) | lib ↔ CLI ↔ WASM ↔ SDK |
| [SDK dependency](sdk.md) | Features, local path, CI pin, upgrades |
| [CLI](cli.md) | Global flags and subcommands |
| [WASM / TS](wasm-ts.md) | `ceps-wasm` pack and Vitest |
| [Testing](testing.md) | Unit / integration / CLI smoke / examples |
| [CI / CD](ci.md) | Gates, Hub/GHCR images, GitHub Releases |
| [Docker](docker.md) | CLI image tags and registries |
| [Contributing](contributing.md) | Tip checkout, `wasm-from-ceps`, SDK pin |
| [SECURITY.md](SECURITY.md) | Keys, trust boundary, reporting |
| [CHANGELOG.md](CHANGELOG.md) | Semver notes |

## CEP closets

| Closet | Standard |
| --- | --- |
| [docs/cep18/](cep18/) | Fungible token |
| [docs/cep78/](cep78/) | Enhanced NFT |
| [docs/cep85/](cep85/) | Multi-token (CEP-85; repo name `cep-1155`) |

Each closet is self-contained: overview, quickstart, mutations, queries, errors, CLI, API.

## Generated API

- Rust: `make doc` → rustdoc under `docs/api-rust/` (`ceps_client/`)
- WASM: typings ship with `make nodejs` as `ceps-wasm/pkg-nodejs/ceps_wasm.d.ts` (see [api-wasm/README.md](api-wasm/README.md))

## License / security

GPL-3.0. See [LICENSE](../LICENSE) and [SECURITY.md](SECURITY.md).
