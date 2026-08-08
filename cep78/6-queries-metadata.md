# Queries and metadata

Contract named keys via `query_contract_key`: `collection_name`, `collection_symbol`, `total_token_supply`, `number_of_minted_tokens`, `events_mode`, and other install modalities.

Dictionaries:

| Dict | Item key |
| --- | --- |
| `token_owners` | token id decimal string or token hash |
| `balances` | owner key hex **without** prefix |
| `approved` | token identifier |
| `operators` | blake2b(owner_key_bytes ‖ operator_key_bytes) hex |
| `metadata_cep78` / `metadata_nft721` / `metadata_raw` / `metadata_custom_validated` | token identifier |

Helpers: `balance_of`, `owner_of`, `get_approved`, `is_approved_for_all`, `metadata(kind)`.
