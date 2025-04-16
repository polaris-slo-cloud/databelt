# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM rust:latest AS buildbase
RUN rustup target add wasm32-wasip1
WORKDIR /src

FROM --platform=$BUILDPLATFORM buildbase AS buildexalarm
COPY skylark_manage ./skylark_manage
COPY . /src
RUN --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/registry/index \
    RUSTFLAGS="--cfg wasmedge --cfg tokio_unstable" cargo build --target wasm32-wasip1 --release

FROM scratch AS exalarm
ENV TZ="Austria/Vienna"
ENTRYPOINT [ "ex_alarm.wasm" ]
COPY --link --from=buildexalarm /src/target/wasm32-wasip1/release/ex_alarm.wasm  ex_alarm.wasm
