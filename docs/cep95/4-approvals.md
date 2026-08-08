# Approvals

- `approve(spender, token_id)` / `revoke_approval(token_id)`
- `approve_for_all(operator)` / `revoke_approval_for_all(operator)`
- Queries: `get_approved(token_id)` (dict `approvals`), `is_approved_for_all(owner, operator)` (dict `operators`)
