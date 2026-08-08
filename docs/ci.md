# CI / CD

Strategy mirrors [`casper-rust-wasm-sdk`](https://github.com/casper-ecosystem/casper-rust-wasm-sdk) `.github/workflows`: a strict PR gate, a heavier overnight job, and Pages for docs. No Hub images or GitHub Releases yet (those land when npm/crates publish is intentional).

## Workflows

| Workflow | Trigger | Role |
| --- | --- | --- |
| `ci-test.yml` | push / PR to `dev`, `ceps-client-test`, `main` | Lint, unit, NCTL live integration, CLI smoke |
| `nightly-test.yml` | cron `0 3 * * *` + `workflow_dispatch` | Same gate + `cargo audit` + `make nodejs` + Vitest |
| `pages.yml` | push to `dev` / `ceps-client-test` + `workflow_dispatch` | `make doc` → GitHub Pages (`docs/`) |

## Checkout layout (Actions)

Path deps and `make wasm-from-ceps` expect siblings next to this repo:

```text
$GITHUB_WORKSPACE/
  ceps-rust-ts-client/   # this repository
  rustSDK/               # casper-rust-wasm-sdk @ pin
  cep-18/                # tip branch (prebuilt tests/wasm)
  cep-78-enhanced-nft/
  cep-1155/              # remote gRoussac/cep-85
```

Cargo `path = "../rustSDK"` and Makefile `CEP*_PRODUCT` defaults resolve from that tree.

## Pins

| Dependency | CI source | Ref |
| --- | --- | --- |
| SDK | `casper-ecosystem/casper-rust-wasm-sdk` | tag `2.2.0` (bump with SDK line) |
| CEP-18 tip | `gRoussac/cep18` | `ceps-client-test` |
| CEP-78 tip | `gRoussac/cep-78-enhanced-nft` | `ceps-client-test` |
| CEP-85 tip | `gRoussac/cep-85` | `ceps-client-test` |
| NCTL | `interchouette/casper-nctl-2-docker:dev` | Docker Hub |

Contract WASMs are **copied** from each tip’s `tests/wasm/` (`make wasm-from-ceps`). CI does not rebuild CEP contracts (avoids nightly toolchains on every PR).

## Secrets / keys

NCTL `:dev` is started with host mounts under `ceps-rust-ts-client/assets/{users,faucet}`. After boot, `SECRET_KEY_USER_1` / `SECRET_KEY_USER_2` are exported from those PEMs (same pattern as the SDK CI). Those keys are public NCTL fixtures.

## Local parity

```bash
make check-lint
make unit-test
# with sibling NCTL + tips:
make wasm-from-ceps
make integration-test
make e2e-test
make nodejs && make ts-test   # nightly extras
make doc                      # Pages input
```
