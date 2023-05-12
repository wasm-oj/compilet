FROM rust:alpine as builder

RUN mkdir /tmp/tempproj && \
    cd /tmp/tempproj && \
    cargo init && \
    cargo add serde && \
    rm -rf /tmp/tempproj

RUN echo "http://dl-cdn.alpinelinux.org/alpine/edge/main" >> /etc/apk/repositories && \
    apk update && \
    apk add --no-cache musl-dev

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine as runtime

WORKDIR /app

# Install Rust toolchain
RUN echo "http://dl-cdn.alpinelinux.org/alpine/edge/main" >> /etc/apk/repositories && \
    apk update && \
    apk add --no-cache curl clang16 lld && \
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
