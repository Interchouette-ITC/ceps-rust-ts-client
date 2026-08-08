# CEP-78 API

`ceps_client::Cep78Client` - see rustdoc (`cargo doc -p ceps-client`).

Install: `install`, `upgrade`.

Mutate: `mint`, `mint_session`, `burn`, `transfer`, `transfer_session`, `register_owner`, `approve`, `revoke`, `set_approval_for_all`, `set_token_metadata`, `set_variables`, `updated_receipts`.

Query: `collection_name`, `collection_symbol`, `total_token_supply`, `number_of_minted_tokens`, `events_mode`, `owner_of`, `balance_of`, `get_approved`, `is_approved_for_all`, `metadata`.

Modes live under `ceps_client::cep78::{OwnershipMode, IdentifierMode, …}`.
