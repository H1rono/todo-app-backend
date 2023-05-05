# todo-app-backend

[![on-push-CI](https://github.com/H1rono/todo-app-backend/actions/workflows/on-push.yml/badge.svg)](https://github.com/H1rono/todo-app-backend/actions/workflows/on-push.yml) [![codecov](https://codecov.io/github/H1rono/todo-app-backend/branch/main/graph/badge.svg?token=LGXD96Q1L5)](https://codecov.io/github/H1rono/todo-app-backend)

server side implementation of my todo app

## dependencies

- MariaDB
    - runs in Docker
- Rust
    - runs in both Docker and host machine
    - cargo crates
        - [`anyhow`](https://docs.rs/anyhow/latest/anyhow/)
        - [`tokio`](https://docs.rs/tokio/latest/tokio/)
        - [`chrono`](https://docs.rs/chrono/latest/chrono/)
        - [`sqlx`](https://docs.rs/sqlx/latest/sqlx/)
        - [`axum`](https://docs.rs/axum/latest/axum/)
        - [`hyper`](https://docs.rs/hyper/latest/hyper/)
        - [`tower-http`](https://docs.rs/tower-http/latest/tower-http)
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
- docker compose down: `down_docker_db`, `down_docker`, `down`
- clean docker images: `clean_docker_db`, `clean_docker`
- clean cargo: `clean_cargo`
- clean all: `clean`
- docker compose -f db/docker-compose.yml build: `build_docker_db`
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