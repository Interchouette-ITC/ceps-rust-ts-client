# CLI

```bash
ceps-client-cli cep78 info
ceps-client-cli cep78 install --name N --symbol S --wasm tests/wasm/cep78/cep78.wasm \
  --secret-key /path/to/secret_key.pem
ceps-client-cli cep78 mint --contract-hash <hash> --token-owner <…> \
  --make-only --initiator-addr <pubkey-hex>
ceps-client-cli cep78 burn --contract-hash <hash> --token-id 0 --secret-key /path/to/secret_key.pem
ceps-client-cli cep78 transfer --contract-hash <hash> --source <…> --target <…> --token-id 0 \
  --secret-key /path/to/secret_key.pem
ceps-client-cli cep78 approve --contract-hash <hash> --operator <…> --token-id 0 \
  --secret-key /path/to/secret_key.pem
ceps-client-cli cep78 name --contract-hash <hash> [--package-hash <hash>]
ceps-client-cli cep78 balance --contract-hash <hash> --account <account-hash-…>
ceps-client-cli cep78 ownership-mode --contract-hash <hash> [--package-hash <hash>]
ceps-client-cli ces parse --contract-hash <hash> --transaction-hash <tx>
```

Burn / transfer / approve take exactly one of `--token-id` or `--token-hash`. Write commands support `--make-only`.
