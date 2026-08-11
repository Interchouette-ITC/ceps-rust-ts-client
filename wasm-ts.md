# WASM / TypeScript

`ceps-client-wasm` is the CEP **client library** compiled for JavaScript (`wasm-bindgen`). Same idea as Rust `ceps-client`: `CEP18Client` / `CEP78Client` / `CEP85Client` for Node or the browser. It replaces the old per-CEP `client-js` packages.

Do not confuse it with **contract** `.wasm` under `tests/wasm/` (on-chain install bytes).

It is **not** the on-chain contract `.wasm`. Contract bytes still go into `install(...)` (from demo tips via `make wasm-from-ceps`, or your own builds).

## Build or download

```bash
make nodejs   # → ceps-client-wasm/pkg-nodejs (Node)
make web      # → ceps-client-wasm/pkg (browser)
make pack     # both
```

Or download `ceps-client-wasm-nodejs-*.tgz` / `ceps-client-wasm-web-*.tgz` from a [GitHub Release](releases.md) (same idea as using a published SDK `pkg` folder).

## Bound methods (today)

| Client | Methods |
| ------ | ------- |
| CEP-18 | `rpcUrl`, `SSEUrl`, `chainName`, `setContractHash`, `install`, `name`, `symbol`, `balanceOf`, `securityBadge` |
| CEP-78 | `rpcUrl`, `SSEUrl`, `setContractHash`, `install`, `collectionName`, `balanceOf`, `ownershipMode` |
| CEP-85 | `rpcUrl`, `SSEUrl`, `setContractHash`, `install(argsJson, wasm)` (`name`/`uri`/`payment_amount` + optional events/burn/filter/tx fields), `collectionName`, `balanceOf`, `balanceOfBatch`, `supplyOfBatch`, `totalSupplyOfBatch`, `totalFungibleSupply`, `enableBurn`, `eventsMode`, `numberOfMintedTokens`, `transferFilterContract`, `transferFilterMethod`, `securityBadge` |
| CEP-95 | `rpcUrl`, `SSEUrl`, `setContractHash`, `install`, `name`, `symbol`, `balanceOf`, `ownerOf`, `getOwner`, `transferOwnership` |

Full mutate/query parity lives in Rust `ceps-client`. The pack `.d.ts` exports CEP clients only (SDK feature `js` is not enabled on the path dependency).

## Schema feature (arg metadata)

For install / entrypoint defaults when there is no on-chain entrypoint metadata, enable the Cargo feature `schema` (pulled in with the default `client` feature, or alone for a lean pack).

```bash
make nodejs-schema   # → ceps-client-wasm/pkg-nodejs-schema
make web-schema      # → ceps-client-wasm/pkg-schema
make schema-check    # feature matrix + lean .d.ts assert
```

Schema-only packs export `schemaJson("cep18")`, `supportedCeps()`, and `cep18SchemaJson()` … `cep95SchemaJson()` — not `CEP18Client` / RPC helpers.

```js
import { cep18SchemaJson, schemaJson } from "ceps-client-wasm";

const schema = JSON.parse(cep18SchemaJson());
// or: JSON.parse(schemaJson("cep18"))
console.log(schema.install, schema.entrypoints.transfer);
```

## Node example

```js
import { CEP18Client } from "ceps-client-wasm";

const client = new CEP18Client(
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
