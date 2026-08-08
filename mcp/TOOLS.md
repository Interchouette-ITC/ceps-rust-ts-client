# ceps-client-mcp tools

Total: **72** tools (`ceps_*`). Write tools take `secret_key_pem` + `payment_amount`.

| Tool | Group |
| --- | --- |
| `ceps_help` | meta |
| `ceps_get_endpoints` | meta |
| `ceps_set_endpoints` | meta |
| `ceps_list_tools` | meta |
| `ceps_list_contract_wasms` | wasm |
| `ceps_read_contract_wasm` | wasm |
| `ceps_canonical_wasm_paths` | wasm |
| `ceps18_install` | cep18 |
| `ceps18_upgrade` | cep18 |
| `ceps18_transfer` | cep18 |
| `ceps18_transfer_from` | cep18 |
| `ceps18_approve` | cep18 |
| `ceps18_increase_allowance` | cep18 |
| `ceps18_decrease_allowance` | cep18 |
| `ceps18_mint` | cep18 |
| `ceps18_burn` | cep18 |
| `ceps18_change_security` | cep18 |
| `ceps18_change_events_mode` | cep18 |
| `ceps18_name` | cep18 |
| `ceps18_symbol` | cep18 |
| `ceps18_decimals` | cep18 |
| `ceps18_total_supply` | cep18 |
| `ceps18_events_mode` | cep18 |
| `ceps18_is_mint_and_burn_enabled` | cep18 |
| `ceps18_balance_of` | cep18 |
| `ceps18_allowances` | cep18 |
| `ceps78_install` | cep78 |
| `ceps78_upgrade` | cep78 |
| `ceps78_mint` | cep78 |
| `ceps78_mint_session` | cep78 |
| `ceps78_burn` | cep78 |
| `ceps78_transfer` | cep78 |
| `ceps78_transfer_session` | cep78 |
| `ceps78_register_owner` | cep78 |
| `ceps78_approve` | cep78 |
| `ceps78_revoke` | cep78 |
| `ceps78_set_approval_for_all` | cep78 |
| `ceps78_set_token_metadata` | cep78 |
| `ceps78_set_variables` | cep78 |
| `ceps78_updated_receipts` | cep78 |
| `ceps78_collection_name` | cep78 |
| `ceps78_collection_symbol` | cep78 |
| `ceps78_total_token_supply` | cep78 |
| `ceps78_number_of_minted_tokens` | cep78 |
| `ceps78_events_mode` | cep78 |
| `ceps78_owner_of` | cep78 |
| `ceps78_balance_of` | cep78 |
| `ceps78_get_approved` | cep78 |
| `ceps78_is_approved_for_all` | cep78 |
| `ceps78_metadata` | cep78 |
| `ceps85_install` | cep85 |
| `ceps85_upgrade` | cep85 |
| `ceps85_mint` | cep85 |
| `ceps85_batch_mint` | cep85 |
| `ceps85_burn` | cep85 |
| `ceps85_batch_burn` | cep85 |
| `ceps85_transfer` | cep85 |
| `ceps85_batch_transfer` | cep85 |
| `ceps85_set_approval_for_all` | cep85 |
| `ceps85_set_uri` | cep85 |
| `ceps85_set_total_supply_of` | cep85 |
| `ceps85_set_total_supply_of_batch` | cep85 |
| `ceps85_change_security` | cep85 |
| `ceps85_set_modalities` | cep85 |
| `ceps85_collection_name` | cep85 |
| `ceps85_collection_uri` | cep85 |
| `ceps85_balance_of` | cep85 |
| `ceps85_is_approved_for_all` | cep85 |
| `ceps85_supply_of` | cep85 |
| `ceps85_total_supply_of` | cep85 |
| `ceps85_uri` | cep85 |
| `ceps85_is_non_fungible` | cep85 |

## Write legend

- **Write**: install/upgrade/mutate (signs with PEM, may put on-chain).
- **Read**: queries and WASM helpers (no chain write).

Pair with `sdk_*` for raw RPC and `nctl_*` for local net lifecycle.
