# Build in one stage with Debian
FROM rust:slim

WORKDIR /app

COPY . .

RUN cargo build --release

RUN chmod +x /app/target/release/fibbot

ENTRYPOINT ["/app/target/release/fibbot"]