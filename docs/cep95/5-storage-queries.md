# Storage / queries

Named keys: `name` (String), `symbol` (String), `total_supply` (U256 when present; Odra tip may omit).

Dictionary encodings (match JS client README):

| Dict | Item key | Value |
| ---- | -------- | ----- |
| `balances` | Base64(CLValue(Key).bytes()) | U256 |
| `owners` / `approvals` / `token_metadata` | Base64(CLValue(U256).bytes()) | Key / Key / metadata pairs |
| `operators` | hex(blake2b-256(owner_key_bytes \|\| operator_key_bytes)) | Bool |
| `state` | hex(blake2b-256 of packed Odra Var path) | Ownable owner (`Option<Address>`) |

Ownable owner for tip `OwnedCEP95` uses path `[0, 0]` (module field 0, `owner` field 0). `get_owner()` reads that `state` item; `transfer_ownership(new_owner)` calls the Ownable entrypoint. Public `get_approved` maps on-chain `approved_for`.

Client reads use these dictionaries (not entrypoint round-trips), same spirit as the JS client.
