# WASM / TypeScript

`ceps-wasm` is the CEP client compiled for JavaScript (`wasm-bindgen`). Same idea as Rust `ceps-client`: `Cep18Client` / `Cep78Client` / `Cep85Client` for Node or the browser.

It is **not** the on-chain contract `.wasm`. Contract bytes still go into `install(...)` (from demo tips via `make wasm-from-ceps`, or your own builds).

## Build

```bash
make nodejs   # → ceps-wasm/pkg-nodejs (Node)
make web      # → ceps-wasm/pkg (browser)
make pack     # both
```

## Bound methods (today)

| Client | Methods |
| --- | --- |
| CEP-18 | `rpcUrl`, `sseUrl`, `chainName`, `setContractHash`, `install`, `name`, `symbol`, `balanceOf` |
| CEP-78 | `rpcUrl`, `sseUrl`, `setContractHash`, `install`, `collectionName`, `balanceOf` |
| CEP-85 | `rpcUrl`, `sseUrl`, `setContractHash`, `install`, `collectionName`, `balanceOf` |

Full mutate/query parity lives in Rust `ceps-client`. Generated `.d.ts` may also list transitive SDK symbols; treat those as SDK surface.

## Node example

```js
import { Cep18Client } from "ceps-wasm";

const client = new Cep18Client(
  "http://127.0.0.1:11101",
  "http://127.0.0.1:18101/events",
  "casper-net-1",
  0,
);
client.setContractHash(contractHash, packageHash);
const name = await client.name();
const bal = await client.balanceOf("account-hash-…");
```

Install returns JSON `{ transactionHash, hasExecutionResult }`. See the README **WASM packs** section for a short overview.

## Vitest

```bash
make ts-test
```
