# CEP-85 quickstart

```rust
use ceps_client::cep85::InstallArgs;
use ceps_client::{Cep85Client, DeployParams, EventsMode, Verbosity};

let mut client = Cep85Client::new(
    "http://127.0.0.1:11101",
    Some("http://127.0.0.1:18101/events".into()),
    Some("casper-net-1".into()),
    Some(Verbosity::Low),
)?;
let args = InstallArgs::new("MyMulti", "https://example.com/{id}.json")
    .with_events_mode(EventsMode::Ces)
    .with_enable_burn(true);
client.install(&args, &wasm, &DeployParams::new(&secret, "550000000000")).await?;
// Bind cep85_contract_hash_{name} / cep85_contract_package_{name}
client.set_contract_hash(contract, Some(package))?;
client.mint(&owner, "1", "10", None, &DeployParams::new(&secret, "5000000000")).await?;
let bal = client.balance_of(&owner, "1").await?;
```

Installer named keys: `cep85_contract_hash_{name}`, `cep85_contract_package_{name}`.
