# Security Policy

## Product scope

**ceps-rust-ts-client** is a Rust library, clap CLI (`ceps-client-cli`), and JS packs (`ceps-client-wasm`) for driving Casper **CEP-18 / CEP-78 / CEP-85** contracts via [`casper-rust-wasm-sdk`](https://github.com/casper-ecosystem/casper-rust-wasm-sdk).

This repository is **not** a custodian, wallet, or hosted signing service. Callers supply keys and network endpoints.

## Keys and signing

| Concern | Behavior |
| --- | --- |
| Private keys / PEM / mnemonics | Supplied by the caller (CLI flags/env, library `DeployParams`, test env). The crates do not generate or store wallet seeds. |
| Where keys live | Process memory / caller-provided files only. Do not commit PEMs; `*.pem` is gitignored. |
| NCTL fixtures | Live tests may load public NCTL `user-*` PEMs or `SECRET_KEY_USER_*` env vars. Those keys are **testnet-only**. Never reuse them on mainnet. |
| WASM / Node | `ceps-client-wasm` install helpers accept PEM strings the same way; treat them as secrets in your app. |
| Logging | Avoid logging PEM bodies or full signed payloads in production verbosity. |

Install and mutate entrypoints sign with the secret material you pass in. Treat PEMs and env secrets as production credentials when pointed at non-test networks.

## Network trust boundary

- RPC and SSE URLs are caller-configured. A malicious or compromised peer can lie about chain state.
- Default URLs target local NCTL (`127.0.0.1:11101` / `18101`). Point them at testnet/mainnet only when you intend to.
- Contract WASM bytes you install are trusted input: verify provenance of tip builds before mainnet use.

## What we do not claim

- No remote key custody
- No built-in HSM / hardware-wallet bridge
- No guarantee that every CEP tip WASM is production-audited; tip SHAs are documented for reproducibility ([contributing.md](contributing.md))

## Reporting a vulnerability

Do **not** open a public GitHub issue for security-sensitive findings.

Prefer a private report via GitHub Security Advisories on this repository (or contact the maintainer listed in `Cargo.toml`).

Include enough detail to reproduce (affected surface, network, whether private key material was at risk). Legitimate reports will be acknowledged and coordinated before public disclosure.
