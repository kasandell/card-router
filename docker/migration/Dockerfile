# syntax=docker/dockerfile:1
#FROM rust:1.76.0 AS builder
#WORKDIR /app
#RUN cargo install diesel_cli --no-default-features --features postgres
#COPY ../../migrations migrations
#ENTRYPOINT diesel migration run --database-url postgres://$DATABASE_USERNAME:$DATABASE_PASSWORD@$DATABASE_HOSTNAME:$DATABASE_PORT/$DATABASE_NAME

FROM rust:1.76.0 AS builder

WORKDIR /app
RUN cargo install diesel_cli --root .cargo/ --no-default-features --features postgres
RUN mkdir -p /out && cp .cargo/bin/diesel /out/

FROM ubuntu:22.04 AS runtime
COPY --from=builder /out/diesel /bin/

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends ca-certificates libpq5 openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY ../../migrations migrations
ENTRYPOINT diesel migration run --database-url postgres://$DATABASE_USERNAME:$DATABASE_PASSWORD@$DATABASE_HOSTNAME:$DATABASE_PORT/$DATABASE_NAME
