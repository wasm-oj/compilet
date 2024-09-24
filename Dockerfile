FROM rust:alpine AS builder

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ARG COMPILET_FEATURES="c,cpp,rs"

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

RUN cargo build --release --no-default-features --features $COMPILET_FEATURES

FROM alpine AS runtime-rs

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ARG COMPILET_FEATURES="rs"

WORKDIR /app

# Install Rust toolchain
RUN echo "http://dl-cdn.alpinelinux.org/alpine/edge/main" >> /etc/apk/repositories && \
    apk update && \
    apk add --no-cache curl lld git && \
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

# Copy the binary from the build stage
COPY --from=builder /app/target/release/compilet /app/compilet

ENTRYPOINT ["/app/compilet"]

FROM alpine AS runtime-c

ARG COMPILET_FEATURES="c,cpp"

WORKDIR /app

# Install Rust toolchain
RUN echo "http://dl-cdn.alpinelinux.org/alpine/edge/main" >> /etc/apk/repositories && \
    apk update && \
    apk add --no-cache clang18 lld

# Copy WASI libs
COPY --from=ghcr.io/webassembly/wasi-sdk /opt/wasi-sdk/share/wasi-sysroot /app/stdlib/wasi-sysroot
COPY --from=ghcr.io/webassembly/wasi-sdk /opt/wasi-sdk/lib/clang/18/lib/wasi /usr/lib/llvm18/lib/clang/18/lib/wasi

# Copy the binary from the build stage
COPY --from=builder /app/target/release/compilet /app/compilet

ENTRYPOINT ["/app/compilet"]

FROM alpine AS runtime

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ARG COMPILET_FEATURES="c,cpp,rs"

WORKDIR /app

# Install Rust toolchain
RUN echo "http://dl-cdn.alpinelinux.org/alpine/edge/main" >> /etc/apk/repositories && \
    apk update && \
    apk add --no-cache curl clang18 lld git && \
    curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable && \
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
COPY --from=ghcr.io/webassembly/wasi-sdk /opt/wasi-sdk/share/wasi-sysroot /app/stdlib/wasi-sysroot
COPY --from=ghcr.io/webassembly/wasi-sdk /opt/wasi-sdk/lib/clang/18/lib/wasi /usr/lib/llvm18/lib/clang/18/lib/wasi

# Copy the binary from the build stage
COPY --from=builder /app/target/release/compilet /app/compilet

ENTRYPOINT ["/app/compilet"]
