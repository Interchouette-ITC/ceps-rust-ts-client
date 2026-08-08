# Mint / burn / security

| Method | Entrypoint | Notes |
| --- | --- | --- |
| `mint` | `mint` | Requires mint/burn enabled + minter rights |
| `burn` | `burn` | Burns from `owner` |
| `change_security` | `change_security` | At least one of `admin_list` / `minter_list` / `none_list` |
| `change_events_mode` | `change_events_mode` | `EventsMode` as U8 |

`is_mint_and_burn_enabled()` reads named key `enable_mint_burn`.
