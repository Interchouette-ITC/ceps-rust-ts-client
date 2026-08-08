# Entity keys

CEP-85 uses addressable-entity style prefixes for CL `Key` args and dictionary encoding:

| Input | Prefixed form |
| --- | --- |
| `account-hash-X` | `entity-account-X` |
| `hash-X` / bare hex | `entity-contract-X` |
| already `entity-*` | unchanged |

Helper: `ceps_client::cep85::prefixed_key`.
