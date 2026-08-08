# ceps-rust-ts-client: Make entrypoints (humans + CI).
# Agents: use Cursor MCP for NCTL (`nctl_*`) and SDK (`sdk_*`); do not invent compose here.

ROOT := $(CURDIR)
CURRENT_DIR := .

# Sibling products (override if not adjacent)
NCTL_DOCKER_PRODUCT ?= $(ROOT)/../casper-nctl-2-docker
RUSTSDK_PRODUCT ?= $(ROOT)/../rustSDK
CEP18_PRODUCT ?= $(ROOT)/../cep-18
CEP78_PRODUCT ?= $(ROOT)/../cep-78-enhanced-nft
CEP85_PRODUCT ?= $(ROOT)/../cep-1155

NCTL_PROFILE ?= dev
NCTL_MCP_IMAGE ?= interchouette/casper-nctl-2-docker-mcp:$(NCTL_PROFILE)
CASPER_SDK_MCP_IMAGE ?= interchouette/casper-rust-wasm-sdk-mcp:dev

WASM_CRATE := ceps-wasm
CLI_CRATE := cli
COMMON_CRATE := ceps-client

WEB_OUT_DIR := pkg
NODEJS_OUT_DIR := pkg-nodejs
WASM_DIR := $(ROOT)/tests/wasm

# Docker / GHCR (nctl-style dual Hub + personal/org GHCR)
IMAGE_NAME := ceps-rust-ts-client
IMAGE_TAG ?= local
HUB_USER ?= interchouette
GHCR_PERSONAL ?= groussac
GHCR_ORG ?= interchouette-itc
DOCKER_CONTEXT_BIN ?= $(ROOT)/target/release/ceps
DOCKERFILE := $(ROOT)/docker/Dockerfile

# Pin Binaryen so wasm-pack does not fall back to vendored 117.
BINARYEN_VERSION := 131
BINARYEN_DIR := $(ROOT)/.tools/binaryen-version_$(BINARYEN_VERSION)
BINARYEN_BIN := $(BINARYEN_DIR)/bin
BINARYEN_PATH_FILE := $(ROOT)/.tools/wasm-opt-bin

# Clear Cursor sandbox cargo/playwright redirects for every recipe.
CARGO := env -u CARGO_TARGET_DIR -u PLAYWRIGHT_BROWSERS_PATH cargo

.DEFAULT_GOAL := help

.PHONY: help prepare ensure-binaryen \
	build check doc doc-check clean \
	format lint clippy check-lint \
	test unit-test integration-test e2e-test examples ts-test wasm-bindgen-test \
	pack web nodejs \
	run-cli \
	release-cli-bin \
	docker-build docker-tag docker-push-hub docker-push-ghcr docker-push \
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
	@echo "  doc / doc-check    rustdoc + closet file presence"
	@echo "  format / lint / clippy / check-lint"
	@echo "  clean              cargo clean + packed wasm dirs + .tools pin file"
	@echo ""
	@echo "Test"
	@echo "  test               unit + integration"
	@echo "  unit-test          ceps-client + ceps-wasm lib tests"
	@echo "  integration-test   tests/rust (NCTL for live cases)"
	@echo "  e2e-test           CLI-driven e2e"
	@echo "  examples           run examples/"
	@echo "  ts-test            Vitest against pkg-nodejs"
	@echo ""
	@echo "WASM pack"
	@echo "  pack / web / nodejs   wasm-pack release (needs ensure-binaryen)"
	@echo ""
	@echo "CLI"
	@echo "  run-cli            cargo run -p $(CLI_CRATE) -- \$$(CLI_ARGS)"
	@echo "  release-cli-bin    cargo build -p cli --release (+ strip)"
	@echo ""
	@echo "Docker / GHCR (IMAGE_TAG=dev|semver|latest)"
	@echo "  docker-build / docker-tag / docker-push-hub / docker-push-ghcr / docker-push"
	@echo ""
	@echo "Sibling NCTL / SDK MCP / Contracts"
	@echo "  nctl-start / nctl-status / nctl-endpoints"
	@echo "  sdk-mcp-http / sdk-mcp-http-stop"
	@echo "  wasm-from-ceps     stage WASMs from ceps-client-test siblings"
	@echo ""
	@echo "CI"
	@echo "  ci-local           check-lint + unit-test (no NCTL)"

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

