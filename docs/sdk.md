# SDK dependency

This workspace depends on [`casper-rust-wasm-sdk`](https://github.com/casper-ecosystem/casper-rust-wasm-sdk) for RPC, transactions, install, query, wait, and CES helpers. CEP-specific APIs live in `ceps-client`.

## Features

Enabled in root [`Cargo.toml`](../Cargo.toml):

```toml
casper-rust-wasm-sdk = { git = "https://github.com/casper-ecosystem/casper-rust-wasm-sdk", branch = "dev", default-features = false, features = [
  "transaction",
  "contract",
  "helpers",
  "watcher",
  "SSE",
] }
```

Deliberately omit SDK feature `js` (wasm-bindgen / js-sys surface). This workspace uses the SDK as a Rust library; CEP JS packs do their own bindgen in `ceps-client-wasm`. Enabling SDK `js` would re-export SDK classes into the CEP `.wasm` / `.d.ts`.

Workspace `[patch.crates-io]` keeps Casper crates aligned with that SDK line.

For a local SDK checkout during tip work, override with a Cargo `[patch]` pointing at your tree (do not commit that patch unless the PR is intentional).

## CI / release

Cargo fetches the git dep at branch **`dev`**. Bump that ref when you upgrade the SDK (see [ci.md](ci.md)).

Release artefacts (CLI binary, `ceps-client-wasm` packs) build against that pin. The Docker image ships only the stripped `ceps-client-cli` binary.

## Upgrade checklist

1. Bump the git `branch` / `rev` in root `Cargo.toml` (and refresh `Cargo.lock`).
2. Align `[patch.crates-io]` if Casper node/client crates moved.
3. `make check-lint && make unit-test` (and NCTL live tests when install/query behaviour changed).
4. Update the pin table in [ci.md](ci.md) / [contributing.md](contributing.md).
5. `make pack && make ts-test` if the bindgen surface shifted.

## `ceps-client-wasm`

JS packs for the CEP **client** (not contracts). Committed under `ceps-client-wasm/pkg` and `pkg-nodejs`; rebuild with `make nodejs` / `make web`. With SDK `js` omitted, the generated `.d.ts` exports CEP clients only (`CEP18Client` / `CEP78Client` / `CEP85Client` / `CEP95Client` and related helpers). For raw RPC / transfers from JS, depend on the SDK packs (`casper-rust-wasm-sdk`) separately.
