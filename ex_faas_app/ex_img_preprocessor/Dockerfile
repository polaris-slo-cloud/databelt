# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM rust:1.70 AS buildbase
RUN rustup target add wasm32-wasi
WORKDIR /src

FROM --platform=$BUILDPLATFORM buildbase AS buildexfirealarm
COPY . /src
RUN --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/registry/index \
    RUSTFLAGS="--cfg wasmedge --cfg tokio_unstable" cargo build --target wasm32-wasi --release

FROM scratch AS exfirealarm
ENTRYPOINT [ "ex_img_preprocessor.wasm" ]
COPY --link --from=buildexfirealarm /src/target/wasm32-wasi/release/ex_img_preprocessor.wasm  ex_img_preprocessor.wasm
