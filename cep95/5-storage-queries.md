# Storage / queries

Named keys: `name` (String), `symbol` (String), `total_supply` (U256 when present; Odra tip may omit).

Dictionary encodings (match JS client README):

| Dict | Item key | Value |
| ---- | -------- | ----- |
| `balances` | Base64(CLValue(Key).bytes()) | U256 |
| `owners` / `approvals` / `token_metadata` | Base64(CLValue(U256).bytes()) | Key / Key / metadata pairs |
| `operators` | hex(blake2b-256(owner_key_bytes \|\| operator_key_bytes)) | Bool |

Client reads use these dictionaries (not entrypoint round-trips), same spirit as the JS client.
