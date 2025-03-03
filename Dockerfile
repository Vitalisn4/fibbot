FROM rust:slim

WORKDIR /app

# Install OpenSSL and pkg-config
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

RUN chmod +x /app/target/release/fibbot

ENTRYPOINT ["/app/target/release/fibbot"]