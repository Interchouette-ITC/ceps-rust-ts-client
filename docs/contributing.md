# Contributing

## CEP tip checkout

Canonical tip assembly lives in `.cursor/plans/cep_test_tip_branches.plan.md`. Tip branch on the gRoussac forks: **`ceps-client-test`**.

1. Check out that branch in each CEP tip product you use for contract builds.
2. Build contracts there (`make build-contract` or the repo Makefile).
3. From this repo:

```bash
make wasm-from-ceps
```

Override tip roots with `CEP18_PRODUCT`, `CEP78_PRODUCT`, `CEP85_PRODUCT` if defaults do not match your layout.

### Tip SHAs (documented for this client line)

Recorded while shipping the unified client on branch `ceps-client-test`:

| Product                                 | Branch             | SHA       |
| --------------------------------------- | ------------------ | --------- |
| CEP-18 (`gRoussac/cep18`)               | `ceps-client-test` | `3e92164` |
| CEP-78 (`gRoussac/cep-78-enhanced-nft`) | `ceps-client-test` | `d650a03` |
| CEP-85 (`gRoussac/cep-85`)              | `ceps-client-test` | `9d437cd` |

Re-pin these when intentionally rebasing tips. Prefer each tip’s `tests/wasm/` artifacts over stale `target/` builds (`make wasm-from-ceps` prefers `tests/wasm`).

## SDK pin

Path dependency on [`casper-rust-wasm-sdk`](https://github.com/casper-ecosystem/casper-rust-wasm-sdk) with features `transaction`, `contract`, `helpers`, `watcher`, `SSE` (see root `Cargo.toml`). This repo does **not** embed SDK packs; see [sdk.md](sdk.md).

CI uses git tag `2.2.0` of `casper-ecosystem/casper-rust-wasm-sdk` and mirrors `[patch.crates-io]` from this workspace.

The SDK TUI remains SDK-owned. Wire CEP installs by calling `ceps-client` from a TUI action or companion binary in that tree; this repo exposes the library surface (`Cep18Client` / `Cep78Client` / `Cep85Client`) for that integration.

## Lint gate

```bash
make check-lint
```

Treat lint failures as blockers before push/PR.

## CI

See [ci.md](ci.md). PR gate is `.github/workflows/ci-test.yml` (NCTL + tip WASMs). Overnight extras live in `nightly-test.yml`. Pages deploys from `pages.yml`. Hub/GHCR images and GitHub Release assets: `hub-images-*.yml` and `release-github-*.yml` (secrets checklist in ci.md).
