# Queries / events

## Named keys

`name`, `symbol`, `decimals`, `total_supply`, `events_mode`, `enable_mint_burn`

## Dictionaries

| Dict         | Item key                                              |
| ------------ | ----------------------------------------------------- |
| `balances`   | Base64(`Key` bytes) of the account / contract key     |
| `allowances` | `blake2b(owner_key_bytes ‖ spender_key_bytes)` as hex |
| `security_badges` | Same Base64(`Key` bytes) encoding as `balances` |

Helpers: `balance_of`, `allowances`, `security_badge`. SDK helpers `get_base64_key_from_account_hash` / `make_dictionary_item_key` are reused.

## Events / CES

`EventsMode::{NoEvents, CES, Native, NativeBytes}`.

With `EventsMode::CES`, waited mutates soft-attach `CallResult.ces_events` when the client has a bound contract hash. Shared helpers on `CEPClient`:

- `parse_ces_execution` / `parse_ces_transaction`
- `collect_ces_events` (SSE `TransactionProcessed` + CES filter by event name)
- CLI `ces parse` / `ces parse-execution` / `ces collect`, MCP `ceps_ces_*`
- Put signed JSON: CLI `put-transaction`, MCP `ceps_put_transaction`; wait later: CLI `wait-transaction`, MCP `ceps_wait_transaction`

Typical CES names: `Mint`, `Burn`, `SetAllowance`, `IncreaseAllowance`, `DecreaseAllowance`, `Transfer`, `TransferFrom`, `ChangeSecurity`, `ChangeEventsMode`.
