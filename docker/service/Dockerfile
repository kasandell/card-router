# syntax=docker/dockerfile:1
# Builder stage
FROM rust:1.76.0 AS builder

WORKDIR /app
RUN apt update && apt install lld clang -y
RUN mkdir -p -m 0700 ~/.ssh && ssh-keyscan github.com >> ~/.ssh/known_hosts
COPY ../../src src
COPY ../../Cargo.toml Cargo.toml
COPY ../../Cargo.lock Cargo.lock
COPY ../../.cargo/config.toml .cargo/config.toml
COPY ../../migrations migrations
COPY ../../api.lithic.com.pub.pem .
RUN --mount=type=ssh \
    cargo build --release --features logs-to-stdout

# Runtime stage
# FROM rust:1.76.0 AS runtime
#FROM debian:bullseye-slim AS runtime
FROM ubuntu:22.04 AS runtime
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates libpq5 \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/card_router card_router
COPY ../../.env .env
COPY ../../migrations migrations
# We need the configuration file at runtime!
EXPOSE 8080
ENTRYPOINT ["./card_router",  "--features", "logs-to-stdout"]