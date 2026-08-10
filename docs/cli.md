# CLI (`ceps-client-cli`)

Package and binary: `ceps-client-cli` (crate path `ceps-client-cli/`).

## Global flags

| Flag / env                         | Default (NCTL)                  |
| ---------------------------------- | ------------------------------- |
| `--rpc-url` / `CEPS_RPC_URL`       | `http://127.0.0.1:11101`        |
| `--sse-url` / `CEPS_SSE_URL`       | `http://127.0.0.1:18101/events` |
| `--chain-name` / `CEPS_CHAIN_NAME` | `casper-net-1`                  |
| `--verbosity`                      | low                             |
| `--json`                           | off                             |

## Commands

Every CEP closet exposes info/query plus install/mutate. Write commands accept `--secret-key` for put, or `--make-only` with `--initiator-addr` (or PEM) for Transaction JSON without put.

Shared `CEPClient` surface (not CEP-specific):

```bash
# After make-only + external sign:
ceps-client-cli put-transaction --transaction-file signed.json
ceps-client-cli put-transaction --transaction-file - --no-wait   # stdin; skip SSE wait
ceps-client-cli wait-transaction --transaction-hash <tx>
ceps-client-cli ces parse --contract-hash <hash> --transaction-hash <tx>
ceps-client-cli ces parse-execution --contract-hash <hash> --execution-file exec.json
ceps-client-cli ces collect --contract-hash <hash> --event-name Mint --max-transactions 8
```

CEP closets:

```bash
ceps-client-cli status
ceps-client-cli cep18 info
ceps-client-cli cep18 install --name N --symbol S --total-supply 1000 \
  --wasm tests/wasm/cep18/cep18.wasm --secret-key /path/to/secret_key.pem
ceps-client-cli cep18 transfer --contract-hash <hash> --recipient <…> --amount 10 \
  --make-only --initiator-addr <pubkey-hex>
ceps-client-cli cep78 info
ceps-client-cli cep78 install --name N --symbol S --wasm tests/wasm/cep78/cep78.wasm \
  --secret-key /path/to/secret_key.pem
ceps-client-cli cep78 mint --contract-hash <hash> --token-owner <…> --make-only \
  --initiator-addr <pubkey-hex>
ceps-client-cli cep85 info
ceps-client-cli cep85 install --name N --uri 'https://example.com/{id}.json' \
  --wasm tests/wasm/cep85/cep85.wasm --secret-key /path/to/secret_key.pem
ceps-client-cli cep85 mint --contract-hash <hash> --recipient <…> --id 1 --amount 10 \
  --make-only --initiator-addr <pubkey-hex>
ceps-client-cli cep95 info
ceps-client-cli cep95 install --name N --symbol S --package-key-name KEY \
  --wasm tests/wasm/cep95/cep95.wasm --secret-key /path/to/secret_key.pem
```

Closet pages: [cep18/7-cli.md](cep18/7-cli.md), [cep78/8-cli.md](cep78/8-cli.md), [cep85/8-cli.md](cep85/8-cli.md), [cep95/7-cli.md](cep95/7-cli.md).
