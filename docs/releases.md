# Release artefacts

Each GitHub Release (stable `vX.Y.Z` or Pre-release `dev-preview`) attaches downloadable packs so you do not need to build the client WASM or demo tip contracts locally.

Base URL (replace tag):

```text
https://github.com/gRoussac/ceps-rust-ts-client/releases/download/<tag>/
```

Examples: `v1.0.0`, `dev-preview`.

## Assets

| File | Contents |
| --- | --- |
| `ceps-{label}-linux-x86_64` | Stripped CLI binary |
| `ceps-wasm-nodejs-{label}.tgz` | Client pack for **Node** (`pkg-nodejs/`) |
| `ceps-wasm-web-{label}.tgz` | Client pack for **browsers** (`pkg/`) |
| `ceps-contracts-{label}.tgz` | Demo tip **on-chain** WASMs (`cep18/`, `cep78/`, `cep85/`) |
| `ceps-contracts-{label}.MANIFEST.txt` | File list / tip note |
| `SHA256SUMS` | Checksums |

`{label}` is the tag without a leading `v` (`1.0.0` or `dev-preview`).

Same idea as using rustSDK’s published `pkg` / `pkg-nodejs`: unpack and point your app at the folder (or `file:` dependency). npm registry publish is not required for that workflow.

## Fetch client WASM (Node)

```bash
TAG=v1.0.0
LABEL=${TAG#v}
curl -fsSL -o ceps-wasm-nodejs.tgz \
  "https://github.com/gRoussac/ceps-rust-ts-client/releases/download/${TAG}/ceps-wasm-nodejs-${LABEL}.tgz"
mkdir -p ceps-wasm && tar -xzf ceps-wasm-nodejs.tgz -C ceps-wasm
# → ceps-wasm/pkg-nodejs/
```

In `package.json`:

```json
"dependencies": {
  "ceps-wasm": "file:./ceps-wasm/pkg-nodejs"
}
```

Web pack: download `ceps-wasm-web-${LABEL}.tgz` and unpack to `ceps-wasm/pkg/`.

## Fetch demo contract WASMs

```bash
TAG=v1.0.0
LABEL=${TAG#v}
curl -fsSL -o ceps-contracts.tgz \
  "https://github.com/gRoussac/ceps-rust-ts-client/releases/download/${TAG}/ceps-contracts-${LABEL}.tgz"
mkdir -p tests/wasm && tar -xzf ceps-contracts.tgz -C tests/wasm
# → tests/wasm/cep18/*.wasm , cep78/ , cep85/
```

Use those bytes in `install(...)` (Rust or JS). They are **demo tip** builds (see [contributing.md](contributing.md)), not a substitute for auditing mainnet contracts.

## Build locally instead

```bash
make pack              # client Node + web packs
make wasm-from-ceps    # contracts from tip checkouts
make release-cli-bin   # CLI
```

CI wiring: [ci.md](ci.md).
