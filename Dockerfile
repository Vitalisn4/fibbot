# Use official Rust image for the build stage
FROM rust:latest as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies
RUN cargo fetch

# Copy the source code into the container
COPY . .

# Build the binary
RUN cargo build --release

# Final stage: Use a small image that includes the shell and necessary runtime libraries
FROM alpine:latest

# Install dependencies for running the app (glibc for Rust binaries, if necessary)
RUN apk add --no-cache \
    libc6-compat \
    gcc \
    libgcc \
    musl-dev \
    libc-dev

# Copy the binary from the build stage to the final image
COPY --from=builder /app/target/release/fibbot /fibbot

# Set the binary as executable
RUN chmod +x /fibbot

# Run the application by default
ENTRYPOINT ["/fibbot"]