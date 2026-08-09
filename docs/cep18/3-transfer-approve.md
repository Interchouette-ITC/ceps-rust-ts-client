# Transfer / approve

Entity keys use CEP-18 prefixes: `account-hash-…` or `hash-…` (not `entity-*`).

| Method               | Entrypoint           | Args                             |
| -------------------- | -------------------- | -------------------------------- |
| `transfer`           | `transfer`           | `recipient: Key`, `amount: U256` |
| `transfer_from`      | `transfer_from`      | `owner`, `recipient`, `amount`   |
| `approve`            | `approve`            | `spender`, `amount`              |
| `increase_allowance` | `increase_allowance` | `spender`, `amount`              |
| `decrease_allowance` | `decrease_allowance` | `spender`, `amount`              |

Each takes `&TransactionParams` (PEM secret, payment, wait / put flags). Default put + wait uses SSE (~120s timeout). Use `TransactionParams::make_only()` to return Transaction JSON without putting (optional PEM; unsigned needs `initiator_addr`).
