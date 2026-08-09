# CEP-18 CLI

```bash
ceps-client-cli [--rpc-url …] [--sse-url …] [--chain-name …] [--json] cep18 <cmd>
```

| Command | Behavior |
| --- | --- |
| `info` | Prints bound RPC / SSE / chain endpoints |
| `install` | Install WASM (`--wasm`, `--secret-key` or `--make-only`) |
| `transfer` / `transfer-from` / `approve` / `mint` / `burn` | Mutates (`--make-only` supported) |
| `name` / `balance` | Queries |

Example make-only transfer:

```bash
ceps-client-cli cep18 transfer --contract-hash <hash> --recipient <account-hash-…> \
  --amount 10 --make-only --initiator-addr <pubkey-hex>
```

See [docs/cli.md](../cli.md) and [8-api.md](8-api.md).
