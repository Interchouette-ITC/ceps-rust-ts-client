# ceps-rust-ts-client-mcp

MCP server for **`ceps-rust-ts-client`**: CEP-18 / CEP-78 / CEP-85 / CEP-95 install, query, and mutate, plus demo contract WASM helpers.

Transports: **stdio** (default) or **Streamable HTTP** on `:6790`.

Raw node RPC stays on [`casper-rust-wasm-sdk-mcp`](https://github.com/casper-ecosystem/casper-rust-wasm-sdk) (`sdk_*`). NCTL lifecycle stays on `casper-nctl-2-docker` (`nctl_*`).

## Quick start

```bash
# stdio (Cursor / agents)
cargo run -p ceps-rust-ts-client-mcp --release

# HTTP
cargo run -p ceps-rust-ts-client-mcp --release -- --http --listen 0.0.0.0:6790
# → http://127.0.0.1:6790/mcp
```

Make:

```bash
make mcp-build
make run-mcp-http
make mcp-http          # compose Hub/local image on :6790
make mcp-test
make mcp-test-live     # ignored; needs NCTL
```

## Images

| Registry   | Image                                                                           |
| ---------- | ------------------------------------------------------------------------------- |
| Docker Hub | `interchouette/ceps-rust-ts-client-mcp`                                                 |
| GHCR       | `ghcr.io/interchouette-itc/ceps-rust-ts-client-mcp` (optional personal: `ghcr.io/groussac/…`) |

Tags: `:dev`, `:latest`, `:X.Y.Z`.

## Environment

| Var               | Default                                      |
| ----------------- | -------------------------------------------- |
| `CEPS_RPC_URL`    | `http://127.0.0.1:11101`                     |
| `CEPS_SSE_URL`    | `http://127.0.0.1:18101/events`              |
| `CEPS_CHAIN_NAME` | `casper-net-1`                               |
| `CEPS_VERBOSITY`  | `low`                                        |
| `CEPS_WASM_ROOT`  | `tests/wasm` (image: `/opt/ceps/tests/wasm`) |
| `CEPS_MCP_ADDR`   | `0.0.0.0:6790`                               |
| `CEPS_MCP_HTTP`   | unset (stdio); set for HTTP                  |

## Tools

Full inventory: [TOOLS.md](TOOLS.md). Meta entrypoint: `ceps_help`.

## Patterns

Layout and shipping notes: [PATTERNS.md](PATTERNS.md). Client config samples: [mcp.json.example](mcp.json.example).
