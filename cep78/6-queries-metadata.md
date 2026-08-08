# Queries and metadata

## Install-config getters

Named keys via `query_contract_key` (JS parity):

| Method | Named key |
| --- | --- |
| `collection_name` / `collection_symbol` | same |
| `total_token_supply` / `number_of_minted_tokens` | same |
| `events_mode` | `events_mode` |
| `allow_minting` | `allow_minting` |
| `minting_mode` / `whitelist_mode` / `burn_mode` / `holder_mode` / `identifier_mode` / `metadata_mutability` / `nft_kind` / `nft_metadata_kind` / `ownership_mode` | matching `*_mode` / kind keys |
| `reporting_mode` | `reporting_mode` (owner reverse lookup) |
| `operator_burn_mode` / `package_operator_mode` / `acl_package_mode` | bool flags |
| `json_schema` | `json_schema` |
| `is_acl_whitelisted(entity)` | dictionary `acl_whitelist` (hex body item key) |

## Dictionaries

| Dict | Item key |
| --- | --- |
| `token_owners` | token id decimal string or token hash |
| `balances` | owner key hex **without** prefix |
| `approved` | token identifier |
| `operators` | blake2b(owner_key_bytes ‖ operator_key_bytes) hex |
| `metadata_cep78` / `metadata_nft721` / `metadata_raw` / `metadata_custom_validated` | token identifier |

Helpers: `balance_of`, `owner_of`, `get_approved`, `is_approved_for_all`, `metadata(kind)`.
