# Contributing

## Contract WASMs for demos and CI

This repo is the **client**. Contract bytecode comes from elsewhere.

For examples, live tests, and CI we use **demo tip** branches **`ceps-client-test`** on **Interchouette-ITC** contract forks: fresh entity-era builds so installs work out of the box. Ecosystem remotes stay for upstream sync. Personal `gRoussac/*` forks are not used in CI checkouts.

**Without cloning tips:** download `ceps-contracts-*.tgz` from a [GitHub Release](releases.md).

| CEP | Demo tip | Branch | Ecosystem upstream |
| --- | --- | --- | --- |
| 18 | [Interchouette-ITC/cep-18](https://github.com/Interchouette-ITC/cep-18) | `ceps-client-test` | [`casper-ecosystem/cep18`](https://github.com/casper-ecosystem/cep18) (no dash) |
| 78 | [Interchouette-ITC/cep-78-enhanced-nft](https://github.com/Interchouette-ITC/cep-78-enhanced-nft) | `ceps-client-test` | [`casper-ecosystem/cep-78-enhanced-nft`](https://github.com/casper-ecosystem/cep-78-enhanced-nft) |
| 85 | [Interchouette-ITC/cep-85](https://github.com/Interchouette-ITC/cep-85) | `ceps-client-test` | [`casper-ecosystem/cep-85`](https://github.com/casper-ecosystem/cep-85) |
| 95 | [Interchouette-ITC/cep-95](https://github.com/Interchouette-ITC/cep-95) | `ceps-client-test` | Odra tip (no ecosystem contract repo) |

1. Clone/check out each tip at `ceps-client-test` (remotes: `origin` = ITC, `ecosystem` = upstream where applicable, `dev` = personal fork).
2. Build contracts there (`make build-contract` or the repo Makefile).
3. From this client:

```bash
make wasm-from-ceps   # → tests/wasm/{cep18,cep78,cep85,cep95}/
```

Override roots with `CEP18_PRODUCT`, `CEP78_PRODUCT`, `CEP85_PRODUCT`, `CEP95_PRODUCT`. Prefer each tip’s `tests/wasm/` over stale `target/` builds.

## Remotes (this client)

| Remote | Repo |
| ------ | ---- |
| `origin` | [`Interchouette-ITC/ceps-rust-ts-client`](https://github.com/Interchouette-ITC/ceps-rust-ts-client) (push here) |
| `dev` | `gRoussac/ceps-rust-ts-client` (personal fork) |

SSH only (`git@github.com:…`). Day-to-day: `git push origin`.

## SDK pin

Path dependency on [`casper-rust-wasm-sdk`](https://github.com/casper-ecosystem/casper-rust-wasm-sdk) with features `transaction`, `contract`, `helpers`, `watcher`, `SSE` (see root `Cargo.toml` and [sdk.md](sdk.md)).

CI checks out `casper-ecosystem/casper-rust-wasm-sdk` at branch **`dev`** and mirrors `[patch.crates-io]` from this workspace.

The SDK TUI remains SDK-owned. Wire CEP installs by calling `ceps-client` from a TUI action or companion binary in that tree; this repo exposes the library surface (`Cep18Client` / `Cep78Client` / `Cep85Client`) for that integration.

## Lint gate

```bash
make check-lint
```

Treat lint failures as blockers before push/PR.

## CI

See [ci.md](ci.md). PR gate is `.github/workflows/ci-test.yml` (NCTL + tip WASMs). Overnight extras live in `nightly-test.yml`. Pages publishes from `pages.yml`. Hub/GHCR images and GitHub Release assets: `hub-images-*.yml` and `release-github-*.yml` (secrets checklist in ci.md).
