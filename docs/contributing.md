# Contributing

## CEP tip checkout

Canonical tip assembly lives in `.cursor/plans/cep_test_tip_branches.plan.md`. Branch name on remote `dev` (gRoussac forks): **`ceps-client-test`**.

```bash
for repo in cep-18 cep-78-enhanced-nft cep-1155; do
  git -C "../$repo" fetch dev
  git -C "../$repo" checkout ceps-client-test
  git -C "../$repo" pull --ff-only dev ceps-client-test
done
```

Build contracts in each sibling (`make build-contract` or the repo Makefile), then:

```bash
make wasm-from-ceps
```

### Tip SHAs (documented for this client line)

Recorded while shipping the unified client on branch `ceps-client-test`:

| Product | Path | Branch | SHA |
| --- | --- | --- | --- |
| CEP-18 | `../cep-18` | `ceps-client-test` | `3e92164` |
| CEP-78 | `../cep-78-enhanced-nft` | `ceps-client-test` | `d650a03` |
| CEP-85 | `../cep-1155` | `ceps-client-test` | `9d437cd` |

Re-pin these when intentionally rebasing tips. Prefer sibling `tests/wasm/` artifacts over stale `target/` builds (`make wasm-from-ceps` prefers `tests/wasm`).

## SDK pin

Local path dependency: `../rustSDK` with features `transaction`, `contract`, `helpers`, `watcher`, `SSE`.

CI uses git tag `2.2.0` of `casper-ecosystem/casper-rust-wasm-sdk` and mirrors `[patch.crates-io]` from this workspace.

The SDK TUI (`examples/tui` in rustSDK) remains SDK-owned. Wire CEP installs by calling `ceps-client` from a TUI action or companion binary in that tree; this repo exposes the library surface (`Cep18Client` / `Cep78Client` / `Cep85Client`) for that integration.

## Lint gate

```bash
make check-lint
```

Treat lint failures as blockers before push/PR.
