# CEP-85 CLI

```bash
ceps-client-cli cep85 info
ceps-client-cli cep85 install --name N --uri 'https://example.com/metadata/{id}.json' \
  --wasm tests/wasm/cep85/cep85.wasm --secret-key /path/to/secret_key.pem
ceps-client-cli cep85 mint --contract-hash <hash> --recipient <…> --id 1 --amount 10 \
  --make-only --initiator-addr <pubkey-hex>
ceps-client-cli cep85 burn --contract-hash <hash> --owner <…> --id 1 --amount 1 \
  --secret-key /path/to/secret_key.pem
ceps-client-cli cep85 transfer --contract-hash <hash> --from <…> --to <…> --id 1 --amount 1 \
  --secret-key /path/to/secret_key.pem
ceps-client-cli cep85 set-approval-for-all --contract-hash <hash> --operator <…> --approved true \
  --secret-key /path/to/secret_key.pem
ceps-client-cli cep85 name --contract-hash <hash> [--package-hash <hash>]
ceps-client-cli cep85 balance --contract-hash <hash> --account <account-hash|entity-...> --id 1
```

Write commands support `--make-only` (Transaction JSON, no put).
