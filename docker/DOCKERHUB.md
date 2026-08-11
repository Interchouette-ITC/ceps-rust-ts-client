# ceps-rust-ts-client

CLI image for the unified Casper **CEP-18**, **CEP-78**, **CEP-85**, and **CEP-95** client (`ceps-client-cli`).

**Product:** `ceps-rust-ts-client`
Source: https://github.com/Interchouette-ITC/ceps-rust-ts-client

Docker Hub: [`interchouette/ceps-rust-ts-client`](https://hub.docker.com/r/interchouette/ceps-rust-ts-client)
Org GHCR: `ghcr.io/interchouette-itc/ceps-rust-ts-client`
Personal GHCR: `ghcr.io/groussac/ceps-rust-ts-client`

Runtime: Debian slim + stripped `ceps-client-cli` binary (SDK linked into the binary).

## Tags

| Tag       | Meaning                                          |
| --------- | ------------------------------------------------ |
| `:dev`    | Tip of `dev`                                     |
| `:X.Y.Z`  | Stable GitHub Release matching workspace version |
| `:latest` | Same digest as the latest stable semver          |

```bash
docker pull interchouette/ceps-rust-ts-client:dev
docker run --rm interchouette/ceps-rust-ts-client:dev --help
```

## MCP image (separate)

Agents / Cursor: [`interchouette/ceps-rust-ts-client-mcp`](https://hub.docker.com/r/interchouette/ceps-rust-ts-client-mcp)

```bash
docker pull interchouette/ceps-rust-ts-client-mcp:dev
# Streamable HTTP on :6790 → http://127.0.0.1:6790/mcp
```

Docs: https://github.com/Interchouette-ITC/ceps-rust-ts-client/blob/dev/docs/docker.md
