# Events and errors

## Events mode

`EventsMode78::{NoEvents, Cep47, Ces, Native, NativeBytes}`. Install / upgrade / `events_mode` query.

## CES

With `EventsMode78::Ces`, mutates that wait attach soft-fail rows on `CallResult.ces_events` when a contract hash is bound (via rustSDK `CESParser`).

Parse explicitly:

```rust
let rows = client
    .core()
    .parse_ces_transaction(&[format!("hash-{contract}")], &tx_hash)
    .await?;
```

CLI: `ceps-client-cli ces parse --contract-hash <hash> --transaction-hash <tx>`.

MCP: `ceps_ces_parse_execution`, `ceps_ces_parse_transaction`.

Typical CES names for CEP-78: `Mint`, `Burn`, `Approval`, `ApprovalForAll`, `ApprovalRevoked`, `Transfer`, `MetadataUpdated`, `Migration`, `RevokedForAll`, `VariablesSet`.

## User errors

On-chain `NFTCoreError` codes `1..=180`. `Cep78Error` maps the full set (`InvalidAccount` … `InvalidVersionContractKey`). Execution failures surface as `CepError::Execution` with optional `user_error` code.
