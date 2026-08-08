# Demo on-chain CEP contract WASMs

These are **contract** bytecode files (what you pass to `install`), staged from the demo tip forks via `make wasm-from-ceps`.

They are **not** the JavaScript client pack. The client-for-JS lives under `ceps-client-wasm/pkg` and `ceps-client-wasm/pkg-nodejs`.

| Dir | Contents |
| --- | --- |
| `cep18/` | CEP-18 fungible contract(s) |
| `cep78/` | CEP-78 NFT + session helpers |
| `cep85/` | CEP-85 multi-token |

Refresh from tip checkouts: `make wasm-from-ceps`. Or download `ceps-contracts-*.tgz` from a GitHub Release (see [docs/releases.md](../../docs/releases.md)).
