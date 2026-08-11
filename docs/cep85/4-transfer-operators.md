# Transfer / operators

Public `transfer` / `batch_transfer` call on-chain `transfer_from` / `batch_transfer_from`. Both take optional `data: Option<&[u8]>` (runtime byte list arg).

`set_approval_for_all` / `is_approved_for_all` use the `operators` dictionary (blake2b of owner‖operator keys).

Named keys `transfer_filter_contract` / `transfer_filter_method` are readable when a filter was set at install/upgrade.
