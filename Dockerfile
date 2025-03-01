# Step 1: Build stage using official Rust image
FROM rust:latest as builder

# Set the working directory
WORKDIR /app

# Install necessary build dependencies
RUN apt-get update && apt-get install -y \
    musl-tools \
    pkg-config \
    libssl-dev \
    gcc \
    make \
    libclang-dev \
    curl \
    bash \
    file \
    git

# Set up the musl target for cross-compiling
RUN rustup target add x86_64-unknown-linux-musl

# Copy only the Cargo.toml and Cargo.lock files (this allows caching dependencies)
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies (this will be cached unless Cargo.toml or Cargo.lock change)
RUN cargo fetch

# Copy the source code into the container
COPY . .

# Build the binary using musl target (for Alpine compatibility)
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Step 2: Final runtime stage with Alpine
FROM alpine:latest

# Install necessary dependencies for musl libc (used by Alpine)
RUN apk add --no-cache \
    musl-dev \
    gcc \
    libgcc \
    libc-dev

# Copy the compiled binary from the build stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/fibbot /fibbot

# Set the binary as executable
RUN chmod +x /fibbot

# Run the application by default
ENTRYPOINT ["/fibbot"]
