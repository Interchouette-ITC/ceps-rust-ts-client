# CI / CD

Quality gate on NCTL live tests; release ships the `ceps-rust-ts-client` and `ceps-rust-ts-client-mcp` images plus GitHub Release artefacts (binaries + WASM packs). crates.io / npm publish are out of scope while the workspace path-depends on `casper-rust-wasm-sdk` (see [sdk.md](sdk.md)).

Official repo: [`Interchouette-ITC/ceps-rust-ts-client`](https://github.com/Interchouette-ITC/ceps-rust-ts-client) (`origin`). Personal fork: `gRoussac/ceps-rust-ts-client` (`dev` remote). Push day-to-day work to **`origin`**.

## Flow

```text
push to `dev`         →  ci-test (+ hub-images-dev :dev)
nightly-test green     →  release-github-preview (dev-preview Pre-release + assets)
GitHub Release vX.Y.Z  →  release-github-stable (assets) + hub-images-release (:X.Y.Z :latest :dev)
push tip docs          →  pages
```

## Workflows

| Workflow                     | Trigger                                        | Role                                                       |
| ---------------------------- | ---------------------------------------------- | ---------------------------------------------------------- |
| `ci-test.yml`                | push / PR to `dev`, `main`                     | Lint, unit, NCTL live integration, MCP unit + live, CLI smoke |
| `nightly-test.yml`           | cron `0 3 * * *` + `workflow_dispatch`         | Same gate + `cargo audit` + `make nodejs` + Vitest         |
| `pages.yml`                  | push to `dev` + `workflow_dispatch`            | `make doc` → GitHub Pages (`docs/`)                        |
| `hub-images-dev.yml`         | push `dev` + `workflow_dispatch`               | CLI + MCP images `:dev` → Hub + GHCR               |
| `hub-images-release.yml`     | stable Release published + `workflow_dispatch` | `:semver` `:latest` `:dev` (CLI + MCP)             |
| `release-github-assets.yml`  | `workflow_call`                                | CLI + MCP binaries + WASM tarballs + contracts     |
| `release-github-stable.yml`  | Release published (not prerelease)             | Attach assets to Latest                                    |
| `release-github-preview.yml` | nightly success / dispatch                     | Overwrite Pre-release `dev-preview` + assets               |

## Images

### CLI (`ceps-rust-ts-client`)

| Registry        | Image                                           |
| --------------- | ----------------------------------------------- |
| Docker Hub      | `interchouette/ceps-rust-ts-client`             |
| GHCR (org)      | `ghcr.io/interchouette-itc/ceps-rust-ts-client` |
| GHCR (personal) | `ghcr.io/groussac/ceps-rust-ts-client`          |

### MCP (`ceps-rust-ts-client-mcp`)

| Registry        | Image                                               |
| --------------- | --------------------------------------------------- |
| Docker Hub      | `interchouette/ceps-rust-ts-client-mcp`             |
| GHCR (org)      | `ghcr.io/interchouette-itc/ceps-rust-ts-client-mcp` |
| GHCR (personal) | `ghcr.io/groussac/ceps-rust-ts-client-mcp`          |

```bash
make release-cli-bin && make docker-build IMAGE_TAG=local
make release-mcp-bin && make docker-build-mcp IMAGE_TAG=local
# after docker login Hub + GHCR:
make docker-push IMAGE_TAG=dev
make docker-push-mcp IMAGE_TAG=dev
```

| Event                   | Tags                        |
| ----------------------- | --------------------------- |
| Tip push `dev`          | `:dev`                      |
| Stable Release `vX.Y.Z` | `:X.Y.Z`, `:latest`, `:dev` |

Stable tag (without `v`) **must** equal `[workspace.package].version` in root `Cargo.toml` or the release image job fails.

## Release artefacts

Attached to each GitHub Release (stable or `dev-preview`). Full fetch guide: [releases.md](releases.md).

| Asset                                  | Contents                                              |
| -------------------------------------- | ----------------------------------------------------- |
| `ceps-client-cli-{label}-linux-x86_64` | Stripped CLI                                          |
| `ceps-rust-ts-client-mcp-{label}-linux-x86_64` | Stripped MCP server (stdio / HTTP)                    |
| `ceps-client-wasm-nodejs-{label}.tgz`  | Client pack for Node (`pkg-nodejs`)                   |
| `ceps-client-wasm-web-{label}.tgz`     | Client pack for browsers (`pkg`)                      |
| `ceps-contracts-{label}.tgz`           | Demo tip on-chain WASMs (`cep18` / `cep78` / `cep85`) |
| `ceps-contracts-{label}.MANIFEST.txt`  | Manifest                                              |
| `SHA256SUMS`                           | Checksums                                             |

`{label}` is the tag without a leading `v` (e.g. `1.0.0` or `dev-preview`).

## Secrets (repo Settings → Secrets and variables → Actions)

Configure on **`Interchouette-ITC/ceps-rust-ts-client`** (mirror on the personal fork only if that fork still runs Actions). Used by `hub-images-dev` / `hub-images-release`:

| Secret               | Purpose |
| -------------------- | ------- |
| `DOCKER_USERNAME`    | Docker Hub **login** user |
| `DOCKER_PASSWORD`    | Docker Hub login password / token |
| `GHCR_USERNAME`      | GHCR login |
| `GHCR_PAT`           | GHCR PAT with package write to `interchouette-itc` (and optional `groussac`) |

Hub image tags hardcode namespace **`interchouette`** (not a secret).

`GITHUB_TOKEN` (Actions default) uploads release assets and moves the `dev-preview` tag. After the first GHCR push, set package visibility public (or grant org access).

NCTL live CI also mounts `assets/{users,faucet}` and exports `SECRET_KEY_USER_1` / `SECRET_KEY_USER_2` from those PEMs (public NCTL fixtures; not repo secrets).

`ci-test` / `nightly-test` check out the SDK pin and CEP tip forks into the Actions workspace so path deps and `make wasm-from-ceps` resolve. Release and hub image jobs check out **client + SDK only** (no tip WASMs required for CLI/WASM pack builds when contracts are already in-tree; release assets still stage tips for `ceps-contracts-*.tgz`).

## Pins

| Dependency | CI source | Ref |
| ---------- | --------- | --- |
| SDK | `casper-ecosystem/casper-rust-wasm-sdk` | branch `dev` |
| CEP-18 tip | `Interchouette-ITC/cep-18` | `ceps-client-test` |
| CEP-78 tip | `Interchouette-ITC/cep-78-enhanced-nft` | `ceps-client-test` |
| CEP-85 tip | `Interchouette-ITC/cep-85` | `ceps-client-test` |
| CEP-95 tip | `Interchouette-ITC/cep-95` | `ceps-client-test` |
| NCTL | `interchouette/casper-nctl-2-docker:dev` | Docker Hub |

Demo tip WASMs are **copied** from each ITC tip’s `tests/wasm/` (`make wasm-from-ceps`). CI does not rebuild CEP contracts. See [contributing.md](contributing.md).

## Same-day stable cut

1. Confirm `[workspace.package] version` (currently `1.0.0`).
2. Fill Hub/GHCR secrets on the **ITC** repo; ensure the PAT can push GHCR namespaces.
3. Push to `origin` `dev` (triggers `:dev` images once Hub/GHCR auth works).
4. Create GitHub Release **`v1.0.0`** (not prerelease) on that commit → stable assets + Hub/GHCR `:1.0.0` `:latest` `:dev`.
