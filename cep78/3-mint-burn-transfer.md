# Mint, burn, transfer

| Method | Entrypoint | Notes |
| --- | --- | --- |
| `mint` | `mint` | `token_owner`, `token_meta_data`, optional `token_hash` |
| `mint_session` | session | Same args + `nft_contract_hash`; needs `mint_session.wasm` |
| `burn` | `burn` | `TokenIdentifier::Id` or `::Hash` |
| `transfer` | `transfer` | `source_key`, `target_key`, token id/hash |
| `transfer_session` | session | Adds `nft_contract_hash`; needs `transfer_session.wasm` |
| `register_owner` | `register_owner` | Required when reverse lookup is Complete / TransfersOnly |
| `set_token_metadata` | `set_token_metadata` | Mutable collections only (tip ABI; hash-id rewrite from #299 is excluded) |
| `set_variables` | `set_variables` | allow_minting / ACL / package operator flags |

Payment guidance on NCTL: install `600_000_000_000`, mutate `5_000_000_000`.
