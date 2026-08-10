# Documentation hub

Unified **Rust** CEP client for Casper (**CEP-18**, **CEP-78**, **CEP-85**, with **CEP-95** supported), with JS packs (`ceps-client-wasm`) and a clap CLI (`ceps-client-cli`).

**CEP-78** is the first-class enhanced NFT surface in this client. **CEP-95** is a supported simpler ERC-721-shaped NFT API (Odra tip); it does not replace CEP-78.

## Quick links

| Page                                  | Content                                    |
| ------------------------------------- | ------------------------------------------ |
| [Getting started](getting-started.md) | Install, NCTL defaults, first client       |
| [Architecture](architecture.md)       | lib ↔ CLI ↔ WASM ↔ SDK                     |
| [SDK dependency](sdk.md)              | Features, local path, CI pin, upgrades     |
| [CLI](cli.md)                         | Global flags and subcommands               |
| [MCP](mcp.md)                         | `ceps-rust-ts-client-mcp` agent tools              |
| [WASM / TS](wasm-ts.md)               | `ceps-client-wasm` pack and Vitest         |
| [Testing](testing.md)                 | Unit / integration / CLI smoke / examples  |
| [CI / CD](ci.md)                      | Gates, Hub/GHCR images, GitHub Releases    |
| [Release artefacts](releases.md)      | Download CLI, client packs, demo contracts |
| [Docker](docker.md)                   | CLI image tags and registries              |
| [Contributing](contributing.md)       | Demo tip WASMs, lint, SDK pin              |
| [SECURITY.md](SECURITY.md)            | Keys, trust boundary, reporting            |
| [CHANGELOG.md](CHANGELOG.md)          | Semver notes                               |

## CEP closets

| Closet                | Standard                                   |
| --------------------- | ------------------------------------------ |
| [docs/cep18/](cep18/) | Fungible token                             |
| [docs/cep78/](cep78/) | Enhanced NFT (first-class)                 |
| [docs/cep85/](cep85/) | Multi-token (CEP-85; repo name `cep-1155`) |
| [docs/cep95/](cep95/) | Simpler ERC-721-shaped NFT (supported; Odra tip) |

Each closet is self-contained: overview, quickstart, mutations, queries, errors, CLI, API.

## Generated API

- Rust: `make doc` → rustdoc under `docs/api-rust/` (`ceps_client/`)
- WASM: typings ship with `make nodejs` as `ceps-client-wasm/pkg-nodejs/ceps_client_wasm.d.ts` (see [api-wasm/README.md](api-wasm/README.md))

## License / security

GPL-3.0. See [LICENSE](../LICENSE) and [SECURITY.md](SECURITY.md).
