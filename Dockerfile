FROM rust:1.67-buster

WORKDIR /usr/src/todo-app-backend
COPY . .

RUN cargo build

CMD [ "cargo", "run" ]
