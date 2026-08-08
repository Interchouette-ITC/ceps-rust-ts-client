# Install (Odra)

Install payment (demo): `600000000000` motes.

`InstallArgs`: `name`, `symbol`, `package_hash_key_name`, plus Odra flags `allow_key_override`, `is_upgradable`, `is_upgrade` (defaults: no override, upgradable, not an upgrade).

Session WASM is the tip `cep95.wasm` (OwnedCep95). There is no separate upgrade session in v1 beyond Odra `is_upgrade` when replacing a package.

After a successful install, call `bind_odra_install(installer_public_key, package_hash_key_name)` to bind contract + package hashes on the client.
