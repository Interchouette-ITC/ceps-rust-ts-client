# Install / upgrade

Install payment (JS e2e): `550000000000` motes.

`InstallArgs`: `name`, `uri`, optional `events_mode`, `enable_burn`, security lists, optional `transfer_filter_contract` + `transfer_filter_method` (both required together).

Upgrade sends `name` plus `upgrade: true`, and may set the same transfer-filter pair.
