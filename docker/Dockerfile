# Thin runtime for the `ceps` CLI. Build the binary outside the image
# (path dep on rustSDK), place it as `docker/ceps`, then:
#   make docker-build IMAGE_TAG=local
ARG RUNTIME_IMAGE=debian:bookworm-slim

FROM ${RUNTIME_IMAGE}

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

COPY ceps /usr/local/bin/ceps
RUN chmod +x /usr/local/bin/ceps

ENTRYPOINT ["ceps"]
