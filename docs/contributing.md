# Contributing

## Contract WASMs for demos and CI

This repo is the **client**. Contract bytecode comes from elsewhere.

For examples, live tests, and CI we use **demo tip** forks with branch **`ceps-client-test`**: fresh entity-era builds so installs work out of the box. They are for demos against this client, not a permanent substitute for ecosystem `casper-ecosystem/*` release tips.

**Without cloning tips:** download `ceps-contracts-*.tgz` from a [GitHub Release](releases.md).

This client and those demo tips are moving under **Interchouette-ITC**; the table is the location **today**:

| CEP | Demo tip (today) | Branch | Recorded SHA |
| --- | --- | --- | --- |
| 18 | [gRoussac/cep18](https://github.com/gRoussac/cep18) | `ceps-client-test` | `3e92164` |
| 78 | [gRoussac/cep-78-enhanced-nft](https://github.com/gRoussac/cep-78-enhanced-nft) | `ceps-client-test` | `d650a03` |
| 85 | [gRoussac/cep-85](https://github.com/gRoussac/cep-85) | `ceps-client-test` | `9d437cd` |

1. Clone/check out each tip at `ceps-client-test`.
2. Build contracts there (`make build-contract` or the repo Makefile).
3. From this client:

```bash
make wasm-from-ceps   # → tests/wasm/{cep18,cep78,cep85}/
```

Override roots with `CEP18_PRODUCT`, `CEP78_PRODUCT`, `CEP85_PRODUCT`. Prefer each tip’s `tests/wasm/` over stale `target/` builds. Re-pin the SHAs when you intentionally rebase a tip.

## SDK pin

Path dependency on [`casper-rust-wasm-sdk`](https://github.com/casper-ecosystem/casper-rust-wasm-sdk) with features `transaction`, `contract`, `helpers`, `watcher`, `SSE` (see root `Cargo.toml` and [sdk.md](sdk.md)).

CI uses git tag `2.2.0` of `casper-ecosystem/casper-rust-wasm-sdk` and mirrors `[patch.crates-io]` from this workspace.

The SDK TUI remains SDK-owned. Wire CEP installs by calling `ceps-client` from a TUI action or companion binary in that tree; this repo exposes the library surface (`Cep18Client` / `Cep78Client` / `Cep85Client`) for that integration.

## Lint gate

```bash
make check-lint
```

Treat lint failures as blockers before push/PR.

## CI

See [ci.md](ci.md). PR gate is `.github/workflows/ci-test.yml` (NCTL + tip WASMs). Overnight extras live in `nightly-test.yml`. Pages deploys from `pages.yml`. Hub/GHCR images and GitHub Release assets: `hub-images-*.yml` and `release-github-*.yml` (secrets checklist in ci.md).
