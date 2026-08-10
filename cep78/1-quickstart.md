# Quickstart

```bash
make wasm-from-ceps   # stages tests/wasm/cep78/*.wasm from tip builds
SECRET_KEY_USER_1="$(cat path/to/user-1/secret_key.pem)" \
  cargo run -p ceps-client --example cep78_install
```

Rust outline:

1. `CEP78Client::new(rpc, sse, chain, verbosity)`
2. `InstallArgs::new(name, symbol, supply).with_events_mode(EventsMode78::CES)`
3. `install(&args, &wasm, &TransactionParams::new(secret, "600000000000"))`
4. Read installer named keys `cep78_contract_hash_{name}` / `cep78_contract_package_{name}`
5. `set_contract_hash`, then `mint` / `owner_of` / `balance_of`
