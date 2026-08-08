# ceps-rust-ts-client

Unified Rust (+ thin WASM/CLI) client for Casper **CEP-18**, **CEP-78**, and **CEP-85**, built on [`casper-rust-wasm-sdk`](https://github.com/casper-ecosystem/casper-rust-wasm-sdk).

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

## Layout

| Path           | Role                                                             |
| -------------- | ---------------------------------------------------------------- |
| `ceps-client/` | Native CEP library (`Cep18Client`, `Cep78Client`, `Cep85Client`) |
| `cli/`         | Clap binary `ceps`                                               |
| `ceps-wasm/`   | Thin `wasm-bindgen` CEP exports                                  |
| `tests/rust/`  | Integration tests (NCTL live cases)                              |
| `tests/ts/`    | Vitest smoke against `ceps-wasm` `pkg-nodejs`                    |
| `tests/wasm/`  | Staged contract WASMs from tip builds                            |
| `docs/`        | Hub + one closet per CEP                                         |

## Contract tips

Develop against tip branch **`ceps-client-test`** on the CEP forks, build contracts there, then `make wasm-from-ceps`. Details: [docs/contributing.md](docs/contributing.md).

## Make

| Target                                             | Purpose                      |
| -------------------------------------------------- | ---------------------------- |
| `make build` / `check` / `check-lint`              | Native workspace             |
| `make unit-test` / `integration-test` / `e2e-test` | Tests                        |
| `make pack` / `nodejs`                             | wasm-pack                    |
| `make run-cli`                                     | `ceps` binary                |
| `make nctl-start` / `nctl-status`                  | Local NCTL (`dev` profile)   |
| `make wasm-from-ceps`                              | Stage tip WASMs              |
| `make release-cli-bin` / `docker-build`            | Stripped CLI + image         |
| GitHub Actions                                     | See [docs/ci.md](docs/ci.md) |

Agents driving NCTL/SDK should use Cursor MCP (`nctl_*` / `sdk_*`).
