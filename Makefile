# ceps-rust-ts-client — Make entrypoints (humans + CI).
# Agents: use Cursor MCP for NCTL (`nctl_*`) and SDK (`sdk_*`); do not invent compose here.

ROOT := $(CURDIR)
CURRENT_DIR := .

# Sibling products (override if not adjacent under /opt2/casper)
NCTL_DOCKER_PRODUCT ?= $(ROOT)/../casper-nctl-2-docker
RUSTSDK_PRODUCT ?= $(ROOT)/../rustSDK
CEP18_PRODUCT ?= $(ROOT)/../cep-18
CEP78_PRODUCT ?= $(ROOT)/../cep-78-enhanced-nft
CEP85_PRODUCT ?= $(ROOT)/../cep-1155

NCTL_PROFILE ?= dev
NCTL_MCP_IMAGE ?= interchouette/casper-nctl-2-docker-mcp:$(NCTL_PROFILE)
CASPER_SDK_MCP_IMAGE ?= interchouette/casper-rust-wasm-sdk-mcp:dev

# Crate dirs (rename later: console→cli, ceps-ts-client→ceps-wasm, common→ceps-client)
WASM_CRATE := ceps-ts-client
CLI_CRATE := console
COMMON_CRATE := common

WEB_OUT_DIR := pkg
NODEJS_OUT_DIR := pkg-nodejs
WASM_DIR := $(ROOT)/tests/wasm

# Pin Binaryen so wasm-pack does not fall back to vendored 117.
BINARYEN_VERSION := 130
BINARYEN_DIR := $(ROOT)/.tools/binaryen-version_$(BINARYEN_VERSION)
BINARYEN_BIN := $(BINARYEN_DIR)/bin
BINARYEN_PATH_FILE := $(ROOT)/.tools/wasm-opt-bin

.DEFAULT_GOAL := help

.PHONY: help prepare ensure-binaryen \
	build check doc clean \
	format lint clippy check-lint \
	test unit-test integration-test e2e-test examples ts-test wasm-bindgen-test \
	pack web nodejs \
	run-cli \
	nctl-start nctl-start-all nctl-stop nctl-stop-all nctl-status nctl-endpoints \
	sdk-mcp-http sdk-mcp-http-stop \
	wasm-from-ceps \
	ci-local

help:
	@echo "ceps-rust-ts-client Make targets"
	@echo ""
	@echo "Build / lint"
	@echo "  prepare            rustup wasm32 target"
	@echo "  build              cargo build --workspace"
	@echo "  check              cargo check --workspace"
	@echo "  format / lint / clippy / check-lint"
	@echo "  clean              cargo clean + packed wasm dirs + .tools pin file"
	@echo ""
	@echo "Test"
	@echo "  test               unit + integration (no live NCTL required for unit)"
	@echo "  unit-test          cargo test (workspace libs)"
	@echo "  integration-test   tests/rust (NCTL expected for live cases)"
	@echo "  e2e-test           CLI-driven e2e (placeholder until CLI lands)"
	@echo "  examples           run examples/ (placeholder)"
	@echo "  ts-test            Vitest against pkg-nodejs"
	@echo "  wasm-bindgen-test  headless chrome wasm tests"
	@echo ""
	@echo "WASM pack"
	@echo "  pack / web / nodejs   wasm-pack release (needs ensure-binaryen)"
	@echo "  ensure-binaryen       pin Binaryen $(BINARYEN_VERSION)"
	@echo ""
	@echo "CLI"
	@echo "  run-cli            cargo run -p $(CLI_CRATE) -- \$$(CLI_ARGS)"
	@echo ""
	@echo "Sibling NCTL (humans/CI; agents use MCP nctl_*)"
	@echo "  nctl-start / nctl-start-all / nctl-stop / nctl-stop-all"
	@echo "  nctl-status / nctl-endpoints   profile=\$$(NCTL_PROFILE) default $(NCTL_PROFILE)"
	@echo "  NCTL_DOCKER_PRODUCT=$(NCTL_DOCKER_PRODUCT)"
	@echo ""
	@echo "Sibling SDK MCP HTTP :5790 (agents prefer CallMcpTool)"
	@echo "  sdk-mcp-http / sdk-mcp-http-stop"
	@echo ""
	@echo "Contracts"
	@echo "  wasm-from-ceps     stage contract WASMs from sibling CEP repos into tests/wasm"
	@echo ""
	@echo "CI"
	@echo "  ci-local           check-lint + unit-test + pack nodejs (no NCTL)"

