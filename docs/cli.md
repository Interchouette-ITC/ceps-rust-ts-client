# CLI (`ceps`)

Binary crate: `cli` (package name `cli`, binary name `ceps`).

## Global flags / env

| Flag           | Env               | Default                         |
| -------------- | ----------------- | ------------------------------- |
| `--rpc-url`    | `CEPS_RPC_URL`    | `http://127.0.0.1:11101`        |
| `--sse-url`    | `CEPS_SSE_URL`    | `http://127.0.0.1:18101/events` |
| `--chain-name` | `CEPS_CHAIN_NAME` | `casper-net-1`                  |
| `--verbosity`  | `CEPS_VERBOSITY`  | `low`                           |
| `--json`       |                   | off                             |

## Subcommands

```text
ceps status
ceps cep18 info
ceps cep78 info
ceps cep78 name --contract-hash <hash> [--package-hash <hash>]
ceps cep78 balance --contract-hash <hash> --account <account-hash-…>
ceps cep85 info
ceps cep85 name --contract-hash <hash> [--package-hash <hash>]
ceps cep85 balance --contract-hash <hash> --account <…> --id <id>
```

Install and other mutations are on the Rust library / examples (`cargo run -p ceps-client --example cep18_install`, and the CEP-78 / CEP-85 counterparts). Closet `*-cli.md` pages match this surface.
