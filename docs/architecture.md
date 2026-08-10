# Architecture

```text
casper-rust-wasm-sdk          (RPC, tx, install, query, wait, CES)
        ^
   ceps-client                (CEPClient + CEP18/78/85/95 facades, CEP enums, error maps, CES helpers)
      ^        ^        ^
   ceps-client-cli  ceps-client-wasm  ceps-rust-ts-client-mcp
      (clap)         (wasm-bindgen)    (mcpkit stdio/HTTP)
```

## Ownership

| Concern                                     | Owner                                                             |
| ------------------------------------------- | ----------------------------------------------------------------- |
| Identities, tx, payment, wait, CLValues     | SDK                                                               |
| CEP enums, dictionary keys, user-error maps | `ceps-client`                                                     |
| Shell UX                                    | `ceps-client-cli`                                                 |
| Browser / Node CEP API                      | `ceps-client-wasm`                                                |
| Agent MCP (`ceps_*`)                        | `ceps-rust-ts-client-mcp`                                         |
| Contract WASM bytes                         | Committed under `tests/wasm/`; refresh with `make wasm-from-ceps` |
| Local chain                                 | External NCTL; this repo only connects to it                      |

## Facades

- `CEP18Client` - fungible token
- `CEP78Client` - first-class enhanced NFT (mode matrix + session helpers)
- `CEP85Client` - multi-token (on-chain standard name **CEP-85**)
- `CEP95Client` - supported simpler ERC-721-shaped NFT (Odra tip; does not replace CEP-78)

CEP facades own install, mutate, and typed queries. Shared [`CEPClient`](../ceps-client/src/core/mod.rs) (via `core()` / `core_mut()`) holds endpoints, bound contract (`set_contract_hash` / `target`), `put_transaction`, `wait_transaction`, and CES parse/collect helpers.

## Put vs make

Mutate and install take `&TransactionParams`:

- **Put (default):** SDK make + put. `CallResult.transaction_hash` from the put response. When `wait` is true (default), SSE wait attaches execution (and CES when bound).
- **Make only:** `TransactionParams::make_only()` (or CLI/MCP `make_only`). Same CEP args/entrypoints, SDK `make_transaction` only, no put. `CallResult.transaction` holds puttable Transaction JSON. PEM is optional; unsigned make needs `initiator_addr`. Wait is skipped.
- **Put signed JSON:** after external signing, `CEPClient::put_transaction(&transaction_json, wait, timeout)` submits that JSON and returns `CallResult`. Same path via CLI `put-transaction` / MCP `ceps_put_transaction`. Use `wait_transaction` (CLI `wait-transaction` / MCP `ceps_wait_transaction`) when you put without wait and need to wait later by hash.

## CLI vs library

The CLI exposes status, shared `CEPClient` helpers (`put-transaction`, `wait-transaction`, `ces *`), and CEP closet install/mutate/query. Closet `*-cli.md` pages document the shipped surface only.

## Testing layers

| Layer       | Role                                    |
| ----------- | --------------------------------------- |
| Unit        | No network; URL/key/error helpers       |
| Integration | Lib against NCTL + staged tip WASMs     |
| CLI smoke   | `ceps-client-cli status` / `info`       |
| Vitest      | Packed `ceps-client-wasm` Node bindings |

Details: [testing.md](testing.md).
