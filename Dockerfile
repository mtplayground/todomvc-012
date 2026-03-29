# Stage 1: Build the application
FROM rust:1-slim-bookworm AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install nightly Rust with wasm target
RUN rustup toolchain install nightly --target wasm32-unknown-unknown

# Install cargo-leptos
RUN cargo +nightly install cargo-leptos --locked

WORKDIR /app

# Copy manifests first for layer caching
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./

# Copy source files
COPY src ./src
COPY style ./style
COPY migrations ./migrations

# Build the application in release mode
RUN cargo leptos build --release

# Stage 2: Runtime image
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the server binary
COPY --from=builder /app/target/release/todomvc ./todomvc

# Copy the site assets (WASM, JS, CSS)
COPY --from=builder /app/target/site ./target/site

# Copy migrations for runtime
COPY --from=builder /app/migrations ./migrations

# Expose the application port
EXPOSE 8080

# Set default environment variables
ENV DATABASE_URL=sqlite:./todos.db
ENV LEPTOS_OUTPUT_NAME=todomvc
ENV LEPTOS_SITE_ROOT=target/site
ENV LEPTOS_SITE_PKG_DIR=pkg
ENV LEPTOS_SITE_ADDR=0.0.0.0:8080

CMD ["./todomvc"]
