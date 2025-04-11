FROM rust:1.73-slim as builder

WORKDIR /usr/src/app

# Install dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Create blank project
RUN cargo new --bin url-shortener

# Copy over manifests
COPY ./Cargo.toml ./Cargo.lock ./url-shortener/

# Build dependencies - this is the caching Docker layer
WORKDIR /usr/src/app/url-shortener
RUN cargo build --release && \
    rm src/*.rs

# Copy actual source code
COPY ./src ./src

# Build for release
RUN touch src/main.rs && \
    cargo build --release

# Final stage
FROM debian:bullseye-slim

# Install required dependencies for runtime
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/app/url-shortener/target/release/url-shortener /usr/local/bin/

# Create directory for any additional files
WORKDIR /app

# Set the startup command
CMD ["url-shortener"]