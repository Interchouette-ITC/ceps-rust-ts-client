# Mint / burn / transfer

- `mint(to, token_id, metadata?)` - owner-gated on OwnedCEP95; metadata defaults to an empty list (JS calls omit metadata).
- `burn(token_id)` - token-owner gated on the tip.
- `transfer_from(from, to, token_id)`
- `safe_transfer_from(from, to, token_id, data?)` - optional receiver data bytes.
