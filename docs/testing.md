# Testing

| Layer       | Make target             | Role                                             |
| ----------- | ----------------------- | ------------------------------------------------ |
| Unit        | `make unit-test`        | No network; URL/key/error helpers                |
| Integration | `make integration-test` | Lib against NCTL + tip WASMs                     |
| CLI smoke   | `make e2e-test`         | `ceps status` + `cep18` / `cep78` / `cep85 info` |
| Examples    | `make examples`         | Install demos (needs NCTL + secrets)             |
| TS smoke    | `make ts-test`          | Vitest on `pkg-nodejs`                           |
| CI          | see [ci.md](ci.md)      | Gates, Hub/GHCR, Release assets                  |

Contract WASMs: tip builds on `ceps-client-test`, then `make wasm-from-ceps` → `tests/wasm/{cep18,cep78,cep85}/`.
