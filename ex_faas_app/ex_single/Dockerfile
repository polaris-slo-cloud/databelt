# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM rust:latest AS buildbase
RUN rustup target add wasm32-wasip1
WORKDIR /src

FROM --platform=$BUILDPLATFORM buildbase AS buildsingle
COPY skylark_lib ./skylark_lib
COPY . /src
RUN --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/registry/index \
    RUSTFLAGS="--cfg wasmedge --cfg tokio_unstable" cargo build --target wasm32-wasip1 --release

FROM scratch AS single
ENTRYPOINT [ "ex_single.wasm" ]
COPY --link --from=buildsingle /src/target/wasm32-wasip1/release/ex_single.wasm  ex_single.wasm
