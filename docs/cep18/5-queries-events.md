# Queries / events

## Named keys

`name`, `symbol`, `decimals`, `total_supply`, `events_mode`, `enable_mint_burn`

## Dictionaries

| Dict | Item key |
| --- | --- |
| `balances` | Base64(`Key` bytes) of the account / contract key |
| `allowances` | `blake2b(owner_key_bytes ‖ spender_key_bytes)` as hex |

Helpers: `balance_of`, `allowances`. SDK helpers `get_base64_key_from_account_hash` / `make_dictionary_item_key` are reused.

## Events

`EventsMode::{NoEvents, Ces, Native, NativeBytes}`. CES subscribe helpers will expand with the watcher/CES core.
