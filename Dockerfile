FROM rust:1.79-alpine AS builder

WORKDIR /app

RUN apk add pkgconfig openssl-dev libc-dev

COPY Cargo.toml .
COPY Cargo.lock .
COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.14 AS runtime

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/yuki /app/yuki

USER yuki:yuki

CMD ["/app/yuki"]
