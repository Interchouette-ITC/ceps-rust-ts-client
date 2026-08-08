# CEP-18 CLI

```bash
ceps [--rpc-url …] [--sse-url …] [--chain-name …] [--json] cep18 <cmd>
```

| Command | Behavior                                 |
| ------- | ---------------------------------------- |
| `info`  | Prints bound RPC / SSE / chain endpoints |

Install, transfer, mint, and other mutations are on `Cep18Client` (see examples and [8-api.md](8-api.md)). Global defaults match NCTL `dev`. See [docs/cli.md](../cli.md).
