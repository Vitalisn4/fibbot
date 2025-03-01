# First Stage: Build the Rust Application
FROM rust:latest as builder

WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# Copy the rest of the application code
COPY . .

# Build the release version of the application
RUN cargo build --release

# Second Stage: Copy the built executable to a minimal image
FROM scratch

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/fibbot /fibbot

# Set the entrypoint to the fibbot executable
ENTRYPOINT ["/fibbot"]
