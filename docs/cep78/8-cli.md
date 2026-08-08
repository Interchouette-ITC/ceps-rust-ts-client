# CLI

```bash
ceps-client-cli cep78 info
ceps-client-cli cep78 name --contract-hash <hash> [--package-hash <hash>]
ceps-client-cli cep78 balance --contract-hash <hash> --account <account-hash-…>
ceps-client-cli cep78 ownership-mode --contract-hash <hash> [--package-hash <hash>]
ceps-client-cli ces parse --contract-hash <hash> --transaction-hash <tx>
```

Install / mint flows: `cargo run -p ceps-client --example cep78_install`.
