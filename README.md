# TodoMVC — Leptos + Axum + SQLite

A full-stack [TodoMVC](https://todomvc.com/) implementation built with:

- **[Leptos](https://leptos.dev/)** — reactive Rust framework for server-side rendering and client-side hydration
- **[Axum](https://github.com/tokio-rs/axum)** — async Rust web framework
- **[SQLite](https://www.sqlite.org/)** via [sqlx](https://github.com/launchbadge/sqlx) — persistent todo storage
- **WebAssembly** — client-side interactivity via WASM hydration

## Features

- Create, complete, and delete todos
- Filter by All / Active / Completed
- Clear all completed todos
- Todo count display
- Persistent storage via SQLite
- Server-rendered HTML with WASM hydration

## Prerequisites

### Local Development

- [Rust](https://rustup.rs/) (nightly, configured via `rust-toolchain.toml`)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos): `cargo install cargo-leptos`

### Docker

- [Docker](https://docs.docker.com/get-docker/) 20.10+

## Usage

### Local Development

```bash
# Install cargo-leptos if not already installed
cargo install cargo-leptos

# Run the development server with hot-reloading
cargo leptos watch
```

The app will be available at [http://localhost:8080](http://localhost:8080).

### Build for Production

```bash
cargo leptos build --release
```

The compiled binary will be at `target/release/todomvc`. Run it with:

```bash
./target/release/todomvc
```

### Docker

**Build the image:**

```bash
docker build -t todomvc .
```

**Run the container:**

```bash
docker run -p 8080:8080 todomvc
```

**With a persistent database volume:**

```bash
docker run -p 8080:8080 \
  -v $(pwd)/data:/app/data \
  -e DATABASE_URL=sqlite:/app/data/todos.db \
  todomvc
```

The app will be available at [http://localhost:8080](http://localhost:8080).

## Configuration

| Environment Variable | Default | Description |
|---|---|---|
| `DATABASE_URL` | `sqlite:./todos.db` | SQLite database URL |
| `LEPTOS_SITE_ADDR` | `0.0.0.0:8080` | Address and port to listen on |
| `LEPTOS_SITE_ROOT` | `target/site` | Directory for static assets |

## Project Structure

```
.
├── src/
│   ├── main.rs          # Axum server entry point
│   ├── lib.rs           # Leptos app root
│   ├── app.rs           # App component
│   ├── todo.rs          # Todo model and server functions
│   └── components/      # UI components
├── style/
│   └── main.scss        # Application styles
├── migrations/          # SQLite migration files
├── Cargo.toml           # Rust dependencies
├── Dockerfile           # Multi-stage Docker build
└── rust-toolchain.toml  # Rust toolchain pinning
```
