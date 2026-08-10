# Docker

## CLI image (`ceps-rust-ts-client`)

Runtime image: Debian slim + stripped `ceps-client-cli` binary. The SDK is **not** copied into the image; CI builds the binary with the SDK path/git dep first, then [`docker/Dockerfile`](../docker/Dockerfile) packs only `ceps-client-cli`.

```bash
make release-cli-bin
make docker-build IMAGE_TAG=local
docker run --rm ceps-rust-ts-client:local --help
```

| Registry        | Image                                           |
| --------------- | ----------------------------------------------- |
| Docker Hub      | `interchouette/ceps-rust-ts-client`             |
| GHCR (org)      | `ghcr.io/interchouette-itc/ceps-rust-ts-client` |
| GHCR (personal) | `ghcr.io/groussac/ceps-rust-ts-client`          |

## MCP image (`ceps-rust-ts-client-mcp`)

Runtime image: Debian slim + stripped `ceps-client-mcp` binary + bundled `tests/wasm` (`CEPS_WASM_ROOT=/opt/ceps/tests/wasm`). Default entrypoint is stdio; pass `--http` for `:6790`.

```bash
make docker-build-mcp IMAGE_TAG=local
docker run --rm -p 6790:6790 ceps-rust-ts-client-mcp:local --http --listen 0.0.0.0:6790
```

| Registry        | Image                                               |
| --------------- | --------------------------------------------------- |
| Docker Hub      | `interchouette/ceps-rust-ts-client-mcp`             |
| GHCR (org)      | `ghcr.io/interchouette-itc/ceps-rust-ts-client-mcp` |
| GHCR (personal) | `ghcr.io/groussac/ceps-rust-ts-client-mcp`          |

## Tags

| Tag       | Meaning                                          |
| --------- | ------------------------------------------------ |
| `:dev`    | Tip of `dev` (hub-images-dev)                    |
| `:X.Y.Z`  | Stable GitHub Release matching workspace version |
| `:latest` | Same digest as the latest stable semver          |

```bash
docker pull interchouette/ceps-rust-ts-client:dev
docker pull interchouette/ceps-rust-ts-client-mcp:dev
```

Push helpers: `make docker-push IMAGE_TAG=dev` and `make docker-push-mcp IMAGE_TAG=dev`. Full pipeline: [ci.md](ci.md). Details: [mcp.md](mcp.md).
