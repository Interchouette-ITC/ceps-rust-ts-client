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
    server.rs         # #[tool_router] + #[tool]
    tool_args.rs      # schemars Parameters structs
    handle.rs         # process-wide endpoints + wasm root
    format.rs
    tools/            # meta, wasm, cep18, cep78, cep85, params
```

`rmcp` stays out of `ceps-client` / `ceps-client-wasm`.

## Transports

- Default: stdio (`rmcp` `transport::stdio`)
- HTTP: `--http` / `CEPS_MCP_HTTP=1`, listen `CEPS_MCP_ADDR` (default `0.0.0.0:6790`)
- Cursor HTTP clients use mcp-remote against `http://127.0.0.1:6790/mcp`

## Shipping

- Binary: `make release-mcp-bin` → `target/release/ceps-rust-ts-client-mcp`
- Image: `make docker-build-mcp` → `ceps-rust-ts-client-mcp` (copies binary + `tests/wasm` into `mcp/` build context)
- Compose: `docker/docker-compose.mcp.yml`
- Release asset: `ceps-rust-ts-client-mcp-{label}-linux-x86_64`
