# Runtime image for ceps-rust-ts-client-mcp (stdio default, HTTP :6790).
# Build binary on the host / CI first, then:
#   make docker-build-mcp IMAGE_TAG=local
# Packs the stripped `ceps-client-mcp` binary (crate shortcut name).
ARG RUNTIME_IMAGE=debian:bookworm-slim

FROM ${RUNTIME_IMAGE}

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

COPY ceps-client-mcp /usr/local/bin/ceps-client-mcp
RUN chmod +x /usr/local/bin/ceps-client-mcp

COPY wasm /opt/ceps/tests/wasm

ENV CEPS_MCP_ADDR=0.0.0.0:6790 \
    CEPS_RPC_URL=http://host.docker.internal:11101 \
    CEPS_SSE_URL=http://host.docker.internal:18101/events \
    CEPS_CHAIN_NAME=casper-net-1 \
    CEPS_WASM_ROOT=/opt/ceps/tests/wasm \
    RUST_LOG=warn \
    SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

EXPOSE 6790

ENTRYPOINT ["ceps-client-mcp"]
