https://www.stratego.io
======
Multiplayer HTML5 [Stratego](https://en.wikipedia.org/wiki/Stratego).

![Stratego.io preview](/preview.png)

A self-contained Rust application — single binary, no external dependencies. The server (axum), frontend (Leptos/WASM), game logic, and WebSocket pub/sub all compile into one Docker image.

In theory should work on anything that has a browser!

Running with Docker
======

    $ docker build -t stratego .
    $ docker run -p 8080:8080 -v ./data:/app/data stratego

Game data persists to the `./data` volume.

Development
======

Prerequisites: [Rust](https://rustup.rs/), wasm target (`rustup target add wasm32-unknown-unknown`). `wasm-bindgen-cli` is installed automatically if not found.

Run the server (auto-builds the client if needed):

    $ cargo run

Force rebuild the client:

    $ cargo run -- --rebuild

Build client assets only:

    $ cargo run -- --build-only

Run API/WebSocket server without static files:

    $ cargo run -- --no-static

Run the tests:

    $ cargo test

CLI options:

| Flag | Default | Description |
|---|---|---|
| `-p, --port` | `8080` | Server listen port |
| `-d, --data-dir` | `./data` | Path to game data directory |
| `-s, --static-dir` | `client/dist` | Path to frontend dist files |
| `--no-static` | | Don't serve static files (API/WebSocket only) |
| `--rebuild` | | Force rebuild client assets before starting |
| `--build-only` | | Build client assets and exit |

Architecture
======

```
Cargo Workspace
├── stratego (lib + bin) — Game types, logic, axum server, CLI
└── client/              — Leptos WASM frontend (depends on stratego lib)
```

Contributing
======
Contributions are always welcome.

Piece Graphics
======
Pieces found at [vector.gissen.nl](http://vector.gissen.nl/stratego.html). These graphics are not covered by this project's LICENSE and remain the property of their original author.

Issues
======
Any bugs or issues, please open an issue through github.
