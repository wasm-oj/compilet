# Compilet

Serveur qui compile **Rust**, **C** et **C++** en **WebAssembly**.

## Utilisation

### Docker

#### Construction

J'ai une image Docker disponible sur [Docker Hub (`jacoblincool/compilet`)](https://hub.docker.com/r/jacoblincool/compilet), qui prend en charge la compilation de Rust, C et C++ dès le départ.

Vous pouvez également construire votre propre image avec la commande suivante :

```bash
docker build -t compilet .
```

#### Exécution

Vous pouvez exécuter l'image avec la commande suivante :

```bash
docker run -p 8000:8000 jacoblincool/compilet
```

Ou utilisez [le fichier docker compose](./docker-compose.yml) pour exécuter l'image :

```bash
docker compose up
```

> Vous pouvez avoir besoin d'un fichier `.env` pour définir les variables d'environnement. Consultez [le fichier `.env.example`](./.env.example) pour plus d'informations.

Les deux commandes ci-dessus exécuteront le serveur sur le port `8000`, vous pouvez donc accéder au serveur à l'adresse `http://localhost:8000`. Vous pouvez également changer le port en définissant la variable d'environnement `ROCKET_PORT`.

## Points d'extrémité

### Validation

Compilet utilise [JWT](https://jwt.io/) pour valider la demande. Vous pouvez définir la variable d'environnement `JWT_SECRET` pour définir la clé secrète pour le jeton JWT, par défaut `SECRET_TOKEN`.

Vous devez passer le jeton JWT dans l'en-tête `Authorization` avec le schéma `Bearer`.

- [x] Point d'extrémité `GET /validate` pour valider si le jeton JWT est valide. Le code d'état `200` signifie que le jeton est valide, sinon le jeton est invalide.

### Compilation

Compilet devrait être capable de mettre en file d'attente la demande de compilation à l'avenir. Mais actuellement, il compile simplement le code source directement.

- [x] Point d'extrémité `POST /compile` pour compiler le code source en WebAssembly

Corps POST :

```json
{
    "lang": "rs",
    "code": "fn main() { println!(\"Hello, world!\"); }"
}
```

Réponse :

```json
{
    "success": true,
    "message": "Compilation réussie",
    "hash": "bb343b0950832ccd077f1515e842196f2ae4bb9e9261b0935ac57916c3cf305d",
    "wasm": "<base64 encoded wasm binary>"
}
```

- [ ] Point d'extrémité `POST /submission` pour compiler le code source en WebAssembly, mais retourner immédiatement et compiler le code source en arrière-plan.

Corps POST :

```json
{
    "lang": "rs",
    "code": "fn main() { println!(\"Hello, world!\"); }"
}
```

Réponse :

```json
{
    "message": "Soumis",
    "hash": "bb343b0950832ccd077f1515e842196f2ae4bb9e9261b0935ac57916c3cf305d"
}
```

- [ ] Point d'extrémité `GET /submission/{hash}` pour obtenir l'état de la soumission et le binaire WebAssembly compilé si la compilation est terminée.

Réponse :

```json
{
    "status": "pending",
    "message": "En attente de compilation",
    "wasm": null
}
```

```json
{
    "status": "success",
    "message": "Compilation réussie",
    "wasm": "<base64 encoded wasm binary>"
}
```

```json
{
    "status": "failed",
    "message": "La compilation a échoué (message d'erreur)",
    "wasm": null
}
```

### Système

- [ ] Point d'extrémité `GET /system` pour obtenir les informations système

Réponse :

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

## Développement

Après avoir cloné le référentiel, vous devez :

- Exécutez `./scripts/stdlib.sh` pour télécharger la bibliothèque standard WASI pour C et C++.
- Copiez `libclang_rt.builtins-wasm32.a` quelque part où Clang peut le trouver. (par exemple `/usr/lib/llvm16/lib/clang/16/lib/wasi`) (Vous pouvez le faire plus tard, le message d'erreur vous indiquera où le mettre.)

Vous pouvez exécuter le serveur en mode développement avec la commande suivante :

```bash
cargo run
```

Construisez le serveur avec la commande suivante :

```bash
cargo build --release
```
