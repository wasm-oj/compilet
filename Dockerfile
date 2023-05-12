FROM rust:alpine as builder

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

RUN echo "http://dl-cdn.alpinelinux.org/alpine/edge/main" >> /etc/apk/repositories && \
    apk update && \
    apk add --no-cache musl-dev git

RUN mkdir /tmp/tempproj && \
    cd /tmp/tempproj && \
    cargo init && \
    cargo add serde && \
    rm -rf /tmp/tempproj

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine as runtime

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

WORKDIR /app

# Install Rust toolchain
RUN echo "http://dl-cdn.alpinelinux.org/alpine/edge/main" >> /etc/apk/repositories && \
    apk update && \
    apk add --no-cache curl clang16 lld git && \
    curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal && \
    source $HOME/.cargo/env && \
    rustup target add wasm32-wasi

ENV PATH="/root/.cargo/bin:${PATH}"

# Update crates.io index
RUN mkdir /tmp/tempproj && \
    cd /tmp/tempproj && \
    cargo init && \
    cargo add serde && \
    cd / && \
    rm -rf /tmp/tempproj

# Copy WASI libs
COPY --from=ghcr.io/webassembly/wasi-sdk /wasi-sysroot /app/stdlib/wasi-sysroot
COPY --from=ghcr.io/webassembly/wasi-sdk /usr/lib/llvm-16/lib/clang/16/lib/wasi /usr/lib/llvm16/lib/clang/16/lib/wasi

# Copy the binary from the build stage
COPY --from=builder /app/target/release/compilet /app/compilet

ENTRYPOINT ["/app/compilet"]
