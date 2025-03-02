# Build stage
FROM rust:1.66 as builder

# Set working directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Build dependencies (for better caching)
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Final stage
FROM gcr.io/distroless/cc-debian11

# Copy the built binary
COPY --from=builder /app/target/release/fibbot /fibbot

# Set the entrypoint
ENTRYPOINT ["/fibbot"]