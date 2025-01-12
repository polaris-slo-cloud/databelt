FROM --platform=$BUILDPLATFORM rust:latest AS builder
RUN apt-get update && \
    apt-get install -y clang && \
    apt-get clean
RUN rustup target add wasm32-wasip1
ENV RUSTFLAGS="--cfg wasmedge --cfg tokio_unstable"
RUN mkdir /build
COPY . /build/
RUN ls -lah /build
WORKDIR /build
RUN cargo build --target wasm32-wasip1 --release
RUN ls -lah
RUN chmod +x target/wasm32-wasip1/release/skylark_elect.wasm

FROM scratch
COPY --from=builder /build/target/wasm32-wasip1/release/skylark_elect.wasm /
COPY src/benchmark .
ENTRYPOINT ["/skylark_elect.wasm"]