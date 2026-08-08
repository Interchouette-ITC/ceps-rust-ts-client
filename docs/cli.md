# CLI (`ceps`)

Binary crate: `cli` (package name `cli`, binary name `ceps`).

## Global flags / env

| Flag | Env | Default |
| --- | --- | --- |
| `--rpc-url` | `CEPS_RPC_URL` | `http://127.0.0.1:11101` |
| `--sse-url` | `CEPS_SSE_URL` | `http://127.0.0.1:18101/events` |
| `--chain-name` | `CEPS_CHAIN_NAME` | `casper-net-1` |
| `--verbosity` | `CEPS_VERBOSITY` | `low` |
| `--json` | | off |

## Subcommands (foundation)

```text
ceps status
ceps cep18 info
ceps cep78 info
ceps cep85 info
```

Mutating subcommands land with each CEP phase (install, mint, transfer, …) and stay mirrored in the matching closet `*-cli.md`.
