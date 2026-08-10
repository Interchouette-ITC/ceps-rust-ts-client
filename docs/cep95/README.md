# CEP-95 client closet

Supported simpler ERC-721-shaped NFT client (`CEP95Client`) with JS-client API parity and Odra OwnedCEP95 tip install. Spec: [0095-nft-standard](https://github.com/casper-network/ceps/blob/master/text/0095-nft-standard.md). Demo WASM: `tests/wasm/cep95/cep95.wasm` (from `Interchouette-ITC/cep-95` @ `ceps-client-test`).

CEP-95 does **not** replace [CEP-78](../cep78/) in this client. CEP-78 remains the first-class enhanced / configurable NFT surface; use CEP-95 when you want the fixed simpler API. The Odra tip has no CES `__events_schema` path: wait via SSE only.

## TOC

1. [Quickstart](1-quickstart.md)
2. [Install (Odra)](2-install.md)
3. [Mint / burn / transfer](3-mint-burn-transfer.md)
4. [Approvals](4-approvals.md)
5. [Storage / queries](5-storage-queries.md)
6. [Errors](6-errors.md)
7. [CLI](7-cli.md)
8. [API](8-api.md)
