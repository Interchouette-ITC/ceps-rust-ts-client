# Architecture

```text
casper-rust-wasm-sdk          (RPC, tx, install, query, wait, CES)
        ^
   ceps-client                (CepCore + Cep18/78/85 facades, CEP enums, error maps)
      ^        ^        ^
   ceps-client-cli  ceps-client-wasm  ceps-client-mcp
      (clap)         (wasm-bindgen)    (mcpkit stdio/HTTP)
```

## Ownership

| Concern                                     | Owner                                                             |
| ------------------------------------------- | ----------------------------------------------------------------- |
| Identities, tx, payment, wait, CLValues     | SDK                                                               |
| CEP enums, dictionary keys, user-error maps | `ceps-client`                                                     |
| Shell UX                                    | `ceps-client-cli`                                                 |
| Browser / Node CEP API                      | `ceps-client-wasm`                                                |
| Agent MCP (`ceps_*`)                        | `ceps-client-mcp`                                                 |
| Contract WASM bytes                         | Committed under `tests/wasm/`; refresh with `make wasm-from-ceps` |
| Local chain                                 | External NCTL; this repo only connects to it                      |

## Facades

- `Cep18Client` - fungible token
- `Cep78Client` - enhanced NFT (mode matrix + session helpers)
- `Cep85Client` - multi-token (on-chain standard name **CEP-85**)

All share `CepCore` for endpoints, contract targeting, install/call/query/wait.

## CLI vs library

The CLI exposes status and selected queries. Install and most mutations live on the library (and `ceps-client` examples). Closet `*-cli.md` pages document the shipped surface only.

## Testing layers

| Layer       | Role                                    |
| ----------- | --------------------------------------- |
| Unit        | No network; URL/key/error helpers       |
| Integration | Lib against NCTL + staged tip WASMs     |
| CLI smoke   | `ceps-client-cli status` / `info`       |
| Vitest      | Packed `ceps-client-wasm` Node bindings |

Details: [testing.md](testing.md).
