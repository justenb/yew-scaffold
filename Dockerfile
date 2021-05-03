ARG BASE_IMAGE=ekidd/rust-musl-builder:latest
ARG serve_port=4000


FROM ${BASE_IMAGE} AS builder
ARG serve_port
ENV SERVE_PORT=$serve_port
RUN echo ${SERVE_PORT}

# add source to /home/rust
ADD --chown=rust:rust . ./

# chown /opt as we need to move wasm-pack into /opt
USER root
RUN chown -R rust:rust /opt
USER rust

RUN export RUSTC_VERSION=$(rustc --version|awk '{print $2}') \
    && export RUSTC_SYSROOT=$(rustc --print sysroot) \
    && echo $RUSTC_SYSROOT \
    # manually download wasm-pack  ref: https://rustwasm.github.io/wasm-pack/book/prerequisites/non-rustup-setups.html
    && curl -OL https://static.rust-lang.org/dist/rust-std-$RUSTC_VERSION-wasm32-unknown-unknown.tar.gz \
    && tar -xf rust-std-$RUSTC_VERSION-wasm32-unknown-unknown.tar.gz \
    && mv rust-std-$RUSTC_VERSION-wasm32-unknown-unknown/rust-std-wasm32-unknown-unknown/lib/rustlib/wasm32-unknown-unknown  \
    $RUSTC_SYSROOT/lib/rustlib

RUN rustup install nightly || true


# build the backend
RUN cargo build -p backend --release


########################################################################################
FROM alpine:latest

RUN mkdir /app
RUN apk --no-cache add ca-certificates

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/backend  /app/backend
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/build    /app/build

ENTRYPOINT [ "/app/backend" ]