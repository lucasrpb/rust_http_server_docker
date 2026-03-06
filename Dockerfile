# -------- Build stage --------
FROM rust:1.79 AS builder

RUN apt-get update && apt-get install -y musl-tools

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

# -------- Runtime stage --------
FROM alpine:3.19

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-http-demo .

EXPOSE 8080

CMD ["./rust-http-demo"]