# CEP-95 CLI

```bash
ceps-client-cli cep95 info
ceps-client-cli cep95 install --name N --symbol S --package-key-name KEY \
  --wasm tests/wasm/cep95/cep95.wasm --secret-key /path/to/secret_key.pem
ceps-client-cli cep95 mint --contract-hash <hash> --package-hash <hash> \
  --to <account-hash|…> --token-id 1 --secret-key /path/to/secret_key.pem
ceps-client-cli cep95 name --contract-hash <hash> [--package-hash <hash>]
ceps-client-cli cep95 balance --contract-hash <hash> --account <…>
ceps-client-cli cep95 owner-of --contract-hash <hash> --token-id 1
ceps-client-cli cep95 get-owner --contract-hash <hash> [--package-hash <hash>]
ceps-client-cli cep95 transfer-ownership --contract-hash <hash> --new-owner <…> \
  --secret-key /path/to/secret_key.pem
```

Also: `burn`, `transfer-from`, `approve`, `revoke-approval`, `approve-for-all`, `revoke-approval-for-all`, `symbol`, `get-approved`, `is-approved-for-all`.

Write commands support `--make-only` (same flag as CEP-18 / 78 / 85).
