<!-- ai18n [ignore] -->
[正體中文](translated/README.zh-TW.md) | [简体中文](translated/README.zh-CN.md) | [日本語](translated/README.ja.md) | [한국어](translated/README.ko.md) | [Français](translated/README.fr.md) | [Deutsch](translated/README.de.md) | [Español](translated/README.es.md)
<!-- /ai18n [ignore] -->

# Compilet

Server that compiles **Rust**, **C**, and **C++** into **WebAssembly**.

## Usage

### Docker

#### Build

I have a docker image available on [Docker Hub (`jacoblincool/compilet`)](https://hub.docker.com/r/jacoblincool/compilet), which supports to compile Rust, C, and C++ out of the box.

Also, you can build your own image with the following command:

```bash
docker build -t compilet .
```

#### Run

You can run the image with the following command:

```bash
docker run -p 8000:8000 jacoblincool/compilet
```

Or use [the docker compose file](./docker-compose.yml) to run the image:

```bash
docker compose up
```

> You may need a `.env` file to set the environment variables. Check [the `.env.example` file](./.env.example) for more information.

Both of the commands above will run the server on port `8000`, so you can access the server at `http://localhost:8000`. You can also change the port by setting the `ROCKET_PORT` environment variable.

## Endpoints

### Validation

Compilet uses [JWT](https://jwt.io/) to validate the request. You can set the `JWT_SECRET` environment variable to set the secret key for the JWT token, default is `SECRET_TOKEN`.

You should pass the JWT token in the `Authorization` header with the `Bearer` scheme.

- [x] `GET /validate` endpoint to validate if the JWT token is valid. Status code `200` means the token is valid, otherwise the token is invalid.

### Compile

Compilet should be able to queue the compile request in the future. But currently, it just compiles the source code directly.

- [x] `POST /compile` endpoint to compile the source code into WebAssembly

POST body:

```json
{
    "lang": "rs",
    "code": "fn main() { println!(\"Hello, world!\"); }"
}
```

Response:

```json
{
    "success": true,
    "message": "Compiled successfully",
    "hash": "bb343b0950832ccd077f1515e842196f2ae4bb9e9261b0935ac57916c3cf305d",
    "wasm": "<base64 encoded wasm binary>"
}
```

- [ ] `POST /submission` endpoint to compile the source code into WebAssembly, but return immediately and compile the source code in the background.

POST body:

```json
{
    "lang": "rs",
    "code": "fn main() { println!(\"Hello, world!\"); }"
}
```

Response:

```json
{
    "message": "Submitted",
    "hash": "bb343b0950832ccd077f1515e842196f2ae4bb9e9261b0935ac57916c3cf305d"
}
```

- [ ] `GET /submission/{hash}` endpoint to get the status of the submission, and the compiled WebAssembly binary if the compilation is finished.

Response:

```json
{
    "status": "pending",
    "message": "Waiting for compilation",
    "wasm": null
}
```

```json
{
    "status": "success",
    "message": "Compiled successfully",
    "wasm": "<base64 encoded wasm binary>"
}
```

```json
{
    "status": "failed",
    "message": "Compilation failed (error message)",
    "wasm": null
}
```

### System

- [x] `GET /system` endpoint to get the system information (currently only the `capabilities` is implemented)

Response:

```json
{
    "capabilities": {
        "rs": "rust 2021 edition + rand 0.8.5, release build",
        "c": "clang 16, level 3 optimizations",
        "cpp": "clang++ 16, level 3 optimizations"
    },
    "status": {
        "compiling": 0,
        "pending": 0
    }
}
```

## Development

After cloning the repository, you need to:

- Run `./scripts/stdlib.sh` to download the WASI standard library for C, and C++.
- Copy `libclang_rt.builtins-wasm32.a` to somewhere that Clang can find it. (e.g. `/usr/lib/llvm16/lib/clang/16/lib/wasi`) (You can do it later, the error message will tell you where to put it.)

You can run the server in development mode with the following command:

```bash
cargo run
```

Build the server with the following command:

```bash
cargo build --release
```
