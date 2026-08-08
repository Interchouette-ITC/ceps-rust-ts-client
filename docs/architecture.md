# Architecture

```text
casper-rust-wasm-sdk          (RPC, tx, install, query, wait, CES)
        ^
   ceps-client                (CepCore + Cep18/78/85 facades, CEP enums, error maps)
      ^        ^
   cli/`ceps`  ceps-wasm      (clap)   (wasm-bindgen CEP-only exports)
```

## Ownership

| Concern | Crate |
| --- | --- |
| Identities, tx, payment, wait, CLValues | SDK |
| CEP enums, dictionary keys, user-error maps | `ceps-client` |
| Shell UX | `cli` |
| Browser / Node CEP API | `ceps-wasm` (no full SDK re-export) |

## Facades

- `Cep18Client` - fungible token
- `Cep78Client` - enhanced NFT
- `Cep85Client` - multi-token (on-chain name CEP-85)

All share `CepCore` for endpoints, contract targeting, install/call/query/wait.
