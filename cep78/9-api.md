# CEP-78 API

`ceps_client::CEP78Client` - see rustdoc (`cargo doc -p ceps-client`).

Install: `install`, `upgrade`.

Mutate: `mint`, `mint_session`, `burn`, `transfer`, `transfer_session`, `register_owner`, `approve`, `revoke`, `set_approval_for_all`, `set_token_metadata`, `set_variables`, `updated_receipts`.

Query: `collection_name`, `collection_symbol`, `total_token_supply`, `number_of_minted_tokens`, `events_mode`, install-config getters (`allow_minting`, `*_mode`, `json_schema`, `is_acl_whitelisted`, …), `owner_of`, `balance_of`, `get_approved`, `is_approved_for_all`, `metadata`.

Query sessions: `owner_of_session`, `balance_of_session`, `get_approved_session`, `is_approved_for_all_session`.

Modes live under `ceps_client::cep78::{OwnershipMode, IdentifierMode, …}`. CES: `CEPClient::parse_ces_*` / `collect_ces_events` / `CallResult.ces_events`.
