FROM rust:alpine3.22 AS base

# Install build dependencies needed for ring crate and other native dependencies
RUN apk add --no-cache \
    build-base \
    musl-dev \
    gcc \
    libc-dev \
    pkgconfig

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
# ---------------------------
# Development stage
# ---------------------------



FROM base AS dev
RUN cargo install cargo-watch
CMD ["cargo", "watch", "-x", "run"]

# ---------------------------
# Production stage (build)
# ---------------------------
FROM base AS builder
RUN cargo build --release

# ---------------------------
# Production runtime stage
# ---------------------------
FROM alpine:3.22.1 AS prod
WORKDIR /app
COPY --from=builder /app/target/release/luckywin .

CMD ["./luckywin"]
