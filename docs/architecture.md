# Architecture

```text
casper-rust-wasm-sdk          (RPC, tx, install, query, wait, CES)
        ^
   ceps-client                (CepCore + Cep18/78/85 facades, CEP enums, error maps)
      ^        ^
   cli/`ceps`  ceps-wasm      (clap)   (wasm-bindgen CEP-only exports)
```

## Ownership

| Concern | Owner |
| --- | --- |
| Identities, tx, payment, wait, CLValues | SDK |
| CEP enums, dictionary keys, user-error maps | `ceps-client` |
| Shell UX | `cli` (`ceps`) |
| Browser / Node CEP API | `ceps-wasm` |
| Contract WASM bytes | Tip CEP builds → `make wasm-from-ceps` → `tests/wasm/` |
| Local chain | External NCTL; this repo only connects to it |

## Facades

- `Cep18Client` - fungible token
- `Cep78Client` - enhanced NFT (mode matrix + session helpers)
- `Cep85Client` - multi-token (on-chain standard name **CEP-85**)

All share `CepCore` for endpoints, contract targeting, install/call/query/wait.

## CLI vs library

The CLI exposes status and selected queries. Install and most mutations live on the library (and `ceps-client` examples). Closet `*-cli.md` pages document the shipped surface only.

## Testing layers

| Layer | Role |
| --- | --- |
| Unit | No network; URL/key/error helpers |
| Integration | Lib against NCTL + staged tip WASMs |
| CLI smoke | `ceps status` / `info` |
| Vitest | Packed `ceps-wasm` Node bindings |

Details: [testing.md](testing.md).
