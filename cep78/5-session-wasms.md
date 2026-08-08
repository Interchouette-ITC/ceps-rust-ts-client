# Session WASMs

Entrypoint calls are the default. Session companions store results under the caller's named keys or register receipts:

| WASM                               | Client method                   | When                                                        |
| ---------------------------------- | ------------------------------- | ----------------------------------------------------------- |
| `mint_session.wasm`                | `mint_session`                  | Mint + `register_owner` + receipt named keys                |
| `transfer_session.wasm`            | `transfer_session`              | Transfer with receipt updates                               |
| `balance_of_session.wasm`          | `balance_of_session`            | Persist balance under `key_name`                            |
| `owner_of_session.wasm`            | `owner_of_session`              | Persist owner under `key_name`                              |
| `get_approved_session.wasm`        | `get_approved_session`          | Persist approval under `key_name`                           |
| `is_approved_for_all_session.wasm` | `is_approved_for_all_session`   | Persist operator flag under `key_name`                      |
| `updated_receipts.wasm`            | `updated_receipts`              | Refresh pages; pass **package** hash as `nft_contract_hash` |

Stage via `make wasm-from-ceps` from the CEP-78 tip (prefer `tests/wasm/` over stale `target/` builds).
