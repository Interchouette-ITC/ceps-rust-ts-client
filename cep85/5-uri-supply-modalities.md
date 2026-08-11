# URI / supply / modalities

| Method | Notes |
| --- | --- |
| `set_uri` | Optional per-id override |
| `collection_uri` | Named key `uri` |
| `set_total_supply_of` / `_batch` | Caps |
| `supply_of` / `total_supply_of` | Dicts `supply` / `total_supply` keyed by id string |
| `supply_of_batch` / `total_supply_of_batch` / `balance_of_batch` | Client-side dict loops |
| `total_fungible_supply` | `total_supply_of - supply_of`; `None` when cap unset/zero |
| `enable_burn` / `events_mode` / `number_of_minted_tokens` | Named-key readers |
| `security_badge` | Dict `security_badges` with hex(Key.to_bytes()) item keys |
| `set_modalities` | `enable_burn` and/or `events_mode` |
