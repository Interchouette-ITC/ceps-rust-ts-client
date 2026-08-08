# Docker (CLI image)

Runtime image: Debian slim + stripped `ceps-client-cli` binary. The SDK is **not** copied into the image; CI builds the binary with the SDK path/git dep first, then [`docker/Dockerfile`](../docker/Dockerfile) packs only `ceps-client-cli`.

## Local build

```bash
make release-cli-bin
make docker-build IMAGE_TAG=local
docker run --rm ceps-rust-ts-client:local --help
docker run --rm ceps-rust-ts-client:local status \
  --rpc-url http://host.docker.internal:11101
```

## Registries

| Registry | Image |
| --- | --- |
| Docker Hub | `interchouette/ceps-rust-ts-client` |
| GHCR (personal) | `ghcr.io/groussac/ceps-rust-ts-client` |
| GHCR (org) | `ghcr.io/interchouette-itc/ceps-rust-ts-client` |

## Tags

| Tag | Meaning |
| --- | --- |
| `:dev` | Tip of `dev` (hub-images-dev) |
| `:X.Y.Z` | Stable GitHub Release matching workspace version |
| `:latest` | Same digest as the latest stable semver |

```bash
docker pull interchouette/ceps-rust-ts-client:dev
# or
docker pull ghcr.io/interchouette-itc/ceps-rust-ts-client:dev
```

Push helpers: `make docker-push IMAGE_TAG=dev` (requires Hub + GHCR login). Full pipeline and secrets: [ci.md](ci.md).
