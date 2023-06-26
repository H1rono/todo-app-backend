FROM rust:1.67-bullseye as builder

WORKDIR /usr/src/todo-app-backend
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=builder /usr/src/todo-app-backend/target/release/todo-app-backend /usr/local/bin/todo-app-backend

CMD ["/usr/local/bin/todo-app-backend"]
