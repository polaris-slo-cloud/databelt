# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM rust:latest AS buildbase
RUN rustup target add wasm32-wasip1
WORKDIR /src

FROM --platform=$BUILDPLATFORM buildbase AS buildobjdetector
COPY skylark_manage ./skylark_manage
COPY . /src
RUN --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/registry/index \
    RUSTFLAGS="--cfg wasmedge --cfg tokio_unstable" cargo build --target wasm32-wasip1 --release

FROM scratch AS objdetector
ENTRYPOINT [ "ex_detect.wasm" ]
COPY --link --from=buildobjdetector /src/target/wasm32-wasip1/release/ex_detect.wasm  ex_detect.wasm
