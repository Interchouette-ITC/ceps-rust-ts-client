# ceps-rust-ts-client

Unified Rust (+ thin WASM) client for Casper CEPs (18 / 78 / 85), built on [`casper-rust-wasm-sdk`](https://github.com/casper-ecosystem/rustSDK).

## Quick start

```bash
make help
make prepare
make build
make check-lint
make test
```

## Make (primary entry)

| Target | Purpose |
| --- | --- |
| `make build` / `check` / `check-lint` | Native workspace |
| `make test` | Unit + integration |
| `make pack` / `web` / `nodejs` | wasm-pack |
| `make run-cli` | CLI binary (`console` crate today; rename → `cli`) |
| `make nctl-start` / `nctl-status` | Forward to sibling `casper-nctl-2-docker` (profile `dev`) |
| `make sdk-mcp-http` | SDK MCP HTTP on `:5790` |
| `make wasm-from-ceps` | Stage contract WASMs from sibling CEP repos |

Agents driving NCTL/SDK should use Cursor MCP (`nctl_*` / `sdk_*`), not invent docker compose in this repo. See [`.cursor/README.md`](.cursor/README.md).

## Layout

| Path | Role |
| --- | --- |
| `common/` | Shared CEP client library (→ `ceps-client`) |
| `ceps-ts-client/` | wasm-bindgen bindings (→ `ceps-wasm`) |
| `console/` | CLI binary (→ `cli`) |
| `tests/rust/` | Integration tests |
| `tests/ts/` | Vitest against packed nodejs WASM |

## Remotes / siblings

App git: `git@github.com:gRoussac/ceps-rust-ts-client.git`

Expected siblings (Make / MCP wrappers):

- `../rustSDK`
- `../casper-nctl-2-docker`
- `../cep-18`, `../cep-78-enhanced-nft`, `../cep-1155`