build:
	$(CARGO) build --workspace

check:
	$(CARGO) check --workspace

doc:
	$(CARGO) doc -p $(COMMON_CRATE) --no-deps
	@mkdir -p docs/api-rust
	@rm -rf docs/api-rust/ceps_client docs/api-rust/src
	@cp -a target/doc/ceps_client docs/api-rust/
	@cp -a target/doc/src docs/api-rust/ 2>/dev/null || true
	@printf '%s\n' \
		'<!DOCTYPE html><html><head><meta charset="utf-8">' \
		'<meta http-equiv="refresh" content="0; url=ceps_client/index.html">' \
		'<title>ceps-client rustdoc</title></head><body>' \
		'<a href="ceps_client/index.html">ceps_client</a></body></html>' \
		> docs/api-rust/index.html
	@echo "doc: rustdoc → docs/api-rust/"

doc-check:
	@set -euo pipefail; \
	missing=0; \
	for f in \
		docs/README.md docs/getting-started.md docs/architecture.md docs/cli.md \
		docs/testing.md docs/contributing.md docs/wasm-ts.md docs/ci.md \
		docs/sdk.md docs/docker.md docs/SECURITY.md \
		docs/api-wasm/README.md \
		docs/cep18/README.md docs/cep18/1-quickstart.md docs/cep18/8-api.md \
		docs/cep78/README.md docs/cep78/2-install-modes.md docs/cep78/5-session-wasms.md docs/cep78/9-api.md \
		docs/cep85/README.md docs/cep85/6-entity-keys.md docs/cep85/9-api.md; do \
		if [ ! -f "$$f" ]; then echo "doc-check: missing $$f"; missing=1; fi; \
	done; \
	exit $$missing

clean:
	rm -rf $(WASM_CRATE)/$(WEB_OUT_DIR) $(WASM_CRATE)/$(NODEJS_OUT_DIR)
	rm -f "$(BINARYEN_PATH_FILE)"
	$(CARGO) clean

format:
	$(CARGO) fmt

lint: format clippy

clippy: prepare
	$(CARGO) clippy -p $(COMMON_CRATE) --lib -- -D warnings
	$(CARGO) clippy -p $(CLI_CRATE) --bins -- -D warnings
	$(CARGO) clippy -p $(WASM_CRATE) --lib -- -D warnings

check-lint: clippy
	$(CARGO) fmt -- --check

unit-test:
	$(CARGO) test -p $(COMMON_CRATE) -- --test-threads=1 --nocapture
	$(CARGO) test -p $(WASM_CRATE) --lib -- --test-threads=1 --nocapture

integration-test:
	cd tests/rust && $(CARGO) test -- --test-threads=1 --nocapture

e2e-test:
	@echo "e2e-test: run CLI status smoke"
	$(CARGO) run -p $(CLI_CRATE) -- status
	$(CARGO) run -p $(CLI_CRATE) -- cep18 info
	$(CARGO) run -p $(CLI_CRATE) -- cep78 info
	$(CARGO) run -p $(CLI_CRATE) -- cep85 info

examples:
	$(CARGO) run -p $(COMMON_CRATE) --example cep18_install
	$(CARGO) run -p $(COMMON_CRATE) --example cep78_install
	$(CARGO) run -p $(COMMON_CRATE) --example cep85_install

ts-test:
	@test -d $(WASM_CRATE)/$(NODEJS_OUT_DIR) || $(MAKE) nodejs
	cd tests/ts && npm install && npm test

wasm-bindgen-test: prepare
	cd $(WASM_CRATE) && wasm-pack test --node

test: unit-test integration-test

pack: web nodejs

web: ensure-binaryen prepare
	PATH="$$(cat "$(BINARYEN_PATH_FILE)"):$$PATH" \
		cd $(WASM_CRATE) && wasm-pack build --target web --release --out-dir $(WEB_OUT_DIR) $(CURRENT_DIR)

nodejs: ensure-binaryen prepare
	PATH="$$(cat "$(BINARYEN_PATH_FILE)"):$$PATH" \
		cd $(WASM_CRATE) && wasm-pack build --target nodejs --release --out-dir $(NODEJS_OUT_DIR) $(CURRENT_DIR)

