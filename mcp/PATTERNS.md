# MCP patterns

How `mcp/` is laid out, built, and run (aligned with rustSDK’s sidecar).

## Layout

```
mcp/
  Cargo.toml
  Dockerfile          # runtime: binary + bundled tests/wasm
  README.md
  TOOLS.md
  PATTERNS.md
  mcp.json.example
  src/
    main.rs           # clap: --http / CEPS_MCP_ADDR
    lib.rs
    server.rs         # #[mcp_server] + #[tool]
    handle.rs         # process-wide endpoints + wasm root
    format.rs
    tools/            # meta, wasm, cep18, cep78, cep85, params
```

`mcpkit` stays out of `ceps-client` / `ceps-client-wasm`.

## Transports

- Default: stdio (`StdioTransport`)
- HTTP: `--http` / `CEPS_MCP_HTTP=1`, listen `CEPS_MCP_ADDR` (default `0.0.0.0:6790`)
- Cursor HTTP clients use mcp-remote against `http://127.0.0.1:6790/mcp`

## Shipping

- Binary: `make release-mcp-bin` → `target/release/ceps-client-mcp`
- Image: `make docker-build-mcp` (copies binary + `tests/wasm` into `mcp/` build context)
- Compose: `docker/docker-compose.mcp.yml`
- Release asset: `ceps-client-mcp-{label}-linux-x86_64`
