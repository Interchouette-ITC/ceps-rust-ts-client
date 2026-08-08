# SDK dependency

This workspace depends on [`casper-rust-wasm-sdk`](https://github.com/casper-ecosystem/casper-rust-wasm-sdk) for RPC, transactions, install, query, wait, and CES helpers. CEP-specific APIs live in `ceps-client`.

## Features

Enabled in root [`Cargo.toml`](../Cargo.toml):

```toml
casper-rust-wasm-sdk = { path = "../rustSDK", default-features = false, features = [
  "transaction",
  "contract",
  "helpers",
  "watcher",
  "SSE",
] }
```

Point `path` at your SDK checkout (or set `RUSTSDK_PRODUCT` for Make wrappers). Workspace `[patch.crates-io]` keeps Casper crates aligned with that SDK line.

## CI / release

GitHub Actions check out `casper-ecosystem/casper-rust-wasm-sdk` at tag **`v2.2.2`** so the path dependency resolves. Bump that ref when you upgrade the SDK (see [ci.md](ci.md)).

Release artefacts (CLI binary, `ceps-client-wasm` packs) build against that pin. The Docker image ships only the stripped `ceps-client-cli` binary.

## Upgrade checklist

1. Update the local SDK checkout (or the CI `ref:`).
2. Align `[patch.crates-io]` if Casper node/client crates moved.
3. `make check-lint && make unit-test` (and NCTL live tests when install/query behaviour changed).
4. Update the pin table in [ci.md](ci.md) / [contributing.md](contributing.md).
5. `make pack && make ts-test` if the bindgen surface shifted.

## `ceps-client-wasm`

JS packs for the CEP **client** (not contracts). Committed under `ceps-client-wasm/pkg` and `pkg-nodejs`; rebuild with `make nodejs` / `make web`. Generated `.d.ts` may also list transitive SDK symbols from `wasm-bindgen`; those are not a supported re-export. Import `casper-rust-wasm-sdk` directly when you need raw RPC helpers.
