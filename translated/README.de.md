# Compilet

Ein Server, der **Rust**, **C** und **C++** in **WebAssembly** kompiliert.

## Verwendung

### Docker

#### Build

Ich habe ein Docker-Image auf [Docker Hub (`jacoblincool/compilet`)](https://hub.docker.com/r/jacoblincool/compilet) verfügbar, das das Kompilieren von Rust, C und C++ out of the box unterstützt.

Sie können auch Ihr eigenes Image mit dem folgenden Befehl erstellen:

```bash
docker build -t compilet .
```

#### Ausführen

Sie können das Image mit dem folgenden Befehl ausführen:

```bash
docker run -p 8000:8000 -e ROCKET_ADDRESS=0.0.0.0 jacoblincool/compilet
```

Oder verwenden Sie [die Docker-Compose-Datei](./docker-compose.yml), um das Image auszuführen:

```bash
docker compose up
```

> Sie benötigen möglicherweise eine `.env`-Datei, um die Umgebungsvariablen festzulegen. Weitere Informationen finden Sie in der Datei [`.env.example`](./.env.example).

Beide oben genannten Befehle führen den Server auf Port `8000` aus, sodass Sie auf den Server unter `http://localhost:8000` zugreifen können. Sie können auch den Port ändern, indem Sie die Umgebungsvariable `ROCKET_PORT` festlegen.

## Endpunkte

### Validierung

Compilet verwendet [JWT](https://jwt.io/) zur Validierung der Anfrage. Sie können die Umgebungsvariable `JWT_SECRET` festlegen, um den geheimen Schlüssel für das JWT-Token festzulegen. Der Standardwert ist `SECRET_TOKEN`.

Sie sollten das JWT-Token im `Authorization`-Header mit dem Schema `Bearer` übergeben.

- [x] `GET /validate`-Endpunkt zur Überprüfung, ob das JWT-Token gültig ist. Der Statuscode `200` bedeutet, dass das Token gültig ist, andernfalls ist das Token ungültig.

### Kompilieren

Compilet sollte in der Lage sein, die Kompilieranforderung in der Zukunft in die Warteschlange zu stellen. Derzeit kompiliert es jedoch nur den Quellcode direkt.

- [x] `POST /compile`-Endpunkt zum Kompilieren des Quellcodes in WebAssembly

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

- [ ] `POST /submission`-Endpunkt zum Kompilieren des Quellcodes in WebAssembly, aber sofort zurückkehren und den Quellcode im Hintergrund kompilieren.

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

- [ ] `GET /submission/{hash}`-Endpunkt zum Abrufen des Status der Einreichung und des kompilierten WebAssembly-Binärcodes, wenn die Kompilierung abgeschlossen ist.

Antwort:

```json
{
    "status": "ausstehend",
    "message": "Warten auf Kompilierung",
    "wasm": null
}
```

```json
{
    "status": "erfolg",
    "message": "Erfolgreich kompiliert",
    "wasm": "<base64-kodiertes wasm-Binary>"
}
```

```json
{
    "status": "fehlgeschlagen",
    "message": "Kompilierung fehlgeschlagen (Fehlermeldung)",
    "wasm": null
}
```

### System

- [ ] `GET /system`-Endpunkt zum Abrufen der Systeminformationen

Antwort:

```json
{
    "capabilities": {
        "rs": "rust 1.71.0 + rand 0.8.5, Release-Build",
        "c": "clang 16.0.3, Level 3-Optimierungen",
        "cpp": "clang++ 16.0.3, Level 3-Optimierungen"
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

Erstellen Sie den Server mit dem folgenden Befehl:

```bash
cargo build --release
```
