# Compilet

**WebAssembly** に **Rust**、**C**、**C++** をコンパイルするサーバー。

## 使用法

### Docker

#### ビルド

[Docker Hub (`jacoblincool/compilet`)](https://hub.docker.com/r/jacoblincool/compilet) に使用可能な Docker イメージがあり、Rust、C、C++ をすぐにコンパイルできます。

また、以下のコマンドで独自のイメージをビルドできます。

```bash
docker build -t compilet .
```

#### 実行

以下のコマンドでイメージを実行できます。

```bash
docker run -p 8000:8000 jacoblincool/compilet
```

または、[docker compose ファイル](./docker-compose.yml) を使用してイメージを実行できます。

```bash
docker compose up
```

> 環境変数を設定するために `.env` ファイルが必要な場合があります。詳細については、[`.env.example` ファイル](./.env.example) を確認してください。

上記のコマンドのいずれかを実行すると、サーバーがポート `8000` で実行されるため、`http://localhost:8000` でサーバーにアクセスできます。また、`PORT` 環境変数を設定することでポートを変更できます。

## エンドポイント

### 検証

Compilet は [JWT](https://jwt.io/) を使用してリクエストを検証します。JWT トークンの秘密鍵を設定するには、`JWT_SECRET` 環境変数を設定します。デフォルトは `SECRET_TOKEN` です。

JWT トークンは `Authorization` ヘッダーに `Bearer` スキームで渡す必要があります。

- [x] `GET /validate` エンドポイントは、JWT トークンが有効かどうかを検証します。ステータスコード `200` はトークンが有効であることを示し、それ以外の場合はトークンが無効であることを示します。

### コンパイル

Compilet は将来的にコンパイルリクエストをキューイングできるようにする必要があります。しかし、現在はソースコードを直接コンパイルしています。

- [x] `POST /compile` エンドポイントは、ソースコードを WebAssembly にコンパイルします。

POST ボディ:

```json
{
    "lang": "rs",
    "code": "fn main() { println!(\"Hello, world!\"); }"
}
```

レスポンス:

```json
{
    "success": true,
    "message": "Compiled successfully",
    "hash": "bb343b0950832ccd077f1515e842196f2ae4bb9e9261b0935ac57916c3cf305d",
    "wasm": "<base64 encoded wasm binary>"
}
```

- [ ] `POST /submission` エンドポイントは、ソースコードを WebAssembly にコンパイルし、すぐに返信し、バックグラウンドでソースコードをコンパイルします。

POST ボディ:

```json
{
    "lang": "rs",
    "code": "fn main() { println!(\"Hello, world!\"); }"
}
```

レスポンス:

```json
{
    "message": "Submitted",
    "hash": "bb343b0950832ccd077f1515e842196f2ae4bb9e9261b0935ac57916c3cf305d"
}
```

- [ ] `GET /submission/{hash}` エンドポイントは、提出の状態と、コンパイルが完了した場合はコンパイルされた WebAssembly バイナリを取得します。

レスポンス:

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

### システム

- [ ] `GET /system` エンドポイントは、システム情報を取得します。

レスポンス:

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

## 開発

リポジトリをクローンした後、次の操作を行う必要があります。

- `./scripts/stdlib.sh` を実行して、C、C++ の WASI 標準ライブラリをダウンロードします。
- `libclang_rt.builtins-wasm32.a` を Clang が見つけられる場所にコピーします。 (例: `/usr/lib/llvm16/lib/clang/16/lib/wasi`) (後で行うこともできます。エラーメッセージに場所が表示されます。)

以下のコマンドで開発モードでサーバーを実行できます。

```bash
cargo run
```

以下のコマンドでサーバーをビルドできます。

```bash
cargo build --release
```
