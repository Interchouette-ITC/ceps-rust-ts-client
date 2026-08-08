# WASM / TypeScript

`ceps-wasm` is the **same CEP client API** as Rust `ceps-client`, compiled with `wasm-bindgen` for Node and browsers. It replaces per-CEP TypeScript `client-js` packages for JS callers.

It is **not** the on-chain contract `.wasm`. Contract bytes still go into `install(...)` as `Uint8Array` (from `tests/wasm/…` or your own build).

```text
ceps-client (Rust)  ─wasm-pack─→  ceps-wasm/pkg-nodejs  (Node)
                              └→  ceps-wasm/pkg         (web)
```

## Build

```bash
make nodejs   # ceps-wasm/pkg-nodejs
make web      # ceps-wasm/pkg
make pack     # both
```

Requires `wasm-pack` and Binaryen `wasm-opt` (Makefile pins version via `ensure-binaryen`).

## Bound methods (today)

| Client | Methods |
| --- | --- |
| CEP-18 | `rpcUrl`, `sseUrl`, `chainName`, `setContractHash`, `install`, `name`, `symbol`, `balanceOf` |
| CEP-78 | `rpcUrl`, `sseUrl`, `setContractHash`, `install`, `collectionName`, `balanceOf` |
| CEP-85 | `rpcUrl`, `sseUrl`, `setContractHash`, `install`, `collectionName`, `balanceOf` |

Full mutate/query parity lives in Rust `ceps-client`. Generated `.d.ts` may also list transitive SDK symbols from `wasm-bindgen`; treat those as SDK surface, not a supported re-export.

## Node usage

```js
import { Cep18Client } from "./ceps-wasm/pkg-nodejs/ceps_wasm.js";

const client = new Cep18Client(
  "http://127.0.0.1:11101",
  "http://127.0.0.1:18101/events",
  "casper-net-1",
  0, // verbosity Low
);
client.setContractHash(contractHash, packageHash);
const name = await client.name();
const bal = await client.balanceOf("account-hash-…");
```

Install returns a JSON string `{ transactionHash, hasExecutionResult }`:

```js
import { readFileSync } from "node:fs";

const contractWasm = new Uint8Array(readFileSync("tests/wasm/cep18/cep18.wasm"));
const secretPem = readFileSync("secret_key.pem", "utf8");
const resultJson = await client.install(
  "MyToken",
  "MTK",
  9,
  "1000000000",
  2, // CES
  contractWasm,
  secretPem,
  "400000000000",
  true,
);
```

## Vitest

```bash
make ts-test
```

Smoke tests under `tests/ts/` import from `pkg-nodejs`.

README also has a **Usage (Node / `ceps-wasm`)** section with the same story.
