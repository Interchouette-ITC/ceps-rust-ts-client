# CEP-18 CLI

```bash
ceps [--rpc-url …] [--sse-url …] [--chain-name …] [--json] cep18 <cmd>
```

| Command | Status |
| --- | --- |
| `info` | Prints bound endpoints |
| `install` / `transfer` / `mint` / … | Landed next to lib methods; prefer lib for scripting until fully wired |

Global defaults match NCTL `dev`. See [docs/cli.md](../cli.md).
