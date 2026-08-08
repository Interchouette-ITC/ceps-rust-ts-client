# Testing

| Layer | Make target | Role |
| --- | --- | --- |
| Unit | `make unit-test` | No network; URL/key/error helpers |
| Integration | `make integration-test` | Lib against NCTL + tip WASMs |
| E2E | `make e2e-test` | CLI-driven |
| Examples | `make examples` | Demos, not CI gates by default |
| TS smoke | `make ts-test` | Vitest on `pkg-nodejs` |

Contract WASMs: build siblings on `ceps-client-test`, then `make wasm-from-ceps` → `tests/wasm/{cep18,cep78,cep85}/`.
