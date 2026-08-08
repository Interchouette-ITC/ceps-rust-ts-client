# Runtime for `ceps-client-cli`. Build the binary outside the image
# (path dep on rustSDK), place it as `docker/ceps-client-cli`, then:
#   make docker-build IMAGE_TAG=local
ARG RUNTIME_IMAGE=debian:bookworm-slim

FROM ${RUNTIME_IMAGE}

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

COPY ceps-client-cli /usr/local/bin/ceps-client-cli
RUN chmod +x /usr/local/bin/ceps-client-cli

ENTRYPOINT ["ceps-client-cli"]
