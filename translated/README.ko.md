# Compilet

**Rust**, **C**, 그리고 **C++**를 **WebAssembly**로 컴파일하는 서버입니다.

## 사용법

### Docker

#### 빌드

[Docker Hub (`jacoblincool/compilet`)](https://hub.docker.com/r/jacoblincool/compilet)에서 사용 가능한 도커 이미지가 있습니다. `latest` 태그는 Rust, C, 그리고 C++를 기본적으로 컴파일할 수 있습니다.

> `rs` 태그(~500MB 압축)는 Rust만 컴파일하고, `c` 태그(~150MB 압축)는 C와 C++만 컴파일할 수 있습니다.

또한, 다음 명령어로 직접 이미지를 빌드할 수 있습니다:

```bash
docker build -t compilet .
```

#### 실행

다음 명령어로 이미지를 실행할 수 있습니다:

```bash
docker run -p 8000:8000 jacoblincool/compilet
```

또는 [docker compose 파일](./docker-compose.yml)을 사용하여 이미지를 실행할 수 있습니다:

```bash
docker compose up
```

> 환경 변수를 설정하기 위해 `.env` 파일이 필요할 수 있습니다. 자세한 정보는 [.env.example 파일](./.env.example)을 확인하세요.

위의 두 명령어는 모두 서버를 `8000` 포트에서 실행합니다. 따라서 서버에는 `http://localhost:8000`로 접근할 수 있습니다. 또한, `PORT` 환경 변수를 설정하여 포트를 변경할 수 있습니다.

### Cargo

Cargo를 통해 Compilet을 설치할 수도 있습니다:

```bash
cargo install compilet
```

다음 명령어로 더 편리하게 실행할 수 있습니다:

```bash
compilet compile <file>
# compilet compile -h를 입력하여 더 많은 정보를 확인하세요.
```

## 엔드포인트

### Validation

Compilet은 요청을 검증하기 위해 [JWT](https://jwt.io/)를 사용합니다. `APP_SECRET` 환경 변수를 설정하여 JWT 토큰의 비밀 키를 설정할 수 있으며, 기본값은 `APP_SECRET`입니다.

JWT 토큰은 `Authorization` 헤더에 `Bearer` 스키마와 함께 전달해야 합니다.

- [x] `GET /validate` 엔드포인트는 JWT 토큰이 유효한지 확인합니다. 상태 코드 `200`은 토큰이 유효함을 의미하며, 그렇지 않으면 토큰이 유효하지 않습니다.

### Compile

Compilet은 컴파일 요청을 나중에 큐에 넣을 수 있어야 합니다. 하지만 현재는 소스 코드를 직접 컴파일합니다.

- [x] `POST /compile` 엔드포인트는 소스 코드를 WebAssembly로 컴파일합니다.

POST 본문:

```json
{
    "lang": "rs",
    "code": "fn main() { println!(\"Hello, world!\"); }"
}
```

응답:

```json
{
    "success": true,
    "message": "Compiled successfully",
    "hash": "bb343b0950832ccd077f1515e842196f2ae4bb9e9261b0935ac57916c3cf305d",
    "wasm": "<base64 encoded wasm binary>"
}
```

- [ ] `POST /submission` 엔드포인트는 소스 코드를 WebAssembly로 컴파일하고 즉시 반환하며, 소스 코드를 백그라운드에서 컴파일합니다.

POST 본문:

```json
{
    "lang": "rs",
    "code": "fn main() { println!(\"Hello, world!\"); }"
}
```

응답:

```json
{
    "message": "Submitted",
    "hash": "bb343b0950832ccd077f1515e842196f2ae4bb9e9261b0935ac57916c3cf305d"
}
```

- [ ] `GET /submission/{hash}` 엔드포인트는 제출 상태와 컴파일이 완료되면 컴파일된 WebAssembly 이진 파일을 가져옵니다.

응답:

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

- [x] `GET /system` 엔드포인트는 시스템 정보(현재는 `capabilities`만 구현됨)를 가져옵니다.

응답:

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

## 개발

저장소를 복제한 후 다음을 수행해야 합니다:

- `./scripts/stdlib.sh`를 실행하여 C와 C++의 WASI 표준 라이브러리를 다운로드합니다.
- `libclang_rt.builtins-wasm32.a`를 Clang이 찾을 수 있는 위치에 복사합니다. (예: `/usr/lib/llvm16/lib/clang/16/lib/wasi`) (나중에 할 수도 있으며, 오류 메시지에서 위치를 알려줍니다.)

다음 명령어로 개발 모드에서 서버를 실행할 수 있습니다:

```bash
cargo run
```

다음 명령어로 서버를 빌드할 수 있습니다:

```bash
cargo build --release
```

---