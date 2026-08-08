# WASM / TypeScript (`ceps-wasm`)

Thin `wasm-bindgen` package exporting **CEP client APIs only**. Apps that need raw SDK calls should depend on `casper-rust-wasm-sdk` separately.

```bash
make nodejs   # → ceps-wasm/pkg-nodejs/
make ts-test  # Vitest smoke
```

Do not commit multi-MB `pkg/` artifacts; build in CI with `make pack`.
