# Compilet

一個可以將 **Rust**、**C** 和 **C++** 編譯成 **WebAssembly** 的伺服器。

## 用法

### Docker

#### 建置

我在 [Docker Hub (`jacoblincool/compilet`)](https://hub.docker.com/r/jacoblincool/compilet) 上提供了一個 Docker 映像檔，支援直接編譯 Rust、C 和 C++。

你也可以使用以下命令建立自己的映像檔：

```bash
docker build -t compilet .
```

#### 執行

你可以使用以下命令執行映像檔：

```bash
docker run -p 8000:8000 -e ROCKET_ADDRESS=0.0.0.0 jacoblincool/compilet
```

或者使用 [docker compose 檔案](./docker-compose.yml) 執行映像檔：

```bash
docker compose up
```

> 你可能需要一個 `.env` 檔案來設置環境變數。請查看 [`.env.example` 檔案](./.env.example) 以獲取更多資訊。

以上兩個命令都會在端口 `8000` 上運行伺服器，因此你可以在 `http://localhost:8000` 上訪問伺服器。你也可以通過設置 `ROCKET_PORT` 環境變數來更改端口。

## 端點

### 驗證

Compilet 使用 [JWT](https://jwt.io/) 來驗證請求。你可以設置 `JWT_SECRET` 環境變數來設置 JWT 令牌的密鑰，默認為 `SECRET_TOKEN`。

你應該在 `Authorization` 標頭中使用 `Bearer` 方案傳遞 JWT 令牌。

- [x] `GET /validate` 端點用於驗證 JWT 令牌是否有效。狀態碼 `200` 表示令牌有效，否則令牌無效。

### 編譯

Compilet 應該能夠在未來將編譯請求排隊。但目前，它只是直接編譯源代碼。

- [x] `POST /compile` 端點用於將源代碼編譯成 WebAssembly。

POST 請求體：

```json
{
    "lang": "rs",
    "code": "fn main() { println!(\"Hello, world!\"); }"
}
```

回應：

```json
{
    "success": true,
    "message": "Compiled successfully",
    "hash": "bb343b0950832ccd077f1515e842196f2ae4bb9e9261b0935ac57916c3cf305d",
    "wasm": "<base64 encoded wasm binary>"
}
```

- [ ] `POST /submission` 端點用於將源代碼編譯成 WebAssembly，但立即返回，並在後台編譯源代碼。

POST 請求體：

```json
{
    "lang": "rs",
    "code": "fn main() { println!(\"Hello, world!\"); }"
}
```

回應：

```json
{
    "message": "Submitted",
    "hash": "bb343b0950832ccd077f1515e842196f2ae4bb9e9261b0935ac57916c3cf305d"
}
```

- [ ] `GET /submission/{hash}` 端點用於獲取提交的狀態，以及如果編譯完成則編譯的 WebAssembly 二進制碼。

回應：

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

### 系統

- [ ] `GET /system` 端點用於獲取系統資訊。

回應：

```json
{
    "capabilities": {
        "rs": "rust 1.71.0 + rand 0.8.5, release build",
        "c": "clang 16.0.3, level 3 optimizations",
        "cpp": "clang++ 16.0.3, level 3 optimizations"
    },
    "status": {
        "compiling": 0,
        "pending": 0
    }
}
```

## 開發

在克隆存儲庫後，你需要：

- 執行 `./scripts/stdlib.sh` 下載 C 和 C++ 的 WASI 標準庫。
- 將 `libclang_rt.builtins-wasm32.a` 複製到 Clang 可以找到它的地方。 (例如 `/usr/lib/llvm16/lib/clang/16/lib/wasi`) (你可以稍後進行此操作，錯誤消息會告訴你應該將它放在哪裡。)

你可以使用以下命令在開發模式下運行伺服器：

```bash
cargo run
```

使用以下命令構建伺服器：

```bash
cargo build --release
```
