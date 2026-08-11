# CEP-85 API

`ceps_client::CEP85Client` - see rustdoc (`cargo doc -p ceps-client`).

Key methods: `install`, `upgrade`, `mint`, `batch_mint`, `burn`, `batch_burn`, `transfer` (optional `data`), `batch_transfer` (optional `data`), `set_approval_for_all`, `set_uri`, `set_total_supply_of`, `set_total_supply_of_batch`, `change_security`, `set_modalities`, `balance_of`, `balance_of_batch`, `supply_of`, `supply_of_batch`, `total_supply_of`, `total_supply_of_batch`, `total_fungible_supply`, `uri`, `is_non_fungible`, `collection_name`, `collection_uri`, `is_approved_for_all`, `enable_burn`, `events_mode`, `number_of_minted_tokens`, `transfer_filter_contract`, `transfer_filter_method`, `security_badge`.

Install / upgrade accept optional transfer-filter contract + method (both required together). Entity prefix helper: `ceps_client::cep85::prefixed_key`.
