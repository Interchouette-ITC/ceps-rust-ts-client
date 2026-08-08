# CLI (`ceps-client-cli`)

Package and binary: `ceps-client-cli` (crate path `ceps-client-cli/`).

## Global flags

| Flag / env | Default (NCTL) |
| --- | --- |
| `--rpc-url` / `CEPS_RPC_URL` | `http://127.0.0.1:11101` |
| `--sse-url` / `CEPS_SSE_URL` | `http://127.0.0.1:18101/events` |
| `--chain-name` / `CEPS_CHAIN_NAME` | `casper-net-1` |
| `--verbosity` | low |
| `--json` | off |

## Commands

```bash
ceps-client-cli status
ceps-client-cli cep18 info
ceps-client-cli cep78 info
ceps-client-cli cep78 name --contract-hash <hash> [--package-hash <hash>]
ceps-client-cli cep78 balance --contract-hash <hash> --account <account-hash-…>
ceps-client-cli cep85 info
ceps-client-cli cep85 name --contract-hash <hash> [--package-hash <hash>]
ceps-client-cli cep85 balance --contract-hash <hash> --account <…> --id <id>
ceps-client-cli cep95 info
ceps-client-cli cep95 name --contract-hash <hash> [--package-hash <hash>]
ceps-client-cli cep95 owner-of --contract-hash <hash> --token-id <id>
```

CEP-95 also exposes install / mint / burn / transfer / approve on the CLI (see [docs/cep95/7-cli.md](cep95/7-cli.md)). Other CEPs keep mutations on the Rust library / examples (`cargo run -p ceps-client --example cep18_install`, and the CEP-78 / CEP-85 / CEP-95 counterparts). Closet `*-cli.md` pages match this surface.
