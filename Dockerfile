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
# Production stage
# ---------------------------
FROM base AS prod 
RUN cargo build --release

FROM alpine:3.22.1 AS runtime
WORKDIR /app
COPY --from=prod /app/target/release/luckywin .

CMD ["./luckywin"]
