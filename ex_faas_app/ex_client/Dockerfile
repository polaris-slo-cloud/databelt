# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM rust:latest AS buildbase
RUN rustup target add wasm32-wasip1
WORKDIR /build

FROM --platform=$BUILDPLATFORM buildbase AS buildclient
COPY . /build
RUN --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/registry/index \
    RUSTFLAGS="--cfg wasmedge --cfg tokio_unstable" cargo build --target wasm32-wasip1 --release

FROM scratch AS client
ENTRYPOINT [ "ex_client.wasm" ]
COPY --link --from=buildclient /build/target/wasm32-wasip1/release/ex_client.wasm  ex_client.wasm
