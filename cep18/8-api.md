# CEP-18 API

Primary type: `ceps_client::CEP18Client`.

| Method | Kind |
| --- | --- |
| `new` / `set_contract_hash` / endpoint setters | setup |
| `install` / `upgrade` | mutate |
| `transfer` / `transfer_from` / `approve` / `increase_allowance` / `decrease_allowance` | mutate |
| `mint` / `burn` / `change_security` / `change_events_mode` | mutate |
| `name` / `symbol` / `decimals` / `total_supply` / `events_mode` / `is_mint_and_burn_enabled` | query |
| `balance_of` / `allowances` / `security_badge` | query |

Generate rustdoc: `cargo doc -p ceps-client --no-deps --open`.
