# SDK dependency

## Do we embed the SDK?

**No.** This product does **not** vendor slim `pkg` / `pkg-nodejs` trees the way [casper-deployer](https://github.com/Interchouette-ITC/casper-deployer#sdk-upgrade-note) does under its `casper-rust-wasm-sdk/` folder.

| Product | How it consumes the SDK |
| --- | --- |
| **casper-deployer** | Vendored slim wasm-pack outputs checked into the app repo; refresh via workflow / manual copy |
| **ceps-rust-ts-client** | Cargo dependency on crate `casper-rust-wasm-sdk` (path locally, git tag in CI) |

CEP logic lives here. Low-level RPC, transaction make/sign/put, install, query, wait, and CES helpers stay in the SDK.

## Local development

Root [`Cargo.toml`](../Cargo.toml) workspace dependency (features enabled):

```toml
casper-rust-wasm-sdk = { path = "../rustSDK", default-features = false, features = [
  "transaction",
  "contract",
  "helpers",
  "watcher",
  "SSE",
] }
```

Point `path` (or `RUSTSDK_PRODUCT` for Make wrappers) at whatever checkout you use. Workspace `[patch.crates-io]` mirrors the SDK line so Casper crates stay coherent.

## CI / release builds

GitHub Actions check out `casper-ecosystem/casper-rust-wasm-sdk` at tag **`2.2.0`** next to this repo so the same path layout resolves. Bump that ref together with local SDK upgrades (see [ci.md](ci.md) pins).

Release artefacts (CLI binary, `ceps-wasm` packs) are built against that pin. The Docker image copies only the stripped `ceps` binary; it does not ship an SDK tree.

## Upgrade checklist

1. Land the desired SDK version in your local checkout (or bump the CI `ref:`).
2. Align `[patch.crates-io]` with that SDK line if Casper node/client crates moved.
3. `make check-lint && make unit-test` (and live tests with NCTL when install/query behaviour changed).
4. Update the pin table in [ci.md](ci.md) / [contributing.md](contributing.md).
5. Rebuild WASM packs (`make pack`) and re-run `make ts-test` if bindgen surface shifted.

## What `ceps-wasm` ships

`make nodejs` / `make web` produce **CEP-facing** bindings. Because the SDK also uses `wasm-bindgen`, generated `.d.ts` may list transitive SDK symbols: treat those as SDK surface, not as a supported re-export from this product. Prefer importing `casper-rust-wasm-sdk` directly when you need raw RPC helpers.
