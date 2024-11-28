# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM rust:1.70 AS buildbase
RUN rustup target add wasm32-wasip1
WORKDIR /src

FROM --platform=$BUILDPLATFORM buildbase AS buildrsclient
COPY . /src
RUN --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/registry/index \
    RUSTFLAGS="--cfg wasmedge --cfg tokio_unstable" cargo build --target wasm32-wasip1 --release

FROM scratch AS rsclient
ENTRYPOINT [ "rsclient.wasm" ]
COPY --link --from=buildrsclient /src/target/wasm32-wasip1/release/rsclient.wasm  rsclient.wasm