prepare:
	rustup target add wasm32-unknown-unknown

ensure-binaryen:
	@set -euo pipefail; \
	mkdir -p "$(ROOT)/.tools"; \
	is_pin() { "$$1" --version 2>/dev/null | grep -qE 'version[_ ]$(BINARYEN_VERSION)([^0-9]|$$)'; }; \
	if command -v wasm-opt >/dev/null 2>&1 && is_pin wasm-opt; then \
		dirname "$$(command -v wasm-opt)" > "$(BINARYEN_PATH_FILE)"; \
		echo "ensure-binaryen: $$(wasm-opt --version) (PATH)"; \
		exit 0; \
	fi; \
	if [ -x "$(BINARYEN_BIN)/wasm-opt" ] && is_pin "$(BINARYEN_BIN)/wasm-opt"; then \
		echo "$(BINARYEN_BIN)" > "$(BINARYEN_PATH_FILE)"; \
		echo "ensure-binaryen: $$($(BINARYEN_BIN)/wasm-opt --version) ($(BINARYEN_BIN))"; \
		exit 0; \
	fi; \
	case "$$(uname -m)" in \
		x86_64|amd64) arch=x86_64 ;; \
		aarch64|arm64) arch=aarch64 ;; \
		*) echo "ensure-binaryen: unsupported arch $$(uname -m)" >&2; exit 1 ;; \
	esac; \
	case "$$(uname -s)" in \
		Linux) plat=linux ;; \
		Darwin) plat=macos ;; \
		*) echo "ensure-binaryen: unsupported OS $$(uname -s)" >&2; exit 1 ;; \
	esac; \
	url="https://github.com/WebAssembly/binaryen/releases/download/version_$(BINARYEN_VERSION)/binaryen-version_$(BINARYEN_VERSION)-$${arch}-$${plat}.tar.gz"; \
	echo "ensure-binaryen: downloading $$url"; \
	rm -rf "$(BINARYEN_DIR)"; \
	curl -fsSL "$$url" | tar -xz -C "$(ROOT)/.tools"; \
	test -x "$(BINARYEN_BIN)/wasm-opt"; \
	is_pin "$(BINARYEN_BIN)/wasm-opt" || { echo "ensure-binaryen: expected version $(BINARYEN_VERSION)" >&2; exit 1; }; \
	echo "$(BINARYEN_BIN)" > "$(BINARYEN_PATH_FILE)"; \
	echo "ensure-binaryen: $$($(BINARYEN_BIN)/wasm-opt --version)"

# --- Build / lint ---

build:
	cargo build --workspace

check:
	cargo check --workspace

doc:
	cargo doc --workspace --no-deps

clean:
	rm -rf $(WASM_CRATE)/$(WEB_OUT_DIR) $(WASM_CRATE)/$(NODEJS_OUT_DIR)
	rm -f "$(BINARYEN_PATH_FILE)"
	cargo clean

format:
	cargo fmt

lint: format clippy

clippy: prepare
	cargo clippy -p $(COMMON_CRATE) --lib -- -D warnings
	cargo clippy -p $(CLI_CRATE) --bins -- -D warnings
	cargo clippy -p $(WASM_CRATE) --lib -- -D warnings
	cargo clippy -p $(WASM_CRATE) --target wasm32-unknown-unknown --lib -- -D warnings

check-lint: clippy
	cargo fmt -- --check

# --- Tests ---

unit-test:
	cargo test -p $(COMMON_CRATE) -- --test-threads=1 --nocapture
	cargo test -p $(WASM_CRATE) --lib -- --test-threads=1 --nocapture

integration-test:
	cd tests/rust && cargo test -- --test-threads=1 --nocapture

e2e-test:
	@echo "e2e-test: CLI e2e not implemented yet (Phase 1+). Use integration-test for now."
	@exit 1

examples:
	@echo "examples: no examples/ yet. Add under examples/rust/ per plan."
	@exit 1

ts-test:
	@test -d $(WASM_CRATE)/$(NODEJS_OUT_DIR) || $(MAKE) nodejs
	cd tests/ts && npm install && npm test

wasm-bindgen-test: prepare
	cd $(WASM_CRATE) && wasm-pack test --headless --chrome

test: unit-test integration-test

# --- WASM pack ---

pack: web nodejs

