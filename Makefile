prepare:
	rustup target add wasm32-unknown-unknown

CURRENT_DIR = .

# Specify the output directories for web and Node.js targets.
WEB_OUT_DIR = pkg
NODEJS_OUT_DIR = pkg-nodejs

.PHONY: all web nodejs clean build doc

pack: web nodejs

web:
	cd ceps-ts-client && wasm-pack build --target web --release --out-dir $(WEB_OUT_DIR) $(CURRENT_DIR)

nodejs:
	cd ceps-ts-client && wasm-pack build --target nodejs --release --out-dir $(NODEJS_OUT_DIR) $(CURRENT_DIR)

clean:
	rm -rf $(WEB_OUT_DIR) $(NODEJS_OUT_DIR)
	cargo clean

wasm-bindgen-test:
	cd ceps-ts-client && wasm-pack test --headless --chrome

unit-test:
	cargo test -- --test-threads=1 --nocapture

integration-test:
	cd tests/rust && cargo test -- --test-threads=1 --nocapture

ts-test:
	cd tests/ts && npm install && npm test

test: unit-test integration-test wasm-bindgen-test ts-test

doc:

build: pack

format:
	cargo fmt

lint: format clippy

clippy:
	cargo clippy --target wasm32-unknown-unknown --bins -- -D warnings
	cargo clippy --lib -- -D warnings
	cargo clippy --no-default-features --lib -- -D warnings

check-lint: clippy
	cargo fmt -- --check

.PHONY: format lint check clippy