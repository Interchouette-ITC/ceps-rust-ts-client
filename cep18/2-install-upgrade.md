# Install / upgrade

## Install

`Cep18Client::install(args, wasm, tx)`

| Arg | CL type | Notes |
| --- | --- | --- |
| `name` | String | Also anchors installer named keys |
| `symbol` | String | |
| `decimals` | U8 | |
| `total_supply` | U256 | Decimal string |
| `events_mode` | U8 | Optional; see `EventsMode` |
| `enable_mint_burn` | U8 | Optional `0`/`1` |
| `admin_list` / `minter_list` | List(Key) | Optional |

Payment (JS e2e reference): `400000000000` motes.

After a successful install, the installer account receives:

- `cep18_contract_hash_{name}`
- `cep18_contract_package_{name}`
- `cep18_contract_version_{name}`
- `cep18_contract_package_access_{name}`

Call `set_contract_hash` before queries or entrypoint calls.

## Upgrade

`Cep18Client::upgrade(args, wasm, tx)` with `UpgradeArgs { name, events_mode? }`.
