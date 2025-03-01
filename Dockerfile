# Stage 1: Build the Rust application
FROM rust:latest as builder

WORKDIR /app

# Copy the Cargo.toml and source code
COPY Cargo.toml .
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create a minimal distroless image
FROM scratch

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/fibbot /fibbot

# Set the entrypoint to run the binary
ENTRYPOINT ["/fibbot"]

