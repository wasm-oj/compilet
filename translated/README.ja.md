# Compilet

**Rust**、**C**、**C++**を**WebAssembly**にコンパイルするサーバー。

## 使用方法

### Docker

#### ビルド

[Docker Hub (`jacoblincool/compilet`)](https://hub.docker.com/r/jacoblincool/compilet)には、`latest`タグがRust、C、C++をコンパイルすることができます。

> `rs`タグ(~500MB圧縮)を使用してRustのみをコンパイルすることもできます。また、`c`タグ(~150MB圧縮)を使用してCとC++のみをコンパイルすることもできます。

以下のコマンドで独自のイメージをビルドすることもできます。

```bash
docker build -t compilet .
```

#### 実行

以下のコマンドでイメージを実行できます。

```bash
docker run -p 8000:8000 jacoblincool/compilet
```

または、[docker composeファイル](./docker-compose.yml)を使用してイメージを実行できます。

```bash
docker compose up
```

> 環境変数を設定するために`.env`ファイルが必要な場合があります。詳細については、[`.env.example`ファイル](./.env.example)を確認してください。

上記のコマンドのいずれかを実行すると、サーバーがポート`8000`で実行されるため、`http://localhost:8000`でサーバーにアクセスできます。また、`PORT`環境変数を設定することでポートを変更することもできます。

### Cargo

Cargoを介してCompiletをインストールすることもできます。

```bash
cargo install compilet
```

CLIツールとして実行するのがより便利です。

```bash
compilet compile <file>
# compilet compile -h for more information
```

## エンドポイント

### 検証

Compiletは、[JWT](https://jwt.io/)を使用してリクエストを検証します。JWTトークンの秘密鍵を設定するには、`APP_SECRET`環境変数を設定します。デフォルトは`APP_SECRET`です。

JWTトークンは、`Bearer`スキームを使用して`Authorization`ヘッダーに渡す必要があります。

- [x] `GET /validate`エンドポイントは、JWTトークンが有効かどうかを検証します。ステータスコード`200`はトークンが有効であることを意味し、それ以外の場合はトークンが無効であることを意味します。

### コンパイル

Compiletは将来的にはコンパイルリクエストをキューイングできるようにする必要がありますが、現在はソースコードを直接コンパイルしています。

- [x] `POST /compile`エンドポイントは、ソースコードをWebAssemblyにコンパイルします。

POSTボディ:

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

- [ ] `POST /submission`エンドポイントは、ソースコードをWebAssemblyにコンパイルして、すぐに返信し、バックグラウンドでソースコードをコンパイルします。

POSTボディ:

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

- [ ] `GET /submission/{hash}`エンドポイントは、提出の状態と、コンパイルが完了した場合はコンパイルされたWebAssemblyバイナリを取得します。

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

- [x] `GET /system`エンドポイントは、システム情報を取得します(現在は`capabilities`のみ実装されています)。

レスポンス:

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

## 開発

リポジトリをクローンした後、次の操作を行う必要があります。

- `./scripts/stdlib.sh`を実行して、C、C++のWASI標準ライブラリをダウンロードします。
- `libclang_rt.builtins-wasm32.a`をClangが見つける場所にコピーします。(例：`/usr/lib/llvm16/lib/clang/16/lib/wasi`) (後で行うこともできます。エラーメッセージに場所が表示されます。)

以下のコマンドで開発モードでサーバーを実行できます。

```bash
cargo run
```

以下のコマンドでサーバーをビルドできます。

```bash
cargo build --release
```

---