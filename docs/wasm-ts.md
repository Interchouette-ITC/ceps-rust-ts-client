# WASM / TypeScript

`ceps-wasm` is a thin `wasm-bindgen` layer over `ceps-client`.

Exported types: `Cep18Client`, `Cep78Client`, `Cep85Client`.

Methods currently bound (JS names):

| Client | Methods                                                                                      |
| ------ | -------------------------------------------------------------------------------------------- |
| CEP-18 | `rpcUrl`, `sseUrl`, `chainName`, `setContractHash`, `install`, `name`, `symbol`, `balanceOf` |
| CEP-78 | `rpcUrl`, `sseUrl`, `setContractHash`, `install`, `collectionName`, `balanceOf`              |
| CEP-85 | `rpcUrl`, `sseUrl`, `setContractHash`, `install`, `collectionName`, `balanceOf`              |

Full mutate/query parity lives in Rust `ceps-client`. Because `casper-rust-wasm-sdk` also uses `wasm-bindgen`, the packed `.d.ts` lists additional SDK symbols: treat those as transitive.

## Build

```bash
make nodejs   # ceps-wasm/pkg-nodejs
make web      # ceps-wasm/pkg
make pack     # both
```

Requires `wasm-pack` and Binaryen `wasm-opt` (Makefile pins version via `ensure-binaryen`).

## Vitest

```bash
make ts-test
```

Smoke tests live under `tests/ts/` and import `ceps-wasm` from `pkg-nodejs`.

## Usage (Node)

```js
import { Cep18Client } from "ceps-wasm";

const client = new Cep18Client(
  "http://127.0.0.1:11101",
  "http://127.0.0.1:18101/events",
  "casper-net-1",
  0, // verbosity Low
);
client.setContractHash(contractHash, packageHash);
const name = await client.name();
```

Install helpers accept `Uint8Array` WASM bytes plus PEM secret and payment amount; they return a JSON string `{ transactionHash, hasExecutionResult }`.
