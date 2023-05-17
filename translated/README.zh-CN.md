# Compilet

一个将 **Rust**、**C** 和 **C++** 编译成 **WebAssembly** 的服务器。

## 用法

### Docker

#### 构建

我们在 [Docker Hub (`jacoblincool/compilet`)](https://hub.docker.com/r/jacoblincool/compilet) 上提供了一个 Docker 镜像，`latest` 标签支持开箱即用地编译 Rust、C 和 C++。

> 您也可以使用 `rs` 标签（约 500MB 压缩）仅编译 Rust，或使用 `c` 标签（约 150MB 压缩）仅编译 C 和 C++。

此外，您可以使用以下命令构建自己的镜像：

```bash
docker build -t compilet .
```

#### 运行

您可以使用以下命令运行镜像：

```bash
docker run -p 8000:8000 jacoblincool/compilet
```

或使用 [docker compose 文件](./docker-compose.yml) 运行镜像：

```bash
docker compose up
```

> 您可能需要一个 `.env` 文件来设置环境变量。有关更多信息，请查看 [`.env.example` 文件](./.env.example)。

上述两个命令都将在端口 `8000` 上运行服务器，因此您可以通过设置 `PORT` 环境变量来访问服务器。

### Cargo

您也可以通过 Cargo 安装 Compilet：

```bash
cargo install compilet
```

将其作为 cli 工具运行更加方便：

```bash
compilet compile <file>
# compilet compile -h 以获取更多信息
```

## 端点

### 验证

Compilet 使用 [JWT](https://jwt.io/) 验证请求。您可以设置 `APP_SECRET` 环境变量来设置 JWT 令牌的密钥，默认为 `APP_SECRET`。

您应该在 `Authorization` 标头中使用 `Bearer` 方案传递 JWT 令牌。

- [x] `GET /validate` 端点用于验证 JWT 令牌是否有效。状态码 `200` 表示令牌有效，否则令牌无效。

### 编译

Compilet 应该能够在将来排队编译请求。但目前，它只是直接编译源代码。

- [x] `POST /compile` 端点用于将源代码编译为 WebAssembly。

POST 正文：

```json
{
    "lang": "rs",
    "code": "fn main() { println!(\"Hello, world!\"); }"
}
```

响应：

```json
{
    "success": true,
    "message": "编译成功",
    "hash": "bb343b0950832ccd077f1515e842196f2ae4bb9e9261b0935ac57916c3cf305d",
    "wasm": "<base64 编码的 wasm 二进制文件>"
}
```

- [ ] `POST /submission` 端点用于将源代码编译为 WebAssembly，但立即返回并在后台编译源代码。

POST 正文：

```json
{
    "lang": "rs",
    "code": "fn main() { println!(\"Hello, world!\"); }"
}
```

响应：

```json
{
    "message": "已提交",
    "hash": "bb343b0950832ccd077f1515e842196f2ae4bb9e9261b0935ac57916c3cf305d"
}
```

- [ ] `GET /submission/{hash}` 端点用于获取提交的状态，以及如果编译完成，则获取已编译的 WebAssembly 二进制文件。

响应：

```json
{
    "status": "pending",
    "message": "等待编译",
    "wasm": null
}
```

```json
{
    "status": "success",
    "message": "编译成功",
    "wasm": "<base64 编码的 wasm 二进制文件>"
}
```

```json
{
    "status": "failed",
    "message": "编译失败（错误消息）",
    "wasm": null
}
```

### 系统

- [x] `GET /system` 端点用于获取系统信息（目前仅实现了 `capabilities`）

响应：

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

## 开发

克隆仓库后，您需要：

- 运行 `./scripts/stdlib.sh` 下载 C 和 C++ 的 WASI 标准库。
- 将 `libclang_rt.builtins-wasm32.a` 复制到 Clang 可以找到它的某个位置。（例如 `/usr/lib/llvm16/lib/clang/16/lib/wasi`）（您可以稍后执行此操作，错误消息将告诉您要将其放在哪里。）

您可以使用以下命令在开发模式下运行服务器：

```bash
cargo run
```

使用以下命令构建服务器：

```bash
cargo build --release
```

---