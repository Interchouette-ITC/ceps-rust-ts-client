# Approvals and operators

| Method | Entrypoint / query |
| --- | --- |
| `approve` | `approve` (`operator` + token) |
| `revoke` | `revoke` |
| `set_approval_for_all` | `set_approval_for_all` (`approve_all` bool) |
| `get_approved` | dictionary `approved` / token id |
| `is_approved_for_all` | dictionary `operators` / blake2b(owner ‖ operator) |

Keys accept `account-hash-…`, `hash-…`, or bare 64-hex (treated as contract hash).
