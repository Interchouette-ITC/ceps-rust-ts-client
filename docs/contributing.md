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

Pin tip SHAs in CI/docs when a phase ships so ABI drift is intentional.

## SDK pin

Local path dependency: `../rustSDK` with features `transaction`, `contract`, `helpers`, `watcher`, `SSE`.

CI should use a git/tag pin on the 2.2.x line (same feature set) and mirror `[patch.crates-io]` from the SDK when needed.

## Lint gate

```bash
make check-lint
```

Treat lint failures as blockers before push/PR.
