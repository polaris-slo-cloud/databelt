# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM rust:latest AS buildbase
RUN rustup target add wasm32-wasip1
WORKDIR /src

FROM --platform=$BUILDPLATFORM buildbase AS buildbundled
COPY skylark_lib ./skylark_lib
COPY . /src
RUN --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/registry/index \
    RUSTFLAGS="--cfg wasmedge --cfg tokio_unstable" cargo build --target wasm32-wasip1 --release

FROM scratch AS bundled
ENTRYPOINT [ "ex_bundled.wasm" ]
COPY --link --from=buildbundled /src/target/wasm32-wasip1/release/ex_bundled.wasm  ex_bundled.wasm
