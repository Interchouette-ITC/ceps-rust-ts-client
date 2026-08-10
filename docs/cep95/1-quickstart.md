# CEP-95 quickstart

```rust
use ceps_client::cep95::InstallArgs;
use ceps_client::{CEP95Client, TransactionParams, Verbosity};

let mut client = CEP95Client::new(
    "http://127.0.0.1:11101",
    Some("http://127.0.0.1:18101/events".into()),
    Some("casper-net-1".into()),
    Some(Verbosity::Low),
)?;
let args = InstallArgs::new("MyNft", "MNFT", "cep95_pkg_demo");
client.install(&args, &wasm, &TransactionParams::new(&secret, "600000000000")).await?;
let pk = /* installer public key hex */;
client.bind_odra_install(&pk, "cep95_pkg_demo").await?;
client.mint(&owner, "1", None, &TransactionParams::new(&secret, "5000000000")).await?;
let bal = client.balance_of(&owner).await?;
```

Installer account named key: the `package_hash_key_name` you pass to `InstallArgs` (Odra `odra_cfg_package_hash_key_name`). After install, `bind_odra_install` resolves the latest contract hash from that package.
