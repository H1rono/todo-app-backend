# todo-app-backend

server side implementation of my todo app

## dependencies

- MariaDB
    - runs in Docker
- Rust
    - runs in both Docker and host machine
    - cargo
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
- [Taskfile](https://taskfile.dev/)

## tasks

see [`Taskfile.yml`](https://github.com/H1rono/todo-app-backend/blob/main/Taskfile.yml)

- build: `task build`
- development: `task dev`
- development only MariaDB: `task dev-db`
- (maybe) production: `task serve`
- (maybe) production only MariaDB: `task serve-db`
- ci of DB: `task ci-test`
- ci of cargo: `task ci-cargo`
- docker compose down: `task down`
- docker compose --env-file .env.dev down: `task down-dev`
