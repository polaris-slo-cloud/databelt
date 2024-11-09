# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM rust:1.82-slim AS buildbase
RUN rustup target add wasm32-wasi
WORKDIR /src

FROM --platform=$BUILDPLATFORM buildbase AS buildfn2
COPY Cargo.toml ./
COPY src ./src
COPY .cargo ./.cargo
# Build the Wasm binary
RUN cargo build --target wasm32-wasi --release
# This line builds the AOT Wasm binary
RUN cp target/wasm32-wasi/release/ex_fn_2.wasm ex_fn_2.wasm
RUN chmod a+x ex_fn_2.wasm

FROM scratch AS ex_fn_2
ENTRYPOINT [ "ex_fn_2.wasm" ]
COPY --link --from=buildfn2 /src/target/wasm32-wasi/release/ex_fn_2.wasm  ex_fn_2.wasm
