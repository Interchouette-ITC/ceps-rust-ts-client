# ceps-rust-ts-client-mcp

MCP server for **ceps-rust-ts-client** (stdio + Streamable HTTP): CEP-18 / 78 / 85 / 95 client APIs plus demo contract WASM helpers (`ceps_*` tools).

**Product:** `ceps-rust-ts-client`
Source: https://github.com/Interchouette-ITC/ceps-rust-ts-client
Docs: https://github.com/Interchouette-ITC/ceps-rust-ts-client/blob/dev/docs/mcp.md

Docker Hub: [`interchouette/ceps-rust-ts-client-mcp`](https://hub.docker.com/r/interchouette/ceps-rust-ts-client-mcp)
Org GHCR: `ghcr.io/interchouette-itc/ceps-rust-ts-client-mcp`
Personal GHCR: `ghcr.io/groussac/ceps-rust-ts-client-mcp`

Companion CLI image: [`interchouette/ceps-rust-ts-client`](https://hub.docker.com/r/interchouette/ceps-rust-ts-client)

## Tags

| Tag       | Meaning                                          |
| --------- | ------------------------------------------------ |
| `:dev`    | Tip of `dev`                                     |
| `:X.Y.Z`  | Stable GitHub Release matching workspace version |
| `:latest` | Same digest as the latest stable semver          |

## Quick start

```bash
docker pull interchouette/ceps-rust-ts-client-mcp:dev

# Streamable HTTP on :6790
docker run --rm -d --name ceps-rust-ts-client-mcp \
  -p 6790:6790 \
  --add-host=host.docker.internal:host-gateway \
  -e CEPS_RPC_URL=http://host.docker.internal:11101 \
  -e CEPS_SSE_URL=http://host.docker.internal:18101/events \
  -e CEPS_CHAIN_NAME=casper-net-1 \
  interchouette/ceps-rust-ts-client-mcp:dev \
  --http --listen 0.0.0.0:6790
```

MCP endpoint: `http://127.0.0.1:6790/mcp`

From a clone:

```bash
make mcp-http
make mcp-http-stop
make run-mcp
make run-mcp-http
```

Bundled demo WASM root in the image: `CEPS_WASM_ROOT=/opt/ceps/tests/wasm`.

## Env

| Variable          | Default (image / compose)                  | Role        |
| ----------------- | ------------------------------------------ | ----------- |
| `CEPS_MCP_ADDR`   | `0.0.0.0:6790`                             | HTTP listen |
| `CEPS_RPC_URL`    | `http://host.docker.internal:11101`        | Node RPC    |
| `CEPS_SSE_URL`    | `http://host.docker.internal:18101/events` | SSE events  |
| `CEPS_CHAIN_NAME` | `casper-net-1`                             | Chain name  |
| `CEPS_WASM_ROOT`  | `/opt/ceps/tests/wasm`                     | Demo WASM   |

Raw JSON-RPC / binary tools stay on the SDK MCP (`sdk_*`). NCTL lifecycle stays on `nctl_*`.
