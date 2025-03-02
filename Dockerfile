# Build stage: Use Rust Alpine image
FROM rust:alpine as builder

# Install build dependencies
RUN apk add --no-cache musl-dev gcc

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create empty source files to cache dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo fetch && \
    cargo build --release && \
    rm -rf src

# Copy the source code into the container
COPY . .

# Build the binary with musl target
RUN cargo build --release

# Final stage: Use Alpine
FROM alpine:latest

# Add minimal runtime dependencies if needed
RUN apk add --no-cache ca-certificates

# Copy the binary from the build stage to the final image
COPY --from=builder /app/target/release/fibbot /fibbot

# Set the binary as executable
RUN chmod +x /fibbot

# Run the application by default
ENTRYPOINT ["/fibbot"]