rm -rf stdlib
mkdir -p stdlib

curl -L "https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-20/wasi-sysroot-20.0.tar.gz" | tar xz
mv wasi-sysroot/ stdlib/wasi-sysroot
