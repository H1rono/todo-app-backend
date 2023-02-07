# todo-app-backend

[![on-push-CI](https://github.com/H1rono/todo-app-backend/actions/workflows/on-push.yml/badge.svg)](https://github.com/H1rono/todo-app-backend/actions/workflows/on-push.yml)

server side implementation of my todo app

## dependencies

- MariaDB
    - runs in Docker
- Rust
    - runs in both Docker and host machine
    - cargo crates
        - [`anyhow`](https://docs.rs/anyhow/latest/anyhow/)
        - [`tokio`](https://docs.rs/tokio/latest/tokio/)
        - [`sqlx`](https://docs.rs/sqlx/latest/sqlx/)
        - [`futures`](https://docs.rs/futures/latest/futures/)
        - [`chrono`](https://docs.rs/chrono/latest/chrono/)
        - [`axum`](https://docs.rs/axum/latest/axum/)
        - [`hyper`](https://docs.rs/hyper/latest/hyper/)
        - [`tower`](https://docs.rs/tower/latest/tower/)
        - [`serde`](https://docs.rs/serde/latest/serde/)
        - [`serde_json`](https://docs.rs/serde_json/latest/serde_json/)
    - cargo tools
        - [rustfmt](https://github.com/rust-lang/rustfmt)
        - [clippy](https://github.com/rust-lang/rust-clippy)
        - [cargo-make](https://github.com/sagiegurari/cargo-make)

## tasks

see [`Makefile.toml`](https://github.com/H1rono/todo-app-backend/blob/main/Makefile.toml)

- rustfmt: `format`
- clippy: `lint`
- docker compose down: `down_docker`, `down`
- clean docker images: `clean_docker`
- clean cargo: `clean_cargo`
- clean all: `clean`
- docker compose -f docker-compose.su.yml build`build_docker_db`
- docker compose build: `build_docker`
- build only cargo: `build_cargo`
- build all: `build`
- cargo test: `test`
- dev only db`up_db`
- dev cargo locally: `up_cargo`
- dev all: `up`
- emurate CI: `ci`
- (maybe) production only db: `serve_db`
- (maybe) production: `serve`