web: ensure-binaryen prepare
	PATH="$$(cat "$(BINARYEN_PATH_FILE)"):$$PATH" \
		cd $(WASM_CRATE) && wasm-pack build --target web --release --out-dir $(WEB_OUT_DIR) $(CURRENT_DIR)

nodejs: ensure-binaryen prepare
	PATH="$$(cat "$(BINARYEN_PATH_FILE)"):$$PATH" \
		cd $(WASM_CRATE) && wasm-pack build --target nodejs --release --out-dir $(NODEJS_OUT_DIR) $(CURRENT_DIR)

# --- CLI ---

run-cli:
	cargo run -p $(CLI_CRATE) -- $(CLI_ARGS)

# --- Sibling NCTL (forward to casper-nctl-2-docker Make; agents: MCP) ---

define require_nctl
	@test -d "$(NCTL_DOCKER_PRODUCT)" || { \
		echo "NCTL product not found at $(NCTL_DOCKER_PRODUCT)"; \
		echo "Set NCTL_DOCKER_PRODUCT or clone casper-nctl-2-docker as a sibling."; \
		exit 1; \
	}
endef

nctl-start:
	$(require_nctl)
	$(MAKE) -C "$(NCTL_DOCKER_PRODUCT)" start $(NCTL_PROFILE)

nctl-start-all:
	$(require_nctl)
	$(MAKE) -C "$(NCTL_DOCKER_PRODUCT)" start-all $(NCTL_PROFILE)

nctl-stop:
	$(require_nctl)
	$(MAKE) -C "$(NCTL_DOCKER_PRODUCT)" stop $(NCTL_PROFILE)

nctl-stop-all:
	$(require_nctl)
	$(MAKE) -C "$(NCTL_DOCKER_PRODUCT)" stop-all $(NCTL_PROFILE)

nctl-status:
	$(require_nctl)
	$(MAKE) -C "$(NCTL_DOCKER_PRODUCT)" status $(NCTL_PROFILE)

nctl-endpoints:
	$(require_nctl)
	@echo "RPC  http://127.0.0.1:11101"
	@echo "SSE  http://127.0.0.1:18101/events"
	@echo "BIN  127.0.0.1:28101"
	@echo "NCTL MCP HTTP  http://127.0.0.1:8790/mcp"
	@echo "SDK  MCP HTTP  http://127.0.0.1:5790/mcp"
	@echo "NCTL_DOCKER_PRODUCT=$(NCTL_DOCKER_PRODUCT) profile=$(NCTL_PROFILE)"

# --- SDK MCP HTTP (Hub image via .cursor scripts) ---

sdk-mcp-http:
	CASPER_SDK_MCP_IMAGE="$(CASPER_SDK_MCP_IMAGE)" \
		bash "$(ROOT)/.cursor/scripts/sdk-ensure.sh"

sdk-mcp-http-stop:
	-docker stop casper-rust-wasm-sdk-mcp-cursor 2>/dev/null
	-docker rm casper-rust-wasm-sdk-mcp-cursor 2>/dev/null

# --- Contract WASMs from sibling CEP repos ---

wasm-from-ceps:
	@mkdir -p "$(WASM_DIR)"
	@set -euo pipefail; \
	copied=0; \
	for pair in \
		"$(CEP18_PRODUCT)|cep18" \
		"$(CEP78_PRODUCT)|cep78" \
		"$(CEP85_PRODUCT)|cep85"; do \
		root="$${pair%%|*}"; name="$${pair##*|}"; \
		if [ ! -d "$$root" ]; then \
			echo "wasm-from-ceps: skip $$name (missing $$root)"; \
			continue; \
		fi; \
		found=$$(find "$$root" -type f -name '*.wasm' \
			! -path '*/target/debug/*' ! -path '*/node_modules/*' \
			2>/dev/null | head -20); \
		if [ -z "$$found" ]; then \
			echo "wasm-from-ceps: no wasm under $$root (build contracts there first)"; \
			continue; \
		fi; \
		mkdir -p "$(WASM_DIR)/$$name"; \
		echo "$$found" | while read -r f; do \
			cp -f "$$f" "$(WASM_DIR)/$$name/"; \
			echo "  staged $$name/$$(basename "$$f")"; \
			copied=1; \
		done; \
	done; \
	echo "wasm-from-ceps: done → $(WASM_DIR)"

# --- Local CI smoke (no NCTL) ---

ci-local: check-lint unit-test
