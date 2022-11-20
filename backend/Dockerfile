FROM rust:1.61 as builder
RUN apt-get update && apt install -y cmake

WORKDIR /usr/src/btcpay-rust
COPY btcpay-rust/src ./src
COPY btcpay-rust/Cargo.toml .
RUN cargo build

WORKDIR /usr/src/back-end
COPY backend/src ./src
COPY backend/Cargo.toml .
COPY backend/diesel.toml .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/btc-chat-backend /usr/local/bin/btc-chat-backend

ENTRYPOINT [ "/usr/local/bin/btc-chat-backend" ]
CMD [ "--help" ]
