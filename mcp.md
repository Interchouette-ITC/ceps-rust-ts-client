# MCP (`ceps-client-mcp`)

Agent surface for this product: CEP-18 / 78 / 85 install, query, and mutate via MCP tools (`ceps_*`), plus demo contract WASM helpers.

| Item           | Value                                               |
| -------------- | --------------------------------------------------- |
| Crate / binary | `ceps-client-mcp`                                   |
| Path           | [`mcp/`](../mcp/)                                   |
| HTTP           | `http://127.0.0.1:6790/mcp` (`CEPS_MCP_ADDR`)       |
| Hub            | `interchouette/ceps-client-mcp:{dev,latest,semver}` |

Raw JSON-RPC / binary-port tools stay on the SDK MCP (`sdk_*`). NCTL lifecycle stays on `nctl_*`.

## Docs in-tree

- [mcp/README.md](../mcp/README.md) - run / env / images
- [mcp/TOOLS.md](../mcp/TOOLS.md) - full tool inventory
- [mcp/PATTERNS.md](../mcp/PATTERNS.md) - layout and shipping
- [mcp/mcp.json.example](../mcp/mcp.json.example) - Cursor samples

## Make

```bash
make mcp-build
make run-mcp-http
make mcp-http          # compose :6790
make docker-build-mcp IMAGE_TAG=local
make mcp-test
make mcp-test-live     # ignored; needs NCTL + SECRET_KEY_USER_1
```

Unit tests run in `ci-local` / `ci-test`. Live smokes (`#[ignore]`) run in `ci-test` and `nightly-test` when NCTL is up.
