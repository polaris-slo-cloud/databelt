# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM rust:latest  AS builder

RUN rustup target add wasm32-wasip1

WORKDIR /build

COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir src && touch src/main.rs && cargo fetch

COPY . .
RUN cargo build --target wasm32-wasip1 --release

FROM scratch

COPY --from=builder /etc/services /etc/services
COPY --from=builder /build/target/wasm32-wasip1/release/skylark_api.wasm skylark_api.wasm 

EXPOSE 8080

ENTRYPOINT [ "skylark_api.wasm" ]
