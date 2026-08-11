# Transfer / operators

Public `transfer` / `batch_transfer` call on-chain `transfer_from` / `batch_transfer_from`. Both take optional `data: Option<&[u8]>`: when `Some`, a `List(U8)` / `Bytes` runtime arg is sent; when `None`, the arg is omitted (matches the contract entrypoint).

`set_approval_for_all` / `is_approved_for_all` use the `operators` dictionary (blake2b of owner‖operator keys).

Named keys `transfer_filter_contract` / `transfer_filter_method` are readable when a filter was set at install/upgrade.
