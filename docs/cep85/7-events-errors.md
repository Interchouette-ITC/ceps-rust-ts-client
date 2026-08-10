# Events / errors

Events mode matches CEP-18 numbering (`NoEvents`, `CES`, `Native`, `NativeBytes`).

With CES mode, waited mutates soft-attach `CallResult.ces_events` when a contract hash is bound. Shared `CEPClient` helpers (`parse_ces_*` / `collect_ces_events`), CLI `ces *`, MCP `ceps_ces_*`, and WASM `parseCES` / `collectCESEvents` apply. Put signed JSON: library / CLI `put-transaction` / MCP `ceps_put_transaction` / WASM `putTransaction`.

Typical CES names: `Mint`, `MintBatch`, `Burn`, `BurnBatch`, `ApprovalForAll`, `Transfer`, `TransferBatch`, `Uri`, `UriBatch`, `SetTotalSupply`, `ChangeSecurity`, `SetModalities`, `Upgrade`, `ChangeEnableBurnMode`, `ChangeEventsMode`.

User errors are `1..=91` (`CEP85Error` in the contract). Common client mappings: `BurnDisabled=1`, `InsufficientBalance=2`, `NotApproved=5`, …
