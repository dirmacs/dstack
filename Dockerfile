# syntax=docker/dockerfile:1.7

# Multi-stage build for the dstack HTTP API + MCP server (dstack-serve).
# Pure-Rust workspace; no native deps, no build-time database.

FROM rust:1.99-bookworm AS builder

WORKDIR /app

# Copy the whole workspace so path deps (dstack-memory, dstack-cli) resolve.
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Cache the cargo registry and target dir across builds.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked -p dstack-server && \
    cp /app/target/release/dstack-serve /tmp/dstack-serve

FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates curl && \
    rm -rf /var/lib/apt/lists/* && \
    useradd --create-home --uid 1000 --shell /usr/sbin/nologin dstack

WORKDIR /app

COPY --from=builder /tmp/dstack-serve /usr/local/bin/dstack-serve

RUN mkdir -p /app/data && chown -R dstack:dstack /app

USER dstack

ENV RUST_LOG=info

# dstack-serve defaults to 127.0.0.1:3500; bind all interfaces in a container.
EXPOSE 3500

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD curl -fsS http://127.0.0.1:3500/health || exit 1

CMD ["dstack-serve", "--bind", "0.0.0.0", "--port", "3500"]
