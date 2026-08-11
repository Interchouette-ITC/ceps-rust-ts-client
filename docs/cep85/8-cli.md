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
  [--data-hex <hex>] --secret-key /path/to/secret_key.pem
ceps-client-cli cep85 set-approval-for-all --contract-hash <hash> --operator <…> --approved true \
  --secret-key /path/to/secret_key.pem
ceps-client-cli cep85 name --contract-hash <hash> [--package-hash <hash>]
ceps-client-cli cep85 balance --contract-hash <hash> --account <account-hash|entity-...> --id 1
ceps-client-cli cep85 balance-of-batch --contract-hash <hash> --accounts a,b --ids 1,2
ceps-client-cli cep85 supply-of-batch --contract-hash <hash> --ids 1,2
ceps-client-cli cep85 total-supply-of-batch --contract-hash <hash> --ids 1,2
ceps-client-cli cep85 total-fungible-supply --contract-hash <hash> --id 1
ceps-client-cli cep85 enable-burn --contract-hash <hash>
ceps-client-cli cep85 events-mode --contract-hash <hash>
ceps-client-cli cep85 number-of-minted-tokens --contract-hash <hash>
ceps-client-cli cep85 transfer-filter-contract --contract-hash <hash>
ceps-client-cli cep85 transfer-filter-method --contract-hash <hash>
ceps-client-cli cep85 security-badge --contract-hash <hash> --entity <account-hash|entity-...>
```

Write commands support `--make-only` (Transaction JSON, no put). Install may take `--transfer-filter-contract` + `--transfer-filter-method` together.
