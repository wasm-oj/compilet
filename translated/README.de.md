# Compilet

Ein Server, der **Rust**, **C** und **C++** in **WebAssembly** kompiliert.

## Verwendung

### Docker

#### Build

Wir haben ein Docker-Image auf [Docker Hub (`jacoblincool/compilet`)](https://hub.docker.com/r/jacoblincool/compilet) verfügbar, das Tag `latest` unterstützt das Kompilieren von Rust, C und C++ out of the box.

> Sie können auch das `rs`-Tag (~500MB komprimiert) verwenden, um nur Rust zu kompilieren, oder das `c`-Tag (~150MB komprimiert), um nur C und C++ zu kompilieren.

Sie können auch Ihr eigenes Image mit dem folgenden Befehl erstellen:

```bash
docker build -t compilet .
```

#### Ausführen

Sie können das Image mit dem folgenden Befehl ausführen:

```bash
docker run -p 8000:8000 jacoblincool/compilet
```

Oder verwenden Sie [die Docker-Compose-Datei](./docker-compose.yml), um das Image auszuführen:

```bash
docker compose up
```

> Sie benötigen möglicherweise eine `.env`-Datei, um die Umgebungsvariablen festzulegen. Weitere Informationen finden Sie in der Datei `.env.example`.

Beide oben genannten Befehle führen den Server auf Port `8000` aus, sodass Sie auf den Server unter `http://localhost:8000` zugreifen können. Sie können auch den Port ändern, indem Sie die Umgebungsvariable `PORT` festlegen.

### Cargo

Sie können Compilet auch über Cargo installieren:

```bash
cargo install compilet
```

Es ist bequemer, es als CLI-Tool auszuführen:

```bash
compilet compile <file>
# compilet compile -h für weitere Informationen
```

## Endpunkte

### Validierung

Compilet verwendet [JWT](https://jwt.io/) zur Validierung der Anfrage. Sie können die Umgebungsvariable `APP_SECRET` festlegen, um den geheimen Schlüssel für das JWT-Token festzulegen. Standardmäßig ist es `APP_SECRET`.

Sie sollten das JWT-Token im `Authorization`-Header mit dem Schema `Bearer` übergeben.

- [x] `GET /validate`-Endpunkt, um zu überprüfen, ob das JWT-Token gültig ist. Der Statuscode `200` bedeutet, dass das Token gültig ist, andernfalls ist das Token ungültig.

### Kompilieren

Compilet sollte in der Lage sein, die Kompilieranforderung in der Zukunft in die Warteschlange zu stellen. Derzeit kompiliert es jedoch nur den Quellcode direkt.

- [x] `POST /compile`-Endpunkt, um den Quellcode in WebAssembly zu kompilieren.

POST-Body:

```json
{
    "lang": "rs",
    "code": "fn main() { println!(\"Hello, world!\"); }"
}
```

Antwort:

```json
{
    "success": true,
    "message": "Erfolgreich kompiliert",
    "hash": "bb343b0950832ccd077f1515e842196f2ae4bb9e9261b0935ac57916c3cf305d",
    "wasm": "<base64-kodiertes wasm-Binary>"
}
```

- [ ] `POST /submission`-Endpunkt, um den Quellcode in WebAssembly zu kompilieren, aber sofort zurückzukehren und den Quellcode im Hintergrund zu kompilieren.

POST-Body:

```json
{
    "lang": "rs",
    "code": "fn main() { println!(\"Hello, world!\"); }"
}
```

Antwort:

```json
{
    "message": "Eingereicht",
    "hash": "bb343b0950832ccd077f1515e842196f2ae4bb9e9261b0935ac57916c3cf305d"
}
```

- [ ] `GET /submission/{hash}`-Endpunkt, um den Status der Einreichung und das kompilierte WebAssembly-Binary abzurufen, wenn die Kompilierung abgeschlossen ist.

Antwort:

```json
{
    "status": "pending",
    "message": "Warten auf Kompilierung",
    "wasm": null
}
```

```json
{
    "status": "success",
    "message": "Erfolgreich kompiliert",
    "wasm": "<base64-kodiertes wasm-Binary>"
}
```

```json
{
    "status": "failed",
    "message": "Kompilierung fehlgeschlagen (Fehlermeldung)",
    "wasm": null
}
```

### System

- [x] `GET /system`-Endpunkt, um die Systeminformationen abzurufen (derzeit ist nur `capabilities` implementiert)

Antwort:

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

## Entwicklung

Nach dem Klonen des Repositorys müssen Sie Folgendes tun:

- Führen Sie `./scripts/stdlib.sh` aus, um die WASI-Standardbibliothek für C und C++ herunterzuladen.
- Kopieren Sie `libclang_rt.builtins-wasm32.a` an einen Ort, an dem Clang es finden kann. (z.B. `/usr/lib/llvm16/lib/clang/16/lib/wasi`) (Sie können es später tun, die Fehlermeldung wird Ihnen sagen, wo Sie es platzieren müssen.)

Sie können den Server im Entwicklungsmodus mit dem folgenden Befehl ausführen:

```bash
cargo run
```

Bauen Sie den Server mit dem folgenden Befehl:

```bash
cargo build --release
```

---