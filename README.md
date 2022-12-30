# todo-app-backend

server side implementation of my todo app

## dependencies

- MariaDB
    - runs on Docker
- Rust
    - runs on both Docker and host machine
    - cargo
        - [`anyhow`](https://docs.rs/anyhow/latest/anyhow/)
        - [`tokio`](https://docs.rs/tokio/latest/tokio/)
        - [`sqlx`](https://docs.rs/sqlx/latest/sqlx/)
        - [`futures`](https://docs.rs/futures/latest/futures/)
        - [`chrono`](https://docs.rs/chrono/latest/chrono/)
- [Taskfile](https://taskfile.dev/)

## tasks

see [`Taskfile.yaml`](https://github.com/H1rono/todo-app-backend/blob/main/Taskfile.yaml)

- build: `task build`
- development: `task dev`
- (maybe) production: `task serve`
- test ci: `task ci-test`
- docker compose down: `task down`
