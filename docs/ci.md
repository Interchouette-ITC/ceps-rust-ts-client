# CI / CD

Quality gate on NCTL live tests; release ships the `ceps` CLI image and GitHub Release artefacts (binary + WASM packs). crates.io / npm publish are out of scope while the workspace path-depends on `casper-rust-wasm-sdk` (see [sdk.md](sdk.md)).

## Flow

```text
push ceps-client-test  →  ci-test (+ hub-images-dev :dev)
nightly-test green     →  release-github-preview (dev-preview Pre-release + assets)
GitHub Release vX.Y.Z  →  release-github-stable (assets) + hub-images-release (:X.Y.Z :latest :dev)
push tip docs          →  pages
```

## Workflows

| Workflow                     | Trigger                                                  | Role                                               |
| ---------------------------- | -------------------------------------------------------- | -------------------------------------------------- |
| `ci-test.yml`                | push / PR to `dev`, `ceps-client-test`, `main`           | Lint, unit, NCTL live integration, CLI smoke       |
| `nightly-test.yml`           | cron `0 3 * * *` + `workflow_dispatch`                   | Same gate + `cargo audit` + `make nodejs` + Vitest |
| `pages.yml`                  | push to `dev` / `ceps-client-test` + `workflow_dispatch` | `make doc` → GitHub Pages (`docs/`)                |
| `hub-images-dev.yml`         | push `ceps-client-test` + `workflow_dispatch`            | CLI image `:dev` → Hub + GHCR                      |
| `hub-images-release.yml`     | stable Release published + `workflow_dispatch`           | `:semver` `:latest` `:dev`                         |
| `release-github-assets.yml`  | `workflow_call`                                          | CLI + client WASM tarballs + demo contracts + `SHA256SUMS` |
| `release-github-stable.yml`  | Release published (not prerelease)                       | Attach assets to Latest                            |
| `release-github-preview.yml` | nightly success / dispatch                               | Overwrite Pre-release `dev-preview` + assets       |

## Images

Local name: `ceps-rust-ts-client`. Registries (same layout as NCTL):

| Registry        | Image                                           |
| --------------- | ----------------------------------------------- |
| Docker Hub      | `interchouette/ceps-rust-ts-client`             |
| GHCR (personal) | `ghcr.io/groussac/ceps-rust-ts-client`          |
| GHCR (org)      | `ghcr.io/interchouette-itc/ceps-rust-ts-client` |

Build is **runtime-only**: CI builds a stripped `ceps` with the SDK path dep, then [`docker/Dockerfile`](../docker/Dockerfile) copies the binary into `debian:bookworm-slim`.

```bash
make release-cli-bin
make docker-build IMAGE_TAG=local
# after docker login Hub + GHCR:
make docker-push IMAGE_TAG=dev
```

| Event                       | Tags                        |
| --------------------------- | --------------------------- |
| Tip push `ceps-client-test` | `:dev`                      |
| Stable Release `vX.Y.Z`     | `:X.Y.Z`, `:latest`, `:dev` |

Stable tag (without `v`) **must** equal `[workspace.package].version` in root `Cargo.toml` or the release image job fails.

## Release artefacts

Attached to each GitHub Release (stable or `dev-preview`). Full fetch guide: [releases.md](releases.md).

| Asset | Contents |
| --- | --- |
| `ceps-{label}-linux-x86_64` | Stripped CLI |
| `ceps-client-wasm-nodejs-{label}.tgz` | Client pack for Node (`pkg-nodejs`) |
| `ceps-client-wasm-web-{label}.tgz` | Client pack for browsers (`pkg`) |
| `ceps-contracts-{label}.tgz` | Demo tip on-chain WASMs (`cep18` / `cep78` / `cep85`) |
| `ceps-contracts-{label}.MANIFEST.txt` | Manifest |
| `SHA256SUMS` | Checksums |

`{label}` is the tag without a leading `v` (e.g. `1.0.0` or `dev-preview`).

## Secrets (repo Settings → Secrets and variables → Actions)

Reuse these Actions secret names:

| Secret | Purpose |
| --- | --- |
| `DOCKER_USERNAME` | Docker Hub login |
| `DOCKER_PASSWORD` | Docker Hub login |
| `GHCR_USERNAME` | GHCR login |
| `GHCR_PAT` | GHCR PAT with package write to `groussac` and `interchouette-itc` |

`GITHUB_TOKEN` (Actions default) uploads release assets and moves the `dev-preview` tag. After the first GHCR push, set package visibility public (or grant org access) for both namespaces.

NCTL live CI also mounts `assets/{users,faucet}` and exports `SECRET_KEY_USER_1` / `SECRET_KEY_USER_2` from those PEMs (public NCTL fixtures; not repo secrets).

`ci-test` / `nightly-test` check out the SDK pin and CEP tip forks into the Actions workspace so path deps and `make wasm-from-ceps` resolve. Release and hub image jobs check out **client + SDK only** (no tip WASMs required for CLI/WASM pack builds).

## Pins

| Dependency | CI source                                | Ref                              |
| ---------- | ---------------------------------------- | -------------------------------- |
| SDK        | `casper-ecosystem/casper-rust-wasm-sdk`  | tag `2.2.0` (bump with SDK line) |
| CEP-18 tip | `gRoussac/cep18` | `ceps-client-test` (demo tip) |
| CEP-78 tip | `gRoussac/cep-78-enhanced-nft` | `ceps-client-test` (demo tip) |
| CEP-85 tip | `gRoussac/cep-85` | `ceps-client-test` (demo tip) |
| NCTL | `interchouette/casper-nctl-2-docker:dev` | Docker Hub |

Demo tip WASMs are **copied** from each fork’s `tests/wasm/` (`make wasm-from-ceps`). CI does not rebuild CEP contracts. See [contributing.md](contributing.md).

## Same-day stable cut

1. Confirm `[workspace.package] version` (currently `1.0.0`).
2. Fill the four Hub/GHCR secrets; ensure the PAT can push both GHCR namespaces.
3. Push tip (or merge) so `ci-test` is green and `hub-images-dev` can publish `:dev`.
4. Create a GitHub Release **`v1.0.0`** (not prerelease) from the UI or a PAT on that commit.
5. Wait for `release-github-stable` (assets) and `hub-images-release` (`:1.0.0` `:latest` `:dev`).

## Local parity

```bash
make check-lint
make unit-test
# with NCTL + tip WASMs staged:
make wasm-from-ceps
make integration-test
make e2e-test
make nodejs && make ts-test   # nightly extras
make doc                      # Pages input
make release-cli-bin && make docker-build IMAGE_TAG=local
```
