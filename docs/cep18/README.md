# CEP-18 client closet

Fungible token client (`Cep18Client`) for CEP-18 contracts on the `ceps-client-test` tip.

## TOC

1. [Quickstart](1-quickstart.md)
2. [Install / upgrade](2-install-upgrade.md)
3. [Transfer / approve](3-transfer-approve.md)
4. [Mint / burn / security](4-mint-burn-security.md)
5. [Queries / events](5-queries-events.md)
6. [Errors](6-errors.md) (`User error: 60000+`)
7. [CLI](7-cli.md)
8. [API](8-api.md)

## When to use

Use this closet when you need a Rust (or CLI) client for CEP-18 balances, allowances, mint/burn, and security lists. On-chain semantics live in the [cep-18 contract docs](https://github.com/casper-ecosystem/cep18); this closet covers the **client** surface only.