run-cli:
	$(CARGO) run -p $(CLI_CRATE) -- $(CLI_ARGS)

release-cli-bin:
	$(CARGO) build -p $(CLI_CRATE) --release
	@strip -s "$(ROOT)/target/release/ceps" 2>/dev/null || strip "$(ROOT)/target/release/ceps"
	@echo "release-cli-bin: $(ROOT)/target/release/ceps"

docker-build:
	@test -f "$(DOCKER_CONTEXT_BIN)" || { \
		echo "docker-build: missing $(DOCKER_CONTEXT_BIN); run make release-cli-bin first"; \
		exit 1; \
	}
	cp -f "$(DOCKER_CONTEXT_BIN)" "$(ROOT)/docker/ceps"
	docker build -f "$(DOCKERFILE)" \
		-t "$(IMAGE_NAME):$(IMAGE_TAG)" \
		"$(ROOT)/docker"
	rm -f "$(ROOT)/docker/ceps"
	@echo "docker-build: $(IMAGE_NAME):$(IMAGE_TAG)"

docker-tag:
	docker tag "$(IMAGE_NAME):$(IMAGE_TAG)" "$(HUB_USER)/$(IMAGE_NAME):$(IMAGE_TAG)"
	docker tag "$(IMAGE_NAME):$(IMAGE_TAG)" "ghcr.io/$(GHCR_PERSONAL)/$(IMAGE_NAME):$(IMAGE_TAG)"
	docker tag "$(IMAGE_NAME):$(IMAGE_TAG)" "ghcr.io/$(GHCR_ORG)/$(IMAGE_NAME):$(IMAGE_TAG)"

docker-push-hub:
	docker push "$(HUB_USER)/$(IMAGE_NAME):$(IMAGE_TAG)"

docker-push-ghcr:
	docker push "ghcr.io/$(GHCR_PERSONAL)/$(IMAGE_NAME):$(IMAGE_TAG)"
	docker push "ghcr.io/$(GHCR_ORG)/$(IMAGE_NAME):$(IMAGE_TAG)"

docker-push: docker-tag docker-push-hub docker-push-ghcr

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

sdk-mcp-http:
	CASPER_SDK_MCP_IMAGE="$(CASPER_SDK_MCP_IMAGE)" \
		bash "$(ROOT)/.cursor/scripts/sdk-ensure.sh"

sdk-mcp-http-stop:
	-docker stop casper-rust-wasm-sdk-mcp-cursor 2>/dev/null
	-docker rm casper-rust-wasm-sdk-mcp-cursor 2>/dev/null

wasm-from-ceps:
	@mkdir -p "$(WASM_DIR)"
	@set -euo pipefail; \
	for pair in \
		"$(CEP18_PRODUCT)|cep18" \
		"$(CEP78_PRODUCT)|cep78" \
		"$(CEP85_PRODUCT)|cep85"; do \
		root="$${pair%%|*}"; name="$${pair##*|}"; \
		if [ ! -d "$$root" ]; then \
			echo "wasm-from-ceps: skip $$name (missing $$root)"; \
			continue; \
		fi; \
		# Prefer tests/wasm (tip stage), then newest release builds by mtime.
		found=$$( { \
			find "$$root/tests/wasm" -type f -name '*.wasm' 2>/dev/null; \
			find "$$root" -type f -name '*.wasm' \
				! -path '*/target/debug/*' ! -path '*/node_modules/*' \
				! -path '*/tests/wasm/*' 2>/dev/null; \
		} | awk 'NF' | while read -r f; do \
			printf '%s\t%s\n' "$$(stat -c '%Y' "$$f" 2>/dev/null || echo 0)" "$$f"; \
		done | sort -nr | cut -f2- | awk -F/ '{ base=$$NF; if (!seen[base]++) print }'); \
		if [ -z "$$found" ]; then \
			echo "wasm-from-ceps: no wasm under $$root (build contracts there first)"; \
			continue; \
		fi; \
		mkdir -p "$(WASM_DIR)/$$name"; \
		echo "$$found" | while read -r f; do \
			cp -f "$$f" "$(WASM_DIR)/$$name/"; \
			echo "  staged $$name/$$(basename "$$f")"; \
		done; \
	done; \
	echo "wasm-from-ceps: done → $(WASM_DIR)"

ci-local: check-lint unit-test doc-check
