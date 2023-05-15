# Compilet

Servidor que compila **Rust**, **C** y **C++** en **WebAssembly**.

## Uso

### Docker

#### Construir

Tengo una imagen de Docker disponible en [Docker Hub (`jacoblincool/compilet`)](https://hub.docker.com/r/jacoblincool/compilet), que admite la compilación de Rust, C y C++ de forma predeterminada.

También puedes construir tu propia imagen con el siguiente comando:

```bash
docker build -t compilet .
```

#### Ejecutar

Puedes ejecutar la imagen con el siguiente comando:

```bash
docker run -p 8000:8000 jacoblincool/compilet
```

O utiliza [el archivo de composición de Docker](./docker-compose.yml) para ejecutar la imagen:

```bash
docker compose up
```

> Es posible que necesites un archivo `.env` para establecer las variables de entorno. Consulta [el archivo `.env.example`](./.env.example) para obtener más información.

Ambos comandos anteriores ejecutarán el servidor en el puerto `8000`, por lo que puedes acceder al servidor en `http://localhost:8000`. También puedes cambiar el puerto estableciendo la variable de entorno `PORT`.

## Endpoints

### Validación

Compilet utiliza [JWT](https://jwt.io/) para validar la solicitud. Puedes establecer la variable de entorno `JWT_SECRET` para establecer la clave secreta para el token JWT, el valor predeterminado es `SECRET_TOKEN`.

Debes pasar el token JWT en la cabecera `Authorization` con el esquema `Bearer`.

- [x] El endpoint `GET /validate` para validar si el token JWT es válido. El código de estado `200` significa que el token es válido, de lo contrario el token es inválido.

### Compilar

Compilet debería ser capaz de encolar la solicitud de compilación en el futuro. Pero actualmente, simplemente compila el código fuente directamente.

- [x] El endpoint `POST /compile` para compilar el código fuente en WebAssembly.

Cuerpo de la solicitud POST:

```json
{
    "lang": "rs",
    "code": "fn main() { println!(\"Hello, world!\"); }"
}
```

Respuesta:

```json
{
    "success": true,
    "message": "Compilado con éxito",
    "hash": "bb343b0950832ccd077f1515e842196f2ae4bb9e9261b0935ac57916c3cf305d",
    "wasm": "<base64 encoded wasm binary>"
}
```

- [ ] El endpoint `POST /submission` para compilar el código fuente en WebAssembly, pero devuelve inmediatamente y compila el código fuente en segundo plano.

Cuerpo de la solicitud POST:

```json
{
    "lang": "rs",
    "code": "fn main() { println!(\"Hello, world!\"); }"
}
```

Respuesta:

```json
{
    "message": "Enviado",
    "hash": "bb343b0950832ccd077f1515e842196f2ae4bb9e9261b0935ac57916c3cf305d"
}
```

- [ ] El endpoint `GET /submission/{hash}` para obtener el estado de la solicitud y el binario WebAssembly compilado si la compilación ha finalizado.

Respuesta:

```json
{
    "status": "pendiente",
    "message": "Esperando compilación",
    "wasm": null
}
```

```json
{
    "status": "éxito",
    "message": "Compilado con éxito",
    "wasm": "<base64 encoded wasm binary>"
}
```

```json
{
    "status": "fallido",
    "message": "La compilación falló (mensaje de error)",
    "wasm": null
}
```

### Sistema

- [ ] El endpoint `GET /system` para obtener la información del sistema.

Respuesta:

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

## Desarrollo

Después de clonar el repositorio, debes:

- Ejecutar `./scripts/stdlib.sh` para descargar la biblioteca estándar WASI para C y C++.
- Copiar `libclang_rt.builtins-wasm32.a` a algún lugar donde Clang pueda encontrarlo. (por ejemplo, `/usr/lib/llvm16/lib/clang/16/lib/wasi`) (Puedes hacerlo más tarde, el mensaje de error te indicará dónde ponerlo.)

Puedes ejecutar el servidor en modo de desarrollo con el siguiente comando:

```bash
cargo run
```

Compila el servidor con el siguiente comando:

```bash
cargo build --release
```
