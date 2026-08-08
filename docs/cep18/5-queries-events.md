# Queries / events

## Named keys

`name`, `symbol`, `decimals`, `total_supply`, `events_mode`, `enable_mint_burn`

## Dictionaries

| Dict | Item key |
| --- | --- |
| `balances` | Base64(`Key` bytes) of the account / contract key |
| `allowances` | `blake2b(owner_key_bytes ‖ spender_key_bytes)` as hex |

Helpers: `balance_of`, `allowances`. SDK helpers `get_base64_key_from_account_hash` / `make_dictionary_item_key` are reused.

## Events / CES

`EventsMode::{NoEvents, Ces, Native, NativeBytes}`.

With `EventsMode::Ces`, waited mutates soft-attach `CallResult.ces_events` when the client has a bound contract hash. Shared helpers on `CepCore`:

- `ces_parser_create` / `parse_ces_execution` / `parse_ces_transaction`
- `collect_ces_events` (SSE `TransactionProcessed` + CES filter by event name)
- CLI `ces parse`, MCP `ceps_ces_*`

Typical CES names: `Mint`, `Burn`, `SetAllowance`, `IncreaseAllowance`, `DecreaseAllowance`, `Transfer`, `TransferFrom`, `ChangeSecurity`, `ChangeEventsMode`.
