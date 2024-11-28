# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM rust:1.70 AS buildbase
RUN rustup target add wasm32-wasip1
WORKDIR /src

FROM --platform=$BUILDPLATFORM buildbase AS buildpreprocess
COPY skylark ./skylark
COPY . /src
RUN --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/registry/index \
    RUSTFLAGS="--cfg wasmedge --cfg tokio_unstable" cargo build --target wasm32-wasip1 --release

FROM scratch AS preprocess
ENTRYPOINT [ "ex_preprocess.wasm" ]
COPY --link --from=buildpreprocess /src/target/wasm32-wasip1/release/ex_preprocess.wasm  ex_preprocess.wasm
